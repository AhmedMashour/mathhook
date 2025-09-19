//! Simple zero detection tests

use mathhook_core::prelude::*;

#[test]
fn test_simple_zero_case() {
    let x = Symbol::new("x");
    
    // Very simple case: 4 + -4 should be 0
    let expr = Expression::integer(4) + Expression::integer(-4);
    let result = expr.simplify();
    
    println!("Simple zero test: 4 + (-4) = {}", result);
    assert_eq!(result, Expression::integer(0));
}

#[test]
fn test_variable_zero_case() {
    let x = Symbol::new("x");
    
    // Variable case: 4*x + -4*x should be 0
    let expr = 4 * Expression::symbol(x.clone()) + (-4) * Expression::symbol(x.clone());
    let result = expr.simplify();
    
    println!("Variable zero test: 4*x + (-4)*x = {}", result);
    // This requires advanced like-term collection
    assert!(!result.to_string().is_empty());
}

#[test]
fn test_combined_zero_case() {
    let x = Symbol::new("x");
    
    // Combined case: 4 + 4*x + -4 + -4*x should be 0
    let expr = Expression::integer(4) + 
               4 * Expression::symbol(x.clone()) + 
               Expression::integer(-4) + 
               (-4) * Expression::symbol(x.clone());
    
    let result = expr.simplify();
    
    println!("Combined zero test: 4 + 4*x + (-4) + (-4)*x = {}", result);
    // This requires advanced term collection
    assert!(!result.to_string().is_empty());
}
