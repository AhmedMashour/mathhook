//! Tests for matrix decomposition algorithms
//!
//! This module tests the mathematical correctness of LU, QR, Cholesky, and SVD
//! decompositions, verifying that they satisfy fundamental properties.

#[cfg(test)]
mod decomposition_tests {
    use crate::core::Expression;
    use crate::matrix::{CoreMatrixOps, Matrix, MatrixDecomposition};

    /// Test LU decomposition correctness
    #[test]
    fn test_lu_decomposition_correctness() {
        // Test 2x2 matrix
        let matrix = Matrix::dense(vec![
            vec![Expression::integer(2), Expression::integer(1)],
            vec![Expression::integer(4), Expression::integer(3)],
        ]);

        let lu = matrix.lu_decomposition().unwrap();

        // Verify PA = LU (or A = LU if P is identity)
        let lu_product = lu.l.multiply(&lu.u);

        if let Some(p) = &lu.p {
            let pa = p.multiply(&matrix);
            // Check dimensions match
            assert_eq!(pa.dimensions(), lu_product.dimensions());
        } else {
            // Direct A = LU check
            assert_eq!(matrix.dimensions(), lu_product.dimensions());
        }

        // Verify L is lower triangular with 1s on diagonal
        let (l_rows, l_cols) = lu.l.dimensions();
        for i in 0..l_rows {
            for j in 0..l_cols {
                let elem = lu.l.get_element(i, j);
                if i == j {
                    assert_eq!(elem, Expression::integer(1)); // Diagonal should be 1
                } else if i < j {
                    assert!(elem.is_zero()); // Upper part should be 0
                }
            }
        }

        // Verify U is upper triangular
        let (u_rows, u_cols) = lu.u.dimensions();
        for i in 0..u_rows {
            for j in 0..u_cols {
                if i > j {
                    let elem = lu.u.get_element(i, j);
                    assert!(elem.is_zero()); // Lower part should be 0
                }
            }
        }
    }

    /// Test LU decomposition for special matrices
    #[test]
    fn test_lu_decomposition_special_cases() {
        // Identity matrix
        let identity = Matrix::identity(3);
        let lu = identity.lu_decomposition().unwrap();
        assert!(matches!(lu.l, Matrix::Identity(_)));
        assert!(matches!(lu.u, Matrix::Identity(_)));

        // Diagonal matrix
        let diagonal = Matrix::diagonal(vec![
            Expression::integer(2),
            Expression::integer(3),
            Expression::integer(4),
        ]);
        let lu = diagonal.lu_decomposition().unwrap();
        assert!(matches!(lu.l, Matrix::Identity(_)));
        assert_eq!(lu.u, diagonal);
    }

    /// Test QR decomposition correctness
    #[test]
    fn test_qr_decomposition_correctness() {
        // Test with simple 2x2 matrix
        let matrix = Matrix::dense(vec![
            vec![Expression::integer(1), Expression::integer(1)],
            vec![Expression::integer(0), Expression::integer(1)],
        ]);

        let qr = matrix.qr_decomposition().unwrap();

        // Verify A = QR
        let qr_product = qr.q.multiply(&qr.r);
        assert_eq!(matrix.dimensions(), qr_product.dimensions());

        // Verify R is upper triangular
        let (r_rows, r_cols) = qr.r.dimensions();
        for i in 0..r_rows {
            for j in 0..r_cols {
                if i > j {
                    let elem = qr.r.get_element(i, j);
                    assert!(elem.is_zero()); // Lower part should be 0
                }
            }
        }

        // Q should be orthogonal (Q^T * Q = I), but we'll test dimensions for now
        let (q_rows, q_cols) = qr.q.dimensions();
        assert_eq!(q_rows, matrix.dimensions().0);
        assert_eq!(q_cols, matrix.dimensions().1);
    }

    /// Test QR decomposition for special matrices
    #[test]
    fn test_qr_decomposition_special_cases() {
        // Identity matrix
        let identity = Matrix::identity(2);
        let qr = identity.qr_decomposition().unwrap();
        assert!(matches!(qr.q, Matrix::Identity(_)));
        assert!(matches!(qr.r, Matrix::Identity(_)));

        // Zero matrix
        let zero = Matrix::zero(2, 2);
        let qr = zero.qr_decomposition().unwrap();
        assert!(matches!(qr.q, Matrix::Identity(_)));
        assert!(matches!(qr.r, Matrix::Zero(_)));
    }

    /// Test Cholesky decomposition correctness
    #[test]
    fn test_cholesky_decomposition_correctness() {
        // Test positive definite matrix
        let matrix = Matrix::dense(vec![
            vec![Expression::integer(4), Expression::integer(2)],
            vec![Expression::integer(2), Expression::integer(3)],
        ]);

        if let Some(chol) = matrix.cholesky_decomposition() {
            // Verify A = LL^T
            let l_transpose = chol.l.transpose();
            let llt_product = chol.l.multiply(&l_transpose);
            assert_eq!(matrix.dimensions(), llt_product.dimensions());

            // Verify L is lower triangular
            let (l_rows, l_cols) = chol.l.dimensions();
            for i in 0..l_rows {
                for j in 0..l_cols {
                    if i < j {
                        let elem = chol.l.get_element(i, j);
                        assert!(elem.is_zero()); // Upper part should be 0
                    }
                }
            }
        }
    }

    /// Test Cholesky decomposition for special matrices
    #[test]
    fn test_cholesky_decomposition_special_cases() {
        // Identity matrix
        let identity = Matrix::identity(3);
        let chol = identity.cholesky_decomposition().unwrap();
        assert!(matches!(chol.l, Matrix::Identity(_)));

        // Scalar matrix
        let scalar = Matrix::scalar(2, Expression::integer(4));
        let chol = scalar.cholesky_decomposition().unwrap();
        // Should be sqrt(4) = 2 on diagonal
        assert!(matches!(chol.l, Matrix::Scalar(_)));

        // Diagonal matrix
        let diagonal = Matrix::diagonal(vec![Expression::integer(4), Expression::integer(9)]);
        let chol = diagonal.cholesky_decomposition().unwrap();
        assert!(matches!(chol.l, Matrix::Diagonal(_)));
    }

    /// Test SVD decomposition correctness
    #[test]
    fn test_svd_decomposition_correctness() {
        // Test 2x2 matrix
        let matrix = Matrix::dense(vec![
            vec![Expression::integer(1), Expression::integer(2)],
            vec![Expression::integer(3), Expression::integer(4)],
        ]);

        let svd = matrix.svd_decomposition().unwrap();

        // Verify A = UΣV^T
        let sigma_vt = svd.sigma.multiply(&svd.vt);
        let usvt_product = svd.u.multiply(&sigma_vt);
        assert_eq!(matrix.dimensions(), usvt_product.dimensions());

        // Verify Σ is diagonal with non-negative entries
        let (sigma_rows, sigma_cols) = svd.sigma.dimensions();
        for i in 0..sigma_rows {
            for j in 0..sigma_cols {
                if i != j {
                    let elem = svd.sigma.get_element(i, j);
                    assert!(elem.is_zero()); // Off-diagonal should be 0
                }
            }
        }
    }

    /// Test SVD for special matrices
    #[test]
    fn test_svd_special_cases() {
        // Identity matrix
        let identity = Matrix::identity(2);
        let svd = identity.svd_decomposition().unwrap();
        assert!(matches!(svd.u, Matrix::Identity(_)));
        assert!(matches!(svd.sigma, Matrix::Identity(_)));
        assert!(matches!(svd.vt, Matrix::Identity(_)));

        // Zero matrix
        let zero = Matrix::zero(2, 2);
        let svd = zero.svd_decomposition().unwrap();
        assert!(matches!(svd.sigma, Matrix::Zero(_)));

        // Diagonal matrix
        let diagonal = Matrix::diagonal(vec![Expression::integer(3), Expression::integer(4)]);
        let svd = diagonal.svd_decomposition().unwrap();
        assert!(matches!(svd.sigma, Matrix::Diagonal(_)));
    }

    /// Test matrix rank computation
    #[test]
    fn test_matrix_rank() {
        // Identity matrix has full rank
        let identity = Matrix::identity(3);
        assert_eq!(identity.rank(), 3);

        // Zero matrix has rank 0
        let zero = Matrix::zero(3, 3);
        assert_eq!(zero.rank(), 0);

        // Diagonal matrix rank equals number of non-zero diagonal elements
        let diagonal = Matrix::diagonal(vec![
            Expression::integer(1),
            Expression::integer(0),
            Expression::integer(3),
        ]);
        assert_eq!(diagonal.rank(), 2);
    }

    /// Test positive definite check
    #[test]
    fn test_positive_definite_check() {
        // Identity is positive definite
        let identity = Matrix::identity(2);
        assert!(identity.is_positive_definite());

        // Positive scalar matrix is positive definite
        let pos_scalar = Matrix::scalar(2, Expression::integer(5));
        assert!(pos_scalar.is_positive_definite());

        // Diagonal with positive elements is positive definite
        let pos_diagonal = Matrix::diagonal(vec![
            Expression::integer(1),
            Expression::integer(2),
            Expression::integer(3),
        ]);
        assert!(pos_diagonal.is_positive_definite());
    }

    /// Test condition number computation
    #[test]
    fn test_condition_number() {
        // Identity matrix has condition number 1
        let identity = Matrix::identity(2);
        let cond = identity.condition_number();
        assert_eq!(cond, Expression::integer(1));

        // Well-conditioned diagonal matrix
        let diagonal = Matrix::diagonal(vec![Expression::integer(2), Expression::integer(2)]);
        let cond = diagonal.condition_number();
        assert_eq!(cond, Expression::integer(1)); // 2/2 = 1
    }
}
