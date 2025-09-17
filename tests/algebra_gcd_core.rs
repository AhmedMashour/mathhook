//! Core algebra GCD tests

use mathhook::prelude::*;

#[test]
fn test_algebra_gcd_basic() {
    let x = Symbol::new("x");
    
    // Basic algebraic GCD
    let expr1 = Expression::mul(vec![
        Expression::integer(4),
        Expression::symbol(x.clone())
    ]);
    let expr2 = Expression::mul(vec![
        Expression::integer(6),
        Expression::symbol(x.clone())
    ]);
    
    let gcd = expr1.gcd(&expr2);
    println!("Algebra GCD basic: {}", gcd);
    
    // Should find 2x as common factor
    assert!(!gcd.is_zero());
}

#[test]
fn test_algebra_gcd_factoring() {
    let x = Symbol::new("x");
    
    // Test factoring with GCD
    let expr = Expression::add(vec![
        Expression::mul(vec![Expression::integer(6), Expression::symbol(x.clone())]),
        Expression::integer(9)
    ]);
    
    let factored = expr.factor_gcd();
    println!("Algebra GCD factoring: {}", factored);
    
    // Should extract common factor 3
    assert!(!factored.is_zero());
}

#[test]
fn test_algebra_gcd_lcm() {
    let a = Expression::integer(12);
    let b = Expression::integer(18);
    
    let gcd = a.gcd(&b);
    let lcm = a.lcm(&b);
    
    println!("GCD(12, 18) = {}", gcd);
    println!("LCM(12, 18) = {}", lcm);
    
    assert_eq!(gcd, Expression::integer(6));
    assert!(!lcm.is_zero());
}

#[test]
fn test_algebra_gcd_cofactors() {
    let a = Expression::integer(24);
    let b = Expression::integer(36);
    
    let (gcd, cofactor_a, cofactor_b) = a.cofactors(&b);
    
    println!("Cofactors: GCD={}, A/GCD={}, B/GCD={}", gcd, cofactor_a, cofactor_b);
    
    assert!(!gcd.is_zero());
}

#[test]
fn test_algebra_gcd_zero_handling() {
    let x = Symbol::new("x");
    let zero = Expression::integer(0);
    
    // Test GCD with zero
    let expr = Expression::symbol(x.clone());
    let gcd1 = expr.gcd(&zero);
    let gcd2 = zero.gcd(&expr);
    
    assert_eq!(gcd1, Expression::symbol(x.clone()));
    assert_eq!(gcd2, Expression::symbol(x));
}

#[test]
fn test_algebra_gcd_performance_core() {
    use std::time::Instant;
    
    let x = Symbol::new("x");
    let start = Instant::now();
    
    // Core algebra GCD performance
    for i in 1..2000 {
        let expr1 = Expression::mul(vec![
            Expression::integer(i),
            Expression::symbol(x.clone())
        ]);
        let expr2 = Expression::mul(vec![
            Expression::integer(i * 2),
            Expression::symbol(x.clone())
        ]);
        let _gcd = expr1.gcd(&expr2);
    }
    
    let duration = start.elapsed();
    let ops_per_sec = 2000.0 / duration.as_secs_f64();
    
    println!("🚀 Algebra GCD Core Performance: {:.2}K ops/sec", ops_per_sec / 1000.0);
    
    // Should be extremely fast
    assert!(ops_per_sec > 100_000.0);
}
