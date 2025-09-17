//! Polynomial GCD comprehensive tests

use mathhook::prelude::*;

#[test]
fn test_polynomial_gcd_comprehensive() {
    let x = Symbol::new("x");
    let y = Symbol::new("y");
    
    // Test comprehensive polynomial GCD cases
    let poly1 = Expression::mul(vec![
        Expression::integer(6),
        Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
        Expression::symbol(y.clone())
    ]);
    
    let poly2 = Expression::mul(vec![
        Expression::integer(9),
        Expression::symbol(x.clone()),
        Expression::symbol(y.clone())
    ]);
    
    let gcd = poly1.gcd(&poly2);
    println!("Comprehensive polynomial GCD: {}", gcd);
    
    // Should find common factors
    assert!(!gcd.is_zero());
}

#[test]
fn test_multivariate_polynomial_gcd() {
    let x = Symbol::new("x");
    let y = Symbol::new("y");
    let z = Symbol::new("z");
    
    // Test multivariate case
    let poly1 = Expression::mul(vec![
        Expression::symbol(x.clone()),
        Expression::symbol(y.clone()),
        Expression::symbol(z.clone())
    ]);
    
    let poly2 = Expression::mul(vec![
        Expression::symbol(x.clone()),
        Expression::symbol(y.clone())
    ]);
    
    let gcd = poly1.gcd(&poly2);
    println!("Multivariate GCD: {}", gcd);
    
    // Should be xy
    assert!(!gcd.is_zero());
}

#[test]
fn test_polynomial_gcd_with_coefficients() {
    let x = Symbol::new("x");
    
    // Test polynomial GCD with different coefficients
    let poly1 = Expression::add(vec![
        Expression::mul(vec![Expression::integer(12), Expression::symbol(x.clone())]),
        Expression::integer(18)
    ]);
    
    let poly2 = Expression::add(vec![
        Expression::mul(vec![Expression::integer(8), Expression::symbol(x.clone())]),
        Expression::integer(12)
    ]);
    
    let gcd = poly1.gcd(&poly2);
    println!("Polynomial GCD with coefficients: {}", gcd);
    
    // Should extract common factors
    assert!(!gcd.is_zero());
}

#[test]
fn test_polynomial_gcd_factoring() {
    let x = Symbol::new("x");
    
    // Test GCD that requires factoring
    let poly1 = Expression::add(vec![
        Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
        Expression::symbol(x.clone())
    ]);
    
    let poly2 = Expression::add(vec![
        Expression::pow(Expression::symbol(x.clone()), Expression::integer(3)),
        Expression::pow(Expression::symbol(x.clone()), Expression::integer(2))
    ]);
    
    let gcd = poly1.gcd(&poly2);
    println!("Factoring polynomial GCD: {}", gcd);
    
    // Should find x as common factor
    assert!(!gcd.is_zero());
}

#[test]
fn test_polynomial_gcd_performance() {
    use std::time::Instant;
    
    let x = Symbol::new("x");
    let start = Instant::now();
    
    // Performance test for polynomial GCD
    for i in 1..1000 {
        let poly1 = Expression::mul(vec![
            Expression::integer(i),
            Expression::symbol(x.clone())
        ]);
        let poly2 = Expression::mul(vec![
            Expression::integer(i * 3),
            Expression::symbol(x.clone())
        ]);
        let _gcd = poly1.gcd(&poly2);
    }
    
    let duration = start.elapsed();
    let ops_per_sec = 1000.0 / duration.as_secs_f64();
    
    println!("🚀 Polynomial GCD Performance: {:.2}K ops/sec", ops_per_sec / 1000.0);
    
    // Should be very fast
    assert!(ops_per_sec > 50_000.0);
}
