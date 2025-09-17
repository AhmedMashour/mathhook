//! Polynomial Coefficient Utilities
//!
//! Functions for extracting and manipulating polynomial coefficients.
//! Provides utilities for working with polynomial coefficient lists
//! and extracting coefficients at specific degrees.

use crate::core::{Expression, Number, Symbol};
use std::collections::HashMap;

/// Extract all coefficients of a polynomial as a map from degree to coefficient
///
/// Returns a HashMap where keys are degrees and values are coefficients.
///
/// # Arguments
///
/// * `expr` - The polynomial expression
/// * `var` - The variable to extract coefficients for
///
/// # Returns
///
/// A HashMap mapping degree (i64) to coefficient (Expression)
///
/// # Examples
///
/// ```rust
/// use mathhook_core::core::polynomial::extract_coefficient_map;
/// use mathhook_core::{expr, symbol};
///
/// let x = symbol!(x);
/// let poly = expr!((3 * (x ^ 2)) + (2 * x) + 1);
///
/// let coeffs = extract_coefficient_map(&poly, &x);
/// // coeffs[0] = 1, coeffs[1] = 2, coeffs[2] = 3
/// ```
pub fn extract_coefficient_map(expr: &Expression, var: &Symbol) -> HashMap<i64, Expression> {
    let mut coefficients = HashMap::new();
    extract_coefficients_recursive(expr, var, &mut coefficients);
    coefficients
}

fn extract_coefficients_recursive(
    expr: &Expression,
    var: &Symbol,
    coefficients: &mut HashMap<i64, Expression>,
) {
    match expr {
        Expression::Number(_) => {
            // Constant term has degree 0
            add_coefficient(coefficients, 0, expr.clone());
        }
        Expression::Symbol(s) => {
            if s == var {
                // x has degree 1, coefficient 1
                add_coefficient(coefficients, 1, Expression::integer(1));
            } else {
                // Other symbol is a constant (degree 0)
                add_coefficient(coefficients, 0, expr.clone());
            }
        }
        Expression::Add(terms) => {
            for term in terms.iter() {
                extract_coefficients_recursive(term, var, coefficients);
            }
        }
        Expression::Mul(factors) => {
            let (coef, deg) = extract_term_coefficient_and_degree(factors, var);
            add_coefficient(coefficients, deg, coef);
        }
        Expression::Pow(base, exp) => {
            if let Expression::Symbol(s) = base.as_ref() {
                if s == var {
                    if let Expression::Number(Number::Integer(n)) = exp.as_ref() {
                        // x^n has degree n, coefficient 1
                        add_coefficient(coefficients, *n, Expression::integer(1));
                        return;
                    }
                }
            }
            // Non-variable power is a constant
            add_coefficient(coefficients, 0, expr.clone());
        }
        _ => {
            // Treat other expressions as constants
            add_coefficient(coefficients, 0, expr.clone());
        }
    }
}

fn add_coefficient(coefficients: &mut HashMap<i64, Expression>, degree: i64, coef: Expression) {
    coefficients
        .entry(degree)
        .and_modify(|existing| {
            *existing = Expression::add(vec![existing.clone(), coef.clone()]);
        })
        .or_insert(coef);
}

fn extract_term_coefficient_and_degree(factors: &[Expression], var: &Symbol) -> (Expression, i64) {
    let mut coefficient_factors = Vec::new();
    let mut total_degree = 0i64;

    for factor in factors.iter() {
        match factor {
            Expression::Symbol(s) if s == var => {
                total_degree += 1;
            }
            Expression::Pow(base, exp) => {
                if let Expression::Symbol(s) = base.as_ref() {
                    if s == var {
                        if let Expression::Number(Number::Integer(n)) = exp.as_ref() {
                            total_degree += n;
                            continue;
                        }
                    }
                }
                coefficient_factors.push(factor.clone());
            }
            _ => {
                coefficient_factors.push(factor.clone());
            }
        }
    }

    let coef = if coefficient_factors.is_empty() {
        Expression::integer(1)
    } else if coefficient_factors.len() == 1 {
        coefficient_factors.into_iter().next().unwrap()
    } else {
        Expression::mul(coefficient_factors)
    };

    (coef, total_degree)
}

/// Get the coefficient at a specific degree
///
/// # Arguments
///
/// * `expr` - The polynomial expression
/// * `var` - The variable
/// * `degree` - The degree to extract coefficient for
///
/// # Returns
///
/// The coefficient at the specified degree, or 0 if not present
///
/// # Examples
///
/// ```rust
/// use mathhook_core::core::polynomial::coefficient_at;
/// use mathhook_core::{expr, symbol};
///
/// let x = symbol!(x);
/// let poly = expr!((3 * (x ^ 2)) + (2 * x) + 1);
///
/// let c2 = coefficient_at(&poly, &x, 2);  // Returns 3
/// let c1 = coefficient_at(&poly, &x, 1);  // Returns 2
/// let c0 = coefficient_at(&poly, &x, 0);  // Returns 1
/// ```
pub fn coefficient_at(expr: &Expression, var: &Symbol, degree: i64) -> Expression {
    let coeffs = extract_coefficient_map(expr, var);
    coeffs
        .get(&degree)
        .cloned()
        .unwrap_or_else(|| Expression::integer(0))
}

/// Get all coefficients as a vector ordered by degree (ascending)
///
/// # Arguments
///
/// * `expr` - The polynomial expression
/// * `var` - The variable
///
/// # Returns
///
/// A vector of (degree, coefficient) pairs ordered by degree
///
/// # Examples
///
/// ```rust
/// use mathhook_core::core::polynomial::coefficients_list;
/// use mathhook_core::{expr, symbol};
///
/// let x = symbol!(x);
/// let poly = expr!((3 * (x ^ 2)) + (2 * x) + 1);
///
/// let coeffs = coefficients_list(&poly, &x);
/// // Returns [(0, 1), (1, 2), (2, 3)]
/// ```
pub fn coefficients_list(expr: &Expression, var: &Symbol) -> Vec<(i64, Expression)> {
    let map = extract_coefficient_map(expr, var);
    let mut list: Vec<_> = map.into_iter().collect();
    list.sort_by_key(|(deg, _)| *deg);
    list
}

/// Extract the constant term (coefficient of degree 0)
///
/// # Arguments
///
/// * `expr` - The polynomial expression
/// * `var` - The variable
///
/// # Returns
///
/// The constant term
pub fn constant_term(expr: &Expression, var: &Symbol) -> Expression {
    coefficient_at(expr, var, 0)
}

/// Check if polynomial is monic (leading coefficient is 1)
///
/// # Arguments
///
/// * `expr` - The polynomial expression
/// * `var` - The variable
///
/// # Returns
///
/// True if the polynomial is monic
pub fn is_monic(expr: &Expression, var: &Symbol) -> bool {
    use super::properties::PolynomialProperties;
    expr.leading_coefficient(var) == Expression::integer(1)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::symbol;

    #[test]
    fn test_extract_coefficient_map_simple() {
        let x = symbol!(x);
        // 3x^2 + 2x + 1
        let poly = Expression::add(vec![
            Expression::mul(vec![
                Expression::integer(3),
                Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
            ]),
            Expression::mul(vec![Expression::integer(2), Expression::symbol(x.clone())]),
            Expression::integer(1),
        ]);

        let coeffs = extract_coefficient_map(&poly, &x);

        assert!(coeffs.contains_key(&0));
        assert!(coeffs.contains_key(&1));
        assert!(coeffs.contains_key(&2));
    }

    #[test]
    fn test_coefficient_at() {
        let x = symbol!(x);
        // x^2 + 2x + 3
        let poly = Expression::add(vec![
            Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
            Expression::mul(vec![Expression::integer(2), Expression::symbol(x.clone())]),
            Expression::integer(3),
        ]);

        let c0 = coefficient_at(&poly, &x, 0);
        let c1 = coefficient_at(&poly, &x, 1);
        let c2 = coefficient_at(&poly, &x, 2);
        let c3 = coefficient_at(&poly, &x, 3);

        assert_eq!(c0, Expression::integer(3));
        assert_eq!(c1, Expression::integer(2));
        assert_eq!(c2, Expression::integer(1));
        assert_eq!(c3, Expression::integer(0));
    }

    #[test]
    fn test_coefficients_list() {
        let x = symbol!(x);
        let poly = Expression::add(vec![
            Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
            Expression::integer(1),
        ]);

        let list = coefficients_list(&poly, &x);

        assert_eq!(list.len(), 2);
        assert_eq!(list[0].0, 0); // degree 0
        assert_eq!(list[1].0, 2); // degree 2
    }

    #[test]
    fn test_constant_term() {
        let x = symbol!(x);
        let poly = Expression::add(vec![Expression::symbol(x.clone()), Expression::integer(5)]);

        assert_eq!(constant_term(&poly, &x), Expression::integer(5));
    }

    #[test]
    fn test_is_monic() {
        let x = symbol!(x);

        // x^2 + 2x + 1 is monic
        let monic = Expression::add(vec![
            Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
            Expression::mul(vec![Expression::integer(2), Expression::symbol(x.clone())]),
            Expression::integer(1),
        ]);
        assert!(is_monic(&monic, &x));

        // 2x^2 + x + 1 is NOT monic
        let not_monic = Expression::add(vec![
            Expression::mul(vec![
                Expression::integer(2),
                Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
            ]),
            Expression::symbol(x.clone()),
            Expression::integer(1),
        ]);
        assert!(!is_monic(&not_monic, &x));
    }
}
