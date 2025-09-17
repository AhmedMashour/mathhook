use super::NumericMatrix;
use crate::error::MathError;

const EPSILON: f64 = 1e-10;

impl NumericMatrix {
    pub fn solve(&self, b: &[f64]) -> Result<Vec<f64>, MathError> {
        if !self.is_square() {
            return Err(MathError::DomainError {
                operation: "solve".to_string(),
                value: crate::Expression::integer(self.dimensions().0 as i64),
                reason: "Solving linear system requires square matrix".to_string(),
            });
        }

        if b.len() != self.rows {
            return Err(MathError::DomainError {
                operation: "solve".to_string(),
                value: crate::Expression::integer(b.len() as i64),
                reason: format!(
                    "Right-hand side length {} does not match matrix dimension {}",
                    b.len(),
                    self.rows
                ),
            });
        }

        let lu = self.lu_decomposition()?;

        let mut pb = vec![0.0; self.rows];
        for i in 0..self.rows {
            pb[i] = b[lu.p[i]];
        }

        let y = forward_substitution(&lu.l, &pb)?;

        backward_substitution(&lu.u, &y)
    }

    pub fn inverse(&self) -> Result<NumericMatrix, MathError> {
        if !self.is_square() {
            return Err(MathError::DomainError {
                operation: "inverse".to_string(),
                value: crate::Expression::integer(self.dimensions().0 as i64),
                reason: "Matrix inverse requires square matrix".to_string(),
            });
        }

        let n = self.rows;
        let mut inv_cols = Vec::with_capacity(n);

        for j in 0..n {
            let mut e = vec![0.0; n];
            e[j] = 1.0;

            let col = self.solve(&e)?;
            inv_cols.push(col);
        }

        let mut inv_data = vec![0.0; n * n];
        for i in 0..n {
            for j in 0..n {
                inv_data[i * n + j] = inv_cols[j][i];
            }
        }

        NumericMatrix::from_flat(n, n, inv_data)
    }
}

#[allow(clippy::needless_range_loop)]
fn forward_substitution(l: &NumericMatrix, b: &[f64]) -> Result<Vec<f64>, MathError> {
    let n = l.rows;
    let mut y = vec![0.0; n];

    for i in 0..n {
        let mut sum = 0.0;
        for j in 0..i {
            sum += l.get(i, j)? * y[j];
        }
        let l_ii = l.get(i, i)?;
        if l_ii.abs() < EPSILON {
            return Err(MathError::DomainError {
                operation: "forward_substitution".to_string(),
                value: crate::Expression::float(l_ii),
                reason: format!("Near-zero diagonal element at position {}", i),
            });
        }
        y[i] = (b[i] - sum) / l_ii;
    }

    Ok(y)
}

#[allow(clippy::needless_range_loop)]
fn backward_substitution(u: &NumericMatrix, y: &[f64]) -> Result<Vec<f64>, MathError> {
    let n = u.rows;
    let mut x = vec![0.0; n];

    for i in (0..n).rev() {
        let mut sum = 0.0;
        for j in (i + 1)..n {
            sum += u.get(i, j)? * x[j];
        }
        let u_ii = u.get(i, i)?;
        if u_ii.abs() < EPSILON {
            return Err(MathError::DomainError {
                operation: "backward_substitution".to_string(),
                value: crate::Expression::float(u_ii),
                reason: format!("Near-zero diagonal element at position {}", i),
            });
        }
        x[i] = (y[i] - sum) / u_ii;
    }

    Ok(x)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn approx_eq(a: f64, b: f64) -> bool {
        (a - b).abs() < EPSILON
    }

    fn vec_approx_eq(a: &[f64], b: &[f64]) -> bool {
        a.len() == b.len() && a.iter().zip(b.iter()).all(|(x, y)| approx_eq(*x, *y))
    }

    fn matrix_approx_eq(a: &NumericMatrix, b: &NumericMatrix) -> bool {
        if a.dimensions() != b.dimensions() {
            return false;
        }
        a.data
            .iter()
            .zip(b.data.iter())
            .all(|(x, y)| approx_eq(*x, *y))
    }

    #[test]
    fn test_solve_2x2() {
        let a = NumericMatrix::from_flat(2, 2, vec![3.0, 2.0, 1.0, 4.0]).unwrap();
        let b = vec![7.0, 9.0];

        let x = a.solve(&b).unwrap();

        let ax: Vec<f64> = (0..2)
            .map(|i| {
                x.iter()
                    .enumerate()
                    .map(|(j, &xj)| a.get(i, j).unwrap() * xj)
                    .sum()
            })
            .collect();

        assert!(vec_approx_eq(&ax, &b));
    }

    #[test]
    fn test_solve_3x3() {
        let a = NumericMatrix::from_flat(3, 3, vec![2.0, 1.0, 1.0, 4.0, 3.0, 3.0, 8.0, 7.0, 9.0])
            .unwrap();
        let b = vec![4.0, 10.0, 24.0];

        let x = a.solve(&b).unwrap();

        let ax: Vec<f64> = (0..3)
            .map(|i| {
                x.iter()
                    .enumerate()
                    .map(|(j, &xj)| a.get(i, j).unwrap() * xj)
                    .sum()
            })
            .collect();

        assert!(vec_approx_eq(&ax, &b));
    }

    #[test]
    fn test_solve_identity() {
        let a = NumericMatrix::identity(3).unwrap();
        let b = vec![1.0, 2.0, 3.0];

        let x = a.solve(&b).unwrap();

        assert!(vec_approx_eq(&x, &b));
    }

    #[test]
    fn test_solve_singular() {
        let a = NumericMatrix::from_flat(2, 2, vec![1.0, 2.0, 2.0, 4.0]).unwrap();
        let b = vec![3.0, 6.0];

        assert!(a.solve(&b).is_err());
    }

    #[test]
    fn test_solve_dimension_mismatch() {
        let a = NumericMatrix::from_flat(2, 2, vec![1.0, 2.0, 3.0, 4.0]).unwrap();
        let b = vec![1.0, 2.0, 3.0];

        assert!(a.solve(&b).is_err());
    }

    #[test]
    fn test_solve_non_square() {
        let a = NumericMatrix::from_flat(2, 3, vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]).unwrap();
        let b = vec![1.0, 2.0];

        assert!(a.solve(&b).is_err());
    }

    #[test]
    fn test_inverse_2x2() {
        let a = NumericMatrix::from_flat(2, 2, vec![4.0, 7.0, 2.0, 6.0]).unwrap();

        let a_inv = a.inverse().unwrap();

        let product = a.multiply(&a_inv).unwrap();
        let identity = NumericMatrix::identity(2).unwrap();

        assert!(matrix_approx_eq(&product, &identity));
    }

    #[test]
    fn test_inverse_3x3() {
        let a = NumericMatrix::from_flat(3, 3, vec![2.0, 1.0, 1.0, 4.0, 3.0, 3.0, 8.0, 7.0, 9.0])
            .unwrap();

        let a_inv = a.inverse().unwrap();

        let product = a.multiply(&a_inv).unwrap();
        let identity = NumericMatrix::identity(3).unwrap();

        assert!(matrix_approx_eq(&product, &identity));
    }

    #[test]
    fn test_inverse_identity() {
        let a = NumericMatrix::identity(3).unwrap();
        let a_inv = a.inverse().unwrap();

        assert!(matrix_approx_eq(&a, &a_inv));
    }

    #[test]
    fn test_inverse_singular() {
        let a = NumericMatrix::from_flat(2, 2, vec![1.0, 2.0, 2.0, 4.0]).unwrap();
        assert!(a.inverse().is_err());
    }

    #[test]
    fn test_inverse_non_square() {
        let a = NumericMatrix::from_flat(2, 3, vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]).unwrap();
        assert!(a.inverse().is_err());
    }
}
