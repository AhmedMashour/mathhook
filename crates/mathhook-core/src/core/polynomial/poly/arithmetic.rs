use super::Poly;
use crate::core::polynomial::traits::Ring;
use std::ops::{Add, Mul, Neg, Sub};

impl<T: Ring> Poly<T> {
    /// Polynomial addition
    #[inline]
    pub fn add(&self, other: &Self) -> Self {
        let max_len = self.coeffs.len().max(other.coeffs.len());
        let mut result = Vec::with_capacity(max_len);

        for i in 0..max_len {
            let a = self.coeff(i);
            let b = other.coeff(i);
            result.push(a.wrapping_add(&b));
        }

        Self::from_coeffs(result)
    }

    /// Polynomial subtraction
    #[inline]
    pub fn sub(&self, other: &Self) -> Self {
        let max_len = self.coeffs.len().max(other.coeffs.len());
        let mut result = Vec::with_capacity(max_len);

        for i in 0..max_len {
            let a = self.coeff(i);
            let b = other.coeff(i);
            result.push(a.wrapping_sub(&b));
        }

        Self::from_coeffs(result)
    }

    /// Polynomial multiplication (schoolbook algorithm)
    ///
    /// O(n*m) where n, m are degrees
    #[inline]
    pub fn mul(&self, other: &Self) -> Self {
        if self.is_zero() || other.is_zero() {
            return Self::zero();
        }

        let result_len = self.coeffs.len() + other.coeffs.len() - 1;
        let mut result = Vec::with_capacity(result_len);
        for _ in 0..result_len {
            result.push(T::zero());
        }

        for (i, a) in self.coeffs.iter().enumerate() {
            if a.is_zero() {
                continue;
            }
            for (j, b) in other.coeffs.iter().enumerate() {
                let prod = a.wrapping_mul(b);
                result[i + j] = result[i + j].wrapping_add(&prod);
            }
        }

        Self::from_coeffs(result)
    }

    /// Scalar multiplication
    #[inline]
    pub fn scale(&self, c: &T) -> Self {
        if c.is_zero() {
            return Self::zero();
        }
        if c.is_one() {
            return self.clone();
        }

        let coeffs: Vec<T> = self.coeffs.iter().map(|x| x.wrapping_mul(c)).collect();
        Self::from_coeffs(coeffs)
    }

    /// Negation
    #[inline]
    pub fn negate(&self) -> Self {
        let coeffs: Vec<T> = self.coeffs.iter().map(|x| -x.clone()).collect();
        Self { coeffs }
    }

    /// Derivative
    #[inline]
    pub fn derivative(&self) -> Self {
        if self.coeffs.len() <= 1 {
            return Self::zero();
        }

        let coeffs: Vec<T> = self.coeffs[1..]
            .iter()
            .enumerate()
            .map(|(i, c)| {
                let multiplier = T::zero().wrapping_add(&T::one());
                let mut result = multiplier.clone();
                for _ in 0..i {
                    result = result.wrapping_add(&multiplier);
                }
                c.wrapping_mul(&result)
            })
            .collect();

        Self::from_coeffs(coeffs)
    }

    /// Evaluate polynomial at a point using Horner's method
    ///
    /// Uses Horner's method: O(n) multiplications instead of O(n²)
    #[inline]
    pub fn evaluate(&self, x: &T) -> T {
        if self.coeffs.is_empty() {
            return T::zero();
        }

        let mut result = T::zero();
        for coeff in self.coeffs.iter().rev() {
            result = result.wrapping_mul(x).wrapping_add(coeff);
        }
        result
    }
}

impl<T: Ring> Add for Poly<T> {
    type Output = Self;

    #[inline]
    fn add(self, other: Self) -> Self {
        Poly::add(&self, &other)
    }
}

impl<T: Ring> Add for &Poly<T> {
    type Output = Poly<T>;

    #[inline]
    fn add(self, other: Self) -> Poly<T> {
        Poly::add(self, other)
    }
}

impl<T: Ring> Sub for Poly<T> {
    type Output = Self;

    #[inline]
    fn sub(self, other: Self) -> Self {
        Poly::sub(&self, &other)
    }
}

impl<T: Ring> Sub for &Poly<T> {
    type Output = Poly<T>;

    #[inline]
    fn sub(self, other: Self) -> Poly<T> {
        Poly::sub(self, other)
    }
}

impl<T: Ring> Mul for Poly<T> {
    type Output = Self;

    #[inline]
    fn mul(self, other: Self) -> Self {
        Poly::mul(&self, &other)
    }
}

impl<T: Ring> Mul for &Poly<T> {
    type Output = Poly<T>;

    #[inline]
    fn mul(self, other: Self) -> Poly<T> {
        Poly::mul(self, other)
    }
}

impl<T: Ring> Neg for Poly<T> {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self {
        self.negate()
    }
}

impl<T: Ring> Neg for &Poly<T> {
    type Output = Poly<T>;

    #[inline]
    fn neg(self) -> Poly<T> {
        self.negate()
    }
}

impl super::IntPoly {
    /// Evaluate at integer point (IntPoly-specific optimized version)
    #[inline]
    pub fn evaluate_i64(&self, x: i64) -> i64 {
        if self.coeffs.is_empty() {
            return 0;
        }

        let mut result = 0i64;
        for &coeff in self.coeffs.iter().rev() {
            result = result.wrapping_mul(x).wrapping_add(coeff);
        }
        result
    }

    /// Scalar multiplication for i64 (IntPoly-specific optimized version)
    #[inline]
    pub fn scale_i64(&self, c: i64) -> Self {
        if c == 0 {
            return Self::zero();
        }
        if c == 1 {
            return self.clone();
        }

        let coeffs: Vec<i64> = self.coeffs.iter().map(|&x| x.wrapping_mul(c)).collect();
        Self::from_coeffs(coeffs)
    }

    /// Derivative for IntPoly (optimized version)
    #[inline]
    pub fn derivative_i64(&self) -> Self {
        if self.coeffs.len() <= 1 {
            return Self::zero();
        }

        let coeffs: Vec<i64> = self.coeffs[1..]
            .iter()
            .enumerate()
            .map(|(i, &c)| c.wrapping_mul((i + 1) as i64))
            .collect();

        Self::from_coeffs(coeffs)
    }
}

#[cfg(test)]
mod tests {
    use crate::core::polynomial::poly::IntPoly;

    #[test]
    fn test_addition() {
        let p1 = IntPoly::from_coeffs(vec![1, 2, 3]);
        let p2 = IntPoly::from_coeffs(vec![4, 5]);
        let sum = &p1 + &p2;
        assert_eq!(sum.coefficients(), &[5, 7, 3]);
    }

    #[test]
    fn test_subtraction() {
        let p1 = IntPoly::from_coeffs(vec![5, 7, 3]);
        let p2 = IntPoly::from_coeffs(vec![4, 5]);
        let diff = &p1 - &p2;
        assert_eq!(diff.coefficients(), &[1, 2, 3]);
    }

    #[test]
    fn test_multiplication() {
        let p = IntPoly::from_coeffs(vec![1, 1]);
        let product = &p * &p;
        assert_eq!(product.coefficients(), &[1, 2, 1]);

        let p1 = IntPoly::from_coeffs(vec![1, 1]);
        let p2 = IntPoly::from_coeffs(vec![1, -1]);
        let product = &p1 * &p2;
        assert_eq!(product.coefficients(), &[1, 0, -1]);
    }

    #[test]
    fn test_evaluation() {
        let p = IntPoly::from_coeffs(vec![1, 2, 3]);
        assert_eq!(p.evaluate_i64(2), 17);
        assert_eq!(p.evaluate_i64(0), 1);
        assert_eq!(p.evaluate_i64(1), 6);
    }

    #[test]
    fn test_derivative() {
        let p = IntPoly::from_coeffs(vec![1, 2, 3, 4]);
        let dp = p.derivative_i64();
        assert_eq!(dp.coefficients(), &[2, 6, 12]);
    }

    #[test]
    fn test_negation() {
        let p = IntPoly::from_coeffs(vec![1, -2, 3]);
        let neg = -&p;
        assert_eq!(neg.coefficients(), &[-1, 2, -3]);
    }

    #[test]
    fn test_scalar_multiplication() {
        let p = IntPoly::from_coeffs(vec![1, 2, 3]);
        let scaled = p.scale_i64(3);
        assert_eq!(scaled.coefficients(), &[3, 6, 9]);
    }
}
