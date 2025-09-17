//! Tests for polynomial GCD algorithms

use super::univariate::{univariate_gcd, univariate_gcd_modular};
use super::{integer_gcd, polynomial_gcd};
use crate::core::Expression;
use crate::symbol;

#[test]
fn test_integer_gcd() {
    assert_eq!(integer_gcd(12, 18), 6);
    assert_eq!(integer_gcd(0, 5), 5);
    assert_eq!(integer_gcd(5, 0), 5);
    assert_eq!(integer_gcd(-12, 18), 6);
    assert_eq!(integer_gcd(17, 13), 1);
    assert_eq!(integer_gcd(100, 25), 25);
    assert_eq!(integer_gcd(7, 7), 7);
}

#[test]
fn test_polynomial_gcd_integers() {
    let p1 = Expression::integer(12);
    let p2 = Expression::integer(18);
    let result = polynomial_gcd(&p1, &p2).unwrap();
    assert_eq!(result, Expression::integer(6));
}

#[test]
fn test_polynomial_gcd_zero() {
    let p1 = Expression::integer(0);
    let p2 = Expression::integer(5);
    let result = polynomial_gcd(&p1, &p2).unwrap();
    assert_eq!(result, Expression::integer(5));

    let result2 = polynomial_gcd(&p2, &p1).unwrap();
    assert_eq!(result2, Expression::integer(5));
}

#[test]
fn test_polynomial_gcd_one() {
    let p1 = Expression::integer(1);
    let p2 = Expression::integer(100);
    let result = polynomial_gcd(&p1, &p2).unwrap();
    assert_eq!(result, Expression::integer(1));

    let result2 = polynomial_gcd(&p2, &p1).unwrap();
    assert_eq!(result2, Expression::integer(1));
}

#[test]
fn test_polynomial_gcd_same_symbol() {
    let x = symbol!(x);
    let p1 = Expression::symbol(x.clone());
    let p2 = Expression::symbol(x.clone());
    let result = polynomial_gcd(&p1, &p2).unwrap();
    assert_eq!(result, Expression::symbol(x));
}

#[test]
fn test_polynomial_gcd_different_symbols() {
    let x = symbol!(x);
    let y = symbol!(y);
    let p1 = Expression::symbol(x);
    let p2 = Expression::symbol(y);
    let result = polynomial_gcd(&p1, &p2).unwrap();
    assert_eq!(result, Expression::integer(1));
}

#[test]
fn test_polynomial_gcd_negative_integers() {
    let p1 = Expression::integer(-12);
    let p2 = Expression::integer(18);
    let result = polynomial_gcd(&p1, &p2).unwrap();
    assert_eq!(result, Expression::integer(6));
}

#[test]
fn test_polynomial_gcd_large_integers() {
    let p1 = Expression::integer(1000000);
    let p2 = Expression::integer(500000);
    let result = polynomial_gcd(&p1, &p2).unwrap();
    assert_eq!(result, Expression::integer(500000));
}

#[test]
fn test_univariate_gcd_basic() {
    let x = symbol!(x);
    let p1 = Expression::pow(Expression::symbol(x.clone()), Expression::integer(2));
    let p2 = Expression::symbol(x.clone());
    let result = univariate_gcd(&p1, &p2, &x);
    assert!(result.is_ok());
}

#[test]
fn test_univariate_gcd_modular() {
    let x = symbol!(x);
    let p1 = Expression::add(vec![
        Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
        Expression::integer(-1),
    ]);
    let p2 = Expression::add(vec![Expression::symbol(x.clone()), Expression::integer(-1)]);
    let result = univariate_gcd_modular(&p1, &p2, &x);
    assert!(result.is_ok());
}
