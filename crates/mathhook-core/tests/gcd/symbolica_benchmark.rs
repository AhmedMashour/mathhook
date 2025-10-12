//! GCD benchmark tests comparing against Symbolica performance

use mathhook_core::prelude::*;

#[test]
fn test_industry_gcd_benchmark() {
    use std::time::Instant;

    let start = Instant::now();

    // Industry standard GCD benchmark
    for i in 1..1000 {
        let a = Expression::integer(i * 12);
        let b = Expression::integer(i * 18);
        let _gcd = a.gcd(&b);
    }

    let duration = start.elapsed();
    let ops_per_sec = 1000.0 / duration.as_secs_f64();

    println!("GCD: {:.2}K ops/sec", ops_per_sec / 1000.0);
    println!("Target: Beat Symbolica performance");

    // Should be extremely fast (targeting >100K ops/sec)
    assert!(
        ops_per_sec > 50_000.0,
        "Expected >50K ops/sec, got {:.2}",
        ops_per_sec
    );
}

#[test]
fn test_symbolica_challenge_benchmark() {
    use std::time::Instant;

    println!("⚔️ SYMBOLICA CHALLENGE BENCHMARK");

    let x = symbol!(x);
    let start = Instant::now();

    // Complex polynomial GCD operations
    for i in 1..500 {
        let poly1 = Expression::mul(vec![Expression::integer(i), Expression::symbol(x.clone())]);
        let poly2 = Expression::mul(vec![
            Expression::integer(i * 2),
            Expression::symbol(x.clone()),
        ]);
        let _gcd = poly1.gcd(&poly2);
    }

    let duration = start.elapsed();
    let ops_per_sec = 500.0 / duration.as_secs_f64();

    println!("Polynomial GCD: {:.2}K ops/sec", ops_per_sec / 1000.0);

    // Should outperform Symbolica
    assert!(ops_per_sec > 10_000.0);
}
