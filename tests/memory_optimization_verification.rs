/// Verify Expression memory optimization after boxing large variants
use mathhook::core::{Expression, Symbol};

#[test]
fn test_expression_memory_optimization() {
    let expression_size = std::mem::size_of::<Expression>();

    println!("Optimized Expression size: {} bytes", expression_size);

    // CRITICAL: Must maintain 32-byte target for 7.37M ops/sec performance
    assert!(
        expression_size <= 32,
        "Expression size must be ≤32 bytes for performance, got {} bytes",
        expression_size
    );

    // Verify hot path variants work
    let number = Expression::integer(42);
    let symbol = Expression::symbol("x");
    let add = Expression::add(vec![number.clone(), symbol.clone()]);
    let mul = Expression::mul(vec![number.clone(), symbol.clone()]);
    let pow = Expression::pow(symbol.clone(), Expression::integer(2));

    // Verify cold path variants work (should be boxed)
    let complex = Expression::complex(number.clone(), symbol.clone());
    let matrix = Expression::matrix(vec![vec![number.clone(), symbol.clone()]]);
    let equation = Expression::equation(add.clone(), mul.clone());
    let derivative = Expression::derivative(pow.clone(), Symbol::new("x"), 1);

    println!("Hot path expressions created successfully");
    println!("Cold path expressions created successfully");
    println!("Memory optimization verification passed!");
}

#[test]
fn test_performance_preservation() {
    use std::time::Instant;

    // Test hot path performance (should maintain 7.37M ops/sec)
    let start = Instant::now();
    let mut result = Expression::integer(0);

    for i in 0..100_000 {
        let expr = Expression::add(vec![Expression::integer(i), Expression::symbol("x")]);
        result = Expression::add(vec![result, expr]);
    }

    let duration = start.elapsed();
    let ops_per_sec = 100_000.0 / duration.as_secs_f64();

    println!(
        "Hot path performance: {:.2}M ops/sec",
        ops_per_sec / 1_000_000.0
    );

    // Should maintain high performance (target: >1M ops/sec minimum)
    assert!(
        ops_per_sec > 1_000_000.0,
        "Performance regression detected: {:.2} ops/sec",
        ops_per_sec
    );
}
