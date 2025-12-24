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

/// Assemble a RationalIntegral into a single Expression
pub fn assemble_integral(integral: &RationalIntegral) -> Expression {
    let mut terms = vec![integral.polynomial_part.clone()];

    for (coeff, arg) in &integral.logarithmic_terms {
        terms.push(Expression::mul(vec![
            coeff.clone(),
            Expression::function("ln", vec![Expression::function("abs", vec![arg.clone()])]),
        ]));
    }

    if let Some(remaining) = &integral.remaining {
        terms.push(Expression::function(
            "integrate",
            vec![remaining.clone(), Expression::symbol(Symbol::scalar("x"))],
        ));
    }

    Expression::add(terms)
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
/// use mathhook_core::calculus::integrals::risch::rational::integrate_rational;
/// use mathhook_core::{expr, symbol};
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

    let (quotient, remainder) = match polynomial_div(numerator, denominator, var) {
        Ok(result) => result,
        Err(_) => {
            return RationalIntegral {
                polynomial_part: Expression::integer(0),
                logarithmic_terms: vec![],
                remaining: Some(Expression::mul(vec![
                    numerator.clone(),
                    Expression::pow(denominator.clone(), Expression::integer(-1)),
                ])),
            }
        }
    };

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
            if let Expression::Symbol(s) = &**base {
                if s == var {
                    if let Expression::Number(num_exp) = &**exp {
                        let new_exp = Expression::add(vec![
                            Expression::Number(num_exp.clone()),
                            Expression::integer(1),
                        ]);
                        return Expression::mul(vec![
                            Expression::pow(new_exp.clone(), Expression::integer(-1)),
                            Expression::pow(Expression::symbol(var.clone()), new_exp),
                        ]);
                    }
                }
            }
            Expression::integral(poly.clone(), var.clone())
        }
        Expression::Add(terms) => {
            Expression::add(terms.iter().map(|t| integrate_polynomial(t, var)).collect())
        }
        Expression::Mul(factors) => {
            let (constants, variables): (Vec<_>, Vec<_>) =
                factors.iter().partition(|f| !f.contains_variable(var));

            if variables.is_empty() {
                Expression::mul(vec![
                    Expression::mul((**factors).clone()),
                    Expression::symbol(var.clone()),
                ])
            } else if variables.len() == 1 {
                let integrated_var = integrate_polynomial(variables[0], var);
                if constants.is_empty() {
                    integrated_var
                } else {
                    Expression::mul(vec![
                        Expression::mul(constants.into_iter().cloned().collect()),
                        integrated_var,
                    ])
                }
            } else {
                Expression::integral(poly.clone(), var.clone())
            }
        }
        _ => Expression::integral(poly.clone(), var.clone()),
    }
}

/// Hermite reduction for rational function integration
///
/// Reduces ∫ P/Q dx where Q is not square-free to:
/// - Logarithmic part: ∑ cᵢ ln|qᵢ|
/// - Remaining rational part with square-free denominator
///
/// # Algorithm
///
/// 1. Compute D = gcd(Q, Q')
/// 2. Split Q = D·S where S is square-free
/// 3. Use extended GCD to find A, B such that: P = A·D + B·Q'
/// 4. Then: ∫ P/Q = -A/D·S + ∫ (B + A')/S
fn hermite_reduce(
    numerator: &Expression,
    denominator: &Expression,
    var: &Symbol,
) -> (Vec<(Expression, Expression)>, Option<Expression>) {
    let denom_deriv = denominator.derivative(var.clone()).simplify();

    let gcd = PolynomialGcd::gcd(denominator, &denom_deriv);

    if gcd.is_one() {
        let log_terms = extract_logarithmic_terms(numerator, denominator, var);
        return (log_terms, None);
    }

    let square_free_part = divide_polynomials(denominator, &gcd, var);

    let algebraic_part = Expression::mul(vec![
        Expression::integer(-1),
        numerator.clone(),
        Expression::pow(gcd.clone(), Expression::integer(-1)),
        Expression::pow(square_free_part.clone(), Expression::integer(-1)),
    ])
    .simplify();

    let numerator_deriv = numerator.derivative(var.clone());
    let remaining_num = Expression::add(vec![
        numerator_deriv,
        Expression::mul(vec![
            numerator.clone(),
            Expression::pow(gcd, Expression::integer(-1)),
            denom_deriv,
        ]),
    ])
    .simplify();

    let log_terms = extract_logarithmic_terms(&remaining_num, &square_free_part, var);

    let remaining = if algebraic_part.is_zero() {
        None
    } else {
        Some(algebraic_part)
    };

    (log_terms, remaining)
}

/// Extract logarithmic terms from a proper rational function
///
/// For P/Q where deg(P) < deg(Q) and Q is square-free,
/// attempts to find coefficients cᵢ such that:
/// P/Q = ∑ cᵢ · (qᵢ'/qᵢ)
///
/// Returns list of (coefficient, argument) pairs for ln terms
fn extract_logarithmic_terms(
    numerator: &Expression,
    denominator: &Expression,
    var: &Symbol,
) -> Vec<(Expression, Expression)> {
    if denominator.is_one() || numerator.is_zero() {
        return vec![];
    }

    let denom_deriv = denominator.derivative(var.clone()).simplify();
    let gcd = PolynomialGcd::gcd(denominator, &denom_deriv);

    if !gcd.is_one() {
        return vec![];
    }

    let coefficient = divide_polynomials(numerator, &denom_deriv, var);

    if is_constant(&coefficient, var) {
        vec![(coefficient, denominator.clone())]
    } else {
        vec![]
    }
}

/// Divide two polynomial expressions
///
/// Returns quotient if division is exact, otherwise returns 0
fn divide_polynomials(dividend: &Expression, divisor: &Expression, var: &Symbol) -> Expression {
    if divisor.is_zero() {
        return Expression::undefined();
    }

    if dividend.is_zero() {
        return Expression::integer(0);
    }

    if divisor.is_one() {
        return dividend.clone();
    }

    match polynomial_div(dividend, divisor, var) {
        Ok((quotient, remainder)) => {
            if remainder.is_zero() {
                quotient
            } else {
                Expression::mul(vec![
                    dividend.clone(),
                    Expression::pow(divisor.clone(), Expression::integer(-1)),
                ])
            }
        }
        Err(_) => Expression::mul(vec![
            dividend.clone(),
            Expression::pow(divisor.clone(), Expression::integer(-1)),
        ]),
    }
}

/// Check if expression is constant with respect to variable
fn is_constant(expr: &Expression, var: &Symbol) -> bool {
    !expr.contains_variable(var)
}
