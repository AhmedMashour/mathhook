//! JSON API integration tests

use mathhook::prelude::*;

#[test]
fn test_json_serialization() {
    let x = Symbol::new("x");
    let expr = Expression::add(vec![
        Expression::symbol(x.clone()),
        Expression::integer(5)
    ]);
    
    // Test JSON serialization
    let json = serde_json::to_string(&expr).unwrap();
    println!("JSON: {}", json);
    
    // Test deserialization
    let deserialized: Expression = serde_json::from_str(&json).unwrap();
    assert_eq!(expr, deserialized);
}

#[test]
fn test_complex_json() {
    let x = Symbol::new("x");
    
    let expr = Expression::pow(
        Expression::add(vec![
            Expression::symbol(x.clone()),
            Expression::integer(1)
        ]),
        Expression::integer(2)
    );
    
    let json = serde_json::to_string(&expr).unwrap();
    let deserialized: Expression = serde_json::from_str(&json).unwrap();
    
    assert_eq!(expr, deserialized);
}

#[test]
fn test_rational_json() {
    let rational = Expression::number(CompactNumber::rational(
        num_rational::BigRational::new(num_bigint::BigInt::from(3), num_bigint::BigInt::from(4))
    ));
    
    let json = serde_json::to_string(&rational).unwrap();
    let deserialized: Expression = serde_json::from_str(&json).unwrap();
    
    assert_eq!(rational, deserialized);
}
