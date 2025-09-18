//! Rational pipeline tests

use mathhook::prelude::*;

#[test]
fn test_rational_pipeline() {
    // Test rational number pipeline
    let expr = Expression::add(vec![
        Expression::number(Number::rational(
            num_rational::BigRational::new(num_bigint::BigInt::from(1), num_bigint::BigInt::from(4))
        )),
        Expression::number(Number::rational(
            num_rational::BigRational::new(num_bigint::BigInt::from(1), num_bigint::BigInt::from(4))
        ))
    ]);
    
    let result = expr.simplify();
    println!("Rational pipeline: 1/4 + 1/4 = {}", result);
    
    assert!(!result.is_zero());
}
