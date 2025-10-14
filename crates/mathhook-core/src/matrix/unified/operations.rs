//! Matrix arithmetic and basic operations

use crate::core::Expression;
use crate::matrix::types::*;
use crate::matrix::unified::Matrix;
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
    pub fn determinant(&self) -> Expression {
        match self {
            Matrix::Identity(_) => Expression::integer(1),
            Matrix::Zero(_) => Expression::integer(0),
            Matrix::Scalar(data) => Expression::pow(
                data.scalar_value.clone(),
                Expression::integer(data.size as i64),
            )
            .simplify(),
            Matrix::Diagonal(data) => Expression::mul(data.diagonal_elements.clone()).simplify(),
            Matrix::Permutation(_data) => Expression::integer(1),
            _ => {
                let (rows, cols) = self.dimensions();
                if rows != cols {
                    return Expression::function("undefined", vec![]);
                }

                if rows == 0 {
                    return Expression::integer(1);
                }

                if rows == 1 {
                    return self.get_element(0, 0);
                }

                if rows == 2 {
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

                let mut terms = Vec::new();
                for j in 0..cols {
                    let element = self.get_element(0, j);
                    if element.is_zero() {
                        continue;
                    }

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
    fn add(&self, other: &Matrix) -> Matrix;
    fn multiply(&self, other: &Matrix) -> Matrix;
    fn transpose(&self) -> Matrix;
    fn inverse(&self) -> Matrix;
}

impl CoreMatrixOps for Matrix {
    fn add(&self, other: &Matrix) -> Matrix {
        match (self, other) {
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
                let (rows1, cols1) = self.dimensions();
                let (rows2, cols2) = other.dimensions();

                if rows1 != rows2 || cols1 != cols2 {
                    return Matrix::Zero(ZeroMatrixData { rows: 0, cols: 0 });
                }

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
        }
    }

    fn multiply(&self, other: &Matrix) -> Matrix {
        match (self, other) {
            (Matrix::Zero(_), _) => Matrix::Zero(ZeroMatrixData {
                rows: self.dimensions().0,
                cols: other.dimensions().1,
            }),
            (_, Matrix::Zero(_)) => Matrix::Zero(ZeroMatrixData {
                rows: self.dimensions().0,
                cols: other.dimensions().1,
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
                let (rows1, cols1) = self.dimensions();
                let (rows2, cols2) = other.dimensions();

                if cols1 != rows2 {
                    return Matrix::Zero(ZeroMatrixData {
                        rows: rows1,
                        cols: cols2,
                    });
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
            _ => self.gauss_jordan_inverse(),
        }
    }
}
