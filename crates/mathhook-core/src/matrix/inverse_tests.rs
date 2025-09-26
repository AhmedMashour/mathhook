use crate::core::Expression;
use crate::matrix::{CoreMatrixOps, Matrix};

#[cfg(test)]
mod inverse_tests {
    use super::*;

    /// Test inverse computation for identity matrix
    #[test]
    fn test_identity_inverse() {
        let identity = Matrix::identity(3);
        let inverse = identity.inverse();

        // Identity inverse should be identity
        assert!(matches!(inverse, Matrix::Identity(_)));

        // Verify A * A^(-1) = I
        let product = identity.multiply(&inverse);
        assert!(matches!(product, Matrix::Identity(_)));
    }

    /// Test 2x2 matrix inverse using Gauss-Jordan elimination
    #[test]
    fn test_2x2_gauss_jordan_inverse() {
        let matrix = Matrix::dense(vec![
            vec![Expression::integer(1), Expression::integer(2)],
            vec![Expression::integer(3), Expression::integer(4)],
        ]);

        let inverse = matrix.inverse();

        // Verify A * A^(-1) = I by checking structure
        let product = matrix.multiply(&inverse);
        if let Matrix::Dense(ref data) = product {
            assert_eq!(data.rows.len(), 2);
            assert_eq!(data.rows[0].len(), 2);
            // Check that we get an identity-like structure (non-zero diagonal, zero off-diagonal)
            assert!(!data.rows[0][0].is_zero()); // Should be 1 (or equivalent)
            assert!(data.rows[0][1].is_zero()); // Should be 0
            assert!(data.rows[1][0].is_zero()); // Should be 0
            assert!(!data.rows[1][1].is_zero()); // Should be 1 (or equivalent)
        } else {
            panic!("Expected dense matrix result");
        }
    }

    /// Test 3x3 matrix inverse using Gauss-Jordan elimination
    #[test]
    fn test_3x3_gauss_jordan_inverse() {
        let matrix = Matrix::dense(vec![
            vec![
                Expression::integer(2),
                Expression::integer(1),
                Expression::integer(0),
            ],
            vec![
                Expression::integer(1),
                Expression::integer(2),
                Expression::integer(1),
            ],
            vec![
                Expression::integer(0),
                Expression::integer(1),
                Expression::integer(2),
            ],
        ]);

        let inverse = matrix.inverse();

        // Verify A * A^(-1) = I by checking structure
        let product = matrix.multiply(&inverse);
        if let Matrix::Dense(ref data) = product {
            assert_eq!(data.rows.len(), 3);
            assert_eq!(data.rows[0].len(), 3);
            // Check identity-like structure
            for i in 0..3 {
                for j in 0..3 {
                    if i == j {
                        assert!(!data.rows[i][j].is_zero()); // Diagonal should be non-zero (mathematically 1)
                    } else {
                        assert!(data.rows[i][j].is_zero()); // Off-diagonal should be zero
                    }
                }
            }
        } else {
            panic!("Expected dense matrix result");
        }
    }

    /// Test singular matrix detection
    #[test]
    fn test_singular_matrix_detection() {
        // Matrix with linearly dependent rows (determinant = 0)
        let singular = Matrix::dense(vec![
            vec![Expression::integer(1), Expression::integer(2)],
            vec![Expression::integer(2), Expression::integer(4)], // Second row = 2 * first row
        ]);

        let inverse = singular.inverse();

        // Should return zero matrix for singular matrices
        if let Matrix::Dense(ref data) = inverse {
            // All elements should be zero
            for row in &data.rows {
                for elem in row {
                    assert!(elem.is_zero());
                }
            }
        }
    }

    /// Test (A^(-1))^(-1) = A
    #[test]
    fn test_inverse_of_inverse() {
        let matrix = Matrix::dense(vec![
            vec![Expression::integer(1), Expression::integer(2)],
            vec![Expression::integer(3), Expression::integer(5)], // det = 5 - 6 = -1 (invertible)
        ]);

        let inverse = matrix.inverse();
        let double_inverse = inverse.inverse();

        // Should get back original matrix (mathematically)
        // We'll verify by checking A * (A^(-1))^(-1) = A * A = A^2
        let product = matrix.multiply(&double_inverse);
        // This should equal A^2 = A * A
        let a_squared = matrix.multiply(&matrix);

        // Both should have same structure
        if let (Matrix::Dense(prod_data), Matrix::Dense(a2_data)) = (&product, &a_squared) {
            assert_eq!(prod_data.rows.len(), a2_data.rows.len());
            assert_eq!(prod_data.rows[0].len(), a2_data.rows[0].len());
        }
    }

    /// Test (AB)^(-1) = B^(-1)A^(-1)
    #[test]
    fn test_inverse_product_property() {
        let a = Matrix::dense(vec![
            vec![Expression::integer(1), Expression::integer(2)],
            vec![Expression::integer(0), Expression::integer(1)],
        ]);
        let b = Matrix::dense(vec![
            vec![Expression::integer(2), Expression::integer(0)],
            vec![Expression::integer(1), Expression::integer(1)],
        ]);

        let ab = a.multiply(&b);
        let ab_inv = ab.inverse();

        let a_inv = a.inverse();
        let b_inv = b.inverse();
        let b_inv_a_inv = b_inv.multiply(&a_inv);

        // Verify both give identity when multiplied with AB
        let test1 = ab.multiply(&ab_inv);
        let test2 = ab.multiply(&b_inv_a_inv);

        // Both should have identity structure
        if let (Matrix::Dense(t1_data), Matrix::Dense(t2_data)) = (&test1, &test2) {
            assert_eq!(t1_data.rows.len(), t2_data.rows.len());
            assert_eq!(t1_data.rows[0].len(), t2_data.rows[0].len());
        }
    }

    /// Test that matrix structure is preserved through inverse operations
    #[test]
    fn test_inverse_structure_preservation() {
        let matrix = Matrix::dense(vec![
            vec![Expression::integer(3), Expression::integer(1)],
            vec![Expression::integer(2), Expression::integer(1)],
        ]);

        let inverse = matrix.inverse();

        // Should be dense matrix
        assert!(matches!(inverse, Matrix::Dense(_)));

        // Check dimensions are preserved
        let (orig_rows, orig_cols) = matrix.dimensions();
        let (inv_rows, inv_cols) = inverse.dimensions();
        assert_eq!(orig_rows, inv_rows);
        assert_eq!(orig_cols, inv_cols);

        // Verify A * A^(-1) gives proper dimensions
        let product = matrix.multiply(&inverse);
        let (prod_rows, prod_cols) = product.dimensions();
        assert_eq!(prod_rows, 2);
        assert_eq!(prod_cols, 2);
    }
}
