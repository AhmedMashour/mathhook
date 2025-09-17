//! Absolute value function evaluation

use crate::core::{Expression, Number};
use num_traits::Signed;

/// Evaluate absolute value function
///
/// # Mathematical Definition
///
/// |x| = { x   if x ≥ 0
///      { -x  if x < 0
///
/// # Arguments
///
/// * `arg` - Expression to compute absolute value of
///
/// # Returns
///
/// Absolute value expression
///
/// # Examples
///
/// ```
/// use mathhook_core::functions::elementary::abs_eval::abs;
/// use mathhook_core::expr;
///
/// let result = abs(&expr!(-5));
/// assert_eq!(result, expr!(5));
/// ```
pub fn abs(arg: &Expression) -> Expression {
    match arg {
        Expression::Number(n) => evaluate_abs_number(n),
        _ => Expression::function("abs", vec![arg.clone()]),
    }
}

fn evaluate_abs_number(n: &Number) -> Expression {
    match n {
        Number::Integer(i) => Expression::integer(i.abs()),
        Number::Float(f) => Expression::float(f.abs()),
        Number::BigInteger(bi) => Expression::big_integer(bi.abs()),
        Number::Rational(r) => Expression::Number(Number::rational(
            num_rational::BigRational::new(r.numer().abs(), r.denom().clone()),
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_abs_zero() {
        assert_eq!(abs(&Expression::integer(0)), Expression::integer(0));
    }

    #[test]
    fn test_abs_positive() {
        assert_eq!(abs(&Expression::integer(5)), Expression::integer(5));
    }

    #[test]
    fn test_abs_negative() {
        assert_eq!(abs(&Expression::integer(-5)), Expression::integer(5));
    }

    #[test]
    fn test_abs_float() {
        assert_eq!(abs(&Expression::float(-3.4)), Expression::float(3.4));
    }
}
