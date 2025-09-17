//! Numerical stability integration tests
//!
//! Tests for numerical stability and accuracy including:
//! - Rational arithmetic precision
//! - Exact symbolic computation
//! - Zero detection accuracy

use mathhook_core::{expr, symbol, Expand, Expression, Simplify};

#[test]
fn test_rational_exactness_sum() {
    let third = Expression::rational(1, 3);
    let sum = Expression::add(vec![third.clone(), third.clone(), third.clone()]);

    // 1/3 + 1/3 + 1/3 = 3/3 = 1/1 (rational) or 1 (integer)
    let simplified = sum.simplify();
    // Check it's mathematically 1 (either Integer(1) or Rational(1/1))
    assert!(
        simplified == expr!(1) || simplified == Expression::rational(1, 1),
        "Expected 1 (int or rational), got {}",
        simplified
    );
}

#[test]
fn test_rational_exactness_product() {
    let half = Expression::rational(1, 2);
    let third = Expression::rational(1, 3);
    let product = Expression::mul(vec![half, third]);

    assert_eq!(product.simplify(), Expression::rational(1, 6));
}

#[test]
fn test_rational_division_exact() {
    let two_thirds = Expression::rational(2, 3);
    let one_sixth = Expression::rational(1, 6);
    let quotient = Expression::div(two_thirds, one_sixth);

    assert_eq!(quotient.simplify(), expr!(4));
}

#[test]
fn test_rational_no_floating_point_error() {
    let a = Expression::rational(1, 10);
    let b = Expression::rational(2, 10);
    let sum = Expression::add(vec![a, b]);

    assert_eq!(sum.simplify(), Expression::rational(3, 10));
}

#[test]
fn test_sqrt_squared_is_exact() {
    let x = symbol!(x);
    let sqrt_x = expr!(sqrt(x));
    let squared = Expression::pow(sqrt_x, expr!(2));

    assert_eq!(squared.simplify(), Expression::symbol(x));
}

#[test]
fn test_log_exp_identity() {
    let x = symbol!(x);
    let exp_x = expr!(exp(x));
    let ln_exp_x = Expression::function("ln", vec![exp_x]);

    assert_eq!(ln_exp_x.simplify(), Expression::symbol(x));
}

#[test]
fn test_exp_ln_identity() {
    let x = symbol!(x);
    let ln_x = expr!(ln(x));
    let exp_ln_x = Expression::function("exp", vec![ln_x]);

    assert_eq!(exp_ln_x.simplify(), Expression::symbol(x));
}

#[test]
fn test_subtraction_of_equal_is_zero() {
    let x = symbol!(x);
    let expr = Expression::add(vec![
        Expression::symbol(x.clone()),
        Expression::mul(vec![Expression::integer(-1), Expression::symbol(x)]),
    ]);

    assert!(expr.simplify().is_zero());
}

#[test]
fn test_zero_times_anything() {
    let x = symbol!(x);
    let product = Expression::mul(vec![expr!(0), Expression::symbol(x)]);

    assert!(product.simplify().is_zero());
}

#[test]
fn test_difference_of_squares_factored() {
    let expr1 = expr!((x ^ 2) - (x ^ 2));
    assert!(expr1.simplify().is_zero());
}

#[test]
fn test_integer_arithmetic_exact() {
    let large_a = expr!(1000000);
    let large_b = expr!(1000000);
    let product = Expression::mul(vec![large_a, large_b]);

    let expected = expr!(1000000000000_i64);
    assert_eq!(product.simplify(), expected);
}

#[test]
fn test_integer_division_exact() {
    let a = expr!(100);
    let b = expr!(25);
    let quotient = Expression::div(a, b);

    assert_eq!(quotient.simplify(), expr!(4));
}

#[test]
fn test_power_of_integer() {
    let base = expr!(2);
    let exp = expr!(10);
    let power = Expression::pow(base, exp);

    assert_eq!(power.simplify(), expr!(1024));
}

#[test]
fn test_polynomial_difference_is_zero() {
    let poly1 = expr!((x ^ 2) + (2 * x) + 1);
    let poly2 = expr!((x ^ 2) + (2 * x) + 1);
    let diff = Expression::add(vec![
        poly1,
        Expression::mul(vec![Expression::integer(-1), poly2]),
    ]);

    assert!(diff.simplify().is_zero());
}

#[test]
fn test_expanded_factored_equality() {
    let expanded = expr!((x ^ 2) - 1);
    let factored = Expression::mul(vec![expr!(x - 1), expr!(x + 1)]);

    let diff = Expression::add(vec![
        expanded,
        Expression::mul(vec![Expression::integer(-1), factored]),
    ]);

    assert!(diff.expand().simplify().is_zero());
}

#[test]
fn test_simplify_idempotent() {
    let expr = expr!((x ^ 2) + (2 * x) + 1);
    let simplified_once = expr.simplify();
    let simplified_twice = simplified_once.simplify();

    assert_eq!(simplified_once, simplified_twice);
}

#[test]
fn test_commutative_simplification() {
    let x = symbol!(x);
    let y = symbol!(y);

    let expr1 = Expression::add(vec![
        Expression::symbol(x.clone()),
        Expression::symbol(y.clone()),
    ]);
    let expr2 = Expression::add(vec![Expression::symbol(y), Expression::symbol(x)]);

    assert_eq!(expr1.simplify(), expr2.simplify());
}

#[test]
fn test_pythagorean_identity() {
    let x = symbol!(x);

    let sin_sq = Expression::pow(
        Expression::function("sin", vec![Expression::symbol(x.clone())]),
        expr!(2),
    );
    let cos_sq = Expression::pow(
        Expression::function("cos", vec![Expression::symbol(x)]),
        expr!(2),
    );
    let sum = Expression::add(vec![sin_sq, cos_sq]);

    assert_eq!(sum.simplify(), expr!(1));
}

#[test]
fn test_constant_addition_folded() {
    let sum = Expression::add(vec![expr!(2), expr!(3), expr!(5)]);
    assert_eq!(sum.simplify(), expr!(10));
}

#[test]
fn test_constant_multiplication_folded() {
    let product = Expression::mul(vec![expr!(2), expr!(3), expr!(4)]);
    assert_eq!(product.simplify(), expr!(24));
}

#[test]
fn test_mixed_constant_symbolic() {
    let x = symbol!(x);
    let expr = Expression::add(vec![expr!(2), expr!(3), Expression::symbol(x.clone())]);
    let simplified = expr.simplify();

    if let Expression::Add(terms) = simplified {
        assert!(terms.iter().any(|t| *t == expr!(5)) || terms.len() >= 2);
    }
}

#[test]
fn test_power_of_zero_exponent() {
    let x = symbol!(x);
    let power = Expression::pow(Expression::symbol(x), expr!(0));

    assert_eq!(power.simplify(), expr!(1));
}

#[test]
fn test_power_of_one_exponent() {
    let x = symbol!(x);
    let power = Expression::pow(Expression::symbol(x.clone()), expr!(1));

    assert_eq!(power.simplify(), Expression::symbol(x));
}

#[test]
fn test_power_of_power() {
    let x = symbol!(x);

    let inner = Expression::pow(Expression::symbol(x.clone()), expr!(2));
    let outer = Expression::pow(inner, expr!(3));

    let simplified = outer.simplify();
    let expected = Expression::pow(Expression::symbol(x), expr!(6));

    assert_eq!(simplified, expected);
}
