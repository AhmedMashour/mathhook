use super::*;
use crate::core::expression::Expression;
use crate::core::number::Number;

#[test]
fn test_beta_symmetry() {
    let result_ab = beta_numerical(2.5, 3.7);
    let result_ba = beta_numerical(3.7, 2.5);
    assert!(
        (result_ab - result_ba).abs() < 1e-14,
        "Beta symmetry: B(a,b) = B(b,a)"
    );
}

#[test]
fn test_beta_numerical_evaluation() {
    let result = beta_numerical(2.0, 3.0);
    assert!((result - 1.0 / 12.0).abs() < 1e-14, "B(2,3) = 1/12");
    let result_2_5 = beta_numerical(2.0, 5.0);
    assert!((result_2_5 - 1.0 / 30.0).abs() < 1e-14, "B(2,5) = 1/30");
}

#[test]
fn test_beta_float_evaluation() {
    let a = Expression::float(2.5);
    let b = Expression::float(3.7);
    let result = beta(&a, &b).unwrap();
    match result {
        Expression::Number(Number::Float(_)) => {}
        _ => panic!("Beta with float inputs should return numerical result"),
    }
}

#[test]
fn test_beta_mixed_evaluation() {
    let a = Expression::float(2.5);
    let b = Expression::integer(3);
    let result = beta(&a, &b).unwrap();
    match result {
        Expression::Number(Number::Float(_)) => {}
        _ => panic!("Beta with mixed inputs should return numerical result"),
    }
}

#[test]
fn test_beta_special_values() {
    let result = beta(&Expression::integer(1), &Expression::integer(1)).unwrap();
    assert_eq!(result, Expression::integer(1));

    let result = beta(&Expression::integer(2), &Expression::integer(3)).unwrap();
    assert_eq!(result, Expression::rational(1, 12));
}

#[test]
fn test_beta_domain_errors() {
    assert!(beta(&Expression::integer(0), &Expression::integer(1)).is_err());
    assert!(beta(&Expression::integer(1), &Expression::integer(0)).is_err());
    assert!(beta(&Expression::integer(-1), &Expression::integer(1)).is_err());

    assert!(beta(&Expression::float(0.0), &Expression::float(1.0)).is_err());
    assert!(beta(&Expression::float(-1.0), &Expression::float(1.0)).is_err());
}
