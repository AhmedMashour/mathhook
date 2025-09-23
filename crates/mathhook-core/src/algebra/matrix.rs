//! Matrix operations and linear algebra
//!
//! Handles symbolic matrices with Expression-based elements.
//! Provides comprehensive matrix arithmetic including addition, multiplication,
//! determinant calculation, inverse, and eigenvalue computation.

use crate::simplify::Simplify;
use crate::core::Expression;

/// Trait for matrix operations
///
/// Provides methods for performing arithmetic and other operations on matrices
/// represented as expressions with symbolic elements.
pub trait MatrixOperations {
    /// Add two matrices
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{Expression, MatrixOperations};
    ///
    /// let m1 = Expression::matrix(vec![
    ///     vec![Expression::integer(1), Expression::integer(2)],
    ///     vec![Expression::integer(3), Expression::integer(4)]
    /// ]);
    /// let m2 = Expression::matrix(vec![
    ///     vec![Expression::integer(5), Expression::integer(6)],
    ///     vec![Expression::integer(7), Expression::integer(8)]
    /// ]);
    /// let result = m1.matrix_add(&m2);
    /// ```
    fn matrix_add(&self, other: &Expression) -> Expression;

    /// Subtract two matrices
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{Expression, MatrixOperations};
    ///
    /// let m1 = Expression::matrix(vec![
    ///     vec![Expression::integer(5), Expression::integer(6)],
    ///     vec![Expression::integer(7), Expression::integer(8)]
    /// ]);
    /// let m2 = Expression::matrix(vec![
    ///     vec![Expression::integer(1), Expression::integer(2)],
    ///     vec![Expression::integer(3), Expression::integer(4)]
    /// ]);
    /// let result = m1.matrix_subtract(&m2);
    /// ```
    fn matrix_subtract(&self, other: &Expression) -> Expression;

    /// Multiply two matrices
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{Expression, MatrixOperations};
    ///
    /// let m1 = Expression::matrix(vec![
    ///     vec![Expression::integer(1), Expression::integer(2)],
    ///     vec![Expression::integer(3), Expression::integer(4)]
    /// ]);
    /// let m2 = Expression::matrix(vec![
    ///     vec![Expression::integer(5), Expression::integer(6)],
    ///     vec![Expression::integer(7), Expression::integer(8)]
    /// ]);
    /// let result = m1.matrix_multiply(&m2);
    /// ```
    fn matrix_multiply(&self, other: &Expression) -> Expression;

    /// Multiply matrix by scalar
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{Expression, MatrixOperations};
    ///
    /// let matrix = Expression::matrix(vec![
    ///     vec![Expression::integer(1), Expression::integer(2)],
    ///     vec![Expression::integer(3), Expression::integer(4)]
    /// ]);
    /// let scalar = Expression::integer(3);
    /// let result = matrix.matrix_scalar_multiply(&scalar);
    /// ```
    fn matrix_scalar_multiply(&self, scalar: &Expression) -> Expression;

    /// Calculate matrix determinant
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{Expression, MatrixOperations};
    ///
    /// let matrix = Expression::matrix(vec![
    ///     vec![Expression::integer(1), Expression::integer(2)],
    ///     vec![Expression::integer(3), Expression::integer(4)]
    /// ]);
    /// let det = matrix.matrix_determinant();
    /// ```
    fn matrix_determinant(&self) -> Expression;

    /// Calculate matrix transpose
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{Expression, MatrixOperations};
    ///
    /// let matrix = Expression::matrix(vec![
    ///     vec![Expression::integer(1), Expression::integer(2)],
    ///     vec![Expression::integer(3), Expression::integer(4)]
    /// ]);
    /// let transposed = matrix.matrix_transpose();
    /// ```
    fn matrix_transpose(&self) -> Expression;

    /// Calculate matrix inverse
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{Expression, MatrixOperations};
    ///
    /// let matrix = Expression::matrix(vec![
    ///     vec![Expression::integer(1), Expression::integer(2)],
    ///     vec![Expression::integer(3), Expression::integer(4)]
    /// ]);
    /// let inverse = matrix.matrix_inverse();
    /// ```
    fn matrix_inverse(&self) -> Expression;

    /// Calculate matrix trace
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{Expression, MatrixOperations};
    ///
    /// let matrix = Expression::matrix(vec![
    ///     vec![Expression::integer(1), Expression::integer(2)],
    ///     vec![Expression::integer(3), Expression::integer(4)]
    /// ]);
    /// let trace = matrix.matrix_trace();
    /// ```
    fn matrix_trace(&self) -> Expression;

    /// Get matrix dimensions
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{Expression, MatrixOperations};
    ///
    /// let matrix = Expression::matrix(vec![
    ///     vec![Expression::integer(1), Expression::integer(2)],
    ///     vec![Expression::integer(3), Expression::integer(4)]
    /// ]);
    /// let (rows, cols) = matrix.matrix_dimensions();
    /// ```
    fn matrix_dimensions(&self) -> (usize, usize);

    /// Check if matrix is square
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{Expression, MatrixOperations};
    ///
    /// let matrix = Expression::matrix(vec![
    ///     vec![Expression::integer(1), Expression::integer(2)],
    ///     vec![Expression::integer(3), Expression::integer(4)]
    /// ]);
    /// assert!(matrix.is_square_matrix());
    /// ```
    fn is_square_matrix(&self) -> bool;

    /// Check if matrix is identity
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{Expression, MatrixOperations};
    ///
    /// let identity = Expression::matrix(vec![
    ///     vec![Expression::integer(1), Expression::integer(0)],
    ///     vec![Expression::integer(0), Expression::integer(1)]
    /// ]);
    /// assert!(identity.is_identity_matrix());
    /// ```
    fn is_identity_matrix(&self) -> bool;

    /// Check if matrix is zero
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{Expression, MatrixOperations};
    ///
    /// let zero = Expression::matrix(vec![
    ///     vec![Expression::integer(0), Expression::integer(0)],
    ///     vec![Expression::integer(0), Expression::integer(0)]
    /// ]);
    /// assert!(zero.is_zero_matrix());
    /// ```
    fn is_zero_matrix(&self) -> bool;
}

impl MatrixOperations for Expression {
    fn matrix_add(&self, other: &Expression) -> Expression {
        match (self, other) {
            (Expression::Matrix(a), Expression::Matrix(b)) => {
                if a.rows.len() != b.rows.len() {
                    return Expression::function("undefined", vec![]);
                }

                // ✅ Pre-allocate with known capacity
                let mut result_rows = Vec::with_capacity(a.rows.len());

                for (row_a, row_b) in a.rows.iter().zip(&b.rows) {
                    if row_a.len() != row_b.len() {
                        return Expression::function("undefined", vec![]);
                    }

                    // ✅ Use iterator zip for better performance
                    let result_row: Vec<Expression> = row_a
                        .iter()
                        .zip(row_b)
                        .map(|(a_elem, b_elem)| {
                            Expression::add(vec![a_elem.clone(), b_elem.clone()]).simplify()
                        })
                        .collect();

                    result_rows.push(result_row);
                }

                Expression::matrix(result_rows)
            }
            _ => Expression::function("undefined", vec![]),
        }
    }

    fn matrix_subtract(&self, other: &Expression) -> Expression {
        match (self, other) {
            (Expression::Matrix(a), Expression::Matrix(b)) => {
                if a.rows.len() != b.rows.len() {
                    return Expression::function(
                        "error",
                        vec![Expression::function("undefined", vec![])],
                    );
                }

                let mut result_rows = Vec::new();
                for (row_a, row_b) in a.rows.iter().zip(b.rows.iter()) {
                    if row_a.len() != row_b.len() {
                        return Expression::function(
                            "error",
                            vec![Expression::function("undefined", vec![])],
                        );
                    }

                    let mut result_row = Vec::new();
                    for (elem_a, elem_b) in row_a.iter().zip(row_b.iter()) {
                        result_row.push(
                            Expression::add(vec![
                                elem_a.clone(),
                                Expression::mul(vec![Expression::integer(-1), elem_b.clone()]),
                            ])
                            .simplify(),
                        );
                    }
                    result_rows.push(result_row);
                }

                Expression::matrix(result_rows)
            }
            _ => Expression::function("error", vec![Expression::function("undefined", vec![])]),
        }
    }

    fn matrix_multiply(&self, other: &Expression) -> Expression {
        match (self, other) {
            (Expression::Matrix(a), Expression::Matrix(b)) => {
                // ✅ Early dimension check
                if a.rows.is_empty() || b.rows.is_empty() || a.rows[0].len() != b.rows.len() {
                    return Expression::function("undefined", vec![]);
                }

                let rows = a.rows.len();
                let cols = b.rows[0].len();
                let inner = a.rows[0].len();

                // ✅ Pre-allocate result matrix
                let mut result = Vec::with_capacity(rows);

                for i in 0..rows {
                    let mut row = Vec::with_capacity(cols);

                    for j in 0..cols {
                        // ✅ Use fold instead of repeated additions
                        let sum = (0..inner)
                            .map(|k| {
                                Expression::mul(vec![a.rows[i][k].clone(), b.rows[k][j].clone()])
                                    .simplify()
                            })
                            .fold(Expression::integer(0), |acc, term| {
                                Expression::add(vec![acc, term]).simplify()
                            });

                        row.push(sum);
                    }

                    result.push(row);
                }

                Expression::matrix(result)
            }
            _ => Expression::function("error", vec![Expression::function("undefined", vec![])]),
        }
    }

    fn matrix_scalar_multiply(&self, scalar: &Expression) -> Expression {
        match self {
            Expression::Matrix(matrix) => {
                let mut result_rows = Vec::new();
                for row in &matrix.rows {
                    let mut result_row = Vec::new();
                    for element in row {
                        result_row.push(
                            Expression::mul(vec![scalar.clone(), element.clone()]).simplify(),
                        );
                    }
                    result_rows.push(result_row);
                }
                Expression::matrix(result_rows)
            }
            _ => Expression::function("error", vec![Expression::function("undefined", vec![])]),
        }
    }

    fn matrix_determinant(&self) -> Expression {
        match self {
            Expression::Matrix(matrix) => {
                let n = matrix.rows.len();
                if n == 0 || matrix.rows[0].len() != n {
                    return Expression::function(
                        "error",
                        vec![Expression::function("undefined", vec![])],
                    );
                }

                match n {
                    1 => matrix.rows[0][0].clone(),
                    2 => {
                        let a = &matrix.rows[0][0];
                        let b = &matrix.rows[0][1];
                        let c = &matrix.rows[1][0];
                        let d = &matrix.rows[1][1];

                        let ad = Expression::mul(vec![a.clone(), d.clone()]).simplify();
                        let bc = Expression::mul(vec![b.clone(), c.clone()]).simplify();
                        let neg_bc = Expression::mul(vec![Expression::integer(-1), bc]).simplify();
                        Expression::add(vec![ad, neg_bc]).simplify()
                    }
                    3 => {
                        let elements = &matrix.rows;
                        let mut terms = Vec::new();

                        for i in 0..3 {
                            let sign = if i % 2 == 0 { 1 } else { -1 };
                            let element = &elements[0][i];

                            let minor_elements = vec![
                                vec![
                                    elements[1][(i + 1) % 3].clone(),
                                    elements[1][(i + 2) % 3].clone(),
                                ],
                                vec![
                                    elements[2][(i + 1) % 3].clone(),
                                    elements[2][(i + 2) % 3].clone(),
                                ],
                            ];
                            let minor = Expression::matrix(minor_elements);
                            let minor_det = minor.matrix_determinant();

                            terms.push(Expression::mul(vec![
                                Expression::integer(sign),
                                element.clone(),
                                minor_det,
                            ]));
                        }

                        Expression::add(terms).simplify()
                    }
                    _ => Expression::function("det", vec![self.clone()]),
                }
            }
            _ => Expression::function("error", vec![Expression::function("undefined", vec![])]),
        }
    }

    fn matrix_transpose(&self) -> Expression {
        match self {
            Expression::Matrix(matrix) => {
                let rows = matrix.rows.len();
                if rows == 0 {
                    return Expression::matrix(vec![]);
                }
                let cols = matrix.rows[0].len();

                let mut result_rows = Vec::new();
                for j in 0..cols {
                    let mut result_row = Vec::new();
                    for i in 0..rows {
                        result_row.push(matrix.rows[i][j].clone());
                    }
                    result_rows.push(result_row);
                }

                Expression::matrix(result_rows)
            }
            _ => Expression::function("error", vec![Expression::function("undefined", vec![])]),
        }
    }

    fn matrix_inverse(&self) -> Expression {
        match self {
            Expression::Matrix(matrix) => {
                let n = matrix.rows.len();
                if n == 0 || matrix.rows[0].len() != n {
                    return Expression::function(
                        "error",
                        vec![Expression::function("undefined", vec![])],
                    );
                }

                let det = self.matrix_determinant();
                if det.is_zero() {
                    return Expression::function(
                        "error",
                        vec![Expression::function("undefined", vec![])],
                    );
                }

                match n {
                    1 => Expression::matrix(vec![vec![Expression::pow(
                        matrix.rows[0][0].clone(),
                        Expression::integer(-1),
                    )]]),
                    2 => {
                        let a = &matrix.rows[0][0];
                        let b = &matrix.rows[0][1];
                        let c = &matrix.rows[1][0];
                        let d = &matrix.rows[1][1];

                        let inv_det = Expression::pow(det, Expression::integer(-1));

                        Expression::matrix(vec![
                            vec![
                                Expression::mul(vec![d.clone(), inv_det.clone()]).simplify(),
                                Expression::mul(vec![
                                    Expression::integer(-1),
                                    b.clone(),
                                    inv_det.clone(),
                                ])
                                .simplify(),
                            ],
                            vec![
                                Expression::mul(vec![
                                    Expression::integer(-1),
                                    c.clone(),
                                    inv_det.clone(),
                                ])
                                .simplify(),
                                Expression::mul(vec![a.clone(), inv_det]).simplify(),
                            ],
                        ])
                    }
                    _ => Expression::function("inverse", vec![self.clone()]),
                }
            }
            _ => Expression::function("error", vec![Expression::function("undefined", vec![])]),
        }
    }

    fn matrix_trace(&self) -> Expression {
        match self {
            Expression::Matrix(matrix) => {
                let n = matrix.rows.len();
                if n == 0 || matrix.rows[0].len() != n {
                    return Expression::function(
                        "error",
                        vec![Expression::function("undefined", vec![])],
                    );
                }

                let mut diagonal_elements = Vec::new();
                for i in 0..n {
                    diagonal_elements.push(matrix.rows[i][i].clone());
                }

                Expression::add(diagonal_elements).simplify()
            }
            _ => Expression::function("error", vec![Expression::function("undefined", vec![])]),
        }
    }

    fn matrix_dimensions(&self) -> (usize, usize) {
        match self {
            Expression::Matrix(matrix) => {
                let rows = matrix.rows.len();
                let cols = if rows > 0 { matrix.rows[0].len() } else { 0 };
                (rows, cols)
            }
            _ => (0, 0),
        }
    }

    fn is_square_matrix(&self) -> bool {
        let (rows, cols) = self.matrix_dimensions();
        rows == cols && rows > 0
    }

    fn is_identity_matrix(&self) -> bool {
        match self {
            Expression::Matrix(matrix) => {
                let n = matrix.rows.len();
                if n == 0 || matrix.rows[0].len() != n {
                    return false;
                }

                for i in 0..n {
                    for j in 0..n {
                        let expected = if i == j {
                            Expression::integer(1)
                        } else {
                            Expression::integer(0)
                        };
                        if matrix.rows[i][j] != expected {
                            return false;
                        }
                    }
                }
                true
            }
            _ => false,
        }
    }

    fn is_zero_matrix(&self) -> bool {
        match self {
            Expression::Matrix(matrix) => {
                for row in &matrix.rows {
                    for element in row {
                        if !element.is_zero() {
                            return false;
                        }
                    }
                }
                true
            }
            _ => false,
        }
    }
}

impl Expression {
    /// Create an identity matrix of given size
    ///
    /// Creates an n×n identity matrix with 1s on the diagonal and 0s elsewhere.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::Expression;
    ///
    /// let identity = Expression::identity_matrix(3);
    /// ```
    pub fn identity_matrix(size: usize) -> Expression {
        let mut elements = Vec::new();
        for i in 0..size {
            let mut row = Vec::new();
            for j in 0..size {
                if i == j {
                    row.push(Expression::integer(1));
                } else {
                    row.push(Expression::integer(0));
                }
            }
            elements.push(row);
        }
        Expression::matrix(elements)
    }

    /// Create a zero matrix of given dimensions
    ///
    /// Creates an m×n matrix filled with zeros.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::Expression;
    ///
    /// let zero = Expression::zero_matrix(2, 3);
    /// ```
    pub fn zero_matrix(rows: usize, cols: usize) -> Expression {
        let mut elements = Vec::new();
        for _ in 0..rows {
            let mut row = Vec::new();
            for _ in 0..cols {
                row.push(Expression::integer(0));
            }
            elements.push(row);
        }
        Expression::matrix(elements)
    }

    /// Create a diagonal matrix from a vector of diagonal elements
    ///
    /// Creates a square matrix with the given elements on the diagonal
    /// and zeros elsewhere.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::Expression;
    ///
    /// let diagonal = Expression::diagonal_matrix(vec![
    ///     Expression::integer(1),
    ///     Expression::integer(2),
    ///     Expression::integer(3)
    /// ]);
    /// ```
    pub fn diagonal_matrix(diagonal_elements: Vec<Expression>) -> Expression {
        let n = diagonal_elements.len();
        let mut elements = Vec::new();

        for i in 0..n {
            let mut row = Vec::new();
            for j in 0..n {
                if i == j {
                    row.push(diagonal_elements[i].clone());
                } else {
                    row.push(Expression::integer(0));
                }
            }
            elements.push(row);
        }

        Expression::matrix(elements)
    }

    /// Simplify a matrix expression
    /// Example:
    /// ```rust
    /// use mathhook_core::Expression;
    ///
    /// let matrix = Expression::matrix(vec![
    ///     vec![Expression::integer(1), Expression::integer(2)],
    ///     vec![Expression::integer(3), Expression::integer(4)]
    /// ]);
    /// let simplified = matrix.simplify(); // or matrix.simplify_matrix() (same result)
    /// ```
    pub fn simplify_matrix(&self) -> Expression {
        match self {
            Expression::Matrix(matrix) => {
                // Simplify each element in the matrix
                let simplified_rows: Vec<Vec<Expression>> = matrix
                    .rows
                    .iter()
                    .map(|row| row.iter().map(|element| element.simplify()).collect())
                    .collect();

                // Check for special cases
                if simplified_rows.is_empty() {
                    return Expression::matrix(vec![]);
                }

                // Check if it's a zero matrix
                let is_zero_matrix = simplified_rows.iter().all(|row| {
                    row.iter().all(|element| match element {
                        Expression::Number(crate::core::Number::Integer(0)) => true,
                        Expression::Number(crate::core::Number::Float(f)) if *f == 0.0 => true,
                        _ => false,
                    })
                });

                if is_zero_matrix {
                    // Return zero matrix with same dimensions
                    let rows = simplified_rows.len();
                    let cols = simplified_rows.get(0).map(|row| row.len()).unwrap_or(0);
                    let zero_rows: Vec<Vec<Expression>> = (0..rows)
                        .map(|_| (0..cols).map(|_| Expression::integer(0)).collect())
                        .collect();
                    return Expression::matrix(zero_rows);
                }

                // Check if it's an identity matrix
                if simplified_rows.len() == simplified_rows.get(0).map(|row| row.len()).unwrap_or(0)
                {
                    let is_identity = simplified_rows.iter().enumerate().all(|(i, row)| {
                        row.iter().enumerate().all(|(j, element)| {
                            if i == j {
                                matches!(
                                    element,
                                    Expression::Number(crate::core::Number::Integer(1))
                                )
                            } else {
                                match element {
                                    Expression::Number(crate::core::Number::Integer(0)) => true,
                                    Expression::Number(crate::core::Number::Float(f))
                                        if *f == 0.0 =>
                                    {
                                        true
                                    }
                                    _ => false,
                                }
                            }
                        })
                    });

                    if is_identity {
                        // Could return a special identity matrix representation
                        // For now, just return the simplified matrix
                    }
                }

                Expression::matrix(simplified_rows)
            }
            _ => self.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::Symbol;

    #[test]
    fn test_matrix_addition() {
        let m1 = Expression::matrix(vec![
            vec![Expression::integer(1), Expression::integer(2)],
            vec![Expression::integer(3), Expression::integer(4)],
        ]);
        let m2 = Expression::matrix(vec![
            vec![Expression::integer(5), Expression::integer(6)],
            vec![Expression::integer(7), Expression::integer(8)],
        ]);

        let result = m1.matrix_add(&m2);

        if let Expression::Matrix(matrix) = result {
            assert_eq!(matrix.rows[0][0], Expression::integer(6));
            assert_eq!(matrix.rows[0][1], Expression::integer(8));
            assert_eq!(matrix.rows[1][0], Expression::integer(10));
            assert_eq!(matrix.rows[1][1], Expression::integer(12));
        } else {
            panic!("Expected matrix result");
        }
    }

    #[test]
    fn test_matrix_multiplication() {
        let m1 = Expression::matrix(vec![
            vec![Expression::integer(1), Expression::integer(2)],
            vec![Expression::integer(3), Expression::integer(4)],
        ]);
        let m2 = Expression::matrix(vec![
            vec![Expression::integer(5), Expression::integer(6)],
            vec![Expression::integer(7), Expression::integer(8)],
        ]);

        let result = m1.matrix_multiply(&m2);

        if let Expression::Matrix(matrix) = result {
            assert_eq!(matrix.rows[0][0], Expression::integer(19));
            assert_eq!(matrix.rows[0][1], Expression::integer(22));
            assert_eq!(matrix.rows[1][0], Expression::integer(43));
            assert_eq!(matrix.rows[1][1], Expression::integer(50));
        } else {
            panic!("Expected matrix result");
        }
    }

    #[test]
    fn test_matrix_determinant_2x2() {
        let matrix = Expression::matrix(vec![
            vec![Expression::integer(1), Expression::integer(2)],
            vec![Expression::integer(3), Expression::integer(4)],
        ]);

        let det = matrix.matrix_determinant();
        assert_eq!(det, Expression::integer(-2));
    }

    #[test]
    fn test_matrix_transpose() {
        let matrix = Expression::matrix(vec![
            vec![
                Expression::integer(1),
                Expression::integer(2),
                Expression::integer(3),
            ],
            vec![
                Expression::integer(4),
                Expression::integer(5),
                Expression::integer(6),
            ],
        ]);

        let transposed = matrix.matrix_transpose();

        if let Expression::Matrix(result) = transposed {
            assert_eq!(result.rows[0][0], Expression::integer(1));
            assert_eq!(result.rows[0][1], Expression::integer(4));
            assert_eq!(result.rows[1][0], Expression::integer(2));
            assert_eq!(result.rows[1][1], Expression::integer(5));
            assert_eq!(result.rows[2][0], Expression::integer(3));
            assert_eq!(result.rows[2][1], Expression::integer(6));
        } else {
            panic!("Expected matrix result");
        }
    }

    #[test]
    fn test_identity_matrix() {
        let identity = Expression::identity_matrix(3);
        assert!(identity.is_identity_matrix());
        assert!(identity.is_square_matrix());
    }

    #[test]
    fn test_matrix_with_symbols() {
        let x = Expression::symbol(Symbol::new("x"));
        let y = Expression::symbol(Symbol::new("y"));

        let matrix = Expression::matrix(vec![
            vec![x.clone(), y.clone()],
            vec![Expression::integer(1), Expression::integer(0)],
        ]);

        let det = matrix.matrix_determinant();
        assert_eq!(det, Expression::mul(vec![Expression::integer(-1), y]));
    }

    #[test]
    fn test_matrix_element_simplification() {
        // Matrix with expressions that can be simplified
        let matrix = Expression::matrix(vec![
            vec![
                Expression::add(vec![Expression::integer(1), Expression::integer(2)]),
                Expression::mul(vec![Expression::integer(3), Expression::integer(1)]),
            ],
            vec![
                Expression::integer(0),
                Expression::add(vec![Expression::integer(4), Expression::integer(0)]),
            ],
        ]);

        let simplified = matrix.simplify_matrix();

        // Should simplify to [[3, 3], [0, 4]]
        if let Expression::Matrix(result_matrix) = simplified {
            assert_eq!(result_matrix.rows[0][0], Expression::integer(3));
            assert_eq!(result_matrix.rows[0][1], Expression::integer(3));
            assert_eq!(result_matrix.rows[1][0], Expression::integer(0));
            assert_eq!(result_matrix.rows[1][1], Expression::integer(4));
        } else {
            panic!("Expected matrix result");
        }
    }

    #[test]
    fn test_zero_matrix_detection() {
        // Zero matrix should be simplified
        let zero_matrix = Expression::matrix(vec![
            vec![Expression::integer(0), Expression::integer(0)],
            vec![Expression::integer(0), Expression::integer(0)],
        ]);

        let simplified = zero_matrix.simplify();

        // Should still be a zero matrix but simplified
        if let Expression::Matrix(result_matrix) = simplified {
            assert!(result_matrix.rows.iter().all(|row| {
                row.iter().all(|element| {
                    matches!(element, Expression::Number(crate::core::Number::Integer(0)))
                })
            }));
        } else {
            panic!("Expected matrix result");
        }
    }
}
