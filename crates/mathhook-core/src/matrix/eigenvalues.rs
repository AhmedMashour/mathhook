//! Eigenvalue and eigenvector computation
//!
//! This module provides methods for computing eigenvalues and eigenvectors
//! of matrices, including both real and complex cases, characteristic polynomials,
//! and matrix functions using eigendecomposition.

pub mod characteristic;
pub mod computation;
pub mod power_methods;

use crate::core::Expression;
use crate::matrix::types::*;
use crate::matrix::unified::Matrix;
use crate::simplify::Simplify;

/// Eigenvalue and eigenvector operations trait
///
/// This trait provides a unified interface for all eigenvalue-related computations.
/// Methods return Options to handle cases where computation is not possible.
pub trait EigenOperations {
    /// Compute eigenvalues and eigenvectors
    ///
    /// Returns eigenvalues and corresponding eigenvectors for real matrices.
    /// For matrices with complex eigenvalues, use `complex_eigen_decomposition`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::matrix::{Matrix, EigenOperations};
    /// use mathhook_core::Expression;
    ///
    /// let matrix = Matrix::diagonal(vec![
    ///     Expression::integer(2),
    ///     Expression::integer(3),
    ///     Expression::integer(4)
    /// ]);
    ///
    /// let eigen = matrix.eigen_decomposition().unwrap();
    /// // eigenvalues = [2, 3, 4], eigenvectors = I
    /// ```
    fn eigen_decomposition(&self) -> Option<EigenDecomposition>;

    /// Compute complex eigenvalues and eigenvectors
    ///
    /// Handles matrices that may have complex eigenvalues and eigenvectors.
    fn complex_eigen_decomposition(&self) -> Option<ComplexEigenDecomposition>;

    /// Compute only eigenvalues (faster than full decomposition)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::matrix::{Matrix, EigenOperations};
    /// use mathhook_core::Expression;
    ///
    /// let matrix = Matrix::scalar(3, Expression::integer(5));
    /// let eigenvals = matrix.eigenvalues();
    /// // Returns [5, 5, 5]
    /// ```
    fn eigenvalues(&self) -> Vec<Expression>;

    /// Compute characteristic polynomial det(A - λI)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::matrix::{Matrix, EigenOperations};
    ///
    /// let matrix = Matrix::identity(2);
    /// let poly = matrix.characteristic_polynomial();
    /// // Returns (1-λ)²
    /// ```
    fn characteristic_polynomial(&self) -> CharacteristicPolynomial;

    /// Get the trace (sum of eigenvalues)
    fn trace(&self) -> Expression;

    /// Get the determinant (product of eigenvalues)
    fn determinant_via_eigenvalues(&self) -> Expression;

    /// Check if matrix is diagonalizable
    fn is_diagonalizable(&self) -> bool;

    /// Compute matrix power using eigendecomposition (A^n = P D^n P^(-1))
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::matrix::{Matrix, EigenOperations};
    /// use mathhook_core::Expression;
    ///
    /// let matrix = Matrix::diagonal(vec![
    ///     Expression::integer(2),
    ///     Expression::integer(3)
    /// ]);
    ///
    /// let power = matrix.matrix_power_eigen(3).unwrap();
    /// // Returns diag([8, 27])
    /// ```
    fn matrix_power_eigen(&self, n: i64) -> Option<Matrix>;

    /// Compute matrix exponential using eigendecomposition
    fn matrix_exponential(&self) -> Option<Matrix>;

    /// Compute matrix logarithm using eigendecomposition
    fn matrix_logarithm(&self) -> Option<Matrix>;

    /// Compute matrix square root using eigendecomposition
    fn matrix_sqrt(&self) -> Option<Matrix>;

    /// Check if matrix is nilpotent
    fn is_nilpotent(&self) -> bool;
}

/// Implementation of EigenOperations trait for Matrix
impl EigenOperations for Matrix {
    fn eigen_decomposition(&self) -> Option<EigenDecomposition> {
        // Delegate to computation module
        self.eigen_decomposition()
    }

    fn complex_eigen_decomposition(&self) -> Option<ComplexEigenDecomposition> {
        // Delegate to computation module
        self.complex_eigen_decomposition()
    }

    fn eigenvalues(&self) -> Vec<Expression> {
        // Delegate to computation module
        self.eigenvalues()
    }

    fn characteristic_polynomial(&self) -> CharacteristicPolynomial {
        // Delegate to characteristic module
        self.characteristic_polynomial()
    }

    fn trace(&self) -> Expression {
        // Trace equals sum of eigenvalues
        self.trace() // Use existing trace method from unified.rs
    }

    fn determinant_via_eigenvalues(&self) -> Expression {
        // Determinant equals product of eigenvalues
        let eigenvals = self.eigenvalues();
        if eigenvals.is_empty() {
            Expression::integer(0)
        } else {
            Expression::mul(eigenvals).simplify()
        }
    }

    fn is_diagonalizable(&self) -> bool {
        // Delegate to computation module
        self.is_diagonalizable()
    }

    fn matrix_power_eigen(&self, n: i64) -> Option<Matrix> {
        // Delegate to power_methods module
        self.matrix_power_eigen(n)
    }

    fn matrix_exponential(&self) -> Option<Matrix> {
        // Delegate to power_methods module
        self.matrix_exponential_eigen()
    }

    fn matrix_logarithm(&self) -> Option<Matrix> {
        // Delegate to power_methods module
        self.matrix_logarithm_eigen()
    }

    fn matrix_sqrt(&self) -> Option<Matrix> {
        // Delegate to power_methods module
        self.matrix_sqrt_eigen()
    }

    fn is_nilpotent(&self) -> bool {
        // Delegate to power_methods module
        self.is_nilpotent()
    }
}
