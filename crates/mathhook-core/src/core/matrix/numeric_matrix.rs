use crate::error::MathError;

mod arithmetic;
mod conversion;
mod decomposition;
mod display;
mod multiply;
mod solve;

pub use decomposition::LUResult;

const EPSILON: f64 = 1e-10;

#[derive(Debug, Clone, PartialEq)]
pub struct NumericMatrix {
    rows: usize,
    cols: usize,
    data: Vec<f64>,
}

impl NumericMatrix {
    pub fn zeros(rows: usize, cols: usize) -> Result<Self, MathError> {
        if rows == 0 || cols == 0 {
            return Err(MathError::DomainError {
                operation: "NumericMatrix::zeros".to_string(),
                value: crate::Expression::integer(0),
                reason: "Matrix dimensions must be positive".to_string(),
            });
        }
        Ok(Self {
            rows,
            cols,
            data: vec![0.0; rows * cols],
        })
    }

    pub fn identity(n: usize) -> Result<Self, MathError> {
        if n == 0 {
            return Err(MathError::DomainError {
                operation: "NumericMatrix::identity".to_string(),
                value: crate::Expression::integer(0),
                reason: "Matrix dimension must be positive".to_string(),
            });
        }
        let mut data = vec![0.0; n * n];
        for i in 0..n {
            data[i * n + i] = 1.0;
        }
        Ok(Self {
            rows: n,
            cols: n,
            data,
        })
    }

    pub fn from_flat(rows: usize, cols: usize, data: Vec<f64>) -> Result<Self, MathError> {
        if rows == 0 || cols == 0 {
            return Err(MathError::DomainError {
                operation: "NumericMatrix::from_flat".to_string(),
                value: crate::Expression::integer(0),
                reason: "Matrix dimensions must be positive".to_string(),
            });
        }
        if data.len() != rows * cols {
            return Err(MathError::DomainError {
                operation: "NumericMatrix::from_flat".to_string(),
                value: crate::Expression::integer(data.len() as i64),
                reason: format!(
                    "Data length {} does not match dimensions {}x{}",
                    data.len(),
                    rows,
                    cols
                ),
            });
        }
        Ok(Self { rows, cols, data })
    }

    pub fn from_fn<F>(rows: usize, cols: usize, mut f: F) -> Result<Self, MathError>
    where
        F: FnMut(usize, usize) -> f64,
    {
        if rows == 0 || cols == 0 {
            return Err(MathError::DomainError {
                operation: "NumericMatrix::from_fn".to_string(),
                value: crate::Expression::integer(0),
                reason: "Matrix dimensions must be positive".to_string(),
            });
        }
        let mut data = Vec::with_capacity(rows * cols);
        for i in 0..rows {
            for j in 0..cols {
                data.push(f(i, j));
            }
        }
        Ok(Self { rows, cols, data })
    }

    pub fn dimensions(&self) -> (usize, usize) {
        (self.rows, self.cols)
    }

    pub fn get(&self, row: usize, col: usize) -> Result<f64, MathError> {
        if row >= self.rows || col >= self.cols {
            return Err(MathError::DomainError {
                operation: "NumericMatrix::get".to_string(),
                value: crate::Expression::integer((row * self.cols + col) as i64),
                reason: format!(
                    "Index ({}, {}) out of bounds for {}x{} matrix",
                    row, col, self.rows, self.cols
                ),
            });
        }
        Ok(self.data[row * self.cols + col])
    }

    pub fn set(&mut self, row: usize, col: usize, value: f64) -> Result<(), MathError> {
        if row >= self.rows || col >= self.cols {
            return Err(MathError::DomainError {
                operation: "NumericMatrix::set".to_string(),
                value: crate::Expression::integer((row * self.cols + col) as i64),
                reason: format!(
                    "Index ({}, {}) out of bounds for {}x{} matrix",
                    row, col, self.rows, self.cols
                ),
            });
        }
        self.data[row * self.cols + col] = value;
        Ok(())
    }

    pub fn is_square(&self) -> bool {
        self.rows == self.cols
    }

    pub fn is_symmetric(&self) -> bool {
        if !self.is_square() {
            return false;
        }
        for i in 0..self.rows {
            for j in (i + 1)..self.cols {
                let diff = (self.data[i * self.cols + j] - self.data[j * self.cols + i]).abs();
                if diff > EPSILON {
                    return false;
                }
            }
        }
        true
    }

    pub fn transpose(&self) -> Self {
        let mut data = vec![0.0; self.rows * self.cols];
        for i in 0..self.rows {
            for j in 0..self.cols {
                data[j * self.rows + i] = self.data[i * self.cols + j];
            }
        }
        Self {
            rows: self.cols,
            cols: self.rows,
            data,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zeros() {
        let m = NumericMatrix::zeros(2, 3).unwrap();
        assert_eq!(m.dimensions(), (2, 3));
        assert_eq!(m.get(0, 0).unwrap(), 0.0);
        assert_eq!(m.get(1, 2).unwrap(), 0.0);
    }

    #[test]
    fn test_identity() {
        let m = NumericMatrix::identity(3).unwrap();
        assert_eq!(m.dimensions(), (3, 3));
        assert_eq!(m.get(0, 0).unwrap(), 1.0);
        assert_eq!(m.get(1, 1).unwrap(), 1.0);
        assert_eq!(m.get(2, 2).unwrap(), 1.0);
        assert_eq!(m.get(0, 1).unwrap(), 0.0);
    }

    #[test]
    fn test_from_flat() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
        let m = NumericMatrix::from_flat(2, 3, data).unwrap();
        assert_eq!(m.dimensions(), (2, 3));
        assert_eq!(m.get(0, 0).unwrap(), 1.0);
        assert_eq!(m.get(1, 2).unwrap(), 6.0);
    }

    #[test]
    fn test_from_fn() {
        let m = NumericMatrix::from_fn(2, 2, |i, j| (i * 2 + j) as f64).unwrap();
        assert_eq!(m.get(0, 0).unwrap(), 0.0);
        assert_eq!(m.get(0, 1).unwrap(), 1.0);
        assert_eq!(m.get(1, 0).unwrap(), 2.0);
        assert_eq!(m.get(1, 1).unwrap(), 3.0);
    }

    #[test]
    fn test_transpose() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
        let m = NumericMatrix::from_flat(2, 3, data).unwrap();
        let t = m.transpose();
        assert_eq!(t.dimensions(), (3, 2));
        assert_eq!(t.get(0, 0).unwrap(), 1.0);
        assert_eq!(t.get(1, 0).unwrap(), 2.0);
        assert_eq!(t.get(2, 1).unwrap(), 6.0);
    }

    #[test]
    fn test_is_symmetric() {
        let sym_data = vec![1.0, 2.0, 2.0, 1.0];
        let m = NumericMatrix::from_flat(2, 2, sym_data).unwrap();
        assert!(m.is_symmetric());

        let nonsym_data = vec![1.0, 2.0, 3.0, 1.0];
        let m = NumericMatrix::from_flat(2, 2, nonsym_data).unwrap();
        assert!(!m.is_symmetric());
    }

    #[test]
    fn test_invalid_dimensions() {
        assert!(NumericMatrix::zeros(0, 3).is_err());
        assert!(NumericMatrix::identity(0).is_err());
        assert!(NumericMatrix::from_flat(2, 3, vec![1.0, 2.0]).is_err());
    }

    #[test]
    fn test_out_of_bounds() {
        let m = NumericMatrix::zeros(2, 2).unwrap();
        assert!(m.get(2, 0).is_err());
        assert!(m.get(0, 2).is_err());
    }
}
