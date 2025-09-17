//! Fast-path matrix simplification helpers
//!
//! Provides optimized direct matrix computation paths to avoid unnecessary
//! expression tree construction during simplification.

use crate::core::Expression;
use crate::error::MathError;
use crate::matrices::CoreMatrixOps;

/// Try to simplify A + B for matrices directly without building expression tree
///
/// Returns Some(Ok(expr)) if both operands are matrices with compatible dimensions.
/// Returns Some(Err(e)) if both operands are matrices but dimensions are incompatible.
/// Returns None if either operand is not a matrix.
pub fn try_matrix_add(a: &Expression, b: &Expression) -> Option<Result<Expression, MathError>> {
    match (a, b) {
        (Expression::Matrix(ma), Expression::Matrix(mb)) => Some(
            ma.add(mb)
                .map(|result| Expression::Matrix(Box::new(result))),
        ),
        _ => None,
    }
}

/// Try to simplify A * B for matrices directly without building expression tree
///
/// Returns Some(Ok(expr)) if both operands are matrices with compatible dimensions.
/// Returns Some(Err(e)) if both operands are matrices but dimensions are incompatible.
/// Returns None if either operand is not a matrix.
pub fn try_matrix_multiply(
    a: &Expression,
    b: &Expression,
) -> Option<Result<Expression, MathError>> {
    match (a, b) {
        (Expression::Matrix(ma), Expression::Matrix(mb)) => Some(
            ma.multiply(mb)
                .map(|result| Expression::Matrix(Box::new(result))),
        ),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::expr;
    use crate::simplify::Simplify;

    #[test]
    fn test_matrix_add_fast_path_compatible() {
        let a = Expression::matrix(vec![vec![expr!(1), expr!(2)], vec![expr!(3), expr!(4)]]);
        let b = Expression::matrix(vec![vec![expr!(5), expr!(6)], vec![expr!(7), expr!(8)]]);

        let result = try_matrix_add(&a, &b);
        assert!(result.is_some());

        match result {
            Some(Ok(Expression::Matrix(m))) => {
                let (rows, cols) = m.dimensions();
                assert_eq!(rows, 2);
                assert_eq!(cols, 2);
            }
            Some(Err(_)) => panic!("Expected Ok result for compatible matrices"),
            None => panic!("Expected Some for matrix operands"),
            _ => panic!("Expected matrix result"),
        }
    }

    #[test]
    fn test_matrix_add_fast_path_incompatible() {
        let a = Expression::matrix(vec![vec![expr!(1), expr!(2)], vec![expr!(3), expr!(4)]]);
        let b = Expression::matrix(vec![vec![expr!(5), expr!(6), expr!(7)]]);

        let result = try_matrix_add(&a, &b);
        assert!(result.is_some());

        match result {
            Some(Err(MathError::DomainError {
                operation, reason, ..
            })) => {
                assert_eq!(operation, "matrix_addition");
                assert!(reason.contains("2x2"));
                assert!(reason.contains("1x3"));
            }
            _ => panic!("Expected DomainError for incompatible dimensions"),
        }
    }

    #[test]
    fn test_matrix_add_fast_path_non_matrix() {
        let a = Expression::matrix(vec![vec![expr!(1), expr!(2)]]);
        let b = expr!(42);

        let result = try_matrix_add(&a, &b);
        assert!(result.is_none());
    }

    #[test]
    fn test_matrix_multiply_fast_path_compatible() {
        let a = Expression::matrix(vec![vec![expr!(1), expr!(2)], vec![expr!(3), expr!(4)]]);
        let b = Expression::matrix(vec![vec![expr!(5), expr!(6)], vec![expr!(7), expr!(8)]]);

        let result = try_matrix_multiply(&a, &b);
        assert!(result.is_some());

        match result {
            Some(Ok(Expression::Matrix(m))) => {
                let (rows, cols) = m.dimensions();
                assert_eq!(rows, 2);
                assert_eq!(cols, 2);
            }
            Some(Err(_)) => panic!("Expected Ok result for compatible matrices"),
            None => panic!("Expected Some for matrix operands"),
            _ => panic!("Expected matrix result"),
        }
    }

    #[test]
    fn test_matrix_multiply_fast_path_incompatible() {
        let a = Expression::matrix(vec![vec![expr!(1), expr!(2)]]);
        let b = Expression::matrix(vec![vec![expr!(5)], vec![expr!(6)], vec![expr!(7)]]);

        let result = try_matrix_multiply(&a, &b);
        assert!(result.is_some());

        match result {
            Some(Err(MathError::DomainError {
                operation, reason, ..
            })) => {
                assert_eq!(operation, "matrix_multiplication");
                assert!(reason.contains("1x2"));
                assert!(reason.contains("3x1"));
                assert!(reason.contains("2 != 3"));
            }
            _ => panic!("Expected DomainError for incompatible dimensions"),
        }
    }

    #[test]
    fn test_matrix_multiply_fast_path_non_matrix() {
        let a = Expression::matrix(vec![vec![expr!(1), expr!(2)]]);
        let b = expr!(x);

        let result = try_matrix_multiply(&a, &b);
        assert!(result.is_none());
    }

    #[test]
    fn test_mixed_symbolic_matrix() {
        let a = Expression::matrix(vec![vec![expr!(x), expr!(y)]]);
        let b = Expression::matrix(vec![vec![expr!(2), expr!(3)]]);

        let result = try_matrix_add(&a, &b);
        assert!(result.is_some());
        assert!(result.unwrap().is_ok());
    }

    /// Quick verification that the benchmark operations don't stack overflow
    #[test]
    fn test_benchmark_operations_no_stack_overflow() {
        fn create_test_matrix(size: usize) -> Expression {
            let mut rows = Vec::new();
            for i in 0..size {
                let mut row = Vec::new();
                for j in 0..size {
                    row.push(Expression::integer((i * size + j + 1) as i64));
                }
                rows.push(row);
            }
            Expression::matrix(rows)
        }

        // Test sizes from the benchmark: [2, 3, 4, 8, 16]
        for size in [2, 3, 4, 8, 16] {
            let matrix_a = create_test_matrix(size);
            let matrix_b = create_test_matrix(size);

            // These are exactly what the benchmark does
            let _add_result = Expression::add(vec![matrix_a.clone(), matrix_b.clone()]).simplify();
            let _mul_result = Expression::mul(vec![matrix_a.clone(), matrix_b.clone()]).simplify();
        }
    }
}
