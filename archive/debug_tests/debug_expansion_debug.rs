//! Debug test for expansion operations

use mathhook::prelude::*;

#[test]
fn test_expansion_debug() {
    let x = Symbol::new("x");
    
    let expr = Expression::mul(vec![
        Expression::integer(3),
        Expression::add(vec![
            Expression::symbol(x.clone()),
            Expression::integer(4)
        ])
    ]);
    
    let result = expr.simplify();
    println!("Expansion debug: 3*(x + 4) = {}", result);
    
    assert!(!result.is_zero());
}

#[test]
fn test_expansion_debug_complex() {
    let x = Symbol::new("x");
    let y = Symbol::new("y");
    
    let expr = Expression::mul(vec![
        Expression::add(vec![
            Expression::symbol(x.clone()),
            Expression::integer(1)
        ]),
        Expression::add(vec![
            Expression::symbol(y.clone()),
            Expression::integer(2)
        ])
    ]);
    
    let result = expr.simplify();
    println!("Complex expansion: (x + 1)(y + 2) = {}", result);
    
    assert!(!result.is_zero());
}
