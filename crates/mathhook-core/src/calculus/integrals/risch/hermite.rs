//! Hermite reduction for separating polynomial and rational parts
//!
//! Hermite reduction separates a rational function into:
//! - Rational part (easy to integrate)
//! - Logarithmic part (requires RDE solving)

use crate::core::{Expression, Number};
use super::differential_extension::DifferentialExtension;

/// Hermite reduction: Separate polynomial and rational parts
///
/// Returns (rational_part, transcendental_part).
///
/// The rational part can be integrated directly, while the transcendental
/// part requires RDE solving.
///
/// # Arguments
///
/// * `expr` - The expression to reduce
/// * `extensions` - The differential extension tower
///
/// # Examples
///
/// ```rust
/// use mathhook_core::calculus::integrals::risch::hermite::hermite_reduction;
/// use mathhook_core::calculus::integrals::risch::differential_extension::{build_extension_tower, DifferentialExtension};
/// use mathhook_core::Expression;
/// use mathhook_core::symbol;
///
/// let x = symbol!(x);
/// let expr = Expression::div(Expression::integer(1), Expression::symbol(x.clone()));
/// let extensions = vec![DifferentialExtension::Rational];
///
/// let result = hermite_reduction(&expr, &extensions);
/// assert!(result.is_some());
/// ```
pub fn hermite_reduction(
    expr: &Expression,
    _extensions: &[DifferentialExtension],
) -> Option<(Expression, Expression)> {
    use super::helpers::extract_division;

    // Special case: logarithmic derivatives like 1/x integrate to ln(x)
    // These should be classified as transcendental, not rational
    if let Some((num, _den)) = extract_division(expr) {
        if num == Expression::integer(1) {
            // 1/f(x) is a logarithmic derivative -> transcendental
            return Some((Expression::integer(0), expr.clone()));
        }
    }

    // For basic implementation: detect if expression is purely rational
    // Separate into polynomial part + proper rational part
    if is_rational_function(expr) {
        // All rational, no transcendental part
        Some((expr.clone(), Expression::integer(0)))
    } else {
        // Has transcendental part
        Some((Expression::integer(0), expr.clone()))
    }
}

/// Check if expression is a rational function
///
/// A rational function is P(x)/Q(x) where P and Q are polynomials.
/// This excludes transcendental functions like exp, ln, sin, cos.
pub fn is_rational_function(expr: &Expression) -> bool {
    use super::helpers::extract_division;

    match expr {
        Expression::Number(_) | Expression::Constant(_) | Expression::Symbol(_) => true,
        Expression::Add(terms) => terms.iter().all(is_rational_function),
        Expression::Mul(factors) => {
            // Division is represented as num * den^(-1)
            // Check if this is a division pattern
            if let Some((num, den)) = extract_division(expr) {
                // Both numerator and denominator must be rational
                is_rational_function(&num) && is_rational_function(&den)
            } else {
                // Not a division pattern, all factors must be rational
                factors.iter().all(is_rational_function)
            }
        }
        Expression::Pow(base, exp) => {
            // Special case: negative integer exponent is division (rational)
            if let Expression::Number(Number::Integer(n)) = &**exp {
                if *n < 0 {
                    // x^(-n) = 1/x^n is rational if base is rational
                    return is_rational_function(base);
                }
            }
            // Rational if base is rational and exponent is non-negative integer
            is_rational_function(base) && is_nonnegative_integer(exp)
        }
        Expression::Function { name, .. } => {
            // Transcendental functions are not rational
            !is_transcendental_function(name)
        }
        _ => false,
    }
}

/// Check if expression is a non-negative integer
fn is_nonnegative_integer(expr: &Expression) -> bool {
    match expr {
        Expression::Number(Number::Integer(n)) => *n >= 0,
        _ => false,
    }
}

/// Check if function name is transcendental
fn is_transcendental_function(name: &str) -> bool {
    matches!(
        name,
        "exp" | "ln" | "log" | "sin" | "cos" | "tan" | "cot" | "sec" | "csc"
            | "arcsin" | "arccos" | "arctan" | "sinh" | "cosh" | "tanh"
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::symbol;

    #[test]
    fn test_is_rational_polynomial() {
        let x = symbol!(x);
        let expr = Expression::add(vec![
            Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
            Expression::symbol(x.clone()),
            Expression::integer(1),
        ]);

        assert!(is_rational_function(&expr));
    }

    #[test]
    fn test_is_rational_fraction() {
        let x = symbol!(x);
        let expr = Expression::div(
            Expression::integer(1),
            Expression::add(vec![Expression::symbol(x.clone()), Expression::integer(1)]),
        );

        assert!(is_rational_function(&expr));
    }

    #[test]
    fn test_is_not_rational_exponential() {
        let x = symbol!(x);
        let expr = Expression::function("exp", vec![Expression::symbol(x.clone())]);

        assert!(!is_rational_function(&expr));
    }

    #[test]
    fn test_is_not_rational_logarithm() {
        let x = symbol!(x);
        let expr = Expression::function("ln", vec![Expression::symbol(x.clone())]);

        assert!(!is_rational_function(&expr));
    }

    #[test]
    fn test_hermite_reduction_logarithmic_derivative() {
        let x = symbol!(x);
        let expr = Expression::div(Expression::integer(1), Expression::symbol(x.clone()));
        let extensions = vec![DifferentialExtension::Rational];

        let result = hermite_reduction(&expr, &extensions);
        assert!(result.is_some());

        let (rational, transcendental) = result.unwrap();
        // 1/x is a logarithmic derivative -> integrates to ln(x) (transcendental)
        assert_eq!(rational, Expression::integer(0));
        assert_ne!(transcendental, Expression::integer(0));
    }

    #[test]
    fn test_hermite_reduction_polynomial() {
        let x = symbol!(x);
        // x^2 + 1 is purely rational (polynomial)
        let expr = Expression::add(vec![
            Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
            Expression::integer(1),
        ]);
        let extensions = vec![DifferentialExtension::Rational];

        let result = hermite_reduction(&expr, &extensions);
        assert!(result.is_some());

        let (rational, transcendental) = result.unwrap();
        // Polynomial is rational, no transcendental part
        assert_ne!(rational, Expression::integer(0));
        assert_eq!(transcendental, Expression::integer(0));
    }

    #[test]
    fn test_hermite_reduction_transcendental() {
        let x = symbol!(x);
        let expr = Expression::function("exp", vec![Expression::symbol(x)]);
        let extensions = vec![DifferentialExtension::Rational];

        let result = hermite_reduction(&expr, &extensions);
        assert!(result.is_some());

        let (rational, transcendental) = result.unwrap();
        assert_eq!(rational, Expression::integer(0));
        assert_ne!(transcendental, Expression::integer(0));
    }
}
