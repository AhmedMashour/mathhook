//! Arithmetic operations for polynomials over finite fields
use super::super::element::Zp;
use super::super::{FiniteFieldError, FiniteFieldResult};
use crate::core::polynomial::finite_field::poly::PolyZp;
impl PolyZp {
    /// Add two polynomials
    pub fn add(&self, other: &Self) -> Self {
        debug_assert_eq!(self.modulus(), other.modulus(), "modulus mismatch");
        let max_len = self.coefficients().len().max(other.coefficients().len());
        let mut result = Vec::with_capacity(max_len);
        for i in 0..max_len {
            let a = self.coefficients().get(i).copied().unwrap_or(0);
            let b = other.coefficients().get(i).copied().unwrap_or(0);
            let sum = (a + b) % self.modulus();
            result.push(sum);
        }
        Self::from_coeffs(result, self.modulus())
    }
    /// Subtract two polynomials
    pub fn sub(&self, other: &Self) -> Self {
        debug_assert_eq!(self.modulus(), other.modulus(), "modulus mismatch");
        let max_len = self.coefficients().len().max(other.coefficients().len());
        let mut result = Vec::with_capacity(max_len);
        for i in 0..max_len {
            let a = self.coefficients().get(i).copied().unwrap_or(0);
            let b = other.coefficients().get(i).copied().unwrap_or(0);
            let diff = if a >= b {
                a - b
            } else {
                self.modulus() - (b - a)
            };
            result.push(diff);
        }
        Self::from_coeffs(result, self.modulus())
    }
    /// Multiply two polynomials using naive algorithm
    ///
    /// Uses naive O(n*m) convolution multiplication. This is the baseline algorithm
    /// that works for any modulus and is efficient for small polynomials.
    ///
    /// For large polynomials with NTT-friendly primes, consider using `mul_fast()`
    /// which automatically switches to O(n log n) NTT multiplication.
    ///
    /// # Complexity
    ///
    /// - **Time**: O(n*m) where n = deg(self), m = deg(other)
    /// - **Space**: O(n + m) for result coefficients
    ///
    /// # Performance
    ///
    /// - **Efficient for**: Small polynomials (degree < 64)
    /// - **Alternative for large**: Use `mul_fast()` for automatic NTT optimization
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::algebra::PolyZp;
    ///
    /// let a = PolyZp::from_coeffs(vec![1, 1], 7);  // x + 1
    /// let b = PolyZp::from_coeffs(vec![1, 1], 7);  // x + 1
    /// let product = a.mul(&b);                      // x^2 + 2x + 1
    /// assert_eq!(product.coefficients(), &[1, 2, 1]);
    /// ```
    #[inline(always)]
    pub fn mul(&self, other: &Self) -> Self {
        debug_assert_eq!(self.modulus(), other.modulus(), "modulus mismatch");
        if self.is_zero() || other.is_zero() {
            return Self::zero(self.modulus());
        }
        let a_coeffs = self.coefficients();
        let b_coeffs = other.coefficients();
        let a_len = a_coeffs.len();
        let b_len = b_coeffs.len();
        let result_len = a_len + b_len - 1;
        let mut result = vec![0u128; result_len];
        unsafe {
            let result_ptr = result.as_mut_ptr();
            for i in 0..a_len {
                let a = *a_coeffs.get_unchecked(i) as u128;
                for j in 0..b_len {
                    let b = *b_coeffs.get_unchecked(j) as u128;
                    *result_ptr.add(i + j) += a * b;
                }
            }
        }
        let modulus = self.modulus() as u128;
        let reduced: Vec<u64> = result.iter().map(|&x| (x % modulus) as u64).collect();
        Self::from_coeffs(reduced, self.modulus())
    }
    /// Fast polynomial multiplication with automatic algorithm selection
    ///
    /// Automatically chooses between naive O(n²) and NTT O(n log n) multiplication
    /// based on polynomial degree and prime modulus support.
    ///
    /// # Algorithm Selection
    ///
    /// - **Small polynomials** (degree ≤ 64): Uses naive multiplication
    /// - **Large polynomials** (degree > 64) with NTT-friendly prime: Uses NTT
    /// - **Unsupported prime**: Falls back to naive multiplication
    ///
    /// # NTT-Friendly Primes
    ///
    /// - 2013265921 = 15 * 2^27 + 1 (supports degree up to 2^27 - 1)
    /// - 469762049  = 7 * 2^26 + 1  (supports degree up to 2^26 - 1)
    /// - 1004535809 = 479 * 2^21 + 1 (supports degree up to 2^21 - 1)
    ///
    /// # Performance
    ///
    /// For degree 1000 polynomials: ~10x faster than naive
    /// For degree 10000 polynomials: ~100x faster than naive
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::algebra::PolyZp;
    ///
    /// // Small polynomials: automatically uses naive
    /// let a = PolyZp::from_coeffs(vec![1, 2, 3], 7);
    /// let b = PolyZp::from_coeffs(vec![4, 5], 7);
    /// let product = a.mul_fast(&b);
    ///
    /// // Large polynomials with NTT prime: automatically uses NTT
    /// let p = 2013265921;  // NTT-friendly prime
    /// let large_a = PolyZp::from_coeffs(vec![1; 100], p);
    /// let large_b = PolyZp::from_coeffs(vec![2; 100], p);
    /// let fast_product = large_a.mul_fast(&large_b);  // Uses NTT
    /// ```
    pub fn mul_fast(&self, other: &Self) -> Self {
        super::super::ntt::multiply_auto(self, other)
    }
    /// Multiply polynomial by a scalar
    pub fn scale(&self, c: Zp) -> Self {
        debug_assert_eq!(c.modulus(), self.modulus(), "modulus mismatch");
        if c.is_zero() || self.is_zero() {
            return Self::zero(self.modulus());
        }
        let new_coeffs: Vec<u64> = self
            .coefficients()
            .iter()
            .map(|&a| (Zp::new(a, self.modulus()) * c).value())
            .collect();
        Self::from_coeffs(new_coeffs, self.modulus())
    }
    /// Polynomial division with remainder
    ///
    /// Returns (quotient, remainder) such that self = quotient * divisor + remainder
    /// and degree(remainder) < degree(divisor).
    ///
    /// # Arguments
    ///
    /// * `divisor` - The divisor polynomial (must be non-zero)
    ///
    /// # Returns
    ///
    /// `Ok((quotient, remainder))` or error if divisor is zero.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::algebra::PolyZp;
    ///
    /// // (x^2 + 2x + 1) / (x + 1) = (x + 1, 0)
    /// let dividend = PolyZp::from_coeffs(vec![1, 2, 1], 7);
    /// let divisor = PolyZp::from_coeffs(vec![1, 1], 7);
    /// let (q, r) = dividend.div_rem(&divisor).unwrap();
    /// assert_eq!(q.coefficients(), &[1, 1]);
    /// assert!(r.is_zero());
    /// ```
    pub fn div_rem(&self, divisor: &Self) -> FiniteFieldResult<(Self, Self)> {
        debug_assert_eq!(self.modulus(), divisor.modulus(), "modulus mismatch");
        if divisor.is_zero() {
            return Err(FiniteFieldError::DivisionByZero);
        }
        if self.is_zero() {
            return Ok((Self::zero(self.modulus()), Self::zero(self.modulus())));
        }
        let div_degree = divisor.degree().unwrap();
        let self_degree = match self.degree() {
            Some(d) => d,
            None => return Ok((Self::zero(self.modulus()), Self::zero(self.modulus()))),
        };
        if self_degree < div_degree {
            return Ok((Self::zero(self.modulus()), self.clone()));
        }
        let lc_inv = divisor.leading_coeff().unwrap().inverse()?;
        let mut remainder = self.coefficients().to_vec();
        let quotient_len = self_degree - div_degree + 1;
        let mut quotient = vec![0u64; quotient_len];
        for i in (0..quotient_len).rev() {
            let rem_idx = i + div_degree;
            if rem_idx >= remainder.len() {
                continue;
            }
            let coeff = Zp::new(remainder[rem_idx], self.modulus()) * lc_inv;
            quotient[i] = coeff.value();
            if coeff.is_zero() {
                continue;
            }
            for (j, &div_coeff) in divisor.coefficients().iter().enumerate() {
                let term = Zp::new(div_coeff, self.modulus()) * coeff;
                let rem_val = Zp::new(remainder[i + j], self.modulus());
                remainder[i + j] = (rem_val - term).value();
            }
        }
        Ok((
            Self::from_coeffs(quotient, self.modulus()),
            Self::from_coeffs(remainder, self.modulus()),
        ))
    }
}
