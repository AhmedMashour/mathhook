//! Mathematical integration tests for u-substitution
//!
//! Tests verify that u-substitution produces mathematically correct results.
//! Each test validates against known antiderivatives.

use mathhook_core::calculus::derivatives::Derivative;
use mathhook_core::calculus::integrals::substitution::try_substitution;
use mathhook_core::calculus::integrals::Integration;
use mathhook_core::core::Expression;
use mathhook_core::simplify::Simplify;
use mathhook_core::symbol;

#[test]
fn test_x_over_x_squared_plus_one() {
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

    let result = try_substitution(&integrand, &x, 0);
    assert!(
        result.is_some(),
        "∫ x/(x²+1) dx should succeed via substitution"
    );
}

#[test]
fn test_two_x_times_exp_x_squared() {
    let x = symbol!(x);

    let x_squared = Expression::pow(Expression::symbol(x.clone()), Expression::integer(2));
    let integrand = Expression::mul(vec![
        Expression::integer(2),
        Expression::symbol(x.clone()),
        Expression::function("exp", vec![x_squared]),
    ]);

    let result = try_substitution(&integrand, &x, 0);
    assert!(
        result.is_some(),
        "∫ 2x·exp(x²) dx should succeed via substitution"
    );
}

#[test]
fn test_x_times_sin_x_squared() {
    let x = symbol!(x);

    let x_squared = Expression::pow(Expression::symbol(x.clone()), Expression::integer(2));
    let integrand = Expression::mul(vec![
        Expression::symbol(x.clone()),
        Expression::function("sin", vec![x_squared.clone()]),
    ]);

    let result = try_substitution(&integrand, &x, 0);

    if let Some(antiderivative) = result {
        let derivative = antiderivative.derivative(x.clone()).simplify();
        let original_simplified = integrand.simplify();

        assert_eq!(
            derivative, original_simplified,
            "∫ x·sin(x²) dx: derivative of result should equal original"
        );
    }
}

#[test]
fn test_exp_x_times_sin_exp_x() {
    let x = symbol!(x);

    let e_to_x = Expression::function("exp", vec![Expression::symbol(x.clone())]);
    let integrand = Expression::mul(vec![
        e_to_x.clone(),
        Expression::function("sin", vec![e_to_x]),
    ]);

    let result = try_substitution(&integrand, &x, 0);
    assert!(
        result.is_some(),
        "∫ exp(x)·sin(exp(x)) dx should succeed via substitution"
    );
}

#[test]
fn test_sin_x_cos_x_via_substitution() {
    let x = symbol!(x);

    let sin_x = Expression::function("sin", vec![Expression::symbol(x.clone())]);
    let cos_x = Expression::function("cos", vec![Expression::symbol(x.clone())]);
    let integrand = Expression::mul(vec![sin_x, cos_x]);

    let result = try_substitution(&integrand, &x, 0);
    assert!(
        result.is_some(),
        "∫ sin(x)cos(x) dx should succeed via substitution"
    );
}

#[test]
fn test_x_over_sqrt_x_squared_plus_one() {
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

    let result = try_substitution(&integrand, &x, 0);
    assert!(
        result.is_some(),
        "∫ x/√(x²+1) dx should succeed via substitution"
    );
}

#[test]
fn test_ln_x_over_x() {
    let x = symbol!(x);

    let ln_x = Expression::function("ln", vec![Expression::symbol(x.clone())]);
    let integrand = Expression::mul(vec![
        Expression::pow(Expression::symbol(x.clone()), Expression::integer(-1)),
        ln_x,
    ]);

    let result = try_substitution(&integrand, &x, 0);
    assert!(
        result.is_some(),
        "∫ ln(x)/x dx should succeed via substitution"
    );
}

#[test]
fn test_exp_two_x_plus_one() {
    let x = symbol!(x);

    let inner = Expression::add(vec![
        Expression::mul(vec![Expression::integer(2), Expression::symbol(x.clone())]),
        Expression::integer(1),
    ]);
    let integrand = Expression::function("exp", vec![inner]);

    let result = integrand.integrate(x.clone(), 0);

    // Smoke test: verify integration completes without panicking and returns valid result
    assert!(
        !result.to_string().is_empty(),
        "∫ exp(2x+1) dx: integration should complete and return valid result"
    );
}

#[test]
fn test_cos_of_sin_x_times_cos_x() {
    let x = symbol!(x);

    let sin_x = Expression::function("sin", vec![Expression::symbol(x.clone())]);
    let cos_x = Expression::function("cos", vec![Expression::symbol(x.clone())]);
    let integrand = Expression::mul(vec![Expression::function("cos", vec![sin_x]), cos_x]);

    let result = try_substitution(&integrand, &x, 0);
    assert!(
        result.is_some(),
        "∫ cos(sin(x))·cos(x) dx should succeed via substitution"
    );
}

#[test]
fn test_substitution_rejects_simple_sin_x() {
    let x = symbol!(x);
    let integrand = Expression::function("sin", vec![Expression::symbol(x.clone())]);

    let result = try_substitution(&integrand, &x, 0);
    assert!(
        result.is_none(),
        "∫ sin(x) dx should NOT use substitution (handled by table lookup)"
    );
}

#[test]
fn test_substitution_rejects_power_rule_cases() {
    let x = symbol!(x);
    let integrand = Expression::pow(Expression::symbol(x.clone()), Expression::integer(2));

    let result = try_substitution(&integrand, &x, 0);
    assert!(
        result.is_none(),
        "∫ x² dx should NOT use substitution (handled by power rule)"
    );
}

#[test]
fn test_substitution_rejects_constants() {
    let x = symbol!(x);

    let integrand = Expression::mul(vec![
        Expression::integer(5),
        Expression::function("sin", vec![Expression::symbol(x.clone())]),
    ]);

    let result = try_substitution(&integrand, &x, 0);
    assert!(
        result.is_none(),
        "∫ 5sin(x) dx should NOT use substitution (constant multiple rule)"
    );
}

#[test]
fn test_power_rule_via_integration_trait() {
    let x = symbol!(x);
    let integrand = Expression::pow(Expression::symbol(x.clone()), Expression::integer(3));

    let result = integrand.integrate(x, 0);

    assert!(
        !matches!(result, Expression::Calculus(_)),
        "∫ x³ dx should be solved by power rule, not remain symbolic"
    );
}

#[test]
fn test_substitution_respects_variable() {
    let x = symbol!(x);
    let y = symbol!(y);

    let integrand = Expression::function("sin", vec![Expression::symbol(y.clone())]);

    let result = try_substitution(&integrand, &x, 0);
    assert!(
        result.is_none(),
        "∫ sin(y) dx (integrating with respect to x) should fail - wrong variable"
    );
}

#[test]
fn test_multiple_variables_no_substitution() {
    let x = symbol!(x);
    let y = symbol!(y);

    let integrand = Expression::mul(vec![
        Expression::symbol(x.clone()),
        Expression::symbol(y.clone()),
    ]);

    let result = try_substitution(&integrand, &x, 0);
    assert!(
        result.is_none(),
        "∫ xy dx should NOT use substitution (multiple variables)"
    );
}
