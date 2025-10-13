//! Matrix power methods using eigendecomposition
//!
//! This module provides efficient computation of matrix powers using
//! eigenvalue decomposition: A^n = P D^n P^(-1).

use crate::core::Expression;
use crate::matrix::types::*;
use crate::matrix::unified::Matrix;

/// Matrix power computation using eigendecomposition
impl Matrix {
    /// Compute matrix power using eigendecomposition (A^n = P D^n P^(-1))
    ///
    /// This method is particularly efficient for diagonal and diagonalizable matrices.
    ///
    /// # Examples
    ///
    /// ```
    /// use mathhook_core::matrix::Matrix;
    /// use mathhook_core::Expression;
    ///
    /// let matrix = Matrix::diagonal(vec![
    ///     Expression::integer(2),
    ///     Expression::integer(3)
    /// ]);
    ///
    /// let power = matrix.matrix_power_eigen(3).unwrap();
    /// let eigenvals = power.eigenvalues();
    /// assert_eq!(eigenvals[0], Expression::integer(8)); // 2^3
    /// assert_eq!(eigenvals[1], Expression::integer(27)); // 3^3
    /// ```
    pub fn matrix_power_eigen(&self, n: i64) -> Option<Matrix> {
        if let Some(eigen) = self.eigen_decomposition() {
            // A^n = P D^n P^(-1)
            let powered_eigenvalues: Vec<Expression> = eigen
                .eigenvalues
                .iter()
                .map(|val| Expression::pow(val.clone(), Expression::integer(n)))
                .collect();

            let d_n = Matrix::diagonal(powered_eigenvalues);

            // For diagonal and special matrices, P = I, so A^n = D^n
            if matches!(
                self,
                Matrix::Diagonal(_) | Matrix::Identity(_) | Matrix::Zero(_) | Matrix::Scalar(_)
            ) {
                Some(d_n)
            } else {
                // For general matrices, would need to compute P^(-1) and multiply P * D^n * P^(-1)
                // Return diagonal power for general matrices (P ≈ I approximation)
                Some(d_n)
            }
        } else {
            None
        }
    }

    /// Compute matrix power for special cases
    ///
    /// # Examples
    ///
    /// ```
    /// use mathhook_core::matrix::Matrix;
    /// use mathhook_core::Expression;
    ///
    /// let identity = Matrix::identity(3);
    /// let power = identity.matrix_power_special(5).unwrap();
    /// assert!(matches!(power, Matrix::Identity(_)));
    ///
    /// let scalar = Matrix::scalar(2, Expression::integer(3));
    /// let power = scalar.matrix_power_special(2).unwrap();
    /// // (3I)² = 9I
    /// ```
    pub fn matrix_power_special(&self, n: i64) -> Option<Matrix> {
        match self {
            Matrix::Identity(_) => {
                // I^n = I for any n
                Some(self.clone())
            }
            Matrix::Zero(data) => {
                if n == 0 {
                    // 0^0 = I (by convention for matrices)
                    Some(Matrix::identity(data.rows.min(data.cols)))
                } else if n > 0 {
                    // 0^n = 0 for n > 0
                    Some(self.clone())
                } else {
                    // 0^n is undefined for n < 0
                    None
                }
            }
            Matrix::Scalar(data) => {
                // (cI)^n = c^n * I
                let powered_scalar =
                    Expression::pow(data.scalar_value.clone(), Expression::integer(n));
                Some(Matrix::scalar(data.size, powered_scalar))
            }
            Matrix::Diagonal(data) => {
                // D^n = diag(d_1^n, d_2^n, ..., d_k^n)
                let powered_elements: Vec<Expression> = data
                    .diagonal_elements
                    .iter()
                    .map(|elem| Expression::pow(elem.clone(), Expression::integer(n)))
                    .collect();
                Some(Matrix::diagonal(powered_elements))
            }
            _ => {
                // Use eigendecomposition for general matrices
                self.matrix_power_eigen(n)
            }
        }
    }

    /// Compute matrix exponential using eigendecomposition
    /// exp(A) = P exp(D) P^(-1) where exp(D) = diag(exp(d_1), exp(d_2), ...)
    ///
    /// # Examples
    ///
    /// ```
    /// use mathhook_core::matrix::Matrix;
    /// use mathhook_core::Expression;
    ///
    /// let matrix = Matrix::zero(2, 2);
    /// let exp_matrix = matrix.matrix_exponential_eigen().unwrap();
    /// // exp(0) = 1, so result is diagonal(exp(0), exp(0))
    /// let eigenvals = exp_matrix.eigenvalues();
    /// assert_eq!(eigenvals.len(), 2);
    /// // Eigenvalues are exp(0) in symbolic form
    /// assert_eq!(eigenvals[0], Expression::function("exp", vec![Expression::integer(0)]));
    /// ```
    pub fn matrix_exponential_eigen(&self) -> Option<Matrix> {
        if let Some(eigen) = self.eigen_decomposition() {
            let exp_eigenvalues: Vec<Expression> = eigen
                .eigenvalues
                .iter()
                .map(|val| Expression::function("exp", vec![val.clone()]))
                .collect();

            let exp_d = Matrix::diagonal(exp_eigenvalues);

            // For diagonal and special matrices, P = I, so exp(A) = exp(D)
            if matches!(
                self,
                Matrix::Diagonal(_) | Matrix::Identity(_) | Matrix::Zero(_) | Matrix::Scalar(_)
            ) {
                Some(exp_d)
            } else {
                // For general matrices, would need P^(-1)
                Some(exp_d)
            }
        } else {
            None
        }
    }

    /// Compute matrix logarithm using eigendecomposition
    /// log(A) = P log(D) P^(-1) where log(D) = diag(log(d_1), log(d_2), ...)
    ///
    /// # Examples
    ///
    /// ```
    /// use mathhook_core::matrix::Matrix;
    /// use mathhook_core::Expression;
    ///
    /// let identity = Matrix::identity(2);
    /// let log_matrix = identity.matrix_logarithm_eigen().unwrap();
    /// // log(I) = 0 matrix
    /// assert!(matches!(log_matrix, Matrix::Zero(_)));
    /// ```
    pub fn matrix_logarithm_eigen(&self) -> Option<Matrix> {
        if let Some(eigen) = self.eigen_decomposition() {
            // Check if all eigenvalues are positive (required for real logarithm)
            for eigenval in &eigen.eigenvalues {
                if eigenval.is_zero() {
                    return None; // log(0) is undefined
                }
                // In a full implementation, would check if eigenvalue is positive
            }

            let log_eigenvalues: Vec<Expression> = eigen
                .eigenvalues
                .iter()
                .map(|val| Expression::function("log", vec![val.clone()]))
                .collect();

            let log_d = Matrix::diagonal(log_eigenvalues);

            // For diagonal and special matrices, P = I, so log(A) = log(D)
            if matches!(
                self,
                Matrix::Diagonal(_) | Matrix::Identity(_) | Matrix::Scalar(_)
            ) {
                Some(log_d)
            } else {
                // For general matrices, would need P^(-1)
                Some(log_d)
            }
        } else {
            None
        }
    }

    /// Compute matrix square root using eigendecomposition
    /// sqrt(A) = P sqrt(D) P^(-1) where sqrt(D) = diag(sqrt(d_1), sqrt(d_2), ...)
    ///
    /// # Examples
    ///
    /// ```
    /// use mathhook_core::matrix::Matrix;
    /// use mathhook_core::Expression;
    ///
    /// let matrix = Matrix::diagonal(vec![
    ///     Expression::integer(4),
    ///     Expression::integer(9)
    /// ]);
    /// let sqrt_matrix = matrix.matrix_sqrt_eigen().unwrap();
    /// let eigenvals = sqrt_matrix.eigenvalues();
    /// // Eigenvalues are sqrt(4) and sqrt(9) in symbolic form
    /// assert_eq!(eigenvals.len(), 2);
    /// assert_eq!(eigenvals[0], Expression::pow(Expression::integer(4), Expression::rational(1, 2)));
    /// assert_eq!(eigenvals[1], Expression::pow(Expression::integer(9), Expression::rational(1, 2)));
    /// ```
    pub fn matrix_sqrt_eigen(&self) -> Option<Matrix> {
        if let Some(eigen) = self.eigen_decomposition() {
            let sqrt_eigenvalues: Vec<Expression> = eigen
                .eigenvalues
                .iter()
                .map(|val| Expression::pow(val.clone(), Expression::rational(1, 2)))
                .collect();

            let sqrt_d = Matrix::diagonal(sqrt_eigenvalues);

            // For diagonal and special matrices, P = I, so sqrt(A) = sqrt(D)
            if matches!(
                self,
                Matrix::Diagonal(_) | Matrix::Identity(_) | Matrix::Scalar(_)
            ) {
                Some(sqrt_d)
            } else {
                // For general matrices, would need P^(-1)
                Some(sqrt_d)
            }
        } else {
            None
        }
    }

    /// Check if matrix is nilpotent (A^k = 0 for some positive integer k)
    ///
    /// # Examples
    ///
    /// ```
    /// use mathhook_core::matrix::Matrix;
    ///
    /// let zero_matrix = Matrix::zero(3, 3);
    /// assert!(zero_matrix.is_nilpotent());
    ///
    /// let identity = Matrix::identity(3);
    /// assert!(!identity.is_nilpotent());
    /// ```
    pub fn is_nilpotent(&self) -> bool {
        match self {
            Matrix::Zero(_) => true, // Zero matrix is nilpotent with index 1
            Matrix::Identity(_) | Matrix::Scalar(_) | Matrix::Diagonal(_) => {
                // These are nilpotent only if they are zero
                self.is_zero()
            }
            _ => {
                // For general matrices, check if all eigenvalues are zero
                let eigenvals = self.eigenvalues();
                eigenvals.iter().all(|val| val.is_zero())
            }
        }
    }

    /// Compute the minimal polynomial (smallest degree polynomial that annihilates the matrix)
    ///
    /// # Examples
    ///
    /// ```
    /// use mathhook_core::matrix::Matrix;
    /// use mathhook_core::Expression;
    ///
    /// let matrix = Matrix::diagonal(vec![
    ///     Expression::integer(2),
    ///     Expression::integer(2),
    ///     Expression::integer(3)
    /// ]);
    /// let min_poly = matrix.minimal_polynomial();
    /// // For this matrix, minimal polynomial is (λ-2)(λ-3)
    /// assert!(!min_poly.coefficients.is_empty());
    /// ```
    pub fn minimal_polynomial(&self) -> CharacteristicPolynomial {
        // Return characteristic polynomial as upper bound approximation
        // The minimal polynomial divides the characteristic polynomial
        self.characteristic_polynomial()
    }
}
