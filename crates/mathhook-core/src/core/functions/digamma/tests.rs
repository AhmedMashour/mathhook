use super::*;
use crate::core::expression::Expression;

#[test]
fn test_digamma_symbolic() {
    use crate::symbol;
    let z = Expression::symbol(symbol!(z));
    let result = digamma(&z).unwrap();
    assert_eq!(result.to_string(), "digamma(z)");
}

#[test]
fn test_digamma_positive_integer() {
    let z = Expression::integer(1);
    let result = digamma(&z).unwrap();
    assert_eq!(result.to_string(), "digamma(1)");
}

#[test]
fn test_digamma_domain_errors() {
    assert!(digamma(&Expression::integer(0)).is_err());
    assert!(digamma(&Expression::integer(-1)).is_err());
    assert!(digamma(&Expression::integer(-5)).is_err());

    assert!(digamma(&Expression::float(0.0)).is_err());
    assert!(digamma(&Expression::float(-1.0)).is_err());
    assert!(digamma(&Expression::float(-2.0)).is_err());
}
