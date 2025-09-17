use super::Poly;
use crate::core::polynomial::traits::EuclideanDomain;
use crate::error::MathError;

/// Integer GCD using Euclidean algorithm
#[inline(always)]
fn gcd_i64(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a.abs()
}

impl<T: EuclideanDomain> Poly<T> {
    /// Polynomial division with remainder
    ///
    /// Returns (quotient, remainder) such that self = quotient * divisor + remainder
    /// and degree(remainder) < degree(divisor)
    ///
    /// # Errors
    /// Returns `MathError::DivisionByZero` if divisor is zero
    #[inline(always)]
    pub fn div_rem(&self, divisor: &Self) -> Result<(Self, Self), MathError> {
        if divisor.is_zero() {
            return Err(MathError::DivisionByZero);
        }

        if self.is_zero() {
            return Ok((Self::zero(), Self::zero()));
        }

        let self_deg = match self.degree() {
            Some(d) => d,
            None => return Ok((Self::zero(), Self::zero())),
        };

        let divisor_deg = match divisor.degree() {
            Some(d) => d,
            None => return Err(MathError::DivisionByZero),
        };

        if self_deg < divisor_deg {
            return Ok((Self::zero(), self.clone()));
        }

        let divisor_lc = divisor.leading_coeff();
        let mut remainder = self.coeffs.clone();
        let mut quotient = Vec::with_capacity(self_deg - divisor_deg + 1);
        for _ in 0..=self_deg - divisor_deg {
            quotient.push(T::zero());
        }

        for i in (0..=self_deg - divisor_deg).rev() {
            let rem_idx = i + divisor_deg;
            let rem_coeff = remainder[rem_idx].clone();

            let (q_coeff, r) = rem_coeff.div_rem(&divisor_lc);
            if !r.is_zero() {
                break;
            }

            quotient[i] = q_coeff.clone();

            for (j, d_coeff) in divisor.coeffs.iter().enumerate() {
                let sub_val = q_coeff.wrapping_mul(d_coeff);
                remainder[i + j] = remainder[i + j].wrapping_sub(&sub_val);
            }
        }

        Ok((Self::from_coeffs(quotient), Self::from_coeffs(remainder)))
    }

    /// Polynomial pseudo-division
    ///
    /// Returns (quotient, remainder, multiplier) such that:
    /// multiplier * self = quotient * divisor + remainder
    ///
    /// This avoids fractions by multiplying by powers of the leading coefficient.
    ///
    /// # Errors
    /// Returns `MathError::DivisionByZero` if divisor is zero
    #[inline(always)]
    pub fn pseudo_div_rem(&self, divisor: &Self) -> Result<(Self, Self, T), MathError> {
        if divisor.is_zero() {
            return Err(MathError::DivisionByZero);
        }

        if self.is_zero() {
            return Ok((Self::zero(), Self::zero(), T::one()));
        }

        let self_deg = match self.degree() {
            Some(d) => d,
            None => return Ok((Self::zero(), Self::zero(), T::one())),
        };

        let divisor_deg = match divisor.degree() {
            Some(d) => d,
            None => return Err(MathError::DivisionByZero),
        };

        if self_deg < divisor_deg {
            return Ok((Self::zero(), self.clone(), T::one()));
        }

        let divisor_lc = divisor.leading_coeff();
        let mut remainder = self.coeffs.clone();
        let mut quotient = Vec::with_capacity(self_deg - divisor_deg + 1);
        for _ in 0..=self_deg - divisor_deg {
            quotient.push(T::zero());
        }
        let mut multiplier = T::one();

        for i in (0..=self_deg - divisor_deg).rev() {
            let rem_idx = i + divisor_deg;

            for c in remainder.iter_mut() {
                *c = c.wrapping_mul(&divisor_lc);
            }
            for c in quotient.iter_mut() {
                *c = c.wrapping_mul(&divisor_lc);
            }
            multiplier = multiplier.wrapping_mul(&divisor_lc);

            let (q_coeff, _) = remainder[rem_idx].div_rem(&divisor_lc);
            quotient[i] = q_coeff.clone();

            for (j, d_coeff) in divisor.coeffs.iter().enumerate() {
                let sub_val = q_coeff.wrapping_mul(d_coeff);
                remainder[i + j] = remainder[i + j].wrapping_sub(&sub_val);
            }
        }

        Ok((
            Self::from_coeffs(quotient),
            Self::from_coeffs(remainder),
            multiplier,
        ))
    }

    /// GCD using Euclidean algorithm
    ///
    /// Returns the greatest common divisor of two polynomials.
    /// The result is made primitive (content = 1).
    ///
    /// # Errors
    /// Returns `MathError::DivisionByZero` if division by zero occurs during computation
    #[inline(always)]
    pub fn gcd(&self, other: &Self) -> Result<Self, MathError> {
        if self.is_zero() {
            return other.primitive_part();
        }
        if other.is_zero() {
            return self.primitive_part();
        }

        let mut a = self.primitive_part()?;
        let mut b = other.primitive_part()?;

        while !b.is_zero() {
            let (_, rem, _) = a.pseudo_div_rem(&b)?;
            a = b;
            b = if rem.is_zero() {
                rem
            } else {
                rem.primitive_part()?
            };
        }

        Ok(a)
    }

    /// Content: GCD of all coefficients
    #[inline(always)]
    pub fn content(&self) -> T {
        if self.coeffs.is_empty() {
            return T::zero();
        }

        let mut g = self.coeffs[0].abs();
        for c in &self.coeffs[1..] {
            g = g.gcd(&c.abs());
            if g.is_one() {
                break;
            }
        }
        g
    }

    /// Primitive part: polynomial / content
    ///
    /// # Errors
    /// Returns `MathError::DivisionByZero` if content is zero
    #[inline(always)]
    pub fn primitive_part(&self) -> Result<Self, MathError> {
        let c = self.content();
        if c.is_zero() {
            return Err(MathError::DivisionByZero);
        }
        if c.is_one() {
            return Ok(self.clone());
        }

        let coeffs: Vec<T> = self
            .coeffs
            .iter()
            .map(|x| {
                let (quot, _) = x.div_rem(&c);
                quot
            })
            .collect();
        Ok(Self { coeffs })
    }
}

impl super::IntPoly {
    /// Check if polynomial is monic (leading coefficient = 1)
    #[inline(always)]
    pub fn is_monic(&self) -> bool {
        self.leading_coeff() == 1
    }

    /// Make polynomial monic by dividing by leading coefficient
    /// Returns None if leading coefficient doesn't divide all coefficients
    #[inline(always)]
    pub fn try_make_monic(&self) -> Option<Self> {
        let lc = self.leading_coeff();
        if lc == 0 {
            return None;
        }
        if lc == 1 {
            return Some(self.clone());
        }

        for &c in &self.coeffs {
            if c % lc != 0 {
                return None;
            }
        }

        let coeffs: Vec<i64> = self.coeffs.iter().map(|&c| c / lc).collect();
        Some(Self { coeffs })
    }

    /// Normalize IntPoly (make leading coefficient positive)
    #[inline(always)]
    pub fn normalize(&self) -> Self {
        if self.leading_coeff() < 0 {
            self.negate()
        } else {
            self.clone()
        }
    }

    /// Content for IntPoly (optimized)
    #[inline(always)]
    pub fn content_i64(&self) -> i64 {
        if self.coeffs.is_empty() {
            return 0;
        }

        let mut g = self.coeffs[0].abs();
        for &c in &self.coeffs[1..] {
            g = gcd_i64(g, c.abs());
            if g == 1 {
                break;
            }
        }
        g
    }

    /// Primitive part for IntPoly (optimized)
    ///
    /// # Errors
    /// Returns `MathError::DivisionByZero` if content is zero
    #[inline(always)]
    pub fn primitive_part_i64(&self) -> Result<Self, MathError> {
        let c = self.content_i64();
        if c == 0 {
            return Err(MathError::DivisionByZero);
        }
        if c == 1 {
            return Ok(self.clone());
        }

        let coeffs: Vec<i64> = self.coeffs.iter().map(|&x| x / c).collect();
        Ok(Self { coeffs })
    }

    /// GCD for IntPoly (optimized)
    ///
    /// # Errors
    /// Returns `MathError::DivisionByZero` if division by zero occurs during computation
    #[inline(always)]
    pub fn gcd_i64(&self, other: &Self) -> Result<Self, MathError> {
        if self.is_zero() {
            return Ok(other.primitive_part_i64()?.normalize());
        }
        if other.is_zero() {
            return Ok(self.primitive_part_i64()?.normalize());
        }

        let mut a = self.primitive_part_i64()?;
        let mut b = other.primitive_part_i64()?;

        while !b.is_zero() {
            let (_, rem, _) = a.pseudo_div_rem(&b)?;
            a = b;
            b = if rem.is_zero() {
                rem
            } else {
                rem.primitive_part_i64()?
            };
        }

        Ok(a.normalize())
    }
}

#[cfg(test)]
mod tests {
    use crate::core::polynomial::poly::IntPoly;

    #[test]
    fn test_division_exact() {
        let dividend = IntPoly::from_coeffs(vec![-1, 0, 1]);
        let divisor = IntPoly::from_coeffs(vec![-1, 1]);
        let (quot, rem) = dividend.div_rem(&divisor).unwrap();

        assert_eq!(quot.coefficients(), &[1, 1]);
        assert!(rem.is_zero());
    }

    #[test]
    fn test_gcd() {
        let p1 = IntPoly::from_coeffs(vec![0, 0, 6]);
        let p2 = IntPoly::from_coeffs(vec![0, 9]);
        let g = p1.gcd_i64(&p2).unwrap();

        assert_eq!(g.degree(), Some(1));
        assert_eq!(g.coeff(0), 0);
        assert!(g.coeff(1) > 0);
    }

    #[test]
    fn test_gcd_coprime() {
        let p1 = IntPoly::from_coeffs(vec![1, 1]);
        let p2 = IntPoly::from_coeffs(vec![2, 1]);
        let g = p1.gcd_i64(&p2).unwrap();

        assert!(g.is_constant());
        assert_eq!(g.coefficients(), &[1]);
    }

    #[test]
    fn test_content() {
        let p = IntPoly::from_coeffs(vec![6, 12, 18]);
        assert_eq!(p.content_i64(), 6);
    }

    #[test]
    fn test_primitive_part() {
        let p = IntPoly::from_coeffs(vec![6, 12, 18]);
        let pp = p.primitive_part_i64().unwrap();
        assert_eq!(pp.coefficients(), &[1, 2, 3]);
    }

    #[test]
    fn test_division_by_zero() {
        let dividend = IntPoly::from_coeffs(vec![1, 2, 3]);
        let divisor = IntPoly::from_coeffs(vec![]);
        assert!(dividend.div_rem(&divisor).is_err());
    }
}
