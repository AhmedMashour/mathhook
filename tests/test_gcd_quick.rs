//! Quick GCD functionality tests

use mathhook::prelude::*;

#[test]
fn test_gcd_quick_integers() {
    // Quick test for integer GCD
    let a = Expression::integer(24);
    let b = Expression::integer(36);
    let gcd = a.gcd(&b);

    assert_eq!(gcd, Expression::integer(12));
}

#[test]
fn test_gcd_quick_symbols() {
    let x = Symbol::new("x");

    // Quick test for symbolic GCD
    let expr = Expression::symbol(x.clone());
    let gcd = expr.gcd(&expr);

    assert_eq!(gcd, Expression::symbol(x));
}

#[test]
fn test_gcd_quick_performance() {
    use std::time::Instant;

    let start = Instant::now();

    // Quick performance test
    for i in 1..100 {
        let a = Expression::integer(i * 4);
        let b = Expression::integer(i * 6);
        let _gcd = a.gcd(&b);
    }

    let duration = start.elapsed();
    let ops_per_sec = 100.0 / duration.as_secs_f64();

    println!("Quick GCD: {:.2}K ops/sec", ops_per_sec / 1000.0);

    // Should be fast
    assert!(ops_per_sec > 10_000.0);
}
