//! Matrix operations and utilities for the MathHook expression system
//!
//! This module provides comprehensive matrix operations using the unified matrix system.
//! All operations are optimized for different matrix types (identity, zero, diagonal, etc.)
//! while maintaining mathematical correctness and performance.

use crate::core::expression::unified_matrix::{Matrix, MatrixOps};
use crate::core::expression::Expression;
use crate::core::Number;

/// Matrix operations trait
///
/// Provides mathematical operations for matrices including addition, multiplication,
/// transpose, inverse, and other linear algebra operations.
pub trait MatrixOperations {
    /// Add two matrices
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::Expression;
    ///
    /// let a = Expression::matrix(vec![
    ///     vec![Expression::integer(1), Expression::integer(2)],
    ///     vec![Expression::integer(3), Expression::integer(4)]
    /// ]);
    /// let b = Expression::matrix(vec![
    ///     vec![Expression::integer(5), Expression::integer(6)],
    ///     vec![Expression::integer(7), Expression::integer(8)]
    /// ]);
    /// let result = a.matrix_add(&b);
    /// // Result: [[6, 8], [10, 12]]
    /// ```
    fn matrix_add(&self, other: &Expression) -> Expression;

    /// Subtract two matrices
    fn matrix_subtract(&self, other: &Expression) -> Expression;

    /// Multiply two matrices
    fn matrix_multiply(&self, other: &Expression) -> Expression;

    /// Multiply matrix by scalar
    fn matrix_scalar_multiply(&self, scalar: &Expression) -> Expression;

    /// Get matrix determinant
    fn matrix_determinant(&self) -> Expression;

    /// Get matrix transpose
    fn matrix_transpose(&self) -> Expression;

    /// Get matrix inverse
    fn matrix_inverse(&self) -> Expression;

    /// Get matrix trace (sum of diagonal elements)
    fn matrix_trace(&self) -> Expression;

    /// Raise matrix to a power
    fn matrix_power(&self, exponent: &Expression) -> Expression;

    /// Check if matrix is identity matrix
    fn is_identity_matrix(&self) -> bool;

    /// Check if matrix is zero matrix
    fn is_zero_matrix(&self) -> bool;

    /// Simplify matrix expression
    fn simplify_matrix(&self) -> Expression;
}

impl Expression {
    /// Get matrix dimensions for any matrix type
    ///
    /// Returns (rows, columns) for all matrix types.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::Expression;
    ///
    /// let matrix = Expression::matrix(vec![
    ///     vec![Expression::integer(1), Expression::integer(2)],
    ///     vec![Expression::integer(3), Expression::integer(4)]
    /// ]);
    /// assert_eq!(matrix.matrix_dimensions(), Some((2, 2)));
    ///
    /// let identity = Expression::identity_matrix(3);
    /// assert_eq!(identity.matrix_dimensions(), Some((3, 3)));
    /// ```
    pub fn matrix_dimensions(&self) -> Option<(usize, usize)> {
        match self {
            Expression::Matrix(matrix) => Some(matrix.dimensions()),
            _ => None,
        }
    }

    /// Check if expression is any kind of matrix
    ///
    /// Returns true for all matrix types in the unified system.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::Expression;
    ///
    /// let matrix = Expression::matrix(vec![vec![Expression::integer(1)]]);
    /// assert!(matrix.is_matrix());
    ///
    /// let identity = Expression::identity_matrix(2);
    /// assert!(identity.is_matrix());
    ///
    /// let number = Expression::integer(42);
    /// assert!(!number.is_matrix());
    /// ```
    pub fn is_matrix(&self) -> bool {
        matches!(self, Expression::Matrix(_))
    }

    /// Convert any matrix to dense matrix representation
    ///
    /// This method converts specialized matrix types to dense representation
    /// when explicit element access is needed.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::Expression;
    ///
    /// let identity = Expression::identity_matrix(2);
    /// let as_matrix = identity.as_matrix();
    /// // Results in: [[1, 0], [0, 1]]
    /// ```
    pub fn as_matrix(&self) -> Expression {
        match self {
            Expression::Matrix(matrix) => {
                // Convert any matrix type to dense matrix representation
                use crate::core::expression::matrix_types::MatrixData;

                let (rows, cols) = matrix.dimensions();
                let dense_rows: Vec<Vec<Expression>> = (0..rows)
                    .map(|i| (0..cols).map(|j| matrix.get_element(i, j)).collect())
                    .collect();

                Expression::Matrix(Box::new(Matrix::Dense(MatrixData { rows: dense_rows })))
            }
            _ => self.clone(),
        }
    }
}

impl MatrixOperations for Expression {
    fn matrix_add(&self, other: &Expression) -> Expression {
        match (self, other) {
            (Expression::Matrix(a), Expression::Matrix(b)) => {
                let result_matrix = a.matrix_add(b);
                Expression::Matrix(Box::new(result_matrix))
            }
            _ => Expression::function("undefined", vec![]),
        }
    }

    fn matrix_subtract(&self, other: &Expression) -> Expression {
        match (self, other) {
            (Expression::Matrix(a), Expression::Matrix(b)) => {
                // Implement subtraction as A - B = A + (-1 * B)
                let neg_b = b.scalar_multiply(&Expression::integer(-1));
                let result_matrix = a.matrix_add(&neg_b);
                Expression::Matrix(Box::new(result_matrix))
            }
            _ => Expression::function("undefined", vec![]),
        }
    }

    fn matrix_multiply(&self, other: &Expression) -> Expression {
        match (self, other) {
            (Expression::Matrix(a), Expression::Matrix(b)) => {
                let result_matrix = a.matrix_multiply(b);
                Expression::Matrix(Box::new(result_matrix))
            }
            _ => Expression::function("undefined", vec![]),
        }
    }

    fn matrix_scalar_multiply(&self, scalar: &Expression) -> Expression {
        match self {
            Expression::Matrix(matrix) => {
                let result_matrix = matrix.scalar_multiply(scalar);
                Expression::Matrix(Box::new(result_matrix))
            }
            _ => Expression::function("undefined", vec![]),
        }
    }

    fn matrix_determinant(&self) -> Expression {
        match self {
            Expression::Matrix(matrix) => matrix.determinant(),
            _ => Expression::function("undefined", vec![]),
        }
    }

    fn matrix_transpose(&self) -> Expression {
        match self {
            Expression::Matrix(matrix) => {
                let result_matrix = matrix.matrix_transpose();
                Expression::Matrix(Box::new(result_matrix))
            }
            _ => Expression::function("undefined", vec![]),
        }
    }

    fn matrix_inverse(&self) -> Expression {
        match self {
            Expression::Matrix(matrix) => {
                let result_matrix = matrix.matrix_inverse();
                Expression::Matrix(Box::new(result_matrix))
            }
            _ => Expression::function("undefined", vec![]),
        }
    }

    fn matrix_trace(&self) -> Expression {
        match self {
            Expression::Matrix(matrix) => matrix.trace(),
            _ => Expression::function("undefined", vec![]),
        }
    }

    fn matrix_power(&self, exponent: &Expression) -> Expression {
        match (self, exponent) {
            (Expression::Matrix(matrix), Expression::Number(Number::Integer(n))) => {
                let n = *n;
                // Handle special cases
                if n == 0 {
                    // A^0 = I (identity matrix of same size)
                    let (rows, cols) = matrix.dimensions();
                    if rows == cols {
                        return Expression::identity_matrix(rows);
                    } else {
                        return Expression::function("undefined", vec![]);
                    }
                }

                if n == 1 {
                    // A^1 = A
                    return self.clone();
                }

                // For identity matrices: I^n = I
                if matrix.is_identity() {
                    return self.clone();
                }

                // For zero matrices: 0^n = 0 (for n > 0)
                if matrix.is_zero() && n > 0 {
                    return self.clone();
                }

                // General case: repeated multiplication
                if n > 1 {
                    let mut result = self.clone();
                    for _ in 1..n {
                        result = result.matrix_multiply(self);
                    }
                    return result;
                }

                // Negative powers: A^(-n) = (A^(-1))^n
                if n < 0 {
                    let inverse = self.matrix_inverse();
                    return inverse.matrix_power(&Expression::number(-n as f64));
                }

                Expression::function("undefined", vec![])
            }
            _ => Expression::function("undefined", vec![]),
        }
    }

    fn is_identity_matrix(&self) -> bool {
        match self {
            Expression::Matrix(matrix) => matrix.is_identity(),
            _ => false,
        }
    }

    fn is_zero_matrix(&self) -> bool {
        match self {
            Expression::Matrix(matrix) => matrix.is_zero(),
            _ => false,
        }
    }

    fn simplify_matrix(&self) -> Expression {
        match self {
            Expression::Matrix(matrix) => {
                // Optimize the matrix to its most efficient representation
                let optimized = matrix.clone().optimize();
                Expression::Matrix(Box::new(optimized))
            }
            _ => self.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matrix_dimensions() {
        let matrix = Expression::matrix(vec![
            vec![Expression::number(1.0), Expression::number(2.0)],
            vec![Expression::number(3.0), Expression::number(4.0)],
        ]);
        assert_eq!(matrix.matrix_dimensions(), Some((2, 2)));

        let identity = Expression::identity_matrix(3);
        assert_eq!(identity.matrix_dimensions(), Some((3, 3)));
    }

    #[test]
    fn test_is_matrix() {
        let matrix = Expression::matrix(vec![vec![Expression::number(1.0)]]);
        assert!(matrix.is_matrix());

        let identity = Expression::identity_matrix(2);
        assert!(identity.is_matrix());

        let number = Expression::number(42.0);
        assert!(!number.is_matrix());
    }

    #[test]
    fn test_matrix_addition() {
        let a = Expression::matrix(vec![
            vec![Expression::number(1.0), Expression::number(2.0)],
            vec![Expression::number(3.0), Expression::number(4.0)],
        ]);
        let b = Expression::matrix(vec![
            vec![Expression::number(5.0), Expression::number(6.0)],
            vec![Expression::number(7.0), Expression::number(8.0)],
        ]);

        let result = a.matrix_add(&b);
        assert!(result.is_matrix());
    }

    #[test]
    fn test_identity_matrix_properties() {
        let identity = Expression::identity_matrix(3);

        // I^n = I
        let power_result = identity.matrix_power(&Expression::number(5.0));
        assert!(power_result.is_identity_matrix());

        // tr(I) = n
        let trace = identity.matrix_trace();
        assert_eq!(trace, Expression::number(3.0));
    }

    #[test]
    fn test_zero_matrix_properties() {
        let zero = Expression::zero_matrix(2, 2);

        assert!(zero.is_zero_matrix());

        // tr(0) = 0
        let trace = zero.matrix_trace();
        assert_eq!(trace, Expression::number(0.0));
    }
}
