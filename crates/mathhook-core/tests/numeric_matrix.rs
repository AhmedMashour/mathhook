use mathhook_core::NumericMatrix;

#[test]
fn test_numeric_matrix_public_api() {
    let m = NumericMatrix::zeros(2, 2).unwrap();
    assert_eq!(m.dimensions(), (2, 2));

    let id = NumericMatrix::identity(3).unwrap();
    assert_eq!(id.dimensions(), (3, 3));
    assert_eq!(id.get(0, 0).unwrap(), 1.0);
    assert_eq!(id.get(1, 1).unwrap(), 1.0);
}

#[test]
fn test_numeric_matrix_arithmetic() {
    let a = NumericMatrix::from_flat(2, 2, vec![1.0, 2.0, 3.0, 4.0]).unwrap();
    let b = NumericMatrix::from_flat(2, 2, vec![5.0, 6.0, 7.0, 8.0]).unwrap();

    let c = a.add(&b).unwrap();
    assert_eq!(c.get(0, 0).unwrap(), 6.0);

    let d = a.sub(&b).unwrap();
    assert_eq!(d.get(0, 0).unwrap(), -4.0);

    let e = a.scalar_mul(2.0);
    assert_eq!(e.get(0, 0).unwrap(), 2.0);
}

#[test]
fn test_numeric_matrix_multiply() {
    let a = NumericMatrix::from_flat(2, 3, vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]).unwrap();
    let b = NumericMatrix::from_flat(3, 2, vec![7.0, 8.0, 9.0, 10.0, 11.0, 12.0]).unwrap();
    let c = a.multiply(&b).unwrap();

    assert_eq!(c.dimensions(), (2, 2));
    assert_eq!(c.get(0, 0).unwrap(), 58.0);
    assert_eq!(c.get(0, 1).unwrap(), 64.0);
}

#[test]
fn test_numeric_matrix_from_fn() {
    let m = NumericMatrix::from_fn(3, 3, |i, j| (i * 3 + j) as f64).unwrap();
    assert_eq!(m.get(0, 0).unwrap(), 0.0);
    assert_eq!(m.get(2, 2).unwrap(), 8.0);
}

#[test]
fn test_numeric_matrix_transpose() {
    let a = NumericMatrix::from_flat(2, 3, vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]).unwrap();
    let t = a.transpose();
    assert_eq!(t.dimensions(), (3, 2));
    assert_eq!(t.get(0, 0).unwrap(), 1.0);
    assert_eq!(t.get(2, 1).unwrap(), 6.0);
}

#[test]
fn test_numeric_matrix_display() {
    let m = NumericMatrix::from_flat(2, 2, vec![1.0, 2.0, 3.0, 4.0]).unwrap();
    let s = m.to_string();
    assert_eq!(s, "[[1, 2], [3, 4]]");
}
