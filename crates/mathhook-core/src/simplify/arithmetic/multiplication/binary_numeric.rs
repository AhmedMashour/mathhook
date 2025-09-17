//! Binary numeric multiplication simplification
//!
//! Optimized fast paths for two-factor numeric multiplication with overflow protection

use super::super::addition::simplify_addition;
use super::super::power::simplify_power;
use crate::core::{Expression, Number};
use num_bigint::BigInt;
use num_rational::BigRational;
use num_traits::ToPrimitive;

/// Try to simplify two-factor multiplication with numeric fast paths
///
/// Handles special cases for numeric multiplication with overflow protection:
/// - Integer * Integer (with checked arithmetic)
/// - Rational * Integer (promoting to BigInt on overflow)
/// - Float * Float
/// - Division via power: a * b^(-1) = a/b
/// - Power rule: x^a * x^b → x^(a+b)
///
/// # Returns
/// Some(simplified) if numeric simplification possible, None otherwise
pub fn try_simplify_binary(factor1: &Expression, factor2: &Expression) -> Option<Expression> {
    match (factor1, factor2) {
        (Expression::Number(Number::Integer(a)), Expression::Number(Number::Integer(b))) => {
            if let Some(result) = a.checked_mul(*b) {
                Some(Expression::integer(result))
            } else {
                Some(Expression::Number(Number::rational(BigRational::new(
                    BigInt::from(*a) * BigInt::from(*b),
                    BigInt::from(1),
                ))))
            }
        }
        (Expression::Number(Number::Rational(r)), Expression::Number(Number::Integer(b))) => {
            if r.denom() == &BigInt::from(1) {
                if let Some(a) = r.numer().to_i64() {
                    if let Some(result) = a.checked_mul(*b) {
                        return Some(Expression::integer(result));
                    } else {
                        return Some(Expression::Number(Number::rational(BigRational::new(
                            r.numer() * BigInt::from(*b),
                            BigInt::from(1),
                        ))));
                    }
                }
            }
            None
        }
        (Expression::Number(Number::Integer(a)), Expression::Number(Number::Rational(r))) => {
            if r.denom() == &BigInt::from(1) {
                if let Some(b) = r.numer().to_i64() {
                    if let Some(result) = a.checked_mul(b) {
                        return Some(Expression::integer(result));
                    } else {
                        return Some(Expression::Number(Number::rational(BigRational::new(
                            BigInt::from(*a) * r.numer(),
                            BigInt::from(1),
                        ))));
                    }
                }
            }
            None
        }
        (Expression::Number(Number::Rational(r1)), Expression::Number(Number::Rational(r2))) => {
            if r1.denom() == &BigInt::from(1) && r2.denom() == &BigInt::from(1) {
                if let (Some(a), Some(b)) = (r1.numer().to_i64(), r2.numer().to_i64()) {
                    if let Some(result) = a.checked_mul(b) {
                        return Some(Expression::integer(result));
                    } else {
                        return Some(Expression::Number(Number::rational(BigRational::new(
                            r1.numer() * r2.numer(),
                            BigInt::from(1),
                        ))));
                    }
                }
            }
            None
        }
        (Expression::Number(Number::Float(a)), Expression::Number(Number::Float(b))) => {
            Some(Expression::Number(Number::float(a * b)))
        }
        (Expression::Number(Number::Integer(a)), Expression::Pow(base, exp)) => {
            // Check if exponent is -1 (either as Integer or Rational(-1/1))
            let is_minus_one = match exp.as_ref() {
                Expression::Number(Number::Integer(-1)) => true,
                Expression::Number(Number::Rational(r)) => {
                    r.denom() == &BigInt::from(1) && r.numer() == &BigInt::from(-1)
                }
                _ => false,
            };
            if is_minus_one {
                if let Expression::Number(Number::Integer(b)) = base.as_ref() {
                    let result = BigRational::new(BigInt::from(*a), BigInt::from(*b));
                    // Normalize to Integer if denominator is 1
                    if result.denom() == &BigInt::from(1) {
                        if let Some(int_val) = result.numer().to_i64() {
                            return Some(Expression::integer(int_val));
                        }
                    }
                    return Some(Expression::Number(Number::rational(result)));
                }
            }
            None
        }
        (Expression::Pow(base, exp), Expression::Number(Number::Integer(a))) => {
            // Check if exponent is -1 (either as Integer or Rational(-1/1))
            let is_minus_one = match exp.as_ref() {
                Expression::Number(Number::Integer(-1)) => true,
                Expression::Number(Number::Rational(r)) => {
                    r.denom() == &BigInt::from(1) && r.numer() == &BigInt::from(-1)
                }
                _ => false,
            };
            if is_minus_one {
                if let Expression::Number(Number::Integer(b)) = base.as_ref() {
                    let result = BigRational::new(BigInt::from(*a), BigInt::from(*b));
                    // Normalize to Integer if denominator is 1
                    if result.denom() == &BigInt::from(1) {
                        if let Some(int_val) = result.numer().to_i64() {
                            return Some(Expression::integer(int_val));
                        }
                    }
                    return Some(Expression::Number(Number::rational(result)));
                }
            }
            None
        }
        (Expression::Number(Number::Rational(r1)), Expression::Pow(base, exp)) => {
            // Check if exponent is -1 (either as Integer or Rational(-1/1))
            let is_minus_one = match exp.as_ref() {
                Expression::Number(Number::Integer(-1)) => true,
                Expression::Number(Number::Rational(r)) => {
                    r.denom() == &BigInt::from(1) && r.numer() == &BigInt::from(-1)
                }
                _ => false,
            };
            if is_minus_one {
                if let Expression::Number(Number::Rational(r2)) = base.as_ref() {
                    let result = r1.as_ref() / r2.as_ref();
                    return Some(Expression::Number(Number::rational(result)));
                }
            }
            None
        }
        (Expression::Pow(base, exp), Expression::Number(Number::Rational(r1))) => {
            // Check if exponent is -1 (either as Integer or Rational(-1/1))
            let is_minus_one = match exp.as_ref() {
                Expression::Number(Number::Integer(-1)) => true,
                Expression::Number(Number::Rational(r)) => {
                    r.denom() == &BigInt::from(1) && r.numer() == &BigInt::from(-1)
                }
                _ => false,
            };
            if is_minus_one {
                if let Expression::Number(Number::Rational(r2)) = base.as_ref() {
                    let result = r1.as_ref() / r2.as_ref();
                    return Some(Expression::Number(Number::rational(result)));
                }
            }
            None
        }
        (Expression::Pow(base1, exp1), Expression::Pow(base2, exp2)) => {
            if base1 == base2 {
                let combined_exp = simplify_addition(&[(**exp1).clone(), (**exp2).clone()]);
                Some(simplify_power(base1, &combined_exp))
            } else {
                None
            }
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_integer_multiplication() {
        let result = try_simplify_binary(&Expression::integer(2), &Expression::integer(3));
        assert_eq!(result, Some(Expression::integer(6)));
    }

    #[test]
    fn test_float_multiplication() {
        let result = try_simplify_binary(
            &Expression::Number(Number::float(2.0)),
            &Expression::Number(Number::float(3.0)),
        );
        assert_eq!(result, Some(Expression::Number(Number::float(6.0))));
    }
}
