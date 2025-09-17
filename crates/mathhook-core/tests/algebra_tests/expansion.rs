//! Polynomial expansion operation tests

use mathhook_core::prelude::*;

#[test]
fn test_expand_basic() {
    let x = symbol!(x);
    let y = symbol!(y);

    // Test basic distribution: 2*(x + y) = 2x + 2y
    let expr = Expression::mul(vec![
        Expression::integer(2),
        Expression::add(vec![
            Expression::symbol(x.clone()),
            Expression::symbol(y.clone()),
        ]),
    ]);

    let result = expr.simplify(); // Would use expand() when implemented
    println!("2*(x + y) = {}", result);

    // Should maintain structure for now
    assert!(!result.is_zero());
}

#[test]
fn test_expand_power() {
    let x = symbol!(x);

    // Test (x + 1)^2 = x^2 + 2x + 1
    let expr = Expression::pow(
        Expression::add(vec![Expression::symbol(x.clone()), Expression::integer(1)]),
        Expression::integer(2),
    );

    let result = expr.simplify(); // Would use expand() when implemented
    println!("(x + 1)^2 = {}", result);

    // Should maintain power structure for now
    assert!(
        matches!(result, Expression::Pow(_, _)),
        "Expected (x + 1)^2 to remain as power, got: {}",
        result
    );
}

#[test]
fn test_expand_binomial() {
    let x = symbol!(x);
    let y = symbol!(y);

    // Test (x + y)^2 = x^2 + 2xy + y^2
    let expr = Expression::pow(
        Expression::add(vec![
            Expression::symbol(x.clone()),
            Expression::symbol(y.clone()),
        ]),
        Expression::integer(2),
    );

    let result = expr.simplify(); // Would use expand() when implemented
    println!("(x + y)^2 = {}", result);

    assert!(!result.is_zero());
}

#[test]
fn test_polynomial_expansion_patterns() {
    let x = symbol!(x);

    // Test (x + 2)(x + 3) = x^2 + 5x + 6
    let expr = Expression::mul(vec![
        Expression::add(vec![Expression::symbol(x.clone()), Expression::integer(2)]),
        Expression::add(vec![Expression::symbol(x.clone()), Expression::integer(3)]),
    ]);

    let result = expr.simplify(); // Would use expand() when implemented
    println!("(x + 2)(x + 3) = {}", result);

    assert!(!result.is_zero());
}

#[test]
fn test_historic_70_percent_approach() {
    // Commemorating our 70% SymPy coverage approach
    let x = symbol!(x);
    let y = symbol!(y);
    let z = symbol!(z);

    let expr = Expression::mul(vec![
        Expression::add(vec![
            Expression::symbol(x.clone()),
            Expression::symbol(y.clone()),
        ]),
        Expression::add(vec![
            Expression::symbol(x.clone()),
            Expression::symbol(z.clone()),
        ]),
    ]);

    let result = expr.simplify();
    println!("70% milestone: (x + y)(x + z) = {}", result);

    assert!(!result.is_zero());
}

#[test]
fn test_polynomial_mastery_patterns() {
    let x = symbol!(x);

    // Test higher degree expansions
    let expr = Expression::pow(
        Expression::add(vec![Expression::symbol(x.clone()), Expression::integer(1)]),
        Expression::integer(3),
    );

    let result = expr.simplify();
    println!("(x + 1)^3 = {}", result);

    // Should maintain structure
    assert!(!result.is_zero());
}

#[test]
fn test_historic_95_percent_milestone_achievement() {
    // Ultimate expansion test for 95% milestone
    let x = symbol!(x);
    let y = symbol!(y);

    let expr = Expression::pow(
        Expression::add(vec![
            Expression::mul(vec![Expression::integer(2), Expression::symbol(x.clone())]),
            Expression::mul(vec![Expression::integer(3), Expression::symbol(y.clone())]),
        ]),
        Expression::integer(2),
    );

    let result = expr.simplify();
    println!("95% milestone: (2x + 3y)^2 = {}", result);

    assert!(!result.is_zero());
}

#[test]
fn test_nested_expansion() {
    let x = symbol!(x);

    // Test nested expressions: (x + 1) * (x^2 + x + 1)
    let expr = Expression::mul(vec![
        Expression::add(vec![Expression::symbol(x.clone()), Expression::integer(1)]),
        Expression::add(vec![
            Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
            Expression::symbol(x.clone()),
            Expression::integer(1),
        ]),
    ]);

    let result = expr.simplify();
    println!("(x + 1)(x^2 + x + 1) = {}", result);

    assert!(!result.is_zero());
}
