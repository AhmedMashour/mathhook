//! Unified matrix system with zero-cost abstractions
//!
//! This module provides a single `Matrix` type that can represent all special
//! matrix types while maintaining optimal memory usage and performance.

use crate::core::Expression;
use crate::matrix::types::*;
use crate::simplify::Simplify;
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
        match self {
            Matrix::Identity(_) => true,
            Matrix::Scalar(data) => {
                // A scalar matrix with value 1 is an identity matrix
                data.scalar_value == Expression::integer(1)
            }
            _ => false,
        }
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

                    // Check if it's an identity matrix (all diagonal elements are 1)
                    if diagonal_elements
                        .iter()
                        .all(|elem| elem == &Expression::integer(1))
                    {
                        return Matrix::Identity(IdentityMatrixData { size: rows });
                    }

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

            // Optimize diagonal matrices
            Matrix::Diagonal(data) => {
                // Check if it's an identity matrix (all diagonal elements are 1)
                if data
                    .diagonal_elements
                    .iter()
                    .all(|elem| elem == &Expression::integer(1))
                {
                    return Matrix::Identity(IdentityMatrixData {
                        size: data.diagonal_elements.len(),
                    });
                }

                // Check if it's a zero matrix (all diagonal elements are 0)
                if data.diagonal_elements.iter().all(|elem| elem.is_zero()) {
                    let size = data.diagonal_elements.len();
                    return Matrix::Zero(ZeroMatrixData {
                        rows: size,
                        cols: size,
                    });
                }

                // Check if it's a scalar matrix (all diagonal elements are the same)
                if data.diagonal_elements.len() > 0
                    && data
                        .diagonal_elements
                        .iter()
                        .all(|elem| elem == &data.diagonal_elements[0])
                {
                    return Matrix::Scalar(ScalarMatrixData {
                        size: data.diagonal_elements.len(),
                        scalar_value: data.diagonal_elements[0].clone(),
                    });
                }

                // Keep as diagonal
                Matrix::Diagonal(data)
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
            ])
            .simplify(),
            Matrix::Diagonal(data) => Expression::add(data.diagonal_elements.clone()).simplify(),
            _ => {
                // For other types, compute trace by summing diagonal
                let (rows, _) = self.dimensions();
                let diagonal_elements: Vec<Expression> =
                    (0..rows).map(|i| self.get_element(i, i)).collect();
                Expression::add(diagonal_elements).simplify()
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
                .simplify()
            }
            Matrix::Diagonal(data) => {
                // Product of diagonal elements
                Expression::mul(data.diagonal_elements.clone()).simplify()
            }
            Matrix::Permutation(_data) => {
                // Determinant is ±1 based on number of inversions
                // Return 1 as simplified determinant (complete implementation would count inversions)
                Expression::integer(1)
            }
            _ => {
                // For other types, implement general determinant calculation
                let (rows, cols) = self.dimensions();
                if rows != cols {
                    // Non-square matrix has no determinant
                    return Expression::function("undefined", vec![]);
                }

                if rows == 0 {
                    return Expression::integer(1); // Empty matrix determinant is 1
                }

                if rows == 1 {
                    return self.get_element(0, 0);
                }

                if rows == 2 {
                    // 2x2 determinant: ad - bc
                    let a = self.get_element(0, 0);
                    let b = self.get_element(0, 1);
                    let c = self.get_element(1, 0);
                    let d = self.get_element(1, 1);

                    let ad = Expression::mul(vec![a, d]);
                    let bc = Expression::mul(vec![b, c]);
                    return Expression::add(vec![
                        ad,
                        Expression::mul(vec![Expression::integer(-1), bc]),
                    ])
                    .simplify();
                }

                // For larger matrices, use cofactor expansion along first row
                let mut terms = Vec::new();
                for j in 0..cols {
                    let element = self.get_element(0, j);
                    if element.is_zero() {
                        continue; // Skip zero elements for efficiency
                    }

                    // Create minor matrix (remove row 0 and column j)
                    let mut minor_rows = Vec::new();
                    for i in 1..rows {
                        let mut minor_row = Vec::new();
                        for k in 0..cols {
                            if k != j {
                                minor_row.push(self.get_element(i, k));
                            }
                        }
                        minor_rows.push(minor_row);
                    }

                    if !minor_rows.is_empty() && !minor_rows[0].is_empty() {
                        let minor_matrix = Matrix::Dense(MatrixData { rows: minor_rows });
                        let minor_det = minor_matrix.determinant();

                        // Apply sign based on position
                        let sign = if j % 2 == 0 { 1 } else { -1 };
                        let cofactor =
                            Expression::mul(vec![Expression::integer(sign), element, minor_det]);
                        terms.push(cofactor);
                    }
                }

                if terms.is_empty() {
                    Expression::integer(0)
                } else {
                    Expression::add(terms).simplify()
                }
            }
        }
    }
}

/// Core matrix operations that work directly on Matrix types
pub trait CoreMatrixOps {
    fn add(&self, other: &Matrix) -> Matrix;
    fn multiply(&self, other: &Matrix) -> Matrix;
    fn transpose(&self) -> Matrix;
    fn inverse(&self) -> Matrix;
}

impl CoreMatrixOps for Matrix {
    fn add(&self, other: &Matrix) -> Matrix {
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

            // Diagonal matrix optimizations
            (Matrix::Diagonal(d1), Matrix::Diagonal(d2))
                if d1.diagonal_elements.len() == d2.diagonal_elements.len() =>
            {
                let result_elements: Vec<Expression> = d1
                    .diagonal_elements
                    .iter()
                    .zip(d2.diagonal_elements.iter())
                    .map(|(a, b)| Expression::add(vec![a.clone(), b.clone()]).simplify())
                    .collect();
                Matrix::Diagonal(DiagonalMatrixData {
                    diagonal_elements: result_elements,
                })
            }

            // Identity + Diagonal = Diagonal with 1s added to diagonal
            (Matrix::Identity(id), Matrix::Diagonal(diag))
            | (Matrix::Diagonal(diag), Matrix::Identity(id))
                if diag.diagonal_elements.len() == id.size =>
            {
                let result_elements: Vec<Expression> = diag
                    .diagonal_elements
                    .iter()
                    .map(|elem| {
                        Expression::add(vec![elem.clone(), Expression::integer(1)]).simplify()
                    })
                    .collect();
                Matrix::Diagonal(DiagonalMatrixData {
                    diagonal_elements: result_elements,
                })
            }

            // Scalar matrix optimizations
            (Matrix::Scalar(s1), Matrix::Scalar(s2)) if s1.size == s2.size => {
                Matrix::Scalar(ScalarMatrixData {
                    size: s1.size,
                    scalar_value: Expression::add(vec![
                        s1.scalar_value.clone(),
                        s2.scalar_value.clone(),
                    ])
                    .simplify(),
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
                        // Simplify the addition to ensure mathematical correctness
                        let sum = Expression::add(vec![elem1, elem2]).simplify();
                        row.push(sum);
                    }
                    result_rows.push(row);
                }

                Matrix::Dense(MatrixData { rows: result_rows }).optimize()
            }
        }
    }

    fn multiply(&self, other: &Matrix) -> Matrix {
        match (self, other) {
            // Zero matrix optimizations
            (Matrix::Zero(_), _) => Matrix::Zero(ZeroMatrixData {
                rows: self.dimensions().0,
                cols: other.dimensions().1,
            }),
            (_, Matrix::Zero(_)) => Matrix::Zero(ZeroMatrixData {
                rows: self.dimensions().0,
                cols: other.dimensions().1,
            }),

            // Identity matrix optimizations
            (Matrix::Identity(_), other) => other.clone(),
            (this, Matrix::Identity(_)) => this.clone(),

            // Diagonal matrix multiplication optimizations
            (Matrix::Diagonal(d1), Matrix::Diagonal(d2))
                if d1.diagonal_elements.len() == d2.diagonal_elements.len() =>
            {
                // D1 * D2 = diagonal with element-wise multiplication
                let result_elements: Vec<Expression> = d1
                    .diagonal_elements
                    .iter()
                    .zip(d2.diagonal_elements.iter())
                    .map(|(a, b)| Expression::mul(vec![a.clone(), b.clone()]))
                    .collect();
                Matrix::Diagonal(DiagonalMatrixData {
                    diagonal_elements: result_elements,
                })
            }

            // Scalar matrix optimizations
            (Matrix::Scalar(s1), Matrix::Scalar(s2)) if s1.size == s2.size => {
                // (c*I) * (d*I) = (c*d)*I
                let product_scalar =
                    Expression::mul(vec![s1.scalar_value.clone(), s2.scalar_value.clone()]);
                Matrix::Scalar(ScalarMatrixData {
                    size: s1.size,
                    scalar_value: product_scalar,
                })
            }
            (Matrix::Scalar(s), other) => {
                // s*I * A = s*A (scalar multiplication)
                other.scalar_multiply(&s.scalar_value)
            }
            (this, Matrix::Scalar(s)) => {
                // A * s*I = s*A (scalar multiplication)
                this.scalar_multiply(&s.scalar_value)
            }

            // General multiplication - implement proper matrix multiplication
            _ => {
                let (rows1, cols1) = self.dimensions();
                let (rows2, cols2) = other.dimensions();

                // Check dimension compatibility for multiplication
                if cols1 != rows2 {
                    // Incompatible dimensions - return zero matrix with correct dimensions
                    return Matrix::Zero(ZeroMatrixData {
                        rows: rows1,
                        cols: cols2,
                    });
                }

                // Perform matrix multiplication
                let mut result_rows = Vec::with_capacity(rows1);
                for i in 0..rows1 {
                    let mut row = Vec::with_capacity(cols2);
                    for j in 0..cols2 {
                        let mut sum_terms = Vec::with_capacity(cols1);
                        for k in 0..cols1 {
                            let elem1 = self.get_element(i, k);
                            let elem2 = other.get_element(k, j);
                            sum_terms.push(Expression::mul(vec![elem1, elem2]));
                        }
                        let sum = Expression::add(sum_terms);
                        row.push(sum);
                    }
                    result_rows.push(row);
                }

                Matrix::Dense(MatrixData { rows: result_rows })
            }
        }
    }

    fn transpose(&self) -> Matrix {
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

    fn inverse(&self) -> Matrix {
        match self {
            Matrix::Identity(data) => Matrix::Identity(data.clone()),
            Matrix::Scalar(data) => {
                // For scalar matrix cI, inverse is (1/c)I
                let inverse_scalar =
                    Expression::pow(data.scalar_value.clone(), Expression::integer(-1)).simplify();
                Matrix::Scalar(ScalarMatrixData {
                    size: data.size,
                    scalar_value: inverse_scalar,
                })
            }
            Matrix::Diagonal(data) => {
                let inverse_elements: Vec<Expression> = data
                    .diagonal_elements
                    .iter()
                    .map(|elem| Expression::pow(elem.clone(), Expression::integer(-1)).simplify())
                    .collect();
                Matrix::Diagonal(DiagonalMatrixData {
                    diagonal_elements: inverse_elements,
                })
            }
            _ => {
                // General inverse using Gauss-Jordan elimination
                self.gauss_jordan_inverse()
            }
        }
    }
}

impl Matrix {
    /// Compute matrix inverse using Gauss-Jordan elimination
    fn gauss_jordan_inverse(&self) -> Matrix {
        let (n, _) = self.dimensions();

        // Create augmented matrix [A | I]
        let mut augmented = Vec::new();
        for i in 0..n {
            let mut row = Vec::new();
            // Add original matrix elements
            for j in 0..n {
                row.push(self.get_element(i, j));
            }
            // Add identity matrix elements
            for j in 0..n {
                if i == j {
                    row.push(Expression::integer(1));
                } else {
                    row.push(Expression::integer(0));
                }
            }
            augmented.push(row);
        }

        // Perform Gauss-Jordan elimination
        for i in 0..n {
            // Find pivot
            let mut pivot_row = i;
            for k in (i + 1)..n {
                if !augmented[k][i].is_zero() && augmented[pivot_row][i].is_zero() {
                    pivot_row = k;
                }
            }

            // Swap rows if needed
            if pivot_row != i {
                augmented.swap(i, pivot_row);
            }

            // Check for zero pivot (singular matrix)
            if augmented[i][i].is_zero() {
                return Matrix::Dense(MatrixData { rows: vec![] }); // Return empty matrix for singular
            }

            // Scale pivot row
            let pivot = augmented[i][i].clone();
            for j in 0..(2 * n) {
                // Use canonical form for division: a / b = a * b^(-1)
                augmented[i][j] = Expression::mul(vec![
                    augmented[i][j].clone(),
                    Expression::pow(pivot.clone(), Expression::integer(-1)),
                ])
                .simplify();
            }

            // Eliminate column
            for k in 0..n {
                if k != i {
                    let factor = augmented[k][i].clone();
                    for j in 0..(2 * n) {
                        let subtract_term =
                            Expression::mul(vec![factor.clone(), augmented[i][j].clone()]);
                        augmented[k][j] = Expression::add(vec![
                            augmented[k][j].clone(),
                            Expression::mul(vec![Expression::integer(-1), subtract_term]),
                        ])
                        .simplify();
                    }
                }
            }
        }

        // Extract inverse matrix from right half of augmented matrix
        let mut inverse_rows = Vec::new();
        for i in 0..n {
            let mut row = Vec::new();
            for j in n..(2 * n) {
                row.push(augmented[i][j].clone());
            }
            inverse_rows.push(row);
        }

        Matrix::Dense(MatrixData { rows: inverse_rows })
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
                scalar_value: Expression::mul(vec![scalar.clone(), data.scalar_value.clone()])
                    .simplify(),
            }),
            Matrix::Diagonal(data) => {
                let scaled_elements: Vec<Expression> = data
                    .diagonal_elements
                    .iter()
                    .map(|elem| Expression::mul(vec![scalar.clone(), elem.clone()]).simplify())
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
                        // Simplify the multiplication to ensure mathematical correctness
                        let product = Expression::mul(vec![scalar.clone(), elem]).simplify();
                        row.push(product);
                    }
                    result_rows.push(row);
                }
                Matrix::Dense(MatrixData { rows: result_rows }).optimize()
            }
        }
    }

    // ========== MATRIX CONSTRUCTORS ==========

    /// Create a dense matrix from rows
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::matrix::Matrix;
    /// use mathhook_core::Expression;
    ///
    /// let matrix = Matrix::dense(vec![
    ///     vec![Expression::integer(1), Expression::integer(2)],
    ///     vec![Expression::integer(3), Expression::integer(4)]
    /// ]);
    /// ```
    pub fn dense(rows: Vec<Vec<Expression>>) -> Self {
        Matrix::Dense(MatrixData { rows }).optimize()
    }

    /// Create an identity matrix of given size
    /// Memory efficient: O(1) storage vs O(n²) for dense matrix
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::matrix::Matrix;
    ///
    /// let identity = Matrix::identity(3);
    /// assert_eq!(identity.dimensions(), (3, 3));
    /// assert!(identity.is_identity());
    /// ```
    pub fn identity(size: usize) -> Self {
        Matrix::Identity(IdentityMatrixData { size })
    }

    /// Create a zero matrix of given dimensions
    /// Memory efficient: O(1) storage vs O(n*m) for dense matrix
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::matrix::Matrix;
    ///
    /// let zero = Matrix::zero(2, 3);
    /// assert_eq!(zero.dimensions(), (2, 3));
    /// assert!(zero.is_zero());
    /// ```
    pub fn zero(rows: usize, cols: usize) -> Self {
        Matrix::Zero(ZeroMatrixData { rows, cols })
    }

    /// Create a diagonal matrix from diagonal elements
    /// Memory efficient: O(n) storage vs O(n²) for dense matrix
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::matrix::Matrix;
    /// use mathhook_core::Expression;
    ///
    /// let diag = Matrix::diagonal(vec![
    ///     Expression::integer(1),
    ///     Expression::integer(2),
    ///     Expression::integer(3)
    /// ]);
    /// assert_eq!(diag.dimensions(), (3, 3));
    /// assert!(diag.is_diagonal());
    /// ```
    pub fn diagonal(diagonal_elements: Vec<Expression>) -> Self {
        Matrix::Diagonal(DiagonalMatrixData { diagonal_elements }).optimize()
    }

    /// Create a scalar matrix (c*I)
    /// Memory efficient: O(1) storage vs O(n²) for dense matrix
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::matrix::Matrix;
    /// use mathhook_core::Expression;
    ///
    /// let scalar = Matrix::scalar(3, Expression::integer(5));
    /// // Represents 5*I (5 times identity matrix)
    /// ```
    pub fn scalar(size: usize, scalar_value: Expression) -> Self {
        Matrix::Scalar(ScalarMatrixData { size, scalar_value })
    }

    /// Create an upper triangular matrix
    /// Memory efficient: ~50% storage vs dense matrix
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::matrix::Matrix;
    /// use mathhook_core::Expression;
    ///
    /// // For 3x3 upper triangular, provide 6 elements: [a11, a12, a13, a22, a23, a33]
    /// let upper = Matrix::upper_triangular(3, vec![
    ///     Expression::integer(1), Expression::integer(2), Expression::integer(3),
    ///     Expression::integer(4), Expression::integer(5),
    ///     Expression::integer(6)
    /// ]);
    /// ```
    pub fn upper_triangular(size: usize, elements: Vec<Expression>) -> Self {
        Matrix::UpperTriangular(UpperTriangularMatrixData { size, elements })
    }

    /// Create a lower triangular matrix
    /// Memory efficient: ~50% storage vs dense matrix
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::matrix::Matrix;
    /// use mathhook_core::Expression;
    ///
    /// // For 3x3 lower triangular, provide 6 elements: [a11, a21, a22, a31, a32, a33]
    /// let lower = Matrix::lower_triangular(3, vec![
    ///     Expression::integer(1),
    ///     Expression::integer(2), Expression::integer(3),
    ///     Expression::integer(4), Expression::integer(5), Expression::integer(6)
    /// ]);
    /// ```
    pub fn lower_triangular(size: usize, elements: Vec<Expression>) -> Self {
        Matrix::LowerTriangular(LowerTriangularMatrixData { size, elements })
    }

    /// Create a symmetric matrix
    /// Memory efficient: ~50% storage vs dense matrix
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::matrix::Matrix;
    /// use mathhook_core::Expression;
    ///
    /// // For 3x3 symmetric, provide upper triangle: [a11, a12, a13, a22, a23, a33]
    /// let symmetric = Matrix::symmetric(3, vec![
    ///     Expression::integer(1), Expression::integer(2), Expression::integer(3),
    ///     Expression::integer(4), Expression::integer(5),
    ///     Expression::integer(6)
    /// ]);
    /// ```
    pub fn symmetric(size: usize, elements: Vec<Expression>) -> Self {
        Matrix::Symmetric(SymmetricMatrixData { size, elements })
    }

    /// Create a permutation matrix
    /// Memory efficient: O(n) storage vs O(n²) for dense matrix
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::matrix::Matrix;
    ///
    /// // Permutation [2, 0, 1] means: row 0 -> col 2, row 1 -> col 0, row 2 -> col 1
    /// let perm = Matrix::permutation(vec![2, 0, 1]);
    /// ```
    pub fn permutation(permutation: Vec<usize>) -> Self {
        Matrix::Permutation(PermutationMatrixData { permutation })
    }

    /// Create matrix from nested arrays (convenience method)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::matrix::Matrix;
    /// use mathhook_core::Expression;
    ///
    /// let matrix = Matrix::from_arrays([
    ///     [1, 2, 3],
    ///     [4, 5, 6]
    /// ]);
    /// ```
    pub fn from_arrays<const R: usize, const C: usize>(arrays: [[i64; C]; R]) -> Self {
        let rows: Vec<Vec<Expression>> = arrays
            .iter()
            .map(|row| row.iter().map(|&val| Expression::integer(val)).collect())
            .collect();
        Matrix::dense(rows)
    }

    /// Create matrix from flat vector (row-major order)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::matrix::Matrix;
    /// use mathhook_core::Expression;
    ///
    /// let matrix = Matrix::from_flat(2, 3, vec![
    ///     Expression::integer(1), Expression::integer(2), Expression::integer(3),
    ///     Expression::integer(4), Expression::integer(5), Expression::integer(6)
    /// ]);
    /// // Creates 2x3 matrix: [[1, 2, 3], [4, 5, 6]]
    /// ```
    pub fn from_flat(rows: usize, cols: usize, elements: Vec<Expression>) -> Self {
        if elements.len() != rows * cols {
            return Matrix::zero(rows, cols);
        }

        let matrix_rows: Vec<Vec<Expression>> =
            elements.chunks(cols).map(|chunk| chunk.to_vec()).collect();

        Matrix::dense(matrix_rows)
    }
}
