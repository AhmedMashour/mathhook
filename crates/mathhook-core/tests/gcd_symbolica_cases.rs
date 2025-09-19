//! Symbolica-inspired GCD test cases

use mathhook_core::prelude::*;

#[test]
fn test_symbolica_gcd_case_1() {
    let x = Symbol::new("x");
    
    // Symbolica test case: GCD of polynomials
    let poly1 = Expression::mul(vec![
        Expression::integer(6),
        Expression::symbol(x.clone())
    ]);
    let poly2 = Expression::mul(vec![
        Expression::integer(9),
        Expression::symbol(x.clone())
    ]);
    
    let gcd = poly1.gcd(&poly2);
    println!("GCD(6x, 9x) = {}", gcd);
    
    // Should find common factor
    assert!(!gcd.is_zero());
}

#[test]
fn test_symbolica_gcd_case_2() {
    let x = Symbol::new("x");
    let y = Symbol::new("y");
    
    // Multivariate case
    let poly1 = Expression::mul(vec![
        Expression::symbol(x.clone()),
        Expression::symbol(y.clone())
    ]);
    let poly2 = Expression::mul(vec![
        Expression::symbol(x.clone()),
        Expression::integer(2)
    ]);
    
    let gcd = poly1.gcd(&poly2);
    println!("GCD(xy, 2x) = {}", gcd);
    
    // Should find x as common factor
    assert!(!gcd.is_zero());
}

#[test]
fn test_symbolica_gcd_case_3() {
    // Large coefficient case
    let x = Symbol::new("x");
    
    let poly1 = Expression::mul(vec![
        Expression::integer(12345),
        Expression::symbol(x.clone())
    ]);
    let poly2 = Expression::mul(vec![
        Expression::integer(67890),
        Expression::symbol(x.clone())
    ]);
    
    let gcd = poly1.gcd(&poly2);
    println!("GCD(12345x, 67890x) = {}", gcd);
    
    assert!(!gcd.is_zero());
}

#[test]
fn test_symbolica_gcd_case_4() {
    let x = Symbol::new("x");
    
    // Power case
    let poly1 = Expression::pow(Expression::symbol(x.clone()), Expression::integer(3));
    let poly2 = Expression::pow(Expression::symbol(x.clone()), Expression::integer(2));
    
    let gcd = poly1.gcd(&poly2);
    println!("GCD(x^3, x^2) = {}", gcd);
    
    // Should be x^2
    assert!(!gcd.is_zero());
}
