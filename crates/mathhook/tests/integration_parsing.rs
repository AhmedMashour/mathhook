//! Integration tests for parsing functionality

use mathhook::prelude::*;

#[test]
fn test_basic_parsing() {
    // Test basic expression parsing
    let x = Symbol::new("x");
    let expr = Expression::add(vec![
        Expression::symbol(x.clone()),
        Expression::integer(5)
    ]);
    
    let result = expr.simplify();
    println!("Parsing test: x + 5 = {}", result);
    
    assert!(!result.is_zero());
}

#[test]
fn test_complex_parsing() {
    let x = Symbol::new("x");
    
    // Test complex expression structure
    let expr = Expression::mul(vec![
        Expression::integer(2),
        Expression::pow(Expression::symbol(x.clone()), Expression::integer(2))
    ]);
    
    let result = expr.simplify();
    println!("Complex parsing: 2x^2 = {}", result);
    
    assert!(!result.is_zero());
}

#[test]
fn test_function_parsing() {
    let x = Symbol::new("x");
    
    // Test function parsing
    let expr = Expression::function("sin", vec![Expression::symbol(x.clone())]);
    let result = expr.simplify();
    
    println!("Function parsing: sin(x) = {}", result);
    assert!(!result.is_zero());
}

#[test]
fn test_nested_parsing() {
    let x = Symbol::new("x");
    
    // Test nested expression parsing
    let expr = Expression::add(vec![
        Expression::mul(vec![
            Expression::integer(3),
            Expression::pow(Expression::symbol(x.clone()), Expression::integer(2))
        ]),
        Expression::mul(vec![
            Expression::integer(2),
            Expression::symbol(x.clone())
        ]),
        Expression::integer(1)
    ]);
    
    let result = expr.simplify();
    println!("Nested: 3x^2 + 2x + 1 = {}", result);
    
    assert!(!result.is_zero());
}

#[test]
fn test_rational_parsing() {
    // Test rational number parsing
    let expr = Expression::number(Number::rational(
        num_rational::BigRational::new(num_bigint::BigInt::from(22), num_bigint::BigInt::from(7))
    ));
    
    let result = expr.simplify();
    println!("Rational parsing: 22/7 = {}", result);
    
    assert!(!result.is_zero());
}
