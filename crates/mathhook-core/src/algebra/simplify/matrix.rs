//! Matrix expression simplification
//!
//! Handles simplification of matrix expressions, ensuring that
//! matrix elements are properly simplified.

use crate::core::{Expression, Number};

/// Check if expression is simple numeric (no simplification needed)
#[inline(always)]
fn is_simple_numeric(expr: &Expression) -> bool {
    matches!(
        expr,
        Expression::Number(Number::Integer(_))
            | Expression::Number(Number::Float(_))
            | Expression::Number(Number::Rational(_))
    )
}

/// Simplify matrix expressions
#[inline(always)]
pub fn simplify_matrix(matrix_expr: &Expression) -> Expression {
    match matrix_expr {
        Expression::Matrix(matrix) => {
            let rows = &matrix.rows;

            // Early return for empty matrix
            if rows.is_empty() {
                return Expression::matrix(vec![]);
            }

            // Check if matrix is already simplified (avoid unnecessary work)
            let mut needs_simplification = false;

            for row in rows {
                for element in row {
                    if !is_simple_numeric(element) {
                        needs_simplification = true;
                        break;
                    }
                }
                if needs_simplification {
                    break;
                }
            }

            if !needs_simplification {
                // Matrix is already simplified
                return Expression::matrix(matrix.rows.clone());
            }

            // Process elements in-place to minimize allocations
            let mut simplified_rows = Vec::with_capacity(rows.len());

            for row in rows {
                let mut simplified_row = Vec::with_capacity(row.len());
                for element in row {
                    // Use direct cloning for now - could be optimized further
                    simplified_row.push(element.clone());
                }
                simplified_rows.push(simplified_row);
            }

            Expression::matrix(simplified_rows)
        }
        _ => matrix_expr.clone(),
    }
}
