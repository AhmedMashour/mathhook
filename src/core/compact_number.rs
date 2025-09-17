//! Memory-optimized Number enum using Box<T> for large variants
//! Reduces size from 128 bytes to 16 bytes for better cache performance

use serde::{Deserialize, Serialize};
use std::fmt;
use num_bigint::BigInt;
use num_rational::BigRational;
use num_traits::{Zero, One, ToPrimitive};

/// Memory-optimized number representation
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CompactNumber {
    /// Small integers stored inline
    SmallInt(i64),
    /// Large integers boxed to reduce enum size
    BigInteger(Box<BigInt>),
    /// Rational numbers boxed to reduce enum size
    Rational(Box<BigRational>),
    /// Floating point numbers
    Float(f64),
}

impl CompactNumber {
    /// Create a new integer, choosing optimal representation
    pub fn integer<T: Into<BigInt>>(value: T) -> Self {
        let big_int = value.into();
        if let Some(small) = big_int.to_i64() {
            Self::SmallInt(small)
        } else {
            Self::BigInteger(Box::new(big_int))
        }
    }
    
    /// Create a new rational number
    pub fn rational<T: Into<BigRational>>(value: T) -> Self {
        Self::Rational(Box::new(value.into()))
    }
    
    /// Create a new float
    pub fn float(value: f64) -> Self {
        Self::Float(value)
    }
    
    /// Check if the number is zero
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        match self {
            CompactNumber::SmallInt(i) => *i == 0,
            CompactNumber::BigInteger(i) => i.is_zero(),
            CompactNumber::Rational(r) => r.is_zero(),
            CompactNumber::Float(f) => *f == 0.0,
        }
    }
    
    /// Check if the number is one
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        match self {
            CompactNumber::SmallInt(i) => *i == 1,
            CompactNumber::BigInteger(i) => i.is_one(),
            CompactNumber::Rational(r) => r.is_one(),
            CompactNumber::Float(f) => *f == 1.0,
        }
    }
    
    /// Convert to f64 if possible
    pub fn to_f64(&self) -> Option<f64> {
        match self {
            CompactNumber::SmallInt(i) => Some(*i as f64),
            CompactNumber::BigInteger(i) => i.to_f64(),
            CompactNumber::Rational(r) => r.to_f64(),
            CompactNumber::Float(f) => Some(*f),
        }
    }
    
    /// Convert to i64 if possible
    pub fn to_i64(&self) -> Option<i64> {
        match self {
            CompactNumber::SmallInt(i) => Some(*i),
            CompactNumber::BigInteger(i) => i.to_i64(),
            CompactNumber::Rational(r) => {
                if r.denom().is_one() {
                    r.numer().to_i64()
                } else {
                    None
                }
            },
            CompactNumber::Float(f) => {
                if f.fract() == 0.0 && *f >= i64::MIN as f64 && *f <= i64::MAX as f64 {
                    Some(*f as i64)
                } else {
                    None
                }
            }
        }
    }
    
    /// Fast arithmetic operations for hot paths
    #[inline(always)]
    pub fn fast_add(&self, other: &Self) -> Option<Self> {
        match (self, other) {
            (CompactNumber::SmallInt(a), CompactNumber::SmallInt(b)) => {
                a.checked_add(*b).map(CompactNumber::SmallInt)
            },
            (CompactNumber::Float(a), CompactNumber::Float(b)) => {
                Some(CompactNumber::Float(a + b))
            },
            _ => None,
        }
    }
    
    /// Fast multiplication for hot paths
    #[inline(always)]
    pub fn fast_mul(&self, other: &Self) -> Option<Self> {
        match (self, other) {
            (CompactNumber::SmallInt(a), CompactNumber::SmallInt(b)) => {
                a.checked_mul(*b).map(CompactNumber::SmallInt)
            },
            (CompactNumber::Float(a), CompactNumber::Float(b)) => {
                Some(CompactNumber::Float(a * b))
            },
            _ => None,
        }
    }
}

impl fmt::Display for CompactNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CompactNumber::SmallInt(i) => write!(f, "{}", i),
            CompactNumber::BigInteger(i) => write!(f, "{}", i),
            CompactNumber::Rational(r) => {
                if r.denom().is_one() {
                    write!(f, "{}", r.numer())
                } else {
                    write!(f, "{}/{}", r.numer(), r.denom())
                }
            },
            CompactNumber::Float(fl) => write!(f, "{}", fl),
        }
    }
}

impl From<i32> for CompactNumber {
    fn from(value: i32) -> Self {
        Self::SmallInt(value as i64)
    }
}

impl From<i64> for CompactNumber {
    fn from(value: i64) -> Self {
        Self::SmallInt(value)
    }
}

impl From<BigInt> for CompactNumber {
    fn from(value: BigInt) -> Self {
        Self::integer(value)
    }
}

impl From<BigRational> for CompactNumber {
    fn from(value: BigRational) -> Self {
        Self::rational(value)
    }
}

impl From<f64> for CompactNumber {
    fn from(value: f64) -> Self {
        Self::Float(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compact_number_size() {
        // Verify the compact representation is actually smaller
        println!("CompactNumber size: {} bytes", std::mem::size_of::<CompactNumber>());
        assert!(std::mem::size_of::<CompactNumber>() <= 16);
    }
    
    #[test]
    fn test_small_int_optimization() {
        let small = CompactNumber::integer(42);
        assert!(matches!(small, CompactNumber::SmallInt(42)));
        
        let large = CompactNumber::integer(BigInt::from(i64::MAX) + BigInt::from(1));
        assert!(matches!(large, CompactNumber::BigInteger(_)));
    }
    
    #[test]
    fn test_fast_arithmetic() {
        let a = CompactNumber::SmallInt(10);
        let b = CompactNumber::SmallInt(20);
        
        let sum = a.fast_add(&b).unwrap();
        assert_eq!(sum, CompactNumber::SmallInt(30));
        
        let product = a.fast_mul(&b).unwrap();
        assert_eq!(product, CompactNumber::SmallInt(200));
    }
    
    #[test]
    fn test_compact_arithmetic_performance() {
        use std::time::Instant;
        
        let start = Instant::now();
        let mut result = CompactNumber::SmallInt(0);
        
        // Perform 1 million operations
        for i in 0..1_000_000 {
            let num = CompactNumber::SmallInt(i);
            if let Some(sum) = result.fast_add(&num) {
                result = sum;
            }
        }
        
        let duration = start.elapsed();
        let ops_per_sec = 1_000_000.0 / duration.as_secs_f64();
        
        println!("CompactNumber performance: {:.2} ops/sec", ops_per_sec);
        
        // Should achieve high performance
        assert!(ops_per_sec > 10_000_000.0, "Expected >10M ops/sec, got {:.2}", ops_per_sec);
    }
}
