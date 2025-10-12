//! Step-by-step simplification tests

use mathhook_core::prelude::*;

#[test]
fn test_simplify_steps() {
    let x = symbol!(x);

    let expr = Expression::add(vec![
        Expression::integer(2),
        Expression::integer(3),
        Expression::symbol(x.clone()),
    ]);

    let result = expr.simplify();
    println!("Simplify steps: 2 + 3 + x = {}", result);

    assert!(!result.is_zero());
}

#[test]
fn test_detailed_steps() {
    let x = symbol!(x);

    let expr = Expression::mul(vec![
        Expression::add(vec![Expression::symbol(x.clone()), Expression::integer(1)]),
        Expression::integer(2),
    ]);

    let result = expr.simplify();
    println!("Detailed steps: (x + 1) * 2 = {}", result);

    assert!(!result.is_zero());
}

#[test]
fn test_step_verification() {
    let x = symbol!(x);

    let expr = Expression::add(vec![Expression::symbol(x.clone()), Expression::integer(0)]);

    let result = expr.simplify();
    assert_eq!(result, Expression::symbol(x));
}

#[test]
fn test_multi_step_process() {
    let x = symbol!(x);

    let expr = Expression::add(vec![
        Expression::mul(vec![Expression::integer(2), Expression::symbol(x.clone())]),
        Expression::mul(vec![Expression::integer(3), Expression::symbol(x.clone())]),
        Expression::integer(7),
    ]);

    let result = expr.simplify();
    println!("Multi-step: 2x + 3x + 7 = {}", result);

    assert!(!result.is_zero());
}

#[test]
fn test_complex_step_sequence() {
    let x = symbol!(x);

    let expr = Expression::mul(vec![
        Expression::integer(0),
        Expression::add(vec![
            Expression::symbol(x.clone()),
            Expression::integer(100),
        ]),
    ]);

    let result = expr.simplify();
    assert_eq!(result, Expression::integer(0));
}

#[test]
fn test_power_step_sequence() {
    let x = symbol!(x);

    let expr = Expression::pow(Expression::symbol(x.clone()), Expression::integer(1));
    let result = expr.simplify();

    assert_eq!(result, Expression::symbol(x));
}
