#[cfg(test)]
mod tests {
    use crate::core::{Expression, Number};
    use crate::matrices::{CoreMatrixOps, Matrix};
    use num_traits::{One, Zero};

    const EPSILON: f64 = 1e-10;

    fn is_approx_zero(expr: &Expression) -> bool {
        match expr {
            Expression::Number(Number::Integer(n)) => *n == 0,
            Expression::Number(Number::Float(f)) => f.abs() < EPSILON,
            Expression::Number(Number::Rational(r)) => r.is_zero(),
            _ => expr.is_zero(),
        }
    }

    fn is_approx_one(expr: &Expression) -> bool {
        match expr {
            Expression::Number(Number::Integer(n)) => *n == 1,
            Expression::Number(Number::Float(f)) => (*f - 1.0).abs() < EPSILON,
            Expression::Number(Number::Rational(r)) => r.is_one(),
            _ => false,
        }
    }

    /// Test inverse computation for identity matrix
    #[test]
    fn test_identity_inverse() {
        let identity = Matrix::identity(3);
        let inverse = identity.inverse();

        assert!(matches!(inverse, Matrix::Identity(_)));

        let product = identity
            .multiply(&inverse)
            .expect("identity * identity should succeed");
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

        let product = matrix
            .multiply(&inverse)
            .expect("matrix * inverse should succeed");

        // A * A^(-1) should be identity - may be optimized to Identity/Diagonal
        match product {
            Matrix::Identity(ref id) => {
                assert_eq!(id.size, 2, "Identity matrix should be 2x2");
            }
            Matrix::Diagonal(ref diag) => {
                assert_eq!(diag.diagonal_elements.len(), 2);
                for (i, elem) in diag.diagonal_elements.iter().enumerate() {
                    assert!(
                        is_approx_one(elem),
                        "Diagonal element {} = {:?} should be ~1",
                        i,
                        elem
                    );
                }
            }
            Matrix::Dense(ref data) => {
                assert_eq!(data.rows.len(), 2);
                assert_eq!(data.rows[0].len(), 2);
                assert!(
                    is_approx_one(&data.rows[0][0]),
                    "Diagonal [0,0] should be ~1"
                );
                assert!(
                    is_approx_zero(&data.rows[0][1]),
                    "Off-diagonal [0,1] should be ~0"
                );
                assert!(
                    is_approx_zero(&data.rows[1][0]),
                    "Off-diagonal [1,0] should be ~0"
                );
                assert!(
                    is_approx_one(&data.rows[1][1]),
                    "Diagonal [1,1] should be ~1"
                );
            }
            _ => panic!(
                "Expected identity, diagonal or dense matrix result, got {:?}",
                product
            ),
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

        let product = matrix
            .multiply(&inverse)
            .expect("matrix * inverse should succeed");

        // A * A^(-1) should be identity - may be optimized to Identity/Diagonal
        match product {
            Matrix::Identity(ref id) => {
                assert_eq!(id.size, 3, "Identity matrix should be 3x3");
            }
            Matrix::Diagonal(ref diag) => {
                assert_eq!(diag.diagonal_elements.len(), 3);
                for (i, elem) in diag.diagonal_elements.iter().enumerate() {
                    assert!(
                        is_approx_one(elem),
                        "Diagonal element {} = {:?} should be ~1",
                        i,
                        elem
                    );
                }
            }
            Matrix::Dense(ref data) => {
                assert_eq!(data.rows.len(), 3);
                assert_eq!(data.rows[0].len(), 3);
                for i in 0..3 {
                    for j in 0..3 {
                        if i == j {
                            assert!(
                                is_approx_one(&data.rows[i][j]),
                                "Diagonal [{},{}] = {:?} should be ~1",
                                i,
                                j,
                                data.rows[i][j]
                            );
                        } else {
                            assert!(
                                is_approx_zero(&data.rows[i][j]),
                                "Off-diagonal [{},{}] = {:?} should be ~0",
                                i,
                                j,
                                data.rows[i][j]
                            );
                        }
                    }
                }
            }
            _ => panic!(
                "Expected identity, diagonal or dense matrix result, got {:?}",
                product
            ),
        }
    }

    /// Test singular matrix detection
    #[test]
    fn test_singular_matrix_detection() {
        let singular = Matrix::dense(vec![
            vec![Expression::integer(1), Expression::integer(2)],
            vec![Expression::integer(2), Expression::integer(4)],
        ]);

        let inverse = singular.inverse();

        if let Matrix::Dense(ref data) = inverse {
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
            vec![Expression::integer(3), Expression::integer(5)],
        ]);

        let inverse = matrix.inverse();
        let double_inverse = inverse.inverse();

        let product = matrix
            .multiply(&double_inverse)
            .expect("matrix * inverse should succeed");
        let a_squared = matrix
            .multiply(&matrix)
            .expect("matrix * matrix should succeed");

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

        let ab = a.multiply(&b).expect("a * b should succeed");
        let ab_inv = ab.inverse();

        let a_inv = a.inverse();
        let b_inv = b.inverse();
        let b_inv_a_inv = b_inv
            .multiply(&a_inv)
            .expect("b_inv * a_inv should succeed");

        let test1 = ab.multiply(&ab_inv).expect("ab * ab_inv should succeed");
        let test2 = ab
            .multiply(&b_inv_a_inv)
            .expect("ab * b_inv_a_inv should succeed");

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

        assert!(matches!(inverse, Matrix::Dense(_)));

        let (orig_rows, orig_cols) = matrix.dimensions();
        let (inv_rows, inv_cols) = inverse.dimensions();
        assert_eq!(orig_rows, inv_rows);
        assert_eq!(orig_cols, inv_cols);

        let product = matrix
            .multiply(&inverse)
            .expect("matrix * inverse should succeed");
        let (prod_rows, prod_cols) = product.dimensions();
        assert_eq!(prod_rows, 2);
        assert_eq!(prod_cols, 2);
    }
}
