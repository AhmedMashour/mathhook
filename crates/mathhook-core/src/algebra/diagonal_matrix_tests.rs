//! Comprehensive tests for DiagonalMatrix mathematical properties and optimizations
//!
//! Tests verify that diagonal matrix operations maintain perfect mathematical accuracy
//! and leverage O(n) memory optimizations.

#[cfg(test)]
mod tests {
    use crate::core::expression::Expression;
    use crate::matrices::operations::MatrixOperations;

    /// Test diagonal matrix creation and basic properties
    #[test]
    fn test_diagonal_matrix_creation() {
        let diag = Expression::diagonal_matrix(vec![
            Expression::integer(2),
            Expression::integer(3),
            Expression::integer(5),
        ]);

        // Test dimensions
        assert_eq!(diag.matrix_dimensions(), Some((3, 3)));
        assert!(diag.is_matrix());

        // Test diagonal property
        assert!(diag.is_diagonal());
        assert!(!diag.is_identity_matrix());
        assert!(!diag.is_zero_matrix());
    }

    /// Test diagonal matrix addition optimizations
    #[test]
    fn test_diagonal_matrix_addition() {
        let diag1 = Expression::diagonal_matrix(vec![
            Expression::integer(1),
            Expression::integer(2),
            Expression::integer(3),
        ]);
        let diag2 = Expression::diagonal_matrix(vec![
            Expression::integer(4),
            Expression::integer(5),
            Expression::integer(6),
        ]);

        // D1 + D2 should be diagonal with element-wise addition
        let result = diag1.matrix_add(&diag2);
        let expected = Expression::diagonal_matrix(vec![
            Expression::integer(5), // 1 + 4
            Expression::integer(7), // 2 + 5
            Expression::integer(9), // 3 + 6
        ]);

        assert_eq!(result, expected);
        assert!(result.is_diagonal());
    }

    /// Test diagonal matrix + identity matrix optimization
    #[test]
    fn test_diagonal_plus_identity() {
        let diag = Expression::diagonal_matrix(vec![
            Expression::integer(2),
            Expression::integer(3),
            Expression::integer(4),
        ]);
        let identity = Expression::identity_matrix(3);

        // D + I should add 1 to each diagonal element
        let result = diag.matrix_add(&identity);
        let expected = Expression::diagonal_matrix(vec![
            Expression::integer(3), // 2 + 1
            Expression::integer(4), // 3 + 1
            Expression::integer(5), // 4 + 1
        ]);

        assert_eq!(result, expected);
        assert!(result.is_diagonal());

        // Test commutativity: I + D = D + I
        let result2 = identity.matrix_add(&diag);
        assert_eq!(result, result2);
    }

    /// Test diagonal matrix optimization from dense matrix
    #[test]
    fn test_diagonal_matrix_optimization() {
        // Create a dense matrix that is actually diagonal
        let dense = Expression::matrix(vec![
            vec![
                Expression::integer(2),
                Expression::integer(0),
                Expression::integer(0),
            ],
            vec![
                Expression::integer(0),
                Expression::integer(3),
                Expression::integer(0),
            ],
            vec![
                Expression::integer(0),
                Expression::integer(0),
                Expression::integer(4),
            ],
        ]);

        // Should be optimized to diagonal matrix
        let optimized = dense;
        let expected = Expression::diagonal_matrix(vec![
            Expression::integer(2),
            Expression::integer(3),
            Expression::integer(4),
        ]);

        assert_eq!(optimized, expected);
        assert!(optimized.is_diagonal());
    }

    /// Test diagonal matrix with zero elements
    #[test]
    fn test_diagonal_matrix_with_zeros() {
        let diag = Expression::diagonal_matrix(vec![
            Expression::integer(2),
            Expression::integer(0), // Zero on diagonal
            Expression::integer(3),
        ]);

        // Should remain diagonal (not optimized to zero matrix)
        assert!(diag.is_diagonal());
        assert!(!diag.is_zero_matrix());

        // Determinant should be zero (has zero on diagonal)
        let det = diag.matrix_determinant();
        assert_eq!(det, Expression::integer(0));

        // Trace should be sum including zero
        let trace = diag.matrix_trace();
        assert_eq!(trace, Expression::integer(5)); // 2 + 0 + 3
    }
}
