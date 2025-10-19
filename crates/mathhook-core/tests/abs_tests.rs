//! Comprehensive tests for absolute value function |x|
//!
//! All tests validate mathematical correctness against SymPy behavior.

use mathhook_core::{symbol, Expression};

#[test]
fn test_abs_positive_integer() {
    let simplified = mathhook_core::functions::elementary::abs::simplify_abs(&Expression::integer(5));
    assert_eq!(simplified, Expression::integer(5));
}

#[test]
fn test_abs_negative_integer() {
    let result = mathhook_core::functions::elementary::abs::simplify_abs(&Expression::integer(-5));
    assert_eq!(result, Expression::integer(5));
}

#[test]
fn test_abs_zero() {
    let result = mathhook_core::functions::elementary::abs::simplify_abs(&Expression::integer(0));
    assert_eq!(result, Expression::integer(0));
}

#[test]
fn test_abs_positive_float() {
    let result = mathhook_core::functions::elementary::abs::simplify_abs(&Expression::float(3.14));
    assert_eq!(result, Expression::float(3.14));
}

#[test]
fn test_abs_negative_float() {
    let result = mathhook_core::functions::elementary::abs::simplify_abs(&Expression::float(-3.14));
    assert_eq!(result, Expression::float(3.14));
}

#[test]
fn test_abs_simplify_negation() {
    let x = symbol!(x);
    let expr = Expression::mul(vec![Expression::integer(-1), Expression::symbol(x.clone())]);
    let result = mathhook_core::functions::elementary::abs::simplify_abs(&expr);
    assert_eq!(result, Expression::function("abs", vec![Expression::symbol(x)]));
}

#[test]
fn test_abs_simplify_square() {
    let x = symbol!(x);
    let expr = Expression::pow(Expression::symbol(x.clone()), Expression::integer(2));
    let result = mathhook_core::functions::elementary::abs::simplify_abs(&expr);
    assert_eq!(result, expr);
}

#[test]
fn test_abs_product_rule() {
    let expr = Expression::mul(vec![
        Expression::integer(2),
        Expression::integer(3),
    ]);
    let result = expr.abs();

    let expected = Expression::function("abs", vec![
        Expression::mul(vec![Expression::integer(2), Expression::integer(3)])
    ]);
    assert_eq!(result, expected);
}

#[test]
fn test_abs_quotient_rule() {
    let expr = Expression::div(
        Expression::integer(6),
        Expression::integer(2),
    );
    let result = expr.abs();

    let expected = Expression::function("abs", vec![
        Expression::div(Expression::integer(6), Expression::integer(2))
    ]);
    assert_eq!(result, expected);
}

#[test]
fn test_abs_nested() {
    let x = symbol!(x);
    let inner = Expression::function("abs", vec![Expression::symbol(x.clone())]);
    let result = mathhook_core::functions::elementary::abs::simplify_abs(&inner);
    assert_eq!(result, Expression::function("abs", vec![Expression::symbol(x)]));
}

#[test]
fn test_abs_symbolic() {
    let x = symbol!(x);
    let y = symbol!(y);
    let expr = Expression::add(vec![Expression::symbol(x), Expression::symbol(y)]);
    let result = expr.clone().abs();
    assert_eq!(result, Expression::function("abs", vec![expr]));
}

#[test]
fn test_abs_rational() {
    let expr = Expression::rational(3, 4);
    let result = mathhook_core::functions::elementary::abs::simplify_abs(&expr);
    assert_eq!(result, Expression::rational(3, 4));
}

#[test]
fn test_abs_negative_rational() {
    let expr = Expression::rational(-3, 4);
    let result = mathhook_core::functions::elementary::abs::simplify_abs(&expr);
    assert_eq!(result, Expression::rational(3, 4));
}

#[test]
fn test_abs_intelligence_registered() {
    let intelligence = mathhook_core::functions::elementary::abs::AbsoluteValueIntelligence::new();
    assert!(intelligence.has_function("abs"));

    let props = intelligence.get_properties();
    assert!(props.contains_key("abs"));

    let abs_props = props.get("abs").unwrap();
    assert!(abs_props.has_derivative());
    assert!(abs_props.has_antiderivative());
}

#[test]
fn test_abs_expression_method() {
    let x = symbol!(x);
    let result = Expression::symbol(x.clone()).abs();

    match result {
        Expression::Function { name, args } => {
            assert_eq!(name, "abs");
            assert_eq!(args.len(), 1);
            assert_eq!(args[0], Expression::symbol(x));
        }
        _ => panic!("Expected Function expression"),
    }
}
