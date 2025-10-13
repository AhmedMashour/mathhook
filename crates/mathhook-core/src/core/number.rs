//! Number type for mathematical computations

use crate::error::MathError;
use num_bigint::BigInt;
use num_rational::BigRational;
use num_traits::{One, Zero};
use serde::{Deserialize, Serialize};
use std::ops::{Add, Div, Mul, Sub};

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
                Ok(Number::Float(a + b))
            }

            (Number::Integer(i), Number::Float(f)) | (Number::Float(f), Number::Integer(i)) => {
                Ok(Number::Float(i as f64 + f))
            }

            (Number::BigInteger(bi), Number::Float(f)) | (Number::Float(f), Number::BigInteger(bi)) => {
                Ok(Number::Float(bi.to_string().parse::<f64>().unwrap_or(f64::INFINITY) + f))
            }

            (Number::Rational(r), Number::Float(f)) | (Number::Float(f), Number::Rational(r)) => {
                let r_float = r.numer().to_string().parse::<f64>().unwrap_or(f64::INFINITY)
                    / r.denom().to_string().parse::<f64>().unwrap_or(1.0);
                Ok(Number::Float(r_float + f))
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
                Ok(Number::Float(a - b))
            }

            (Number::Integer(i), Number::Float(f)) => {
                Ok(Number::Float(i as f64 - f))
            }

            (Number::Float(f), Number::Integer(i)) => {
                Ok(Number::Float(f - i as f64))
            }

            (Number::BigInteger(bi), Number::Float(f)) => {
                Ok(Number::Float(bi.to_string().parse::<f64>().unwrap_or(f64::INFINITY) - f))
            }

            (Number::Float(f), Number::BigInteger(bi)) => {
                Ok(Number::Float(f - bi.to_string().parse::<f64>().unwrap_or(f64::INFINITY)))
            }

            (Number::Rational(r), Number::Float(f)) => {
                let r_float = r.numer().to_string().parse::<f64>().unwrap_or(f64::INFINITY)
                    / r.denom().to_string().parse::<f64>().unwrap_or(1.0);
                Ok(Number::Float(r_float - f))
            }

            (Number::Float(f), Number::Rational(r)) => {
                let r_float = r.numer().to_string().parse::<f64>().unwrap_or(f64::INFINITY)
                    / r.denom().to_string().parse::<f64>().unwrap_or(1.0);
                Ok(Number::Float(f - r_float))
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
                Ok(Number::Float(a * b))
            }

            (Number::Integer(i), Number::Float(f)) | (Number::Float(f), Number::Integer(i)) => {
                Ok(Number::Float(i as f64 * f))
            }

            (Number::BigInteger(bi), Number::Float(f)) | (Number::Float(f), Number::BigInteger(bi)) => {
                Ok(Number::Float(bi.to_string().parse::<f64>().unwrap_or(f64::INFINITY) * f))
            }

            (Number::Rational(r), Number::Float(f)) | (Number::Float(f), Number::Rational(r)) => {
                let r_float = r.numer().to_string().parse::<f64>().unwrap_or(f64::INFINITY)
                    / r.denom().to_string().parse::<f64>().unwrap_or(1.0);
                Ok(Number::Float(r_float * f))
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
                Ok(Number::Float(a / b))
            }

            (Number::Integer(i), Number::Float(f)) => {
                Ok(Number::Float(i as f64 / f))
            }

            (Number::Float(f), Number::Integer(i)) => {
                Ok(Number::Float(f / i as f64))
            }

            (Number::BigInteger(bi), Number::Float(f)) => {
                Ok(Number::Float(bi.to_string().parse::<f64>().unwrap_or(f64::INFINITY) / f))
            }

            (Number::Float(f), Number::BigInteger(bi)) => {
                Ok(Number::Float(f / bi.to_string().parse::<f64>().unwrap_or(f64::INFINITY)))
            }

            (Number::Rational(r), Number::Float(f)) => {
                let r_float = r.numer().to_string().parse::<f64>().unwrap_or(f64::INFINITY)
                    / r.denom().to_string().parse::<f64>().unwrap_or(1.0);
                Ok(Number::Float(r_float / f))
            }

            (Number::Float(f), Number::Rational(r)) => {
                let r_float = r.numer().to_string().parse::<f64>().unwrap_or(f64::INFINITY)
                    / r.denom().to_string().parse::<f64>().unwrap_or(1.0);
                Ok(Number::Float(f / r_float))
            }
        }
    }
}
