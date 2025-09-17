//! Integer arithmetic operations
//!
//! Implements power operations with checked arithmetic for integer bases and exponents.
//! Uses checked arithmetic to detect overflow and promotes to BigInt when needed.

use super::types::Number;
use crate::error::MathError;
use num_bigint::BigInt;

impl Number {
    /// Power operation with overflow checking
    ///
    /// Computes self raised to the power of exponent. For integer bases and positive
    /// integer exponents, uses checked arithmetic with promotion to BigInt on overflow.
    /// For other cases, converts to float and checks for infinity/NaN.
    ///
    /// # Arguments
    ///
    /// * `exponent` - The exponent to raise self to
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::Number;
    ///
    /// let base = Number::integer(2);
    /// let exp = Number::integer(3);
    /// let result = base.pow(&exp).unwrap();
    /// assert_eq!(result, Number::integer(8));
    /// ```
    ///
    /// # Errors
    ///
    /// Returns `MathError::NumericOverflow` if:
    /// - Float power results in infinity or NaN
    /// - Exponentiation produces non-representable result
    pub fn pow(&self, exponent: &Number) -> Result<Number, MathError> {
        match (self, exponent) {
            (Number::Integer(base), Number::Integer(exp)) if *exp >= 0 => {
                if *exp > u32::MAX as i64 {
                    return Err(MathError::NumericOverflow {
                        operation: "exponent too large for integer power".to_owned(),
                    });
                }

                let exp_u32 = *exp as u32;

                if let Some(result) = Self::checked_pow_i64(*base, exp_u32) {
                    Ok(Number::Integer(result))
                } else {
                    let base_bigint = BigInt::from(*base);
                    Ok(Number::BigInteger(Box::new(num_traits::Pow::pow(
                        base_bigint,
                        exp_u32,
                    ))))
                }
            }

            (Number::BigInteger(base), Number::Integer(exp)) if *exp >= 0 => {
                if *exp > u32::MAX as i64 {
                    return Err(MathError::NumericOverflow {
                        operation: "exponent too large for BigInteger power".to_owned(),
                    });
                }
                Ok(Number::BigInteger(Box::new(num_traits::Pow::pow(
                    base.as_ref().clone(),
                    *exp as u32,
                ))))
            }

            _ => {
                let base_float = self.to_float()?;
                let exp_float = exponent.to_float()?;
                let result = base_float.powf(exp_float);

                if result.is_infinite() || result.is_nan() {
                    Err(MathError::NumericOverflow {
                        operation: "float power".to_owned(),
                    })
                } else {
                    Ok(Number::Float(result))
                }
            }
        }
    }

    /// Helper function for checked integer power
    fn checked_pow_i64(base: i64, exp: u32) -> Option<i64> {
        if exp == 0 {
            return Some(1);
        }

        let mut result = 1i64;
        let mut base = base;
        let mut exp = exp;

        while exp > 0 {
            if exp % 2 == 1 {
                result = result.checked_mul(base)?;
            }
            base = base.checked_mul(base)?;
            exp /= 2;
        }

        Some(result)
    }
}
