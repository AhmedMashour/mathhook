//! Polynomial long division operations
//!
//! Implements polynomial division algorithms for univariate polynomials,
//! supporting the Euclidean GCD algorithm and general polynomial arithmetic.
//!
//! # Algorithm
//!
//! This module implements the standard polynomial long division algorithm, which is
//! analogous to long division of integers. Given polynomials f(x) and g(x) with
//! g(x) ≠ 0, the algorithm computes quotient q(x) and remainder r(x) such that:
//!
//! ```text
//! f(x) = q(x) · g(x) + r(x)
//! ```
//!
//! where `degree(r) < degree(g)` or `r = 0`.
//!
//! The algorithm proceeds by repeatedly:
//! 1. Dividing the leading term of the current remainder by the leading term of the divisor
//! 2. Multiplying the result by the divisor and subtracting from the remainder
//! 3. Continuing until the remainder degree is less than the divisor degree
//!
//! # Examples
//!
//! Basic polynomial division:
//!
//! ```rust
//! use mathhook_core::{expr, symbol};
//! use mathhook_core::algebra::polynomial_division::polynomial_div;
//!
//! let x = symbol!(x);
//! // Divide (x^2 - 1) by (x - 1)
//! // Expected: (x^2 - 1) = (x - 1)(x + 1) + 0
//! let dividend = expr!((x^2) - 1);
//! let divisor = expr!(x - 1);
//! let (quotient, remainder) = polynomial_div(&dividend, &divisor, &x);
//! // quotient = x + 1, remainder = 0
//! ```
//!
//! Division with non-zero remainder:
//!
//! ```rust
//! use mathhook_core::{expr, symbol};
//! use mathhook_core::algebra::polynomial_division::polynomial_div;
//!
//! let x = symbol!(x);
//! // Divide (x^2 + 1) by (x - 1)
//! // Expected: (x^2 + 1) = (x - 1)(x + 1) + 2
//! let dividend = expr!((x^2) + 1);
//! let divisor = expr!(x - 1);
//! let (quotient, remainder) = polynomial_div(&dividend, &divisor, &x);
//! // quotient = x + 1, remainder = 2
//! ```
//!
//! # Mathematical Correctness
//!
//! The implementation ensures:
//! - Exact rational arithmetic (no floating point approximation)
//! - Proper handling of edge cases (division by zero, zero dividend, identical polynomials)
//! - Preservation of the division identity: `dividend = divisor * quotient + remainder`
//! - Correct degree properties: `degree(remainder) < degree(divisor)` when remainder ≠ 0

use crate::core::{Expression, Number, Symbol};
use crate::simplify::Simplify;
use std::collections::HashMap;

/// Polynomial long division
///
/// Returns (quotient, remainder) such that:
/// `dividend = divisor * quotient + remainder`
/// and `degree(remainder) < degree(divisor)`
///
/// This implementation uses the standard long division algorithm for univariate
/// polynomials. For multivariate polynomials, the variable parameter specifies
/// which variable to treat as the main polynomial variable.
///
/// # Arguments
///
/// * `dividend` - Polynomial to divide
/// * `divisor` - Polynomial to divide by (must be non-zero)
/// * `var` - Variable to treat as polynomial variable
///
/// # Examples
///
/// ```rust
/// use mathhook_core::{expr, symbol};
/// use mathhook_core::algebra::polynomial_division::polynomial_div;
///
/// let x = symbol!(x);
/// // (x^2 + 3x + 2) / (x + 1) = (x + 2) with remainder 0
/// let dividend = expr!(add: (x^2), (3*x), 2);
/// let divisor = expr!(x + 1);
/// let (quot, rem) = polynomial_div(&dividend, &divisor, &x);
/// ```
///
/// # Returns
///
/// Returns `(quotient, remainder)` tuple where both are expressions
pub fn polynomial_div(
    dividend: &Expression,
    divisor: &Expression,
    var: &Symbol,
) -> (Expression, Expression) {
    // Handle edge cases
    if divisor.is_zero() {
        return (Expression::undefined(), Expression::undefined());
    }

    if dividend.is_zero() {
        return (Expression::integer(0), Expression::integer(0));
    }

    // Fast path: identical polynomials
    if dividend == divisor {
        return (Expression::integer(1), Expression::integer(0));
    }

    // Fast path: divisor is constant
    if let Some(divisor_const) = extract_constant(divisor) {
        if divisor_const.is_zero() {
            return (Expression::undefined(), Expression::undefined());
        }
        let quotient = Expression::mul(vec![
            dividend.clone(),
            Expression::pow(divisor.clone(), Expression::integer(-1)),
        ])
        .simplify();
        return (quotient, Expression::integer(0));
    }

    // Get polynomial degrees
    let dividend_degree = polynomial_degree_in_var(dividend, var);
    let divisor_degree = polynomial_degree_in_var(divisor, var);

    // If dividend degree < divisor degree, quotient is 0
    if dividend_degree < divisor_degree {
        return (Expression::integer(0), dividend.clone());
    }

    // Extract coefficients
    let dividend_coeffs = extract_coefficients(dividend, var);
    let divisor_coeffs = extract_coefficients(divisor, var);

    // Perform polynomial long division
    let mut remainder_coeffs = dividend_coeffs.clone();
    let mut quotient_coeffs: HashMap<i64, Expression> = HashMap::new();

    let divisor_leading_degree = divisor_degree;
    let divisor_leading_coeff = divisor_coeffs
        .get(&divisor_leading_degree)
        .cloned()
        .unwrap_or_else(|| Expression::integer(1));

    while !remainder_coeffs.is_empty() {
        let remainder_leading_degree = *remainder_coeffs.keys().max().unwrap_or(&0);

        if remainder_leading_degree < divisor_leading_degree {
            break;
        }

        let remainder_leading_coeff = remainder_coeffs
            .get(&remainder_leading_degree)
            .cloned()
            .unwrap_or_else(|| Expression::integer(0));

        // Compute quotient term: leading_term_remainder / leading_term_divisor
        let quotient_term_degree = remainder_leading_degree - divisor_leading_degree;
        let quotient_term_coeff = Expression::mul(vec![
            remainder_leading_coeff.clone(),
            Expression::pow(divisor_leading_coeff.clone(), Expression::integer(-1)),
        ])
        .simplify();

        // Add to quotient
        quotient_coeffs.insert(quotient_term_degree, quotient_term_coeff.clone());

        // Subtract (quotient_term * divisor) from remainder
        for (deg, coeff) in &divisor_coeffs {
            let new_degree = deg + quotient_term_degree;
            let term = Expression::mul(vec![quotient_term_coeff.clone(), coeff.clone()]).simplify();

            let current = remainder_coeffs
                .get(&new_degree)
                .cloned()
                .unwrap_or_else(|| Expression::integer(0));
            let updated = Expression::add(vec![
                current,
                Expression::mul(vec![Expression::integer(-1), term]),
            ])
            .simplify();

            if updated.is_zero() {
                remainder_coeffs.remove(&new_degree);
            } else {
                remainder_coeffs.insert(new_degree, updated);
            }
        }
    }

    // Build quotient and remainder expressions from coefficients
    let quotient = build_polynomial_from_coeffs(&quotient_coeffs, var);
    let remainder = build_polynomial_from_coeffs(&remainder_coeffs, var);

    (quotient, remainder)
}

/// Polynomial quotient (division without remainder)
///
/// Returns only the quotient part of polynomial division
///
/// # Arguments
///
/// * `dividend` - Polynomial to divide
/// * `divisor` - Polynomial to divide by
/// * `var` - Variable to treat as polynomial variable
///
/// # Examples
///
/// ```rust
/// use mathhook_core::{expr, symbol};
/// use mathhook_core::algebra::polynomial_division::polynomial_quo;
///
/// let x = symbol!(x);
/// let dividend = expr!(add: (x^2), (3*x), 2);
/// let divisor = expr!(x + 1);
/// let quot = polynomial_quo(&dividend, &divisor, &x);
/// ```
pub fn polynomial_quo(dividend: &Expression, divisor: &Expression, var: &Symbol) -> Expression {
    polynomial_div(dividend, divisor, var).0
}

/// Polynomial remainder
///
/// Returns only the remainder part of polynomial division
///
/// # Arguments
///
/// * `dividend` - Polynomial to divide
/// * `divisor` - Polynomial to divide by
/// * `var` - Variable to treat as polynomial variable
///
/// # Examples
///
/// ```rust
/// use mathhook_core::{expr, symbol};
/// use mathhook_core::algebra::polynomial_division::polynomial_rem;
///
/// let x = symbol!(x);
/// let dividend = expr!((x^2) + 1);
/// let divisor = expr!(x - 1);
/// let rem = polynomial_rem(&dividend, &divisor, &x);
/// ```
pub fn polynomial_rem(dividend: &Expression, divisor: &Expression, var: &Symbol) -> Expression {
    polynomial_div(dividend, divisor, var).1
}

/// Extract constant value from expression if it's a constant
fn extract_constant(expr: &Expression) -> Option<Expression> {
    match expr {
        Expression::Number(_) => Some(expr.clone()),
        _ => None,
    }
}

/// Get polynomial degree with respect to a specific variable
fn polynomial_degree_in_var(expr: &Expression, var: &Symbol) -> i64 {
    match expr {
        Expression::Symbol(s) if s == var => 1,
        Expression::Number(_) => 0,
        Expression::Pow(base, exp) => {
            if let (Expression::Symbol(s), Expression::Number(Number::Integer(e))) =
                (base.as_ref(), exp.as_ref())
            {
                if s == var {
                    return *e;
                }
            }
            0
        }
        Expression::Add(terms) => {
            let mut max_degree = 0i64;
            for term in terms.iter() {
                let deg = polynomial_degree_in_var(term, var);
                max_degree = max_degree.max(deg);
            }
            max_degree
        }
        Expression::Mul(factors) => {
            let mut total_degree = 0i64;
            for factor in factors.iter() {
                total_degree += polynomial_degree_in_var(factor, var);
            }
            total_degree
        }
        _ => 0,
    }
}

/// Extract coefficients from polynomial
///
/// Returns HashMap mapping degree -> coefficient expression
fn extract_coefficients(expr: &Expression, var: &Symbol) -> HashMap<i64, Expression> {
    let mut coeffs: HashMap<i64, Expression> = HashMap::new();

    match expr {
        Expression::Number(n) => {
            if !n.is_zero() {
                coeffs.insert(0, expr.clone());
            }
        }
        Expression::Symbol(s) if s == var => {
            coeffs.insert(1, Expression::integer(1));
        }
        Expression::Pow(base, exp) => {
            if let (Expression::Symbol(s), Expression::Number(Number::Integer(e))) =
                (base.as_ref(), exp.as_ref())
            {
                if s == var {
                    coeffs.insert(*e, Expression::integer(1));
                }
            }
        }
        Expression::Mul(factors) => {
            let mut coeff = Expression::integer(1);
            let mut degree = 0i64;

            for factor in factors.iter() {
                if let Expression::Symbol(s) = factor {
                    if s == var {
                        degree += 1;
                        continue;
                    }
                }
                if let Expression::Pow(base, exp) = factor {
                    if let (Expression::Symbol(s), Expression::Number(Number::Integer(e))) =
                        (base.as_ref(), exp.as_ref())
                    {
                        if s == var {
                            degree += *e;
                            continue;
                        }
                    }
                }
                coeff = Expression::mul(vec![coeff, factor.clone()]).simplify();
            }

            if !coeff.is_zero() {
                coeffs.insert(degree, coeff);
            }
        }
        Expression::Add(terms) => {
            for term in terms.iter() {
                let term_coeffs = extract_coefficients(term, var);
                for (deg, coeff) in term_coeffs {
                    let current = coeffs.get(&deg).cloned().unwrap_or_else(|| Expression::integer(0));
                    let updated = Expression::add(vec![current, coeff]).simplify();
                    if updated.is_zero() {
                        coeffs.remove(&deg);
                    } else {
                        coeffs.insert(deg, updated);
                    }
                }
            }
        }
        _ => {}
    }

    coeffs
}

/// Build polynomial from coefficient map
fn build_polynomial_from_coeffs(
    coeffs: &HashMap<i64, Expression>,
    var: &Symbol,
) -> Expression {
    if coeffs.is_empty() {
        return Expression::integer(0);
    }

    let mut terms = Vec::new();

    for (deg, coeff) in coeffs {
        if coeff.is_zero() {
            continue;
        }

        let term = if *deg == 0 {
            coeff.clone()
        } else if *deg == 1 {
            if coeff.is_one() {
                Expression::symbol(var.clone())
            } else {
                Expression::mul(vec![coeff.clone(), Expression::symbol(var.clone())])
            }
        } else {
            let var_power = Expression::pow(
                Expression::symbol(var.clone()),
                Expression::integer(*deg),
            );
            if coeff.is_one() {
                var_power
            } else {
                Expression::mul(vec![coeff.clone(), var_power])
            }
        };

        terms.push(term);
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
    fn test_polynomial_div_exact() {
        let x = symbol!(x);

        // (x^2 - 1) / (x - 1) = x + 1, remainder 0
        // SymPy: sympy.div(x**2 - 1, x - 1) = (x + 1, 0)
        let dividend = expr!((x^2) - 1);
        let divisor = expr!(x - 1);
        let (quot, rem) = polynomial_div(&dividend, &divisor, &x);

        println!("Quotient: {}, Remainder: {}", quot, rem);
        assert!(rem.is_zero(), "Expected zero remainder");
    }

    #[test]
    fn test_polynomial_div_with_remainder() {
        let x = symbol!(x);

        // (x^2 + 1) / (x - 1) = x + 1, remainder 2
        // SymPy: sympy.div(x**2 + 1, x - 1) = (x + 1, 2)
        let dividend = expr!((x^2) + 1);
        let divisor = expr!(x - 1);
        let (quot, rem) = polynomial_div(&dividend, &divisor, &x);

        println!("Quotient: {}, Remainder: {}", quot, rem);
        assert!(!rem.is_zero(), "Expected non-zero remainder");
    }

    #[test]
    fn test_polynomial_div_by_constant() {
        let x = symbol!(x);

        // (x^2 + 2x + 1) / 2 = (x^2/2 + x + 1/2)
        let dividend = Expression::add(vec![
            Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
            Expression::mul(vec![Expression::integer(2), Expression::symbol(x.clone())]),
            Expression::integer(1),
        ]);
        let divisor = Expression::integer(2);
        let (quot, rem) = polynomial_div(&dividend, &divisor, &x);

        println!("Quotient: {}, Remainder: {}", quot, rem);
        assert!(rem.is_zero(), "Expected zero remainder");
    }

    #[test]
    fn test_polynomial_div_identical() {
        let x = symbol!(x);

        // (x + 1) / (x + 1) = 1, remainder 0
        let dividend = expr!(x + 1);
        let divisor = expr!(x + 1);
        let (quot, rem) = polynomial_div(&dividend, &divisor, &x);

        assert_eq!(quot, Expression::integer(1));
        assert!(rem.is_zero());
    }

    #[test]
    fn test_polynomial_quo() {
        let x = symbol!(x);

        let dividend = expr!((x^2) - 1);
        let divisor = expr!(x - 1);
        let quot = polynomial_quo(&dividend, &divisor, &x);

        println!("Quotient only: {}", quot);
        assert!(!quot.is_zero());
    }

    #[test]
    fn test_polynomial_rem() {
        let x = symbol!(x);

        let dividend = expr!((x^2) + 1);
        let divisor = expr!(x - 1);
        let rem = polynomial_rem(&dividend, &divisor, &x);

        println!("Remainder only: {}", rem);
        assert!(!rem.is_zero());
    }

    #[test]
    fn test_extract_coefficients() {
        let x = symbol!(x);

        // Test: 2x^2 + 3x + 1
        let poly = Expression::add(vec![
            Expression::mul(vec![
                Expression::integer(2),
                Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
            ]),
            Expression::mul(vec![Expression::integer(3), Expression::symbol(x.clone())]),
            Expression::integer(1),
        ]);
        let coeffs = extract_coefficients(&poly, &x);

        println!("Coefficients: {:?}", coeffs);
        assert!(!coeffs.is_empty());
    }
}
