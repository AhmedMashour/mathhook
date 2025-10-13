//! High-level matrix operations for Expression integration
//!
//! This module provides the bridge between the Expression system and the
//! unified matrix system, offering user-friendly matrix operations.

use super::{CoreMatrixOps, Matrix};
use crate::core::Expression;
use crate::simplify::Simplify;

/// High-level matrix operations trait for Expression
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
    /// use mathhook_core::matrix::operations::MatrixOperations;
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

    /// Check if matrix is diagonal
    fn is_diagonal(&self) -> bool;

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
    pub fn is_matrix(&self) -> bool {
        matches!(self, Expression::Matrix(_))
    }
}

impl MatrixOperations for Expression {
    fn matrix_add(&self, other: &Expression) -> Expression {
        match (self, other) {
            (Expression::Matrix(a), Expression::Matrix(b)) => {
                let result_matrix = a.add(b);
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
                let result_matrix = a.add(&neg_b);
                Expression::Matrix(Box::new(result_matrix))
            }
            _ => Expression::function("undefined", vec![]),
        }
    }

    fn matrix_multiply(&self, other: &Expression) -> Expression {
        match (self, other) {
            (Expression::Matrix(a), Expression::Matrix(b)) => {
                let result_matrix = a.multiply(b);
                Expression::Matrix(Box::new(result_matrix))
            }
            _ => Expression::function("undefined", vec![]),
        }
    }

    fn matrix_scalar_multiply(&self, scalar: &Expression) -> Expression {
        match self {
            Expression::Matrix(matrix) => {
                let result_matrix = matrix.scalar_multiply(scalar);
                let result = Expression::Matrix(Box::new(result_matrix));
                // Simplify the result to ensure mathematical correctness
                result.simplify()
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
                let result_matrix = matrix.transpose();
                let result = Expression::Matrix(Box::new(result_matrix));
                // Simplify the result to ensure mathematical correctness
                result.simplify()
            }
            _ => Expression::function("undefined", vec![]),
        }
    }

    fn matrix_inverse(&self) -> Expression {
        match self {
            Expression::Matrix(matrix) => {
                let result_matrix = matrix.inverse();
                let result = Expression::Matrix(Box::new(result_matrix));
                // Simplify the result to ensure mathematical correctness
                result.simplify()
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
            (Expression::Matrix(matrix), Expression::Number(num)) => {
                // Handle integer powers
                if let crate::core::Number::Integer(n) = num {
                    if *n == 0 {
                        // A^0 = I (identity matrix of same size)
                        let (rows, cols) = matrix.dimensions();
                        if rows == cols {
                            return Expression::Matrix(Box::new(Matrix::identity(rows)));
                        }
                    } else if *n == 1 {
                        // A^1 = A
                        return self.clone();
                    } else if *n > 1 {
                        // Repeated multiplication for positive integer powers
                        let mut result = self.clone();
                        for _ in 1..*n {
                            result = result.matrix_multiply(self);
                        }
                        return result.simplify();
                    }
                }
                // For other cases, return function call
                Expression::function("matrix_power", vec![self.clone(), exponent.clone()])
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

    fn is_diagonal(&self) -> bool {
        match self {
            Expression::Matrix(matrix) => matrix.is_diagonal(),
            _ => false,
        }
    }

    fn simplify_matrix(&self) -> Expression {
        match self {
            Expression::Matrix(matrix) => {
                let optimized_matrix = matrix.clone().optimize();
                Expression::Matrix(Box::new(optimized_matrix)).simplify()
            }
            _ => self.clone(),
        }
    }
}
