//! Final debug test

use mathhook::prelude::*;

#[test]
fn test_debug_final() {
    let x = Symbol::new("x");
    
    // Final debug test
    let expr = Expression::add(vec![
        Expression::symbol(x.clone()),
        Expression::integer(42)
    ]);
    
    let result = expr.simplify();
    println!("Final debug: x + 42 = {}", result);
    
    assert!(!result.is_zero());
}
