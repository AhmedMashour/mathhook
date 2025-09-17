//! Rational function integration using Hermite reduction algorithm
//!
//! Implements the Hermite reduction method for integrating rational functions.
//! This is a core component of the Risch algorithm for symbolic integration.
//!
//! # Mathematical Background
//!
//! For a rational function R(x) = P(x)/Q(x), the integral decomposes as:
//!
//! ∫ R(x) dx = polynomial_part + ∑ cᵢ ln|qᵢ(x)| + ∫ remaining_rational
//!
//! where:
//! - polynomial_part comes from polynomial long division if deg(P) ≥ deg(Q)
//! - logarithmic terms arise from square-free factorization of denominator
//! - remaining_rational is a proper rational function with square-free denominator
//!
//! # Algorithm Steps
//!
//! 1. **Polynomial Division**: If deg(P) ≥ deg(Q), divide to get quotient + remainder
//! 2. **Square-Free Factorization**: Factor Q = q₁·q₂²·q₃³·... into square-free parts
//! 3. **Hermite Reduction**: Extract logarithmic terms using GCD operations
//! 4. **Partial Fractions**: Decompose remaining rational part
//!
//! # References
//!
//! - Bronstein, M. (2005). *Symbolic Integration I: Transcendental Functions*
//! - Geddes, K. et al. (1992). *Algorithms for Computer Algebra*

use crate::algebra::gcd::PolynomialGcd;
use crate::algebra::polynomial_division::polynomial_div;
use crate::calculus::derivatives::Derivative;
use crate::core::{Expression, Symbol};
use crate::simplify::Simplify;

/// Result of rational function integration
#[derive(Debug, Clone, PartialEq)]
pub struct RationalIntegral {
    /// Polynomial part from long division
    pub polynomial_part: Expression,

    /// Logarithmic terms: ∑ cᵢ ln|qᵢ(x)|
    pub logarithmic_terms: Vec<(Expression, Expression)>,

    /// Remaining rational part (if any)
    pub remaining: Option<Expression>,
}

/// Integrate a rational function P(x)/Q(x)
///
/// Implements Hermite reduction algorithm for complete rational function integration.
///
/// # Arguments
///
/// * `numerator` - Polynomial P(x)
/// * `denominator` - Polynomial Q(x) (must be non-zero)
/// * `var` - Variable of integration
///
/// # Algorithm
///
/// 1. If deg(P) ≥ deg(Q): Perform polynomial long division
///    - P/Q = quotient + remainder/Q
///    - Integrate quotient using power rule
/// 2. Apply Hermite reduction to remainder/Q:
///    - Compute D = gcd(Q, Q') where Q' is derivative
///    - Use extended GCD to extract logarithmic terms
///    - Separate into algebraic + logarithmic parts
/// 3. Return complete integral
///
/// # Examples
///
/// ```rust
/// use mathhook_core::{expr, symbol};
/// use mathhook_core::calculus::integrals::risch::rational::integrate_rational;
///
/// let x = symbol!(x);
/// let num = expr!((x^2) + 1);
/// let den = expr!(x - 1);
/// let result = integrate_rational(&num, &den, &x);
/// ```
///
/// # Returns
///
/// Returns `RationalIntegral` containing:
/// - `polynomial_part`: Result of integrating quotient
/// - `logarithmic_terms`: List of (coefficient, argument) for ln terms
/// - `remaining`: Any unintegrated rational part (None if complete)
pub fn integrate_rational(
    numerator: &Expression,
    denominator: &Expression,
    var: &Symbol,
) -> RationalIntegral {
    if denominator.is_zero() {
        return RationalIntegral {
            polynomial_part: Expression::undefined(),
            logarithmic_terms: vec![],
            remaining: None,
        };
    }

    if numerator.is_zero() {
        return RationalIntegral {
            polynomial_part: Expression::integer(0),
            logarithmic_terms: vec![],
            remaining: None,
        };
    }

    let (quotient, remainder) = polynomial_div(numerator, denominator, var);

    let polynomial_part = integrate_polynomial(&quotient, var);

    if remainder.is_zero() {
        return RationalIntegral {
            polynomial_part,
            logarithmic_terms: vec![],
            remaining: None,
        };
    }

    let (log_terms, remaining_rational) = hermite_reduce(&remainder, denominator, var);

    RationalIntegral {
        polynomial_part,
        logarithmic_terms: log_terms,
        remaining: remaining_rational,
    }
}

/// Integrate polynomial using power rule
///
/// For polynomial p(x) = ∑ aᵢ xⁱ, returns ∑ aᵢ xⁱ⁺¹/(i+1)
fn integrate_polynomial(poly: &Expression, var: &Symbol) -> Expression {
    match poly {
        Expression::Number(n) => Expression::mul(vec![
            Expression::Number(n.clone()),
            Expression::symbol(var.clone()),
        ]),
        Expression::Symbol(s) if s == var => Expression::mul(vec![
            Expression::rational(1, 2),
            Expression::pow(Expression::symbol(var.clone()), Expression::integer(2)),
        ]),
        Expression::Pow(base, exp) => {
            if let (Expression::Symbol(s), Expression::Number(n)) = (base.as_ref(), exp.as_ref()) {
                if s == var {
                    let new_exp = Expression::add(vec![
                        Expression::Number(n.clone()),
                        Expression::integer(1),
                    ]);
                    let denom = Expression::add(vec![
                        Expression::Number(n.clone()),
                        Expression::integer(1),
                    ]);
                    return Expression::mul(vec![
                        Expression::pow(base.as_ref().clone(), new_exp),
                        Expression::pow(denom, Expression::integer(-1)),
                    ])
                    .simplify();
                }
            }
            Expression::function(
                "integrate",
                vec![poly.clone(), Expression::symbol(var.clone())],
            )
        }
        Expression::Mul(factors) => {
            let mut coeff = Expression::integer(1);
            let mut var_part = Expression::integer(1);

            for factor in factors.iter() {
                if contains_var(factor, var) {
                    var_part = Expression::mul(vec![var_part, factor.clone()]);
                } else {
                    coeff = Expression::mul(vec![coeff, factor.clone()]);
                }
            }

            let integrated_var_part = integrate_polynomial(&var_part, var);
            Expression::mul(vec![coeff, integrated_var_part]).simplify()
        }
        Expression::Add(terms) => {
            let integrated_terms: Vec<Expression> = terms
                .iter()
                .map(|term| integrate_polynomial(term, var))
                .collect();
            Expression::add(integrated_terms).simplify()
        }
        _ => Expression::function(
            "integrate",
            vec![poly.clone(), Expression::symbol(var.clone())],
        ),
    }
}

/// Hermite reduction: separate rational function into logarithmic and algebraic parts
///
/// For proper rational R = P/Q, computes:
/// - Logarithmic part: ∑ cᵢ ln|qᵢ(x)|
/// - Remaining rational part (if any)
///
/// # Algorithm
///
/// 1. Compute D = gcd(Q, Q') where Q' = dQ/dx
/// 2. If D = 1 (square-free denominator):
///    - Use partial fractions (future implementation)
///    - Extract logarithmic terms directly
/// 3. If D ≠ 1 (repeated factors):
///    - Use extended GCD to find Bézout coefficients
///    - Separate into V'/V (logarithmic) and remaining
///
/// # Mathematical Background
///
/// The Hermite reduction lemma states:
/// ∫ P/Q dx = R + ∑ cᵢ ln|qᵢ| where R is rational (no logarithm)
///
/// This is computed by finding gcd(Q, Q') and using the identity:
/// P/Q = (A/D)' + B/Q where D = gcd(Q, Q')
fn hermite_reduce(
    numerator: &Expression,
    denominator: &Expression,
    var: &Symbol,
) -> (Vec<(Expression, Expression)>, Option<Expression>) {
    let denom_deriv = denominator.derivative(var.clone());

    let d = denominator.gcd(&denom_deriv);

    if d == Expression::integer(1) {
        return handle_square_free_denominator(numerator, denominator, var);
    }

    extract_logarithmic_terms(numerator, denominator, &d, var)
}

/// Handle square-free denominator case
///
/// When gcd(Q, Q') = 1, the denominator has no repeated factors.
/// For rational P/Q with square-free Q:
/// - If P' = derivative of P matches pattern, result is ln|Q|
/// - Otherwise, use partial fractions
fn handle_square_free_denominator(
    numerator: &Expression,
    denominator: &Expression,
    var: &Symbol,
) -> (Vec<(Expression, Expression)>, Option<Expression>) {
    let denom_deriv = denominator.derivative(var.clone());

    if let Some(coeff) = extract_derivative_coefficient(numerator, &denom_deriv) {
        return (vec![(coeff, denominator.clone())], None);
    }

    (
        vec![],
        Some(Expression::div(numerator.clone(), denominator.clone())),
    )
}

/// Extract logarithmic terms using extended GCD
///
/// Uses the Hermite reduction algorithm with extended GCD
fn extract_logarithmic_terms(
    numerator: &Expression,
    denominator: &Expression,
    d: &Expression,
    var: &Symbol,
) -> (Vec<(Expression, Expression)>, Option<Expression>) {
    let s = denominator.quo_polynomial(d, var);

    let (_gcd, _s_coeff, t_coeff) = s.cofactors(d);

    let s_deriv = s.derivative(var.clone());
    let intermediate = Expression::add(vec![
        d.clone(),
        Expression::mul(vec![
            Expression::integer(-1),
            Expression::mul(vec![t_coeff, s_deriv]),
        ]),
    ])
    .simplify();

    let v = d.gcd(&intermediate);

    if v != Expression::integer(1) && !v.is_zero() {
        let log_coeff = numerator.quo_polynomial(&v, var);

        let remaining_num = numerator.rem_polynomial(&v, var);
        let remaining = if !remaining_num.is_zero() {
            Some(Expression::div(remaining_num, denominator.clone()))
        } else {
            None
        };

        return (vec![(log_coeff, v)], remaining);
    }

    (
        vec![],
        Some(Expression::div(numerator.clone(), denominator.clone())),
    )
}

/// Try to extract coefficient if numerator = c·derivative
///
/// Checks if numerator is a constant multiple of the given derivative
fn extract_derivative_coefficient(
    numerator: &Expression,
    derivative: &Expression,
) -> Option<Expression> {
    if numerator == derivative {
        return Some(Expression::integer(1));
    }

    if let Expression::Mul(factors) = numerator {
        let mut constant_part = Expression::integer(1);
        let mut remaining_part = Expression::integer(1);

        for factor in factors.iter() {
            if !contains_any_symbol(factor) {
                constant_part = Expression::mul(vec![constant_part, factor.clone()]);
            } else {
                remaining_part = Expression::mul(vec![remaining_part, factor.clone()]);
            }
        }

        if remaining_part.simplify() == derivative.simplify() {
            return Some(constant_part.simplify());
        }
    }

    None
}

/// Check if expression contains a specific variable
fn contains_var(expr: &Expression, var: &Symbol) -> bool {
    match expr {
        Expression::Symbol(s) => s == var,
        Expression::Number(_) => false,
        Expression::Add(terms) | Expression::Mul(terms) => {
            terms.iter().any(|t| contains_var(t, var))
        }
        Expression::Pow(base, exp) => contains_var(base, var) || contains_var(exp, var),
        Expression::Function { args, .. } => args.iter().any(|a| contains_var(a, var)),
        _ => false,
    }
}

/// Check if expression contains any symbols (not necessarily the integration variable)
fn contains_any_symbol(expr: &Expression) -> bool {
    match expr {
        Expression::Symbol(_) => true,
        Expression::Number(_) => false,
        Expression::Add(terms) | Expression::Mul(terms) => terms.iter().any(contains_any_symbol),
        Expression::Pow(base, exp) => contains_any_symbol(base) || contains_any_symbol(exp),
        Expression::Function { args, .. } => args.iter().any(contains_any_symbol),
        _ => false,
    }
}

/// Assemble final integral expression from RationalIntegral
///
/// Converts structured result into Expression form
pub fn assemble_integral(result: &RationalIntegral) -> Expression {
    let mut terms = vec![];

    if result.polynomial_part != Expression::integer(0) {
        terms.push(result.polynomial_part.clone());
    }

    for (coeff, arg) in &result.logarithmic_terms {
        let log_term = Expression::mul(vec![
            coeff.clone(),
            Expression::function("ln", vec![Expression::function("abs", vec![arg.clone()])]),
        ]);
        terms.push(log_term);
    }

    if let Some(remaining) = &result.remaining {
        let symbolic_integral = Expression::function("integrate", vec![remaining.clone()]);
        terms.push(symbolic_integral);
    }

    if terms.is_empty() {
        Expression::integer(0)
    } else if terms.len() == 1 {
        terms[0].clone()
    } else {
        Expression::add(terms).simplify()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{expr, symbol};

    #[test]
    fn test_integrate_polynomial() {
        let x = symbol!(x);

        let result = integrate_polynomial(&expr!(x), &x);
        println!("∫ x dx = {}", result);
        assert!(!result.is_zero());

        let result = integrate_polynomial(&expr!(x ^ 2), &x);
        println!("∫ x² dx = {}", result);
        assert!(!result.is_zero());

        let result = integrate_polynomial(&expr!(5), &x);
        println!("∫ 5 dx = {}", result);
        assert!(!result.is_zero());
    }

    #[test]
    fn test_integrate_rational_exact_division() {
        let x = symbol!(x);

        let num = expr!((x ^ 2) - 1);
        let den = expr!(x - 1);
        let result = integrate_rational(&num, &den, &x);

        println!("∫ (x² - 1)/(x - 1) dx:");
        println!("  Polynomial part: {}", result.polynomial_part);
        println!("  Log terms: {:?}", result.logarithmic_terms);
        println!("  Remaining: {:?}", result.remaining);

        assert!(!result.polynomial_part.is_zero());
        assert!(result.logarithmic_terms.is_empty());
        assert!(result.remaining.is_none());
    }

    #[test]
    fn test_integrate_rational_with_remainder() {
        let x = symbol!(x);

        let num = expr!((x ^ 2) + 1);
        let den = expr!(x - 1);
        let result = integrate_rational(&num, &den, &x);

        println!("∫ (x² + 1)/(x - 1) dx:");
        println!("  Polynomial part: {}", result.polynomial_part);
        println!("  Log terms: {:?}", result.logarithmic_terms);
        println!("  Remaining: {:?}", result.remaining);

        assert!(!result.polynomial_part.is_zero());
    }

    #[test]
    fn test_integrate_rational_logarithmic() {
        let x = symbol!(x);

        let num = expr!(1);
        let den = expr!(x - 1);
        let result = integrate_rational(&num, &den, &x);

        println!("∫ 1/(x - 1) dx:");
        println!("  Polynomial part: {}", result.polynomial_part);
        println!("  Log terms: {:?}", result.logarithmic_terms);
        println!("  Remaining: {:?}", result.remaining);

        assert_eq!(result.polynomial_part, Expression::integer(0));
    }

    #[test]
    fn test_integrate_rational_derivative_pattern() {
        let x = symbol!(x);

        let num = expr!(2 * x);
        let den = expr!((x ^ 2) + 1);
        let result = integrate_rational(&num, &den, &x);

        println!("∫ 2x/(x² + 1) dx:");
        println!("  Polynomial part: {}", result.polynomial_part);
        println!("  Log terms: {:?}", result.logarithmic_terms);
        println!("  Remaining: {:?}", result.remaining);

        let assembled = assemble_integral(&result);
        println!("  Assembled: {}", assembled);
    }

    #[test]
    fn test_integrate_rational_partial_fractions() {
        let x = symbol!(x);

        let num = expr!(1);
        let den = expr!((x ^ 2) - 1);
        let result = integrate_rational(&num, &den, &x);

        println!("∫ 1/(x² - 1) dx:");
        println!("  Polynomial part: {}", result.polynomial_part);
        println!("  Log terms: {:?}", result.logarithmic_terms);
        println!("  Remaining: {:?}", result.remaining);

        let assembled = assemble_integral(&result);
        println!("  Assembled: {}", assembled);
    }

    #[test]
    fn test_integrate_rational_complex_numerator() {
        let x = symbol!(x);

        let num = expr!((3 * (x ^ 2)) + (2 * x) + 1);
        let den = Expression::mul(vec![expr!(x - 1), expr!((x ^ 2) + 1)]).simplify();
        let result = integrate_rational(&num, &den, &x);

        println!("∫ (3x² + 2x + 1)/((x-1)(x²+1)) dx:");
        println!("  Polynomial part: {}", result.polynomial_part);
        println!("  Log terms: {:?}", result.logarithmic_terms);
        println!("  Remaining: {:?}", result.remaining);

        let assembled = assemble_integral(&result);
        println!("  Assembled: {}", assembled);
    }
}
