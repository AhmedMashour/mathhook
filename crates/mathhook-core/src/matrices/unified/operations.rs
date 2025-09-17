//! Matrix arithmetic and basic operations

use crate::core::Expression;
use crate::error::MathError;
use crate::matrices::types::*;
use crate::matrices::unified::Matrix;
use crate::simplify::Simplify;

impl Matrix {
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
                let (rows, _) = self.dimensions();
                let diagonal_elements: Vec<Expression> =
                    (0..rows).map(|i| self.get_element(i, i)).collect();
                Expression::add(diagonal_elements).simplify()
            }
        }
    }

    /// Get the determinant efficiently (for square matrices)
    ///
    /// # Returns
    /// Result containing the determinant expression, or MathError for non-square matrices
    ///
    /// # Errors
    /// Returns DomainError if matrix is not square
    ///
    /// # Algorithm
    /// - Special matrices (Identity, Zero, Scalar, Diagonal): O(1) or O(n)
    /// - Small matrices (1x1, 2x2): Direct formulas
    /// - Numeric matrices: NumericMatrix fast-path with O(n³) LU decomposition
    /// - Larger symbolic matrices (n≥3): LU decomposition O(n³)
    pub fn determinant(&self) -> Result<Expression, MathError> {
        let (rows, cols) = self.dimensions();

        if rows != cols {
            return Err(MathError::DomainError {
                operation: "determinant".to_string(),
                value: Expression::function("matrix", vec![]),
                reason: format!("Determinant requires square matrix, got {}x{}", rows, cols),
            });
        }

        match self {
            Matrix::Identity(_) => return Ok(Expression::integer(1)),
            Matrix::Zero(_) => return Ok(Expression::integer(0)),
            Matrix::Scalar(data) => {
                return Ok(Expression::pow(
                    data.scalar_value.clone(),
                    Expression::integer(data.size as i64),
                )
                .simplify())
            }
            Matrix::Diagonal(data) => {
                return Ok(Expression::mul(data.diagonal_elements.clone()).simplify())
            }
            Matrix::Permutation(_data) => return Ok(Expression::integer(1)),
            _ => {}
        }

        if let Some(numeric) = self.as_numeric() {
            let det = numeric.determinant()?;
            return Ok(Expression::float(det));
        }

        if rows == 0 {
            return Ok(Expression::integer(1));
        }

        if rows == 1 {
            return Ok(self.get_element(0, 0));
        }

        if rows == 2 {
            let a = self.get_element(0, 0);
            let b = self.get_element(0, 1);
            let c = self.get_element(1, 0);
            let d = self.get_element(1, 1);

            let ad = Expression::mul(vec![a, d]);
            let bc = Expression::mul(vec![b, c]);
            return Ok(Expression::add(vec![
                ad,
                Expression::mul(vec![Expression::integer(-1), bc]),
            ])
            .simplify());
        }

        Ok(self.determinant_lu())
    }

    /// Compute determinant using LU decomposition: det(A) = det(L) * det(U) * sign(P)
    ///
    /// O(n³) algorithm significantly faster than cofactor expansion O(n!)
    ///
    /// For symbolic matrices, falls back to cofactor expansion on small minors
    fn determinant_lu(&self) -> Expression {
        let (n, _) = self.dimensions();

        let mut a: Vec<Vec<Expression>> = (0..n)
            .map(|i| (0..n).map(|j| self.get_element(i, j)).collect())
            .collect();

        let mut sign = 1i64;

        for k in 0..n {
            let pivot = ((k + 1)..n).find(|&i| !a[i][k].is_zero_fast()).unwrap_or(k);

            if pivot != k {
                a.swap(k, pivot);
                sign = -sign;
            }

            if a[k][k].is_zero_fast() {
                return Expression::integer(0);
            }

            for i in (k + 1)..n {
                let factor = Expression::mul(vec![
                    a[i][k].clone(),
                    Expression::pow(a[k][k].clone(), Expression::integer(-1)),
                ]);

                let pivot_row: Vec<Expression> = a[k][k..n].to_vec();
                for (j_offset, pivot_val) in pivot_row.into_iter().enumerate() {
                    let j = k + j_offset;
                    let subtraction = Expression::mul(vec![factor.clone(), pivot_val]);
                    a[i][j] = a[i][j].clone() - subtraction;
                }
            }
        }

        let det_u: Vec<Expression> = (0..n).map(|i| a[i][i].clone()).collect();

        Expression::mul(vec![Expression::integer(sign), Expression::mul(det_u)])
    }

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
                let (rows, cols) = self.dimensions();
                let mut result_rows = Vec::with_capacity(rows);
                for i in 0..rows {
                    let mut row = Vec::with_capacity(cols);
                    for j in 0..cols {
                        let elem = self.get_element(i, j);
                        let product = Expression::mul(vec![scalar.clone(), elem]).simplify();
                        row.push(product);
                    }
                    result_rows.push(row);
                }
                Matrix::Dense(MatrixData { rows: result_rows }).optimize()
            }
        }
    }
}

/// Core matrix operations that work directly on Matrix types
pub trait CoreMatrixOps {
    fn add(&self, other: &Matrix) -> Result<Matrix, MathError>;
    fn multiply(&self, other: &Matrix) -> Result<Matrix, MathError>;
    fn transpose(&self) -> Matrix;
    fn inverse(&self) -> Matrix;
}

impl CoreMatrixOps for Matrix {
    fn add(&self, other: &Matrix) -> Result<Matrix, MathError> {
        let (rows1, cols1) = self.dimensions();
        let (rows2, cols2) = other.dimensions();

        if rows1 != rows2 || cols1 != cols2 {
            return Err(MathError::DomainError {
                operation: "matrix_addition".to_string(),
                value: Expression::function("incompatible_matrices", vec![]),
                reason: format!(
                    "Cannot add {}x{} matrix to {}x{} matrix",
                    rows1, cols1, rows2, cols2
                ),
            });
        }

        let result = match (self, other) {
            (Matrix::Zero(_), other) => other.clone(),
            (this, Matrix::Zero(_)) => this.clone(),

            (Matrix::Identity(id), Matrix::Dense(dense))
            | (Matrix::Dense(dense), Matrix::Identity(id)) => {
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

            _ => {
                let mut result_rows = Vec::with_capacity(rows1);
                for i in 0..rows1 {
                    let mut row = Vec::with_capacity(cols1);
                    for j in 0..cols1 {
                        let elem1 = self.get_element(i, j);
                        let elem2 = other.get_element(i, j);
                        let sum = Expression::add(vec![elem1, elem2]).simplify();
                        row.push(sum);
                    }
                    result_rows.push(row);
                }

                Matrix::Dense(MatrixData { rows: result_rows }).optimize()
            }
        };

        Ok(result)
    }

    fn multiply(&self, other: &Matrix) -> Result<Matrix, MathError> {
        let (rows1, cols1) = self.dimensions();
        let (rows2, cols2) = other.dimensions();

        if cols1 != rows2 {
            return Err(MathError::DomainError {
                operation: "matrix_multiplication".to_string(),
                value: Expression::function("incompatible_matrices", vec![]),
                reason: format!(
                    "Cannot multiply {}x{} matrix by {}x{} matrix (inner dimensions {} != {})",
                    rows1, cols1, rows2, cols2, cols1, rows2
                ),
            });
        }

        let result = match (self, other) {
            (Matrix::Zero(_), _) => Matrix::Zero(ZeroMatrixData {
                rows: rows1,
                cols: cols2,
            }),
            (_, Matrix::Zero(_)) => Matrix::Zero(ZeroMatrixData {
                rows: rows1,
                cols: cols2,
            }),

            (Matrix::Identity(_), other) => other.clone(),
            (this, Matrix::Identity(_)) => this.clone(),

            (Matrix::Diagonal(d1), Matrix::Diagonal(d2))
                if d1.diagonal_elements.len() == d2.diagonal_elements.len() =>
            {
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

            (Matrix::Scalar(s1), Matrix::Scalar(s2)) if s1.size == s2.size => {
                let product_scalar =
                    Expression::mul(vec![s1.scalar_value.clone(), s2.scalar_value.clone()]);
                Matrix::Scalar(ScalarMatrixData {
                    size: s1.size,
                    scalar_value: product_scalar,
                })
            }
            (Matrix::Scalar(s), other) => other.scalar_multiply(&s.scalar_value),
            (this, Matrix::Scalar(s)) => this.scalar_multiply(&s.scalar_value),

            _ => {
                if let (Some(num1), Some(num2)) = (self.as_numeric(), other.as_numeric()) {
                    let result = num1.multiply(&num2)?;
                    return Ok(result.to_matrix().optimize());
                }

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
        };

        Ok(result)
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
                elements: data.elements.clone(),
            }),
            Matrix::LowerTriangular(data) => Matrix::UpperTriangular(UpperTriangularMatrixData {
                size: data.size,
                elements: data.elements.clone(),
            }),
            _ => {
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
                if let Some(numeric) = self.as_numeric() {
                    if let Ok(inv) = numeric.inverse() {
                        return inv.to_matrix();
                    }
                }

                if let Some(inv) = self.inverse_via_lu() {
                    inv
                } else {
                    self.gauss_jordan_inverse()
                }
            }
        }
    }
}
