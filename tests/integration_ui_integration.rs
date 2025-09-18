//! UI integration tests

use mathhook::prelude::*;

#[test]
fn test_ui_expression_display() {
    let x = Symbol::new("x");
    
    let expr = Expression::add(vec![
        Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
        Expression::symbol(x.clone()),
        Expression::integer(1)
    ]);
    
    let display = format!("{}", expr);
    println!("UI display: {}", display);
    
    assert!(!display.is_empty());
}

#[test]
fn test_ui_latex_output() {
    let x = Symbol::new("x");
    
    let expr = Expression::pow(Expression::symbol(x.clone()), Expression::integer(2));
    
    // Would use to_latex() when implemented
    let display = format!("{}", expr);
    println!("LaTeX output: {}", display);
    
    assert!(!display.is_empty());
}

#[test]
fn test_ui_complex_display() {
    let x = Symbol::new("x");
    let y = Symbol::new("y");
    
    let expr = Expression::add(vec![
        Expression::mul(vec![
            Expression::integer(2),
            Expression::symbol(x.clone()),
            Expression::symbol(y.clone())
        ]),
        Expression::integer(5)
    ]);
    
    let display = format!("{}", expr);
    println!("Complex UI: {}", display);
    
    assert!(!display.is_empty());
}

#[test]
fn test_ui_rational_display() {
    let rational = Expression::number(Number::rational(
        num_rational::BigRational::new(num_bigint::BigInt::from(22), num_bigint::BigInt::from(7))
    ));
    
    let display = format!("{}", rational);
    println!("Rational UI: {}", display);
    
    assert!(!display.is_empty());
}
