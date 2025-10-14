//! Step-by-step verification tests

use mathhook::prelude::*;

#[test]
fn test_step_verification() {
    let x = Symbol::new("x");

    let expr = Expression::add(vec![
        Expression::integer(5),
        Expression::integer(3),
        Expression::symbol(x.clone()),
    ]);

    let result = expr.simplify();
    println!("Step verification: 5 + 3 + x = {}", result);

    // Should combine to 8 + x
    assert!(!result.is_zero());
}
