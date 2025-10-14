//! Content validation tests for integration educational explanations
//!
//! These tests verify that educational explanations contain actual mathematical
//! content, not just step counts. Each test checks for specific integration
//! concepts and terminology.

use mathhook_core::calculus::integrals::educational::*;
use mathhook_core::{symbol, Expression};

/// Helper function to check if any step contains a specific substring (case-insensitive)
fn has_step_containing(
    explanation: &mathhook_core::educational::step_by_step::StepByStepExplanation,
    search_term: &str,
) -> bool {
    let search_lower = search_term.to_lowercase();
    explanation.steps.iter().any(|step| {
        step.title.to_lowercase().contains(&search_lower)
            || step.description.to_lowercase().contains(&search_lower)
    })
}

#[test]
fn test_reverse_power_rule_explained() {
    let x = symbol!(x);
    let base = Expression::symbol(x.clone());
    let exponent = Expression::integer(2);

    let explanation = explain_power_rule(&base, &exponent, &x);

    assert!(
        has_step_containing(&explanation, "power rule")
            || has_step_containing(&explanation, "power")
    );
    assert!(
        has_step_containing(&explanation, "n+1")
            || has_step_containing(&explanation, "n + 1")
            || has_step_containing(&explanation, "(n+1)")
    );
    assert!(
        has_step_containing(&explanation, "3")
            || has_step_containing(&explanation, "exponent_plus_one")
    );
}

#[test]
fn test_constant_rule_explained() {
    let x = symbol!(x);
    let constant = Expression::integer(5);

    let explanation = explain_constant_rule(&constant, &x);

    assert!(has_step_containing(&explanation, "constant"));
    assert!(has_step_containing(&explanation, "5") || has_step_containing(&explanation, "k"));
    assert!(has_step_containing(&explanation, "x") || has_step_containing(&explanation, &x.name()));
}

#[test]
fn test_sum_rule_explained() {
    let x = symbol!(x);
    let terms = vec![
        Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
        Expression::mul(vec![Expression::integer(3), Expression::symbol(x.clone())]),
        Expression::integer(5),
    ];

    let explanation = explain_sum_rule(&terms, &x);

    assert!(
        has_step_containing(&explanation, "sum") || has_step_containing(&explanation, "separate")
    );
    assert!(has_step_containing(&explanation, "each") || has_step_containing(&explanation, "term"));
    assert!(
        has_step_containing(&explanation, "combine")
            || has_step_containing(&explanation, "antiderivative")
    );
}

#[test]
fn test_u_substitution_identified() {
    let x = symbol!(x);
    let integrand = Expression::mul(vec![Expression::integer(2), Expression::symbol(x.clone())]);
    let substitution = Expression::pow(Expression::symbol(x.clone()), Expression::integer(2));

    let explanation = explain_u_substitution(&integrand, &substitution, &x);

    assert!(
        has_step_containing(&explanation, "u") || has_step_containing(&explanation, "substitution")
    );
    assert!(
        has_step_containing(&explanation, "du") || has_step_containing(&explanation, "derivative")
    );
    assert!(
        has_step_containing(&explanation, "back")
            || has_step_containing(&explanation, "substitute")
    );
}

#[test]
fn test_u_substitution_shows_steps() {
    let x = symbol!(x);
    let integrand = Expression::mul(vec![Expression::integer(2), Expression::symbol(x.clone())]);
    let substitution = Expression::pow(Expression::symbol(x.clone()), Expression::integer(2));

    let explanation = explain_u_substitution(&integrand, &substitution, &x);

    assert!(
        has_step_containing(&explanation, "inner")
            || has_step_containing(&explanation, "candidate")
    );
    assert!(has_step_containing(&explanation, "find") || has_step_containing(&explanation, "du"));
    assert!(
        has_step_containing(&explanation, "rewrite")
            || has_step_containing(&explanation, "substitute")
    );
    assert!(has_step_containing(&explanation, "integrate"));
}

#[test]
fn test_integration_by_parts_formula() {
    let x = symbol!(x);
    let u_choice = Expression::symbol(x.clone());
    let dv_choice = Expression::function("exp", vec![Expression::symbol(x.clone())]);

    let explanation = explain_integration_by_parts(&u_choice, &dv_choice, &x);

    assert!(
        has_step_containing(&explanation, "parts") || has_step_containing(&explanation, "product")
    );
    assert!(
        has_step_containing(&explanation, "uv") || has_step_containing(&explanation, "formula")
    );
    assert!(has_step_containing(&explanation, "u") && has_step_containing(&explanation, "dv"));
}

#[test]
fn test_integration_by_parts_shows_all_steps() {
    let x = symbol!(x);
    let u_choice = Expression::symbol(x.clone());
    let dv_choice = Expression::function("exp", vec![Expression::symbol(x.clone())]);

    let explanation = explain_integration_by_parts(&u_choice, &dv_choice, &x);

    assert!(has_step_containing(&explanation, "identify"));
    assert!(has_step_containing(&explanation, "formula"));
    assert!(has_step_containing(&explanation, "du") && has_step_containing(&explanation, "v"));
    assert!(has_step_containing(&explanation, "apply"));
    assert!(has_step_containing(&explanation, "remaining"));
    assert!(
        has_step_containing(&explanation, "complete")
            || has_step_containing(&explanation, "solution")
    );
}

#[test]
fn test_definite_integral_bounds() {
    let x = symbol!(x);
    let integrand = Expression::pow(Expression::symbol(x.clone()), Expression::integer(2));
    let lower = Expression::integer(0);
    let upper = Expression::integer(2);

    let explanation = explain_definite_integral(&integrand, &x, &lower, &upper);

    assert!(
        has_step_containing(&explanation, "antiderivative")
            || has_step_containing(&explanation, "F")
    );
    assert!(has_step_containing(&explanation, "upper") || has_step_containing(&explanation, "2"));
    assert!(has_step_containing(&explanation, "lower") || has_step_containing(&explanation, "0"));
    assert!(
        has_step_containing(&explanation, "difference") || has_step_containing(&explanation, "-")
    );
}

#[test]
fn test_definite_integral_fundamental_theorem() {
    let x = symbol!(x);
    let integrand = Expression::pow(Expression::symbol(x.clone()), Expression::integer(2));
    let lower = Expression::integer(0);
    let upper = Expression::integer(2);

    let explanation = explain_definite_integral(&integrand, &x, &lower, &upper);

    assert!(
        has_step_containing(&explanation, "fundamental")
            || has_step_containing(&explanation, "theorem")
            || has_step_containing(&explanation, "calculus")
    );
}

#[test]
fn test_power_rule_mentions_exponent() {
    let x = symbol!(x);
    let base = Expression::symbol(x.clone());
    let exponent = Expression::integer(3);

    let explanation = explain_power_rule(&base, &exponent, &x);

    assert!(
        has_step_containing(&explanation, "3") || has_step_containing(&explanation, "exponent")
    );
    assert!(
        has_step_containing(&explanation, "4")
            || has_step_containing(&explanation, "n+1")
            || has_step_containing(&explanation, "plus")
    );
}

#[test]
fn test_constant_multiple_mentioned() {
    let x = symbol!(x);
    let constant = Expression::integer(7);

    let explanation = explain_constant_rule(&constant, &x);

    assert!(
        has_step_containing(&explanation, "7") || has_step_containing(&explanation, "constant")
    );
}

#[test]
fn test_sum_rule_shows_multiple_integrals() {
    let x = symbol!(x);
    let terms = vec![
        Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
        Expression::symbol(x.clone()),
        Expression::integer(1),
    ];

    let explanation = explain_sum_rule(&terms, &x);

    assert!(has_step_containing(&explanation, "integral"));
    assert!(explanation.steps.len() >= 3);
}

#[test]
fn test_explanations_have_required_minimum_steps() {
    let x = symbol!(x);

    let power_explanation =
        explain_power_rule(&Expression::symbol(x.clone()), &Expression::integer(2), &x);
    assert!(power_explanation.steps.len() >= 3);

    let constant_explanation = explain_constant_rule(&Expression::integer(5), &x);
    assert!(constant_explanation.steps.len() >= 2);

    let sum_explanation = explain_sum_rule(
        &vec![Expression::symbol(x.clone()), Expression::integer(1)],
        &x,
    );
    assert!(sum_explanation.steps.len() >= 3);

    let u_sub_explanation = explain_u_substitution(
        &Expression::mul(vec![Expression::integer(2), Expression::symbol(x.clone())]),
        &Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
        &x,
    );
    assert!(u_sub_explanation.steps.len() >= 6);

    let by_parts_explanation = explain_integration_by_parts(
        &Expression::symbol(x.clone()),
        &Expression::function("exp", vec![Expression::symbol(x.clone())]),
        &x,
    );
    assert!(by_parts_explanation.steps.len() >= 7);

    let definite_explanation = explain_definite_integral(
        &Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
        &x,
        &Expression::integer(0),
        &Expression::integer(2),
    );
    assert!(definite_explanation.steps.len() >= 5);
}
