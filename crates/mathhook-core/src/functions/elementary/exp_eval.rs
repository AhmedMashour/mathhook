//! Exponential function evaluation

use crate::core::{Expression, Number};

/// Evaluate exponential function e^x
///
/// # Mathematical Definition
///
/// exp(x) = e^x where e ≈ 2.71828...
///
/// # Arguments
///
/// * `arg` - Exponent expression
///
/// # Returns
///
/// Exponential expression
///
/// # Examples
///
/// ```
/// use mathhook_core::functions::elementary::exp_eval::exp;
/// use mathhook_core::expr;
///
/// let result = exp(&expr!(0));
/// assert_eq!(result, expr!(1));
/// ```
pub fn exp(arg: &Expression) -> Expression {
    match arg {
        Expression::Number(Number::Integer(0)) => Expression::integer(1),
        Expression::Number(Number::Float(f)) => Expression::float(f.exp()),
        Expression::Number(Number::Integer(i)) => Expression::float((*i as f64).exp()),
        _ => Expression::function("exp", vec![arg.clone()]),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exp_zero() {
        assert_eq!(exp(&Expression::integer(0)), Expression::integer(1));
    }

    #[test]
    fn test_exp_one() {
        let result = exp(&Expression::integer(1));
        if let Expression::Number(Number::Float(f)) = result {
            assert!((f - std::f64::consts::E).abs() < 1e-10);
        } else {
            panic!("Expected float result");
        }
    }
}
