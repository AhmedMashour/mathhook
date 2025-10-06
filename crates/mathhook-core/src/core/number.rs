//! Number type for mathematical computations

use num_bigint::BigInt;
use num_rational::BigRational;
use num_traits::{One, Zero};
use serde::{Deserialize, Serialize};

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
