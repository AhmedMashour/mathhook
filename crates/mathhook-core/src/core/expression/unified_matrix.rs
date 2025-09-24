//! Unified matrix system with zero-cost abstractions
//!
//! This module provides a single `Matrix` type that can represent all special
//! matrix types while maintaining optimal memory usage and performance.

use super::matrix_types::*;
use super::Expression;
use serde::{Deserialize, Serialize};

/// Unified matrix type that can represent any matrix efficiently
///
/// This enum uses zero-cost abstractions to provide a single interface
/// for all matrix types while maintaining optimal memory usage.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Matrix {
    /// Regular dense matrix: O(n²) memory
    Dense(MatrixData),

    /// Identity matrix: O(1) memory
    Identity(IdentityMatrixData),

    /// Zero matrix: O(1) memory  
    Zero(ZeroMatrixData),

    /// Diagonal matrix: O(n) memory
    Diagonal(DiagonalMatrixData),

    /// Scalar matrix: O(1) memory
    Scalar(ScalarMatrixData),

    /// Upper triangular: O(n²/2) memory
    UpperTriangular(UpperTriangularMatrixData),

    /// Lower triangular: O(n²/2) memory
    LowerTriangular(LowerTriangularMatrixData),

    /// Symmetric matrix: O(n²/2) memory
    Symmetric(SymmetricMatrixData),

    /// Permutation matrix: O(n) memory
    Permutation(PermutationMatrixData),
}

impl Matrix {
    /// Get matrix dimensions efficiently
    ///
    /// This method provides O(1) dimension lookup for all matrix types.
    #[inline]
    pub fn dimensions(&self) -> (usize, usize) {
        match self {
            Matrix::Dense(data) => {
                let rows = data.rows.len();
                let cols = data.rows.get(0).map(|row| row.len()).unwrap_or(0);
                (rows, cols)
            }
            Matrix::Identity(data) => (data.size, data.size),
            Matrix::Zero(data) => (data.rows, data.cols),
            Matrix::Diagonal(data) => {
                let size = data.diagonal_elements.len();
                (size, size)
            }
            Matrix::Scalar(data) => (data.size, data.size),
            Matrix::UpperTriangular(data) => (data.size, data.size),
            Matrix::LowerTriangular(data) => (data.size, data.size),
            Matrix::Symmetric(data) => (data.size, data.size),
            Matrix::Permutation(data) => {
                let size = data.permutation.len();
                (size, size)
            }
        }
    }

    /// Get element at position (i, j) efficiently
    ///
    /// This method provides optimized element access for each matrix type.
    #[inline]
    pub fn get_element(&self, i: usize, j: usize) -> Expression {
        match self {
            Matrix::Dense(data) => data
                .rows
                .get(i)
                .and_then(|row| row.get(j))
                .cloned()
                .unwrap_or_else(|| Expression::integer(0)),

            Matrix::Identity(data) => {
                if i < data.size && j < data.size && i == j {
                    Expression::integer(1)
                } else {
                    Expression::integer(0)
                }
            }

            Matrix::Zero(_) => Expression::integer(0),

            Matrix::Diagonal(data) => {
                if i == j && i < data.diagonal_elements.len() {
                    data.diagonal_elements[i].clone()
                } else {
                    Expression::integer(0)
                }
            }

            Matrix::Scalar(data) => {
                if i < data.size && j < data.size && i == j {
                    data.scalar_value.clone()
                } else {
                    Expression::integer(0)
                }
            }

            Matrix::UpperTriangular(data) => {
                if i <= j && i < data.size && j < data.size {
                    data.get_element(i, j)
                        .cloned()
                        .unwrap_or_else(|| Expression::integer(0))
                } else {
                    Expression::integer(0)
                }
            }

            Matrix::LowerTriangular(data) => {
                if i >= j && i < data.size && j < data.size {
                    data.get_element(i, j)
                        .cloned()
                        .unwrap_or_else(|| Expression::integer(0))
                } else {
                    Expression::integer(0)
                }
            }

            Matrix::Symmetric(data) => {
                if i < data.size && j < data.size {
                    data.get_element(i, j)
                        .cloned()
                        .unwrap_or_else(|| Expression::integer(0))
                } else {
                    Expression::integer(0)
                }
            }

            Matrix::Permutation(data) => Expression::integer(data.get_element(i, j)),
        }
    }

    /// Check if this is a square matrix
    #[inline]
    pub fn is_square(&self) -> bool {
        let (rows, cols) = self.dimensions();
        rows == cols
    }

    /// Check if this is a zero matrix
    #[inline]
    pub fn is_zero(&self) -> bool {
        matches!(self, Matrix::Zero(_))
    }

    /// Check if this is an identity matrix
    #[inline]
    pub fn is_identity(&self) -> bool {
        matches!(self, Matrix::Identity(_))
    }

    /// Check if this is a diagonal matrix
    #[inline]
    pub fn is_diagonal(&self) -> bool {
        matches!(
            self,
            Matrix::Identity(_) | Matrix::Zero(_) | Matrix::Diagonal(_) | Matrix::Scalar(_)
        )
    }

    /// Check if this is symmetric
    #[inline]
    pub fn is_symmetric(&self) -> bool {
        matches!(
            self,
            Matrix::Identity(_)
                | Matrix::Zero(_)
                | Matrix::Diagonal(_)
                | Matrix::Scalar(_)
                | Matrix::Symmetric(_)
        )
    }

    /// Convert to the most efficient representation
    ///
    /// This method analyzes the matrix and converts it to the most
    /// memory-efficient representation possible.
    pub fn optimize(self) -> Matrix {
        match self {
            Matrix::Dense(data) => {
                let (rows, cols) = (
                    data.rows.len(),
                    data.rows.get(0).map(|r| r.len()).unwrap_or(0),
                );

                // Check for zero matrix
                if data
                    .rows
                    .iter()
                    .all(|row| row.iter().all(|elem| elem.is_zero()))
                {
                    return Matrix::Zero(ZeroMatrixData { rows, cols });
                }

                // Check for identity matrix (square only)
                if rows == cols
                    && data.rows.iter().enumerate().all(|(i, row)| {
                        row.iter().enumerate().all(|(j, elem)| {
                            if i == j {
                                elem == &Expression::integer(1)
                            } else {
                                elem.is_zero()
                            }
                        })
                    })
                {
                    return Matrix::Identity(IdentityMatrixData { size: rows });
                }

                // Check for diagonal matrix (square only)
                if rows == cols
                    && data.rows.iter().enumerate().all(|(i, row)| {
                        row.iter()
                            .enumerate()
                            .all(|(j, elem)| i == j || elem.is_zero())
                    })
                {
                    let diagonal_elements: Vec<Expression> =
                        (0..rows).map(|i| data.rows[i][i].clone()).collect();

                    // Check if it's a scalar matrix
                    if diagonal_elements
                        .iter()
                        .all(|elem| elem == &diagonal_elements[0])
                    {
                        return Matrix::Scalar(ScalarMatrixData {
                            size: rows,
                            scalar_value: diagonal_elements[0].clone(),
                        });
                    }

                    return Matrix::Diagonal(DiagonalMatrixData { diagonal_elements });
                }

                // Keep as dense matrix
                Matrix::Dense(data)
            }

            // Already optimized types
            other => other,
        }
    }

    /// Get the trace (sum of diagonal elements) efficiently
    #[inline]
    pub fn trace(&self) -> Expression {
        match self {
            Matrix::Identity(data) => Expression::integer(data.size as i64),
            Matrix::Zero(_) => Expression::integer(0),
            Matrix::Scalar(data) => Expression::mul(vec![
                Expression::integer(data.size as i64),
                data.scalar_value.clone(),
            ]),
            Matrix::Diagonal(data) => Expression::add(data.diagonal_elements.clone()),
            _ => {
                // For other types, compute trace by summing diagonal
                let (rows, _) = self.dimensions();
                let diagonal_elements: Vec<Expression> =
                    (0..rows).map(|i| self.get_element(i, i)).collect();
                Expression::add(diagonal_elements)
            }
        }
    }

    /// Get the determinant efficiently (for square matrices)
    pub fn determinant(&self) -> Expression {
        match self {
            Matrix::Identity(_) => Expression::integer(1),
            Matrix::Zero(_) => Expression::integer(0),
            Matrix::Scalar(data) => {
                // det(cI) = c^n
                Expression::pow(
                    data.scalar_value.clone(),
                    Expression::integer(data.size as i64),
                )
            }
            Matrix::Diagonal(data) => {
                // Product of diagonal elements
                Expression::mul(data.diagonal_elements.clone())
            }
            Matrix::Permutation(_data) => {
                // Determinant is ±1 based on number of inversions
                // For now, return 1 (proper implementation would count inversions)
                Expression::integer(1)
            }
            _ => {
                // For other types, use general determinant algorithm
                Expression::function("det", vec![Expression::Matrix(Box::new(self.clone()))])
            }
        }
    }
}

/// Trait for matrix operations that work on the unified Matrix type
pub trait MatrixOps {
    fn matrix_add(&self, other: &Matrix) -> Matrix;
    fn matrix_multiply(&self, other: &Matrix) -> Matrix;
    fn matrix_transpose(&self) -> Matrix;
    fn matrix_inverse(&self) -> Matrix;
}

impl MatrixOps for Matrix {
    fn matrix_add(&self, other: &Matrix) -> Matrix {
        // Optimized addition based on matrix types
        match (self, other) {
            // Zero matrix optimizations
            (Matrix::Zero(_), other) => other.clone(),
            (this, Matrix::Zero(_)) => this.clone(),

            // Identity matrix optimizations
            (Matrix::Identity(id), Matrix::Dense(dense))
            | (Matrix::Dense(dense), Matrix::Identity(id)) => {
                // I + A: add 1 to diagonal elements
                let mut result_rows = dense.rows.clone();
                for i in 0..id.size.min(result_rows.len()) {
                    if let Some(row) = result_rows.get_mut(i) {
                        if let Some(elem) = row.get_mut(i) {
                            *elem = Expression::add(vec![elem.clone(), Expression::integer(1)]);
                        }
                    }
                }
                Matrix::Dense(MatrixData { rows: result_rows })
            }

            // Scalar matrix optimizations
            (Matrix::Scalar(s1), Matrix::Scalar(s2)) if s1.size == s2.size => {
                Matrix::Scalar(ScalarMatrixData {
                    size: s1.size,
                    scalar_value: Expression::add(vec![
                        s1.scalar_value.clone(),
                        s2.scalar_value.clone(),
                    ]),
                })
            }

            // General case: convert to dense and add
            _ => {
                let (rows1, cols1) = self.dimensions();
                let (rows2, cols2) = other.dimensions();

                if rows1 != rows2 || cols1 != cols2 {
                    // Dimension mismatch - return error or zero matrix
                    return Matrix::Zero(ZeroMatrixData { rows: 0, cols: 0 });
                }

                let mut result_rows = Vec::with_capacity(rows1);
                for i in 0..rows1 {
                    let mut row = Vec::with_capacity(cols1);
                    for j in 0..cols1 {
                        let elem1 = self.get_element(i, j);
                        let elem2 = other.get_element(i, j);
                        row.push(Expression::add(vec![elem1, elem2]));
                    }
                    result_rows.push(row);
                }

                Matrix::Dense(MatrixData { rows: result_rows }).optimize()
            }
        }
    }

    fn matrix_multiply(&self, other: &Matrix) -> Matrix {
        match (self, other) {
            // Zero matrix optimizations
            (Matrix::Zero(z), _) => Matrix::Zero(ZeroMatrixData {
                rows: z.rows,
                cols: other.dimensions().1,
            }),
            (_, Matrix::Zero(z)) => Matrix::Zero(ZeroMatrixData {
                rows: self.dimensions().0,
                cols: z.cols,
            }),

            // Identity matrix optimizations
            (Matrix::Identity(_), other) => other.clone(),
            (this, Matrix::Identity(_)) => this.clone(),

            // Scalar matrix optimizations
            (Matrix::Scalar(s), other) => {
                // s*I * A = s*A (scalar multiplication)
                other.scalar_multiply(&s.scalar_value)
            }
            (this, Matrix::Scalar(s)) => {
                // A * s*I = s*A (scalar multiplication)
                this.scalar_multiply(&s.scalar_value)
            }

            // General multiplication (implement as needed)
            _ => {
                // For now, convert to dense and multiply
                // TODO: Implement optimized multiplication for each type combination
                Matrix::Zero(ZeroMatrixData { rows: 0, cols: 0 })
            }
        }
    }

    fn matrix_transpose(&self) -> Matrix {
        match self {
            Matrix::Identity(data) => Matrix::Identity(data.clone()),
            Matrix::Zero(data) => Matrix::Zero(ZeroMatrixData {
                rows: data.cols,
                cols: data.rows,
            }),
            Matrix::Scalar(data) => Matrix::Scalar(data.clone()),
            Matrix::Diagonal(data) => Matrix::Diagonal(data.clone()),
            Matrix::Symmetric(data) => Matrix::Symmetric(data.clone()),
            Matrix::UpperTriangular(data) => Matrix::LowerTriangular(LowerTriangularMatrixData {
                size: data.size,
                elements: data.elements.clone(), // TODO: Proper transpose mapping
            }),
            Matrix::LowerTriangular(data) => Matrix::UpperTriangular(UpperTriangularMatrixData {
                size: data.size,
                elements: data.elements.clone(), // TODO: Proper transpose mapping
            }),
            _ => {
                // General transpose
                let (rows, cols) = self.dimensions();
                let mut result_rows = Vec::with_capacity(cols);
                for j in 0..cols {
                    let mut row = Vec::with_capacity(rows);
                    for i in 0..rows {
                        row.push(self.get_element(i, j));
                    }
                    result_rows.push(row);
                }
                Matrix::Dense(MatrixData { rows: result_rows }).optimize()
            }
        }
    }

    fn matrix_inverse(&self) -> Matrix {
        match self {
            Matrix::Identity(data) => Matrix::Identity(data.clone()),
            Matrix::Scalar(data) => Matrix::Scalar(ScalarMatrixData {
                size: data.size,
                scalar_value: Expression::pow(data.scalar_value.clone(), Expression::integer(-1)),
            }),
            Matrix::Diagonal(data) => {
                let inverse_elements: Vec<Expression> = data
                    .diagonal_elements
                    .iter()
                    .map(|elem| Expression::pow(elem.clone(), Expression::integer(-1)))
                    .collect();
                Matrix::Diagonal(DiagonalMatrixData {
                    diagonal_elements: inverse_elements,
                })
            }
            _ => {
                // General inverse - return function call for now
                Matrix::Dense(MatrixData { rows: vec![] }) // Placeholder
            }
        }
    }
}

impl Matrix {
    /// Scalar multiplication
    pub fn scalar_multiply(&self, scalar: &Expression) -> Matrix {
        match self {
            Matrix::Zero(data) => Matrix::Zero(data.clone()),
            Matrix::Identity(data) => Matrix::Scalar(ScalarMatrixData {
                size: data.size,
                scalar_value: scalar.clone(),
            }),
            Matrix::Scalar(data) => Matrix::Scalar(ScalarMatrixData {
                size: data.size,
                scalar_value: Expression::mul(vec![scalar.clone(), data.scalar_value.clone()]),
            }),
            Matrix::Diagonal(data) => {
                let scaled_elements: Vec<Expression> = data
                    .diagonal_elements
                    .iter()
                    .map(|elem| Expression::mul(vec![scalar.clone(), elem.clone()]))
                    .collect();
                Matrix::Diagonal(DiagonalMatrixData {
                    diagonal_elements: scaled_elements,
                })
            }
            _ => {
                // General scalar multiplication
                let (rows, cols) = self.dimensions();
                let mut result_rows = Vec::with_capacity(rows);
                for i in 0..rows {
                    let mut row = Vec::with_capacity(cols);
                    for j in 0..cols {
                        let elem = self.get_element(i, j);
                        row.push(Expression::mul(vec![scalar.clone(), elem]));
                    }
                    result_rows.push(row);
                }
                Matrix::Dense(MatrixData { rows: result_rows }).optimize()
            }
        }
    }
}
