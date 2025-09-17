//! Helper functions for Risch algorithm
//!
//! Common utilities used across Risch implementation.

use crate::core::{Expression, Number, Symbol};

/// Check if expression is just the variable
///
/// Returns true if the expression is exactly the given symbol variable.
///
/// # Arguments
///
/// * `expr` - The expression to check
/// * `var` - The variable symbol to compare against
///
/// # Examples
///
/// ```rust
/// use mathhook_core::calculus::integrals::risch::helpers::is_just_variable;
/// use mathhook_core::{Expression, symbol};
///
/// let x = symbol!(x);
/// let expr = Expression::symbol(x.clone());
/// assert!(is_just_variable(&expr, &x));
/// ```
pub fn is_just_variable(expr: &Expression, var: &Symbol) -> bool {
    matches!(expr, Expression::Symbol(s) if *s == *var)
}

/// Check if expression is the constant 1
///
/// # Arguments
///
/// * `expr` - The expression to check
///
/// # Examples
///
/// ```rust
/// use mathhook_core::calculus::integrals::risch::helpers::is_one;
/// use mathhook_core::Expression;
///
/// let one = Expression::integer(1);
/// assert!(is_one(&one));
///
/// let two = Expression::integer(2);
/// assert!(!is_one(&two));
/// ```
pub fn is_one(expr: &Expression) -> bool {
    matches!(expr, Expression::Number(n) if n.is_one())
}

/// Extract division pattern from expression
///
/// Division can be represented as:
/// 1. Mul([numerator, Pow(denominator, -1)]) for general divisions like 3/x
/// 2. Pow(denominator, -1) for 1/x (the 1 is simplified away)
///
/// Returns Some((numerator, denominator)) if pattern matches.
pub fn extract_division(expr: &Expression) -> Option<(Expression, Expression)> {
    match expr {
        // Case 1: General division num/den represented as Mul([num, den^(-1)])
        Expression::Mul(factors) if factors.len() == 2 => {
            // Check if second factor is denominator^(-1)
            if let Expression::Pow(base, exp) = &factors[1] {
                if let Expression::Number(Number::Integer(-1)) = &**exp {
                    return Some((factors[0].clone(), (**base).clone()));
                }
            }
            // Check if first factor is denominator^(-1)
            if let Expression::Pow(base, exp) = &factors[0] {
                if let Expression::Number(Number::Integer(-1)) = &**exp {
                    return Some((factors[1].clone(), (**base).clone()));
                }
            }
            None
        }
        // Case 2: 1/den represented as den^(-1) (the 1 is simplified away)
        Expression::Pow(base, exp) => {
            if let Expression::Number(Number::Integer(-1)) = &**exp {
                Some((Expression::integer(1), (**base).clone()))
            } else {
                None
            }
        }
        _ => None,
    }
}
