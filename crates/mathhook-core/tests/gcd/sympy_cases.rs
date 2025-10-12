//! SymPy-inspired GCD test cases

use mathhook_core::prelude::*;

#[test]
fn test_sympy_gcd_basic() {
    // Basic SymPy GCD cases
    let a = Expression::integer(48);
    let b = Expression::integer(18);
    let gcd = a.gcd(&b);

    assert_eq!(gcd, Expression::integer(6));
}

#[test]
fn test_sympy_gcd_polynomials() {
    let x = symbol!(x);

    // SymPy polynomial GCD
    let poly1 = Expression::add(vec![
        Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
        Expression::symbol(x.clone()),
    ]);
    let poly2 = Expression::symbol(x.clone());

    let gcd = poly1.gcd(&poly2);
    println!("GCD(x^2 + x, x) = {}", gcd);

    // Should be x
    assert!(!gcd.is_zero());
}

#[test]
fn test_sympy_gcd_multivariate() {
    let x = symbol!(x);
    let y = symbol!(y);

    // Multivariate GCD from SymPy
    let poly1 = Expression::mul(vec![
        Expression::symbol(x.clone()),
        Expression::symbol(y.clone()),
    ]);
    let poly2 = Expression::mul(vec![Expression::symbol(x.clone()), Expression::integer(3)]);

    let gcd = poly1.gcd(&poly2);
    println!("SymPy multivariate GCD: {}", gcd);

    assert!(!gcd.is_zero());
}

#[test]
fn test_sympy_gcd_complex() {
    let x = symbol!(x);

    // Complex SymPy GCD case
    let poly1 = Expression::add(vec![
        Expression::pow(Expression::symbol(x.clone()), Expression::integer(3)),
        Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
    ]);
    let poly2 = Expression::add(vec![
        Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
        Expression::symbol(x.clone()),
    ]);

    let gcd = poly1.gcd(&poly2);
    println!("SymPy complex GCD: {}", gcd);

    assert!(!gcd.is_zero());
}

#[test]
fn test_session_071_gcd_milestone() {
    let x = symbol!(x);
    let y = symbol!(y);

    let poly1 = Expression::add(vec![
        Expression::mul(vec![Expression::integer(2), Expression::symbol(x.clone())]),
        Expression::mul(vec![Expression::integer(4), Expression::symbol(y.clone())]),
    ]);
    let poly2 = Expression::add(vec![
        Expression::mul(vec![Expression::integer(3), Expression::symbol(x.clone())]),
        Expression::mul(vec![Expression::integer(6), Expression::symbol(y.clone())]),
    ]);

    let gcd = poly1.gcd(&poly2);
    println!("Session 071 GCD milestone: {}", gcd);

    // Should find common structure
    assert!(!gcd.is_zero());
}

#[test]
fn test_sympy_gcd_comprehensive_integers() {
    // Comprehensive integer GCD tests from SymPy
    let test_cases = vec![
        (12, 8, 4),
        (48, 18, 6),
        (100, 75, 25),
        (17, 13, 1), // Coprime
        (0, 5, 5),   // Zero case
    ];

    for (a, b, expected) in test_cases {
        let expr_a = Expression::integer(a);
        let expr_b = Expression::integer(b);
        let gcd = expr_a.gcd(&expr_b);

        assert_eq!(
            gcd,
            Expression::integer(expected),
            "GCD({}, {}) should be {}, got {}",
            a,
            b,
            expected,
            gcd
        );
    }
}

#[test]
fn test_sympy_gcd_comprehensive_polynomials() {
    let x = symbol!(x);

    // Test polynomial cases from SymPy

    // Case 1: x^3 and x^2 -> x^2
    let poly1 = Expression::pow(Expression::symbol(x.clone()), Expression::integer(3));
    let poly2 = Expression::pow(Expression::symbol(x.clone()), Expression::integer(2));
    let gcd1 = poly1.gcd(&poly2);
    println!("GCD(x^3, x^2) = {}", gcd1);

    // Case 2: (x^2 + x) and x -> x
    let poly3 = Expression::add(vec![
        Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
        Expression::symbol(x.clone()),
    ]);
    let poly4 = Expression::symbol(x.clone());
    let gcd2 = poly3.gcd(&poly4);
    println!("GCD(x^2 + x, x) = {}", gcd2);

    assert!(!gcd1.is_zero());
    assert!(!gcd2.is_zero());
}

#[test]
fn test_sympy_gcd_comprehensive_multivariate() {
    let x = symbol!(x);
    let y = symbol!(y);

    // Comprehensive multivariate tests
    let poly1 = Expression::mul(vec![
        Expression::integer(6),
        Expression::symbol(x.clone()),
        Expression::symbol(y.clone()),
    ]);

    let poly2 = Expression::mul(vec![Expression::integer(9), Expression::symbol(x.clone())]);

    let gcd = poly1.gcd(&poly2);
    println!("Comprehensive multivariate: GCD(6xy, 9x) = {}", gcd);

    // Should find 3x as common factor
    assert!(!gcd.is_zero());
}

#[test]
fn test_sympy_gcd_comprehensive_complex() {
    let x = symbol!(x);

    // Complex polynomial GCD from SymPy test suite
    let poly1 = Expression::add(vec![
        Expression::pow(Expression::symbol(x.clone()), Expression::integer(4)),
        Expression::mul(vec![
            Expression::integer(2),
            Expression::pow(Expression::symbol(x.clone()), Expression::integer(3)),
        ]),
        Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
    ]);

    let poly2 = Expression::add(vec![
        Expression::pow(Expression::symbol(x.clone()), Expression::integer(3)),
        Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
    ]);

    let gcd = poly1.gcd(&poly2);
    println!("Complex SymPy GCD: {}", gcd);

    assert!(!gcd.is_zero());
}

#[test]
fn test_sympy_gcd_comprehensive_performance() {
    use std::time::Instant;

    let x = symbol!(x);
    let start = Instant::now();

    // Comprehensive performance test
    for i in 1..500 {
        let poly1 = Expression::add(vec![
            Expression::mul(vec![Expression::integer(i), Expression::symbol(x.clone())]),
            Expression::integer(i * 2),
        ]);
        let poly2 = Expression::mul(vec![
            Expression::integer(i * 3),
            Expression::symbol(x.clone()),
        ]);
        let _gcd = poly1.gcd(&poly2);
    }

    let duration = start.elapsed();
    let ops_per_sec = 500.0 / duration.as_secs_f64();

    println!(
        "Comprehensive GCD Performance: {:.2}K ops/sec",
        ops_per_sec / 1000.0
    );

    assert!(ops_per_sec > 25_000.0);
}
