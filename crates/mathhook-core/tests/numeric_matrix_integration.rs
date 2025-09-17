use mathhook_core::matrices::{CoreMatrixOps, Matrix};
use mathhook_core::Expression;

#[test]
fn test_numeric_fast_path_multiply() {
    let m1 = Matrix::dense(vec![
        vec![Expression::float(1.0), Expression::float(2.0)],
        vec![Expression::float(3.0), Expression::float(4.0)],
    ]);

    let m2 = Matrix::dense(vec![
        vec![Expression::float(5.0), Expression::float(6.0)],
        vec![Expression::float(7.0), Expression::float(8.0)],
    ]);

    assert!(m1.as_numeric().is_some());
    assert!(m2.as_numeric().is_some());

    let result = m1.multiply(&m2).unwrap();

    match result {
        Matrix::Dense(data) => {
            assert_eq!(data.rows.len(), 2);
            assert_eq!(data.rows[0].len(), 2);
        }
        _ => panic!("Expected dense result"),
    }
}

#[test]
fn test_numeric_fast_path_determinant() {
    let m = Matrix::dense(vec![
        vec![Expression::float(1.0), Expression::float(2.0)],
        vec![Expression::float(3.0), Expression::float(4.0)],
    ]);

    assert!(m.as_numeric().is_some());

    let det = m.determinant().unwrap();

    match det {
        Expression::Number(_) => {}
        _ => panic!("Expected numeric result"),
    }
}

#[test]
fn test_numeric_fast_path_inverse() {
    let m = Matrix::dense(vec![
        vec![Expression::float(4.0), Expression::float(7.0)],
        vec![Expression::float(2.0), Expression::float(6.0)],
    ]);

    assert!(m.as_numeric().is_some());

    let inv = m.inverse();

    match inv {
        Matrix::Dense(_) => {
            let product = m.multiply(&inv).unwrap();
            match product.get_element(0, 0) {
                Expression::Number(_) => {}
                _ => panic!("Expected numeric result"),
            }
        }
        _ => panic!("Expected dense result"),
    }
}

#[test]
fn test_symbolic_matrix_not_numeric() {
    let m = Matrix::dense(vec![
        vec![Expression::symbol("x"), Expression::integer(2)],
        vec![Expression::integer(3), Expression::symbol("y")],
    ]);

    assert!(m.as_numeric().is_none());
}

#[test]
fn test_numeric_matrix_conversion() {
    let m = Matrix::from_arrays([[1, 2], [3, 4]]);

    if let Some(numeric) = m.as_numeric() {
        assert_eq!(numeric.dimensions(), (2, 2));

        let back = numeric.to_matrix();
        assert_eq!(back.dimensions(), (2, 2));
    } else {
        panic!("Should convert to numeric");
    }
}

#[test]
fn test_identity_preserves_type() {
    let identity = Matrix::identity(3);
    let inverse = identity.inverse();

    assert!(matches!(inverse, Matrix::Identity(_)));

    let product = identity.multiply(&inverse).unwrap();
    assert!(matches!(product, Matrix::Identity(_)));
}

#[test]
fn test_diagonal_preserves_type() {
    let diag = Matrix::diagonal(vec![
        Expression::integer(2),
        Expression::integer(3),
        Expression::integer(4),
    ]);

    let inverse = diag.inverse();

    assert!(matches!(inverse, Matrix::Diagonal(_)));
}

#[test]
fn test_numeric_multiply_optimizes() {
    let id1 = Matrix::from_arrays([[1, 0], [0, 1]]);
    let id2 = Matrix::from_arrays([[1, 0], [0, 1]]);

    let product = id1.multiply(&id2).unwrap();

    assert!(matches!(product, Matrix::Identity(_)));
}
