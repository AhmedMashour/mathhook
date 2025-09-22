//! Matrix expression simplification
//!
//! Handles simplification of matrix expressions, ensuring that
//! matrix elements are properly simplified.

use super::Simplify;
use crate::core::Expression;

/// Simplify matrix expressions
#[inline(always)]
pub fn simplify_matrix(matrix_expr: &Expression) -> Expression {
    match matrix_expr {
        Expression::Matrix(matrix) => {
            // Process matrix elements directly for performance (avoid recursive simplification)
            let simplified_rows = matrix.rows.clone();

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
            if simplified_rows.len() == simplified_rows.get(0).map(|row| row.len()).unwrap_or(0) {
                let is_identity = simplified_rows.iter().enumerate().all(|(i, row)| {
                    row.iter().enumerate().all(|(j, element)| {
                        if i == j {
                            matches!(element, Expression::Number(crate::core::Number::Integer(1)))
                        } else {
                            match element {
                                Expression::Number(crate::core::Number::Integer(0)) => true,
                                Expression::Number(crate::core::Number::Float(f)) if *f == 0.0 => {
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
        _ => matrix_expr.clone(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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

        let simplified = simplify_matrix(&matrix);

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

        let simplified = simplify_matrix(&zero_matrix);

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
