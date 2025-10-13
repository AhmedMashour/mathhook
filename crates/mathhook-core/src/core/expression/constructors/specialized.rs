//! Specialized expression constructors for complex, matrix, set, and advanced types

use crate::core::expression::{ComplexData, Expression, IntervalData, PiecewiseData};

impl Expression {
    /// Create a complex number expression
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::Expression;
    ///
    /// let expr = Expression::complex(
    ///     Expression::integer(3),
    ///     Expression::integer(4),
    /// );
    /// ```
    pub fn complex(real: Expression, imag: Expression) -> Self {
        Self::Complex(Box::new(ComplexData { real, imag }))
    }

    /// Create a set expression
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::Expression;
    ///
    /// let set = Expression::set(vec![
    ///     Expression::integer(1),
    ///     Expression::integer(2),
    ///     Expression::integer(3),
    /// ]);
    /// ```
    pub fn set(elements: Vec<Expression>) -> Self {
        Self::Set(Box::new(elements))
    }

    /// Create an interval expression
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::Expression;
    ///
    /// let interval = Expression::interval(
    ///     Expression::integer(0),
    ///     Expression::integer(10),
    ///     true,
    ///     false,
    /// );
    /// ```
    pub fn interval(
        start: Expression,
        end: Expression,
        start_inclusive: bool,
        end_inclusive: bool,
    ) -> Self {
        Self::Interval(Box::new(IntervalData {
            start,
            end,
            start_inclusive,
            end_inclusive,
        }))
    }

    /// Create a piecewise function expression
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{Expression, symbol};
    ///
    /// let piecewise = Expression::piecewise(
    ///     vec![(Expression::symbol(symbol!(x)), Expression::integer(1))],
    ///     Some(Expression::integer(0)),
    /// );
    /// ```
    pub fn piecewise(pieces: Vec<(Expression, Expression)>, default: Option<Expression>) -> Self {
        Self::Piecewise(Box::new(PiecewiseData { pieces, default }))
    }

    /// Create a matrix expression from rows
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
    /// ```
    pub fn matrix(rows: Vec<Vec<Expression>>) -> Self {
        use crate::matrix::Matrix;
        Self::Matrix(Box::new(Matrix::dense(rows)))
    }

    /// Create an identity matrix expression
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{Expression, MatrixOperations};
    ///
    /// let identity = Expression::identity_matrix(3);
    /// assert!(identity.is_identity_matrix());
    /// ```
    pub fn identity_matrix(size: usize) -> Self {
        use crate::matrix::Matrix;
        Self::Matrix(Box::new(Matrix::identity(size)))
    }

    /// Create a method call expression
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{Expression, symbol};
    ///
    /// let matrix = Expression::symbol(symbol!(A));
    /// let det_call = Expression::method_call(matrix, "det", vec![]);
    /// let trace_call = Expression::method_call(
    ///     Expression::symbol(symbol!(B)),
    ///     "trace",
    ///     vec![]
    /// );
    /// ```
    pub fn method_call(
        object: Expression,
        method_name: impl Into<String>,
        args: Vec<Expression>,
    ) -> Self {
        use crate::core::expression::MethodCallData;
        Self::MethodCall(Box::new(MethodCallData {
            object,
            method_name: method_name.into(),
            args,
        }))
    }

    /// Create a zero matrix expression
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{Expression, MatrixOperations};
    ///
    /// let zero = Expression::zero_matrix(2, 3);
    /// assert!(zero.is_zero_matrix());
    /// ```
    pub fn zero_matrix(rows: usize, cols: usize) -> Self {
        use crate::matrix::Matrix;
        Self::Matrix(Box::new(Matrix::zero(rows, cols)))
    }

    /// Create a diagonal matrix expression
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::Expression;
    ///
    /// let diag = Expression::diagonal_matrix(vec![
    ///     Expression::integer(1),
    ///     Expression::integer(2),
    ///     Expression::integer(3)
    /// ]);
    /// ```
    pub fn diagonal_matrix(diagonal_elements: Vec<Expression>) -> Self {
        use crate::matrix::Matrix;
        Self::Matrix(Box::new(Matrix::diagonal(diagonal_elements)))
    }

    /// Create a scalar matrix expression (c*I)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::Expression;
    ///
    /// let scalar = Expression::scalar_matrix(3, Expression::integer(5));
    /// // Creates 5*I (5 times the 3x3 identity matrix)
    /// ```
    pub fn scalar_matrix(size: usize, scalar_value: Expression) -> Self {
        use crate::matrix::Matrix;
        Self::Matrix(Box::new(Matrix::scalar(size, scalar_value)))
    }

    /// Create matrix from nested arrays (convenience method)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::Expression;
    ///
    /// let matrix = Expression::matrix_from_arrays([
    ///     [1, 2, 3],
    ///     [4, 5, 6]
    /// ]);
    /// ```
    pub fn matrix_from_arrays<const R: usize, const C: usize>(arrays: [[i64; C]; R]) -> Self {
        use crate::matrix::Matrix;
        Self::Matrix(Box::new(Matrix::from_arrays(arrays)))
    }
}
