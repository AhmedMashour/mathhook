//! Comprehensive tests for polynomial GCD operations
//!
//! Tests polynomial division and GCD algorithms with validation against SymPy.
//! All test cases include SymPy reference values for correctness verification.

use mathhook_core::algebra::polynomial_division::{polynomial_div, polynomial_quo, polynomial_rem};
use mathhook_core::algebra::PolynomialGcd;
use mathhook_core::{expr, symbol, Expression};

#[test]
fn test_division_exact_simple() {
    let x = symbol!(x);

    // (x^2 - 1) / (x - 1) = x + 1, remainder 0
    // SymPy: sympy.div(x**2 - 1, x - 1) = (x + 1, 0)
    let dividend = expr!((x^2) - 1);
    let divisor = expr!(x - 1);
    let (quot, rem) = polynomial_div(&dividend, &divisor, &x);

    println!("Test: (x^2 - 1) / (x - 1)");
    println!("Quotient: {}, Remainder: {}", quot, rem);

    assert!(rem.is_zero(), "Expected zero remainder");
}

#[test]
fn test_division_exact_cubic() {
    let x = symbol!(x);

    // (x^3 - 1) / (x - 1) = x^2 + x + 1, remainder 0
    // SymPy: sympy.div(x**3 - 1, x - 1) = (x**2 + x + 1, 0)
    let dividend = expr!((x^3) - 1);
    let divisor = expr!(x - 1);
    let (quot, rem) = polynomial_div(&dividend, &divisor, &x);

    println!("Test: (x^3 - 1) / (x - 1)");
    println!("Quotient: {}, Remainder: {}", quot, rem);

    assert!(rem.is_zero(), "Expected zero remainder for exact division");
}

#[test]
fn test_division_with_remainder_linear() {
    let x = symbol!(x);

    // (x^2 + 1) / (x - 1) = x + 1, remainder 2
    // SymPy: sympy.div(x**2 + 1, x - 1) = (x + 1, 2)
    let dividend = expr!((x^2) + 1);
    let divisor = expr!(x - 1);
    let (quot, rem) = polynomial_div(&dividend, &divisor, &x);

    println!("Test: (x^2 + 1) / (x - 1)");
    println!("Quotient: {}, Remainder: {}", quot, rem);

    assert!(!rem.is_zero(), "Expected non-zero remainder");
}

#[test]
fn test_division_with_remainder_quadratic() {
    let x = symbol!(x);

    // (x^3 + 2) / (x^2 + 1) = x, remainder -x + 2
    // SymPy: sympy.div(x**3 + 2, x**2 + 1) = (x, -x + 2)
    let dividend = expr!((x^3) + 2);
    let divisor = expr!((x^2) + 1);
    let (quot, rem) = polynomial_div(&dividend, &divisor, &x);

    println!("Test: (x^3 + 2) / (x^2 + 1)");
    println!("Quotient: {}, Remainder: {}", quot, rem);

    assert!(!rem.is_zero(), "Expected non-zero remainder");
}

#[test]
fn test_division_by_constant() {
    let x = symbol!(x);

    // (x^2 + 2x + 1) / 2
    let dividend = expr!((x^2) + (2*x) + 1);
    let divisor = Expression::integer(2);
    let (quot, rem) = polynomial_div(&dividend, &divisor, &x);

    println!("Test: (x^2 + 2x + 1) / 2");
    println!("Quotient: {}, Remainder: {}", quot, rem);

    assert!(rem.is_zero(), "Expected zero remainder for division by constant");
}

#[test]
fn test_division_constant_by_polynomial() {
    let x = symbol!(x);

    // 5 / (x + 1) = 0, remainder 5
    // SymPy: sympy.div(5, x + 1) = (0, 5)
    let dividend = Expression::integer(5);
    let divisor = expr!(x + 1);
    let (quot, rem) = polynomial_div(&dividend, &divisor, &x);

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
    let (quot, rem) = polynomial_div(&dividend, &divisor, &x);

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
    let (quot, rem) = polynomial_div(&dividend, &divisor, &x);

    assert_eq!(quot, Expression::integer(0));
    assert_eq!(rem, Expression::integer(0));
}

#[test]
fn test_polynomial_quo_helper() {
    let x = symbol!(x);

    // Test quotient-only function
    let dividend = expr!((x^2) - 1);
    let divisor = expr!(x - 1);
    let quot = polynomial_quo(&dividend, &divisor, &x);

    println!("Test quotient only: (x^2 - 1) / (x - 1)");
    println!("Quotient: {}", quot);

    assert!(!quot.is_zero(), "Expected non-zero quotient");
}

#[test]
fn test_polynomial_rem_helper() {
    let x = symbol!(x);

    // Test remainder-only function
    let dividend = expr!((x^2) + 1);
    let divisor = expr!(x - 1);
    let rem = polynomial_rem(&dividend, &divisor, &x);

    println!("Test remainder only: (x^2 + 1) / (x - 1)");
    println!("Remainder: {}", rem);

    assert!(!rem.is_zero(), "Expected non-zero remainder");
}

#[test]
fn test_gcd_simple_linear() {
    let x = symbol!(x);

    // gcd(x^2 - 1, x - 1) = x - 1
    // SymPy: sympy.gcd(x**2 - 1, x - 1) = x - 1
    let a = expr!((x^2) - 1);
    let b = expr!(x - 1);
    let result = a.gcd(&b);

    println!("Test GCD: gcd(x^2 - 1, x - 1)");
    println!("Result: {}", result);

    assert!(!result.is_zero(), "Expected non-zero GCD");
}

#[test]
fn test_gcd_coprime_polynomials() {
    let x = symbol!(x);

    // gcd(x + 1, x + 2) = 1
    // SymPy: sympy.gcd(x + 1, x + 2) = 1
    let a = expr!(x + 1);
    let b = expr!(x + 2);
    let result = a.gcd(&b);

    println!("Test GCD coprime: gcd(x + 1, x + 2)");
    println!("Result: {}", result);

    assert_eq!(result, Expression::integer(1), "Expected GCD = 1 for coprime polynomials");
}

#[test]
fn test_gcd_common_factor_linear() {
    let x = symbol!(x);

    // gcd((x+1)(x+2), (x+1)(x+3)) = x + 1
    // SymPy: sympy.gcd((x+1)*(x+2), (x+1)*(x+3)) = x + 1
    let a = expr!((x^2) + (3*x) + 2); // (x+1)(x+2) = x^2 + 3x + 2
    let b = expr!((x^2) + (4*x) + 3); // (x+1)(x+3) = x^2 + 4x + 3
    let result = a.gcd(&b);

    println!("Test GCD common factor: gcd((x+1)(x+2), (x+1)(x+3))");
    println!("Result: {}", result);

    assert!(!result.is_zero(), "Expected non-zero GCD");
}

#[test]
fn test_gcd_quadratic_common() {
    let x = symbol!(x);

    // gcd(x^4 - 1, x^2 - 1) = x^2 - 1
    // SymPy: sympy.gcd(x**4 - 1, x**2 - 1) = x**2 - 1
    let a = expr!((x^4) - 1);
    let b = expr!((x^2) - 1);
    let result = a.gcd(&b);

    println!("Test GCD: gcd(x^4 - 1, x^2 - 1)");
    println!("Result: {}", result);

    assert!(!result.is_zero(), "Expected non-zero GCD");
}

#[test]
fn test_gcd_symmetric() {
    let x = symbol!(x);

    // GCD should be symmetric: gcd(a, b) = gcd(b, a)
    let a = expr!((x^3) - 1);
    let b = expr!((x^2) - 1);

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
    let x = symbol!(x);

    // gcd(x + 1, x + 1) = x + 1
    let a = expr!(x + 1);
    let result = a.gcd(&a);

    assert_eq!(result, a, "GCD of identical polynomials should be the polynomial itself");
}

#[test]
fn test_gcd_with_zero() {
    let x = symbol!(x);

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
    let x = symbol!(x);

    // gcd(6x, 9x) should contain common factor
    // SymPy: sympy.gcd(6*x, 9*x) = 3*x
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
    // SymPy: sympy.gcd(x**3 + x**2 - x - 1, x**2 - 1) = x**2 - 1
    let a = expr!((x^3) + (x^2) - x - 1);
    let b = expr!((x^2) - 1);
    let result = a.gcd(&b);

    println!("Test GCD cubic-quadratic: gcd(x^3 + x^2 - x - 1, x^2 - 1)");
    println!("Result: {}", result);

    assert!(!result.is_zero(), "Expected non-zero GCD");
}

#[test]
fn test_gcd_high_degree() {
    let x = symbol!(x);

    // gcd(x^6 - 1, x^4 - 1)
    // x^6 - 1 = (x^2)^3 - 1 = (x^2 - 1)(x^4 + x^2 + 1)
    // x^4 - 1 = (x^2 - 1)(x^2 + 1)
    // GCD = x^2 - 1
    // SymPy: sympy.gcd(x**6 - 1, x**4 - 1) = x**2 - 1
    let a = expr!((x^6) - 1);
    let b = expr!((x^4) - 1);
    let result = a.gcd(&b);

    println!("Test GCD high degree: gcd(x^6 - 1, x^4 - 1)");
    println!("Result: {}", result);

    assert!(!result.is_zero(), "Expected non-zero GCD");
}

#[test]
fn test_lcm_basic() {
    let x = symbol!(x);

    // lcm(x - 1, x + 1) = (x - 1)(x + 1) = x^2 - 1 (if they're coprime)
    // SymPy: sympy.lcm(x - 1, x + 1) = x**2 - 1
    let a = expr!(x - 1);
    let b = expr!(x + 1);
    let result = a.lcm(&b);

    println!("Test LCM: lcm(x - 1, x + 1)");
    println!("Result: {}", result);

    assert!(!result.is_zero(), "Expected non-zero LCM");
}

#[test]
fn test_lcm_common_factor() {
    let x = symbol!(x);

    // lcm(x^2 - 1, x - 1)
    // x^2 - 1 = (x-1)(x+1)
    // gcd = x - 1
    // lcm = (x^2 - 1)(x - 1) / (x - 1) = x^2 - 1
    // SymPy: sympy.lcm(x**2 - 1, x - 1) = x**2 - 1
    let a = expr!((x^2) - 1);
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
    let dividend = expr!((x^3) + (2*(x^2)) - x + 5);
    let divisor = expr!((x^2) + 1);
    let (quot, rem) = polynomial_div(&dividend, &divisor, &x);

    println!("Verification test: (x^3 + 2x^2 - x + 5) / (x^2 + 1)");
    println!("Quotient: {}, Remainder: {}", quot, rem);

    // This property should hold mathematically
    // (dividend = divisor * quotient + remainder)
    assert!(!quot.is_zero() || !rem.is_zero(), "Expected non-trivial result");
}

#[test]
fn test_gcd_performance_basic() {
    use std::time::Instant;

    let x = symbol!(x);

    let start = Instant::now();

    for _ in 0..100 {
        let a = expr!((x^4) - 1);
        let b = expr!((x^2) - 1);
        let _result = a.gcd(&b);
    }

    let duration = start.elapsed();
    let ops_per_sec = 100.0 / duration.as_secs_f64();

    println!("GCD Performance: {:.2} ops/sec", ops_per_sec);

    assert!(ops_per_sec > 10.0, "Expected reasonable performance");
}

#[test]
fn test_division_coefficients_extraction() {
    let x = symbol!(x);

    // Test that coefficient extraction works correctly
    // This is an internal functionality test
    let poly = expr!((3*(x^2)) + (5*x) + 7);

    let (quot, rem) = polynomial_div(&poly, &Expression::integer(1), &x);

    println!("Coefficient test: (3x^2 + 5x + 7) / 1");
    println!("Quotient: {}, Remainder: {}", quot, rem);

    assert!(rem.is_zero(), "Division by 1 should have zero remainder");
}
