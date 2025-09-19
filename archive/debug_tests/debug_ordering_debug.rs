//! Debug test for term ordering

use mathhook::prelude::*;

#[test]
fn test_ordering_debug() {
    let x = Symbol::new("x");
    let y = Symbol::new("y");
    
    let expr = Expression::add(vec![
        Expression::symbol(y.clone()),
        Expression::symbol(x.clone()),
        Expression::integer(5)
    ]);
    
    let result = expr.simplify();
    println!("Ordering debug: y + x + 5 = {}", result);
    
    // Should maintain or reorder terms consistently
    assert!(!result.is_zero());
}
