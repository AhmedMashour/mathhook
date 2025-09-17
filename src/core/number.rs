//! High-performance number representation supporting multiple numeric types

use serde::{Deserialize, Serialize};
use std::fmt;
use num_bigint::BigInt;
use num_rational::BigRational;
use num_traits::{Zero, One, ToPrimitive};

/// Represents different types of numbers in the algebra system
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Number {
    /// 64-bit signed integer for fast arithmetic
    Integer(BigInt),
    /// Rational number (fraction) for exact arithmetic
    Rational(BigRational),
    /// 64-bit floating point for approximate calculations
    Float(f64),
}

impl Number {
    /// Create a new integer
    pub fn integer<T: Into<BigInt>>(value: T) -> Self {
        Self::Integer(value.into())
    }
    
    /// Create a new rational number
    pub fn rational<T: Into<BigRational>>(value: T) -> Self {
        Self::Rational(value.into())
    }
    
    /// Create a new float
    pub fn float(value: f64) -> Self {
        Self::Float(value)
    }
    
    /// Check if the number is zero
    pub fn is_zero(&self) -> bool {
        match self {
            Number::Integer(i) => i.is_zero(),
            Number::Rational(r) => r.is_zero(),
            Number::Float(f) => *f == 0.0,
        }
    }
    
    /// Check if the number is one
    pub fn is_one(&self) -> bool {
        match self {
            Number::Integer(i) => i.is_one(),
            Number::Rational(r) => r.is_one(),
            Number::Float(f) => *f == 1.0,
        }
    }
    
    /// Convert to f64 if possible
    pub fn to_f64(&self) -> Option<f64> {
        match self {
            Number::Integer(i) => i.to_f64(),
            Number::Rational(r) => r.to_f64(),
            Number::Float(f) => Some(*f),
        }
    }
    
    /// Convert to i64 if possible
    pub fn to_i64(&self) -> Option<i64> {
        match self {
            Number::Integer(i) => i.to_i64(),
            Number::Rational(r) => {
                if r.denom().is_one() {
                    r.numer().to_i64()
                } else {
                    None
                }
            },
            Number::Float(f) => {
                if f.fract() == 0.0 && *f >= i64::MIN as f64 && *f <= i64::MAX as f64 {
                    Some(*f as i64)
                } else {
                    None
                }
            }
        }
    }
}

impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Number::Integer(i) => write!(f, "{}", i),
            Number::Rational(r) => {
                if r.denom().is_one() {
                    write!(f, "{}", r.numer())
                } else {
                    write!(f, "{}/{}", r.numer(), r.denom())
                }
            },
            Number::Float(fl) => write!(f, "{}", fl),
        }
    }
}

impl From<i32> for Number {
    fn from(value: i32) -> Self {
        Self::Integer(BigInt::from(value))
    }
}

impl From<i64> for Number {
    fn from(value: i64) -> Self {
        Self::Integer(BigInt::from(value))
    }
}

impl From<BigInt> for Number {
    fn from(value: BigInt) -> Self {
        Self::Integer(value)
    }
}

impl From<BigRational> for Number {
    fn from(value: BigRational) -> Self {
        Self::Rational(value)
    }
}

impl From<f64> for Number {
    fn from(value: f64) -> Self {
        Self::Float(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_number_creation() {
        let int_num = Number::integer(42);
        let rat_num = Number::rational(BigRational::new(BigInt::from(3), BigInt::from(4)));
        let float_num = Number::float(3.14);
        
        assert!(matches!(int_num, Number::Integer(_)));
        assert!(matches!(rat_num, Number::Rational(_)));
        assert!(matches!(float_num, Number::Float(_)));
    }
    
    #[test]
    fn test_zero_and_one() {
        let zero = Number::integer(0);
        let one = Number::integer(1);
        let two = Number::integer(2);
        
        assert!(zero.is_zero());
        assert!(!zero.is_one());
        assert!(one.is_one());
        assert!(!one.is_zero());
        assert!(!two.is_zero());
        assert!(!two.is_one());
    }
    
    #[test]
    fn test_conversions() {
        let int_num = Number::integer(42);
        assert_eq!(int_num.to_i64(), Some(42));
        assert_eq!(int_num.to_f64(), Some(42.0));
        
        let float_num = Number::float(3.14);
        assert_eq!(float_num.to_f64(), Some(3.14));
        assert_eq!(float_num.to_i64(), None);
    }
}
