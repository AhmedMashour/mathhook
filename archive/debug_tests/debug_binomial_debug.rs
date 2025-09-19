//! Debug test for binomial expansion

use mathhook::prelude::*;

#[test]
fn test_binomial_debug() {
    let x = Symbol::new("x");
    
    // Debug binomial expansion
    let expr = Expression::pow(
        Expression::add(vec![
            Expression::symbol(x.clone()),
            Expression::integer(1)
        ]),
        Expression::integer(2)
    );
    
    let result = expr.simplify();
    println!("Debug: (x + 1)^2 = {}", result);
    
    assert!(!result.is_zero());
}
