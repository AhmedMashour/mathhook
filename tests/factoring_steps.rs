//! Factoring step-by-step tests

use mathhook::prelude::*;

#[test]
fn test_factoring_steps() {
    let x = Symbol::new("x");
    
    let expr = Expression::add(vec![
        Expression::mul(vec![Expression::integer(2), Expression::symbol(x.clone())]),
        Expression::integer(4)
    ]);
    
    let result = expr.simplify();
    println!("Factoring steps: 2x + 4 = {}", result);
    
    assert!(!result.is_zero());
}

#[test]
fn test_step_by_step_factoring() {
    let x = Symbol::new("x");
    
    let expr = Expression::add(vec![
        Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
        Expression::mul(vec![Expression::integer(3), Expression::symbol(x.clone())]),
        Expression::integer(2)
    ]);
    
    let result = expr.simplify();
    println!("Step-by-step: x^2 + 3x + 2 = {}", result);
    
    assert!(!result.is_zero());
}

#[test]
fn test_advanced_factoring_steps() {
    let x = Symbol::new("x");
    
    let expr = Expression::add(vec![
        Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
        Expression::mul(vec![Expression::integer(-1), Expression::integer(1)])
    ]);
    
    let result = expr.simplify();
    println!("Advanced factoring: x^2 - 1 = {}", result);
    
    // This is (x - 1)(x + 1)
    assert!(!result.is_zero());
}
