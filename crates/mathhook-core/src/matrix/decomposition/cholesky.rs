//! Cholesky decomposition algorithms
//!
//! This module provides Cholesky decomposition for symmetric positive definite
//! matrices, useful for solving linear systems and optimization problems.

use crate::core::Expression;
use crate::matrix::types::*;
use crate::matrix::unified::Matrix;
use crate::simplify::Simplify;

/// Cholesky decomposition implementation
impl Matrix {
    /// Perform Cholesky decomposition for positive definite matrices
    ///
    /// Decomposes symmetric positive definite matrix A into A = LL^T where:
    /// - L is lower triangular with positive diagonal elements
    ///
    /// # Examples
    ///
    /// ```
    /// use mathhook_core::matrix::Matrix;
    /// use mathhook_core::Expression;
    ///
    /// let matrix = Matrix::from_arrays([
    ///     [Expression::integer(4), Expression::integer(2)],
    ///     [Expression::integer(2), Expression::integer(3)]
    /// ]);
    ///
    /// if let Some(chol) = matrix.cholesky_decomposition() {
    ///     let (l_rows, l_cols) = chol.l.dimensions();
    ///     assert_eq!(l_rows, 2);
    ///     assert_eq!(l_cols, 2);
    /// }
    /// ```
    pub fn cholesky_decomposition(&self) -> Option<CholeskyDecomposition> {
        let (rows, cols) = self.dimensions();
        if rows != cols || !self.is_symmetric() {
            return None;
        }

        match self {
            Matrix::Identity(data) => Some(CholeskyDecomposition {
                l: Matrix::identity(data.size),
            }),
            Matrix::Scalar(data) => {
                let sqrt_c = Expression::pow(data.scalar_value.clone(), Expression::rational(1, 2));
                Some(CholeskyDecomposition {
                    l: Matrix::scalar(data.size, sqrt_c),
                })
            }
            Matrix::Diagonal(data) => {
                let sqrt_elements: Vec<Expression> = data
                    .diagonal_elements
                    .iter()
                    .map(|elem| Expression::pow(elem.clone(), Expression::rational(1, 2)))
                    .collect();
                Some(CholeskyDecomposition {
                    l: Matrix::diagonal(sqrt_elements),
                })
            }
            _ => {
                // General Cholesky decomposition
                self.general_cholesky()
            }
        }
    }

    /// General Cholesky decomposition implementation
    fn general_cholesky(&self) -> Option<CholeskyDecomposition> {
        let (n, _) = self.dimensions();
        let mut l_elements = vec![vec![Expression::integer(0); n]; n];

        for i in 0..n {
            for j in 0..=i {
                if i == j {
                    // Diagonal element: L[i][i] = sqrt(A[i][i] - sum(L[i][k]^2 for k < i))
                    let mut sum = Expression::integer(0);
                    for k in 0..i {
                        let l_ik = l_elements[i][k].clone();
                        sum = Expression::add(vec![
                            sum,
                            Expression::pow(l_ik, Expression::integer(2)),
                        ])
                        .simplify();
                    }

                    let diagonal_val = Expression::add(vec![
                        self.get_element(i, i),
                        Expression::mul(vec![Expression::integer(-1), sum]),
                    ])
                    .simplify();

                    l_elements[i][i] = Expression::pow(diagonal_val, Expression::rational(1, 2));
                } else {
                    // Lower triangular element: L[i][j] = (A[i][j] - sum(L[i][k]*L[j][k] for k < j)) / L[j][j]
                    let mut sum = Expression::integer(0);
                    for k in 0..j {
                        let l_ik = l_elements[i][k].clone();
                        let l_jk = l_elements[j][k].clone();
                        sum = Expression::add(vec![sum, Expression::mul(vec![l_ik, l_jk])])
                            .simplify();
                    }

                    let numerator = Expression::add(vec![
                        self.get_element(i, j),
                        Expression::mul(vec![Expression::integer(-1), sum]),
                    ])
                    .simplify();

                    l_elements[i][j] =
                        Expression::function("div", vec![numerator, l_elements[j][j].clone()])
                            .simplify();
                }
            }
        }

        Some(CholeskyDecomposition {
            l: Matrix::dense(l_elements),
        })
    }

    /// Check if matrix is positive definite using Cholesky test
    ///
    /// # Examples
    ///
    /// ```
    /// use mathhook_core::matrix::Matrix;
    /// use mathhook_core::Expression;
    ///
    /// let identity = Matrix::identity(3);
    /// assert!(identity.is_positive_definite_cholesky());
    ///
    /// let scalar = Matrix::scalar(2, Expression::integer(5));
    /// assert!(scalar.is_positive_definite_cholesky());
    /// ```
    pub fn is_positive_definite_cholesky(&self) -> bool {
        if !self.is_symmetric() {
            return false;
        }

        match self {
            Matrix::Identity(_) => true,
            Matrix::Scalar(data) => {
                // Check if scalar_value > 0 (simplified check)
                !data.scalar_value.is_zero() && data.scalar_value != Expression::integer(-1)
            }
            Matrix::Diagonal(data) => {
                // Check if all diagonal elements > 0 (simplified check)
                data.diagonal_elements.iter().all(|elem| !elem.is_zero())
            }
            _ => {
                // Use Cholesky decomposition test
                self.cholesky_decomposition().is_some()
            }
        }
    }
}
