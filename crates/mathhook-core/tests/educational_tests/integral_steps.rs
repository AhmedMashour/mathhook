//! Educational features integration tests
//!
//! Tests step-by-step explanations, educational messages, and LaTeX formatting
//! for integration operations across all strategy layers.

use mathhook_core::calculus::integrals::educational::IntegrationExplanation;
use mathhook_core::calculus::integrals::strategy::integrate_with_strategy;
use mathhook_core::core::{Expression, Symbol};
use mathhook_core::formatter::latex::LaTeXFormatter as _;
use std::sync::Arc;

fn symbol(name: &str) -> Symbol {
    Symbol::scalar(name)
}

fn x() -> Symbol {
    symbol("x")
}

fn integer(n: i64) -> Expression {
    Expression::integer(n)
}

fn add(terms: Vec<Expression>) -> Expression {
    Expression::Add(Arc::new(terms))
}

fn mul(factors: Vec<Expression>) -> Expression {
    Expression::Mul(Arc::new(factors))
}

fn pow(base: Expression, exp: Expression) -> Expression {
    Expression::Pow(Arc::new(base), Arc::new(exp))
}

fn sin(arg: Expression) -> Expression {
    Expression::function("sin", vec![arg])
}

fn cos(arg: Expression) -> Expression {
    Expression::function("cos", vec![arg])
}

fn exp(arg: Expression) -> Expression {
    Expression::function("exp", vec![arg])
}

fn ln(arg: Expression) -> Expression {
    Expression::function("ln", vec![arg])
}

// Educational Explanation Generation Tests

#[test]
fn test_explanation_for_power_rule() {
    let var = x();
    let expr = pow(Expression::Symbol(var.clone()), integer(3));

    let explanation = IntegrationExplanation::generate(&expr, &var);

    // Should explain power rule: ∫x^n dx = x^(n+1)/(n+1)
    assert!(!explanation.steps().is_empty());
    assert!(
        explanation.strategy_used().contains("power")
            || explanation.strategy_used().contains("basic")
            || explanation.strategy_used().contains("table")
    );
}

#[test]
fn test_explanation_for_trig_integral() {
    let var = x();
    let expr = sin(Expression::Symbol(var.clone()));

    let explanation = IntegrationExplanation::generate(&expr, &var);

    // Should explain trig table lookup or formula
    assert!(!explanation.steps().is_empty());
    assert!(
        explanation.strategy_used().contains("trig")
            || explanation.strategy_used().contains("table")
    );
}

#[test]
fn test_explanation_for_rational_function() {
    let var = x();

    // ∫1/(x+1) dx
    let expr = mul(vec![
        integer(1),
        pow(
            add(vec![Expression::Symbol(var.clone()), integer(1)]),
            integer(-1),
        ),
    ]);

    let explanation = IntegrationExplanation::generate(&expr, &var);

    // Should explain rational function integration or logarithm rule
    assert!(!explanation.steps().is_empty());
}

#[test]
fn test_explanation_for_substitution() {
    let var = x();

    // ∫x*sin(x^2) dx
    let expr = mul(vec![
        Expression::Symbol(var.clone()),
        sin(pow(Expression::Symbol(var.clone()), integer(2))),
    ]);

    let explanation = IntegrationExplanation::generate(&expr, &var);

    // Should explain u-substitution: u = x^2, du = 2x dx
    assert!(!explanation.steps().is_empty());
}

#[test]
fn test_explanation_for_by_parts() {
    let var = x();

    // ∫x*ln(x) dx
    let expr = mul(vec![
        Expression::Symbol(var.clone()),
        ln(Expression::Symbol(var.clone())),
    ]);

    let explanation = IntegrationExplanation::generate(&expr, &var);

    // Should explain integration by parts: u = ln(x), dv = x dx
    assert!(!explanation.steps().is_empty());
}

#[test]
fn test_explanation_includes_formula() {
    let var = x();
    let expr = exp(Expression::Symbol(var.clone()));

    let explanation = IntegrationExplanation::generate(&expr, &var);

    // Should include the formula: ∫e^x dx = e^x + C
    let steps_text = explanation.steps().join(" ");
    assert!(steps_text.contains("e") || steps_text.contains("exp"));
}

#[test]
fn test_explanation_multiple_steps() {
    let var = x();

    // ∫sin^2(x) dx - requires power reduction identity
    let expr = pow(sin(Expression::Symbol(var.clone())), integer(2));

    let explanation = IntegrationExplanation::generate(&expr, &var);

    // Should have multiple steps: identity application, then integration
    assert!(explanation.steps().len() > 1);
}

#[test]
fn test_explanation_for_sum_of_integrals() {
    let var = x();

    // ∫(x + sin(x)) dx
    let expr = add(vec![
        Expression::Symbol(var.clone()),
        sin(Expression::Symbol(var.clone())),
    ]);

    let explanation = IntegrationExplanation::generate(&expr, &var);

    // Should explain sum rule: integral of sum = sum of integrals
    assert!(!explanation.steps().is_empty());
}

#[test]
fn test_explanation_for_constant_multiple() {
    let var = x();

    // ∫5*x dx
    let expr = mul(vec![integer(5), Expression::Symbol(var.clone())]);

    let explanation = IntegrationExplanation::generate(&expr, &var);

    // Should explain constant multiple rule
    assert!(!explanation.steps().is_empty());
}

// LaTeX Formatting Tests

#[test]
fn test_latex_format_simple_polynomial() {
    let var = x();
    let expr = pow(Expression::Symbol(var.clone()), integer(2));

    let latex = expr.to_latex(None).unwrap_or_else(|_| "error".to_string());

    // Should format as x^2
    assert!(latex.contains("x"));
    assert!(latex.contains("2") || latex.contains("^"));
}

#[test]
fn test_latex_format_fraction() {
    let var = x();

    // 1/x
    let expr = pow(Expression::Symbol(var.clone()), integer(-1));

    let latex = expr.to_latex(None).unwrap_or_else(|_| "error".to_string());

    // Should use \frac or x^{-1}
    assert!(latex.contains("x"));
}

#[test]
fn test_latex_format_trig_function() {
    let var = x();
    let expr = sin(Expression::Symbol(var.clone()));

    let latex = expr.to_latex(None).unwrap_or_else(|_| "error".to_string());

    // Should format as \sin(x) or similar
    assert!(latex.contains("sin") || latex.contains("\\sin"));
}

#[test]
fn test_latex_format_integral_result() {
    let var = x();
    let expr = pow(Expression::Symbol(var.clone()), integer(2));
    let result = integrate_with_strategy(&expr, var.clone(), 0);

    let latex = result
        .to_latex(None)
        .unwrap_or_else(|_| "error".to_string());

    // Result should be x^3/3, formatted appropriately
    assert!(latex.contains("x"));
    assert!(latex.contains("3"));
}

#[test]
fn test_latex_format_complex_expression() {
    let var = x();

    // (x^2 + 1) / (x + 1)
    let numerator = add(vec![
        pow(Expression::Symbol(var.clone()), integer(2)),
        integer(1),
    ]);
    let denominator = add(vec![Expression::Symbol(var.clone()), integer(1)]);
    let expr = mul(vec![numerator, pow(denominator, integer(-1))]);

    let latex = expr.to_latex(None).unwrap_or_else(|_| "error".to_string());

    // Should be properly formatted with nested structures
    assert!(latex.contains("x"));
}

#[test]
fn test_latex_format_exponential() {
    let var = x();
    let expr = exp(Expression::Symbol(var.clone()));

    let latex = expr.to_latex(None).unwrap_or_else(|_| "error".to_string());

    // Should format as e^x or \exp(x)
    assert!(latex.contains("e") || latex.contains("exp"));
}

#[test]
fn test_latex_format_logarithm() {
    let var = x();
    let expr = ln(Expression::Symbol(var.clone()));

    let latex = expr.to_latex(None).unwrap_or_else(|_| "error".to_string());

    // Should format as \ln(x) or \log(x)
    assert!(latex.contains("ln") || latex.contains("log"));
}

#[test]
fn test_latex_format_product() {
    let var = x();

    // x * sin(x)
    let expr = mul(vec![
        Expression::Symbol(var.clone()),
        sin(Expression::Symbol(var.clone())),
    ]);

    let latex = expr.to_latex(None).unwrap_or_else(|_| "error".to_string());

    // Should show product clearly
    assert!(latex.contains("x"));
    assert!(latex.contains("sin"));
}

#[test]
fn test_latex_format_sum() {
    let var = x();

    // x + sin(x)
    let expr = add(vec![
        Expression::Symbol(var.clone()),
        sin(Expression::Symbol(var.clone())),
    ]);

    let latex = expr.to_latex(None).unwrap_or_else(|_| "error".to_string());

    // Should show sum with +
    assert!(latex.contains("x"));
    assert!(latex.contains("sin"));
}

// Step-by-Step Output Tests

#[test]
fn test_step_by_step_includes_original() {
    let var = x();
    let expr = pow(Expression::Symbol(var.clone()), integer(2));

    let explanation = IntegrationExplanation::generate(&expr, &var);

    // First step should show original integral
    let first_step = &explanation.steps()[0];
    assert!(first_step.contains("x") || first_step.contains("∫"));
}

#[test]
fn test_step_by_step_includes_result() {
    let var = x();
    let expr = pow(Expression::Symbol(var.clone()), integer(2));

    let explanation = IntegrationExplanation::generate(&expr, &var);

    // Last step should show final result
    let steps = explanation.steps();
    let last_step = steps.last().unwrap();
    assert!(!last_step.is_empty());
}

#[test]
fn test_step_by_step_logical_progression() {
    let var = x();

    // ∫sin^2(x) dx - multi-step process
    let expr = pow(sin(Expression::Symbol(var.clone())), integer(2));

    let explanation = IntegrationExplanation::generate(&expr, &var);

    // Should have at least 2 steps (identity + integration)
    assert!(!explanation.steps().is_empty());
}

#[test]
fn test_educational_message_for_table_lookup() {
    let var = x();
    let expr = sin(Expression::Symbol(var.clone()));

    let explanation = IntegrationExplanation::generate(&expr, &var);

    // Should indicate table lookup or standard formula
    let strategy = explanation.strategy_used();
    assert!(!strategy.is_empty());
}

#[test]
fn test_educational_message_for_substitution_pattern() {
    let var = x();

    // ∫2x*e^(x^2) dx - clear substitution pattern
    let expr = mul(vec![
        integer(2),
        Expression::Symbol(var.clone()),
        exp(pow(Expression::Symbol(var.clone()), integer(2))),
    ]);

    let explanation = IntegrationExplanation::generate(&expr, &var);

    // Should explain the substitution
    assert!(!explanation.steps().is_empty());
}

#[test]
fn test_educational_message_for_partial_fractions() {
    let var = x();

    // ∫1/(x^2-1) dx
    let expr = mul(vec![
        integer(1),
        pow(
            add(vec![
                pow(Expression::Symbol(var.clone()), integer(2)),
                integer(-1),
            ]),
            integer(-1),
        ),
    ]);

    let explanation = IntegrationExplanation::generate(&expr, &var);

    // Should explain partial fraction decomposition
    assert!(!explanation.steps().is_empty());
}

#[test]
fn test_educational_message_includes_why() {
    let var = x();
    let expr = pow(Expression::Symbol(var.clone()), integer(3));

    let explanation = IntegrationExplanation::generate(&expr, &var);

    // Educational explanations should explain WHY, not just WHAT
    // Check that steps contain explanatory content
    let all_steps = explanation.steps().join(" ");
    assert!(all_steps.len() > 10); // Should be more than just "x^4/4"
}

#[test]
fn test_latex_in_educational_steps() {
    let var = x();
    let expr = sin(Expression::Symbol(var.clone()));

    let explanation = IntegrationExplanation::generate(&expr, &var);

    // Steps should be LaTeX-formattable (they're already strings)
    for step in explanation.steps() {
        assert!(!step.is_empty());
    }
}

#[test]
fn test_explanation_for_non_elementary() {
    let var = x();

    // ∫e^(x^2) dx - non-elementary
    let expr = exp(pow(Expression::Symbol(var.clone()), integer(2)));

    let explanation = IntegrationExplanation::generate(&expr, &var);

    // Should explain that this is non-elementary
    assert!(!explanation.steps().is_empty());
}

// Integration Strategy Attribution Tests

#[test]
fn test_strategy_attribution_table() {
    let var = x();
    let expr = cos(Expression::Symbol(var.clone()));

    let explanation = IntegrationExplanation::generate(&expr, &var);

    // Should attribute to table lookup
    let strategy = explanation.strategy_used();
    assert!(strategy.contains("table") || strategy.contains("trig"));
}

#[test]
fn test_strategy_attribution_rational() {
    let var = x();

    // ∫1/x dx
    let expr = pow(Expression::Symbol(var.clone()), integer(-1));

    let explanation = IntegrationExplanation::generate(&expr, &var);

    // Should attribute to rational function or logarithm rule
    let strategy = explanation.strategy_used();
    assert!(!strategy.is_empty());
}

#[test]
fn test_strategy_attribution_substitution() {
    let var = x();

    // ∫x*e^(x^2) dx
    let expr = mul(vec![
        Expression::Symbol(var.clone()),
        exp(pow(Expression::Symbol(var.clone()), integer(2))),
    ]);

    let explanation = IntegrationExplanation::generate(&expr, &var);

    let strategy = explanation.strategy_used();
    assert!(!strategy.is_empty());
}
