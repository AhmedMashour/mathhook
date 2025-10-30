use super::*;
use crate::core::expression::Expression;
use crate::core::number::Number;

#[test]
fn test_gamma_positive_integers() {
    assert_eq!(gamma(&Expression::integer(1)).unwrap(), Expression::integer(1));
    assert_eq!(gamma(&Expression::integer(2)).unwrap(), Expression::integer(1));
    assert_eq!(gamma(&Expression::integer(3)).unwrap(), Expression::integer(2));
    assert_eq!(gamma(&Expression::integer(4)).unwrap(), Expression::integer(6));
    assert_eq!(gamma(&Expression::integer(5)).unwrap(), Expression::integer(24));
}

#[test]
fn test_gamma_half_integers() {
    let result = gamma(&Expression::float(0.5)).unwrap();
    let expected = Expression::sqrt(Expression::pi());
    assert_eq!(result, expected);

    let result_1_5 = gamma(&Expression::float(1.5)).unwrap();
    let sqrt_pi = Expression::sqrt(Expression::pi());
    let expected_1_5 = Expression::div(sqrt_pi, Expression::integer(2));
    assert_eq!(result_1_5, expected_1_5);
}

#[test]
fn test_gamma_symbolic() {
    use crate::core::symbol::Symbol;
    let x = Expression::Symbol(Symbol::new("x"));
    let result = gamma(&x).unwrap();
    assert_eq!(result.to_string(), "gamma(x)");
}

#[test]
fn test_gamma_domain_errors() {
    assert!(gamma(&Expression::integer(0)).is_err());
    assert!(gamma(&Expression::integer(-1)).is_err());
    assert!(gamma(&Expression::integer(-5)).is_err());

    assert!(gamma(&Expression::float(0.0)).is_err());
    assert!(gamma(&Expression::float(-1.0)).is_err());
    assert!(gamma(&Expression::float(-2.0)).is_err());
}

#[test]
fn test_lanczos_gamma_numerical() {
    let result = lanczos_gamma(5.0);
    assert!((result - 24.0).abs() < 1e-10);
}

#[test]
fn test_lanczos_gamma_accuracy() {
    let result_half = lanczos_gamma(0.5);
    let expected_half = std::f64::consts::PI.sqrt();
    assert!(
        (result_half - expected_half).abs() < 1e-14,
        "Γ(1/2) accuracy: expected {}, got {}",
        expected_half,
        result_half
    );
    let result_one = lanczos_gamma(1.0);
    assert!((result_one - 1.0).abs() < 1e-14, "Γ(1) = 1");
    let result_two = lanczos_gamma(2.0);
    assert!((result_two - 1.0).abs() < 1e-14, "Γ(2) = 1");
    let result_three = lanczos_gamma(3.0);
    assert!((result_three - 2.0).abs() < 1e-14, "Γ(3) = 2");
}

#[test]
fn test_gamma_float_numerical() {
    let result = gamma(&Expression::float(3.7)).unwrap();
    match result {
        Expression::Number(Number::Float(_)) => {}
        _ => panic!("Γ(3.7) should return numerical Float"),
    }
}

#[test]
fn test_lanczos_gamma_input_validation() {
    assert!(lanczos_gamma(f64::NAN).is_nan());
    assert!(lanczos_gamma(f64::INFINITY).is_nan());
    assert!(lanczos_gamma(0.0).is_infinite());
    assert!(lanczos_gamma(-1.0).is_infinite());
    assert!(lanczos_gamma(-2.0).is_infinite());
}
