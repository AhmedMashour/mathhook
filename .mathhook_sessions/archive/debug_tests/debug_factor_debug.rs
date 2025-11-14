//! Debug test for factorization

use mathhook::prelude::*;

#[test]
fn test_factor_debug() {
    let x = Symbol::new("x");
    
    let expr = Expression::add(vec![
        Expression::mul(vec![Expression::integer(6), Expression::symbol(x.clone())]),
        Expression::integer(12)
    ]);
    
    let result = expr.simplify();
    println!("Factor debug: 6x + 12 = {}", result);
    
    // Should factor out 6: 6(x + 2)
    assert!(!result.is_zero());
}
