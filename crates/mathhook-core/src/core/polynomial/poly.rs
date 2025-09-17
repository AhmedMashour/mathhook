//! Generic univariate polynomial over any ring
//!
//! `Poly<T>` stores coefficients of type T for maximum performance and flexibility.
//! This bypasses the Expression tree for 20-100x speedup on polynomial operations.
//!
//! # Type Aliases
//!
//! ```rust
//! use mathhook_core::core::polynomial::poly::{Poly, IntPoly, RationalPoly};
//! use num_rational::Ratio;
//!
//! // IntPoly is an alias for Poly<i64>
//! let p: IntPoly = IntPoly::from_coeffs(vec![1, 2, 3]);
//! let q: Poly<i64> = Poly::from_coeffs(vec![1, 2, 3]);
//! assert_eq!(p, q);
//!
//! // RationalPoly is an alias for Poly<Ratio<i64>>
//! let r: RationalPoly = RationalPoly::from_coeffs(vec![
//!     Ratio::new(1, 2),
//!     Ratio::new(3, 4),
//! ]);
//! ```

mod arithmetic;
mod conversion;
mod display;
mod division;

use super::traits::Ring;
use num_rational::Ratio;

/// Generic univariate polynomial with coefficients of type T
///
/// Coefficients are stored in ascending order: ```coeffs[i]``` is the coefficient of x^i.
/// The zero polynomial has an empty coefficient vector.
///
/// # Examples
///
/// ```rust
/// use mathhook_core::core::polynomial::poly::IntPoly;
///
/// let p = IntPoly::from_coeffs(vec![1, 2, 3]);  // 1 + 2x + 3x²
/// assert_eq!(p.degree(), Some(2));
/// assert_eq!(p.leading_coeff(), 3);
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Poly<T: Ring> {
    coeffs: Vec<T>,
}

/// Type alias for integer polynomials (backward compatibility)
pub type IntPoly = Poly<i64>;

/// Type alias for rational coefficient polynomials
pub type RationalPoly = Poly<Ratio<i64>>;

impl<T: Ring> Poly<T> {
    /// Create polynomial from coefficients (ascending order)
    ///
    /// # Arguments
    /// * `coeffs` - Coefficients where ``coeffs[i]`` is coefficient of x^i
    ///
    /// # Example
    /// ```rust
    /// use mathhook_core::core::polynomial::poly::IntPoly;
    ///
    /// let p = IntPoly::from_coeffs(vec![1, 2, 3]);
    /// assert_eq!(p.degree(), Some(2));
    /// ```
    #[inline(always)]
    pub fn from_coeffs(mut coeffs: Vec<T>) -> Self {
        while coeffs.last().is_some_and(|c| c.is_zero()) {
            coeffs.pop();
        }
        Self { coeffs }
    }

    /// Create zero polynomial
    #[inline(always)]
    pub fn zero() -> Self {
        Self { coeffs: Vec::new() }
    }

    /// Create constant polynomial
    #[inline(always)]
    pub fn constant(c: T) -> Self {
        if c.is_zero() {
            Self::zero()
        } else {
            Self { coeffs: vec![c] }
        }
    }

    /// Check if polynomial is zero
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        self.coeffs.is_empty()
    }

    /// Check if polynomial is constant
    #[inline(always)]
    pub fn is_constant(&self) -> bool {
        self.coeffs.len() <= 1
    }

    /// Get polynomial degree (None for zero polynomial)
    #[inline(always)]
    pub fn degree(&self) -> Option<usize> {
        if self.coeffs.is_empty() {
            None
        } else {
            Some(self.coeffs.len() - 1)
        }
    }

    /// Get leading coefficient (zero for zero polynomial)
    #[inline(always)]
    pub fn leading_coeff(&self) -> T {
        self.coeffs.last().cloned().unwrap_or_else(T::zero)
    }

    /// Get coefficient of x^i
    #[inline(always)]
    pub fn coeff(&self, i: usize) -> T {
        self.coeffs.get(i).cloned().unwrap_or_else(T::zero)
    }

    /// Get all coefficients
    #[inline(always)]
    pub fn coefficients(&self) -> &[T] {
        &self.coeffs
    }
}

impl IntPoly {
    /// Create monomial x^n (IntPoly-specific)
    #[inline(always)]
    pub fn monomial(n: usize) -> Self {
        let mut coeffs = vec![0; n + 1];
        coeffs[n] = 1;
        Self { coeffs }
    }

    /// Create monomial c * x^n (IntPoly-specific)
    #[inline(always)]
    pub fn term(coeff: i64, power: usize) -> Self {
        if coeff == 0 {
            Self::zero()
        } else {
            let mut coeffs = vec![0; power + 1];
            coeffs[power] = coeff;
            Self { coeffs }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construction() {
        let p = IntPoly::from_coeffs(vec![1, 2, 3]);
        assert_eq!(p.degree(), Some(2));
        assert_eq!(p.leading_coeff(), 3);
        assert_eq!(p.coeff(0), 1);
        assert_eq!(p.coeff(1), 2);
        assert_eq!(p.coeff(2), 3);
        assert_eq!(p.coeff(3), 0);
    }

    #[test]
    fn test_zero_polynomial() {
        let z = IntPoly::zero();
        assert!(z.is_zero());
        assert_eq!(z.degree(), None);
        assert_eq!(z.leading_coeff(), 0);
    }

    #[test]
    fn test_trailing_zeros_removed() {
        let p = IntPoly::from_coeffs(vec![1, 2, 0, 0, 0]);
        assert_eq!(p.degree(), Some(1));
        assert_eq!(p.coefficients(), &[1, 2]);
    }

    #[test]
    fn test_constant_polynomial() {
        let p = IntPoly::constant(5);
        assert!(p.is_constant());
        assert_eq!(p.degree(), Some(0));
        assert_eq!(p.coeff(0), 5);

        let z = IntPoly::constant(0);
        assert!(z.is_zero());
        assert_eq!(z.degree(), None);
    }

    #[test]
    fn test_rational_poly_construction() {
        let p =
            RationalPoly::from_coeffs(vec![Ratio::new(1, 2), Ratio::new(3, 4), Ratio::new(5, 6)]);
        assert_eq!(p.degree(), Some(2));
        assert_eq!(p.leading_coeff(), Ratio::new(5, 6));
        assert_eq!(p.coeff(0), Ratio::new(1, 2));
    }

    #[test]
    fn test_rational_poly_zero() {
        let z = RationalPoly::zero();
        assert!(z.is_zero());
        assert_eq!(z.degree(), None);
        assert_eq!(z.leading_coeff(), Ratio::new(0, 1));
    }
}
