//! Step-by-step integration tests

use mathhook::prelude::*;

#[test]
fn test_step_by_step_basic() {
    let x = Symbol::new("x");
    
    let expr = Expression::add(vec![
        Expression::symbol(x.clone()),
        Expression::integer(0)
    ]);
    
    let result = expr.simplify();
    println!("Step-by-step: x + 0 = {}", result);
    
    // Should simplify to x
    assert_eq!(result, Expression::symbol(x));
}

#[test]
fn test_step_by_step_complex() {
    let x = Symbol::new("x");
    
    let expr = Expression::add(vec![
        Expression::integer(2),
        Expression::integer(3),
        Expression::symbol(x.clone())
    ]);
    
    let result = expr.simplify();
    println!("Step-by-step complex: 2 + 3 + x = {}", result);
    
    assert!(!result.is_zero());
}

#[test]
fn test_step_by_step_verification() {
    let x = Symbol::new("x");
    
    let expr = Expression::mul(vec![
        Expression::symbol(x.clone()),
        Expression::integer(1)
    ]);
    
    let result = expr.simplify();
    assert_eq!(result, Expression::symbol(x));
}
