//! Zero detection step tests

use mathhook::prelude::*;

#[test]
fn test_zero_detection_basic() {
    let zero = Expression::integer(0);
    assert!(zero.is_zero());
}

#[test]
fn test_zero_detection_arithmetic() {
    let expr = Expression::add(vec![
        Expression::integer(5),
        Expression::integer(-5)
    ]);
    
    let result = expr.simplify();
    assert_eq!(result, Expression::integer(0));
}

#[test]
fn test_zero_detection_multiplication() {
    let x = Symbol::new("x");
    
    let expr = Expression::mul(vec![
        Expression::integer(0),
        Expression::symbol(x.clone())
    ]);
    
    let result = expr.simplify();
    assert_eq!(result, Expression::integer(0));
}
