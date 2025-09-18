//! Memory optimization performance tests

use mathhook::prelude::*;

#[test]
fn test_memory_usage() {
    println!("Memory optimization test");
    println!("Number size: {} bytes", std::mem::size_of::<Number>());
    println!("Expression size: {} bytes", std::mem::size_of::<Expression>());
    
    // Verify optimized sizes
    assert!(std::mem::size_of::<Number>() <= 16);
    assert!(std::mem::size_of::<Expression>() <= 64);
}

#[test]
fn test_arena_memory_efficiency() {
    // Test memory efficiency with many expressions
    let expressions: Vec<Expression> = (0..1000)
        .map(|i| Expression::integer(i))
        .collect();
    
    println!("Created {} expressions", expressions.len());
    assert_eq!(expressions.len(), 1000);
}

#[test]
#[ignore] // Ignore by default as it's memory intensive
fn test_large_scale_memory() {
    // Test large scale memory usage
    let _large_expressions: Vec<Expression> = (0..100_000)
        .map(|i| Expression::add(vec![
            Expression::integer(i),
            Expression::symbol(Symbol::new("x"))
        ]))
        .collect();
    
    println!("Large scale memory test completed");
}
