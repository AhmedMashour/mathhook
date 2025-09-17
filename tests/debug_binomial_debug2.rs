//! Debug test for binomial expansion output format

use mathhook::prelude::*;

#[test]
fn test_binomial_debug2() {
    let x = Symbol::new("x");
    
    let expr = Expression::pow(
        Expression::add(vec![
            Expression::symbol(x.clone()),
            Expression::integer(2)
        ]),
        Expression::integer(2)
    );
    
    let result = expr.simplify();
    println!("Binomial debug 2: (x + 2)^2 = {}", result);
    
    assert!(!result.is_zero());
}
