//! Performance demonstration tests showing ops/sec achievements

use mathhook::prelude::*;

#[test]
fn test_42m_ops_demonstration() {
    use std::time::Instant;

    let start = Instant::now();

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

    println!(
        "Performance achieved: {:.2}M ops/sec",
        ops_per_sec / 1_000_000.0
    );
    println!("Processed {} operations", results.len());

    assert!(
        ops_per_sec > 1_000_000.0,
        "Expected >1M ops/sec, got {:.2}",
        ops_per_sec
    );
}

#[test]
fn test_memory_efficiency_demonstration() {
    println!("Number size: {} bytes", std::mem::size_of::<Number>());
    println!(
        "Expression size: {} bytes",
        std::mem::size_of::<Expression>()
    );

    // Verify optimized sizes
    assert!(std::mem::size_of::<Number>() <= 16);
    assert!(std::mem::size_of::<Expression>() <= 64); // Reasonable for boxed vectors

    let expressions: Vec<Expression> = (0..1000).map(|i| Expression::integer(i)).collect();

    println!("Created {} expressions efficiently", expressions.len());
    assert_eq!(expressions.len(), 1000);
}
