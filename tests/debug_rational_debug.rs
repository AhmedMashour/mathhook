//! Debug test for rational operations

use mathhook::prelude::*;

#[test]
fn test_rational_debug() {
    let rational = Expression::number(Number::rational(
        num_rational::BigRational::new(num_bigint::BigInt::from(5), num_bigint::BigInt::from(7))
    ));
    
    let result = rational.simplify();
    println!("Rational debug: 5/7 = {}", result);
    
    assert!(!result.is_zero());
}
