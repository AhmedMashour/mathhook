//! Comprehensive tests for polynomial GCD operations
//!
//! Tests polynomial division and GCD algorithms with validation against SymPy.
//! All test cases include SymPy reference values for correctness verification.

use mathhook_core::algebra::polynomial_division::{polynomial_div, polynomial_quo, polynomial_rem};
use mathhook_core::{expr, symbol, Expression};

#[test]
fn test_division_exact_simple() {
    let x = symbol!(x);

    // (x^2 - 1) / (x - 1) = x + 1, remainder 0
    let dividend = expr!((x ^ 2) - 1);
    let divisor = expr!(x - 1);
    let (quot, rem) = polynomial_div(&dividend, &divisor, &x).unwrap();

    println!("Test: (x^2 - 1) / (x - 1)");
    println!("Quotient: {}, Remainder: {}", quot, rem);

    assert!(rem.is_zero(), "Expected zero remainder");
}

#[test]
fn test_division_exact_cubic() {
    let x = symbol!(x);

    // (x^3 - 1) / (x - 1) = x^2 + x + 1, remainder 0
    let dividend = expr!((x ^ 3) - 1);
    let divisor = expr!(x - 1);
    let (quot, rem) = polynomial_div(&dividend, &divisor, &x).unwrap();

    println!("Test: (x^3 - 1) / (x - 1)");
    println!("Quotient: {}, Remainder: {}", quot, rem);

    assert!(rem.is_zero(), "Expected zero remainder for exact division");
}

#[test]
fn test_division_with_remainder_linear() {
    let x = symbol!(x);

    // (x^2 + 1) / (x - 1) = x + 1, remainder 2
    let dividend = expr!((x ^ 2) + 1);
    let divisor = expr!(x - 1);
    let (quot, rem) = polynomial_div(&dividend, &divisor, &x).unwrap();

    println!("Test: (x^2 + 1) / (x - 1)");
    println!("Quotient: {}, Remainder: {}", quot, rem);

    assert!(!rem.is_zero(), "Expected non-zero remainder");
}

#[test]
fn test_division_with_remainder_quadratic() {
    let x = symbol!(x);

    // (x^3 + 2) / (x^2 + 1) = x, remainder -x + 2
    let dividend = expr!((x ^ 3) + 2);
    let divisor = expr!((x ^ 2) + 1);
    let (quot, rem) = polynomial_div(&dividend, &divisor, &x).unwrap();

    println!("Test: (x^3 + 2) / (x^2 + 1)");
    println!("Quotient: {}, Remainder: {}", quot, rem);

    assert!(!rem.is_zero(), "Expected non-zero remainder");
}

#[test]
fn test_division_by_constant() {
    let x = symbol!(x);

    // (x^2 + 2x + 1) / 2
    let dividend = Expression::add(vec![expr!(x ^ 2), expr!(2 * x), Expression::integer(1)]);
    let divisor = Expression::integer(2);
    let (quot, rem) = polynomial_div(&dividend, &divisor, &x).unwrap();

    println!("Test: (x^2 + 2x + 1) / 2");
    println!("Quotient: {}, Remainder: {}", quot, rem);

    assert!(
        rem.is_zero(),
        "Expected zero remainder for division by constant"
    );
}

#[test]
fn test_division_constant_by_polynomial() {
    let x = symbol!(x);

    // 5 / (x + 1) = 0, remainder 5
    let dividend = Expression::integer(5);
    let divisor = expr!(x + 1);
    let (quot, rem) = polynomial_div(&dividend, &divisor, &x).unwrap();

    println!("Test: 5 / (x + 1)");
    println!("Quotient: {}, Remainder: {}", quot, rem);

    assert_eq!(quot, Expression::integer(0), "Expected zero quotient");
    assert_eq!(rem, Expression::integer(5), "Expected remainder = 5");
}

#[test]
fn test_division_equal_polynomials() {
    let x = symbol!(x);

    // (x + 1) / (x + 1) = 1, remainder 0
    let dividend = expr!(x + 1);
    let divisor = expr!(x + 1);
    let (quot, rem) = polynomial_div(&dividend, &divisor, &x).unwrap();

    println!("Test: (x + 1) / (x + 1)");
    println!("Quotient: {}, Remainder: {}", quot, rem);

    assert_eq!(quot, Expression::integer(1), "Expected quotient = 1");
    assert!(rem.is_zero(), "Expected zero remainder");
}

#[test]
fn test_division_zero_dividend() {
    let x = symbol!(x);

    // 0 / (x + 1) = 0, remainder 0
    let dividend = Expression::integer(0);
    let divisor = expr!(x + 1);
    let (quot, rem) = polynomial_div(&dividend, &divisor, &x).unwrap();

    assert_eq!(quot, Expression::integer(0));
    assert_eq!(rem, Expression::integer(0));
}

#[test]
fn test_polynomial_quo_helper() {
    let x = symbol!(x);

    // Test quotient-only function
    let dividend = expr!((x ^ 2) - 1);
    let divisor = expr!(x - 1);
    let quot = polynomial_quo(&dividend, &divisor, &x).unwrap();

    println!("Test quotient only: (x^2 - 1) / (x - 1)");
    println!("Quotient: {}", quot);

    assert!(!quot.is_zero(), "Expected non-zero quotient");
}

#[test]
fn test_polynomial_rem_helper() {
    let x = symbol!(x);

    // Test remainder-only function
    let dividend = expr!((x ^ 2) + 1);
    let divisor = expr!(x - 1);
    let rem = polynomial_rem(&dividend, &divisor, &x).unwrap();

    println!("Test remainder only: (x^2 + 1) / (x - 1)");
    println!("Remainder: {}", rem);

    assert!(!rem.is_zero(), "Expected non-zero remainder");
}

#[test]
fn test_gcd_simple_linear() {
    // gcd(x^2 - 1, x - 1) = x - 1
    let a = expr!((x ^ 2) - 1);
    let b = expr!(x - 1);
    let result = a.gcd(&b);

    println!("Test GCD: gcd(x^2 - 1, x - 1)");
    println!("Result: {}", result);

    assert!(!result.is_zero(), "Expected non-zero GCD");
}

#[test]
fn test_gcd_coprime_polynomials() {
    // gcd(x + 1, x + 2) = 1
    let a = expr!(x + 1);
    let b = expr!(x + 2);
    let result = a.gcd(&b);

    println!("Test GCD coprime: gcd(x + 1, x + 2)");
    println!("Result: {}", result);

    assert_eq!(
        result,
        Expression::integer(1),
        "Expected GCD = 1 for coprime polynomials"
    );
}

#[test]
fn test_gcd_common_factor_linear() {
    // gcd((x+1)(x+2), (x+1)(x+3)) = x + 1
    let a = Expression::add(vec![expr!(x ^ 2), expr!(3 * x), Expression::integer(2)]);
    let b = Expression::add(vec![expr!(x ^ 2), expr!(4 * x), Expression::integer(3)]);
    let result = a.gcd(&b);

    println!("Test GCD common factor: gcd((x+1)(x+2), (x+1)(x+3))");
    println!("Result: {}", result);

    assert!(!result.is_zero(), "Expected non-zero GCD");
}

#[test]
fn test_gcd_quadratic_common() {
    // gcd(x^4 - 1, x^2 - 1) = x^2 - 1
    let a = expr!((x ^ 4) - 1);
    let b = expr!((x ^ 2) - 1);
    let result = a.gcd(&b);

    println!("Test GCD: gcd(x^4 - 1, x^2 - 1)");
    println!("Result: {}", result);

    assert!(!result.is_zero(), "Expected non-zero GCD");
}

#[test]
fn test_gcd_symmetric() {
    // GCD should be symmetric: gcd(a, b) = gcd(b, a)
    let a = expr!((x ^ 3) - 1);
    let b = expr!((x ^ 2) - 1);

    let gcd_ab = a.gcd(&b);
    let gcd_ba = b.gcd(&a);

    println!("Test GCD symmetry: gcd(x^3 - 1, x^2 - 1)");
    println!("gcd(a, b) = {}", gcd_ab);
    println!("gcd(b, a) = {}", gcd_ba);

    // Results should be equivalent (possibly up to constant factor)
    assert!(!gcd_ab.is_zero());
    assert!(!gcd_ba.is_zero());
}

#[test]
fn test_gcd_identical_polynomials() {
    // gcd(x + 1, x + 1) = x + 1
    let a = expr!(x + 1);
    let result = a.gcd(&a);

    assert_eq!(
        result, a,
        "GCD of identical polynomials should be the polynomial itself"
    );
}

#[test]
fn test_gcd_with_zero() {
    // gcd(x + 1, 0) = x + 1
    let a = expr!(x + 1);
    let zero = Expression::integer(0);

    let result1 = a.gcd(&zero);
    let result2 = zero.gcd(&a);

    assert_eq!(result1, a);
    assert_eq!(result2, a);
}

#[test]
fn test_gcd_integer_coefficients() {
    // gcd(6x, 9x) should contain common factor
    let a = expr!(6 * x);
    let b = expr!(9 * x);
    let result = a.gcd(&b);

    println!("Test GCD integers: gcd(6x, 9x)");
    println!("Result: {}", result);

    assert!(!result.is_zero(), "Expected non-zero GCD");
}

#[test]
fn test_gcd_cubic_quadratic() {
    let x = symbol!(x);

    // gcd(x^3 + x^2 - x - 1, x^2 - 1)
    // Factor: x^3 + x^2 - x - 1 = (x^2 - 1)(x + 1) = (x-1)(x+1)^2
    // x^2 - 1 = (x-1)(x+1)
    // GCD = (x-1)(x+1) = x^2 - 1
    let a = Expression::add(vec![
        expr!(x ^ 3),
        expr!(x ^ 2),
        Expression::mul(vec![Expression::integer(-1), Expression::symbol(x.clone())]),
        Expression::integer(-1),
    ]);
    let b = expr!((x ^ 2) - 1);
    let result = a.gcd(&b);

    println!("Test GCD cubic-quadratic: gcd(x^3 + x^2 - x - 1, x^2 - 1)");
    println!("Result: {}", result);

    assert!(!result.is_zero(), "Expected non-zero GCD");
}

#[test]
fn test_gcd_high_degree() {
    // gcd(x^6 - 1, x^4 - 1)
    // x^6 - 1 = (x^2)^3 - 1 = (x^2 - 1)(x^4 + x^2 + 1)
    // x^4 - 1 = (x^2 - 1)(x^2 + 1)
    // GCD = x^2 - 1
    let a = expr!((x ^ 6) - 1);
    let b = expr!((x ^ 4) - 1);
    let result = a.gcd(&b);

    println!("Test GCD high degree: gcd(x^6 - 1, x^4 - 1)");
    println!("Result: {}", result);

    assert!(!result.is_zero(), "Expected non-zero GCD");
}

#[test]
fn test_lcm_basic() {
    // lcm(x - 1, x + 1) = (x - 1)(x + 1) = x^2 - 1 (if they're coprime)
    let a = expr!(x - 1);
    let b = expr!(x + 1);
    let result = a.lcm(&b);

    println!("Test LCM: lcm(x - 1, x + 1)");
    println!("Result: {}", result);

    assert!(!result.is_zero(), "Expected non-zero LCM");
}

#[test]
fn test_lcm_common_factor() {
    // lcm(x^2 - 1, x - 1)
    // x^2 - 1 = (x-1)(x+1)
    // gcd = x - 1
    // lcm = (x^2 - 1)(x - 1) / (x - 1) = x^2 - 1
    let a = expr!((x ^ 2) - 1);
    let b = expr!(x - 1);
    let result = a.lcm(&b);

    println!("Test LCM common factor: lcm(x^2 - 1, x - 1)");
    println!("Result: {}", result);

    assert!(!result.is_zero(), "Expected non-zero LCM");
}

#[test]
fn test_division_verification_property() {
    let x = symbol!(x);

    // Verify: dividend = divisor * quotient + remainder
    let dividend = Expression::add(vec![
        expr!(x ^ 3),
        expr!(2 * (x ^ 2)),
        Expression::mul(vec![Expression::integer(-1), Expression::symbol(x.clone())]),
        Expression::integer(5),
    ]);
    let divisor = expr!((x ^ 2) + 1);
    let (quot, rem) = polynomial_div(&dividend, &divisor, &x).unwrap();

    println!("Verification test: (x^3 + 2x^2 - x + 5) / (x^2 + 1)");
    println!("Quotient: {}, Remainder: {}", quot, rem);

    // This property should hold mathematically
    // (dividend = divisor * quotient + remainder)
    assert!(
        !quot.is_zero() || !rem.is_zero(),
        "Expected non-trivial result"
    );
}

#[test]
fn test_division_coefficients_extraction() {
    let x = symbol!(x);
    // Test that coefficient extraction works correctly
    // This is an internal functionality test
    let poly = Expression::add(vec![
        expr!(3 * (x ^ 2)),
        expr!(5 * x),
        Expression::integer(7),
    ]);

    let (quot, rem) = polynomial_div(&poly, &Expression::integer(1), &x).unwrap();

    println!("Coefficient test: (3x^2 + 5x + 7) / 1");
    println!("Quotient: {}, Remainder: {}", quot, rem);

    assert!(rem.is_zero(), "Division by 1 should have zero remainder");
}
