//! Content extraction and primitive part computation
//!
//! This module provides methods for extracting the content (GCD of coefficients)
//! and computing the primitive part of polynomials.

use crate::core::{Expression, Number};

/// Integer GCD
pub(crate) fn integer_gcd(mut a: i64, mut b: i64) -> i64 {
    a = a.abs();
    b = b.abs();
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a.max(1)
}

/// Compute content (GCD of numeric coefficients)
pub(crate) fn compute_content_impl(expr: &Expression) -> Expression {
    let coefficients = extract_numeric_coefficients(expr);
    if coefficients.is_empty() {
        return Expression::integer(1);
    }

    // Compute GCD of all coefficients
    let mut gcd = coefficients[0].abs();
    for &coef in coefficients.iter().skip(1) {
        gcd = integer_gcd(gcd, coef.abs());
        if gcd == 1 {
            return Expression::integer(1);
        }
    }

    Expression::integer(gcd)
}

/// Extract all numeric coefficients from an expression
fn extract_numeric_coefficients(expr: &Expression) -> Vec<i64> {
    let mut coefficients = Vec::new();
    extract_coefficients_impl(expr, &mut coefficients);
    coefficients
}

fn extract_coefficients_impl(expr: &Expression, coefficients: &mut Vec<i64>) {
    match expr {
        Expression::Number(Number::Integer(n)) => {
            coefficients.push(*n);
        }
        Expression::Symbol(_) => {
            // Symbol has implicit coefficient 1
            coefficients.push(1);
        }
        Expression::Add(terms) => {
            for term in terms.iter() {
                extract_coefficients_impl(term, coefficients);
            }
        }
        Expression::Mul(factors) => {
            // Extract numeric factor from product
            let mut numeric_coef = 1i64;
            let mut found_numeric = false;
            for factor in factors.iter() {
                if let Expression::Number(Number::Integer(n)) = factor {
                    numeric_coef *= n;
                    found_numeric = true;
                }
            }
            if found_numeric {
                coefficients.push(numeric_coef);
            } else {
                coefficients.push(1);
            }
        }
        Expression::Pow(base, _) => {
            // Power has coefficient 1 unless base has coefficient
            if let Expression::Mul(factors) = base.as_ref() {
                for factor in factors.iter() {
                    if let Expression::Number(Number::Integer(n)) = factor {
                        coefficients.push(*n);
                        return;
                    }
                }
            }
            coefficients.push(1);
        }
        _ => {
            coefficients.push(1);
        }
    }
}

/// Divide polynomial by integer content
pub(crate) fn divide_by_integer(expr: &Expression, divisor: &Expression) -> Expression {
    let div_val = match divisor {
        Expression::Number(Number::Integer(n)) if *n != 0 => *n,
        _ => return expr.clone(),
    };

    if div_val == 1 {
        return expr.clone();
    }

    divide_expr_by_integer(expr, div_val)
}

fn divide_expr_by_integer(expr: &Expression, divisor: i64) -> Expression {
    match expr {
        Expression::Number(Number::Integer(n)) => {
            if n % divisor == 0 {
                Expression::integer(n / divisor)
            } else {
                // Keep as rational or just return original
                expr.clone()
            }
        }
        Expression::Add(terms) => {
            let divided_terms: Vec<Expression> = terms
                .iter()
                .map(|t| divide_expr_by_integer(t, divisor))
                .collect();
            Expression::add(divided_terms)
        }
        Expression::Mul(factors) => {
            // Divide only the numeric factor
            let mut divided = false;
            let new_factors: Vec<Expression> = factors
                .iter()
                .map(|f| {
                    if !divided {
                        if let Expression::Number(Number::Integer(n)) = f {
                            if n % divisor == 0 {
                                divided = true;
                                return Expression::integer(n / divisor);
                            }
                        }
                    }
                    f.clone()
                })
                .collect();
            Expression::mul(new_factors)
        }
        _ => expr.clone(),
    }
}
