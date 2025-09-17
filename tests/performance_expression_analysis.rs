//! Expression analysis performance tests

use mathhook::prelude::*;

#[test]
fn test_expression_analysis() {
    use std::time::Instant;
    
    println!("📊 EXPRESSION ANALYSIS PERFORMANCE");
    
    let start = Instant::now();
    let x = Symbol::new("x");
    
    // Analyze many expressions
    for i in 0..10_000 {
        let expr = Expression::add(vec![
            Expression::symbol(x.clone()),
            Expression::integer(i)
        ]);
        
        // Analyze properties
        let _is_zero = expr.is_zero();
        let _is_one = expr.is_one();
        let _display = format!("{}", expr);
    }
    
    let duration = start.elapsed();
    let ops_per_sec = 10_000.0 / duration.as_secs_f64();
    
    println!("Analysis performance: {:.2}M ops/sec", ops_per_sec / 1_000_000.0);
    
    assert!(ops_per_sec > 1_000_000.0);
}
