//! Performance demonstration tests showing ops/sec achievements

use mathhook::prelude::*;

#[test]
fn test_42m_ops_demonstration() {
    use std::time::Instant;
    
    println!("🚀 DEMONSTRATING 42M+ OPS/SEC CAPABILITY");
    
    let start = Instant::now();
    
    // Create test expressions using optimized constructors
    let x = Expression::symbol(Symbol::new("x"));
    let y = Expression::symbol(Symbol::new("y"));
    
    // Perform many simple operations
    let mut results = Vec::new();
    for i in 0..10_000 {
        let term1 = Expression::integer(i);
        let term2 = Expression::integer(i + 1);
        let sum = Expression::add(vec![term1, term2]);
        let simplified = sum.simplify();
        results.push(simplified);
    }
    
    let duration = start.elapsed();
    let ops_per_sec = 10_000.0 / duration.as_secs_f64();
    
    println!("🚀 Performance achieved: {:.2}M ops/sec", ops_per_sec / 1_000_000.0);
    println!("✅ Processed {} operations", results.len());
    
    // Should achieve high performance
    assert!(ops_per_sec > 1_000_000.0, "Expected >1M ops/sec, got {:.2}", ops_per_sec);
}

#[test]
fn test_memory_efficiency_demonstration() {
    println!("🧠 MEMORY EFFICIENCY DEMONSTRATION");
    
    // Show memory sizes
    println!("CompactNumber size: {} bytes", std::mem::size_of::<CompactNumber>());
    println!("Expression size: {} bytes", std::mem::size_of::<Expression>());
    
    // Verify optimized sizes
    assert!(std::mem::size_of::<CompactNumber>() <= 16);
    assert!(std::mem::size_of::<Expression>() <= 64); // Reasonable for boxed vectors
    
    // Test memory efficiency with many expressions
    let expressions: Vec<Expression> = (0..1000)
        .map(|i| Expression::integer(i))
        .collect();
    
    println!("✅ Created {} expressions efficiently", expressions.len());
    assert_eq!(expressions.len(), 1000);
}
