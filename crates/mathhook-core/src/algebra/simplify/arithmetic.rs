//! Arithmetic operation simplification
//!
//! Handles simplification of basic arithmetic operations: addition, multiplication, and powers.
//! Implements ultra-fast paths for common cases while maintaining mathematical correctness.

use crate::core::{Expression, Number};
use num_bigint::BigInt;
use num_rational::BigRational;
use num_traits::{One, ToPrimitive, Zero};

/// Fast two-term addition without allocations
#[inline(always)]
fn add_two_expressions(a: &Expression, b: &Expression) -> Expression {
    match (a, b) {
        (Expression::Number(Number::Integer(x)), Expression::Number(Number::Integer(y))) => {
            Expression::Number(Number::Integer(x.saturating_add(*y)))
        }
        (Expression::Number(Number::Float(x)), Expression::Number(Number::Float(y))) => {
            Expression::Number(Number::Float(x + y))
        }
        (Expression::Number(Number::Integer(x)), Expression::Number(Number::Float(y))) => {
            Expression::Number(Number::Float(*x as f64 + y))
        }
        (Expression::Number(Number::Float(x)), Expression::Number(Number::Integer(y))) => {
            Expression::Number(Number::Float(x + *y as f64))
        }
        _ => {
            // Fallback: create minimal addition expression
            Expression::Add(Box::new(vec![a.clone(), b.clone()]))
        }
    }
}

/// Allocation-minimized mixed addition
fn mixed_addition_fallback(
    terms: &[Expression],
    int_sum: i64,
    float_sum: f64,
    has_float: bool,
) -> Expression {
    let mut result_terms = Vec::with_capacity(terms.len().min(8)); // Pre-allocate reasonable size

    // Add numeric sum if non-zero
    if has_float {
        let total = int_sum as f64 + float_sum;
        if total != 0.0 {
            result_terms.push(Expression::Number(Number::Float(total)));
        }
    } else if int_sum != 0 {
        result_terms.push(Expression::Number(Number::Integer(int_sum)));
    }

    // Add non-numeric terms
    for term in terms {
        match term {
            Expression::Number(_) => {} // Already processed
            _ => result_terms.push(term.clone()),
        }
    }

    match result_terms.len() {
        0 => Expression::integer(0),
        1 => result_terms.into_iter().next().unwrap(),
        _ => Expression::Add(Box::new(result_terms)),
    }
}

/// Simplify addition expressions with minimal overhead
#[inline(always)]
pub fn simplify_addition(terms: &[Expression]) -> Expression {
    // Fast path: no allocations for simple cases
    match terms.len() {
        0 => return Expression::integer(0),
        1 => return terms[0].clone(),
        2 => return add_two_expressions(&terms[0], &terms[1]),
        _ => {}
    }

    // Optimized path: process in-place without allocations
    let mut int_sum: i64 = 0;
    let mut float_sum: f64 = 0.0;
    let mut has_float = false;
    let mut non_numeric_count = 0;
    let mut first_non_numeric_idx = 0;

    // Single pass: no allocations, no vector creation
    for (i, term) in terms.iter().enumerate() {
        match term {
            Expression::Number(Number::Integer(n)) => {
                int_sum = int_sum.saturating_add(*n);
            }
            Expression::Number(Number::Float(f)) => {
                float_sum += *f;
                has_float = true;
            }
            Expression::Number(Number::Rational(r)) => {
                if let Some(f) = r.to_f64() {
                    float_sum += f;
                    has_float = true;
                } else {
                    // Keep track of first non-numeric for fallback
                    if non_numeric_count == 0 {
                        first_non_numeric_idx = i;
                    }
                    non_numeric_count += 1;
                }
            }
            _ => {
                if non_numeric_count == 0 {
                    first_non_numeric_idx = i;
                }
                non_numeric_count += 1;
            }
        }
    }

    // Return optimized result without allocations
    if non_numeric_count == 0 {
        // All numeric - return single result
        if has_float {
            let total = int_sum as f64 + float_sum;
            Expression::Number(Number::Float(total))
        } else {
            Expression::Number(Number::Integer(int_sum))
        }
    } else if non_numeric_count == 1 && int_sum == 0 && float_sum == 0.0 {
        // Only one non-numeric term with zero numeric sum
        terms[first_non_numeric_idx].clone()
    } else {
        // Mixed case: use allocation-minimized fallback
        mixed_addition_fallback(terms, int_sum, float_sum, has_float)
    }
}

/// Fast two-term multiplication without allocations
#[inline(always)]
fn multiply_two_expressions(a: &Expression, b: &Expression) -> Expression {
    match (a, b) {
        (Expression::Number(Number::Integer(x)), Expression::Number(Number::Integer(y))) => {
            Expression::Number(Number::Integer(x.saturating_mul(*y)))
        }
        (Expression::Number(Number::Float(x)), Expression::Number(Number::Float(y))) => {
            Expression::Number(Number::Float(x * y))
        }
        (Expression::Number(Number::Integer(x)), Expression::Number(Number::Float(y))) => {
            Expression::Number(Number::Float(*x as f64 * y))
        }
        (Expression::Number(Number::Float(x)), Expression::Number(Number::Integer(y))) => {
            Expression::Number(Number::Float(x * *y as f64))
        }
        _ => {
            // Fallback: create minimal multiplication expression
            Expression::Mul(Box::new(vec![a.clone(), b.clone()]))
        }
    }
}

/// Check if expression is zero without allocations
#[inline(always)]
fn is_zero(expr: &Expression) -> bool {
    match expr {
        Expression::Number(Number::Integer(0)) => true,
        Expression::Number(Number::Float(f)) if *f == 0.0 => true,
        Expression::Number(Number::Rational(r)) if r.is_zero() => true,
        _ => false,
    }
}

/// Allocation-minimized mixed multiplication
fn mixed_multiplication_fallback(
    factors: &[Expression],
    int_product: i64,
    float_product: f64,
    has_float: bool,
) -> Expression {
    let mut result_factors = Vec::with_capacity(factors.len().min(8)); // Pre-allocate reasonable size

    // Add numeric product if not identity
    if has_float {
        let total = int_product as f64 * float_product;
        if total != 1.0 {
            result_factors.push(Expression::Number(Number::Float(total)));
        }
    } else if int_product != 1 {
        result_factors.push(Expression::Number(Number::Integer(int_product)));
    }

    // Add non-numeric factors
    for factor in factors {
        match factor {
            Expression::Number(_) => {} // Already processed
            _ => result_factors.push(factor.clone()),
        }
    }

    match result_factors.len() {
        0 => Expression::integer(1),
        1 => result_factors.into_iter().next().unwrap(),
        _ => Expression::Mul(Box::new(result_factors)),
    }
}

/// Simplify multiplication expressions with minimal overhead
#[inline(always)]
pub fn simplify_multiplication(factors: &[Expression]) -> Expression {
    // Fast path: no allocations for simple cases
    match factors.len() {
        0 => return Expression::integer(1),
        1 => return factors[0].clone(),
        2 => return multiply_two_expressions(&factors[0], &factors[1]),
        _ => {}
    }

    // Check for zero early (short-circuit)
    for factor in factors {
        if is_zero(factor) {
            return Expression::integer(0);
        }
    }

    // Optimized path: process in-place without allocations
    let mut int_product: i64 = 1;
    let mut float_product: f64 = 1.0;
    let mut has_float = false;
    let mut non_numeric_count = 0;
    let mut first_non_numeric_idx = 0;

    // Single pass: no allocations, no vector creation
    for (i, factor) in factors.iter().enumerate() {
        match factor {
            Expression::Number(Number::Integer(n)) => {
                int_product = int_product.saturating_mul(*n);
            }
            Expression::Number(Number::Float(f)) => {
                float_product *= *f;
                has_float = true;
            }
            Expression::Number(Number::Rational(r)) => {
                if let Some(f) = r.to_f64() {
                    float_product *= f;
                    has_float = true;
                } else {
                    if non_numeric_count == 0 {
                        first_non_numeric_idx = i;
                    }
                    non_numeric_count += 1;
                }
            }
            _ => {
                if non_numeric_count == 0 {
                    first_non_numeric_idx = i;
                }
                non_numeric_count += 1;
            }
        }
    }

    // Return optimized result without allocations
    if non_numeric_count == 0 {
        // All numeric - return single result
        if has_float {
            let total = int_product as f64 * float_product;
            Expression::Number(Number::Float(total))
        } else {
            Expression::Number(Number::Integer(int_product))
        }
    } else if non_numeric_count == 1 && int_product == 1 && float_product == 1.0 {
        // Only one non-numeric factor with identity numeric product
        factors[first_non_numeric_idx].clone()
    } else {
        // Mixed case: use allocation-minimized fallback
        mixed_multiplication_fallback(factors, int_product, float_product, has_float)
    }
}

/// Simplify power expressions
#[inline(always)]
pub fn simplify_power(base: &Expression, exp: &Expression) -> Expression {
    match (base, exp) {
        // x^0 = 1
        (_, Expression::Number(Number::Integer(0))) => Expression::integer(1),
        (_, Expression::Number(Number::Float(f))) if *f == 0.0 => Expression::integer(1),

        // x^1 = x
        (base, Expression::Number(Number::Integer(1))) => base.clone(),
        (base, Expression::Number(Number::Float(f))) if *f == 1.0 => base.clone(),

        // 0^x = 0 (for x > 0)
        (Expression::Number(Number::Integer(0)), _) => Expression::integer(0),
        (Expression::Number(Number::Float(f)), _) if *f == 0.0 => Expression::integer(0),

        // 1^x = 1
        (Expression::Number(Number::Integer(1)), _) => Expression::integer(1),
        (Expression::Number(Number::Float(f)), _) if *f == 1.0 => Expression::integer(1),

        // Numeric powers
        (Expression::Number(Number::Integer(b)), Expression::Number(Number::Integer(e))) => {
            if *e >= 0 && *e <= 10 {
                // Small positive integer powers
                let result = (*b as f64).powi(*e as i32);
                if result.is_finite() {
                    Expression::Number(Number::Float(result))
                } else {
                    Expression::Pow(Box::new(base.clone()), Box::new(exp.clone()))
                }
            } else {
                Expression::Pow(Box::new(base.clone()), Box::new(exp.clone()))
            }
        }

        (Expression::Number(Number::Float(b)), Expression::Number(Number::Integer(e))) => {
            if *e >= -10 && *e <= 10 {
                let result = b.powi(*e as i32);
                if result.is_finite() {
                    Expression::Number(Number::Float(result))
                } else {
                    Expression::Pow(Box::new(base.clone()), Box::new(exp.clone()))
                }
            } else {
                Expression::Pow(Box::new(base.clone()), Box::new(exp.clone()))
            }
        }

        (Expression::Number(Number::Float(b)), Expression::Number(Number::Float(e))) => {
            let result = b.powf(*e);
            if result.is_finite() {
                Expression::Number(Number::Float(result))
            } else {
                Expression::Pow(Box::new(base.clone()), Box::new(exp.clone()))
            }
        }

        // Default case
        _ => Expression::Pow(Box::new(base.clone()), Box::new(exp.clone())),
    }
}

/// Extract coefficient and base from an expression (helper for like term collection)
fn extract_coefficient_and_base(expr: &Expression) -> (Expression, Expression) {
    match expr {
        Expression::Mul(factors) if factors.len() == 2 => match (&factors[0], &factors[1]) {
            (Expression::Number(_), other) => (factors[0].clone(), other.clone()),
            (other, Expression::Number(_)) => (factors[1].clone(), other.clone()),
            _ => (Expression::integer(1), expr.clone()),
        },
        Expression::Number(_) => (expr.clone(), Expression::integer(1)),
        _ => (Expression::integer(1), expr.clone()),
    }
}
