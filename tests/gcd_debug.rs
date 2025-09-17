//! GCD debug tests

use mathhook::prelude::*;

#[test]
fn test_gcd_debug() {
    let x = Symbol::new("x");
    
    let poly1 = Expression::mul(vec![
        Expression::integer(15),
        Expression::symbol(x.clone())
    ]);
    let poly2 = Expression::mul(vec![
        Expression::integer(25),
        Expression::symbol(x.clone())
    ]);
    
    let gcd = poly1.gcd(&poly2);
    println!("GCD debug: GCD(15x, 25x) = {}", gcd);
    
    // Should be 5x
    assert!(!gcd.is_zero());
}
