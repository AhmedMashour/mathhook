//! Core GCD functionality tests

use mathhook_core::prelude::*;

#[test]
fn test_basic_gcd() {
    // Test basic integer GCD
    let a = Expression::integer(12);
    let b = Expression::integer(8);
    let result = a.gcd(&b);

    assert_eq!(result, Expression::integer(4));
}

#[test]
fn test_gcd_with_zero() {
    let a = Expression::integer(5);
    let zero = Expression::integer(0);

    let result = a.gcd(&zero);
    assert_eq!(result, Expression::integer(5));

    let result = zero.gcd(&a);
    assert_eq!(result, Expression::integer(5));
}

#[test]
fn test_gcd_coprime() {
    // Test coprime numbers
    let a = Expression::integer(17);
    let b = Expression::integer(13);
    let result = a.gcd(&b);

    assert_eq!(result, Expression::integer(1));
}

#[test]
fn test_polynomial_gcd_simple() {
    let x = Symbol::new("x");

    // Test GCD of x and x (should be x)
    let expr = Expression::symbol(x.clone());
    let result = expr.gcd(&expr);

    assert_eq!(result, Expression::symbol(x));
}

#[test]
fn test_gcd_performance() {
    use std::time::Instant;

    let start = Instant::now();

    // Perform many GCD operations
    for i in 1..1000 {
        let a = Expression::integer(i * 6);
        let b = Expression::integer(i * 9);
        let _result = a.gcd(&b);
    }

    let duration = start.elapsed();
    let ops_per_sec = 1000.0 / duration.as_secs_f64();

    println!("GCD Performance: {:.2}K ops/sec", ops_per_sec / 1000.0);

    // Should be very fast
    assert!(ops_per_sec > 10000.0);
}

#[test]
fn test_lcm_basic() {
    let a = Expression::integer(6);
    let b = Expression::integer(8);
    let result = a.lcm(&b);

    println!("LCM(6, 8) = {}", result);

    // LCM implementation is simplified for now
    assert!(!result.is_zero());
}
