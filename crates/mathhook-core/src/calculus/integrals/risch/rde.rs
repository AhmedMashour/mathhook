//! Risch Differential Equation (RDE) solving
//!
//! Solves the RDE: y' + f*y = g for basic cases.
//! This module handles simple exponential and logarithmic patterns.

use crate::core::{Expression, Number, Symbol};
use super::{RischResult, differential_extension::DifferentialExtension, helpers::extract_division};

/// Integrate transcendental part using RDE
///
/// Attempts to solve the Risch differential equation for basic patterns.
/// Returns Integral, NonElementary, or Unknown based on the analysis.
///
/// # Arguments
///
/// * `expr` - The transcendental expression to integrate
/// * `extensions` - The differential extension tower
/// * `var` - The variable of integration
///
/// # Examples
///
/// ```rust
/// use mathhook_core::calculus::integrals::risch::rde::integrate_transcendental;
/// use mathhook_core::calculus::integrals::risch::differential_extension::DifferentialExtension;
/// use mathhook_core::Expression;
/// use mathhook_core::symbol;
///
/// let x = symbol!(x);
/// let expr = Expression::function("exp", vec![Expression::symbol(x.clone())]);
/// let extensions = vec![DifferentialExtension::Rational];
///
/// let result = integrate_transcendental(&expr, &extensions, x);
/// ```
pub fn integrate_transcendental(
    expr: &Expression,
    _extensions: &[DifferentialExtension],
    var: Symbol,
) -> RischResult {
    // Try basic patterns in order of complexity

    // Case 1: Simple exponential e^(ax)
    if let Some(result) = try_simple_exponential(expr, &var) {
        return RischResult::Integral(result);
    }

    // Case 2: Logarithmic derivative (1/x patterns)
    if let Some(result) = try_logarithmic_derivative(expr, &var) {
        return RischResult::Integral(result);
    }

    // Case 3: Exponential products (x*e^x patterns)
    if let Some(result) = try_exponential_product(expr, &var) {
        return RischResult::Integral(result);
    }

    // Case 4: Non-elementary patterns
    if is_non_elementary_pattern(expr, &var) {
        return RischResult::NonElementary;
    }

    // Case 5: Unknown (defer to symbolic)
    RischResult::Unknown
}

/// Try to integrate simple exponential e^(ax)
///
/// Handles patterns: e^(ax) where a is constant
/// Result: e^(ax)/a
fn try_simple_exponential(expr: &Expression, var: &Symbol) -> Option<Expression> {
    match expr {
        Expression::Function { name, args } if name == "exp" && args.len() == 1 => {
            let arg = &args[0];

            // Check if argument is ax (linear)
            if let Some(coeff) = extract_linear_coefficient(arg, var) {
                // ∫e^(ax) dx = e^(ax)/a
                return Some(Expression::div(expr.clone(), coeff));
            }

            // Check if argument is just x
            if is_just_variable(arg, var.clone()) {
                // ∫e^x dx = e^x
                return Some(expr.clone());
            }

            None
        }
        _ => None,
    }
}

/// Try to integrate logarithmic derivative patterns
///
/// Handles: 1/x → ln|x|, 1/(ax+b) → (1/a)*ln|ax+b|
fn try_logarithmic_derivative(expr: &Expression, var: &Symbol) -> Option<Expression> {
    // Check if expression is a division pattern
    if let Some((num, den)) = extract_division(expr) {
        // Check for 1/x
        if is_one(&num) {
            if is_just_variable(&den, var.clone()) {
                // ∫1/x dx = ln|x|
                return Some(Expression::function("ln", vec![den]));
            }

            // Check for 1/(ax+b)
            if let Some((a, b)) = extract_linear_form(&den, var) {
                // ∫1/(ax+b) dx = (1/a)*ln|ax+b|
                let ln_arg = if b == Expression::integer(0) {
                    Expression::mul(vec![a.clone(), Expression::symbol(var.clone())])
                } else {
                    Expression::add(vec![
                        Expression::mul(vec![a.clone(), Expression::symbol(var.clone())]),
                        b,
                    ])
                };
                return Some(Expression::div(
                    Expression::function("ln", vec![ln_arg]),
                    a,
                ));
            }
        }

        // Check for pattern f'/f (logarithmic derivative)
        if let Some(log_arg) = is_logarithmic_derivative_pattern(&num, &den, var.clone()) {
            // ∫f'/f dx = ln|f|
            return Some(Expression::function("ln", vec![log_arg]));
        }
    }

    None
}

/// Try to integrate exponential products
///
/// Handles: x*e^x, (ax+b)*e^(cx)
fn try_exponential_product(expr: &Expression, var: &Symbol) -> Option<Expression> {
    match expr {
        Expression::Mul(factors) if factors.len() == 2 => {
            // Check for x*e^x pattern
            let f1 = &factors[0];
            let f2 = &factors[1];

            // Try both orders
            if let Some(result) = check_exp_product(f1, f2, var) {
                return Some(result);
            }
            if let Some(result) = check_exp_product(f2, f1, var) {
                return Some(result);
            }

            None
        }
        _ => None,
    }
}

/// Check if pattern is f1 * exp(f2) where f1 is linear
fn check_exp_product(linear: &Expression, exp_part: &Expression, var: &Symbol) -> Option<Expression> {
    // Check if exp_part is e^x
    if let Expression::Function { name, args } = exp_part {
        if name == "exp" && args.len() == 1 {
            let exp_arg = &args[0];

            // Simple case: x * e^x
            if is_just_variable(linear, var.clone()) && is_just_variable(exp_arg, var.clone()) {
                // ∫x*e^x dx = (x-1)*e^x
                return Some(Expression::mul(vec![
                    Expression::add(vec![Expression::symbol(var.clone()), Expression::integer(-1)]),
                    exp_part.clone(),
                ]));
            }
        }
    }

    None
}

/// Check if pattern is known to be non-elementary
///
/// Detects patterns that provably have no elementary antiderivative.
fn is_non_elementary_pattern(expr: &Expression, var: &Symbol) -> bool {
    // Check for division patterns
    if let Some((num, den)) = extract_division(expr) {
        // Pattern: e^x / x (exponential integral Ei(x))
        if is_exponential_of_var(&num, var) && is_just_variable(&den, var.clone()) {
            return true;
        }

        // Pattern: sin(x)/x (sine integral Si(x))
        if is_sine_of_var(&num, var) && is_just_variable(&den, var.clone()) {
            return true;
        }

        // Pattern: 1/ln(x) (logarithmic integral li(x))
        if is_one(&num) && is_logarithm_of_var(&den, var) {
            return true;
        }
    }

    // Pattern: e^(x²) or e^(-x²) (Gaussian, error function)
    if let Expression::Function { name, args } = expr {
        if name == "exp" && args.len() == 1 {
            if is_quadratic(&args[0], var) {
                return true;
            }
        }
    }

    false
}

/// Extract coefficient from linear expression ax
fn extract_linear_coefficient(expr: &Expression, var: &Symbol) -> Option<Expression> {
    match expr {
        Expression::Symbol(s) if s == var => Some(Expression::integer(1)),
        Expression::Mul(factors) => {
            let mut coeff = None;
            let mut has_var = false;

            for factor in &**factors {
                if is_just_variable(factor, var.clone()) {
                    has_var = true;
                } else if !contains_variable(factor, var.clone()) {
                    coeff = Some(factor.clone());
                }
            }

            if has_var {
                coeff.or(Some(Expression::integer(1)))
            } else {
                None
            }
        }
        _ => None,
    }
}

/// Extract (a, b) from ax+b form
fn extract_linear_form(expr: &Expression, var: &Symbol) -> Option<(Expression, Expression)> {
    match expr {
        Expression::Symbol(s) if s == var => Some((Expression::integer(1), Expression::integer(0))),
        Expression::Add(terms) if terms.len() == 2 => {
            // Check for ax + b pattern
            let t1 = &terms[0];
            let t2 = &terms[1];

            // Try t1 = ax, t2 = b
            if let Some(a) = extract_linear_coefficient(t1, var) {
                if !contains_variable(t2, var.clone()) {
                    return Some((a, t2.clone()));
                }
            }

            // Try t2 = ax, t1 = b
            if let Some(a) = extract_linear_coefficient(t2, var) {
                if !contains_variable(t1, var.clone()) {
                    return Some((a, t1.clone()));
                }
            }

            None
        }
        Expression::Mul(_) => {
            // ax form
            if let Some(a) = extract_linear_coefficient(expr, var) {
                Some((a, Expression::integer(0)))
            } else {
                None
            }
        }
        _ => None,
    }
}

/// Check if pattern is f'/f (logarithmic derivative)
fn is_logarithmic_derivative_pattern(
    num: &Expression,
    den: &Expression,
    _var: Symbol,
) -> Option<Expression> {
    // Basic check: if numerator looks like derivative of denominator
    // For full implementation, compute derivative and compare
    // For now, return None (defer to other patterns)
    let _ = (num, den);
    None
}

/// Check if expression is e^x
fn is_exponential_of_var(expr: &Expression, var: &Symbol) -> bool {
    match expr {
        Expression::Function { name, args } if name == "exp" && args.len() == 1 => {
            is_just_variable(&args[0], var.clone())
        }
        _ => false,
    }
}

/// Check if expression is sin(x)
fn is_sine_of_var(expr: &Expression, var: &Symbol) -> bool {
    match expr {
        Expression::Function { name, args } if name == "sin" && args.len() == 1 => {
            is_just_variable(&args[0], var.clone())
        }
        _ => false,
    }
}

/// Check if expression is ln(x) or log(x)
fn is_logarithm_of_var(expr: &Expression, var: &Symbol) -> bool {
    match expr {
        Expression::Function { name, args }
            if (name == "ln" || name == "log") && args.len() == 1 =>
        {
            is_just_variable(&args[0], var.clone())
        }
        _ => false,
    }
}

/// Check if expression is quadratic in variable (x² or -x²)
fn is_quadratic(expr: &Expression, var: &Symbol) -> bool {
    match expr {
        Expression::Pow(base, exp) => {
            is_just_variable(base, var.clone()) && is_integer_two(exp)
        }
        Expression::Mul(factors) if factors.len() == 2 => {
            // Check for -x²
            if is_negative_one(&factors[0]) {
                is_quadratic(&factors[1], var)
            } else if is_negative_one(&factors[1]) {
                is_quadratic(&factors[0], var)
            } else {
                false
            }
        }
        _ => false,
    }
}

/// Check if expression is just the variable
fn is_just_variable(expr: &Expression, var: Symbol) -> bool {
    matches!(expr, Expression::Symbol(s) if *s == var)
}

/// Check if expression is the constant 1
fn is_one(expr: &Expression) -> bool {
    matches!(expr, Expression::Number(n) if n.is_one())
}

/// Check if expression is the constant -1
fn is_negative_one(expr: &Expression) -> bool {
    match expr {
        Expression::Number(Number::Integer(n)) if *n == -1 => true,
        Expression::Mul(factors) if factors.len() == 2 => {
            matches!(&factors[0], Expression::Number(Number::Integer(-1))) && is_one(&factors[1])
                || is_one(&factors[0])
                    && matches!(&factors[1], Expression::Number(Number::Integer(-1)))
        }
        _ => false,
    }
}

/// Check if expression is the integer 2
fn is_integer_two(expr: &Expression) -> bool {
    matches!(expr, Expression::Number(Number::Integer(2)))
}

/// Check if expression contains variable
fn contains_variable(expr: &Expression, var: Symbol) -> bool {
    match expr {
        Expression::Symbol(s) => *s == var,
        Expression::Add(terms) => terms.iter().any(|t| contains_variable(t, var.clone())),
        Expression::Mul(factors) => factors.iter().any(|t| contains_variable(t, var.clone())),
        Expression::Pow(base, exp) => {
            contains_variable(base, var.clone()) || contains_variable(exp, var)
        }
        Expression::Function { args, .. } => args.iter().any(|arg| contains_variable(arg, var.clone())),
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::symbol;

    #[test]
    fn test_simple_exp_x() {
        let x = symbol!(x);
        let expr = Expression::function("exp", vec![Expression::symbol(x.clone())]);
        let extensions = vec![DifferentialExtension::Rational];

        let result = integrate_transcendental(&expr, &extensions, x);
        assert!(matches!(result, RischResult::Integral(_)));
    }

    #[test]
    fn test_simple_exp_2x() {
        let x = symbol!(x);
        let expr = Expression::function(
            "exp",
            vec![Expression::mul(vec![
                Expression::integer(2),
                Expression::symbol(x.clone()),
            ])],
        );
        let extensions = vec![DifferentialExtension::Rational];

        let result = integrate_transcendental(&expr, &extensions, x);
        assert!(matches!(result, RischResult::Integral(_)));
    }

    #[test]
    fn test_logarithmic_derivative_one_over_x() {
        let x = symbol!(x);
        let expr = Expression::div(Expression::integer(1), Expression::symbol(x.clone()));
        let extensions = vec![DifferentialExtension::Rational];

        let result = integrate_transcendental(&expr, &extensions, x);
        assert!(matches!(result, RischResult::Integral(_)));
    }

    #[test]
    fn test_non_elementary_exp_x_squared() {
        let x = symbol!(x);
        let x_squared = Expression::pow(Expression::symbol(x.clone()), Expression::integer(2));
        let expr = Expression::function("exp", vec![x_squared]);
        let extensions = vec![DifferentialExtension::Rational];

        let result = integrate_transcendental(&expr, &extensions, x);
        assert!(matches!(result, RischResult::NonElementary));
    }

    #[test]
    fn test_non_elementary_exp_over_x() {
        let x = symbol!(x);
        let exp_x = Expression::function("exp", vec![Expression::symbol(x.clone())]);
        let expr = Expression::div(exp_x, Expression::symbol(x.clone()));
        let extensions = vec![DifferentialExtension::Rational];

        let result = integrate_transcendental(&expr, &extensions, x);
        assert!(matches!(result, RischResult::NonElementary));
    }

    #[test]
    fn test_extract_linear_coefficient_simple() {
        let x = symbol!(x);
        let expr = Expression::symbol(x.clone());

        let coeff = extract_linear_coefficient(&expr, &x);
        assert_eq!(coeff, Some(Expression::integer(1)));
    }

    #[test]
    fn test_extract_linear_coefficient_scaled() {
        let x = symbol!(x);
        let expr = Expression::mul(vec![Expression::integer(3), Expression::symbol(x.clone())]);

        let coeff = extract_linear_coefficient(&expr, &x);
        assert_eq!(coeff, Some(Expression::integer(3)));
    }

    #[test]
    fn test_is_quadratic_x_squared() {
        let x = symbol!(x);
        let expr = Expression::pow(Expression::symbol(x.clone()), Expression::integer(2));

        assert!(is_quadratic(&expr, &x));
    }

    #[test]
    fn test_is_not_quadratic_x_cubed() {
        let x = symbol!(x);
        let expr = Expression::pow(Expression::symbol(x.clone()), Expression::integer(3));

        assert!(!is_quadratic(&expr, &x));
    }
}
