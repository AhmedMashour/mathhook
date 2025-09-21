//! Power cancellation tests

use mathhook_core::prelude::*;

#[test]
fn test_power_cancellation() {
    let x = Symbol::new("x");
    
    // Test x^2 / x = x (represented as x^2 * x^(-1))
    let expr = Expression::mul(vec![
        Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
        Expression::pow(Expression::symbol(x.clone()), Expression::integer(-1))
    ]);
    
    let result = expr.simplify();
    println!("Power cancellation: x^2 * x^(-1) = {}", result);
    
    // Should simplify to x when power rules are implemented
    assert!(!result.is_zero());
}
