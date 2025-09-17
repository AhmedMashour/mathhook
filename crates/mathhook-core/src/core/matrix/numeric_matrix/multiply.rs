use crate::core::matrix::numeric_matrix::NumericMatrix;
use crate::error::MathError;

const BLOCK_SIZE: usize = 64;
const BLOCK_THRESHOLD: usize = 128;

impl NumericMatrix {
    pub fn multiply(&self, other: &Self) -> Result<Self, MathError> {
        if self.cols != other.rows {
            return Err(MathError::DomainError {
                operation: "NumericMatrix::multiply".to_string(),
                value: crate::Expression::integer(0),
                reason: format!(
                    "Cannot multiply {}x{} matrix with {}x{} matrix: inner dimensions must match",
                    self.rows, self.cols, other.rows, other.cols
                ),
            });
        }

        if self.rows >= BLOCK_THRESHOLD
            && self.cols >= BLOCK_THRESHOLD
            && other.cols >= BLOCK_THRESHOLD
        {
            self.blocked_multiply(other)
        } else {
            self.simple_multiply(other)
        }
    }

    fn simple_multiply(&self, other: &Self) -> Result<Self, MathError> {
        let mut data = vec![0.0; self.rows * other.cols];

        for i in 0..self.rows {
            for k in 0..self.cols {
                let a_ik = self.data[i * self.cols + k];
                for j in 0..other.cols {
                    data[i * other.cols + j] += a_ik * other.data[k * other.cols + j];
                }
            }
        }

        Ok(Self {
            rows: self.rows,
            cols: other.cols,
            data,
        })
    }

    fn blocked_multiply(&self, other: &Self) -> Result<Self, MathError> {
        let mut data = vec![0.0; self.rows * other.cols];

        for i_block in (0..self.rows).step_by(BLOCK_SIZE) {
            for j_block in (0..other.cols).step_by(BLOCK_SIZE) {
                for k_block in (0..self.cols).step_by(BLOCK_SIZE) {
                    let i_end = (i_block + BLOCK_SIZE).min(self.rows);
                    let j_end = (j_block + BLOCK_SIZE).min(other.cols);
                    let k_end = (k_block + BLOCK_SIZE).min(self.cols);

                    for i in i_block..i_end {
                        for k in k_block..k_end {
                            let a_ik = self.data[i * self.cols + k];
                            for j in j_block..j_end {
                                data[i * other.cols + j] += a_ik * other.data[k * other.cols + j];
                            }
                        }
                    }
                }
            }
        }

        Ok(Self {
            rows: self.rows,
            cols: other.cols,
            data,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EPSILON: f64 = 1e-10;

    #[test]
    fn test_multiply_small() {
        let a = NumericMatrix::from_flat(2, 3, vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]).unwrap();
        let b = NumericMatrix::from_flat(3, 2, vec![7.0, 8.0, 9.0, 10.0, 11.0, 12.0]).unwrap();
        let c = a.multiply(&b).unwrap();

        assert_eq!(c.dimensions(), (2, 2));
        assert!((c.get(0, 0).unwrap() - 58.0).abs() < EPSILON);
        assert!((c.get(0, 1).unwrap() - 64.0).abs() < EPSILON);
        assert!((c.get(1, 0).unwrap() - 139.0).abs() < EPSILON);
        assert!((c.get(1, 1).unwrap() - 154.0).abs() < EPSILON);
    }

    #[test]
    fn test_multiply_identity() {
        let a = NumericMatrix::from_flat(2, 2, vec![1.0, 2.0, 3.0, 4.0]).unwrap();
        let i = NumericMatrix::identity(2).unwrap();
        let c = a.multiply(&i).unwrap();

        assert_eq!(c.dimensions(), (2, 2));
        assert!((c.get(0, 0).unwrap() - 1.0).abs() < EPSILON);
        assert!((c.get(0, 1).unwrap() - 2.0).abs() < EPSILON);
        assert!((c.get(1, 0).unwrap() - 3.0).abs() < EPSILON);
        assert!((c.get(1, 1).unwrap() - 4.0).abs() < EPSILON);
    }

    #[test]
    fn test_multiply_zero() {
        let a = NumericMatrix::from_flat(2, 2, vec![1.0, 2.0, 3.0, 4.0]).unwrap();
        let z = NumericMatrix::zeros(2, 2).unwrap();
        let c = a.multiply(&z).unwrap();

        assert_eq!(c.dimensions(), (2, 2));
        for i in 0..2 {
            for j in 0..2 {
                assert!((c.get(i, j).unwrap()).abs() < EPSILON);
            }
        }
    }

    #[test]
    fn test_multiply_dimension_mismatch() {
        let a = NumericMatrix::from_flat(2, 3, vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]).unwrap();
        let b = NumericMatrix::from_flat(2, 2, vec![1.0, 2.0, 3.0, 4.0]).unwrap();
        assert!(a.multiply(&b).is_err());
    }

    #[test]
    fn test_blocked_multiply_large() {
        let n = 200;
        let a = NumericMatrix::from_fn(n, n, |i, j| (i + j) as f64).unwrap();
        let b = NumericMatrix::from_fn(n, n, |i, j| (i * j) as f64).unwrap();

        let c = a.multiply(&b).unwrap();
        assert_eq!(c.dimensions(), (n, n));

        let expected_00 = (0..n).map(|k| k as f64 * 0.0).sum::<f64>();
        assert!((c.get(0, 0).unwrap() - expected_00).abs() < EPSILON);

        let expected_11 = (0..n).map(|k| (1 + k) as f64 * k as f64).sum::<f64>();
        assert!((c.get(1, 1).unwrap() - expected_11).abs() < EPSILON);
    }

    #[test]
    fn test_simple_vs_blocked() {
        let n = 150;
        let a = NumericMatrix::from_fn(n, n, |i, j| (i as f64 + j as f64) / (n as f64)).unwrap();
        let b = NumericMatrix::from_fn(n, n, |i, j| (i as f64 * j as f64) / (n as f64)).unwrap();

        let c_simple = a.simple_multiply(&b).unwrap();
        let c_blocked = a.blocked_multiply(&b).unwrap();

        assert_eq!(c_simple.dimensions(), c_blocked.dimensions());
        for i in 0..n {
            for j in 0..n {
                let diff = (c_simple.get(i, j).unwrap() - c_blocked.get(i, j).unwrap()).abs();
                assert!(
                    diff < 1e-8,
                    "Mismatch at ({}, {}): simple={}, blocked={}, diff={}",
                    i,
                    j,
                    c_simple.get(i, j).unwrap(),
                    c_blocked.get(i, j).unwrap(),
                    diff
                );
            }
        }
    }
}
