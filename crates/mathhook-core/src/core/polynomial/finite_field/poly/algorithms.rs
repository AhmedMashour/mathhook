//! Polynomial algorithms (evaluation, power, derivative, monic)
use super::super::element::Zp;
use super::super::{FiniteFieldError, FiniteFieldResult};
use crate::core::polynomial::finite_field::poly::PolyZp;
impl PolyZp {
    /// Evaluate polynomial at a point using Horner's method
    ///
    /// This is O(n) multiplications for degree n polynomial.
    ///
    /// # Arguments
    ///
    /// * `x` - The evaluation point
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::algebra::PolyZp;
    ///
    /// // p(x) = x^2 + 2x + 3
    /// let p = PolyZp::from_coeffs(vec![3, 2, 1], 7);
    /// // p(2) = 4 + 4 + 3 = 11 ≡ 4 (mod 7)
    /// assert_eq!(p.evaluate(2).value(), 4);
    /// ```
    pub fn evaluate(&self, x: u64) -> Zp {
        if self.is_zero() {
            return Zp::zero(self.modulus());
        }
        let x_field = Zp::new(x, self.modulus());
        let mut result = Zp::zero(self.modulus());
        for &coeff in self.coefficients().iter().rev() {
            result = result * x_field + Zp::new(coeff, self.modulus());
        }
        result
    }
    /// Evaluate polynomial at a field element
    #[inline]
    pub fn evaluate_zp(&self, x: Zp) -> Zp {
        debug_assert_eq!(x.modulus(), self.modulus(), "modulus mismatch");
        self.evaluate(x.value())
    }
    /// Make the polynomial monic (leading coefficient = 1)
    ///
    /// Divides all coefficients by the leading coefficient.
    ///
    /// # Returns
    ///
    /// `Ok(monic_poly)` if non-zero, `Err` if zero polynomial.
    pub fn make_monic(&self) -> FiniteFieldResult<Self> {
        if self.is_zero() {
            return Err(FiniteFieldError::EmptyPolynomial);
        }
        let lc = self.leading_coeff().unwrap();
        if lc.is_one() {
            return Ok(self.clone());
        }
        let lc_inv = lc.inverse()?;
        let new_coeffs: Vec<u64> = self
            .coefficients()
            .iter()
            .map(|&c| (Zp::new(c, self.modulus()) * lc_inv).value())
            .collect();
        Ok(Self::from_coeffs(new_coeffs, self.modulus()))
    }
    /// Shift polynomial by multiplying by x^n
    pub fn shift(&self, n: usize) -> Self {
        if self.is_zero() || n == 0 {
            return self.clone();
        }
        let mut new_coeffs = vec![0u64; n];
        new_coeffs.extend_from_slice(self.coefficients());
        Self::from_coeffs(new_coeffs, self.modulus())
    }
}
