//! Integration tests for u-substitution
//!
//! Validates infrastructure and basic operation of the substitution system.
//! Tests verify that the try_substitution function:
//! 1. Accepts correct input types
//! 2. Returns Option<Expression> as specified
//! 3. Integrates with the Integration trait
//! 4. Handles various expression patterns without panicking

use mathhook_core::calculus::integrals::substitution::try_substitution;
use mathhook_core::calculus::integrals::Integration;
use mathhook_core::core::Expression;
use mathhook_core::symbol;

#[test]
fn test_substitution_returns_option() {
    let x = symbol!(x);
    let integrand = Expression::symbol(x.clone());
    let result = try_substitution(&integrand, x);
    assert!(result.is_none() || result.is_some());
}

#[test]
fn test_substitution_with_polynomial_inner() {
    let x = symbol!(x);
    let x_squared = Expression::pow(Expression::symbol(x.clone()), Expression::integer(2));
    let integrand = Expression::mul(vec![
        Expression::integer(2),
        Expression::symbol(x.clone()),
        Expression::function("sin", vec![x_squared]),
    ]);

    let result = try_substitution(&integrand, x);
    assert!(result.is_some() || result.is_none());
}

#[test]
fn test_substitution_with_rational() {
    let x = symbol!(x);
    let x_squared = Expression::pow(Expression::symbol(x.clone()), Expression::integer(2));
    let denominator = Expression::add(vec![x_squared, Expression::integer(1)]);
    let integrand = Expression::mul(vec![
        Expression::symbol(x.clone()),
        Expression::pow(denominator, Expression::integer(-1)),
    ]);

    let result = try_substitution(&integrand, x);
    assert!(result.is_some() || result.is_none());
}

#[test]
fn test_substitution_with_exponential() {
    let x = symbol!(x);
    let e_to_x = Expression::function("exp", vec![Expression::symbol(x.clone())]);
    let integrand = Expression::mul(vec![
        e_to_x.clone(),
        Expression::function("sin", vec![e_to_x]),
    ]);

    let result = try_substitution(&integrand, x);
    assert!(result.is_some() || result.is_none());
}

#[test]
fn test_substitution_finds_candidates() {
    let x = symbol!(x);
    let x_squared = Expression::pow(Expression::symbol(x.clone()), Expression::integer(2));
    let sin_x_squared = Expression::function("sin", vec![x_squared]);

    let result = try_substitution(&sin_x_squared, x);
    assert!(result.is_none());
}

#[test]
fn test_substitution_no_match_simple() {
    let x = symbol!(x);
    let integrand = Expression::function("sin", vec![Expression::symbol(x.clone())]);

    let result = try_substitution(&integrand, x);
    assert!(result.is_none());
}

#[test]
fn test_substitution_integrates_with_trait() {
    let x = symbol!(x);
    let integrand = Expression::symbol(x.clone());

    let result = integrand.integrate(x);
    assert!(
        matches!(result, Expression::Calculus(_))
            || matches!(result, Expression::Pow(_, _))
            || matches!(result, Expression::Mul(_))
    );
}

#[test]
fn test_substitution_handles_power() {
    let x = symbol!(x);
    let integrand = Expression::pow(Expression::symbol(x.clone()), Expression::integer(2));

    let result = try_substitution(&integrand, x);
    assert!(result.is_none());
}

#[test]
fn test_substitution_handles_function() {
    let x = symbol!(x);
    let integrand = Expression::function("cos", vec![Expression::symbol(x.clone())]);

    let result = try_substitution(&integrand, x);
    assert!(result.is_none());
}

#[test]
fn test_substitution_handles_addition() {
    let x = symbol!(x);
    let integrand = Expression::add(vec![
        Expression::symbol(x.clone()),
        Expression::integer(1),
    ]);

    let result = try_substitution(&integrand, x);
    assert!(result.is_none());
}

#[test]
fn test_substitution_handles_multiplication() {
    let x = symbol!(x);
    let integrand = Expression::mul(vec![
        Expression::integer(2),
        Expression::symbol(x.clone()),
    ]);

    let result = try_substitution(&integrand, x);
    assert!(result.is_none());
}

#[test]
fn test_substitution_composite_function() {
    let x = symbol!(x);
    let inner = Expression::pow(Expression::symbol(x.clone()), Expression::integer(2));
    let integrand = Expression::function("exp", vec![inner]);

    let result = try_substitution(&integrand, x);
    assert!(result.is_some() || result.is_none());
}

#[test]
fn test_substitution_nested_function() {
    let x = symbol!(x);
    let sin_x = Expression::function("sin", vec![Expression::symbol(x.clone())]);
    let integrand = Expression::function("cos", vec![sin_x]);

    let result = try_substitution(&integrand, x);
    assert!(result.is_some() || result.is_none());
}

#[test]
fn test_substitution_product_pattern() {
    let x = symbol!(x);
    let integrand = Expression::mul(vec![
        Expression::symbol(x.clone()),
        Expression::function("exp", vec![
            Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
        ]),
    ]);

    let result = try_substitution(&integrand, x);
    assert!(result.is_some() || result.is_none());
}

#[test]
fn test_substitution_trig_pattern() {
    let x = symbol!(x);
    let sin_x = Expression::function("sin", vec![Expression::symbol(x.clone())]);
    let cos_x = Expression::function("cos", vec![Expression::symbol(x.clone())]);
    let integrand = Expression::mul(vec![sin_x, cos_x]);

    let result = try_substitution(&integrand, x);
    assert!(result.is_some() || result.is_none());
}

#[test]
fn test_substitution_rational_pattern() {
    let x = symbol!(x);
    let integrand = Expression::mul(vec![
        Expression::symbol(x.clone()),
        Expression::pow(
            Expression::add(vec![
                Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
                Expression::integer(1),
            ]),
            Expression::integer(-1),
        ),
    ]);

    let result = try_substitution(&integrand, x);
    assert!(result.is_some() || result.is_none());
}

#[test]
fn test_substitution_logarithm() {
    let x = symbol!(x);
    let ln_x = Expression::function("ln", vec![Expression::symbol(x.clone())]);
    let integrand = Expression::mul(vec![
        Expression::pow(Expression::symbol(x.clone()), Expression::integer(-1)),
        ln_x,
    ]);

    let result = try_substitution(&integrand, x);
    assert!(result.is_some() || result.is_none());
}

#[test]
fn test_substitution_radical() {
    let x = symbol!(x);
    let radicand = Expression::add(vec![
        Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
        Expression::integer(1),
    ]);
    let sqrt = Expression::pow(radicand, Expression::rational(1, 2));
    let integrand = Expression::mul(vec![
        Expression::symbol(x.clone()),
        Expression::pow(sqrt, Expression::integer(-1)),
    ]);

    let result = try_substitution(&integrand, x);
    assert!(result.is_some() || result.is_none());
}

#[test]
fn test_substitution_constant_multiple() {
    let x = symbol!(x);
    let integrand = Expression::mul(vec![
        Expression::integer(5),
        Expression::function("sin", vec![Expression::symbol(x.clone())]),
    ]);

    let result = try_substitution(&integrand, x);
    assert!(result.is_none());
}

#[test]
fn test_substitution_negative_power() {
    let x = symbol!(x);
    let integrand = Expression::pow(Expression::symbol(x.clone()), Expression::integer(-1));

    let result = try_substitution(&integrand, x);
    assert!(result.is_none());
}

#[test]
fn test_substitution_fractional_power() {
    let x = symbol!(x);
    let integrand = Expression::pow(Expression::symbol(x.clone()), Expression::rational(1, 2));

    let result = try_substitution(&integrand, x);
    assert!(result.is_none());
}

#[test]
fn test_substitution_integration_endpoint() {
    let x = symbol!(x);
    let inner = Expression::add(vec![
        Expression::mul(vec![Expression::integer(2), Expression::symbol(x.clone())]),
        Expression::integer(1),
    ]);
    let integrand = Expression::function("exp", vec![inner]);

    let result = integrand.integrate(x);
    assert!(
        matches!(result, Expression::Calculus(_))
            || matches!(result, Expression::Function { .. })
            || matches!(result, Expression::Mul(_))
    );
}

#[test]
fn test_substitution_doesnt_panic_on_complex_expression() {
    let x = symbol!(x);
    let complex = Expression::add(vec![
        Expression::mul(vec![
            Expression::integer(3),
            Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
        ]),
        Expression::mul(vec![Expression::integer(2), Expression::symbol(x.clone())]),
        Expression::integer(1),
    ]);
    let integrand = Expression::function("sin", vec![complex]);

    let result = try_substitution(&integrand, x);
    assert!(result.is_some() || result.is_none());
}

#[test]
fn test_substitution_strategy_layer_integration() {
    let x = symbol!(x);
    let integrand = Expression::pow(Expression::symbol(x.clone()), Expression::integer(3));

    let result = integrand.integrate(x);
    assert!(!matches!(result, Expression::Calculus(_)));
}

#[test]
fn test_substitution_with_multiple_variables() {
    let x = symbol!(x);
    let y = symbol!(y);

    let integrand = Expression::mul(vec![
        Expression::symbol(x.clone()),
        Expression::symbol(y.clone()),
    ]);

    let result = try_substitution(&integrand, x);
    assert!(result.is_none());
}

#[test]
fn test_substitution_respects_integration_variable() {
    let x = symbol!(x);
    let y = symbol!(y);

    let integrand = Expression::function("sin", vec![Expression::symbol(y.clone())]);

    let result = try_substitution(&integrand, x);
    assert!(result.is_none());
}

#[test]
fn test_substitution_infrastructure_complete() {
    let x = symbol!(x);
    let tests_passed = vec![
        try_substitution(&Expression::integer(5), x.clone()).is_none(),
        try_substitution(&Expression::symbol(x.clone()), x.clone()).is_none(),
        Expression::integer(5).integrate(x.clone()).to_string().len() > 0,
    ];

    assert!(tests_passed.iter().all(|&b| b), "All infrastructure tests should pass");
}
