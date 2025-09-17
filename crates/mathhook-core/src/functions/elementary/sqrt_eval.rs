//! Square root function evaluation

use crate::core::constants::EPSILON;
use crate::core::{Expression, Number};

/// Evaluate square root function
///
/// # Mathematical Definition
///
/// √x = x^(1/2)
///
/// # Arguments
///
/// * `arg` - Expression to compute square root of
///
/// # Returns
///
/// Square root expression
///
/// # Examples
///
/// ```
/// use mathhook_core::functions::elementary::sqrt_eval::sqrt;
/// use mathhook_core::expr;
///
/// let result = sqrt(&expr!(4));
/// assert_eq!(result, expr!(2));
/// ```
pub fn sqrt(arg: &Expression) -> Expression {
    match arg {
        Expression::Number(n) => evaluate_sqrt_number(n),
        _ => Expression::function("sqrt", vec![arg.clone()]),
    }
}

fn evaluate_sqrt_number(n: &Number) -> Expression {
    match n {
        Number::Integer(i) if *i >= 0 => {
            let sqrt_val = (*i as f64).sqrt();
            if sqrt_val.fract().abs() < EPSILON {
                Expression::integer(sqrt_val as i64)
            } else {
                Expression::float(sqrt_val)
            }
        }
        Number::Integer(i) if *i < 0 => {
            let abs_val = i.abs();
            let sqrt_abs = (abs_val as f64).sqrt();
            if sqrt_abs.fract().abs() < EPSILON {
                let sqrt_int = sqrt_abs as i64;
                if sqrt_int == 1 {
                    Expression::i()
                } else {
                    Expression::mul(vec![Expression::integer(sqrt_int), Expression::i()])
                }
            } else {
                Expression::mul(vec![Expression::float(sqrt_abs), Expression::i()])
            }
        }
        Number::Float(f) if *f >= 0.0 => Expression::float(f.sqrt()),
        Number::Float(f) if *f < 0.0 => {
            let sqrt_abs = f.abs().sqrt();
            Expression::mul(vec![Expression::float(sqrt_abs), Expression::i()])
        }
        _ => Expression::function("sqrt", vec![Expression::Number(n.clone())]),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::SQRT_2;

    #[test]
    fn test_sqrt_zero() {
        assert_eq!(sqrt(&Expression::integer(0)), Expression::integer(0));
    }

    #[test]
    fn test_sqrt_perfect_square() {
        assert_eq!(sqrt(&Expression::integer(4)), Expression::integer(2));
        assert_eq!(sqrt(&Expression::integer(9)), Expression::integer(3));
    }

    #[test]
    fn test_sqrt_non_perfect() {
        let result = sqrt(&Expression::integer(2));
        if let Expression::Number(Number::Float(f)) = result {
            assert!((f - SQRT_2).abs() < 1e-10);
        } else {
            panic!("Expected float result");
        }
    }

    #[test]
    fn test_sqrt_negative_one() {
        assert_eq!(sqrt(&Expression::integer(-1)), Expression::i());
    }

    #[test]
    fn test_sqrt_negative_perfect_square() {
        let result = sqrt(&Expression::integer(-4));
        assert_eq!(
            result,
            Expression::mul(vec![Expression::integer(2), Expression::i()])
        );
    }

    #[test]
    fn test_sqrt_negative_non_perfect() {
        let result = sqrt(&Expression::integer(-2));
        if let Expression::Mul(factors) = result {
            assert_eq!(factors.len(), 2);
            if let Expression::Number(Number::Float(f)) = &factors[0] {
                assert!((f - SQRT_2).abs() < 1e-10);
            } else {
                panic!("Expected float for sqrt(2)");
            }
            assert_eq!(factors[1], Expression::i());
        } else {
            panic!("Expected multiplication expression");
        }
    }

    #[test]
    fn test_sqrt_negative_float() {
        let result = sqrt(&Expression::float(-4.0));
        if let Expression::Mul(factors) = result {
            assert_eq!(factors.len(), 2);
            if let Expression::Number(Number::Float(f)) = &factors[0] {
                assert!((f - 2.0).abs() < 1e-10);
            } else {
                panic!("Expected float for sqrt(4.0)");
            }
            assert_eq!(factors[1], Expression::i());
        } else {
            panic!("Expected multiplication expression");
        }
    }
}
