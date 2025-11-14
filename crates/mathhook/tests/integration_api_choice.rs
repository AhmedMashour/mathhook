//! API choice and interface tests

use mathhook::prelude::*;

#[test]
fn test_expression_api() {
    let x = symbol!(x);

    // Test different ways to create expressions
    let expr1 = Expression::symbol(x.clone());
    let expr2 = Expression::integer(5);
    let expr3 = Expression::add(vec![expr1, expr2]);

    let result = expr3.simplify();
    println!("API test: {}", result);

    assert!(!result.is_zero());
}

#[test]
fn test_operator_overloading() {
    let x = symbol!(x);
    let y = symbol!(y);

    // Test operator overloading
    let sum = Expression::symbol(x.clone()) + Expression::symbol(y.clone());
    let product = Expression::symbol(x.clone()) * Expression::integer(2);
    let power = Expression::pow(Expression::symbol(x.clone()), Expression::integer(2));

    println!("Sum: {}, Product: {}, Power: {}", sum, product, power);

    assert!(!sum.is_zero());
    assert!(!product.is_zero());
    assert!(!power.is_zero());
}

#[test]
fn test_convenience_methods() {
    let x = symbol!(x);

    // Test convenience methods
    let zero = Expression::integer(0);
    let one = Expression::integer(1);
    let symbol = Expression::symbol(x.clone());

    assert!(zero.is_zero());
    assert!(one.is_one());
    assert!(!symbol.is_zero());
    assert!(!symbol.is_one());
}

#[test]
fn test_api_consistency() {
    let x = symbol!(x);

    // Test that different API approaches give same results
    let method1 = Expression::add(vec![Expression::symbol(x.clone()), Expression::integer(1)]);

    let method2 = Expression::symbol(x.clone()) + Expression::integer(1);

    // Both should create equivalent expressions
    println!("Method 1: {}, Method 2: {}", method1, method2);
    assert_eq!(method1, method2);
}
