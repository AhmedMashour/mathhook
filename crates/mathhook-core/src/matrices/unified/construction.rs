//! Matrix construction and property methods

use crate::core::matrix::NumericMatrix;
use crate::core::Expression;
use crate::matrices::types::*;
use crate::matrices::unified::Matrix;

impl Matrix {
    /// Get matrix dimensions efficiently
    ///
    /// This method provides O(1) dimension lookup for all matrix types.
    #[inline]
    pub fn dimensions(&self) -> (usize, usize) {
        match self {
            Matrix::Dense(data) => {
                let rows = data.rows.len();
                let cols = data.rows.first().map(|row| row.len()).unwrap_or(0);
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

    /// Try to convert this matrix to a NumericMatrix for fast numeric operations.
    ///
    /// Returns Some(NumericMatrix) if all elements can be converted to f64,
    /// None otherwise (e.g., if matrix contains symbolic expressions).
    pub fn as_numeric(&self) -> Option<NumericMatrix> {
        NumericMatrix::try_from_matrix(self)
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
            Matrix::Scalar(data) => data.scalar_value == Expression::integer(1),
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
                    data.rows.first().map(|r| r.len()).unwrap_or(0),
                );

                if data
                    .rows
                    .iter()
                    .all(|row| row.iter().all(|elem| elem.is_zero_fast()))
                {
                    return Matrix::Zero(ZeroMatrixData { rows, cols });
                }

                if rows == cols
                    && data.rows.iter().enumerate().all(|(i, row)| {
                        row.iter().enumerate().all(|(j, elem)| {
                            if i == j {
                                elem == &Expression::integer(1)
                            } else {
                                elem.is_zero_fast()
                            }
                        })
                    })
                {
                    return Matrix::Identity(IdentityMatrixData { size: rows });
                }

                if rows == cols
                    && data.rows.iter().enumerate().all(|(i, row)| {
                        row.iter()
                            .enumerate()
                            .all(|(j, elem)| i == j || elem.is_zero_fast())
                    })
                {
                    let diagonal_elements: Vec<Expression> =
                        (0..rows).map(|i| data.rows[i][i].clone()).collect();

                    if diagonal_elements
                        .iter()
                        .all(|elem| elem == &Expression::integer(1))
                    {
                        return Matrix::Identity(IdentityMatrixData { size: rows });
                    }

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

                Matrix::Dense(data)
            }

            Matrix::Diagonal(data) => {
                if data
                    .diagonal_elements
                    .iter()
                    .all(|elem| elem == &Expression::integer(1))
                {
                    return Matrix::Identity(IdentityMatrixData {
                        size: data.diagonal_elements.len(),
                    });
                }

                if data
                    .diagonal_elements
                    .iter()
                    .all(|elem| elem.is_zero_fast())
                {
                    let size = data.diagonal_elements.len();
                    return Matrix::Zero(ZeroMatrixData {
                        rows: size,
                        cols: size,
                    });
                }

                if !data.diagonal_elements.is_empty()
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

                Matrix::Diagonal(data)
            }

            other => other,
        }
    }

    /// Create a dense matrix from rows
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::matrices::Matrix;
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
    /// use mathhook_core::matrices::Matrix;
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
    /// use mathhook_core::matrices::Matrix;
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
    /// use mathhook_core::matrices::Matrix;
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
    /// use mathhook_core::matrices::Matrix;
    /// use mathhook_core::Expression;
    ///
    /// let scalar = Matrix::scalar(3, Expression::integer(5));
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
    /// use mathhook_core::matrices::Matrix;
    /// use mathhook_core::Expression;
    ///
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
    /// use mathhook_core::matrices::Matrix;
    /// use mathhook_core::Expression;
    ///
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
    /// use mathhook_core::matrices::Matrix;
    /// use mathhook_core::Expression;
    ///
    /// let symmetric = Matrix::symmetric(3, vec![
    ///     Expression::integer(1),
    ///     Expression::integer(2), Expression::integer(3),
    ///     Expression::integer(4), Expression::integer(5), Expression::integer(6)
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
    /// use mathhook_core::matrices::Matrix;
    ///
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
    /// use mathhook_core::matrices::Matrix;
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
    /// use mathhook_core::matrices::Matrix;
    /// use mathhook_core::Expression;
    ///
    /// let matrix = Matrix::from_flat(2, 3, &[
    ///     Expression::integer(1), Expression::integer(2), Expression::integer(3),
    ///     Expression::integer(4), Expression::integer(5), Expression::integer(6)
    /// ]);
    /// ```
    pub fn from_flat(rows: usize, cols: usize, elements: &[Expression]) -> Self {
        if elements.len() != rows * cols {
            return Matrix::zero(rows, cols);
        }

        let matrix_rows: Vec<Vec<Expression>> =
            elements.chunks(cols).map(|chunk| chunk.to_vec()).collect();

        Matrix::dense(matrix_rows)
    }
}
