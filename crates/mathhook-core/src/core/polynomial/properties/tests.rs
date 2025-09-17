//! Tests for polynomial properties

use crate::core::polynomial::PolynomialProperties;
use crate::core::Expression;
use crate::{expr, symbol};

#[test]
fn test_degree() {
    let x = symbol!(x);
    let poly = expr!(x ^ 3);
    assert_eq!(poly.degree(&x), Some(3));
}

#[test]
fn test_degree_constant() {
    let x = symbol!(x);
    assert_eq!(Expression::integer(5).degree(&x), Some(0));
}

#[test]
fn test_degree_linear() {
    let x = symbol!(x);
    let poly = Expression::symbol(x.clone());
    assert_eq!(poly.degree(&x), Some(1));
}

#[test]
fn test_degree_polynomial() {
    let x = symbol!(x);
    // x^2 + x + 1
    let poly = Expression::add(vec![
        Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
        Expression::symbol(x.clone()),
        Expression::integer(1),
    ]);
    assert_eq!(poly.degree(&x), Some(2));
}

#[test]
fn test_total_degree() {
    let x = symbol!(x);
    let y = symbol!(y);
    let poly = Expression::mul(vec![
        Expression::symbol(x.clone()),
        Expression::symbol(y.clone()),
    ]);
    assert_eq!(poly.total_degree(), Some(2));
}

#[test]
fn test_total_degree_multivariate() {
    let x = symbol!(x);
    let y = symbol!(y);
    // x^2 * y + x * y^2
    let poly = Expression::add(vec![
        Expression::mul(vec![
            Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
            Expression::symbol(y.clone()),
        ]),
        Expression::mul(vec![
            Expression::symbol(x.clone()),
            Expression::pow(Expression::symbol(y.clone()), Expression::integer(2)),
        ]),
    ]);
    assert_eq!(poly.total_degree(), Some(3));
}

#[test]
fn test_leading_coefficient_simple() {
    let x = symbol!(x);
    let poly = Expression::symbol(x.clone());
    assert_eq!(poly.leading_coefficient(&x), Expression::integer(1));
}

#[test]
fn test_leading_coefficient_with_coef() {
    let x = symbol!(x);
    // 5 * x^2
    let poly = Expression::mul(vec![
        Expression::integer(5),
        Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
    ]);
    assert_eq!(poly.leading_coefficient(&x), Expression::integer(5));
}

#[test]
fn test_content_simple() {
    // 6x + 9 -> content is 3
    let x = symbol!(x);
    let poly = Expression::add(vec![
        Expression::mul(vec![Expression::integer(6), Expression::symbol(x.clone())]),
        Expression::integer(9),
    ]);
    assert_eq!(poly.content(), Expression::integer(3));
}

#[test]
fn test_content_coprime() {
    let x = symbol!(x);
    // 2x + 3 -> content is 1 (coefficients are coprime)
    let poly = Expression::add(vec![
        Expression::mul(vec![Expression::integer(2), Expression::symbol(x.clone())]),
        Expression::integer(3),
    ]);
    assert_eq!(poly.content(), Expression::integer(1));
}

#[test]
fn test_primitive_part() {
    let x = symbol!(x);
    // 6x + 9 -> primitive part is 2x + 3
    let poly = Expression::add(vec![
        Expression::mul(vec![Expression::integer(6), Expression::symbol(x.clone())]),
        Expression::integer(9),
    ]);
    let pp = poly.primitive_part();
    // Should be 2x + 3
    let expected = Expression::add(vec![
        Expression::mul(vec![Expression::integer(2), Expression::symbol(x.clone())]),
        Expression::integer(3),
    ]);
    assert_eq!(pp, expected);
}

#[test]
fn test_integer_gcd() {
    use super::content::integer_gcd;
    assert_eq!(integer_gcd(12, 18), 6);
    assert_eq!(integer_gcd(6, 9), 3);
    assert_eq!(integer_gcd(7, 11), 1);
    assert_eq!(integer_gcd(100, 25), 25);
}
