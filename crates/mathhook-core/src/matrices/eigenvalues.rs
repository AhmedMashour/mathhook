//! Eigenvalue and eigenvector computation
//!
//! This module provides methods for computing eigenvalues and eigenvectors
//! of matrices, including both real and complex cases, characteristic polynomials,
//! and matrix functions using eigendecomposition.

pub mod characteristic;
pub mod computation;
pub mod eigenvalues_tests;
pub mod power_methods;

use crate::core::expression::Expression;
use crate::core::symbol::Symbol;
use crate::matrices::eigenvalues::characteristic::CharacteristicPolynomial;
use crate::matrices::types::*;
use crate::matrices::unified::Matrix;
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
    /// ```
    /// use mathhook_core::matrices::Matrix;
    /// use mathhook_core::matrices::eigenvalues::EigenOperations;
    /// use mathhook_core::Expression;
    ///
    /// let matrix = Matrix::diagonal(vec![
    ///     Expression::integer(2),
    ///     Expression::integer(3)
    /// ]);
    ///
    /// let eigen = matrix.eigen_decomposition().unwrap();
    /// assert_eq!(eigen.eigenvalues.len(), 2);
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
    /// ```
    /// use mathhook_core::matrices::Matrix;
    /// use mathhook_core::matrices::eigenvalues::EigenOperations;
    /// use mathhook_core::Expression;
    ///
    /// let matrix = Matrix::diagonal(vec![
    ///     Expression::integer(2),
    ///     Expression::integer(3)
    /// ]);
    ///
    /// let eigenvalues = matrix.eigenvalues();
    /// assert_eq!(eigenvalues.len(), 2);
    /// assert_eq!(eigenvalues[0], Expression::integer(2));
    /// assert_eq!(eigenvalues[1], Expression::integer(3));
    /// ```
    fn eigenvalues(&self) -> Vec<Expression>;

    /// Compute characteristic polynomial det(A - λI)
    ///
    /// # Examples
    ///
    /// ```
    /// use mathhook_core::matrices::Matrix;
    /// use mathhook_core::matrices::eigenvalues::EigenOperations;
    /// use mathhook_core::Expression;
    ///
    /// let matrix = Matrix::diagonal(vec![
    ///     Expression::integer(2),
    ///     Expression::integer(3)
    /// ]);
    ///
    /// let poly = matrix.characteristic_polynomial();
    /// assert_eq!(poly.degree(), 2);
    /// ```
    fn characteristic_polynomial(&self) -> CharacteristicPolynomial;

    /// Get the trace (sum of eigenvalues)
    fn trace(&self) -> Expression;

    /// Get the determinant (product of eigenvalues)
    fn determinant_via_eigenvalues(&self) -> Expression;

    /// Check if matrix is diagonalizable
    fn is_diagonalizable(&self) -> bool;

    /// Compute matrix power using eigendecomposition
    ///
    /// For diagonalizable matrices, computes A^n = P D^n P^(-1)
    ///
    /// # Examples
    ///
    /// ```
    /// use mathhook_core::matrices::Matrix;
    /// use mathhook_core::matrices::eigenvalues::EigenOperations;
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
        // Call inherent method from computation module
        // Rust resolves this correctly because it's defined as impl Matrix {...}
        Matrix::eigen_decomposition(self)
    }

    fn complex_eigen_decomposition(&self) -> Option<ComplexEigenDecomposition> {
        // Call inherent method from computation module
        Matrix::complex_eigen_decomposition(self)
    }

    fn eigenvalues(&self) -> Vec<Expression> {
        // Call inherent method from computation module
        Matrix::eigenvalues(self)
    }

    fn characteristic_polynomial(&self) -> CharacteristicPolynomial {
        // Construct characteristic polynomial from eigenvalues
        // For a matrix with eigenvalues λ₁, λ₂, ..., λₙ:
        // char_poly(λ) = (λ - λ₁)(λ - λ₂)...(λ - λₙ)
        let eigenvals = Matrix::eigenvalues(self);
        let lambda = Symbol::scalar("lambda");

        if eigenvals.is_empty() {
            return CharacteristicPolynomial::new(vec![Expression::integer(0)], lambda);
        }

        // Start with (λ - λ₁)
        let mut poly = CharacteristicPolynomial::new(
            vec![
                Expression::mul(vec![Expression::integer(-1), eigenvals[0].clone()]),
                Expression::integer(1),
            ],
            lambda.clone(),
        );

        // Multiply by (λ - λᵢ) for remaining eigenvalues
        for eigenval in eigenvals.iter().skip(1) {
            let factor = CharacteristicPolynomial::new(
                vec![
                    Expression::mul(vec![Expression::integer(-1), eigenval.clone()]),
                    Expression::integer(1),
                ],
                lambda.clone(),
            );
            poly = poly.multiply(&factor);
        }

        poly
    }

    fn trace(&self) -> Expression {
        // Call inherent method from unified/operations.rs
        // Note: Matrix has a trace() inherent method, not from this trait
        Matrix::trace(self)
    }

    fn determinant_via_eigenvalues(&self) -> Expression {
        // Determinant equals product of eigenvalues
        let eigenvals = Matrix::eigenvalues(self);
        if eigenvals.is_empty() {
            Expression::integer(0)
        } else {
            Expression::mul(eigenvals).simplify()
        }
    }

    fn is_diagonalizable(&self) -> bool {
        // Call inherent method from computation module
        Matrix::is_diagonalizable(self)
    }

    fn matrix_power_eigen(&self, n: i64) -> Option<Matrix> {
        // Call inherent method from power_methods module
        Matrix::matrix_power_eigen(self, n)
    }

    fn matrix_exponential(&self) -> Option<Matrix> {
        // Call inherent method from power_methods module
        // Note: The inherent method is named matrix_exponential_eigen()
        Matrix::matrix_exponential_eigen(self)
    }

    fn matrix_logarithm(&self) -> Option<Matrix> {
        // Call inherent method from power_methods module
        // Note: The inherent method is named matrix_logarithm_eigen()
        Matrix::matrix_logarithm_eigen(self)
    }

    fn matrix_sqrt(&self) -> Option<Matrix> {
        // Call inherent method from power_methods module
        // Note: The inherent method is named matrix_sqrt_eigen()
        Matrix::matrix_sqrt_eigen(self)
    }

    fn is_nilpotent(&self) -> bool {
        // Matrix is nilpotent if all eigenvalues are zero
        let eigenvals = Matrix::eigenvalues(self);
        eigenvals.iter().all(|val| val.is_zero())
    }
}
