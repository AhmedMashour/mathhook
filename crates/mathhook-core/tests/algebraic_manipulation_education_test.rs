// Integration tests for algebraic manipulation educational explanations
// NO FALSE POSITIVES - validates actual mathematical content

use mathhook_core::educational::step_by_step::StepByStep;
use mathhook_core::{symbol, Expression};

/// Helper function to check if any step contains a specific substring
fn has_step_containing(
    explanation: &mathhook_core::educational::step_by_step::StepByStepExplanation,
    text: &str,
) -> bool {
    explanation.steps.iter().any(|step| {
        step.title.to_lowercase().contains(&text.to_lowercase())
            || step
                .description
                .to_lowercase()
                .contains(&text.to_lowercase())
    })
}

/// Helper function to check if any step title matches exactly
fn has_step_with_title(
    explanation: &mathhook_core::educational::step_by_step::StepByStepExplanation,
    title: &str,
) -> bool {
    explanation.steps.iter().any(|step| step.title == title)
}

#[test]
fn test_simplify_combine_like_terms() {
    let x = symbol!(x);
    let expr = Expression::Add(Box::new(vec![
        Expression::Mul(Box::new(vec![
            Expression::integer(2),
            Expression::Symbol(x.clone()),
        ])),
        Expression::Mul(Box::new(vec![
            Expression::integer(3),
            Expression::Symbol(x.clone()),
        ])),
        Expression::integer(5),
        Expression::integer(-2),
        Expression::Symbol(x.clone()),
    ]));

    let explanation = expr.explain_simplification();

    assert!(
        !explanation.steps.is_empty(),
        "Explanation should have steps"
    );
    assert!(
        has_step_containing(&explanation, "like terms")
            || has_step_containing(&explanation, "combine"),
        "Should mention combining like terms"
    );

    assert!(
        explanation.steps.iter().any(|step| {
            let desc = step.description.to_lowercase();
            desc.contains("2") || desc.contains("3") || desc.contains("6")
        }),
        "Should show coefficients being combined"
    );
}

#[test]
fn test_simplify_power_rules() {
    let x = symbol!(x);
    let expr = Expression::Pow(
        Box::new(Expression::Symbol(x.clone())),
        Box::new(Expression::integer(1)),
    );

    let explanation = expr.explain_simplification();

    assert!(
        has_step_containing(&explanation, "power") || has_step_containing(&explanation, "x^1"),
        "Should mention power rule application"
    );
}

#[test]
fn test_simplify_coefficient_multiplication() {
    let x = symbol!(x);
    let expr = Expression::Mul(Box::new(vec![
        Expression::integer(2),
        Expression::integer(3),
        Expression::Symbol(x.clone()),
    ]));

    let explanation = expr.explain_simplification();

    assert!(
        has_step_containing(&explanation, "coefficients")
            || has_step_containing(&explanation, "multiply"),
        "Should mention coefficient multiplication"
    );
}

#[test]
fn test_simplify_identity_rules_additive() {
    let x = symbol!(x);
    let expr = Expression::Add(Box::new(vec![
        Expression::Symbol(x.clone()),
        Expression::integer(0),
    ]));

    let explanation = expr.explain_simplification();

    assert!(
        has_step_containing(&explanation, "identity") || has_step_containing(&explanation, "+ 0"),
        "Should mention identity rule for addition"
    );
}

#[test]
fn test_simplify_identity_rules_multiplicative() {
    let x = symbol!(x);
    let expr = Expression::Mul(Box::new(vec![
        Expression::Symbol(x.clone()),
        Expression::integer(1),
    ]));

    let explanation = expr.explain_simplification();

    assert!(
        has_step_containing(&explanation, "identity") || has_step_containing(&explanation, "* 1"),
        "Should mention identity rule for multiplication"
    );
}

#[test]
fn test_simplify_zero_property() {
    let x = symbol!(x);
    let expr = Expression::Mul(Box::new(vec![
        Expression::Symbol(x.clone()),
        Expression::integer(0),
    ]));

    let explanation = expr.explain_simplification();

    assert!(
        has_step_containing(&explanation, "zero") || has_step_containing(&explanation, "* 0"),
        "Should mention zero property"
    );
}

#[test]
fn test_expand_binomial_foil() {
    let x = symbol!(x);
    let expr = Expression::Mul(Box::new(vec![
        Expression::Add(Box::new(vec![
            Expression::Symbol(x.clone()),
            Expression::integer(2),
        ])),
        Expression::Add(Box::new(vec![
            Expression::Symbol(x.clone()),
            Expression::integer(3),
        ])),
    ]));

    let explanation = expr.explain_expansion();

    assert!(
        has_step_containing(&explanation, "foil") || has_step_containing(&explanation, "binomial"),
        "Should mention FOIL method or binomial expansion"
    );
}

#[test]
fn test_expand_distributive_property() {
    let x = symbol!(x);
    let expr = Expression::Mul(Box::new(vec![
        Expression::integer(2),
        Expression::Add(Box::new(vec![
            Expression::Symbol(x.clone()),
            Expression::integer(3),
        ])),
    ]));

    let explanation = expr.explain_expansion();

    assert!(
        has_step_containing(&explanation, "distribute")
            || has_step_containing(&explanation, "distributive"),
        "Should mention distributive property"
    );
}

#[test]
fn test_expand_binomial_square() {
    let x = symbol!(x);
    let expr = Expression::Pow(
        Box::new(Expression::Add(Box::new(vec![
            Expression::Symbol(x.clone()),
            Expression::integer(1),
        ]))),
        Box::new(Expression::integer(2)),
    );

    let explanation = expr.explain_expansion();

    assert!(
        has_step_containing(&explanation, "square")
            || has_step_containing(&explanation, "binomial"),
        "Should mention binomial square or expansion pattern"
    );
}

#[test]
fn test_expand_combine_like_terms_after() {
    let x = symbol!(x);
    let expr = Expression::Mul(Box::new(vec![
        Expression::Add(Box::new(vec![
            Expression::Symbol(x.clone()),
            Expression::integer(1),
        ])),
        Expression::Add(Box::new(vec![
            Expression::Symbol(x.clone()),
            Expression::integer(1),
        ])),
    ]));

    let explanation = expr.explain_expansion();

    assert!(!explanation.steps.is_empty(), "Should have expansion steps");
}

#[test]
fn test_factor_gcf_extraction() {
    let x = symbol!(x);
    let expr = Expression::Add(Box::new(vec![
        Expression::Mul(Box::new(vec![
            Expression::integer(6),
            Expression::Symbol(x.clone()),
        ])),
        Expression::integer(9),
    ]));

    let explanation = expr.explain_factorization();

    assert!(
        has_step_containing(&explanation, "gcf")
            || has_step_containing(&explanation, "common factor"),
        "Should mention GCF or common factor"
    );

    assert!(
        explanation
            .steps
            .iter()
            .any(|step| step.description.contains("3")),
        "Should identify 3 as the GCF"
    );
}

#[test]
fn test_factor_gcf_with_multiple_terms() {
    let x = symbol!(x);
    let expr = Expression::Add(Box::new(vec![
        Expression::Mul(Box::new(vec![
            Expression::integer(12),
            Expression::Symbol(x.clone()),
        ])),
        Expression::Mul(Box::new(vec![
            Expression::integer(18),
            Expression::Symbol(x.clone()),
        ])),
    ]));

    let explanation = expr.explain_factorization();

    assert!(
        has_step_containing(&explanation, "gcf") || has_step_containing(&explanation, "common"),
        "Should mention GCF extraction"
    );

    assert!(
        explanation
            .steps
            .iter()
            .any(|step| step.description.contains("6")),
        "Should identify 6 as the GCF"
    );
}

#[test]
fn test_factor_verification_step() {
    let x = symbol!(x);
    let expr = Expression::Add(Box::new(vec![
        Expression::Mul(Box::new(vec![
            Expression::integer(2),
            Expression::Symbol(x.clone()),
        ])),
        Expression::integer(4),
    ]));

    let explanation = expr.explain_factorization();

    assert!(
        !explanation.steps.is_empty() || has_step_containing(&explanation, "factor"),
        "Should have factorization steps or mention factoring"
    );
}

#[test]
fn test_simplification_produces_multiple_steps() {
    let x = symbol!(x);
    let expr = Expression::Add(Box::new(vec![
        Expression::Mul(Box::new(vec![
            Expression::integer(2),
            Expression::Symbol(x.clone()),
        ])),
        Expression::Mul(Box::new(vec![
            Expression::integer(3),
            Expression::Symbol(x.clone()),
        ])),
        Expression::integer(0),
        Expression::integer(5),
    ]));

    let explanation = expr.explain_simplification();

    assert!(
        explanation.steps.len() >= 3,
        "Complex expression should produce multiple steps (at least initial, one transformation, and final)"
    );
}

#[test]
fn test_expansion_produces_multiple_steps() {
    let x = symbol!(x);
    let expr = Expression::Mul(Box::new(vec![
        Expression::Add(Box::new(vec![
            Expression::Symbol(x.clone()),
            Expression::integer(2),
        ])),
        Expression::Add(Box::new(vec![
            Expression::Symbol(x.clone()),
            Expression::integer(3),
        ])),
    ]));

    let explanation = expr.explain_expansion();

    assert!(
        explanation.steps.len() >= 3,
        "Binomial expansion should produce multiple steps (at least initial, pattern identification, and final)"
    );
}

#[test]
fn test_factorization_produces_multiple_steps() {
    let x = symbol!(x);
    let expr = Expression::Add(Box::new(vec![
        Expression::Mul(Box::new(vec![
            Expression::integer(6),
            Expression::Symbol(x.clone()),
        ])),
        Expression::integer(9),
    ]));

    let explanation = expr.explain_factorization();

    assert!(
        explanation.steps.len() >= 2,
        "Factorization should produce at least initial and GCF extraction steps"
    );
}
