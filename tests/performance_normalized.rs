//! Performance normalization tests

use mathhook::prelude::*;

#[test]
fn test_normalized_performance() {
    use std::time::Instant;
    
    println!("🚀 PERFORMANCE NORMALIZATION TEST");
    
    let start = Instant::now();
    let x = Expression::symbol(Symbol::new("x"));
    
    // Test normalized performance
    for i in 0..10_000 {
        let expr = Expression::add(vec![
            x.clone(),
            Expression::integer(i)
        ]);
        let _result = expr.simplify();
    }
    
    let duration = start.elapsed();
    let ops_per_sec = 10_000.0 / duration.as_secs_f64();
    
    println!("Normalized: {:.1}ns = {:.2}M ops/sec", 
             duration.as_nanos() as f64 / 10_000.0, ops_per_sec / 1_000_000.0);
    
    // Should exceed 1M ops/sec
    assert!(ops_per_sec >= 1_000_000.0, "Normalized should exceed 1M ops/sec");
}

#[test]
fn test_memory_normalization() {
    println!("🧠 MEMORY NORMALIZATION TEST");
    
    // Verify memory optimization is the default
    let expr = Expression::integer(42);
    match expr {
        Expression::Number(Number::SmallInt(n)) => {
            assert_eq!(n, 42);
            println!("✅ Small integers optimized by default");
        },
        _ => panic!("Expected small integer optimization"),
    }
}
