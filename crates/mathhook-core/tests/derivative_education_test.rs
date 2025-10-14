//! Content validation tests for derivative educational explanations
//!
//! These tests validate that derivative step-by-step explanations contain
//! actual mathematical content and proper rule application.

use mathhook_core::calculus::derivatives::DerivativeWithSteps;
use mathhook_core::educational::step_by_step::StepByStepExplanation;
use mathhook_core::{expr, symbol, Expression};

/// Helper function to check if explanation contains text (case-insensitive)
fn has_step_containing(explanation: &StepByStepExplanation, text: &str) -> bool {
    let text_lower = text.to_lowercase();
    explanation.steps.iter().any(|step| {
        step.description.to_lowercase().contains(&text_lower)
            || step.title.to_lowercase().contains(&text_lower)
    })
}

/// Helper function to check if multiple terms appear in explanation
fn has_steps_containing_all(explanation: &StepByStepExplanation, texts: &[&str]) -> bool {
    texts.iter().all(|text| has_step_containing(explanation, text))
}

#[test]
fn test_power_rule_explained() {
    let x = symbol!(x);
    let expr = Expression::pow(Expression::symbol(x.clone()), Expression::integer(3));

    let explanation = expr.derivative_with_steps(&x, 1);

    assert!(
        explanation.steps.len() >= 4,
        "Power rule should have at least 4 steps, got {}",
        explanation.steps.len()
    );

    assert!(
        has_step_containing(&explanation, "power"),
        "Must mention 'power' rule"
    );

    assert!(
        has_step_containing(&explanation, "n") || has_step_containing(&explanation, "exponent"),
        "Must mention exponent or variable n"
    );

    assert!(
        has_step_containing(&explanation, "3") && has_step_containing(&explanation, "x"),
        "Must show the actual exponent value 3 and variable x"
    );
}

#[test]
fn test_constant_rule_identifies_constant() {
    let x = symbol!(x);
    let expr = Expression::integer(42);

    let explanation = expr.derivative_with_steps(&x, 1);

    assert!(
        explanation.steps.len() >= 2,
        "Constant rule should have at least 2 steps, got {}",
        explanation.steps.len()
    );

    assert!(
        has_step_containing(&explanation, "constant"),
        "Must identify expression as constant"
    );

    assert_eq!(
        explanation.final_expression,
        Expression::integer(0),
        "Derivative of constant must be 0"
    );
}

#[test]
fn test_variable_rule_identifies_variable() {
    let x = symbol!(x);
    let expr = Expression::symbol(x.clone());

    let explanation = expr.derivative_with_steps(&x, 1);

    assert!(
        explanation.steps.len() >= 2,
        "Variable rule should have at least 2 steps, got {}",
        explanation.steps.len()
    );

    assert!(
        has_step_containing(&explanation, "variable") || has_step_containing(&explanation, "itself"),
        "Must mention differentiating variable with respect to itself"
    );

    assert_eq!(
        explanation.final_expression,
        Expression::integer(1),
        "Derivative of x with respect to x must be 1"
    );
}

#[test]
fn test_sum_rule_identifies_sum() {
    let x = symbol!(x);
    let expr = Expression::add(vec![
        Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
        Expression::mul(vec![Expression::integer(3), Expression::symbol(x.clone())]),
        Expression::integer(-5),
    ]);

    let explanation = expr.derivative_with_steps(&x, 1);

    assert!(
        explanation.steps.len() >= 4,
        "Sum rule should have at least 4 steps, got {}",
        explanation.steps.len()
    );

    assert!(
        has_step_containing(&explanation, "sum") || has_step_containing(&explanation, "term"),
        "Must identify sum or mention terms"
    );

    assert!(
        has_step_containing(&explanation, "separately")
            || has_step_containing(&explanation, "each term")
            || has_step_containing(&explanation, "f' + g'"),
        "Must explain sum rule: differentiate each term separately"
    );
}

#[test]
fn test_chain_rule_identifies_composition() {
    let x = symbol!(x);
    let expr = Expression::function(
        "sin",
        vec![Expression::pow(
            Expression::symbol(x.clone()),
            Expression::integer(2),
        )],
    );

    let explanation = expr.derivative_with_steps(&x, 1);

    assert!(
        explanation.steps.len() >= 5,
        "Chain rule should have at least 5 steps, got {}",
        explanation.steps.len()
    );

    assert!(
        has_step_containing(&explanation, "chain") || has_step_containing(&explanation, "composite"),
        "Must mention chain rule or composite function"
    );

    assert!(
        has_steps_containing_all(&explanation, &["outer", "inner"]),
        "Must identify outer and inner functions"
    );

    assert!(
        has_step_containing(&explanation, "multiply") || has_step_containing(&explanation, "f'(g(x)) * g'(x)"),
        "Must explain multiplying derivatives"
    );
}

#[test]
fn test_product_rule_formula_shown() {
    let x = symbol!(x);
    let expr = Expression::mul(vec![
        Expression::symbol(x.clone()),
        Expression::function("sin", vec![Expression::symbol(x.clone())]),
    ]);

    let explanation = expr.derivative_with_steps(&x, 1);

    assert!(
        explanation.steps.len() >= 5,
        "Product rule should have at least 5 steps, got {}",
        explanation.steps.len()
    );

    assert!(
        has_step_containing(&explanation, "product"),
        "Must mention product rule"
    );

    assert!(
        has_step_containing(&explanation, "f'*g") || has_step_containing(&explanation, "f'g + fg'"),
        "Must show product rule formula"
    );

    assert!(
        has_steps_containing_all(&explanation, &["first", "second"])
            || has_steps_containing_all(&explanation, &["f(x)", "g(x)"]),
        "Must identify the two functions being multiplied"
    );
}

#[test]
fn test_quotient_rule_formula_shown() {
    let x = symbol!(x);
    let expr = Expression::mul(vec![
        Expression::function("sin", vec![Expression::symbol(x.clone())]),
        Expression::pow(Expression::symbol(x.clone()), Expression::integer(-1)),
    ]);

    let explanation = expr.derivative_with_steps(&x, 1);

    assert!(
        explanation.steps.len() >= 6,
        "Quotient rule should have at least 6 steps, got {}",
        explanation.steps.len()
    );

    assert!(
        has_step_containing(&explanation, "quotient") || has_step_containing(&explanation, "numerator"),
        "Must mention quotient or numerator/denominator"
    );

    assert!(
        has_step_containing(&explanation, "f'*g - f*g'")
            || has_step_containing(&explanation, "f'g - fg'")
            || (has_step_containing(&explanation, "numerator") && has_step_containing(&explanation, "denominator")),
        "Must show quotient rule formula or explain numerator/denominator"
    );
}

#[test]
fn test_power_rule_negative_exponent() {
    let x = symbol!(x);
    let expr = Expression::pow(Expression::symbol(x.clone()), Expression::integer(-2));

    let explanation = expr.derivative_with_steps(&x, 1);

    assert!(
        explanation.steps.len() >= 4,
        "Power rule with negative exponent should have at least 4 steps"
    );

    assert!(
        has_step_containing(&explanation, "power"),
        "Must apply power rule"
    );
}

#[test]
fn test_product_rule_differentiates_both_factors() {
    let x = symbol!(x);
    let expr = Expression::mul(vec![
        Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
        Expression::function("cos", vec![Expression::symbol(x.clone())]),
    ]);

    let explanation = expr.derivative_with_steps(&x, 1);

    assert!(
        explanation.steps.len() >= 5,
        "Product rule should differentiate both factors"
    );

    let has_first_deriv = has_step_containing(&explanation, "first")
        || has_step_containing(&explanation, "f'");
    let has_second_deriv = has_step_containing(&explanation, "second")
        || has_step_containing(&explanation, "g'");

    assert!(
        has_first_deriv && has_second_deriv,
        "Must differentiate both factors separately"
    );
}

#[test]
fn test_chain_rule_shows_inner_and_outer_derivatives() {
    let x = symbol!(x);
    let expr = Expression::function(
        "cos",
        vec![Expression::mul(vec![
            Expression::integer(3),
            Expression::symbol(x.clone()),
        ])],
    );

    let explanation = expr.derivative_with_steps(&x, 1);

    assert!(
        explanation.steps.len() >= 5,
        "Chain rule must show both inner and outer derivatives"
    );

    assert!(
        has_step_containing(&explanation, "outer") || has_step_containing(&explanation, "f'"),
        "Must compute outer function derivative"
    );

    assert!(
        has_step_containing(&explanation, "inner") || has_step_containing(&explanation, "g'"),
        "Must compute inner function derivative"
    );
}

#[test]
fn test_sum_rule_shows_individual_term_derivatives() {
    let x = symbol!(x);
    let expr = Expression::add(vec![
        Expression::symbol(x.clone()),
        Expression::integer(5),
        Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
    ]);

    let explanation = expr.derivative_with_steps(&x, 1);

    assert!(
        explanation.steps.len() >= 4,
        "Sum rule should show derivatives of each term"
    );

    assert!(
        has_step_containing(&explanation, "term") || has_step_containing(&explanation, "each"),
        "Must explain differentiating each term"
    );
}

#[test]
fn test_higher_order_derivative_notation() {
    let x = symbol!(x);
    let expr = Expression::pow(Expression::symbol(x.clone()), Expression::integer(4));

    let explanation = expr.derivative_with_steps(&x, 2);

    assert!(
        !explanation.steps.is_empty(),
        "Higher-order derivative must have steps"
    );

    assert!(
        explanation.steps.len() > 1,
        "Second derivative should have multiple steps"
    );
}

#[test]
fn test_constant_multiple_in_derivative() {
    let x = symbol!(x);
    let expr = Expression::mul(vec![
        Expression::integer(5),
        Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
    ]);

    let explanation = expr.derivative_with_steps(&x, 1);

    assert!(
        !explanation.steps.is_empty(),
        "Constant multiple derivative must have steps"
    );

    assert!(
        explanation.final_expression
            != Expression::integer(0),
        "Derivative of 5x^2 should not be zero"
    );
}

#[test]
fn test_explanation_has_latex_output() {
    let x = symbol!(x);
    let expr = Expression::pow(Expression::symbol(x.clone()), Expression::integer(2));

    let explanation = expr.derivative_with_steps(&x, 1);

    let has_latex = explanation
        .steps
        .iter()
        .any(|step| step.latex.is_some());

    assert!(
        has_latex,
        "At least some steps should have LaTeX output"
    );
}

#[test]
fn test_zero_order_derivative_returns_original() {
    let x = symbol!(x);
    let expr = Expression::pow(Expression::symbol(x.clone()), Expression::integer(3));

    let explanation = expr.derivative_with_steps(&x, 0);

    assert_eq!(
        explanation.final_expression, expr,
        "Zero-order derivative should return original function"
    );

    assert!(
        has_step_containing(&explanation, "0th") || has_step_containing(&explanation, "zero"),
        "Must explain zero-order derivative"
    );
}
