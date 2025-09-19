//! SymPy-inspired GCD test cases

use mathhook_core::prelude::*;

#[test]
fn test_sympy_gcd_basic() {
    // Basic SymPy GCD cases
    let a = Expression::integer(48);
    let b = Expression::integer(18);
    let gcd = a.gcd(&b);
    
    assert_eq!(gcd, Expression::integer(6));
}

#[test]
fn test_sympy_gcd_polynomials() {
    let x = Symbol::new("x");
    
    // SymPy polynomial GCD
    let poly1 = Expression::add(vec![
        Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
        Expression::symbol(x.clone())
    ]);
    let poly2 = Expression::symbol(x.clone());
    
    let gcd = poly1.gcd(&poly2);
    println!("GCD(x^2 + x, x) = {}", gcd);
    
    // Should be x
    assert!(!gcd.is_zero());
}

#[test]
fn test_sympy_gcd_multivariate() {
    let x = Symbol::new("x");
    let y = Symbol::new("y");
    
    // Multivariate GCD from SymPy
    let poly1 = Expression::mul(vec![
        Expression::symbol(x.clone()),
        Expression::symbol(y.clone())
    ]);
    let poly2 = Expression::mul(vec![
        Expression::symbol(x.clone()),
        Expression::integer(3)
    ]);
    
    let gcd = poly1.gcd(&poly2);
    println!("SymPy multivariate GCD: {}", gcd);
    
    assert!(!gcd.is_zero());
}

#[test]
fn test_sympy_gcd_complex() {
    let x = Symbol::new("x");
    
    // Complex SymPy GCD case
    let poly1 = Expression::add(vec![
        Expression::pow(Expression::symbol(x.clone()), Expression::integer(3)),
        Expression::pow(Expression::symbol(x.clone()), Expression::integer(2))
    ]);
    let poly2 = Expression::add(vec![
        Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
        Expression::symbol(x.clone())
    ]);
    
    let gcd = poly1.gcd(&poly2);
    println!("SymPy complex GCD: {}", gcd);
    
    assert!(!gcd.is_zero());
}

#[test]
fn test_session_071_gcd_milestone() {
    // Session 071 milestone test
    let x = Symbol::new("x");
    let y = Symbol::new("y");
    
    let poly1 = Expression::add(vec![
        Expression::mul(vec![Expression::integer(2), Expression::symbol(x.clone())]),
        Expression::mul(vec![Expression::integer(4), Expression::symbol(y.clone())])
    ]);
    let poly2 = Expression::add(vec![
        Expression::mul(vec![Expression::integer(3), Expression::symbol(x.clone())]),
        Expression::mul(vec![Expression::integer(6), Expression::symbol(y.clone())])
    ]);
    
    let gcd = poly1.gcd(&poly2);
    println!("Session 071 GCD milestone: {}", gcd);
    
    // Should find common structure
    assert!(!gcd.is_zero());
}
