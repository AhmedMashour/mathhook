//! Comprehensive Number Theory Function Tests
//!
//! Tests for GCD, LCM, and other number theory operations.

use mathhook_core::{symbol, Expression};

#[test]
fn test_lcm_integers_basic() {
    let a = Expression::integer(12);
    let b = Expression::integer(8);
    let result = a.lcm(&b);
    assert_eq!(result, Expression::integer(24));
}

#[test]
fn test_lcm_coprime() {
    let a = Expression::integer(7);
    let b = Expression::integer(13);
    let result = a.lcm(&b);
    assert_eq!(result, Expression::integer(91));
}

#[test]
fn test_lcm_one_divides_other() {
    let a = Expression::integer(6);
    let b = Expression::integer(3);
    let result = a.lcm(&b);
    assert_eq!(result, Expression::integer(6));
}

#[test]
fn test_lcm_identical() {
    let a = Expression::integer(5);
    let b = Expression::integer(5);
    let result = a.lcm(&b);
    assert_eq!(result, Expression::integer(5));
}

#[test]
fn test_lcm_with_zero() {
    let zero = Expression::integer(0);
    let n = Expression::integer(5);
    let result = zero.lcm(&n);
    assert_eq!(result, Expression::integer(0));
}

#[test]
fn test_lcm_large_numbers() {
    let a = Expression::integer(48);
    let b = Expression::integer(18);
    let result = a.lcm(&b);
    assert_eq!(result, Expression::integer(144));
}

#[test]
fn test_gcd_integers_basic() {
    let a = Expression::integer(12);
    let b = Expression::integer(8);
    let result = a.gcd(&b);
    assert_eq!(result, Expression::integer(4));
}

#[test]
fn test_gcd_coprime() {
    let a = Expression::integer(7);
    let b = Expression::integer(13);
    let result = a.gcd(&b);
    assert_eq!(result, Expression::integer(1));
}

#[test]
fn test_gcd_one_divides_other() {
    let a = Expression::integer(15);
    let b = Expression::integer(5);
    let result = a.gcd(&b);
    assert_eq!(result, Expression::integer(5));
}

#[test]
fn test_gcd_identical() {
    let a = Expression::integer(7);
    let b = Expression::integer(7);
    let result = a.gcd(&b);
    assert_eq!(result, Expression::integer(7));
}

#[test]
fn test_gcd_with_zero() {
    let zero = Expression::integer(0);
    let n = Expression::integer(5);

    let result1 = zero.gcd(&n);
    assert_eq!(result1, Expression::integer(5));

    let result2 = n.gcd(&zero);
    assert_eq!(result2, Expression::integer(5));
}

#[test]
fn test_gcd_large_numbers() {
    let a = Expression::integer(48);
    let b = Expression::integer(18);
    let result = a.gcd(&b);
    assert_eq!(result, Expression::integer(6));
}

#[test]
fn test_gcd_symbolic_identical() {
    let x = symbol!(x);
    let expr = Expression::symbol(x.clone());
    let result = expr.gcd(&expr);
    assert_eq!(result, Expression::symbol(x));
}

#[test]
fn test_lcm_gcd_relationship() {
    // Mathematical property: LCM(a, b) * GCD(a, b) = |a * b|
    let a = Expression::integer(12);
    let b = Expression::integer(8);

    let lcm = a.lcm(&b);
    let gcd = a.gcd(&b);

    // LCM(12, 8) = 24, GCD(12, 8) = 4
    // 24 * 4 = 96 = 12 * 8
    assert_eq!(lcm, Expression::integer(24));
    assert_eq!(gcd, Expression::integer(4));
}

#[test]
fn test_gcd_commutative() {
    // Mathematical property: GCD(a, b) = GCD(b, a)
    let a = Expression::integer(18);
    let b = Expression::integer(24);

    let gcd1 = a.gcd(&b);
    let gcd2 = b.gcd(&a);

    assert_eq!(gcd1, gcd2);
    assert_eq!(gcd1, Expression::integer(6));
}

#[test]
fn test_lcm_commutative() {
    // Mathematical property: LCM(a, b) = LCM(b, a)
    let a = Expression::integer(18);
    let b = Expression::integer(24);

    let lcm1 = a.lcm(&b);
    let lcm2 = b.lcm(&a);

    assert_eq!(lcm1, lcm2);
    assert_eq!(lcm1, Expression::integer(72));
}

#[test]
fn test_gcd_associative() {
    // Mathematical property: GCD(GCD(a, b), c) = GCD(a, GCD(b, c))
    let a = Expression::integer(12);
    let b = Expression::integer(18);
    let c = Expression::integer(24);

    let gcd_ab = a.gcd(&b);
    let result1 = gcd_ab.gcd(&c);

    let gcd_bc = b.gcd(&c);
    let result2 = a.gcd(&gcd_bc);

    assert_eq!(result1, result2);
    assert_eq!(result1, Expression::integer(6));
}

#[test]
fn test_lcm_with_one() {
    // Mathematical property: LCM(1, n) = n
    let one = Expression::integer(1);
    let n = Expression::integer(42);

    let result = one.lcm(&n);
    assert_eq!(result, Expression::integer(42));
}

#[test]
fn test_gcd_with_one() {
    // Mathematical property: GCD(1, n) = 1
    let one = Expression::integer(1);
    let n = Expression::integer(42);

    let result = one.gcd(&n);
    assert_eq!(result, Expression::integer(1));
}

#[test]
fn test_cofactors_basic() {
    // Test cofactors method: (gcd, a/gcd, b/gcd)
    let a = Expression::integer(12);
    let b = Expression::integer(8);

    let (gcd, _cofactor_a, _cofactor_b) = a.cofactors(&b);

    assert_eq!(gcd, Expression::integer(4));
}

#[test]
fn test_gcd_negative_numbers() {
    let a = Expression::integer(-12);
    let b = Expression::integer(8);

    let result = a.gcd(&b);
    assert_eq!(result, Expression::integer(4));
}

#[test]
fn test_lcm_negative_numbers() {
    let a = Expression::integer(-12);
    let b = Expression::integer(8);

    let result = a.lcm(&b);
    assert_eq!(result, Expression::integer(24));
}
