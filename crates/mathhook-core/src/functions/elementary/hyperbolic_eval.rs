//! Hyperbolic function evaluation

use crate::core::{Expression, Number};

/// Evaluate hyperbolic sine sinh(x)
///
/// # Mathematical Definition
///
/// sinh(x) = (e^x - e^(-x)) / 2
///
/// # Arguments
///
/// * `arg` - Expression to compute sinh of
///
/// # Returns
///
/// Hyperbolic sine expression
///
/// # Examples
///
/// ```
/// use mathhook_core::functions::elementary::hyperbolic_eval::sinh;
/// use mathhook_core::expr;
///
/// let result = sinh(&expr!(0));
/// assert_eq!(result, expr!(0));
/// ```
pub fn sinh(arg: &Expression) -> Expression {
    match arg {
        Expression::Number(Number::Integer(0)) => Expression::integer(0),
        Expression::Number(Number::Float(f)) => Expression::float(f.sinh()),
        Expression::Number(Number::Integer(i)) => Expression::float((*i as f64).sinh()),
        _ => Expression::function("sinh", vec![arg.clone()]),
    }
}

/// Evaluate hyperbolic cosine cosh(x)
///
/// # Mathematical Definition
///
/// cosh(x) = (e^x + e^(-x)) / 2
///
/// # Arguments
///
/// * `arg` - Expression to compute cosh of
///
/// # Returns
///
/// Hyperbolic cosine expression
///
/// # Examples
///
/// ```
/// use mathhook_core::functions::elementary::hyperbolic_eval::cosh;
/// use mathhook_core::expr;
///
/// let result = cosh(&expr!(0));
/// assert_eq!(result, expr!(1));
/// ```
pub fn cosh(arg: &Expression) -> Expression {
    match arg {
        Expression::Number(Number::Integer(0)) => Expression::integer(1),
        Expression::Number(Number::Float(f)) => Expression::float(f.cosh()),
        Expression::Number(Number::Integer(i)) => Expression::float((*i as f64).cosh()),
        _ => Expression::function("cosh", vec![arg.clone()]),
    }
}

/// Evaluate hyperbolic tangent tanh(x)
///
/// # Mathematical Definition
///
/// tanh(x) = sinh(x) / cosh(x) = (e^x - e^(-x)) / (e^x + e^(-x))
///
/// # Arguments
///
/// * `arg` - Expression to compute tanh of
///
/// # Returns
///
/// Hyperbolic tangent expression
///
/// # Examples
///
/// ```
/// use mathhook_core::functions::elementary::hyperbolic_eval::tanh;
/// use mathhook_core::expr;
///
/// let result = tanh(&expr!(0));
/// assert_eq!(result, expr!(0));
/// ```
pub fn tanh(arg: &Expression) -> Expression {
    match arg {
        Expression::Number(Number::Integer(0)) => Expression::integer(0),
        Expression::Number(Number::Float(f)) => Expression::float(f.tanh()),
        Expression::Number(Number::Integer(i)) => Expression::float((*i as f64).tanh()),
        _ => Expression::function("tanh", vec![arg.clone()]),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sinh_zero() {
        assert_eq!(sinh(&Expression::integer(0)), Expression::integer(0));
    }

    #[test]
    fn test_cosh_zero() {
        assert_eq!(cosh(&Expression::integer(0)), Expression::integer(1));
    }

    #[test]
    fn test_tanh_zero() {
        assert_eq!(tanh(&Expression::integer(0)), Expression::integer(0));
    }
}
