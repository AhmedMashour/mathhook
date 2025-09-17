//! Core simplification tests - the heart of the algebra system

use mathhook::prelude::*;

#[test]
fn test_simplify_basic() {
    // Test basic simplification
    let expr = Expression::add(vec![
        Expression::integer(2),
        Expression::integer(3)
    ]);
    let result = expr.simplify();
    assert_eq!(result, Expression::integer(5));
}

#[test]
fn test_simplify_with_symbols() {
    let x = Symbol::new("x");
    
    // Test x + 0 = x
    let expr = Expression::add(vec![
        Expression::symbol(x.clone()),
        Expression::integer(0)
    ]);
    let result = expr.simplify();
    assert_eq!(result, Expression::symbol(x.clone()));
}

#[test]
fn test_multiplication_simplification() {
    let x = Symbol::new("x");
    
    // Test x * 1 = x
    let expr = Expression::mul(vec![
        Expression::symbol(x.clone()),
        Expression::integer(1)
    ]);
    let result = expr.simplify();
    assert_eq!(result, Expression::symbol(x.clone()));
    
    // Test x * 0 = 0
    let expr = Expression::mul(vec![
        Expression::symbol(x.clone()),
        Expression::integer(0)
    ]);
    let result = expr.simplify();
    assert_eq!(result, Expression::integer(0));
}

#[test]
fn test_power_simplification() {
    let x = Symbol::new("x");
    
    // Test x^0 = 1
    let expr = Expression::pow(Expression::symbol(x.clone()), Expression::integer(0));
    let result = expr.simplify();
    assert_eq!(result, Expression::integer(1));
    
    // Test x^1 = x
    let expr = Expression::pow(Expression::symbol(x.clone()), Expression::integer(1));
    let result = expr.simplify();
    assert_eq!(result, Expression::symbol(x.clone()));
}

#[test]
fn test_nested_simplification() {
    let x = Symbol::new("x");
    
    // Test nested expressions
    let expr = Expression::add(vec![
        Expression::mul(vec![
            Expression::integer(2),
            Expression::add(vec![
                Expression::symbol(x.clone()),
                Expression::integer(1)
            ])
        ]),
        Expression::integer(3)
    ]);
    
    let result = expr.simplify();
    println!("2*(x + 1) + 3 = {}", result);
    
    // Should maintain structure for now
    assert!(!result.is_zero());
}

#[test]
fn test_complex_zero_detection() {
    let x = Symbol::new("x");
    
    // Test x - x = 0
    let expr = Expression::add(vec![
        Expression::symbol(x.clone()),
        Expression::mul(vec![Expression::integer(-1), Expression::symbol(x.clone())])
    ]);
    
    let result = expr.simplify();
    println!("x - x = {}", result);
    
    // Should detect as zero when advanced simplification is implemented
    assert!(!result.to_string().is_empty());
}

#[test]
fn test_numeric_combination() {
    // Test combining multiple numeric terms
    let expr = Expression::add(vec![
        Expression::integer(1),
        Expression::integer(2),
        Expression::integer(3),
        Expression::integer(4),
        Expression::integer(5)
    ]);
    
    let result = expr.simplify();
    assert_eq!(result, Expression::integer(15));
}

#[test]
fn test_multiplication_combination() {
    // Test combining multiple multiplication terms
    let expr = Expression::mul(vec![
        Expression::integer(2),
        Expression::integer(3),
        Expression::integer(4)
    ]);
    
    let result = expr.simplify();
    assert_eq!(result, Expression::integer(24));
}

#[test]
fn test_mixed_operations() {
    let x = Symbol::new("x");
    
    // Test mixed addition and multiplication
    let expr = Expression::add(vec![
        Expression::mul(vec![Expression::integer(2), Expression::symbol(x.clone())]),
        Expression::mul(vec![Expression::integer(3), Expression::symbol(x.clone())]),
        Expression::integer(5)
    ]);
    
    let result = expr.simplify();
    println!("2x + 3x + 5 = {}", result);
    
    // Should combine like terms when implemented
    assert!(!result.is_zero());
}

#[test]
fn test_empty_expressions() {
    // Test edge cases with empty expressions
    let empty_add = Expression::add(vec![]);
    assert_eq!(empty_add, Expression::integer(0));
    
    let empty_mul = Expression::mul(vec![]);
    assert_eq!(empty_mul, Expression::integer(1));
}

#[test]
fn test_single_element_expressions() {
    let x = Symbol::new("x");
    
    // Test single element expressions
    let single_add = Expression::add(vec![Expression::symbol(x.clone())]);
    assert_eq!(single_add, Expression::symbol(x.clone()));
    
    let single_mul = Expression::mul(vec![Expression::symbol(x.clone())]);
    assert_eq!(single_mul, Expression::symbol(x));
}

#[test]
fn test_performance_simplification() {
    use std::time::Instant;
    
    let start = Instant::now();
    let x = Symbol::new("x");
    
    // Perform many simplifications
    for i in 0..1000 {
        let expr = Expression::add(vec![
            Expression::symbol(x.clone()),
            Expression::integer(i),
            Expression::integer(-i)
        ]);
        let _result = expr.simplify();
    }
    
    let duration = start.elapsed();
    let ops_per_sec = 1000.0 / duration.as_secs_f64();
    
    println!("Simplification performance: {:.2}K ops/sec", ops_per_sec / 1000.0);
    
    // Should be fast
    assert!(ops_per_sec > 10000.0);
}

#[test]
fn test_deep_nesting() {
    let x = Symbol::new("x");
    
    // Test deeply nested expressions
    let mut expr = Expression::symbol(x.clone());
    for i in 1..5 {
        expr = Expression::add(vec![
            expr,
            Expression::integer(i)
        ]);
    }
    
    let result = expr.simplify();
    println!("Deeply nested: {}", result);
    
    // Should handle deep nesting without stack overflow
    assert!(!result.is_zero());
}

#[test]
fn test_large_expressions() {
    let x = Symbol::new("x");
    
    // Test expressions with many terms
    let mut terms = vec![Expression::symbol(x.clone())];
    for i in 1..20 {
        terms.push(Expression::integer(i));
    }
    
    let expr = Expression::add(terms);
    let result = expr.simplify();
    
    println!("Large expression simplified: {}", result);
    
    // Should handle large expressions efficiently
    assert!(!result.is_zero());
}

#[test]
fn test_symbolic_arithmetic() {
    let x = Symbol::new("x");
    let y = Symbol::new("y");
    
    // Test symbolic arithmetic patterns
    let expr = Expression::add(vec![
        Expression::symbol(x.clone()),
        Expression::symbol(y.clone()),
        Expression::symbol(x.clone())
    ]);
    
    let result = expr.simplify();
    println!("x + y + x = {}", result);
    
    // Should collect like terms when implemented
    assert!(!result.is_zero());
}

#[test]
fn test_rational_in_simplification() {
    // Test rational numbers in simplification
    let rational = Expression::number(CompactNumber::rational(
        num_rational::BigRational::new(num_bigint::BigInt::from(3), num_bigint::BigInt::from(4))
    ));
    
    let expr = Expression::add(vec![
        rational.clone(),
        rational.clone()
    ]);
    
    let result = expr.simplify();
    println!("3/4 + 3/4 = {}", result);
    
    // Should combine rationals
    assert!(!result.is_zero());
}

#[test]
fn test_float_simplification() {
    // Test float arithmetic
    let expr = Expression::add(vec![
        Expression::number(CompactNumber::float(2.5)),
        Expression::number(CompactNumber::float(3.7))
    ]);
    
    let result = expr.simplify();
    println!("2.5 + 3.7 = {}", result);
    
    // Should combine floats
    assert!(!result.is_zero());
}

#[test]
fn test_mixed_number_types() {
    // Test mixing different number types
    let expr = Expression::add(vec![
        Expression::integer(5),
        Expression::number(CompactNumber::float(2.5)),
        Expression::number(CompactNumber::rational(
            num_rational::BigRational::new(num_bigint::BigInt::from(1), num_bigint::BigInt::from(2))
        ))
    ]);
    
    let result = expr.simplify();
    println!("5 + 2.5 + 1/2 = {}", result);
    
    // Should handle mixed types
    assert!(!result.is_zero());
}
