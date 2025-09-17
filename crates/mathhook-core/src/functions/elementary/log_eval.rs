//! Logarithm function evaluation

use crate::core::{Expression, MathConstant, Number};

/// Evaluate natural logarithm ln(x)
///
/// # Mathematical Definition
///
/// ln(x) = log_e(x), inverse of exp(x)
///
/// # Arguments
///
/// * `arg` - Expression to compute logarithm of
///
/// # Returns
///
/// Logarithm expression
///
/// # Examples
///
/// ```
/// use mathhook_core::functions::elementary::log_eval::ln;
/// use mathhook_core::expr;
///
/// let result = ln(&expr!(1));
/// assert_eq!(result, expr!(0));
/// ```
pub fn ln(arg: &Expression) -> Expression {
    match arg {
        Expression::Constant(MathConstant::E) => Expression::integer(1),
        Expression::Number(Number::Integer(1)) => Expression::integer(0),
        Expression::Number(Number::Float(f)) if *f > 0.0 => Expression::float(f.ln()),
        Expression::Number(Number::Integer(i)) if *i > 0 => Expression::float((*i as f64).ln()),
        _ => Expression::function("ln", vec![arg.clone()]),
    }
}

/// Evaluate base-10 logarithm log10(x)
///
/// # Mathematical Definition
///
/// log10(x) = log_10(x)
///
/// # Arguments
///
/// * `arg` - Expression to compute logarithm of
///
/// # Returns
///
/// Logarithm expression
///
/// # Examples
///
/// ```
/// use mathhook_core::functions::elementary::log_eval::log10;
/// use mathhook_core::expr;
///
/// let result = log10(&expr!(10));
/// assert_eq!(result, expr!(1));
/// ```
pub fn log10(arg: &Expression) -> Expression {
    match arg {
        Expression::Number(Number::Integer(1)) => Expression::integer(0),
        Expression::Number(Number::Integer(10)) => Expression::integer(1),
        Expression::Number(Number::Float(f)) if *f > 0.0 => Expression::float(f.log10()),
        Expression::Number(Number::Integer(i)) if *i > 0 => Expression::float((*i as f64).log10()),
        _ => Expression::function("log10", vec![arg.clone()]),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ln_one() {
        assert_eq!(ln(&Expression::integer(1)), Expression::integer(0));
    }

    #[test]
    fn test_log10_one() {
        assert_eq!(log10(&Expression::integer(1)), Expression::integer(0));
    }

    #[test]
    fn test_log10_ten() {
        assert_eq!(log10(&Expression::integer(10)), Expression::integer(1));
    }
}
