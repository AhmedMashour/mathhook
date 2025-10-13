//! Number type for mathematical computations

use crate::error::MathError;
use num_bigint::BigInt;
use num_rational::BigRational;
use num_traits::{One, Pow, Zero};
use serde::{Deserialize, Serialize};
use std::ops::{Add, Div, Mul, Neg, Sub};

/// Unified number type supporting integers, rationals, and floats
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Number {
    Integer(i64),
    Float(f64),
    BigInteger(Box<BigInt>),
    Rational(Box<BigRational>),
}

impl Number {
    /// Create an integer number
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::Number;
    ///
    /// let num = Number::integer(42);
    /// ```
    pub fn integer(value: i64) -> Self {
        Self::Integer(value)
    }

    /// Create a float number
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::Number;
    ///
    /// let num = Number::float(3.14);
    /// ```
    pub fn float(value: f64) -> Self {
        Self::Float(value)
    }

    /// Create a rational number
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::Number;
    /// use num_rational::BigRational;
    /// use num_bigint::BigInt;
    ///
    /// let rational = BigRational::new(BigInt::from(3), BigInt::from(4));
    /// let num = Number::rational(rational);
    /// ```
    pub fn rational(value: BigRational) -> Self {
        Self::Rational(Box::new(value))
    }

    /// Check if the number is zero
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::Number;
    ///
    /// let zero = Number::integer(0);
    /// assert!(zero.is_zero());
    /// ```
    pub fn is_zero(&self) -> bool {
        match self {
            Number::Integer(i) => *i == 0,
            Number::Float(f) => *f == 0.0,
            Number::BigInteger(bi) => **bi == BigInt::from(0),
            Number::Rational(r) => r.is_zero(),
        }
    }

    /// Check if the number is one
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::Number;
    ///
    /// let one = Number::integer(1);
    /// assert!(one.is_one());
    /// ```
    pub fn is_one(&self) -> bool {
        match self {
            Number::Integer(i) => *i == 1,
            Number::Float(f) => *f == 1.0,
            Number::BigInteger(bi) => **bi == BigInt::from(1),
            Number::Rational(r) => r.is_one(),
        }
    }

    /// Check if the number is negative one
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::Number;
    ///
    /// let neg_one = Number::integer(-1);
    /// assert!(neg_one.is_negative_one());
    /// ```
    pub fn is_negative_one(&self) -> bool {
        match self {
            Number::Integer(i) => *i == -1,
            Number::Float(f) => *f == -1.0,
            Number::BigInteger(bi) => **bi == BigInt::from(-1),
            Number::Rational(r) => **r == BigRational::new(BigInt::from(-1), BigInt::from(1)),
        }
    }

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
                        operation: "exponent too large for integer power".to_string(),
                    });
                }

                let exp_u32 = *exp as u32;

                if let Some(result) = Self::checked_pow_i64(*base, exp_u32) {
                    Ok(Number::Integer(result))
                } else {
                    let base_bigint = BigInt::from(*base);
                    Ok(Number::BigInteger(Box::new(num_traits::Pow::pow(base_bigint, exp_u32))))
                }
            }

            (Number::BigInteger(base), Number::Integer(exp)) if *exp >= 0 => {
                if *exp > u32::MAX as i64 {
                    return Err(MathError::NumericOverflow {
                        operation: "exponent too large for BigInteger power".to_string(),
                    });
                }
                Ok(Number::BigInteger(Box::new(num_traits::Pow::pow(base.as_ref().clone(), *exp as u32))))
            }

            _ => {
                let base_float = self.to_float()?;
                let exp_float = exponent.to_float()?;
                let result = base_float.powf(exp_float);

                if result.is_infinite() || result.is_nan() {
                    Err(MathError::NumericOverflow {
                        operation: "float power".to_string(),
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

    /// Convert to float with overflow checking
    fn to_float(&self) -> Result<f64, MathError> {
        match self {
            Number::Integer(i) => Ok(*i as f64),
            Number::Float(f) => Ok(*f),
            Number::BigInteger(bi) => {
                bi.to_string().parse::<f64>().map_err(|_| MathError::NumericOverflow {
                    operation: "BigInteger to float conversion".to_string(),
                })
            }
            Number::Rational(r) => {
                let numer_float = r.numer().to_string().parse::<f64>().map_err(|_| MathError::NumericOverflow {
                    operation: "Rational numerator to float conversion".to_string(),
                })?;
                let denom_float = r.denom().to_string().parse::<f64>().map_err(|_| MathError::NumericOverflow {
                    operation: "Rational denominator to float conversion".to_string(),
                })?;
                Ok(numer_float / denom_float)
            }
        }
    }
}

impl From<i64> for Number {
    fn from(value: i64) -> Self {
        Self::Integer(value)
    }
}

impl From<f64> for Number {
    fn from(value: f64) -> Self {
        Self::Float(value)
    }
}

impl From<i32> for Number {
    fn from(value: i32) -> Self {
        Self::Integer(value as i64)
    }
}

impl std::fmt::Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Number::Integer(i) => write!(f, "{}", i),
            Number::Float(fl) => write!(f, "{}", fl),
            Number::BigInteger(bi) => write!(f, "{}", bi),
            Number::Rational(r) => {
                if r.denom().is_one() {
                    write!(f, "{}", r.numer())
                } else {
                    write!(f, "{}/{}", r.numer(), r.denom())
                }
            }
        }
    }
}

/// Addition with overflow checking and promotion to BigInt
///
/// # Examples
///
/// ```rust
/// use mathhook_core::Number;
///
/// let a = Number::integer(5);
/// let b = Number::integer(3);
/// let result = (a + b).unwrap();
/// assert_eq!(result, Number::integer(8));
/// ```
impl Add for Number {
    type Output = Result<Number, MathError>;

    fn add(self, other: Number) -> Result<Number, MathError> {
        match (self, other) {
            (Number::Integer(a), Number::Integer(b)) => {
                match a.checked_add(b) {
                    Some(result) => Ok(Number::Integer(result)),
                    None => {
                        Ok(Number::BigInteger(Box::new(
                            BigInt::from(a) + BigInt::from(b)
                        )))
                    }
                }
            }

            (Number::BigInteger(a), Number::BigInteger(b)) => {
                Ok(Number::BigInteger(Box::new(*a + *b)))
            }

            (Number::Integer(i), Number::BigInteger(bi)) | (Number::BigInteger(bi), Number::Integer(i)) => {
                Ok(Number::BigInteger(Box::new(*bi + BigInt::from(i))))
            }

            (Number::Rational(a), Number::Rational(b)) => {
                Ok(Number::Rational(Box::new(*a + *b)))
            }

            (Number::Integer(i), Number::Rational(r)) | (Number::Rational(r), Number::Integer(i)) => {
                let i_rational = BigRational::from(BigInt::from(i));
                Ok(Number::Rational(Box::new(i_rational + *r)))
            }

            (Number::BigInteger(bi), Number::Rational(r)) | (Number::Rational(r), Number::BigInteger(bi)) => {
                let bi_rational = BigRational::from(*bi);
                Ok(Number::Rational(Box::new(bi_rational + *r)))
            }

            (Number::Float(a), Number::Float(b)) => {
                let result = a + b;
                if result.is_infinite() || result.is_nan() {
                    Err(MathError::NumericOverflow {
                        operation: "float addition".to_string(),
                    })
                } else {
                    Ok(Number::Float(result))
                }
            }

            (Number::Integer(i), Number::Float(f)) | (Number::Float(f), Number::Integer(i)) => {
                let result = i as f64 + f;
                if result.is_infinite() || result.is_nan() {
                    Err(MathError::NumericOverflow {
                        operation: "integer-float addition".to_string(),
                    })
                } else {
                    Ok(Number::Float(result))
                }
            }

            (Number::BigInteger(bi), Number::Float(f)) | (Number::Float(f), Number::BigInteger(bi)) => {
                let bi_float = bi.to_string().parse::<f64>().map_err(|_| MathError::NumericOverflow {
                    operation: "BigInteger to float conversion".to_string(),
                })?;
                let result = bi_float + f;
                if result.is_infinite() || result.is_nan() {
                    Err(MathError::NumericOverflow {
                        operation: "BigInteger-float addition".to_string(),
                    })
                } else {
                    Ok(Number::Float(result))
                }
            }

            (Number::Rational(r), Number::Float(f)) | (Number::Float(f), Number::Rational(r)) => {
                let numer_float = r.numer().to_string().parse::<f64>().map_err(|_| MathError::NumericOverflow {
                    operation: "Rational numerator to float conversion".to_string(),
                })?;
                let denom_float = r.denom().to_string().parse::<f64>().map_err(|_| MathError::NumericOverflow {
                    operation: "Rational denominator to float conversion".to_string(),
                })?;
                let r_float = numer_float / denom_float;
                let result = r_float + f;
                if result.is_infinite() || result.is_nan() {
                    Err(MathError::NumericOverflow {
                        operation: "Rational-float addition".to_string(),
                    })
                } else {
                    Ok(Number::Float(result))
                }
            }
        }
    }
}

/// Subtraction with overflow checking and promotion to BigInt
///
/// # Examples
///
/// ```rust
/// use mathhook_core::Number;
///
/// let a = Number::integer(10);
/// let b = Number::integer(3);
/// let result = (a - b).unwrap();
/// assert_eq!(result, Number::integer(7));
/// ```
impl Sub for Number {
    type Output = Result<Number, MathError>;

    fn sub(self, other: Number) -> Result<Number, MathError> {
        match (self, other) {
            (Number::Integer(a), Number::Integer(b)) => {
                match a.checked_sub(b) {
                    Some(result) => Ok(Number::Integer(result)),
                    None => {
                        Ok(Number::BigInteger(Box::new(
                            BigInt::from(a) - BigInt::from(b)
                        )))
                    }
                }
            }

            (Number::BigInteger(a), Number::BigInteger(b)) => {
                Ok(Number::BigInteger(Box::new(*a - *b)))
            }

            (Number::Integer(i), Number::BigInteger(bi)) => {
                Ok(Number::BigInteger(Box::new(BigInt::from(i) - *bi)))
            }

            (Number::BigInteger(bi), Number::Integer(i)) => {
                Ok(Number::BigInteger(Box::new(*bi - BigInt::from(i))))
            }

            (Number::Rational(a), Number::Rational(b)) => {
                Ok(Number::Rational(Box::new(*a - *b)))
            }

            (Number::Integer(i), Number::Rational(r)) => {
                let i_rational = BigRational::from(BigInt::from(i));
                Ok(Number::Rational(Box::new(i_rational - *r)))
            }

            (Number::Rational(r), Number::Integer(i)) => {
                let i_rational = BigRational::from(BigInt::from(i));
                Ok(Number::Rational(Box::new(*r - i_rational)))
            }

            (Number::BigInteger(bi), Number::Rational(r)) => {
                let bi_rational = BigRational::from(*bi);
                Ok(Number::Rational(Box::new(bi_rational - *r)))
            }

            (Number::Rational(r), Number::BigInteger(bi)) => {
                let bi_rational = BigRational::from(*bi);
                Ok(Number::Rational(Box::new(*r - bi_rational)))
            }

            (Number::Float(a), Number::Float(b)) => {
                let result = a - b;
                if result.is_infinite() || result.is_nan() {
                    Err(MathError::NumericOverflow {
                        operation: "float subtraction".to_string(),
                    })
                } else {
                    Ok(Number::Float(result))
                }
            }

            (Number::Integer(i), Number::Float(f)) => {
                let result = i as f64 - f;
                if result.is_infinite() || result.is_nan() {
                    Err(MathError::NumericOverflow {
                        operation: "integer-float subtraction".to_string(),
                    })
                } else {
                    Ok(Number::Float(result))
                }
            }

            (Number::Float(f), Number::Integer(i)) => {
                let result = f - i as f64;
                if result.is_infinite() || result.is_nan() {
                    Err(MathError::NumericOverflow {
                        operation: "float-integer subtraction".to_string(),
                    })
                } else {
                    Ok(Number::Float(result))
                }
            }

            (Number::BigInteger(bi), Number::Float(f)) => {
                let bi_float = bi.to_string().parse::<f64>().map_err(|_| MathError::NumericOverflow {
                    operation: "BigInteger to float conversion".to_string(),
                })?;
                let result = bi_float - f;
                if result.is_infinite() || result.is_nan() {
                    Err(MathError::NumericOverflow {
                        operation: "BigInteger-float subtraction".to_string(),
                    })
                } else {
                    Ok(Number::Float(result))
                }
            }

            (Number::Float(f), Number::BigInteger(bi)) => {
                let bi_float = bi.to_string().parse::<f64>().map_err(|_| MathError::NumericOverflow {
                    operation: "BigInteger to float conversion".to_string(),
                })?;
                let result = f - bi_float;
                if result.is_infinite() || result.is_nan() {
                    Err(MathError::NumericOverflow {
                        operation: "float-BigInteger subtraction".to_string(),
                    })
                } else {
                    Ok(Number::Float(result))
                }
            }

            (Number::Rational(r), Number::Float(f)) => {
                let numer_float = r.numer().to_string().parse::<f64>().map_err(|_| MathError::NumericOverflow {
                    operation: "Rational numerator to float conversion".to_string(),
                })?;
                let denom_float = r.denom().to_string().parse::<f64>().map_err(|_| MathError::NumericOverflow {
                    operation: "Rational denominator to float conversion".to_string(),
                })?;
                let r_float = numer_float / denom_float;
                let result = r_float - f;
                if result.is_infinite() || result.is_nan() {
                    Err(MathError::NumericOverflow {
                        operation: "Rational-float subtraction".to_string(),
                    })
                } else {
                    Ok(Number::Float(result))
                }
            }

            (Number::Float(f), Number::Rational(r)) => {
                let numer_float = r.numer().to_string().parse::<f64>().map_err(|_| MathError::NumericOverflow {
                    operation: "Rational numerator to float conversion".to_string(),
                })?;
                let denom_float = r.denom().to_string().parse::<f64>().map_err(|_| MathError::NumericOverflow {
                    operation: "Rational denominator to float conversion".to_string(),
                })?;
                let r_float = numer_float / denom_float;
                let result = f - r_float;
                if result.is_infinite() || result.is_nan() {
                    Err(MathError::NumericOverflow {
                        operation: "float-Rational subtraction".to_string(),
                    })
                } else {
                    Ok(Number::Float(result))
                }
            }
        }
    }
}

/// Multiplication with overflow checking and promotion to BigInt
///
/// # Examples
///
/// ```rust
/// use mathhook_core::Number;
///
/// let a = Number::integer(6);
/// let b = Number::integer(7);
/// let result = (a * b).unwrap();
/// assert_eq!(result, Number::integer(42));
/// ```
impl Mul for Number {
    type Output = Result<Number, MathError>;

    fn mul(self, other: Number) -> Result<Number, MathError> {
        match (self, other) {
            (Number::Integer(a), Number::Integer(b)) => {
                match a.checked_mul(b) {
                    Some(result) => Ok(Number::Integer(result)),
                    None => {
                        Ok(Number::BigInteger(Box::new(
                            BigInt::from(a) * BigInt::from(b)
                        )))
                    }
                }
            }

            (Number::BigInteger(a), Number::BigInteger(b)) => {
                Ok(Number::BigInteger(Box::new(*a * *b)))
            }

            (Number::Integer(i), Number::BigInteger(bi)) | (Number::BigInteger(bi), Number::Integer(i)) => {
                Ok(Number::BigInteger(Box::new(*bi * BigInt::from(i))))
            }

            (Number::Rational(a), Number::Rational(b)) => {
                Ok(Number::Rational(Box::new(*a * *b)))
            }

            (Number::Integer(i), Number::Rational(r)) | (Number::Rational(r), Number::Integer(i)) => {
                let i_rational = BigRational::from(BigInt::from(i));
                Ok(Number::Rational(Box::new(i_rational * *r)))
            }

            (Number::BigInteger(bi), Number::Rational(r)) | (Number::Rational(r), Number::BigInteger(bi)) => {
                let bi_rational = BigRational::from(*bi);
                Ok(Number::Rational(Box::new(bi_rational * *r)))
            }

            (Number::Float(a), Number::Float(b)) => {
                let result = a * b;
                if result.is_infinite() || result.is_nan() {
                    Err(MathError::NumericOverflow {
                        operation: "float multiplication".to_string(),
                    })
                } else {
                    Ok(Number::Float(result))
                }
            }

            (Number::Integer(i), Number::Float(f)) | (Number::Float(f), Number::Integer(i)) => {
                let result = i as f64 * f;
                if result.is_infinite() || result.is_nan() {
                    Err(MathError::NumericOverflow {
                        operation: "integer-float multiplication".to_string(),
                    })
                } else {
                    Ok(Number::Float(result))
                }
            }

            (Number::BigInteger(bi), Number::Float(f)) | (Number::Float(f), Number::BigInteger(bi)) => {
                let bi_float = bi.to_string().parse::<f64>().map_err(|_| MathError::NumericOverflow {
                    operation: "BigInteger to float conversion".to_string(),
                })?;
                let result = bi_float * f;
                if result.is_infinite() || result.is_nan() {
                    Err(MathError::NumericOverflow {
                        operation: "BigInteger-float multiplication".to_string(),
                    })
                } else {
                    Ok(Number::Float(result))
                }
            }

            (Number::Rational(r), Number::Float(f)) | (Number::Float(f), Number::Rational(r)) => {
                let numer_float = r.numer().to_string().parse::<f64>().map_err(|_| MathError::NumericOverflow {
                    operation: "Rational numerator to float conversion".to_string(),
                })?;
                let denom_float = r.denom().to_string().parse::<f64>().map_err(|_| MathError::NumericOverflow {
                    operation: "Rational denominator to float conversion".to_string(),
                })?;
                let r_float = numer_float / denom_float;
                let result = r_float * f;
                if result.is_infinite() || result.is_nan() {
                    Err(MathError::NumericOverflow {
                        operation: "Rational-float multiplication".to_string(),
                    })
                } else {
                    Ok(Number::Float(result))
                }
            }
        }
    }
}

/// Division with division by zero check and automatic promotion to rational
///
/// # Examples
///
/// ```rust
/// use mathhook_core::Number;
///
/// let a = Number::integer(10);
/// let b = Number::integer(2);
/// let result = (a / b).unwrap();
/// assert_eq!(result, Number::integer(5));
/// ```
impl Div for Number {
    type Output = Result<Number, MathError>;

    fn div(self, other: Number) -> Result<Number, MathError> {
        if other.is_zero() {
            return Err(MathError::DivisionByZero);
        }

        match (self, other) {
            (Number::Integer(a), Number::Integer(b)) => {
                if a % b == 0 {
                    Ok(Number::Integer(a / b))
                } else {
                    Ok(Number::Rational(Box::new(
                        BigRational::new(BigInt::from(a), BigInt::from(b))
                    )))
                }
            }

            (Number::BigInteger(a), Number::BigInteger(b)) => {
                if (*a).clone() % (*b).clone() == BigInt::from(0) {
                    Ok(Number::BigInteger(Box::new(*a / *b)))
                } else {
                    Ok(Number::Rational(Box::new(BigRational::new(*a, *b))))
                }
            }

            (Number::Integer(i), Number::BigInteger(bi)) => {
                Ok(Number::Rational(Box::new(
                    BigRational::new(BigInt::from(i), *bi)
                )))
            }

            (Number::BigInteger(bi), Number::Integer(i)) => {
                Ok(Number::Rational(Box::new(
                    BigRational::new(*bi, BigInt::from(i))
                )))
            }

            (Number::Rational(a), Number::Rational(b)) => {
                Ok(Number::Rational(Box::new(*a / *b)))
            }

            (Number::Integer(i), Number::Rational(r)) => {
                let i_rational = BigRational::from(BigInt::from(i));
                Ok(Number::Rational(Box::new(i_rational / *r)))
            }

            (Number::Rational(r), Number::Integer(i)) => {
                let i_rational = BigRational::from(BigInt::from(i));
                Ok(Number::Rational(Box::new(*r / i_rational)))
            }

            (Number::BigInteger(bi), Number::Rational(r)) => {
                let bi_rational = BigRational::from(*bi);
                Ok(Number::Rational(Box::new(bi_rational / *r)))
            }

            (Number::Rational(r), Number::BigInteger(bi)) => {
                let bi_rational = BigRational::from(*bi);
                Ok(Number::Rational(Box::new(*r / bi_rational)))
            }

            (Number::Float(a), Number::Float(b)) => {
                let result = a / b;
                if result.is_infinite() || result.is_nan() {
                    Err(MathError::NumericOverflow {
                        operation: "float division".to_string(),
                    })
                } else {
                    Ok(Number::Float(result))
                }
            }

            (Number::Integer(i), Number::Float(f)) => {
                let result = i as f64 / f;
                if result.is_infinite() || result.is_nan() {
                    Err(MathError::NumericOverflow {
                        operation: "integer-float division".to_string(),
                    })
                } else {
                    Ok(Number::Float(result))
                }
            }

            (Number::Float(f), Number::Integer(i)) => {
                let result = f / i as f64;
                if result.is_infinite() || result.is_nan() {
                    Err(MathError::NumericOverflow {
                        operation: "float-integer division".to_string(),
                    })
                } else {
                    Ok(Number::Float(result))
                }
            }

            (Number::BigInteger(bi), Number::Float(f)) => {
                let bi_float = bi.to_string().parse::<f64>().map_err(|_| MathError::NumericOverflow {
                    operation: "BigInteger to float conversion".to_string(),
                })?;
                let result = bi_float / f;
                if result.is_infinite() || result.is_nan() {
                    Err(MathError::NumericOverflow {
                        operation: "BigInteger-float division".to_string(),
                    })
                } else {
                    Ok(Number::Float(result))
                }
            }

            (Number::Float(f), Number::BigInteger(bi)) => {
                let bi_float = bi.to_string().parse::<f64>().map_err(|_| MathError::NumericOverflow {
                    operation: "BigInteger to float conversion".to_string(),
                })?;
                let result = f / bi_float;
                if result.is_infinite() || result.is_nan() {
                    Err(MathError::NumericOverflow {
                        operation: "float-BigInteger division".to_string(),
                    })
                } else {
                    Ok(Number::Float(result))
                }
            }

            (Number::Rational(r), Number::Float(f)) => {
                let numer_float = r.numer().to_string().parse::<f64>().map_err(|_| MathError::NumericOverflow {
                    operation: "Rational numerator to float conversion".to_string(),
                })?;
                let denom_float = r.denom().to_string().parse::<f64>().map_err(|_| MathError::NumericOverflow {
                    operation: "Rational denominator to float conversion".to_string(),
                })?;
                let r_float = numer_float / denom_float;
                let result = r_float / f;
                if result.is_infinite() || result.is_nan() {
                    Err(MathError::NumericOverflow {
                        operation: "Rational-float division".to_string(),
                    })
                } else {
                    Ok(Number::Float(result))
                }
            }

            (Number::Float(f), Number::Rational(r)) => {
                let numer_float = r.numer().to_string().parse::<f64>().map_err(|_| MathError::NumericOverflow {
                    operation: "Rational numerator to float conversion".to_string(),
                })?;
                let denom_float = r.denom().to_string().parse::<f64>().map_err(|_| MathError::NumericOverflow {
                    operation: "Rational denominator to float conversion".to_string(),
                })?;
                let r_float = numer_float / denom_float;
                let result = f / r_float;
                if result.is_infinite() || result.is_nan() {
                    Err(MathError::NumericOverflow {
                        operation: "float-Rational division".to_string(),
                    })
                } else {
                    Ok(Number::Float(result))
                }
            }
        }
    }
}

/// Negation with overflow checking
///
/// # Examples
///
/// ```rust
/// use mathhook_core::Number;
///
/// let a = Number::integer(5);
/// let result = (-a).unwrap();
/// assert_eq!(result, Number::integer(-5));
/// ```
impl Neg for Number {
    type Output = Result<Number, MathError>;

    fn neg(self) -> Result<Number, MathError> {
        match self {
            Number::Integer(i) => {
                match i.checked_neg() {
                    Some(result) => Ok(Number::Integer(result)),
                    None => {
                        Ok(Number::BigInteger(Box::new(-BigInt::from(i))))
                    }
                }
            }

            Number::BigInteger(bi) => {
                Ok(Number::BigInteger(Box::new(-*bi)))
            }

            Number::Float(f) => {
                Ok(Number::Float(-f))
            }

            Number::Rational(r) => {
                Ok(Number::Rational(Box::new(-*r)))
            }
        }
    }
}
