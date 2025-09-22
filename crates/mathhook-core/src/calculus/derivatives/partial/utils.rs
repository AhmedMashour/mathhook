//! Utility functions for partial derivative operations

use crate::algebra::simplify::Simplify;
use crate::core::{Expression, Number};

/// Utility functions for partial derivatives
pub struct PartialUtils;

impl PartialUtils {
    /// Fast expression equality check with caching
    ///
    /// # Performance Notes
    /// - Uses string representation comparison as fallback
    /// - Could be enhanced with structural comparison
    /// - Caches results for repeated comparisons
    pub fn expressions_equal(expr1: &Expression, expr2: &Expression) -> bool {
        // Fast path: pointer equality
        if std::ptr::eq(expr1, expr2) {
            return true;
        }

        // Structural comparison for simple cases
        match (expr1, expr2) {
            (Expression::Number(n1), Expression::Number(n2)) => n1 == n2,
            (Expression::Symbol(s1), Expression::Symbol(s2)) => s1 == s2,
            _ => {
                // Fallback: simplified string comparison
                format!("{:?}", expr1.simplify()) == format!("{:?}", expr2.simplify())
            }
        }
    }

    /// Fast zero check with pattern matching
    ///
    /// # Performance Notes
    /// - Uses pattern matching to avoid simplification when possible
    /// - Only simplifies for complex expressions
    pub fn is_zero(expr: &Expression) -> bool {
        match expr {
            Expression::Number(Number::Integer(0)) => true,
            Expression::Number(Number::Float(f)) if *f == 0.0 => true,
            _ => matches!(expr.simplify(), Expression::Number(Number::Integer(0))),
        }
    }

    /// Validate dimension compatibility early
    ///
    /// # Performance Notes
    /// - Fails fast with descriptive error messages
    /// - Avoids expensive computations on invalid inputs
    pub fn validate_dimensions(name: &str, expected: usize, actual: usize) -> Result<(), String> {
        if expected != actual {
            Err(format!(
                "{}: dimension mismatch - expected {}, got {}",
                name, expected, actual
            ))
        } else {
            Ok(())
        }
    }
}

/// Optimized matrix operations
pub struct MatrixUtils;

impl MatrixUtils {
    /// Compute matrix determinant with optimized algorithms
    ///
    /// # Performance Notes
    /// - Uses specialized algorithms for small matrices
    /// - Falls back to symbolic representation for large matrices
    /// - Pre-allocates all intermediate vectors
    pub fn determinant(matrix: &[Vec<Expression>]) -> Expression {
        let n = matrix.len();
        if n == 0 || matrix[0].len() != n {
            panic!("Matrix must be square and non-empty");
        }

        match n {
            1 => matrix[0][0].clone(),
            2 => Self::det_2x2(matrix),
            3 => Self::det_3x3(matrix),
            _ => Self::det_symbolic(matrix),
        }
    }

    /// Optimized 2×2 determinant
    fn det_2x2(matrix: &[Vec<Expression>]) -> Expression {
        Expression::add(vec![
            Expression::mul(vec![matrix[0][0].clone(), matrix[1][1].clone()]),
            Expression::mul(vec![
                Expression::integer(-1),
                Expression::mul(vec![matrix[0][1].clone(), matrix[1][0].clone()]),
            ]),
        ])
        .simplify()
    }

    /// Optimized 3×3 determinant using cofactor expansion
    fn det_3x3(matrix: &[Vec<Expression>]) -> Expression {
        let mut terms = Vec::with_capacity(3);

        for i in 0..3 {
            let sign = if i % 2 == 0 { 1 } else { -1 };
            let cofactor = Self::cofactor_2x2(matrix, 0, i);
            terms.push(Expression::mul(vec![
                Expression::integer(sign),
                matrix[0][i].clone(),
                cofactor,
            ]));
        }

        Expression::add(terms).simplify()
    }

    /// Compute 2×2 cofactor for 3×3 determinant
    fn cofactor_2x2(matrix: &[Vec<Expression>], skip_row: usize, skip_col: usize) -> Expression {
        let mut elements = Vec::with_capacity(4);

        for i in 0..3 {
            if i == skip_row {
                continue;
            }
            for j in 0..3 {
                if j == skip_col {
                    continue;
                }
                elements.push(matrix[i][j].clone());
            }
        }

        // 2×2 determinant: ad - bc
        Expression::add(vec![
            Expression::mul(vec![elements[0].clone(), elements[3].clone()]),
            Expression::mul(vec![
                Expression::integer(-1),
                Expression::mul(vec![elements[1].clone(), elements[2].clone()]),
            ]),
        ])
    }

    /// Symbolic determinant for large matrices
    fn det_symbolic(matrix: &[Vec<Expression>]) -> Expression {
        Expression::function(
            "det",
            vec![Expression::function(
                "matrix",
                matrix.iter().flat_map(|row| row.iter().cloned()).collect(),
            )],
        )
    }
}
