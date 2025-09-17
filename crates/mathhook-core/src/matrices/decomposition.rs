//! Matrix decomposition algorithms
//!
//! This module provides various matrix decomposition methods including LU, QR,
//! Cholesky, and SVD decompositions optimized for the unified matrix system.

pub mod cholesky;
pub mod decomposition_tests;
pub mod lu;
pub mod qr;
pub mod svd;
use crate::core::Expression;
use crate::matrices::types::*;
use crate::matrices::unified::Matrix;

/// Matrix decomposition operations trait
///
/// This trait provides a unified interface for all matrix decomposition methods.
/// Each method returns an Option to handle cases where decomposition is not possible.
pub trait MatrixDecomposition {
    /// Perform LU decomposition with partial pivoting
    ///
    /// Decomposes matrix A into PA = LU where:
    /// - P is a permutation matrix
    /// - L is lower triangular with 1s on diagonal
    /// - U is upper triangular
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::matrices::{Matrix, MatrixDecomposition};
    /// use mathhook_core::Expression;
    ///
    /// let matrix = Matrix::from_arrays([
    ///     [2, 1, 1],
    ///     [4, 3, 3],
    ///     [8, 7, 9]
    /// ]);
    ///
    /// let lu = matrix.lu_decomposition().unwrap();
    /// // Verify: P*A = L*U
    /// ```
    fn lu_decomposition(&self) -> Option<LUDecomposition>;

    /// Perform QR decomposition using Gram-Schmidt process
    ///
    /// Decomposes matrix A into A = QR where:
    /// - Q is orthogonal (Q^T * Q = I)
    /// - R is upper triangular
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::matrices::{Matrix, MatrixDecomposition};
    ///
    /// let matrix = Matrix::from_arrays([
    ///     [1, 1, 0],
    ///     [1, 0, 1],
    ///     [0, 1, 1]
    /// ]);
    ///
    /// let qr = matrix.qr_decomposition().unwrap();
    /// // Verify: A = Q*R and Q^T*Q = I
    /// ```
    fn qr_decomposition(&self) -> Option<QRDecomposition>;

    /// Perform Cholesky decomposition for positive definite matrices
    ///
    /// Decomposes symmetric positive definite matrix A into A = LL^T where:
    /// - L is lower triangular with positive diagonal elements
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::matrices::{Matrix, MatrixDecomposition};
    ///
    /// let matrix = Matrix::from_arrays([
    ///     [4, 2, 1],
    ///     [2, 3, 0],
    ///     [1, 0, 2]
    /// ]);
    ///
    /// if let Some(chol) = matrix.cholesky_decomposition() {
    ///     // Verify: A = L*L^T
    /// }
    /// ```
    fn cholesky_decomposition(&self) -> Option<CholeskyDecomposition>;

    /// Perform Singular Value Decomposition
    ///
    /// Decomposes matrix A into A = UΣV^T where:
    /// - U contains left singular vectors (orthogonal)
    /// - Σ contains singular values (diagonal, non-negative)
    /// - V^T contains right singular vectors (orthogonal)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::matrices::{Matrix, MatrixDecomposition};
    ///
    /// let matrix = Matrix::from_arrays([
    ///     [1, 2],
    ///     [3, 4],
    ///     [5, 6]
    /// ]);
    ///
    /// let svd = matrix.svd_decomposition().unwrap();
    /// // Verify: A = U*Σ*V^T
    /// ```
    fn svd_decomposition(&self) -> Option<SVDDecomposition>;

    /// Get matrix rank using SVD
    fn rank(&self) -> usize;

    /// Check if matrix is positive definite
    fn is_positive_definite(&self) -> bool;

    /// Get condition number (ratio of largest to smallest singular value)
    fn condition_number(&self) -> Expression;
}

/// Implementation of MatrixDecomposition trait for Matrix
impl MatrixDecomposition for Matrix {
    fn lu_decomposition(&self) -> Option<LUDecomposition> {
        // Delegate to lu module implementation
        self.lu_decomposition()
    }

    fn qr_decomposition(&self) -> Option<QRDecomposition> {
        // Delegate to qr module implementation
        self.qr_decomposition()
    }

    fn cholesky_decomposition(&self) -> Option<CholeskyDecomposition> {
        // Delegate to cholesky module implementation
        self.cholesky_decomposition()
    }

    fn svd_decomposition(&self) -> Option<SVDDecomposition> {
        // Delegate to svd module implementation
        self.svd_decomposition()
    }

    fn rank(&self) -> usize {
        // Delegate to svd module implementation
        self.rank_via_svd()
    }

    fn is_positive_definite(&self) -> bool {
        // Delegate to cholesky module implementation
        self.is_positive_definite_cholesky()
    }

    fn condition_number(&self) -> Expression {
        // Delegate to svd module implementation
        self.condition_number_via_svd()
    }
}

/// Helper methods for matrix operations
impl Matrix {
    /// Convert any matrix to dense representation
    pub(crate) fn to_dense_matrix(&self) -> Matrix {
        let (rows, cols) = self.dimensions();
        let mut dense_rows = Vec::with_capacity(rows);

        for i in 0..rows {
            let mut row = Vec::with_capacity(cols);
            for j in 0..cols {
                row.push(self.get_element(i, j));
            }
            dense_rows.push(row);
        }

        Matrix::dense(dense_rows)
    }

    /// Swap two rows in a matrix
    pub(crate) fn swap_rows(&self, row1: usize, row2: usize) -> Matrix {
        if row1 == row2 {
            return self.clone();
        }

        let (rows, cols) = self.dimensions();
        let mut new_rows = Vec::with_capacity(rows);

        for i in 0..rows {
            let mut row = Vec::with_capacity(cols);
            let source_row = if i == row1 {
                row2
            } else if i == row2 {
                row1
            } else {
                i
            };

            for j in 0..cols {
                row.push(self.get_element(source_row, j));
            }
            new_rows.push(row);
        }

        Matrix::dense(new_rows)
    }

    /// Set element at position (i, j) - returns new matrix
    pub(crate) fn set_element(&self, i: usize, j: usize, value: &Expression) -> Matrix {
        let (rows, cols) = self.dimensions();
        let mut new_rows = Vec::with_capacity(rows);

        for row_idx in 0..rows {
            let mut row = Vec::with_capacity(cols);
            for col_idx in 0..cols {
                if row_idx == i && col_idx == j {
                    row.push(value.clone());
                } else {
                    row.push(self.get_element(row_idx, col_idx));
                }
            }
            new_rows.push(row);
        }

        Matrix::dense(new_rows)
    }
}
