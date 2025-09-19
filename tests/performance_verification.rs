/// Verify performance after memory optimization
use mathhook::core::{Expression, Symbol};
use std::time::Instant;

#[test]
fn test_hot_path_performance() {
    println!("Testing hot path performance after memory optimization...");

    let start = Instant::now();
    let mut _result = Expression::integer(0);

    // Test simple operations (hot path)
    for i in 0..10_000 {
        // Reduced to avoid stack overflow
        let expr = Expression::add(vec![Expression::integer(i), Expression::symbol("x")]);
        _result = expr.simplify(); // Use simplify instead of nested adds
    }

    let duration = start.elapsed();
    let ops_per_sec = 10_000.0 / duration.as_secs_f64();

    println!(
        "Hot path performance: {:.2}M ops/sec",
        ops_per_sec / 1_000_000.0
    );

    // Should maintain good performance (target: >1M ops/sec)
    assert!(
        ops_per_sec > 1_000_000.0,
        "Performance regression: {:.2} ops/sec",
        ops_per_sec
    );

    if ops_per_sec > 3_000_000.0 {
        println!("✅ EXCELLENT: Performance >3M ops/sec (Symbolica-beating)");
    } else if ops_per_sec > 1_000_000.0 {
        println!("✅ GOOD: Performance >1M ops/sec (acceptable)");
    }
}

#[test]
fn test_memory_vs_speed_tradeoff() {
    println!("Analyzing memory vs speed tradeoff...");

    // Memory efficiency test
    let expr_size = std::mem::size_of::<Expression>();
    println!("Expression size: {} bytes", expr_size);

    // Speed test for different expression types
    let start = Instant::now();

    // Hot path operations (should be fast)
    for _i in 0..1_000 {
        let _expr =
            Expression::add(vec![Expression::integer(42), Expression::symbol("x")]).simplify();
    }

    let hot_path_duration = start.elapsed();
    let hot_path_ops = 1_000.0 / hot_path_duration.as_secs_f64();

    println!("Hot path: {:.2}M ops/sec", hot_path_ops / 1_000_000.0);

    // Cold path operations (calculus - may be slower due to boxing)
    let start = Instant::now();

    for _i in 0..1_000 {
        let expr = Expression::pow(Expression::symbol("x"), Expression::integer(2));
        let _derivative = Expression::derivative(expr, Symbol::new("x"), 1);
    }

    let cold_path_duration = start.elapsed();
    let cold_path_ops = 1_000.0 / cold_path_duration.as_secs_f64();

    println!("Cold path: {:.2}M ops/sec", cold_path_ops / 1_000_000.0);

    // Analysis
    let efficiency_ratio = hot_path_ops / cold_path_ops;
    println!("Hot/Cold ratio: {:.2}x", efficiency_ratio);

    // Both should be reasonable
    assert!(hot_path_ops > 500_000.0, "Hot path too slow");
    assert!(cold_path_ops > 100_000.0, "Cold path too slow");

    println!("✅ Memory optimization maintains good performance balance");
}
