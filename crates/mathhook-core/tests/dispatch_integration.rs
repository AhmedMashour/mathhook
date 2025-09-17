//! Integration tests for unified polynomial dispatch layer

use mathhook_core::core::polynomial::dispatch::{polynomial_div, polynomial_gcd, polynomial_rem};
use mathhook_core::{expr, symbol, Expression, Number};
use num_bigint::BigInt;
use num_rational::BigRational;

#[test]
fn test_integer_polynomial_gcd() {
    let x = symbol!(x);
    let p1 = expr!((x ^ 2) - 1);
    let p2 = expr!(x - 1);

    let gcd = polynomial_gcd(&p1, &p2, &x);

    assert!(!gcd.is_zero());
    let expected = expr!(x - 1);
    assert_eq!(gcd.to_string(), expected.to_string());
}

#[test]
fn test_integer_polynomial_division() {
    let x = symbol!(x);
    let dividend = expr!((x ^ 2) + (3 * x) + 2);
    let divisor = expr!(x + 1);

    let (quot, rem) = polynomial_div(&dividend, &divisor, &x);

    assert!(!quot.is_zero());
    assert!(rem.is_zero());
}

#[test]
fn test_integer_polynomial_remainder() {
    let x = symbol!(x);
    let dividend = expr!((x ^ 2) + 1);
    let divisor = expr!(x - 1);

    let rem = polynomial_rem(&dividend, &divisor, &x);

    assert!(!rem.is_zero());
    assert_eq!(rem, Expression::integer(2));
}

#[test]
fn test_rational_polynomial_gcd() {
    let x = symbol!(x);
    let half = Expression::Number(Number::rational(BigRational::new(
        BigInt::from(1),
        BigInt::from(2),
    )));

    let p1 = Expression::mul(vec![half, expr!((x ^ 2) - 1)]);
    let p2 = expr!(x - 1);

    let gcd = polynomial_gcd(&p1, &p2, &x);

    assert!(!gcd.is_zero());
}

#[test]
fn test_rational_polynomial_division() {
    let x = symbol!(x);
    let half = Expression::Number(Number::rational(BigRational::new(
        BigInt::from(1),
        BigInt::from(2),
    )));

    let dividend = Expression::add(vec![
        Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
        Expression::mul(vec![half, Expression::symbol(x.clone())]),
        Expression::integer(1),
    ]);
    let divisor = expr!(x + 1);

    let (quot, _) = polynomial_div(&dividend, &divisor, &x);

    assert!(!quot.is_zero());
}

#[test]
fn test_mixed_integer_rational_gcd() {
    let x = symbol!(x);

    let p1 = expr!((x ^ 2) - 1);
    let half = Expression::Number(Number::rational(BigRational::new(
        BigInt::from(1),
        BigInt::from(2),
    )));
    let p2 = Expression::mul(vec![half, expr!(x - 1)]);

    let gcd = polynomial_gcd(&p1, &p2, &x);

    assert!(!gcd.is_zero());
}

#[test]
fn test_zero_polynomials() {
    let x = symbol!(x);
    let zero = Expression::integer(0);
    let p = expr!(x + 1);

    let gcd = polynomial_gcd(&zero, &p, &x);
    assert_eq!(gcd.to_string(), p.to_string());

    let gcd = polynomial_gcd(&p, &zero, &x);
    assert_eq!(gcd.to_string(), p.to_string());
}

#[test]
fn test_non_polynomial_divisor() {
    let x = symbol!(x);
    let dividend = expr!((x ^ 2) + (2 * x) + 1);
    let divisor = Expression::integer(2);

    let (quot, rem) = polynomial_div(&dividend, &divisor, &x);

    assert!(quot.is_zero());
    assert_eq!(rem, dividend);
}

#[test]
fn test_dispatch_performance() {
    use std::time::Instant;

    let x = symbol!(x);
    let p1 = expr!((x ^ 5) + (2 * (x ^ 4)) + (3 * (x ^ 3)) + (4 * (x ^ 2)) + (5 * x) + 6);
    let p2 = expr!((2 * (x ^ 5)) + (4 * (x ^ 4)) + (6 * (x ^ 3)) + (8 * (x ^ 2)) + (10 * x) + 12);

    let start = Instant::now();
    for _ in 0..1000 {
        let _gcd = polynomial_gcd(&p1, &p2, &x);
    }
    let duration = start.elapsed();

    let ops_per_sec = 1000.0 / duration.as_secs_f64();
    println!("Dispatch GCD Performance: {:.2} ops/sec", ops_per_sec);

    assert!(ops_per_sec > 100.0, "Expected >100 ops/sec");
}

#[test]
fn test_identical_polynomials() {
    let x = symbol!(x);
    let p = expr!((x ^ 2) + (2 * x) + 1);

    let (quot, rem) = polynomial_div(&p, &p, &x);

    assert_eq!(quot, Expression::integer(1));
    assert!(rem.is_zero());
}

#[test]
fn test_high_degree_polynomial() {
    let x = symbol!(x);

    let mut terms = Vec::new();
    for i in 0..=10 {
        terms.push(Expression::mul(vec![
            Expression::integer(i),
            Expression::pow(Expression::symbol(x.clone()), Expression::integer(i)),
        ]));
    }
    let p1 = Expression::add(terms);

    let p2 = expr!(x - 1);

    let gcd = polynomial_gcd(&p1, &p2, &x);

    assert!(!gcd.is_zero());
}
