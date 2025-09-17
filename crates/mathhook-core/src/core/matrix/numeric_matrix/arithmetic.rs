use crate::core::matrix::numeric_matrix::NumericMatrix;
use crate::error::MathError;
use std::ops::{Add, Mul, Neg, Sub};

impl NumericMatrix {
    pub fn add(&self, other: &Self) -> Result<Self, MathError> {
        if self.rows != other.rows || self.cols != other.cols {
            return Err(MathError::DomainError {
                operation: "NumericMatrix::add".to_string(),
                value: crate::Expression::integer(0),
                reason: format!(
                    "Cannot add matrices with different dimensions: {}x{} and {}x{}",
                    self.rows, self.cols, other.rows, other.cols
                ),
            });
        }

        let data: Vec<f64> = self
            .data
            .iter()
            .zip(other.data.iter())
            .map(|(a, b)| a + b)
            .collect();

        Ok(Self {
            rows: self.rows,
            cols: self.cols,
            data,
        })
    }

    pub fn sub(&self, other: &Self) -> Result<Self, MathError> {
        if self.rows != other.rows || self.cols != other.cols {
            return Err(MathError::DomainError {
                operation: "NumericMatrix::sub".to_string(),
                value: crate::Expression::integer(0),
                reason: format!(
                    "Cannot subtract matrices with different dimensions: {}x{} and {}x{}",
                    self.rows, self.cols, other.rows, other.cols
                ),
            });
        }

        let data: Vec<f64> = self
            .data
            .iter()
            .zip(other.data.iter())
            .map(|(a, b)| a - b)
            .collect();

        Ok(Self {
            rows: self.rows,
            cols: self.cols,
            data,
        })
    }

    pub fn scalar_mul(&self, scalar: f64) -> Self {
        let data: Vec<f64> = self.data.iter().map(|x| x * scalar).collect();
        Self {
            rows: self.rows,
            cols: self.cols,
            data,
        }
    }

    pub fn negate(&self) -> Self {
        let data: Vec<f64> = self.data.iter().map(|x| -x).collect();
        Self {
            rows: self.rows,
            cols: self.cols,
            data,
        }
    }
}

impl Add for NumericMatrix {
    type Output = Result<Self, MathError>;

    fn add(self, other: Self) -> Self::Output {
        NumericMatrix::add(&self, &other)
    }
}

impl Add for &NumericMatrix {
    type Output = Result<NumericMatrix, MathError>;

    fn add(self, other: Self) -> Self::Output {
        NumericMatrix::add(self, other)
    }
}

impl Sub for NumericMatrix {
    type Output = Result<Self, MathError>;

    fn sub(self, other: Self) -> Self::Output {
        NumericMatrix::sub(&self, &other)
    }
}

impl Sub for &NumericMatrix {
    type Output = Result<NumericMatrix, MathError>;

    fn sub(self, other: Self) -> Self::Output {
        NumericMatrix::sub(self, other)
    }
}

impl Mul<f64> for NumericMatrix {
    type Output = Self;

    fn mul(self, scalar: f64) -> Self::Output {
        self.scalar_mul(scalar)
    }
}

impl Mul<f64> for &NumericMatrix {
    type Output = NumericMatrix;

    fn mul(self, scalar: f64) -> Self::Output {
        self.scalar_mul(scalar)
    }
}

impl Mul<NumericMatrix> for f64 {
    type Output = NumericMatrix;

    fn mul(self, matrix: NumericMatrix) -> Self::Output {
        matrix.scalar_mul(self)
    }
}

impl Mul<&NumericMatrix> for f64 {
    type Output = NumericMatrix;

    fn mul(self, matrix: &NumericMatrix) -> Self::Output {
        matrix.scalar_mul(self)
    }
}

impl Neg for NumericMatrix {
    type Output = Self;

    fn neg(self) -> Self::Output {
        self.negate()
    }
}

impl Neg for &NumericMatrix {
    type Output = NumericMatrix;

    fn neg(self) -> Self::Output {
        self.negate()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_method() {
        let a = NumericMatrix::from_flat(2, 2, vec![1.0, 2.0, 3.0, 4.0]).unwrap();
        let b = NumericMatrix::from_flat(2, 2, vec![5.0, 6.0, 7.0, 8.0]).unwrap();
        let c = NumericMatrix::add(&a, &b).unwrap();
        assert_eq!(c.get(0, 0).unwrap(), 6.0);
        assert_eq!(c.get(0, 1).unwrap(), 8.0);
        assert_eq!(c.get(1, 0).unwrap(), 10.0);
        assert_eq!(c.get(1, 1).unwrap(), 12.0);
    }

    #[test]
    fn test_sub_method() {
        let a = NumericMatrix::from_flat(2, 2, vec![5.0, 6.0, 7.0, 8.0]).unwrap();
        let b = NumericMatrix::from_flat(2, 2, vec![1.0, 2.0, 3.0, 4.0]).unwrap();
        let c = NumericMatrix::sub(&a, &b).unwrap();
        assert_eq!(c.get(0, 0).unwrap(), 4.0);
        assert_eq!(c.get(0, 1).unwrap(), 4.0);
        assert_eq!(c.get(1, 0).unwrap(), 4.0);
        assert_eq!(c.get(1, 1).unwrap(), 4.0);
    }

    #[test]
    fn test_scalar_mul() {
        let a = NumericMatrix::from_flat(2, 2, vec![1.0, 2.0, 3.0, 4.0]).unwrap();
        let b = a.scalar_mul(2.0);
        assert_eq!(b.get(0, 0).unwrap(), 2.0);
        assert_eq!(b.get(0, 1).unwrap(), 4.0);
        assert_eq!(b.get(1, 0).unwrap(), 6.0);
        assert_eq!(b.get(1, 1).unwrap(), 8.0);
    }

    #[test]
    fn test_negate() {
        let a = NumericMatrix::from_flat(2, 2, vec![1.0, 2.0, 3.0, 4.0]).unwrap();
        let b = a.negate();
        assert_eq!(b.get(0, 0).unwrap(), -1.0);
        assert_eq!(b.get(0, 1).unwrap(), -2.0);
        assert_eq!(b.get(1, 0).unwrap(), -3.0);
        assert_eq!(b.get(1, 1).unwrap(), -4.0);
    }

    #[test]
    fn test_add_trait() {
        let a = NumericMatrix::from_flat(2, 2, vec![1.0, 2.0, 3.0, 4.0]).unwrap();
        let b = NumericMatrix::from_flat(2, 2, vec![5.0, 6.0, 7.0, 8.0]).unwrap();
        let c = (&a + &b).unwrap();
        assert_eq!(c.get(0, 0).unwrap(), 6.0);
    }

    #[test]
    fn test_sub_trait() {
        let a = NumericMatrix::from_flat(2, 2, vec![5.0, 6.0, 7.0, 8.0]).unwrap();
        let b = NumericMatrix::from_flat(2, 2, vec![1.0, 2.0, 3.0, 4.0]).unwrap();
        let c = (&a - &b).unwrap();
        assert_eq!(c.get(0, 0).unwrap(), 4.0);
    }

    #[test]
    fn test_mul_trait() {
        let a = NumericMatrix::from_flat(2, 2, vec![1.0, 2.0, 3.0, 4.0]).unwrap();
        let b = &a * 2.0;
        assert_eq!(b.get(0, 0).unwrap(), 2.0);

        let c = 3.0 * &a;
        assert_eq!(c.get(0, 0).unwrap(), 3.0);
    }

    #[test]
    fn test_neg_trait() {
        let a = NumericMatrix::from_flat(2, 2, vec![1.0, 2.0, 3.0, 4.0]).unwrap();
        let b = -&a;
        assert_eq!(b.get(0, 0).unwrap(), -1.0);
    }

    #[test]
    fn test_dimension_mismatch() {
        let a = NumericMatrix::from_flat(2, 2, vec![1.0, 2.0, 3.0, 4.0]).unwrap();
        let b = NumericMatrix::from_flat(2, 3, vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]).unwrap();
        assert!(NumericMatrix::add(&a, &b).is_err());
        assert!(NumericMatrix::sub(&a, &b).is_err());
    }
}
