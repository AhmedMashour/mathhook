//! Canonical form tests for expression constructors

use crate::core::expression::Expression;

#[test]
fn test_addition_commutativity() {
    let x = Expression::symbol("x");
    let y = Expression::symbol("y");

    let expr1 = Expression::add(vec![x.clone(), y.clone()]);
    let expr2 = Expression::add(vec![y.clone(), x.clone()]);

    assert_eq!(
        expr1, expr2,
        "Addition should be commutative in canonical form"
    );
}

#[test]
fn test_multiplication_commutativity() {
    let x = Expression::symbol("x");
    let y = Expression::symbol("y");

    let expr1 = Expression::mul(vec![x.clone(), y.clone()]);
    let expr2 = Expression::mul(vec![y.clone(), x.clone()]);

    assert_eq!(
        expr1, expr2,
        "Multiplication should be commutative in canonical form"
    );
}

#[test]
fn test_multi_term_commutativity() {
    let x = Expression::symbol("x");
    let y = Expression::symbol("y");
    let z = Expression::symbol("z");

    let expr1 = Expression::add(vec![x.clone(), y.clone(), z.clone()]);
    let expr2 = Expression::add(vec![z.clone(), y.clone(), x.clone()]);

    assert_eq!(
        expr1, expr2,
        "Multi-term addition should have canonical order"
    );
}

#[test]
fn test_addition_identity() {
    let x = Expression::symbol("x");
    let expr = Expression::add(vec![x.clone(), Expression::integer(0)]);

    assert_eq!(expr, x, "Adding zero should return the original expression");
}

#[test]
fn test_multiplication_identity() {
    let x = Expression::symbol("x");
    let expr = Expression::mul(vec![x.clone(), Expression::integer(1)]);

    assert_eq!(
        expr, x,
        "Multiplying by one should return the original expression"
    );
}

#[test]
fn test_power_identity_exponent_one() {
    let x = Expression::symbol("x");
    let expr = Expression::pow(x.clone(), Expression::integer(1));

    assert_eq!(
        expr, x,
        "Raising to power 1 should return the original expression"
    );
}

#[test]
fn test_power_identity_exponent_zero() {
    let x = Expression::symbol("x");
    let expr = Expression::pow(x, Expression::integer(0));

    assert_eq!(
        expr,
        Expression::integer(1),
        "Any expression raised to power 0 should equal 1"
    );
}

#[test]
fn test_power_identity_base_one() {
    let x = Expression::symbol("x");
    let expr = Expression::pow(Expression::integer(1), x);

    assert_eq!(
        expr,
        Expression::integer(1),
        "One raised to any power should equal 1"
    );
}

#[test]
fn test_multiplication_zero() {
    let x = Expression::symbol("x");
    let expr = Expression::mul(vec![x, Expression::integer(0)]);

    assert_eq!(
        expr,
        Expression::integer(0),
        "Multiplying by zero should return zero"
    );
}

#[test]
fn test_addition_associativity_flattening() {
    let x = Expression::symbol("x");
    let y = Expression::symbol("y");
    let z = Expression::symbol("z");

    let inner = Expression::add(vec![x.clone(), y.clone()]);
    let expr = Expression::add(vec![inner, z.clone()]);

    match &expr {
        Expression::Add(terms) => {
            assert_eq!(terms.len(), 3, "Addition should be flattened");
            for term in terms.iter() {
                assert!(!matches!(term, Expression::Add(_)), "No nested Add nodes");
            }
        }
        _ => panic!("Expected Add expression"),
    }
}

#[test]
fn test_multiplication_associativity_flattening() {
    let x = Expression::symbol("x");
    let y = Expression::symbol("y");
    let z = Expression::symbol("z");

    let inner = Expression::mul(vec![x.clone(), y.clone()]);
    let expr = Expression::mul(vec![inner, z.clone()]);

    match &expr {
        Expression::Mul(factors) => {
            assert_eq!(factors.len(), 3, "Multiplication should be flattened");
            for factor in factors.iter() {
                assert!(!matches!(factor, Expression::Mul(_)), "No nested Mul nodes");
            }
        }
        _ => panic!("Expected Mul expression"),
    }
}

#[test]
fn test_addition_constant_folding() {
    let expr = Expression::add(vec![Expression::integer(2), Expression::integer(3)]);

    assert_eq!(
        expr,
        Expression::integer(5),
        "Constant addition should be evaluated"
    );
}

#[test]
fn test_multiplication_constant_folding() {
    let expr = Expression::mul(vec![Expression::integer(2), Expression::integer(3)]);

    assert_eq!(
        expr,
        Expression::integer(6),
        "Constant multiplication should be evaluated"
    );
}

#[test]
fn test_power_constant_folding() {
    let expr = Expression::pow(Expression::integer(2), Expression::integer(3));

    assert_eq!(
        expr,
        Expression::integer(8),
        "Constant power should be evaluated"
    );
}

#[test]
fn test_mixed_constant_and_symbolic() {
    let x = Expression::symbol("x");
    let expr = Expression::add(vec![
        Expression::integer(2),
        Expression::integer(3),
        x.clone(),
    ]);

    match &expr {
        Expression::Add(terms) => {
            assert_eq!(terms.len(), 2, "Constants should be combined");
            assert!(terms.contains(&Expression::integer(5)));
            assert!(terms.contains(&x));
        }
        _ => panic!("Expected Add expression with 2 terms"),
    }
}

#[test]
fn test_constructor_idempotency() {
    let x = Expression::symbol("x");
    let y = Expression::symbol("y");

    let expr1 = Expression::add(vec![x.clone(), y.clone()]);
    let expr2 = Expression::add(vec![expr1.clone()]);

    assert_eq!(expr1, expr2, "Constructor should be idempotent");
}

#[test]
fn test_combining_like_terms() {
    let x = Expression::symbol("x");
    let term1 = Expression::mul(vec![Expression::integer(2), x.clone()]);
    let term2 = Expression::mul(vec![Expression::integer(3), x.clone()]);
    let expr = Expression::add(vec![term1, term2]);

    match &expr {
        Expression::Mul(factors) => {
            assert_eq!(factors.len(), 2, "Should combine like terms");
            assert_eq!(factors[0], Expression::integer(5));
            assert_eq!(factors[1], x);
        }
        _ => panic!("Expected Mul expression for 5x"),
    }
}

#[test]
fn test_div_symbolic() {
    let x = Expression::symbol("x");
    let y = Expression::symbol("y");

    let expr = Expression::div(x.clone(), y.clone());

    match &expr {
        Expression::Mul(factors) => {
            assert_eq!(factors.len(), 2, "Division should be a * b^(-1)");
            assert_eq!(factors[0], x);
            assert!(matches!(factors[1], Expression::Pow(_, _)));
        }
        _ => panic!("Expected Mul expression for division"),
    }
}

#[test]
fn test_div_checked_valid() {
    let result = Expression::div_checked(Expression::integer(10), Expression::integer(2));

    assert!(result.is_ok(), "Valid division should succeed");

    let result = Expression::div_checked(
        Expression::symbol("x"),
        Expression::symbol("y"),
    );

    assert!(
        result.is_ok(),
        "Symbolic division with non-zero denominator should succeed"
    );
}

#[test]
fn test_div_checked_zero_denominator() {
    use crate::error::MathError;

    let result = Expression::div_checked(Expression::integer(1), Expression::integer(0));

    assert!(
        matches!(result, Err(MathError::DivisionByZero)),
        "Division by zero should return DivisionByZero error"
    );

    let result = Expression::div_checked(
        Expression::symbol("x"),
        Expression::integer(0),
    );

    assert!(
        matches!(result, Err(MathError::DivisionByZero)),
        "Division by exact zero should return DivisionByZero error"
    );
}

#[test]
fn test_div_vs_div_checked() {
    let x = Expression::symbol("x");

    let div_result = Expression::div(x.clone(), Expression::integer(0));

    assert!(
        !div_result.is_zero(),
        "div() should succeed even with zero denominator (symbolic context)"
    );

    let div_checked_result =
        Expression::div_checked(x.clone(), Expression::integer(0));

    assert!(
        div_checked_result.is_err(),
        "div_checked() should fail with zero denominator"
    );
}
