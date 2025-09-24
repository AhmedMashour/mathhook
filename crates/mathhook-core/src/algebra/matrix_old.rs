//! Matrix operations and linear algebra
//!
//! Handles symbolic matrices with Expression-based elements.
//! Provides comprehensive matrix arithmetic including addition, multiplication,
//! determinant calculation, inverse, and eigenvalue computation.

use crate::core::Expression;
use crate::simplify::Simplify;

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

    /// Raise matrix to a power
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{Expression, MatrixOperations};
    ///
    /// let matrix = Expression::matrix(vec![
    ///     vec![Expression::integer(2), Expression::integer(0)],
    ///     vec![Expression::integer(0), Expression::integer(3)]
    /// ]);
    /// let squared = matrix.matrix_power(&Expression::integer(2));
    /// ```
    fn matrix_power(&self, exponent: &Expression) -> Expression;
}

impl Expression {
    /// Get matrix dimensions for any matrix type
    ///
    /// Returns (rows, columns) for both regular matrices and identity matrices.
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
    /// Returns true for both regular matrices and identity matrices.
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

    /// Convert any matrix to regular dense Matrix when needed
    ///
    /// This method provides a fallback for operations that need explicit matrix elements.
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
                use crate::core::expression::unified_matrix::Matrix;

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
            // Matrix + Matrix using unified system
            (Expression::Matrix(a), Expression::Matrix(b)) => {
                use crate::core::expression::unified_matrix::MatrixOps;
                let result_matrix = a.matrix_add(b);
                Expression::Matrix(Box::new(result_matrix))
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
            // Regular matrix * Regular matrix
            (Expression::Matrix(a), Expression::Matrix(b)) => {
                if a.rows.is_empty() || b.rows.is_empty() || a.rows[0].len() != b.rows.len() {
                    return Expression::function("undefined", vec![]);
                }

                let rows = a.rows.len();
                let cols = b.rows[0].len();
                let inner = a.rows[0].len();

                let mut result = Vec::with_capacity(rows);

                for i in 0..rows {
                    let mut row = Vec::with_capacity(cols);

                    for j in 0..cols {
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

            // Identity * Matrix = Matrix (optimization: I * A = A)
            (Expression::IdentityMatrix(id), Expression::Matrix(m)) => {
                if id.size != m.rows.len() {
                    return Expression::function("undefined", vec![]);
                }
                // I * A = A
                Expression::Matrix(m.clone())
            }

            // Matrix * Identity = Matrix (optimization: A * I = A)
            (Expression::Matrix(m), Expression::IdentityMatrix(id)) => {
                if m.rows.get(0).map(|row| row.len()).unwrap_or(0) != id.size {
                    return Expression::function("undefined", vec![]);
                }
                // A * I = A
                Expression::Matrix(m.clone())
            }

            // Identity * Identity = Identity (optimization: I * I = I)
            (Expression::IdentityMatrix(a), Expression::IdentityMatrix(b)) => {
                if a.size != b.size {
                    return Expression::function("undefined", vec![]);
                }
                // I * I = I
                Expression::IdentityMatrix(a.clone())
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

            // Scalar * Identity = Scalar * I (create diagonal matrix with scalar on diagonal)
            Expression::IdentityMatrix(id) => {
                let size = id.size;
                let rows: Vec<Vec<Expression>> = (0..size)
                    .map(|i| {
                        (0..size)
                            .map(|j| {
                                if i == j {
                                    scalar.clone()
                                } else {
                                    Expression::integer(0)
                                }
                            })
                            .collect()
                    })
                    .collect();
                Expression::matrix(rows)
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

            // Determinant of identity matrix is always 1
            Expression::IdentityMatrix(_) => Expression::integer(1),

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

            // Transpose of identity matrix is identity matrix
            Expression::IdentityMatrix(id) => Expression::IdentityMatrix(id.clone()),

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

            // Inverse of identity matrix is ide1ntity matrix (I^(-1) = I)
            Expression::IdentityMatrix(id) => Expression::IdentityMatrix(id.clone()),

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

            // Trace of identity matrix is the size (tr(I_n) = n)
            Expression::IdentityMatrix(id) => Expression::integer(id.size as i64),

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

            /// IdentityMatrix is always an identity matrix
            Expression::IdentityMatrix(_) => true,

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

    fn matrix_power(&self, exponent: &Expression) -> Expression {
        match (self, exponent) {
            // Identity matrix to any power is identity matrix (I^n = I)
            (Expression::IdentityMatrix(id), _) => {
                // Check if exponent is valid (non-negative for matrix powers)
                match exponent {
                    Expression::Number(crate::core::Number::Integer(n)) if *n >= 0 => {
                        Expression::IdentityMatrix(id.clone())
                    }
                    Expression::Number(crate::core::Number::Integer(-1)) => {
                        // I^(-1) = I (identity is its own inverse)
                        Expression::IdentityMatrix(id.clone())
                    }
                    _ => Expression::function("power", vec![self.clone(), exponent.clone()]),
                }
            }

            // Regular matrix powers
            (Expression::Matrix(_), Expression::Number(crate::core::Number::Integer(n))) => {
                match *n {
                    0 => {
                        // A^0 = I (identity matrix of same size)
                        if let Some((rows, cols)) = self.matrix_dimensions() {
                            if rows == cols {
                                Expression::identity_matrix(rows)
                            } else {
                                Expression::function(
                                    "error",
                                    vec![Expression::function("undefined", vec![])],
                                )
                            }
                        } else {
                            Expression::function(
                                "error",
                                vec![Expression::function("undefined", vec![])],
                            )
                        }
                    }
                    1 => self.clone(),
                    -1 => self.matrix_inverse(),
                    n if n > 1 => {
                        // Repeated multiplication: A^n = A * A * ... * A
                        let mut result = self.clone();
                        for _ in 1..n {
                            result = result.matrix_multiply(self);
                        }
                        result
                    }
                    n if n < -1 => {
                        // A^(-n) = (A^(-1))^n
                        let inverse = self.matrix_inverse();
                        inverse.matrix_power(&Expression::integer(-n))
                    }
                    _ => Expression::function("power", vec![self.clone(), exponent.clone()]),
                }
            }

            _ => Expression::function("power", vec![self.clone(), exponent.clone()]),
        }
    }
}

impl Expression {
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

    /// Convert IdentityMatrix to regular Matrix when needed
    /// Example:
    /// ```rust
    /// use mathhook_core::Expression;
    ///
    /// let identity = Expression::identity_matrix(3);
    /// let matrix = identity.as_matrix();
    /// ```
    pub fn as_matrix(&self) -> Expression {
        match self {
            Expression::IdentityMatrix(data) => {
                let size = data.size;
                let rows: Vec<Vec<Expression>> = (0..size)
                    .map(|i| {
                        (0..size)
                            .map(|j| {
                                if i == j {
                                    Expression::integer(1)
                                } else {
                                    Expression::integer(0)
                                }
                            })
                            .collect()
                    })
                    .collect();
                Expression::matrix(rows)
            }
            Expression::Matrix(_) => self.clone(),
            _ => panic!("Not a matrix expression"),
        }
    }

    /// Check if expression is any kind of matrix
    /// Example:
    /// ```rust
    /// use mathhook_core::Expression;
    ///
    /// let identity = Expression::identity_matrix(3);
    /// assert!(identity.is_matrix());
    /// ```
    pub fn is_matrix(&self) -> bool {
        matches!(self, Expression::Matrix(_) | Expression::IdentityMatrix(_))
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
    /// let simplified = matrix.simplify(); // or Expression::simplify_matrix(&matrix) (same result)
    /// ```
    pub fn simplify_matrix(expr: &Expression) -> Expression {
        match expr {
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
                        return Expression::identity_matrix(simplified_rows.len());
                    }
                }

                Expression::matrix(simplified_rows)
            }
            _ => expr.clone(),
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

        let simplified = Expression::simplify_matrix(&matrix);

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
