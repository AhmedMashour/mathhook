//! Tests for polynomial educational module

use super::super::super::Expression;
use super::PolynomialEducational;
use crate::symbol;

#[test]
fn test_explain_poly_division() {
    let x = symbol!(x);

    let dividend = Expression::add(vec![
        Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
        Expression::integer(-1),
    ]);
    let divisor = Expression::add(vec![Expression::symbol(x.clone()), Expression::integer(-1)]);

    let explanation = dividend.explain_poly_division(&divisor, &x);

    assert!(!explanation.steps.is_empty());
    assert!(explanation.steps.len() >= 3);
}

#[test]
fn test_explain_poly_gcd_integers() {
    let a = Expression::integer(12);
    let b = Expression::integer(18);

    let explanation = a.explain_poly_gcd(&b);

    assert!(!explanation.steps.is_empty());
    assert_eq!(explanation.final_expression, Expression::integer(6));
}

#[test]
fn test_explain_poly_gcd_with_zero() {
    let x = symbol!(x);
    let a = Expression::symbol(x);
    let b = Expression::integer(0);

    let explanation = a.explain_poly_gcd(&b);

    assert!(!explanation.steps.is_empty());
    assert_eq!(explanation.final_expression, explanation.initial_expression);
}

#[test]
fn test_explain_poly_factorization() {
    let x = symbol!(x);
    let poly = Expression::add(vec![
        Expression::mul(vec![Expression::integer(2), Expression::symbol(x.clone())]),
        Expression::integer(4),
    ]);

    let explanation = poly.explain_poly_factorization(&x);

    assert!(!explanation.steps.is_empty());
}

#[test]
fn test_explain_division_by_zero() {
    let x = symbol!(x);
    let dividend = Expression::symbol(x.clone());
    let divisor = Expression::integer(0);

    let explanation = dividend.explain_poly_division(&divisor, &x);

    let has_error = explanation
        .steps
        .iter()
        .any(|s| s.title.contains("Zero") || s.description.contains("zero"));
    assert!(has_error);
}

#[test]
fn test_explain_gcd_coprime() {
    let a = Expression::integer(7);
    let b = Expression::integer(13);

    let explanation = a.explain_poly_gcd(&b);

    assert_eq!(explanation.final_expression, Expression::integer(1));
}

#[test]
fn test_explain_poly_gcd_shows_algorithm_selection() {
    let x = symbol!(x);
    let poly1 = Expression::add(vec![
        Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
        Expression::symbol(x.clone()),
    ]);
    let poly2 = Expression::symbol(x);

    let explanation = poly1.explain_poly_gcd(&poly2);

    let has_analysis = explanation
        .steps
        .iter()
        .any(|s| s.title.contains("Analyze") || s.title.contains("Characteristics"));
    assert!(has_analysis, "Should have polynomial analysis step");

    let has_selection = explanation
        .steps
        .iter()
        .any(|s| s.title.contains("Algorithm Selection"));
    assert!(has_selection, "Should have algorithm selection step");
}

#[test]
fn test_explain_poly_gcd_high_degree_shows_zippel() {
    let x = symbol!(x);

    let high_deg_poly = Expression::pow(Expression::symbol(x.clone()), Expression::integer(15));
    let other_poly = Expression::pow(Expression::symbol(x), Expression::integer(10));

    let explanation = high_deg_poly.explain_poly_gcd(&other_poly);

    let has_zippel = explanation
        .steps
        .iter()
        .any(|s| s.title.contains("Zippel") || s.description.contains("Zippel"));
    assert!(
        has_zippel,
        "High-degree polynomial should explain Zippel algorithm"
    );
}

#[test]
fn test_explain_poly_gcd_low_degree_shows_euclidean() {
    let x = symbol!(x);
    let poly1 = Expression::add(vec![
        Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
        Expression::integer(1),
    ]);
    let poly2 = Expression::add(vec![Expression::symbol(x), Expression::integer(1)]);

    let explanation = poly1.explain_poly_gcd(&poly2);

    let has_euclidean = explanation
        .steps
        .iter()
        .any(|s| s.title.contains("Euclidean") && !s.title.contains("Zippel"));
    assert!(
        has_euclidean,
        "Low-degree polynomial should explain Euclidean algorithm"
    );
}

#[test]
fn test_explain_selection_rationale_function() {
    use super::super::algorithms::zippel_gcd::educational;

    let rationale = educational::explain_selection_rationale(true, 5, false, false);
    assert!(rationale.contains("Euclidean") || rationale.contains("Classical"));
    assert!(rationale.contains("Univariate: yes"));

    let rationale_high = educational::explain_selection_rationale(true, 15, true, false);
    assert!(rationale_high.contains("Zippel"));
    assert!(rationale_high.contains("sparse"));
}

#[test]
fn test_explain_iteration_step_function() {
    use super::super::algorithms::zippel_gcd::educational;

    let step = educational::explain_iteration_step(1, 65537, 3, false);
    assert!(step.contains("Iteration 1"));
    assert!(step.contains("65537"));
    assert!(step.contains("not yet stabilized"));

    let step_converged = educational::explain_iteration_step(5, 104729, 2, true);
    assert!(step_converged.contains("Iteration 5"));
    assert!(step_converged.contains("CONVERGED"));
}

#[test]
fn test_explain_algorithm_overview() {
    use super::super::algorithms::zippel_gcd::educational;

    let overview = educational::algorithm_overview();
    assert!(overview.contains("Zippel"));
    assert!(overview.contains("CONTENT EXTRACTION"));
    assert!(overview.contains("CRT"));
    assert!(overview.contains("TRIAL DIVISION"));
}
