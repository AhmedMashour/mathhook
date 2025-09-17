//! Polynomial over finite field `Z_p[x]`
//!
//! This module provides `PolyZp` - polynomials with coefficients in Z_p.
//! Core arithmetic operations: add, sub, mul, div_rem, evaluate.
//!
//! # Representation
//!
//! Coefficients stored as `Vec<u64>` where index i = coefficient of x^i.
//! Zero polynomial has empty coefficient vector.
//!
//! # Invariants
//!
//! - Leading coefficient is always non-zero (except for zero polynomial)
//! - Coefficients are normalized (0 <= c < p)
use super::element::Zp;
mod algorithms;
mod arithmetic;
mod display;
#[cfg(test)]
mod tests;
/// Polynomial over finite field `Z_p[x]`
///
/// Represented as a vector of coefficients where index i is the coefficient of x^i.
/// The zero polynomial has empty coefficients.
///
/// # Invariants
///
/// - Leading coefficient is always non-zero (except for zero polynomial)
/// - Coefficients are normalized (0 <= c < p)
///
/// # Memory Layout
///
/// Uses `Vec<u64>` for coefficients plus a `u64` for the modulus.
/// This is cache-friendly and allows efficient iteration.
///
/// # Examples
///
/// ```rust
/// use mathhook_core::algebra::PolyZp;
///
/// // Create polynomial x^2 + 2x + 3 mod 7
/// let p = PolyZp::from_coeffs(vec![3, 2, 1], 7);
/// assert_eq!(p.degree(), Some(2));
///
/// // Evaluate at x = 2: 4 + 4 + 3 = 11 ≡ 4 (mod 7)
/// let result = p.evaluate(2);
/// assert_eq!(result.value(), 4);
/// ```
#[derive(Clone, PartialEq, Eq)]
pub struct PolyZp {
    coeffs: Vec<u64>,
    modulus: u64,
}
impl PolyZp {
    /// Create polynomial from owned coefficients
    ///
    /// Use this when you already have an owned `Vec<u64>` to avoid extra allocation.
    /// This is more efficient than `from_coeffs` which always copies.
    ///
    /// # Arguments
    ///
    /// * `coeffs` - Owned vector of coefficients where index i is coefficient of x^i
    /// * `modulus` - The prime modulus
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::algebra::PolyZp;
    ///
    /// // x^2 + 2x + 3 mod 7 - takes ownership, no extra allocation
    /// let coeffs = vec![3, 2, 1];
    /// let p = PolyZp::from_coeffs(coeffs, 7);
    /// assert_eq!(p.degree(), Some(2));
    /// ```
    #[inline]
    pub fn from_coeffs(mut coeffs: Vec<u64>, modulus: u64) -> Self {
        for c in coeffs.iter_mut() {
            *c %= modulus;
        }
        while coeffs.last() == Some(&0) {
            coeffs.pop();
        }
        Self { coeffs, modulus }
    }

    /// Create polynomial from signed coefficients
    pub fn from_signed_coeffs(coeffs: &[i64], modulus: u64) -> Self {
        let m = modulus as i64;
        let normalized: Vec<u64> = coeffs.iter().map(|&c| (((c % m) + m) % m) as u64).collect();
        Self::from_coeffs(normalized, modulus)
    }

    /// Create the zero polynomial
    #[inline]
    pub fn zero(modulus: u64) -> Self {
        Self {
            coeffs: Vec::new(),
            modulus,
        }
    }
    /// Create the constant polynomial c
    #[inline]
    pub fn constant(c: u64, modulus: u64) -> Self {
        if c % modulus == 0 {
            Self::zero(modulus)
        } else {
            Self {
                coeffs: vec![c % modulus],
                modulus,
            }
        }
    }
    /// Create the polynomial x
    #[inline]
    pub fn x(modulus: u64) -> Self {
        Self {
            coeffs: vec![0, 1],
            modulus,
        }
    }
    /// Create the monic polynomial x - a
    #[inline]
    pub fn x_minus_a(a: u64, modulus: u64) -> Self {
        let neg_a = if a % modulus == 0 {
            0
        } else {
            modulus - (a % modulus)
        };
        Self {
            coeffs: vec![neg_a, 1],
            modulus,
        }
    }
    /// Check if this is the zero polynomial
    #[inline]
    pub fn is_zero(&self) -> bool {
        self.coeffs.is_empty()
    }
    /// Check if this is a constant polynomial
    #[inline]
    pub fn is_constant(&self) -> bool {
        self.coeffs.len() <= 1
    }
    /// Get the degree of the polynomial
    ///
    /// Returns `None` for the zero polynomial, `Some(d)` otherwise.
    #[inline]
    pub fn degree(&self) -> Option<usize> {
        if self.coeffs.is_empty() {
            None
        } else {
            Some(self.coeffs.len() - 1)
        }
    }
    /// Get the leading coefficient
    ///
    /// Returns `None` for the zero polynomial.
    #[inline]
    pub fn leading_coeff(&self) -> Option<Zp> {
        self.coeffs.last().map(|&c| Zp::new(c, self.modulus))
    }
    /// Get coefficient of x^i
    #[inline]
    pub fn coeff(&self, i: usize) -> Zp {
        let value = self.coeffs.get(i).copied().unwrap_or(0);
        Zp::new(value, self.modulus)
    }
    /// Get the modulus
    #[inline]
    pub fn modulus(&self) -> u64 {
        self.modulus
    }
    /// Get coefficients as slice
    #[inline]
    pub fn coefficients(&self) -> &[u64] {
        &self.coeffs
    }
}
