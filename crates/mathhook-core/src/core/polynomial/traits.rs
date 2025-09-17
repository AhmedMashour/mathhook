//! Algebraic trait hierarchy for polynomial coefficient types
//!
//! Defines Ring, EuclideanDomain, and Field traits to enable generic polynomial arithmetic.

use num_rational::Ratio;
use std::fmt::Debug;
use std::ops::{Add, Div, Mul, Neg, Rem, Sub};

/// Ring: addition, subtraction, multiplication with identity elements
///
/// A ring is an algebraic structure with two binary operations (+ and *)
/// satisfying these properties:
/// - Additive identity: exists 0 such that a + 0 = a
/// - Multiplicative identity: exists 1 such that a * 1 = a
/// - Additive inverse: for all a, exists -a such that a + (-a) = 0
/// - Associative: (a + b) + c = a + (b + c), (a * b) * c = a * (b * c)
/// - Distributive: a * (b + c) = a * b + a * c
pub trait Ring:
    Sized
    + Clone
    + PartialEq
    + Debug
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Neg<Output = Self>
{
    fn zero() -> Self;
    fn one() -> Self;
    fn is_zero(&self) -> bool;
    fn is_one(&self) -> bool;

    #[inline(always)]
    fn wrapping_add(&self, other: &Self) -> Self {
        self.clone() + other.clone()
    }

    #[inline(always)]
    fn wrapping_sub(&self, other: &Self) -> Self {
        self.clone() - other.clone()
    }

    #[inline(always)]
    fn wrapping_mul(&self, other: &Self) -> Self {
        self.clone() * other.clone()
    }
}

/// Euclidean domain: ring with division algorithm
///
/// A Euclidean domain is a ring where division with remainder is defined.
/// Examples: integers Z, polynomials `K[x]` over field K, Gaussian integers `Z[i]`
///
/// Key property: For any a, b ≠ 0, exists q, r such that a = q*b + r
/// where either r = 0 or deg(r) < deg(b)
pub trait EuclideanDomain: Ring + Div<Output = Self> + Rem<Output = Self> {
    /// Division with remainder: returns (quotient, remainder)
    ///
    /// # Example
    /// ```ignore
    /// let (q, r) = a.div_rem(&b);
    /// assert_eq!(a, &(&q * &b) + &r);
    /// ```
    fn div_rem(&self, other: &Self) -> (Self, Self);

    /// Greatest common divisor using Euclidean algorithm
    ///
    /// # Example
    /// ```ignore
    /// assert_eq!(gcd(12, 18), 6);
    /// ```
    fn gcd(&self, other: &Self) -> Self {
        let mut a = self.clone();
        let mut b = other.clone();
        while !b.is_zero() {
            let (_, r) = a.div_rem(&b);
            a = b;
            b = r;
        }
        a
    }

    /// Absolute value (for content extraction and normalization)
    fn abs(&self) -> Self;
}

/// Field: Euclidean domain where every non-zero element has multiplicative inverse
///
/// Examples: rational numbers Q, real numbers R, finite fields Z_p (p prime)
///
/// Key property: For any a ≠ 0, exists a⁻¹ such that a * a⁻¹ = 1
pub trait Field: EuclideanDomain {
    /// Multiplicative inverse
    ///
    /// Returns None if element is zero
    fn inv(&self) -> Option<Self>;
}

impl Ring for i64 {
    #[inline(always)]
    fn zero() -> Self {
        0
    }

    #[inline(always)]
    fn one() -> Self {
        1
    }

    #[inline(always)]
    fn is_zero(&self) -> bool {
        *self == 0
    }

    #[inline(always)]
    fn is_one(&self) -> bool {
        *self == 1
    }

    #[inline(always)]
    fn wrapping_add(&self, other: &Self) -> Self {
        i64::wrapping_add(*self, *other)
    }

    #[inline(always)]
    fn wrapping_sub(&self, other: &Self) -> Self {
        i64::wrapping_sub(*self, *other)
    }

    #[inline(always)]
    fn wrapping_mul(&self, other: &Self) -> Self {
        i64::wrapping_mul(*self, *other)
    }
}

impl EuclideanDomain for i64 {
    #[inline(always)]
    fn div_rem(&self, other: &Self) -> (Self, Self) {
        (*self / *other, *self % *other)
    }

    #[inline(always)]
    fn abs(&self) -> Self {
        i64::abs(*self)
    }
}

impl Ring for Ratio<i64> {
    #[inline(always)]
    fn zero() -> Self {
        Ratio::new(0, 1)
    }

    #[inline(always)]
    fn one() -> Self {
        Ratio::new(1, 1)
    }

    #[inline(always)]
    fn is_zero(&self) -> bool {
        self.numer() == &0
    }

    #[inline(always)]
    fn is_one(&self) -> bool {
        self.numer() == &1 && self.denom() == &1
    }
}

impl EuclideanDomain for Ratio<i64> {
    #[inline(always)]
    fn div_rem(&self, other: &Self) -> (Self, Self) {
        (self / other, Ratio::zero())
    }

    #[inline(always)]
    fn abs(&self) -> Self {
        Ratio::new(self.numer().abs(), *self.denom())
    }
}

impl Field for Ratio<i64> {
    #[inline]
    fn inv(&self) -> Option<Self> {
        if self.is_zero() {
            None
        } else {
            Some(self.recip())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_i64_ring_properties() {
        assert_eq!(i64::zero(), 0);
        assert_eq!(i64::one(), 1);
        assert!(0i64.is_zero());
        assert!(1i64.is_one());
        assert!(!5i64.is_zero());
        assert!(!5i64.is_one());
    }

    #[test]
    fn test_i64_euclidean_domain() {
        let (q, r) = 17i64.div_rem(&5);
        assert_eq!(q, 3);
        assert_eq!(r, 2);
        assert_eq!(17, 3 * 5 + 2);
    }

    #[test]
    fn test_i64_gcd() {
        assert_eq!(12i64.gcd(&18), 6);
        assert_eq!(17i64.gcd(&19), 1);
        assert_eq!(0i64.gcd(&5), 5);
        assert_eq!(5i64.gcd(&0), 5);
    }

    #[test]
    fn test_i64_abs() {
        assert_eq!((-5i64).abs(), 5);
        assert_eq!(5i64.abs(), 5);
        assert_eq!(0i64.abs(), 0);
    }

    #[test]
    fn test_wrapping_arithmetic() {
        let a = i64::MAX;
        let b = 1i64;
        let sum = Ring::wrapping_add(&a, &b);
        assert_eq!(sum, i64::MIN);
    }

    #[test]
    fn test_ratio_ring_properties() {
        assert_eq!(Ratio::<i64>::zero(), Ratio::new(0, 1));
        assert_eq!(Ratio::<i64>::one(), Ratio::new(1, 1));
        assert!(Ratio::new(0, 1).is_zero());
        assert!(Ratio::new(1, 1).is_one());
        assert!(!Ratio::new(5, 1).is_zero());
        assert!(!Ratio::new(5, 1).is_one());
    }

    #[test]
    fn test_ratio_arithmetic() {
        let a = Ratio::new(1, 2);
        let b = Ratio::new(1, 3);
        let sum = a + b;
        assert_eq!(sum, Ratio::new(5, 6));

        let diff = a - b;
        assert_eq!(diff, Ratio::new(1, 6));

        let prod = a * b;
        assert_eq!(prod, Ratio::new(1, 6));

        let quot = a / b;
        assert_eq!(quot, Ratio::new(3, 2));
    }

    #[test]
    fn test_ratio_euclidean_domain() {
        let a = Ratio::new(5, 2);
        let b = Ratio::new(3, 4);
        let (q, r) = a.div_rem(&b);
        assert_eq!(q, Ratio::new(10, 3));
        assert_eq!(r, Ratio::zero());
    }

    #[test]
    fn test_ratio_abs() {
        assert_eq!(Ratio::new(-5, 2).abs(), Ratio::new(5, 2));
        assert_eq!(Ratio::new(5, 2).abs(), Ratio::new(5, 2));
        assert_eq!(Ratio::new(0, 1).abs(), Ratio::new(0, 1));
    }

    #[test]
    fn test_ratio_field() {
        let a = Ratio::new(2, 3);
        let inv = a.inv().unwrap();
        assert_eq!(inv, Ratio::new(3, 2));
        assert_eq!(a * inv, Ratio::one());

        assert!(Ratio::zero().inv().is_none());
    }
}
