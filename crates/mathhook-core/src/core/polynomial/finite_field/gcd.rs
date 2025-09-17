//! GCD algorithms for polynomials over finite fields
//!
//! Provides Euclidean and extended Euclidean algorithm for PolyZp.
//! These are foundational for modular GCD computation.
use super::element::Zp;
use super::poly::PolyZp;
use super::FiniteFieldError;
use super::FiniteFieldResult;
impl PolyZp {
    /// Compute GCD using Euclidean algorithm
    ///
    /// Returns a monic GCD (leading coefficient = 1).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::algebra::PolyZp;
    ///
    /// // gcd(x^2 - 1, x - 1) = x - 1
    /// let p1 = PolyZp::from_signed_coeffs(&[-1, 0, 1], 7);  // x^2 - 1
    /// let p2 = PolyZp::from_signed_coeffs(&[-1, 1], 7);      // x - 1
    /// let gcd = p1.gcd(&p2).unwrap();
    /// // Should be x - 1 (monic), which is [6, 1] in Z_7
    /// assert_eq!(gcd.degree(), Some(1));
    /// ```
    pub fn gcd(&self, other: &Self) -> FiniteFieldResult<Self> {
        debug_assert_eq!(self.modulus(), other.modulus(), "modulus mismatch");
        if self.is_zero() {
            return if other.is_zero() {
                Ok(Self::zero(self.modulus()))
            } else {
                other.make_monic()
            };
        }
        if other.is_zero() {
            return self.make_monic();
        }
        let mut a = self.clone();
        let mut b = other.clone();
        while !b.is_zero() {
            let (_, r) = a.div_rem(&b)?;
            a = b;
            b = r;
        }
        a.make_monic()
    }
    /// Extended Euclidean algorithm for polynomials
    ///
    /// Returns (gcd, s, t) such that gcd = s*self + t*other.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::algebra::PolyZp;
    ///
    /// let p1 = PolyZp::from_coeffs(vec![1, 2, 1], 7);
    /// let p2 = PolyZp::from_coeffs(vec![1, 1], 7);
    /// let (gcd, s, t) = p1.extended_gcd(&p2).unwrap();
    ///
    /// // Verify: gcd = s*p1 + t*p2
    /// let check = s.mul(&p1).add(&t.mul(&p2));
    /// assert_eq!(gcd.coefficients(), check.coefficients());
    /// ```
    pub fn extended_gcd(&self, other: &Self) -> FiniteFieldResult<(Self, Self, Self)> {
        debug_assert_eq!(self.modulus(), other.modulus(), "modulus mismatch");
        if self.is_zero() {
            return if other.is_zero() {
                Ok((
                    Self::zero(self.modulus()),
                    Self::zero(self.modulus()),
                    Self::zero(self.modulus()),
                ))
            } else {
                let monic = other.make_monic()?;
                let lc_inv = other.leading_coeff().unwrap().inverse()?;
                Ok((
                    monic,
                    Self::zero(self.modulus()),
                    Self::constant(lc_inv.value(), self.modulus()),
                ))
            };
        }
        if other.is_zero() {
            let monic = self.make_monic()?;
            let lc_inv = self.leading_coeff().unwrap().inverse()?;
            return Ok((
                monic,
                Self::constant(lc_inv.value(), self.modulus()),
                Self::zero(self.modulus()),
            ));
        }
        let mut old_r = self.clone();
        let mut r = other.clone();
        let mut old_s = Self::constant(1, self.modulus());
        let mut s = Self::zero(self.modulus());
        let mut old_t = Self::zero(self.modulus());
        let mut t = Self::constant(1, self.modulus());
        while !r.is_zero() {
            let (q, rem) = old_r.div_rem(&r)?;
            old_r = r;
            r = rem;
            let new_s = old_s.sub(&q.mul(&s));
            old_s = s;
            s = new_s;
            let new_t = old_t.sub(&q.mul(&t));
            old_t = t;
            t = new_t;
        }
        if old_r.is_zero() {
            return Ok((
                Self::zero(self.modulus()),
                Self::zero(self.modulus()),
                Self::zero(self.modulus()),
            ));
        }
        let lc_inv = old_r.leading_coeff().unwrap().inverse()?;
        let gcd = old_r.scale(lc_inv);
        let s_normalized = old_s.scale(lc_inv);
        let t_normalized = old_t.scale(lc_inv);
        Ok((gcd, s_normalized, t_normalized))
    }
    /// Compute modular inverse of a polynomial mod another polynomial
    ///
    /// Returns polynomial p such that self * p ≡ 1 (mod modpoly)
    ///
    /// # Arguments
    ///
    /// * `modpoly` - The modulus polynomial
    ///
    /// # Returns
    ///
    /// The inverse if it exists (gcd = 1), error otherwise.
    pub fn mod_inverse(&self, modpoly: &Self) -> FiniteFieldResult<Self> {
        let (gcd, s, _) = self.extended_gcd(modpoly)?;
        if !gcd.is_constant() || gcd.is_zero() {
            return Err(FiniteFieldError::NoInverse {
                element: 0,
                modulus: self.modulus(),
            });
        }
        let (_, remainder) = s.div_rem(modpoly)?;
        Ok(remainder)
    }
}
/// Compute content (GCD of all coefficients)
pub fn content(poly: &PolyZp) -> Zp {
    if poly.is_zero() {
        return Zp::zero(poly.modulus());
    }
    let mut result = poly.coeff(0);
    for i in 1..=poly.degree().unwrap_or(0) {
        let c = poly.coeff(i);
        result = gcd_zp(result, c);
        if result.is_one() {
            break;
        }
    }
    result
}
/// GCD of two field elements
fn gcd_zp(a: Zp, b: Zp) -> Zp {
    debug_assert_eq!(a.modulus(), b.modulus());
    if a.is_zero() {
        return b;
    }
    if b.is_zero() {
        return a;
    }
    Zp::one(a.modulus())
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_poly_gcd() {
        let p1 = PolyZp::from_signed_coeffs(&[-1, 0, 1], 7);
        let p2 = PolyZp::from_signed_coeffs(&[-1, 1], 7);
        let gcd = p1.gcd(&p2).unwrap();
        assert_eq!(gcd.degree(), Some(1));
        assert_eq!(gcd.leading_coeff().unwrap().value(), 1);
    }
    #[test]
    fn test_poly_gcd_coprime() {
        let p1 = PolyZp::from_coeffs(vec![1, 0, 1], 7);
        let p2 = PolyZp::from_coeffs(vec![1, 1], 7);
        let gcd = p1.gcd(&p2).unwrap();
        assert!(gcd.is_constant());
        assert_eq!(gcd.leading_coeff().unwrap().value(), 1);
    }
    #[test]
    fn test_poly_extended_gcd() {
        let p1 = PolyZp::from_coeffs(vec![1, 2, 1], 7);
        let p2 = PolyZp::from_coeffs(vec![1, 1], 7);
        let (gcd, s, t) = p1.extended_gcd(&p2).unwrap();
        let check = s.mul(&p1).add(&t.mul(&p2));
        assert_eq!(gcd.coefficients(), check.coefficients());
    }
    #[test]
    fn test_poly_gcd_zero() {
        let p1 = PolyZp::from_coeffs(vec![1, 2, 1], 7);
        let p2 = PolyZp::zero(7);
        let gcd = p1.gcd(&p2).unwrap();
        assert_eq!(gcd.degree(), p1.make_monic().unwrap().degree());
    }
    #[test]
    fn test_content() {
        let p = PolyZp::from_coeffs(vec![2, 4, 6], 11);
        let c = content(&p);
        assert_eq!(c.value(), 1);
    }
}
