//! Finite field element (Zp) arithmetic
//!
//! This module provides the `Zp` type representing elements of the finite field Z_p
//! (integers modulo prime p). This is foundational for modular GCD algorithms.
//!
//! # Mathematical Background
//!
//! A finite field Z_p contains integers {0, 1, 2, ..., p-1} with arithmetic modulo prime p.
//! Every non-zero element has a multiplicative inverse (Fermat's little theorem: a^(p-1) = 1 mod p).
//!
//! # Performance
//!
//! - Uses `u64` for field elements to leverage native CPU operations
//! - Struct is 16 bytes (two u64), fitting well in registers
//! - All operations use branchless conditionals where possible

use super::FiniteFieldError;
use super::FiniteFieldResult;
use std::fmt;
use std::ops::{Add, Div, Mul, Neg, Sub};

/// Element of finite field Z_p (integers modulo prime p)
///
/// This type provides efficient modular arithmetic with automatic normalization.
/// All operations maintain the invariant that `value < modulus`.
///
/// # Memory Layout
///
/// The struct is 16 bytes (two u64), fitting well in registers and cache lines.
///
/// # Thread Safety
///
/// `Zp` is `Copy`, `Send`, and `Sync`, making it safe for parallel computation.
///
/// # Examples
///
/// ```rust
/// use mathhook_core::algebra::Zp;
///
/// // Create field elements mod 7
/// let a = Zp::new(3, 7);
/// let b = Zp::new(5, 7);
///
/// // Arithmetic
/// let sum = a + b;  // 3 + 5 = 8 ≡ 1 (mod 7)
/// assert_eq!(sum.value(), 1);
///
/// let product = a * b;  // 3 * 5 = 15 ≡ 1 (mod 7)
/// assert_eq!(product.value(), 1);
/// ```
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Zp {
    value: u64,
    modulus: u64,
}

impl Zp {
    /// Create a new finite field element
    ///
    /// The value is automatically reduced modulo p.
    ///
    /// # Arguments
    ///
    /// * `value` - The integer value (will be reduced mod p)
    /// * `modulus` - The prime modulus p
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::algebra::Zp;
    ///
    /// let a = Zp::new(10, 7);
    /// assert_eq!(a.value(), 3);  // 10 mod 7 = 3
    /// ```
    #[inline]
    pub fn new(value: u64, modulus: u64) -> Self {
        debug_assert!(modulus > 1, "modulus must be > 1");
        Self {
            value: value % modulus,
            modulus,
        }
    }

    /// Create a new finite field element from a signed integer
    ///
    /// Handles negative values correctly using symmetric representation.
    ///
    /// # Arguments
    ///
    /// * `value` - The signed integer value
    /// * `modulus` - The prime modulus p
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::algebra::Zp;
    ///
    /// let a = Zp::from_signed(-3, 7);
    /// assert_eq!(a.value(), 4);  // -3 ≡ 4 (mod 7)
    /// ```
    #[inline]
    pub fn from_signed(value: i64, modulus: u64) -> Self {
        debug_assert!(modulus > 1, "modulus must be > 1");
        let m = modulus as i64;
        let normalized = ((value % m) + m) % m;
        Self {
            value: normalized as u64,
            modulus,
        }
    }

    /// Get the underlying value
    #[inline]
    pub fn value(&self) -> u64 {
        self.value
    }

    /// Get the modulus
    #[inline]
    pub fn modulus(&self) -> u64 {
        self.modulus
    }

    /// Check if this element is zero
    #[inline]
    pub fn is_zero(&self) -> bool {
        self.value == 0
    }

    /// Check if this element is one
    #[inline]
    pub fn is_one(&self) -> bool {
        self.value == 1
    }

    /// Get the zero element of this field
    #[inline]
    pub fn zero(modulus: u64) -> Self {
        Self { value: 0, modulus }
    }

    /// Get the multiplicative identity (one) of this field
    #[inline]
    pub fn one(modulus: u64) -> Self {
        Self { value: 1, modulus }
    }

    /// Compute the additive inverse (-a mod p)
    #[inline]
    pub fn negate(&self) -> Self {
        if self.value == 0 {
            *self
        } else {
            Self {
                value: self.modulus - self.value,
                modulus: self.modulus,
            }
        }
    }

    /// Compute the multiplicative inverse using extended Euclidean algorithm
    ///
    /// Uses Fermat's little theorem: a^(-1) ≡ a^(p-2) (mod p) for prime p.
    /// However, extended GCD is faster for single inversions.
    ///
    /// # Returns
    ///
    /// `Ok(inverse)` if the element is non-zero, `Err` otherwise.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::algebra::Zp;
    ///
    /// let a = Zp::new(3, 7);
    /// let inv = a.inverse().unwrap();
    /// assert_eq!((a * inv).value(), 1);  // 3 * 5 = 15 ≡ 1 (mod 7)
    /// ```
    pub fn inverse(&self) -> FiniteFieldResult<Self> {
        if self.value == 0 {
            return Err(FiniteFieldError::DivisionByZero);
        }

        let (gcd, x, _) = extended_gcd(self.value as i64, self.modulus as i64);

        if gcd != 1 {
            return Err(FiniteFieldError::NoInverse {
                element: self.value,
                modulus: self.modulus,
            });
        }

        Ok(Self::from_signed(x, self.modulus))
    }

    /// Compute a^n mod p using binary exponentiation
    ///
    /// This is O(log n) multiplications.
    ///
    /// # Arguments
    ///
    /// * `exp` - The exponent
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::algebra::Zp;
    ///
    /// let a = Zp::new(2, 7);
    /// let result = a.pow(3);
    /// assert_eq!(result.value(), 1);  // 2^3 = 8 ≡ 1 (mod 7)
    /// ```
    #[inline]
    pub fn pow(&self, mut exp: u64) -> Self {
        if exp == 0 {
            return Self::one(self.modulus);
        }

        let mut base = *self;
        let mut result = Self::one(self.modulus);

        while exp > 0 {
            if exp & 1 == 1 {
                result = result * base;
            }
            base = base * base;
            exp >>= 1;
        }

        result
    }

    /// Convert to signed representation in [-p/2, p/2]
    ///
    /// This is the symmetric representation, useful for CRT reconstruction.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::algebra::Zp;
    ///
    /// let a = Zp::new(6, 7);
    /// assert_eq!(a.to_symmetric(), -1);  // 6 ≡ -1 (mod 7)
    /// ```
    #[inline]
    pub fn to_symmetric(&self) -> i64 {
        let half = self.modulus / 2;
        if self.value > half {
            self.value as i64 - self.modulus as i64
        } else {
            self.value as i64
        }
    }
}

impl fmt::Debug for Zp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}_{}", self.value, self.modulus)
    }
}

impl fmt::Display for Zp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl Add for Zp {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self {
        debug_assert_eq!(self.modulus, rhs.modulus, "modulus mismatch in addition");
        let sum = self.value + rhs.value;
        Self {
            value: if sum >= self.modulus {
                sum - self.modulus
            } else {
                sum
            },
            modulus: self.modulus,
        }
    }
}

impl Sub for Zp {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self {
        debug_assert_eq!(self.modulus, rhs.modulus, "modulus mismatch in subtraction");
        self + rhs.negate()
    }
}

impl Mul for Zp {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: Self) -> Self {
        debug_assert_eq!(
            self.modulus, rhs.modulus,
            "modulus mismatch in multiplication"
        );
        let product = (self.value as u128 * rhs.value as u128) % self.modulus as u128;
        Self {
            value: product as u64,
            modulus: self.modulus,
        }
    }
}

impl Div for Zp {
    type Output = Self;

    #[inline]
    fn div(self, rhs: Self) -> Self {
        debug_assert_eq!(self.modulus, rhs.modulus, "modulus mismatch in division");
        self * rhs.inverse().expect("division by zero in finite field")
    }
}

impl Neg for Zp {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self {
        self.negate()
    }
}

/// Extended Euclidean algorithm
///
/// Returns (gcd, x, y) such that ax + by = gcd(a, b)
#[inline]
pub fn extended_gcd(a: i64, b: i64) -> (i64, i64, i64) {
    if a == 0 {
        return (b, 0, 1);
    }

    let (gcd, x1, y1) = extended_gcd(b % a, a);
    let x = y1 - (b / a) * x1;
    let y = x1;

    (gcd, x, y)
}

/// Check if a number is prime using trial division
///
/// For use in debug assertions and error reporting.
/// For cryptographic primes, use a proper primality test.
///
/// # Examples
///
/// ```rust
/// use mathhook_core::algebra::is_prime;
///
/// assert!(is_prime(7));
/// assert!(is_prime(11));
/// assert!(!is_prime(9));
/// ```
pub fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    if n == 2 || n == 3 {
        return true;
    }
    if n % 2 == 0 || n % 3 == 0 {
        return false;
    }

    let mut i = 5u64;
    while i * i <= n {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
        i += 6;
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zp_basic_arithmetic() {
        let a = Zp::new(3, 7);
        let b = Zp::new(5, 7);

        assert_eq!((a + b).value(), 1);
        assert_eq!((a * b).value(), 1);
        assert_eq!((a - b).value(), 5);
        assert_eq!((-a).value(), 4);
    }

    #[test]
    fn test_zp_from_signed() {
        assert_eq!(Zp::from_signed(-3, 7).value(), 4);
        assert_eq!(Zp::from_signed(-7, 7).value(), 0);
        assert_eq!(Zp::from_signed(-1, 7).value(), 6);
    }

    #[test]
    fn test_zp_inverse() {
        let a = Zp::new(3, 7);
        let inv = a.inverse().unwrap();
        assert_eq!((a * inv).value(), 1);

        let b = Zp::new(2, 7);
        let inv_b = b.inverse().unwrap();
        assert_eq!((b * inv_b).value(), 1);
    }

    #[test]
    fn test_zp_inverse_of_zero() {
        let zero = Zp::zero(7);
        assert!(zero.inverse().is_err());
    }

    #[test]
    fn test_zp_pow() {
        let a = Zp::new(2, 7);
        assert_eq!(a.pow(0).value(), 1);
        assert_eq!(a.pow(1).value(), 2);
        assert_eq!(a.pow(2).value(), 4);
        assert_eq!(a.pow(3).value(), 1);
        assert_eq!(a.pow(6).value(), 1);
    }

    #[test]
    fn test_zp_symmetric() {
        assert_eq!(Zp::new(3, 7).to_symmetric(), 3);
        assert_eq!(Zp::new(4, 7).to_symmetric(), -3);
        assert_eq!(Zp::new(6, 7).to_symmetric(), -1);
    }

    #[test]
    fn test_large_prime() {
        let p: u64 = 2_147_483_647;
        let a = Zp::new(1_000_000, p);
        let b = Zp::new(1_000_000, p);
        let product = a * b;
        assert!(product.value() < p);

        let inv = a.inverse().unwrap();
        assert_eq!((a * inv).value(), 1);
    }

    #[test]
    fn test_is_prime() {
        assert!(is_prime(2));
        assert!(is_prime(3));
        assert!(is_prime(7));
        assert!(is_prime(11));
        assert!(is_prime(13));
        assert!(!is_prime(1));
        assert!(!is_prime(4));
        assert!(!is_prime(9));
        assert!(!is_prime(15));
    }
}
