//! Rational function integration via partial fraction decomposition
//!
//! Implements integration of rational functions `P(x)/Q(x)` using:
//! 1. Polynomial long division if `deg(P) >= deg(Q)`
//! 2. Factor denominator into linear and irreducible quadratic factors
//! 3. Decompose into partial fractions using Heaviside's method
//! 4. Integrate each term using standard formulas
//!
//! # Mathematical Background
//!
//! For `P(x)/Q(x)` where `deg(P) < deg(Q)`, factor `Q(x)` as:
//! ```text
//! Q(x) = (x-r₁)^n₁ · ... · (x-rₖ)^nₖ · (x²+p₁x+q₁)^m₁ · ...
//! ```
//!
//! Then the partial fraction decomposition is:
//! ```text
//! P(x)/Q(x) = Σᵢ Σⱼ₌₁ⁿⁱ Aᵢⱼ/(x-rᵢ)ʲ + Σᵢ Σⱼ₌₁ᵐⁱ (Bᵢⱼx+Cᵢⱼ)/(x²+pᵢx+qᵢ)ʲ
//! ```
//!
//! # Integration Formulas
//!
//! Linear terms:
//! - `∫A/(x-r) dx = A·ln|x-r| + C`
//! - `∫A/(x-r)ⁿ dx = -A/((n-1)(x-r)^(n-1)) + C` for `n > 1`
//!
//! Quadratic terms (irreducible `x²+px+q` with `p²-4q < 0`):
//! - Complete the square: `x²+px+q = (x+p/2)² + a²` where `a² = q - p²/4`
//! - `∫(Bx+C)/(x²+px+q) dx = (B/2)·ln|x²+px+q| + ((C-Bp/2)/a)·arctan((x+p/2)/a) + C`
//!
//! # Implementation Status
//!
//! **Fully Implemented:**
//! - Simple linear factors `(x-r)` via cover-up method
//! - Repeated linear factors `(x-r)^n` via Heaviside's method with derivatives
//! - Simple irreducible quadratics `(x²+px+q)` with proper coefficient extraction
//! - Repeated irreducible quadratics `(x²+px+q)²` via Ostrogradsky's reduction formula
//!
//! **Not Yet Implemented:**
//! - Repeated irreducible quadratics `(x²+px+q)^m` with `m > 2`
//!   (Can be generalized using recursive Ostrogradsky reduction)
//! - Automatic polynomial factorization (assumes factored form)
//!
//! # References
//!
//! This implementation follows the approaches in:
//! - Heaviside's cover-up method and derivative technique for repeated poles
//! - Ostrogradsky's reduction formula for repeated quadratics
//! - Stewart, Calculus (8th ed), Chapter 7
//! - Bronstein, "Symbolic Integration I"

use crate::algebra::gcd::PolynomialGcd;
use crate::core::constants::EPSILON;
use crate::core::{Expression, Number, Symbol};
use crate::simplify::Simplify;

pub mod helpers;
pub mod linear;
pub mod quadratic;

use helpers::{is_polynomial, polynomial_degree};
use linear::integrate_linear_factor;
use quadratic::{integrate_repeated_quadratic, integrate_simple_quadratic};

#[derive(Debug, Clone)]
pub struct LinearTerm {
    pub coefficient: Expression,
    pub root: Expression,
    pub power: i64,
}

#[derive(Debug, Clone)]
pub struct QuadraticTerm {
    pub numerator_linear_coeff: Expression,
    pub numerator_constant: Expression,
    pub p_coeff: Expression,
    pub q_coeff: Expression,
    pub power: i64,
}

#[derive(Debug, Clone)]
pub struct PartialFractionDecomposition {
    pub polynomial_part: Expression,
    pub linear_terms: Vec<LinearTerm>,
    pub quadratic_terms: Vec<QuadraticTerm>,
}

#[derive(Debug, Clone)]
enum Factor {
    Linear {
        root: Expression,
        power: i64,
    },
    Quadratic {
        p: Expression,
        q: Expression,
        power: i64,
    },
}

/// Check if expression is a rational function `P(x)/Q(x)`
///
/// # Arguments
///
/// * `expr` - Expression to check
/// * `var` - Variable
///
/// # Examples
///
/// ```
/// use mathhook_core::{Expression, symbol};
/// use mathhook_core::calculus::integrals::rational::is_rational_function;
///
/// let x = symbol!(x);
/// let rational = Expression::mul(vec![
///     Expression::symbol(x.clone()),
///     Expression::pow(
///         Expression::add(vec![
///             Expression::symbol(x.clone()),
///             Expression::integer(1),
///         ]),
///         Expression::integer(-1),
///     ),
/// ]);
///
/// assert!(is_rational_function(&rational, &x));
/// ```
pub fn is_rational_function(expr: &Expression, var: &Symbol) -> bool {
    // Mathematically, a rational function is p(x)/q(x) where p and q are polynomials
    // Polynomials are rational functions with denominator 1
    // So we accept: polynomials OR expressions with polynomial numerator/denominator

    // First check if it's just a polynomial (most common case)
    if is_polynomial(expr, var) {
        return true;
    }

    // Check for rational expressions (including sums of rational functions)
    match expr {
        // Sum of rational functions is also a rational function
        Expression::Add(terms) => terms.iter().all(|term| is_rational_function(term, var)),

        // Product form: p(x) * q(x)^(-1) or products of polynomials
        Expression::Mul(factors) => {
            // Check if all factors are either polynomials or powers of polynomials
            factors.iter().all(|factor| {
                match factor {
                    Expression::Pow(base, exp) => {
                        if let Expression::Number(Number::Integer(_e)) = exp.as_ref() {
                            is_polynomial(base, var) // Accept both positive and negative powers
                        } else {
                            false
                        }
                    }
                    _ => is_polynomial(factor, var),
                }
            })
        }

        // Power of polynomial
        Expression::Pow(base, exp) => {
            if let Expression::Number(Number::Integer(_e)) = exp.as_ref() {
                is_polynomial(base, var) // Accept both x^2 and x^(-1)
            } else {
                false
            }
        }
        _ => false,
    }
}

/// Extract numerator and denominator from rational expression
///
/// # Arguments
///
/// * `expr` - Rational expression
///
/// # Returns
///
/// Tuple `(numerator, denominator)`
///
/// # Examples
///
/// ```
/// use mathhook_core::{Expression, symbol};
/// use mathhook_core::calculus::integrals::rational::extract_numerator_denominator;
///
/// let x = symbol!(x);
/// let expr = Expression::mul(vec![
///     Expression::integer(2),
///     Expression::pow(
///         Expression::symbol(x.clone()),
///         Expression::integer(-1),
///     ),
/// ]);
///
/// let (num, den) = extract_numerator_denominator(&expr);
/// ```
pub fn extract_numerator_denominator(expr: &Expression) -> (Expression, Expression) {
    match expr {
        Expression::Mul(factors) => {
            let mut numerator_factors = Vec::new();
            let mut denominator_factors = Vec::new();

            for factor in factors.iter() {
                if let Expression::Pow(base, exp) = factor {
                    if let Expression::Number(Number::Integer(e)) = exp.as_ref() {
                        if *e < 0 {
                            let positive_exp = Expression::integer(-e);
                            denominator_factors
                                .push(Expression::pow((**base).clone(), positive_exp));
                        } else {
                            numerator_factors.push(factor.clone());
                        }
                    } else {
                        numerator_factors.push(factor.clone());
                    }
                } else {
                    numerator_factors.push(factor.clone());
                }
            }

            let numerator = if numerator_factors.is_empty() {
                Expression::integer(1)
            } else {
                Expression::mul(numerator_factors)
            };

            let denominator = if denominator_factors.is_empty() {
                Expression::integer(1)
            } else {
                Expression::mul(denominator_factors)
            };

            (numerator, denominator)
        }
        Expression::Pow(base, exp) => {
            if let Expression::Number(Number::Integer(e)) = exp.as_ref() {
                if *e < 0 {
                    (
                        Expression::integer(1),
                        Expression::pow((**base).clone(), Expression::integer(-e)),
                    )
                } else {
                    (expr.clone(), Expression::integer(1))
                }
            } else {
                (expr.clone(), Expression::integer(1))
            }
        }
        _ => (expr.clone(), Expression::integer(1)),
    }
}

/// Integrate rational function `P(x)/Q(x)` via partial fractions
///
/// # Arguments
///
/// * `expr` - Rational expression to integrate
/// * `var` - Integration variable
///
/// # Returns
///
/// Integrated expression or `None` if not a rational function or unsupported
///
/// # Examples
///
/// ```
/// use mathhook_core::{Expression, symbol};
/// use mathhook_core::calculus::integrals::rational::integrate_rational;
///
/// let x = symbol!(x);
/// let rational = Expression::mul(vec![
///     Expression::integer(1),
///     Expression::pow(
///         Expression::add(vec![
///             Expression::symbol(x.clone()),
///             Expression::integer(-1),
///         ]),
///         Expression::integer(-1),
///     ),
/// ]);
///
/// let result = integrate_rational(&rational, &x);
/// assert!(result.is_some());
/// ```
pub fn integrate_rational(expr: &Expression, var: &Symbol) -> Option<Expression> {
    if !is_rational_function(expr, var) {
        return None;
    }

    let (numerator, denominator) = extract_numerator_denominator(expr);

    let num_degree = polynomial_degree(&numerator, var);
    let den_degree = polynomial_degree(&denominator, var);

    // Early return for simple constant/x^n cases ONLY if denominator is a pure monomial (just x^n)
    // This prevents expensive partial fraction decomposition for trivial cases like 1/x^2
    // BUT we must NOT early return for general polynomials like 1/(x^2 + 2x + 1)
    let is_simple_monomial = match &denominator {
        Expression::Symbol(_) => true,
        Expression::Pow(base, _) => matches!(base.as_ref(), Expression::Symbol(_)),
        _ => false,
    };

    if num_degree == 0 && den_degree >= 1 && is_simple_monomial {
        // Numerator is constant, denominator is simple monomial: c/x^n pattern
        // Let basic integration rules handle this via power rule
        return None;
    }

    // Early return if denominator doesn't actually contain the variable
    // This happens when expression is in wrong variable (e.g., contains 'x' but var is 'u')
    // Return None to let other strategies handle it
    if den_degree == 0 {
        return None;
    }

    let (quotient, remainder) = if num_degree >= den_degree {
        numerator.div_polynomial(&denominator, var)
    } else {
        (Expression::integer(0), numerator)
    };

    let polynomial_integral = if !quotient.is_zero() {
        integrate_polynomial(&quotient, var)
    } else {
        Expression::integer(0)
    };

    if remainder.is_zero() {
        return Some(polynomial_integral);
    }

    let factors = factor_simple_denominator(&denominator, var)?;

    let mut result = polynomial_integral;

    for factor in factors.iter() {
        match factor {
            Factor::Linear { root, power } => {
                let factor_result =
                    integrate_linear_factor(&remainder, &denominator, root, *power, var)?;
                result = Expression::add(vec![result, factor_result]).simplify();
            }
            Factor::Quadratic { p, q, power } => {
                if *power == 1 {
                    let factor_result =
                        integrate_simple_quadratic(&remainder, &denominator, p, q, var)?;
                    result = Expression::add(vec![result, factor_result]).simplify();
                } else {
                    let factor_result =
                        integrate_repeated_quadratic(&remainder, &denominator, p, q, *power, var)?;
                    result = Expression::add(vec![result, factor_result]).simplify();
                }
            }
        }
    }

    Some(result.simplify())
}

fn integrate_polynomial(poly: &Expression, var: &Symbol) -> Expression {
    match poly {
        Expression::Number(_) => {
            Expression::mul(vec![poly.clone(), Expression::symbol(var.clone())])
        }
        Expression::Symbol(s) if s == var => Expression::mul(vec![
            Expression::rational(1, 2),
            Expression::pow(Expression::symbol(var.clone()), Expression::integer(2)),
        ]),
        Expression::Pow(base, exp) => {
            if let (Expression::Symbol(s), Expression::Number(Number::Integer(n))) =
                (base.as_ref(), exp.as_ref())
            {
                if s == var {
                    let new_exp = n + 1;
                    return Expression::mul(vec![
                        Expression::rational(1, new_exp),
                        Expression::pow(
                            Expression::symbol(var.clone()),
                            Expression::integer(new_exp),
                        ),
                    ]);
                }
            }
            Expression::mul(vec![poly.clone(), Expression::symbol(var.clone())])
        }
        Expression::Mul(factors) => {
            let mut coeff = Expression::integer(1);
            let mut var_power = 0i64;

            for factor in factors.iter() {
                if let Expression::Symbol(s) = factor {
                    if s == var {
                        var_power += 1;
                        continue;
                    }
                }
                if let Expression::Pow(base, exp) = factor {
                    if let (Expression::Symbol(s), Expression::Number(Number::Integer(e))) =
                        (base.as_ref(), exp.as_ref())
                    {
                        if s == var {
                            var_power += e;
                            continue;
                        }
                    }
                }
                coeff = Expression::mul(vec![coeff, factor.clone()]);
            }

            let new_power = var_power + 1;
            Expression::mul(vec![
                Expression::mul(vec![coeff, Expression::rational(1, new_power)]),
                Expression::pow(
                    Expression::symbol(var.clone()),
                    Expression::integer(new_power),
                ),
            ])
        }
        Expression::Add(terms) => {
            Expression::add(terms.iter().map(|t| integrate_polynomial(t, var)).collect())
        }
        _ => Expression::mul(vec![poly.clone(), Expression::symbol(var.clone())]),
    }
}

fn factor_simple_denominator(denom: &Expression, var: &Symbol) -> Option<Vec<Factor>> {
    let mut factors = Vec::new();

    match denom {
        Expression::Add(terms) => {
            if terms.len() == 2 {
                if let Expression::Symbol(s) = &terms[0] {
                    if s == var {
                        if let Expression::Number(_) = &terms[1] {
                            let root =
                                Expression::mul(vec![Expression::integer(-1), terms[1].clone()]);
                            factors.push(Factor::Linear { root, power: 1 });
                            return Some(factors);
                        }
                    }
                }
            }

            if let Some((p, q)) = helpers::try_extract_quadratic(denom, var) {
                // Check if this quadratic factors over the reals
                // Discriminant Δ = p^2 - 4q determines factorability:
                //   Δ < 0: irreducible (complex roots)
                //   Δ = 0: perfect square (double root)
                //   Δ > 0: two distinct real roots

                // For integer coefficients, check if discriminant is a perfect square
                if let (
                    Expression::Number(Number::Integer(p_val)),
                    Expression::Number(Number::Integer(q_val)),
                ) = (&p, &q)
                {
                    let discriminant = p_val * p_val - 4 * q_val;

                    if discriminant == 0 {
                        let root = Expression::rational(-p_val, 2);
                        factors.push(Factor::Linear { root, power: 2 });
                        return Some(factors);
                    } else if discriminant > 0 {
                        // Check if discriminant is a perfect square (for rational roots)
                        let sqrt_disc = (discriminant as f64).sqrt();
                        if sqrt_disc.fract().abs() < EPSILON {
                            let sqrt_disc_int = sqrt_disc as i64;
                            let root1 = Expression::rational(-p_val + sqrt_disc_int, 2);
                            let root2 = Expression::rational(-p_val - sqrt_disc_int, 2);
                            factors.push(Factor::Linear {
                                root: root1,
                                power: 1,
                            });
                            factors.push(Factor::Linear {
                                root: root2,
                                power: 1,
                            });
                            return Some(factors);
                        }
                    }
                }

                // Irreducible quadratic (complex roots or irrational roots)
                factors.push(Factor::Quadratic { p, q, power: 1 });
                return Some(factors);
            }

            factors.push(Factor::Linear {
                root: Expression::integer(0),
                power: 1,
            });
        }
        Expression::Mul(terms) => {
            for term in terms.iter() {
                if let Some(mut term_factors) = factor_simple_denominator(term, var) {
                    factors.append(&mut term_factors);
                }
            }
        }
        Expression::Pow(base, exp) => {
            if let Expression::Number(Number::Integer(e)) = exp.as_ref() {
                if let Some(base_factors) = factor_simple_denominator(base, var) {
                    for factor in base_factors {
                        match factor {
                            Factor::Linear { root, power } => {
                                factors.push(Factor::Linear {
                                    root,
                                    power: power * e,
                                });
                            }
                            Factor::Quadratic { p, q, power } => {
                                factors.push(Factor::Quadratic {
                                    p,
                                    q,
                                    power: power * e,
                                });
                            }
                        }
                    }
                    return Some(factors);
                }
            }
        }
        Expression::Symbol(s) if s == var => {
            factors.push(Factor::Linear {
                root: Expression::integer(0),
                power: 1,
            });
        }
        _ => {
            return None;
        }
    }

    Some(factors)
}
