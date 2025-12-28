//! Comprehensive content validation tests for limit educational explanations
//!
//! These tests validate that limit educational explanations contain correct
//! mathematical content and follow proper pedagogical structure.

use mathhook_core::calculus::limits::educational::LimitEducation;
use mathhook_core::{symbol, Expression};
use std::sync::Arc;

// Helper function to check if any step contains the target text
fn has_step_containing(
    explanation: &mathhook_core::educational::enhanced_steps::EnhancedStepExplanation,
    target: &str,
) -> bool {
    let target_lower = target.to_lowercase();
    explanation.steps.iter().any(|step| {
        step.human_message.to_lowercase().contains(&target_lower)
            || step.title.to_lowercase().contains(&target_lower)
    })
}

// Helper function to count steps in an explanation
fn count_steps(
    explanation: &mathhook_core::educational::enhanced_steps::EnhancedStepExplanation,
) -> usize {
    explanation.steps.len()
}

#[test]
fn test_direct_substitution_explained() {
    let x = symbol!(x);
    let expr = Expression::pow(Expression::symbol(x.clone()), Expression::integer(2));
    let point = Expression::integer(2);
    let result = Expression::integer(4);

    let explanation = LimitEducation::direct_substitution_explanation(&expr, &x, &point, &result);

    assert!(
        count_steps(&explanation) >= 3,
        "Direct substitution should have at least 3 steps"
    );

    assert!(
        has_step_containing(&explanation, "direct substitution")
            || has_step_containing(&explanation, "substitute"),
        "Should mention direct substitution"
    );

    assert!(
        has_step_containing(&explanation, "2") && has_step_containing(&explanation, "4"),
        "Should show the substitution and result"
    );

    assert!(
        has_step_containing(&explanation, "indeterminate")
            || has_step_containing(&explanation, "well-defined"),
        "Should verify the form is not indeterminate"
    );
}

#[test]
fn test_direct_substitution_polynomial() {
    let x = symbol!(x);
    let expr = Expression::add(vec![
        Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
        Expression::mul(vec![Expression::integer(3), Expression::symbol(x.clone())]),
        Expression::integer(1),
    ]);
    let point = Expression::integer(2);
    let result = Expression::integer(11);

    let explanation = LimitEducation::direct_substitution_explanation(&expr, &x, &point, &result);

    assert!(count_steps(&explanation) >= 3);
    assert!(has_step_containing(&explanation, "substitute"));
    assert!(has_step_containing(&explanation, "11"));
}

#[test]
fn test_indeterminate_form_detected() {
    let x = symbol!(x);
    let numerator = Expression::add(vec![
        Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
        Expression::integer(-1),
    ]);
    let denominator = Expression::add(vec![Expression::symbol(x.clone()), Expression::integer(-1)]);
    let expr = Expression::mul(vec![
        numerator.clone(),
        Expression::pow(denominator.clone(), Expression::integer(-1)),
    ]);
    let point = Expression::integer(1);

    let explanation = LimitEducation::indeterminate_form_explanation(&expr, &x, &point, "0/0");

    assert!(
        count_steps(&explanation) >= 3,
        "Indeterminate form detection should have at least 3 steps"
    );

    assert!(
        has_step_containing(&explanation, "0/0")
            || has_step_containing(&explanation, "indeterminate")
            || has_step_containing(&explanation, "form"),
        "Should identify indeterminate form"
    );

    assert!(
        has_step_containing(&explanation, "factorization")
            || has_step_containing(&explanation, "l'hopital")
            || has_step_containing(&explanation, "resolution"),
        "Should suggest resolution strategy"
    );
}

#[test]
fn test_indeterminate_form_components_evaluated() {
    let x = symbol!(x);
    let numerator = Expression::symbol(x.clone());
    let denominator = Expression::symbol(x.clone());
    // Use raw Mul constructor to bypass simplification (x/x = 1)
    // This preserves the fraction structure for educational analysis
    let expr = Expression::Mul(Arc::new(vec![
        numerator,
        Expression::Pow(Arc::new(denominator), Arc::new(Expression::integer(-1))),
    ]));
    let point = Expression::integer(0);

    let explanation = LimitEducation::indeterminate_form_explanation(&expr, &x, &point, "0/0");

    assert!(count_steps(&explanation) >= 4);
    assert!(
        has_step_containing(&explanation, "numerator")
            && has_step_containing(&explanation, "denominator")
    );
}

#[test]
fn test_lhopital_rule_applied() {
    let x = symbol!(x);
    let numerator = Expression::symbol(x.clone());
    let denominator = Expression::symbol(x.clone());
    let point = Expression::integer(0);

    let explanation =
        LimitEducation::lhopital_rule_explanation(&numerator, &denominator, &x, &point);

    assert!(
        count_steps(&explanation) >= 6,
        "L'Hopital's rule should have at least 6 steps"
    );

    assert!(
        has_step_containing(&explanation, "l'hopital")
            || has_step_containing(&explanation, "l'hospital"),
        "Should mention L'Hopital's rule"
    );

    assert!(
        has_step_containing(&explanation, "differentiate numerator")
            || has_step_containing(&explanation, "derivative of numerator"),
        "Should differentiate numerator"
    );

    assert!(
        has_step_containing(&explanation, "differentiate denominator")
            || has_step_containing(&explanation, "derivative of denominator"),
        "Should differentiate denominator"
    );

    assert!(
        has_step_containing(&explanation, "0/0")
            || has_step_containing(&explanation, "indeterminate"),
        "Should identify indeterminate form"
    );
}

#[test]
fn test_lhopital_rule_complete_process() {
    let x = symbol!(x);
    let numerator = Expression::function("sin", vec![Expression::symbol(x.clone())]);
    let denominator = Expression::symbol(x.clone());
    let point = Expression::integer(0);

    let explanation =
        LimitEducation::lhopital_rule_explanation(&numerator, &denominator, &x, &point);

    assert!(count_steps(&explanation) >= 6);
    assert!(
        has_step_containing(&explanation, "state") || has_step_containing(&explanation, "rule")
    );
    assert!(has_step_containing(&explanation, "apply"));
    assert!(has_step_containing(&explanation, "evaluate"));
}

#[test]
fn test_limit_laws_explained() {
    let x = symbol!(x);
    let expr = Expression::add(vec![
        Expression::mul(vec![
            Expression::integer(3),
            Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
        ]),
        Expression::mul(vec![Expression::integer(2), Expression::symbol(x.clone())]),
    ]);
    let point = Expression::integer(2);

    let explanation = LimitEducation::limit_laws_explanation(&expr, &x, &point);

    assert!(
        count_steps(&explanation) >= 4,
        "Limit laws should have at least 4 steps"
    );

    assert!(
        has_step_containing(&explanation, "sum law")
            || has_step_containing(&explanation, "product law")
            || has_step_containing(&explanation, "constant multiple"),
        "Should mention limit laws"
    );

    assert!(
        has_step_containing(&explanation, "combine")
            || has_step_containing(&explanation, "individual"),
        "Should combine or evaluate individual limits"
    );
}

#[test]
fn test_limit_laws_sum_explained() {
    let x = symbol!(x);
    let expr = Expression::add(vec![Expression::symbol(x.clone()), Expression::integer(5)]);
    let point = Expression::integer(3);

    let explanation = LimitEducation::limit_laws_explanation(&expr, &x, &point);

    assert!(count_steps(&explanation) >= 4);
    assert!(has_step_containing(&explanation, "sum") || has_step_containing(&explanation, "add"));
}

#[test]
fn test_limit_laws_product_explained() {
    let x = symbol!(x);
    let expr = Expression::mul(vec![
        Expression::symbol(x.clone()),
        Expression::add(vec![Expression::symbol(x.clone()), Expression::integer(1)]),
    ]);
    let point = Expression::integer(2);

    let explanation = LimitEducation::limit_laws_explanation(&expr, &x, &point);

    assert!(count_steps(&explanation) >= 4);
    assert!(
        has_step_containing(&explanation, "product")
            || has_step_containing(&explanation, "constant multiple")
            || has_step_containing(&explanation, "factor")
    );
}

#[test]
fn test_limit_at_infinity_technique() {
    let x = symbol!(x);
    let numerator = Expression::add(vec![
        Expression::mul(vec![
            Expression::integer(3),
            Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
        ]),
        Expression::mul(vec![Expression::integer(2), Expression::symbol(x.clone())]),
    ]);
    let denominator = Expression::add(vec![
        Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
        Expression::integer(1),
    ]);
    let expr = Expression::mul(vec![
        numerator,
        Expression::pow(denominator, Expression::integer(-1)),
    ]);

    let explanation = LimitEducation::limit_at_infinity_explanation(&expr, &x);

    assert!(
        count_steps(&explanation) >= 2,
        "Limits at infinity should have at least 2 steps"
    );

    assert!(
        has_step_containing(&explanation, "divide by highest power")
            || has_step_containing(&explanation, "highest power")
            || has_step_containing(&explanation, "dominant")
            || has_step_containing(&explanation, "infinity"),
        "Should mention technique for limits at infinity"
    );

    assert!(
        has_step_containing(&explanation, "infinity"),
        "Should explain behavior as x approaches infinity"
    );
}

#[test]
fn test_limit_at_infinity_polynomial() {
    let x = symbol!(x);
    let expr = Expression::pow(Expression::symbol(x.clone()), Expression::integer(3));

    let explanation = LimitEducation::limit_at_infinity_explanation(&expr, &x);

    assert!(count_steps(&explanation) >= 4);
    assert!(has_step_containing(&explanation, "infinity"));
    assert!(
        has_step_containing(&explanation, "dominant") || has_step_containing(&explanation, "power")
    );
}

#[test]
fn test_all_methods_produce_valid_json() {
    let x = symbol!(x);
    let expr = Expression::pow(Expression::symbol(x.clone()), Expression::integer(2));
    let point = Expression::integer(2);
    let result = Expression::integer(4);

    let direct_explanation =
        LimitEducation::direct_substitution_explanation(&expr, &x, &point, &result);
    assert!(
        direct_explanation.to_json().is_ok(),
        "Direct substitution should produce valid JSON"
    );

    let indet_expr = Expression::mul(vec![
        Expression::symbol(x.clone()),
        Expression::pow(Expression::symbol(x.clone()), Expression::integer(-1)),
    ]);
    let indet_explanation = LimitEducation::indeterminate_form_explanation(
        &indet_expr,
        &x,
        &Expression::integer(0),
        "0/0",
    );
    assert!(
        indet_explanation.to_json().is_ok(),
        "Indeterminate form should produce valid JSON"
    );

    let numerator = Expression::symbol(x.clone());
    let denominator = Expression::symbol(x.clone());
    let lhopital_explanation =
        LimitEducation::lhopital_rule_explanation(&numerator, &denominator, &x, &point);
    assert!(
        lhopital_explanation.to_json().is_ok(),
        "L'Hopital should produce valid JSON"
    );

    let laws_explanation = LimitEducation::limit_laws_explanation(&expr, &x, &point);
    assert!(
        laws_explanation.to_json().is_ok(),
        "Limit laws should produce valid JSON"
    );

    let infinity_explanation = LimitEducation::limit_at_infinity_explanation(&expr, &x);
    assert!(
        infinity_explanation.to_json().is_ok(),
        "Limit at infinity should produce valid JSON"
    );
}

#[test]
fn test_explanations_use_correct_variable_names() {
    let theta = symbol!(theta);
    let expr = Expression::pow(Expression::symbol(theta.clone()), Expression::integer(2));
    let point = Expression::rational(1, 2);
    let result = Expression::rational(1, 4);

    let explanation =
        LimitEducation::direct_substitution_explanation(&expr, &theta, &point, &result);

    assert!(
        has_step_containing(&explanation, "theta"),
        "Should use correct variable name 'theta'"
    );
}

#[test]
fn test_step_ordering_logical() {
    let x = symbol!(x);
    let numerator = Expression::symbol(x.clone());
    let denominator = Expression::symbol(x.clone());
    let point = Expression::integer(0);

    let explanation =
        LimitEducation::lhopital_rule_explanation(&numerator, &denominator, &x, &point);

    let steps = &explanation.steps;
    assert!(steps.len() >= 6);

    assert!(
        steps[0].title.to_lowercase().contains("check")
            || steps[0].title.to_lowercase().contains("condition")
    );

    let has_state_before_apply = steps.windows(2).any(|window| {
        window[0].title.to_lowercase().contains("state")
            && window[1].title.to_lowercase().contains("differentiate")
    });
    assert!(
        has_state_before_apply || steps[1].title.to_lowercase().contains("state"),
        "Should state rule before applying it"
    );
}

#[test]
fn test_all_explanations_have_minimum_steps() {
    let x = symbol!(x);
    let expr = Expression::pow(Expression::symbol(x.clone()), Expression::integer(2));
    let point = Expression::integer(2);
    let result = Expression::integer(4);

    let direct = LimitEducation::direct_substitution_explanation(&expr, &x, &point, &result);
    assert!(
        count_steps(&direct) >= 3,
        "Direct substitution needs 3+ steps"
    );

    let indet_expr = Expression::mul(vec![
        Expression::symbol(x.clone()),
        Expression::pow(Expression::symbol(x.clone()), Expression::integer(-1)),
    ]);
    let indet = LimitEducation::indeterminate_form_explanation(
        &indet_expr,
        &x,
        &Expression::integer(0),
        "0/0",
    );
    assert!(
        count_steps(&indet) >= 3,
        "Indeterminate form needs 3+ steps"
    );

    let numerator = Expression::symbol(x.clone());
    let denominator = Expression::symbol(x.clone());
    let lhopital = LimitEducation::lhopital_rule_explanation(&numerator, &denominator, &x, &point);
    assert!(count_steps(&lhopital) >= 6, "L'Hopital needs 6+ steps");

    let laws = LimitEducation::limit_laws_explanation(&expr, &x, &point);
    assert!(count_steps(&laws) >= 3, "Limit laws need 3+ steps");

    let infinity = LimitEducation::limit_at_infinity_explanation(&expr, &x);
    assert!(
        count_steps(&infinity) >= 2,
        "Limit at infinity needs 2+ steps"
    );
}
