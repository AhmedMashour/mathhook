//! QR decomposition algorithms
//!
//! This module provides QR decomposition using the Gram-Schmidt process
//! for orthogonalization and solving least squares problems.

use crate::core::Expression;
use crate::matrix::types::*;
use crate::matrix::unified::Matrix;
use crate::simplify::Simplify;

/// QR decomposition implementation
impl Matrix {
    /// Perform QR decomposition using Gram-Schmidt process
    ///
    /// Decomposes matrix A into A = QR where:
    /// - Q is orthogonal (Q^T * Q = I)
    /// - R is upper triangular
    ///
    /// # Examples
    ///
    /// ```
    /// use mathhook_core::matrix::Matrix;
    /// use mathhook_core::Expression;
    ///
    /// let matrix = Matrix::from_arrays([
    ///     [Expression::integer(1), Expression::integer(1)],
    ///     [Expression::integer(0), Expression::integer(1)]
    /// ]);
    ///
    /// let qr = matrix.qr_decomposition().unwrap();
    /// let (q_rows, q_cols) = qr.q.dimensions();
    /// assert_eq!(q_rows, 2);
    /// assert_eq!(q_cols, 2);
    /// ```
    pub fn qr_decomposition(&self) -> Option<QRDecomposition> {
        match self {
            Matrix::Identity(data) => Some(QRDecomposition {
                q: Matrix::identity(data.size),
                r: Matrix::identity(data.size),
            }),
            Matrix::Zero(data) => Some(QRDecomposition {
                q: Matrix::identity(data.rows),
                r: Matrix::zero(data.rows, data.cols),
            }),
            _ => {
                // General QR decomposition using Gram-Schmidt process
                self.gram_schmidt_qr()
            }
        }
    }

    /// Gram-Schmidt QR decomposition implementation
    fn gram_schmidt_qr(&self) -> Option<QRDecomposition> {
        let (rows, cols) = self.dimensions();
        let mut q_columns: Vec<Vec<Expression>> = Vec::new();
        let mut r_elements = vec![vec![Expression::integer(0); cols]; cols];

        // Convert columns to vectors for processing
        for j in 0..cols {
            let mut column: Vec<Expression> = (0..rows).map(|i| self.get_element(i, j)).collect();

            // Orthogonalize against previous columns
            for k in 0..j {
                let dot_product = self.vector_dot(&column, &q_columns[k]);
                r_elements[k][j] = dot_product.clone();

                // column = column - dot_product * q_k
                for i in 0..rows {
                    let old_val = column[i].clone();
                    let subtract_val =
                        Expression::mul(vec![dot_product.clone(), q_columns[k][i].clone()]);
                    column[i] = Expression::add(vec![
                        old_val,
                        Expression::mul(vec![Expression::integer(-1), subtract_val]),
                    ])
                    .simplify();
                }
            }

            // Normalize the column
            let norm = self.vector_norm(&column);
            if norm.is_zero() {
                return None; // Linearly dependent columns
            }

            r_elements[j][j] = norm.clone();

            // Normalize column to get q_j
            let mut q_column = Vec::new();
            for i in 0..rows {
                // Use canonical form for division: a / b = a * b^(-1)
                q_column.push(
                    Expression::mul(vec![
                        column[i].clone(),
                        Expression::pow(norm.clone(), Expression::integer(-1)),
                    ])
                    .simplify(),
                );
            }
            q_columns.push(q_column);
        }

        // Build Q and R matrices
        let mut q_rows = vec![vec![Expression::integer(0); cols]; rows];
        for i in 0..rows {
            for j in 0..cols {
                if j < q_columns.len() {
                    q_rows[i][j] = q_columns[j][i].clone();
                }
            }
        }

        Some(QRDecomposition {
            q: Matrix::dense(q_rows),
            r: Matrix::dense(r_elements),
        })
    }

    /// Compute dot product of two vectors
    ///
    /// # Examples
    ///
    /// ```
    /// use mathhook_core::matrix::Matrix;
    /// use mathhook_core::Expression;
    ///
    /// let matrix = Matrix::identity(2);
    /// let v1 = vec![Expression::integer(1), Expression::integer(2)];
    /// let v2 = vec![Expression::integer(3), Expression::integer(4)];
    /// let dot = matrix.vector_dot(&v1, &v2);
    /// assert_eq!(dot, Expression::integer(11)); // 1*3 + 2*4 = 11
    /// ```
    fn vector_dot(&self, v1: &[Expression], v2: &[Expression]) -> Expression {
        if v1.len() != v2.len() {
            return Expression::integer(0);
        }

        let products: Vec<Expression> = v1
            .iter()
            .zip(v2.iter())
            .map(|(a, b)| Expression::mul(vec![a.clone(), b.clone()]))
            .collect();

        Expression::add(products).simplify()
    }

    /// Compute norm of a vector
    ///
    /// # Examples
    ///
    /// ```
    /// use mathhook_core::matrix::Matrix;
    /// use mathhook_core::Expression;
    ///
    /// let matrix = Matrix::identity(2);
    /// let v = vec![Expression::integer(3), Expression::integer(4)];
    /// let norm = matrix.vector_norm(&v);
    /// assert_eq!(norm, Expression::integer(5)); // sqrt(3² + 4²) = 5
    /// ```
    fn vector_norm(&self, v: &[Expression]) -> Expression {
        let sum_of_squares: Vec<Expression> = v
            .iter()
            .map(|x| Expression::pow(x.clone(), Expression::integer(2)))
            .collect();

        let sum = Expression::add(sum_of_squares).simplify();
        Expression::pow(sum, Expression::rational(1, 2))
    }
}
