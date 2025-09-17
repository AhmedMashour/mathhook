//! Integration tests for matrix linear system solvers

use mathhook_core::core::{Expression, Number};
use mathhook_core::error::MathError;
use mathhook_core::matrices::unified::Matrix;
use mathhook_core::simplify::Simplify;
use num_bigint::BigInt;

fn expr_equals_integer(expr: &Expression, value: i64) -> bool {
    let simplified = expr.clone().simplify();
    match simplified {
        Expression::Number(Number::Integer(i)) => i == value,
        Expression::Number(Number::Rational(r)) => {
            r.denom() == &BigInt::from(1) && r.numer() == &BigInt::from(value)
        }
        _ => false,
    }
}

#[test]
fn test_forward_substitution_identity() {
    let l = Matrix::identity(3);
    let b = vec![
        Expression::integer(1),
        Expression::integer(2),
        Expression::integer(3),
    ];
    let x = l.forward_substitution(&b).unwrap();

    assert_eq!(x[0].simplify(), Expression::integer(1));
    assert_eq!(x[1].simplify(), Expression::integer(2));
    assert_eq!(x[2].simplify(), Expression::integer(3));
}

#[test]
fn test_forward_substitution_2x2() {
    let l = Matrix::from_arrays([[1, 0], [2, 1]]);
    let b = vec![Expression::integer(3), Expression::integer(8)];
    let x = l.forward_substitution(&b).unwrap();

    assert_eq!(x[0].simplify(), Expression::integer(3));
    assert_eq!(x[1].simplify(), Expression::integer(2));
}

#[test]
fn test_forward_substitution_zero_diagonal() {
    let l = Matrix::from_arrays([[0, 0], [1, 1]]);
    let b = vec![Expression::integer(1), Expression::integer(2)];
    let result = l.forward_substitution(&b);

    assert!(matches!(result, Err(MathError::DivisionByZero)));
}

#[test]
fn test_backward_substitution_identity() {
    let u = Matrix::identity(3);
    let b = vec![
        Expression::integer(1),
        Expression::integer(2),
        Expression::integer(3),
    ];
    let x = u.backward_substitution(&b).unwrap();

    assert_eq!(x[0].simplify(), Expression::integer(1));
    assert_eq!(x[1].simplify(), Expression::integer(2));
    assert_eq!(x[2].simplify(), Expression::integer(3));
}

#[test]
fn test_backward_substitution_2x2() {
    let u = Matrix::from_arrays([[1, 2], [0, 1]]);
    let b = vec![Expression::integer(5), Expression::integer(2)];
    let x = u.backward_substitution(&b).unwrap();

    assert_eq!(x[0].simplify(), Expression::integer(1));
    assert_eq!(x[1].simplify(), Expression::integer(2));
}

#[test]
fn test_backward_substitution_zero_diagonal() {
    let u = Matrix::from_arrays([[1, 1], [0, 0]]);
    let b = vec![Expression::integer(1), Expression::integer(2)];
    let result = u.backward_substitution(&b);

    assert!(matches!(result, Err(MathError::DivisionByZero)));
}

#[test]
fn test_solve_2x2_integer() {
    let a = Matrix::from_arrays([[2, 1], [1, 3]]);
    let b = vec![Expression::integer(5), Expression::integer(7)];
    let x = a.solve(&b).unwrap();

    let ax0 = Expression::add(vec![
        Expression::mul(vec![Expression::integer(2), x[0].clone()]),
        x[1].clone(),
    ])
    .simplify();
    let ax1 = Expression::add(vec![
        x[0].clone(),
        Expression::mul(vec![Expression::integer(3), x[1].clone()]),
    ])
    .simplify();

    assert!(
        expr_equals_integer(&ax0, 5),
        "Expected ax0 to equal 5, got {:?}",
        ax0
    );
    assert!(
        expr_equals_integer(&ax1, 7),
        "Expected ax1 to equal 7, got {:?}",
        ax1
    );
}

#[test]
fn test_solve_identity() {
    let a = Matrix::identity(2);
    let b = vec![Expression::integer(3), Expression::integer(7)];
    let x = a.solve(&b).unwrap();

    assert_eq!(x[0].simplify(), Expression::integer(3));
    assert_eq!(x[1].simplify(), Expression::integer(7));
}

#[test]
fn test_solve_dimension_mismatch() {
    let a = Matrix::identity(2);
    let b = vec![Expression::integer(1)];
    let result = a.solve(&b);

    assert!(matches!(result, Err(MathError::DomainError { .. })));
}

#[test]
fn test_solve_non_square() {
    let a = Matrix::from_arrays([[1, 2, 3], [4, 5, 6]]);
    let b = vec![Expression::integer(1), Expression::integer(2)];
    let result = a.solve(&b);

    assert!(matches!(result, Err(MathError::DomainError { .. })));
}

#[test]
fn test_solve_spd_uses_cholesky() {
    // Symmetric positive definite matrix [[4, 2], [2, 3]]
    let a = Matrix::from_arrays([[4, 2], [2, 3]]);
    let b = vec![Expression::integer(10), Expression::integer(8)];
    let x = a.solve(&b).unwrap();

    // Verify Ax = b
    let ax0 = Expression::add(vec![
        Expression::mul(vec![Expression::integer(4), x[0].clone()]),
        Expression::mul(vec![Expression::integer(2), x[1].clone()]),
    ])
    .simplify();
    let ax1 = Expression::add(vec![
        Expression::mul(vec![Expression::integer(2), x[0].clone()]),
        Expression::mul(vec![Expression::integer(3), x[1].clone()]),
    ])
    .simplify();

    assert!(
        expr_equals_integer(&ax0, 10),
        "Expected ax0 to equal 10, got {:?}",
        ax0
    );
    assert!(
        expr_equals_integer(&ax1, 8),
        "Expected ax1 to equal 8, got {:?}",
        ax1
    );
}
