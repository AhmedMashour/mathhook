//! Inverse trigonometric function evaluation

use crate::core::{Expression, Number};

pub fn arcsin(arg: &Expression) -> Expression {
    match arg {
        Expression::Number(Number::Integer(0)) => Expression::integer(0),
        Expression::Number(Number::Integer(1)) => {
            Expression::mul(vec![Expression::rational(1, 2), Expression::pi()])
        }
        Expression::Number(Number::Integer(-1)) => {
            Expression::mul(vec![Expression::rational(-1, 2), Expression::pi()])
        }
        Expression::Number(Number::Float(f)) if f.abs() <= 1.0 => Expression::float(f.asin()),
        Expression::Number(Number::Integer(i)) => {
            let f = *i as f64;
            if f.abs() <= 1.0 {
                Expression::float(f.asin())
            } else {
                Expression::function("arcsin", vec![arg.clone()])
            }
        }
        _ => Expression::function("arcsin", vec![arg.clone()]),
    }
}

/// Evaluate arccosine function
///
/// # Mathematical Definition
///
/// arccos(x) = cos⁻¹(x), domain: [-1, 1], range: [0, π]
///
/// # Arguments
///
/// * `arg` - Expression to compute arccosine of
///
/// # Returns
///
/// Arccosine expression
///
/// # Examples
///
/// ```
/// use mathhook_core::functions::elementary::trigonometric::arccos;
/// use mathhook_core::expr;
///
/// let result = arccos(&expr!(1));
/// assert_eq!(result, expr!(0));
/// ```
pub fn arccos(arg: &Expression) -> Expression {
    match arg {
        Expression::Number(Number::Integer(1)) => Expression::integer(0),
        Expression::Number(Number::Integer(0)) => {
            Expression::mul(vec![Expression::rational(1, 2), Expression::pi()])
        }
        Expression::Number(Number::Integer(-1)) => Expression::pi(),
        Expression::Number(Number::Float(f)) if f.abs() <= 1.0 => Expression::float(f.acos()),
        Expression::Number(Number::Integer(i)) => {
            let f = *i as f64;
            if f.abs() <= 1.0 {
                Expression::float(f.acos())
            } else {
                Expression::function("arccos", vec![arg.clone()])
            }
        }
        _ => Expression::function("arccos", vec![arg.clone()]),
    }
}

/// Evaluate arctangent function
///
/// # Mathematical Definition
///
/// arctan(x) = tan⁻¹(x), domain: ℝ, range: (-π/2, π/2)
///
/// # Arguments
///
/// * `arg` - Expression to compute arctangent of
///
/// # Returns
///
/// Arctangent expression
///
/// # Examples
///
/// ```
/// use mathhook_core::functions::elementary::trigonometric::arctan;
/// use mathhook_core::expr;
///
/// let result = arctan(&expr!(0));
/// assert_eq!(result, expr!(0));
/// ```
pub fn arctan(arg: &Expression) -> Expression {
    match arg {
        Expression::Number(Number::Integer(0)) => Expression::integer(0),
        Expression::Number(Number::Integer(1)) => {
            Expression::mul(vec![Expression::rational(1, 4), Expression::pi()])
        }
        Expression::Number(Number::Integer(-1)) => {
            Expression::mul(vec![Expression::rational(-1, 4), Expression::pi()])
        }
        Expression::Number(Number::Float(f)) => Expression::float(f.atan()),
        Expression::Number(Number::Integer(i)) => Expression::float((*i as f64).atan()),
        _ => Expression::function("arctan", vec![arg.clone()]),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_arcsin_zero() {
        assert_eq!(arcsin(&Expression::integer(0)), Expression::integer(0));
    }

    #[test]
    fn test_arcsin_one() {
        let result = arcsin(&Expression::integer(1));
        assert_eq!(
            result,
            Expression::mul(vec![Expression::rational(1, 2), Expression::pi()])
        );
    }

    #[test]
    fn test_arccos_one() {
        assert_eq!(arccos(&Expression::integer(1)), Expression::integer(0));
    }

    #[test]
    fn test_arccos_zero() {
        let result = arccos(&Expression::integer(0));
        assert_eq!(
            result,
            Expression::mul(vec![Expression::rational(1, 2), Expression::pi()])
        );
    }

    #[test]
    fn test_arctan_zero() {
        assert_eq!(arctan(&Expression::integer(0)), Expression::integer(0));
    }

    #[test]
    fn test_arctan_one() {
        let result = arctan(&Expression::integer(1));
        assert_eq!(
            result,
            Expression::mul(vec![Expression::rational(1, 4), Expression::pi()])
        );
    }
}
