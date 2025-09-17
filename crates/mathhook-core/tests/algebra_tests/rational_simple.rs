//! Simple rational tests

use mathhook_core::prelude::*;

#[test]
fn test_simple_rational() {
    let rational = Expression::number(Number::rational(num_rational::BigRational::new(
        num_bigint::BigInt::from(3),
        num_bigint::BigInt::from(4),
    )));

    let result = rational.simplify();
    println!("Simple rational: 3/4 = {}", result);

    assert!(!result.is_zero());
}
