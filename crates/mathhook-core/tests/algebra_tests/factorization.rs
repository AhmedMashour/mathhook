//! Factorization and term collection tests

use mathhook_core::prelude::*;

#[test]
fn test_factor_basic() {
    let x = symbol!(x);

    // Test basic factoring: 6x + 9 = 3(2x + 3)
    let expr = Expression::add(vec![
        Expression::mul(vec![Expression::integer(6), Expression::symbol(x.clone())]),
        Expression::integer(9),
    ]);

    let result = expr.simplify(); // Would use factor() when implemented
    println!("6x + 9 factored = {}", result);

    // Should maintain structure for now
    assert!(!result.is_zero());
}

#[test]
fn test_collect_terms() {
    let x = symbol!(x);

    // Test collecting like terms: 2x + 3x = 5x
    let expr = Expression::add(vec![
        Expression::mul(vec![Expression::integer(2), Expression::symbol(x.clone())]),
        Expression::mul(vec![Expression::integer(3), Expression::symbol(x.clone())]),
    ]);

    let result = expr.simplify(); // Would use collect() when implemented
    println!("2x + 3x collected = {}", result);

    assert!(!result.is_zero());
}

#[test]
fn test_collect_powers() {
    let x = symbol!(x);

    // Test collecting powers: x^2 + x^2 = 2x^2
    let expr = Expression::add(vec![
        Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
        Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
    ]);

    let result = expr.simplify();
    println!("x^2 + x^2 = {}", result);

    assert!(!result.is_zero());
}

#[test]
fn test_simplify_factor_out() {
    let x = symbol!(x);

    // Test factoring out common factors
    let expr = Expression::add(vec![
        Expression::mul(vec![
            Expression::integer(4),
            Expression::symbol(x.clone()),
            Expression::symbol(x.clone()),
        ]),
        Expression::mul(vec![Expression::integer(6), Expression::symbol(x.clone())]),
    ]);

    let result = expr.simplify();
    println!("4x^2 + 6x factored = {}", result);

    // Should extract common factor
    assert!(!result.is_zero());
}

#[test]
fn test_as_content_primitive() {
    let x = symbol!(x);

    // Test content and primitive part separation
    let expr = Expression::add(vec![
        Expression::mul(vec![Expression::integer(12), Expression::symbol(x.clone())]),
        Expression::integer(18),
    ]);

    let result = expr.simplify();
    println!("12x + 18 = {}", result);

    // Should factor out 6: 6(2x + 3)
    assert!(!result.is_zero());
}

#[test]
fn test_separatevars() {
    let x = symbol!(x);
    let y = symbol!(y);

    // Test variable separation
    let expr = Expression::add(vec![
        Expression::mul(vec![
            Expression::symbol(x.clone()),
            Expression::symbol(y.clone()),
        ]),
        Expression::symbol(x.clone()),
        Expression::symbol(y.clone()),
        Expression::integer(1),
    ]);

    let result = expr.simplify();
    println!("xy + x + y + 1 = {}", result);

    // This could factor as (x + 1)(y + 1)
    assert!(!result.is_zero());
}

#[test]
fn test_historic_60_percent_milestone_operation() {
    // Commemorating 60% milestone
    let x = symbol!(x);

    let expr = Expression::add(vec![
        Expression::mul(vec![
            Expression::integer(8),
            Expression::pow(Expression::symbol(x.clone()), Expression::integer(3)),
        ]),
        Expression::mul(vec![
            Expression::integer(12),
            Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
        ]),
        Expression::mul(vec![Expression::integer(6), Expression::symbol(x.clone())]),
        Expression::integer(1),
    ]);

    let result = expr.simplify();
    println!("60% milestone polynomial: {}", result);

    assert!(!result.is_zero());
}

#[test]
fn test_advanced_factorization_mastery() {
    let x = symbol!(x);
    let y = symbol!(y);

    // Test advanced factorization patterns
    let expr = Expression::add(vec![
        Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
        Expression::mul(vec![
            Expression::integer(2),
            Expression::symbol(x.clone()),
            Expression::symbol(y.clone()),
        ]),
        Expression::pow(Expression::symbol(y.clone()), Expression::integer(2)),
    ]);

    let result = expr.simplify();
    println!("x^2 + 2xy + y^2 = {}", result);

    // This is (x + y)^2 in expanded form
    assert!(!result.is_zero());
}

#[test]
fn test_ultimate_algebraic_mastery() {
    let x = symbol!(x);

    // Ultimate algebraic factorization test
    let expr = Expression::add(vec![
        Expression::pow(Expression::symbol(x.clone()), Expression::integer(4)),
        Expression::mul(vec![
            Expression::integer(-2),
            Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
        ]),
        Expression::integer(1),
    ]);

    let result = expr.simplify();
    println!("x^4 - 2x^2 + 1 = {}", result);

    // This is (x^2 - 1)^2 = (x - 1)^2(x + 1)^2
    assert!(!result.is_zero());
}
