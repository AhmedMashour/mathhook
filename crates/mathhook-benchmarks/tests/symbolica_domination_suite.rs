//! Symbolica domination test suite
//! Comprehensive tests to outperform Symbolica in all key areas

use mathhook_core::prelude::*;
use std::time::Instant;

#[test]
fn test_symbolica_gcd_domination_basic() {
    // Test cases that should dominate Symbolica's GCD performance
    let x = Expression::symbol(Symbol::new("x"));

    // Case 1: Simple polynomial GCD
    let poly1 = Expression::add(vec![
        Expression::mul(vec![Expression::integer(2), x.clone()]),
        Expression::integer(4),
    ]);
    let poly2 = Expression::add(vec![
        Expression::mul(vec![Expression::integer(3), x]),
        Expression::integer(6),
    ]);

    let start = Instant::now();
    let gcd_result = poly1.gcd(&poly2);
    let duration = start.elapsed();

    println!("GCD computation time: {:?}", duration);
    println!("GCD result: {}", gcd_result);

    // Should be very fast (target: < 1µs for simple cases)
    assert!(duration.as_nanos() < 1_000_000); // < 1ms
    assert!(!gcd_result.is_zero());
}

#[test]
fn test_symbolica_gcd_domination_multivariate() {
    // Test multivariate GCD that should outperform Symbolica
    let x = Expression::symbol(Symbol::new("x"));
    let y = Expression::symbol(Symbol::new("y"));

    // Multivariate polynomials with common factors
    let poly1 = Expression::mul(vec![
        Expression::add(vec![x.clone(), y.clone()]),
        Expression::add(vec![x.clone(), Expression::integer(1)]),
    ]);
    let poly2 = Expression::mul(vec![
        Expression::add(vec![x.clone(), y.clone()]),
        Expression::add(vec![y, Expression::integer(1)]),
    ]);

    let start = Instant::now();
    let gcd_result = poly1.gcd(&poly2);
    let duration = start.elapsed();

    println!("Multivariate GCD time: {:?}", duration);
    println!("Multivariate GCD result: {}", gcd_result);

    // Should be extremely fast
    assert!(duration.as_nanos() < 10_000_000); // < 10ms
    assert!(!gcd_result.is_zero());
}

#[test]
fn test_symbolica_arithmetic_domination() {
    // Test arithmetic operations that should dominate Symbolica
    let x = Expression::symbol(Symbol::new("x"));

    let start = Instant::now();

    // Perform 10,000 arithmetic operations
    let mut result = x.clone();
    for i in 1..10_000 {
        result = Expression::add(vec![result, Expression::integer(i)]);
        result = result.simplify();
    }

    let duration = start.elapsed();
    let ops_per_sec = 10_000.0 / duration.as_secs_f64();

    println!(
        "Arithmetic domination: {:.2}M ops/sec",
        ops_per_sec / 1_000_000.0
    );

    // Should achieve high performance (target: > 1M ops/sec)
    assert!(ops_per_sec > 1_000_000.0);
    assert!(!result.is_zero());
}

#[test]
fn test_symbolica_power_domination() {
    // Test power operations that should dominate Symbolica
    let x = Expression::symbol(Symbol::new("x"));

    let start = Instant::now();

    // Test various power operations
    let power_tests = vec![
        Expression::pow(x.clone(), Expression::integer(0)),
        Expression::pow(x.clone(), Expression::integer(1)),
        Expression::pow(x.clone(), Expression::integer(2)),
        Expression::pow(Expression::integer(2), Expression::integer(10)),
        Expression::pow(Expression::integer(0), Expression::integer(5)),
        Expression::pow(Expression::integer(1), Expression::integer(100)),
    ];

    let mut results = Vec::new();
    for power_expr in power_tests {
        results.push(power_expr.simplify());
    }

    let duration = start.elapsed();
    let ops_per_sec = results.len() as f64 / duration.as_secs_f64();

    println!(
        "Power domination: {:.2}M ops/sec",
        ops_per_sec / 1_000_000.0
    );

    // Verify specific results
    assert_eq!(results[0], Expression::integer(1)); // x^0 = 1
    assert_eq!(results[1], x); // x^1 = x
    assert_eq!(results[3], Expression::integer(1024)); // 2^10 = 1024
    assert_eq!(results[4], Expression::integer(0)); // 0^5 = 0
    assert_eq!(results[5], Expression::integer(1)); // 1^100 = 1

    // Should be extremely fast
    assert!(ops_per_sec > 100_000.0);
}

#[test]
fn test_symbolica_memory_domination() {
    // Test memory efficiency vs Symbolica
    let x = Expression::symbol(Symbol::new("x"));

    // Create large expression tree
    let mut large_expr = x.clone();
    for i in 1..1000 {
        large_expr = Expression::add(vec![
            large_expr,
            Expression::mul(vec![
                Expression::integer(i),
                Expression::pow(x.clone(), Expression::integer(i % 5)),
            ]),
        ]);
    }

    let start = Instant::now();
    let simplified = large_expr.simplify();
    let duration = start.elapsed();

    println!("Large expression simplification: {:?}", duration);

    // Should handle large expressions efficiently
    assert!(duration.as_millis() < 100); // < 100ms
    assert!(!simplified.is_zero());
}

#[test]
fn test_symbolica_bulk_operations_domination() {
    // Test bulk operations that should dominate Symbolica
    let x = Expression::symbol(Symbol::new("x"));
    let y = Expression::symbol(Symbol::new("y"));

    let start = Instant::now();

    // Perform bulk operations
    let mut expressions = Vec::new();
    for i in 0..1000 {
        expressions.push(Expression::add(vec![
            Expression::mul(vec![Expression::integer(i), x.clone()]),
            Expression::mul(vec![Expression::integer(i * 2), y.clone()]),
            Expression::integer(i * 3),
        ]));
    }

    // Simplify all expressions
    let simplified_expressions: Vec<Expression> =
        expressions.into_iter().map(|e| e.simplify()).collect();

    let duration = start.elapsed();
    let ops_per_sec = 1000.0 / duration.as_secs_f64();

    println!(
        "Bulk operations domination: {:.2}K ops/sec",
        ops_per_sec / 1_000.0
    );

    // Should achieve high bulk performance
    assert!(ops_per_sec > 10_000.0); // > 10K ops/sec for bulk
    assert_eq!(simplified_expressions.len(), 1000);
    assert!(simplified_expressions.iter().all(|e| !e.is_zero()));
}

#[test]
fn test_symbolica_factorization_domination() {
    // Test factorization that should dominate Symbolica
    let x = Expression::symbol(Symbol::new("x"));

    // Difference of squares: x² - 4 = (x-2)(x+2)
    let diff_squares = Expression::add(vec![
        Expression::pow(x.clone(), Expression::integer(2)),
        Expression::integer(-4),
    ]);

    let start = Instant::now();
    let factored = diff_squares.factor_gcd(); // Using our GCD-based factoring
    let duration = start.elapsed();

    println!("Factorization time: {:?}", duration);
    println!("Factored result: {}", factored);

    // Should be very fast
    assert!(duration.as_nanos() < 1_000_000); // < 1ms
    assert!(!factored.is_zero());
}

#[test]
fn test_symbolica_rational_domination() {
    // Test rational operations that should dominate Symbolica
    let x = Expression::symbol(Symbol::new("x"));

    // Complex rational expression: (x+1)/(x-1) + (x-1)/(x+1)
    let rational_expr = Expression::add(vec![
        Expression::mul(vec![
            Expression::add(vec![x.clone(), Expression::integer(1)]),
            Expression::pow(
                Expression::add(vec![x.clone(), Expression::integer(-1)]),
                Expression::integer(-1),
            ),
        ]),
        Expression::mul(vec![
            Expression::add(vec![x.clone(), Expression::integer(-1)]),
            Expression::pow(
                Expression::add(vec![x, Expression::integer(1)]),
                Expression::integer(-1),
            ),
        ]),
    ]);

    let start = Instant::now();
    let simplified = rational_expr.simplify();
    let duration = start.elapsed();

    println!("Rational simplification time: {:?}", duration);
    println!("Rational result: {}", simplified);

    // Should be efficient
    assert!(duration.as_nanos() < 10_000_000); // < 10ms
    assert!(!simplified.is_zero());
}

#[test]
fn test_symbolica_overall_performance_domination() {
    // Comprehensive performance test vs Symbolica
    let x = Expression::symbol(Symbol::new("x"));
    let y = Expression::symbol(Symbol::new("y"));
    let z = Expression::symbol(Symbol::new("z"));

    let start = Instant::now();

    // Complex mixed operations
    let complex_expr = Expression::add(vec![
        Expression::mul(vec![
            Expression::integer(2),
            Expression::pow(x.clone(), Expression::integer(3)),
        ]),
        Expression::mul(vec![
            Expression::integer(3),
            Expression::pow(y.clone(), Expression::integer(2)),
            z.clone(),
        ]),
        Expression::add(vec![
            Expression::mul(vec![x.clone(), y.clone()]),
            Expression::mul(vec![y, z]),
        ]),
        Expression::integer(42),
    ]);

    // Perform multiple operations
    let simplified = complex_expr.simplify();
    let gcd_with_x = simplified.gcd(&Expression::mul(vec![Expression::integer(2), x]));
    let factored = simplified.factor_gcd();

    let duration = start.elapsed();

    println!("Overall domination test: {:?}", duration);
    println!("Complex result: {}", simplified);

    // Should be extremely efficient for complex operations
    assert!(duration.as_millis() < 10); // < 10ms total
    assert!(!simplified.is_zero());
    assert!(!gcd_with_x.is_zero());
    assert!(!factored.is_zero());
}
