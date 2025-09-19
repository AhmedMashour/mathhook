//! Targeted rational operation tests

use mathhook_core::prelude::*;

#[test]
fn test_targeted_rational() {
    let x = Symbol::new("x");
    
    let expr = Expression::mul(vec![
        Expression::number(Number::rational(
            num_rational::BigRational::new(num_bigint::BigInt::from(2), num_bigint::BigInt::from(3))
        )),
        Expression::symbol(x.clone())
    ]);
    
    let result = expr.simplify();
    println!("Targeted rational: (2/3)*x = {}", result);
    
    assert!(!result.is_zero());
}
