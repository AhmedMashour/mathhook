//! Helper functions for residue calculus
//!
//! Utility functions for expression analysis and pattern matching.

use crate::core::Expression;

/// Extract the denominator from a rational expression
///
/// Handles various forms:
/// - `a * b^(-1)` → denominator is `b`
/// - `a * b^(-n)` → denominator is `b^n`
/// - `a^(-n)` → denominator is `a^n`
///
/// # Arguments
///
/// * `expr` - The expression to analyze
///
/// # Returns
///
/// `Some(denominator)` if the expression is a rational function, `None` otherwise
pub fn extract_denominator(expr: &Expression) -> Option<Expression> {
    match expr {
        // Handle a * b^(-1) or a * b^(-n)
        Expression::Mul(factors) => {
            let mut denominator_factors = Vec::new();

            for factor in factors.iter() {
                if let Expression::Pow(base, exp) = factor {
                    if let Expression::Number(crate::core::Number::Integer(exp_val)) = exp.as_ref()
                    {
                        if *exp_val < 0 {
                            // This is a denominator term: b^(-n) → b^n in denominator
                            let positive_exp = Expression::integer(-exp_val);
                            denominator_factors
                                .push(Expression::pow(base.as_ref().clone(), positive_exp));
                        }
                    }
                }
            }

            if !denominator_factors.is_empty() {
                Some(Expression::mul(denominator_factors))
            } else {
                None
            }
        }
        // Handle direct a^(-n) form
        Expression::Pow(base, exp) => {
            if let Expression::Number(crate::core::Number::Integer(exp_val)) = exp.as_ref() {
                if *exp_val < 0 {
                    let positive_exp = Expression::integer(-exp_val);
                    return Some(Expression::pow(base.as_ref().clone(), positive_exp));
                }
            }
            None
        }
        _ => None,
    }
}

/// Check if an expression evaluates to zero
///
/// # Arguments
///
/// * `expr` - The expression to check
///
/// # Returns
///
/// `true` if the expression is zero (or simplifies to zero), `false` otherwise
pub fn is_expression_zero(expr: &Expression) -> bool {
    match expr {
        Expression::Number(n) => n.is_zero(),
        // Multiplication with any zero factor is zero
        Expression::Mul(factors) => factors.iter().any(is_expression_zero),
        // Addition is zero if all terms are zero
        Expression::Add(terms) => terms.iter().all(is_expression_zero),
        _ => false,
    }
}

/// Check if an expression represents infinity
///
/// # Arguments
///
/// * `expr` - The expression to check
///
/// # Returns
///
/// `true` if the expression represents infinity (positive or negative), `false` otherwise
pub fn is_infinity(expr: &Expression) -> bool {
    match expr {
        Expression::Function { name, .. }
            if name.as_ref() == "infinity" || name.as_ref() == "oo" =>
        {
            true
        }
        // Check for negative infinity: -1 * infinity
        Expression::Mul(factors) if factors.len() == 2 => {
            matches!(&factors[0], Expression::Number(n) if *n == crate::core::Number::Integer(-1))
                && is_infinity(&factors[1])
        }
        _ => false,
    }
}

/// Check if an expression is undefined
///
/// # Arguments
///
/// * `expr` - The expression to check
///
/// # Returns
///
/// `true` if the expression represents an undefined value, `false` otherwise
pub fn is_undefined(expr: &Expression) -> bool {
    matches!(expr, Expression::Function { name, .. } if name.as_ref() == "undefined" || name.as_ref() == "nan")
}

/// Check if an expression is finite (not infinity, not undefined)
///
/// # Arguments
///
/// * `expr` - The expression to check
///
/// # Returns
///
/// `true` if the expression is finite, `false` otherwise
pub fn is_finite(expr: &Expression) -> bool {
    !is_infinity(expr) && !is_undefined(expr)
}
