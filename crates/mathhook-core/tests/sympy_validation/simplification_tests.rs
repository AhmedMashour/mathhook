//! SymPy validation tests for simplification operations
//!
//! Tests validate MathHook simplification against SymPy reference implementation
//! located at ~/Documents/work/math/sympy/
//!
//! Each test documents the equivalent SymPy command for verification.

use mathhook_core::prelude::*;

#[test]
fn test_simplify_integer_addition() {
    // simplify(2 + 3) = 5
    let expr = expr!(2 + 3);
    let result = expr.simplify();
    assert_eq!(result, expr!(5));
}

#[test]
fn test_simplify_integer_multiplication() {
    // SymPy: simplify(2 * 3) = 6
    let expr = expr!(2 * 3);
    let result = expr.simplify();
    assert_eq!(result, expr!(6));
}

#[test]
fn test_simplify_integer_power() {
    // SymPy: simplify(2**3) = 8
    let expr = expr!(2 ^ 3);
    let result = expr.simplify();
    assert_eq!(result, expr!(8));
}

#[test]
fn test_simplify_power_of_one() {
    // SymPy: simplify(x**1) = x
    let x = symbol!(x);
    let expr = expr!(x ^ 1);
    let result = expr.simplify();
    assert_eq!(result, expr!(x));
}

#[test]
fn test_simplify_power_of_zero() {
    // SymPy: simplify(x**0) = 1
    let x = symbol!(x);
    let expr = expr!(x ^ 0);
    let result = expr.simplify();
    assert_eq!(result, expr!(1));
}

#[test]
fn test_simplify_zero_multiplication() {
    // SymPy: simplify(0 * x) = 0
    let x = symbol!(x);
    let expr = expr!(0 * x);
    let result = expr.simplify();
    assert_eq!(result, expr!(0));
}

#[test]
fn test_simplify_one_multiplication() {
    // SymPy: simplify(1 * x) = x
    let x = symbol!(x);
    let expr = expr!(1 * x);
    let result = expr.simplify();
    assert_eq!(result, expr!(x));
}

#[test]
fn test_simplify_zero_addition() {
    // SymPy: simplify(x + 0) = x
    let x = symbol!(x);
    let expr = expr!(x + 0);
    let result = expr.simplify();
    assert_eq!(result, expr!(x));
}

#[test]
fn test_simplify_like_terms() {
    // SymPy: simplify(2*x + 3*x) = 5*x
    let x = symbol!(x);
    let expr = expr!(add: (2 * x), (3 * x));
    let result = expr.simplify();
    let expected = expr!(5 * x);
    assert_eq!(result, expected);
}

#[test]
fn test_simplify_nested_addition() {
    // SymPy: simplify(1 + 2 + 3 + 4) = 10
    let expr = expr!(add: 1, 2, 3, 4);
    let result = expr.simplify();
    assert_eq!(result, expr!(10));
}

#[test]
fn test_simplify_nested_multiplication() {
    // SymPy: simplify(2 * 3 * 4) = 24
    let expr = expr!(mul: 2, 3, 4);
    let result = expr.simplify();
    assert_eq!(result, expr!(24));
}

#[test]
fn test_simplify_double_negation() {
    // SymPy: simplify(-(-x)) = x
    let x = symbol!(x);
    let expr = Expression::mul(vec![
        Expression::integer(-1),
        Expression::mul(vec![Expression::integer(-1), Expression::symbol(x.clone())]),
    ]);
    let result = expr.simplify();
    assert_eq!(result, expr!(x));
}

#[test]
fn test_simplify_sin_zero() {
    // SymPy: simplify(sin(0)) = 0
    let expr = function!(sin, expr!(0));
    let result = expr.simplify();
    assert_eq!(result, expr!(0));
}

#[test]
fn test_simplify_cos_zero() {
    // SymPy: simplify(cos(0)) = 1
    let expr = function!(cos, expr!(0));
    let result = expr.simplify();
    assert_eq!(result, expr!(1));
}

#[test]
fn test_simplify_exp_zero() {
    // SymPy: simplify(exp(0)) = 1
    let expr = function!(exp, expr!(0));
    let result = expr.simplify();
    assert_eq!(result, expr!(1));
}

#[test]
fn test_simplify_log_one() {
    // SymPy: simplify(log(1)) = 0
    let expr = function!(log, expr!(1));
    let result = expr.simplify();
    assert_eq!(result, expr!(0));
}

#[test]
fn test_simplify_sqrt_zero() {
    // SymPy: simplify(sqrt(0)) = 0
    let expr = function!(sqrt, expr!(0));
    let result = expr.simplify();
    assert_eq!(result, expr!(0));
}

#[test]
fn test_simplify_sqrt_one() {
    // SymPy: simplify(sqrt(1)) = 1
    let expr = function!(sqrt, expr!(1));
    let result = expr.simplify();
    assert_eq!(result, expr!(1));
}

#[test]
fn test_simplify_sqrt_four() {
    // SymPy: simplify(sqrt(4)) = 2
    let expr = function!(sqrt, expr!(4));
    let result = expr.simplify();
    assert_eq!(result, expr!(2));
}

#[test]
fn test_simplify_abs_positive() {
    // SymPy: simplify(abs(5)) = 5
    let expr = function!(abs, expr!(5));
    let result = expr.simplify();
    assert_eq!(result, expr!(5));
}

#[test]
fn test_simplify_abs_negative() {
    // SymPy: simplify(abs(-5)) = 5
    let expr = function!(abs, Expression::integer(-5));
    let result = expr.simplify();
    assert_eq!(result, expr!(5));
}

#[test]
fn test_simplify_abs_zero() {
    // SymPy: simplify(abs(0)) = 0
    let expr = function!(abs, expr!(0));
    let result = expr.simplify();
    assert_eq!(result, expr!(0));
}

#[test]
fn test_simplify_distributive_property() {
    // SymPy: simplify(2*(x + 3)) = 2*x + 6 (when expanded)
    let x = symbol!(x);
    let expr = expr!(2 * (x + 3));
    let result = expr.expand();
    let expected = expr!(add: (2 * x), 6);
    assert_eq!(result, expected);
}

#[test]
fn test_simplify_negative_times_positive() {
    // SymPy: simplify(-2 * 3) = -6
    let expr = Expression::mul(vec![Expression::integer(-2), Expression::integer(3)]);
    let result = expr.simplify();
    assert_eq!(result, Expression::integer(-6));
}

#[test]
fn test_simplify_negative_times_negative() {
    // SymPy: simplify(-2 * -3) = 6
    let expr = Expression::mul(vec![Expression::integer(-2), Expression::integer(-3)]);
    let result = expr.simplify();
    assert_eq!(result, expr!(6));
}

#[test]
fn test_simplify_power_rule_multiplication() {
    // SymPy: simplify(x**2 * x**3) = x**5
    let x = symbol!(x);
    let expr = expr!(mul: (x ^ 2), (x ^ 3));
    let result = expr.simplify();
    let expected = expr!(x ^ 5);
    assert_eq!(result, expected);
}

#[test]
fn test_simplify_power_of_power() {
    // SymPy: simplify((x**2)**3) = x**6
    let x = symbol!(x);
    let expr = expr!((x ^ 2) ^ 3);
    let result = expr.simplify();
    let expected = expr!(x ^ 6);
    assert_eq!(result, expected);
}

#[test]
fn test_simplify_complex_nested() {
    // SymPy: simplify(1 + 2 + (3 + 4)) = 10
    let expr = Expression::add(vec![
        Expression::integer(1),
        Expression::integer(2),
        Expression::add(vec![Expression::integer(3), Expression::integer(4)]),
    ]);
    let result = expr.simplify();
    assert_eq!(result, expr!(10));
}

#[test]
fn test_simplify_zero_power() {
    // SymPy: simplify(0**2) = 0
    let expr = expr!(0 ^ 2);
    let result = expr.simplify();
    assert_eq!(result, expr!(0));
}

#[test]
fn test_simplify_one_power() {
    // SymPy: simplify(1**5) = 1
    let expr = expr!(1 ^ 5);
    let result = expr.simplify();
    assert_eq!(result, expr!(1));
}
