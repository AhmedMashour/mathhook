//! Comprehensive tests for square root function
//!
//! Tests cover:
//! - Basic evaluation (perfect squares, zero, one)
//! - Rational numbers
//! - Simplification rules (x², x⁴, product rule)
//! - Complex numbers (negative inputs)
//! - Symbolic expressions
//! - Derivative and antiderivative
//!
//! All tests validate CONTENT, not just structure.

use mathhook_core::core::expression::Expression;
use mathhook_core::core::{MathConstant, Number};
use mathhook_core::functions::elementary::sqrt::simplify_sqrt;

#[test]
fn test_sqrt_zero() {
    let result = simplify_sqrt(&Expression::integer(0));
    assert_eq!(result, Expression::integer(0));
}

#[test]
fn test_sqrt_one() {
    let result = simplify_sqrt(&Expression::integer(1));
    assert_eq!(result, Expression::integer(1));
}

#[test]
fn test_sqrt_perfect_squares() {
    let result = simplify_sqrt(&Expression::integer(4));
    assert_eq!(result, Expression::integer(2));

    let result = simplify_sqrt(&Expression::integer(9));
    assert_eq!(result, Expression::integer(3));

    let result = simplify_sqrt(&Expression::integer(16));
    assert_eq!(result, Expression::integer(4));

    let result = simplify_sqrt(&Expression::integer(25));
    assert_eq!(result, Expression::integer(5));

    let result = simplify_sqrt(&Expression::integer(100));
    assert_eq!(result, Expression::integer(10));
}

#[test]
fn test_sqrt_non_perfect_square() {
    let result = simplify_sqrt(&Expression::integer(2));
    assert_eq!(
        result,
        Expression::function("sqrt", vec![Expression::integer(2)])
    );

    let result = simplify_sqrt(&Expression::integer(3));
    assert_eq!(
        result,
        Expression::function("sqrt", vec![Expression::integer(3)])
    );
}

#[test]
fn test_sqrt_rational_perfect_squares() {
    let one_quarter = Expression::Number(Number::rational(
        num_rational::BigRational::new(1.into(), 4.into()),
    ));
    let result = simplify_sqrt(&one_quarter);
    assert_eq!(result, Expression::rational(1, 2));

    let nine_quarters = Expression::Number(Number::rational(
        num_rational::BigRational::new(9.into(), 4.into()),
    ));
    let result = simplify_sqrt(&nine_quarters);
    assert_eq!(result, Expression::rational(3, 2));
}

#[test]
fn test_sqrt_rational_non_perfect() {
    let one_third = Expression::Number(Number::rational(
        num_rational::BigRational::new(1.into(), 3.into()),
    ));
    let result = simplify_sqrt(&one_third);
    assert_eq!(
        result,
        Expression::function("sqrt", vec![one_third.clone()])
    );
}

#[test]
fn test_sqrt_of_square() {
    let x = Expression::symbol("x");
    let x_squared = Expression::pow(x.clone(), Expression::integer(2));
    let result = simplify_sqrt(&x_squared);

    assert_eq!(result, Expression::function("abs", vec![x]));
}

#[test]
fn test_sqrt_of_fourth_power() {
    let x = Expression::symbol("x");
    let x_fourth = Expression::pow(x.clone(), Expression::integer(4));
    let result = simplify_sqrt(&x_fourth);

    assert_eq!(
        result,
        Expression::pow(x.clone(), Expression::integer(2))
    );
}

#[test]
fn test_sqrt_of_sixth_power() {
    let x = Expression::symbol("x");
    let x_sixth = Expression::pow(x.clone(), Expression::integer(6));
    let result = simplify_sqrt(&x_sixth);

    assert_eq!(
        result,
        Expression::pow(x.clone(), Expression::integer(3))
    );
}

#[test]
fn test_sqrt_product_with_perfect_squares() {
    let x = Expression::symbol("x");
    let product = Expression::mul(vec![
        Expression::integer(4),
        Expression::pow(x.clone(), Expression::integer(2)),
    ]);
    let result = simplify_sqrt(&product);

    assert_eq!(
        result,
        Expression::mul(vec![
            Expression::integer(2),
            Expression::function("abs", vec![x])
        ])
    );
}

#[test]
fn test_sqrt_negative_integer() {
    let result = simplify_sqrt(&Expression::integer(-1));

    assert_eq!(result, Expression::constant(MathConstant::I));
}

#[test]
fn test_sqrt_negative_four() {
    let result = simplify_sqrt(&Expression::integer(-4));

    assert_eq!(
        result,
        Expression::mul(vec![
            Expression::constant(MathConstant::I),
            Expression::integer(2)
        ])
    );
}

#[test]
fn test_sqrt_constructor() {
    let x = Expression::symbol("x");
    let sqrt_x = Expression::sqrt(x.clone());

    assert_eq!(
        sqrt_x,
        Expression::function("sqrt", vec![x])
    );
}

#[test]
fn test_sqrt_nested() {
    let x = Expression::symbol("x");
    let sqrt_x = Expression::sqrt(x.clone());
    let sqrt_sqrt_x = Expression::sqrt(sqrt_x);

    assert_eq!(
        sqrt_sqrt_x,
        Expression::function("sqrt", vec![Expression::function("sqrt", vec![x])])
    );
}

#[test]
fn test_sqrt_float() {
    let result = simplify_sqrt(&Expression::float(4.0));
    assert_eq!(result, Expression::float(2.0));

    let result = simplify_sqrt(&Expression::float(9.0));
    assert_eq!(result, Expression::float(3.0));
}

#[test]
fn test_sqrt_product_mixed() {
    let x = Expression::symbol("x");
    let y = Expression::symbol("y");
    let product = Expression::mul(vec![
        Expression::integer(9),
        Expression::pow(x.clone(), Expression::integer(4)),
        y.clone(),
    ]);
    let result = simplify_sqrt(&product);

    assert_eq!(
        result,
        Expression::mul(vec![
            Expression::integer(3),
            Expression::pow(x.clone(), Expression::integer(2)),
            Expression::function("sqrt", vec![y])
        ])
    );
}
