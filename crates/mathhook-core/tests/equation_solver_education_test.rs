//! Content validation tests for equation solver educational explanations
//!
//! These tests validate that educational explanations contain specific mathematical
//! content, NOT just that steps exist. This prevents false positives.

use mathhook_core::algebra::solvers::{
    EquationSolver, PolynomialSolver, SystemEquationSolver, SystemSolver,
};
use mathhook_core::educational::step_by_step::StepByStepExplanation;
use mathhook_core::{symbol, Expression};

/// Helper to check if explanation contains specific content
fn has_step_containing(explanation: &StepByStepExplanation, text: &str) -> bool {
    explanation.steps.iter().any(|step| {
        step.description.contains(text) || step.title.contains(text)
    })
}

/// Helper to check if explanation contains any of the given texts
fn has_step_containing_any(explanation: &StepByStepExplanation, texts: &[&str]) -> bool {
    texts.iter().any(|text| has_step_containing(explanation, text))
}

#[test]
fn test_polynomial_rational_root_theorem_shown() {
    let x = symbol!(x);
    let equation = Expression::add(vec![
        Expression::pow(Expression::symbol(x.clone()), Expression::integer(3)),
        Expression::mul(vec![Expression::integer(-6), Expression::pow(Expression::symbol(x.clone()), Expression::integer(2))]),
        Expression::mul(vec![Expression::integer(11), Expression::symbol(x.clone())]),
        Expression::integer(-6),
    ]);

    let solver = PolynomialSolver::new();
    let (_, explanation) = solver.solve_with_explanation(&equation, &x);

    assert!(
        has_step_containing(&explanation, "Rational Root Theorem") ||
        has_step_containing(&explanation, "rational roots") ||
        has_step_containing(&explanation, "candidates"),
        "Should mention Rational Root Theorem or rational root candidates"
    );

    assert!(
        has_step_containing_any(&explanation, &["1", "2", "3", "6", "-1", "-2", "-3", "-6"]),
        "Should show candidate values"
    );
}

#[test]
fn test_polynomial_root_finding_steps() {
    let x = symbol!(x);
    let equation = Expression::add(vec![
        Expression::pow(Expression::symbol(x.clone()), Expression::integer(3)),
        Expression::integer(-8),
    ]);

    let solver = PolynomialSolver::new();
    let (_, explanation) = solver.solve_with_explanation(&equation, &x);

    assert!(
        explanation.steps.len() >= 5,
        "Should have at least 5 steps for polynomial solving"
    );

    assert!(
        has_step_containing(&explanation, "Introduction") ||
        has_step_containing(&explanation, "cubic") ||
        has_step_containing(&explanation, "degree"),
        "Should introduce the polynomial type"
    );
}

#[test]
fn test_polynomial_factorization_explained() {
    let x = symbol!(x);
    let equation = Expression::add(vec![
        Expression::pow(Expression::symbol(x.clone()), Expression::integer(3)),
        Expression::mul(vec![Expression::integer(-6), Expression::pow(Expression::symbol(x.clone()), Expression::integer(2))]),
        Expression::mul(vec![Expression::integer(11), Expression::symbol(x.clone())]),
        Expression::integer(-6),
    ]);

    let solver = PolynomialSolver::new();
    let (_, explanation) = solver.solve_with_explanation(&equation, &x);

    assert!(
        has_step_containing(&explanation, "factor") ||
        has_step_containing(&explanation, "Factor") ||
        has_step_containing(&explanation, "synthetic division") ||
        has_step_containing(&explanation, "Synthetic Division"),
        "Should explain factorization or synthetic division"
    );
}

#[test]
fn test_polynomial_solutions_listed() {
    let x = symbol!(x);
    let equation = Expression::add(vec![
        Expression::pow(Expression::symbol(x.clone()), Expression::integer(3)),
        Expression::integer(-8),
    ]);

    let solver = PolynomialSolver::new();
    let (_, explanation) = solver.solve_with_explanation(&equation, &x);

    assert!(
        has_step_containing(&explanation, "x = ") ||
        has_step_containing(&explanation, "Solution") ||
        has_step_containing(&explanation, "solution"),
        "Should list solutions explicitly"
    );

    assert!(
        has_step_containing(&explanation, "2"),
        "Should find x = 2 as a solution"
    );
}

#[test]
fn test_system_substitution_steps_shown() {
    let x = symbol!(x);
    let y = symbol!(y);

    let eq1 = Expression::add(vec![
        Expression::symbol(x.clone()),
        Expression::mul(vec![Expression::integer(-1), Expression::symbol(y.clone())]),
        Expression::integer(-1),
    ]);

    let eq2 = Expression::add(vec![
        Expression::mul(vec![Expression::integer(2), Expression::symbol(x.clone())]),
        Expression::mul(vec![Expression::integer(3), Expression::symbol(y.clone())]),
        Expression::integer(-13),
    ]);

    let solver = SystemSolver::new();
    let (_, explanation) = solver.solve_system_with_explanation(&[eq1, eq2], &[x, y]);

    assert!(
        has_step_containing(&explanation, "Substitution") ||
        has_step_containing(&explanation, "substitution") ||
        has_step_containing(&explanation, "Isolate") ||
        has_step_containing(&explanation, "isolate"),
        "Should mention substitution method or isolation"
    );

    assert!(
        has_step_containing(&explanation, "Substitute") ||
        has_step_containing(&explanation, "substitute"),
        "Should describe substitution step"
    );

    assert!(
        has_step_containing(&explanation, "Back") ||
        has_step_containing(&explanation, "back"),
        "Should mention back-substitution"
    );
}

#[test]
fn test_system_elimination_steps_shown() {
    let x = symbol!(x);
    let y = symbol!(y);

    let eq1 = Expression::add(vec![
        Expression::mul(vec![Expression::integer(2), Expression::symbol(x.clone())]),
        Expression::mul(vec![Expression::integer(3), Expression::symbol(y.clone())]),
        Expression::integer(-13),
    ]);

    let eq2 = Expression::add(vec![
        Expression::mul(vec![Expression::integer(3), Expression::symbol(x.clone())]),
        Expression::mul(vec![Expression::integer(2), Expression::symbol(y.clone())]),
        Expression::integer(-12),
    ]);

    let solver = SystemSolver::new();
    let (_, explanation) = solver.solve_system_with_explanation(&[eq1, eq2], &[x, y]);

    assert!(
        has_step_containing(&explanation, "Elimination") ||
        has_step_containing(&explanation, "elimination") ||
        has_step_containing(&explanation, "eliminate"),
        "Should mention elimination method"
    );

    assert!(
        has_step_containing(&explanation, "Multiply") ||
        has_step_containing(&explanation, "multiply") ||
        has_step_containing(&explanation, "multiplier"),
        "Should describe multiplying equations"
    );

    assert!(
        has_step_containing(&explanation, "Add") ||
        has_step_containing(&explanation, "Subtract") ||
        has_step_containing(&explanation, "add") ||
        has_step_containing(&explanation, "subtract"),
        "Should describe adding or subtracting equations"
    );
}

#[test]
fn test_system_solution_verified() {
    let x = symbol!(x);
    let y = symbol!(y);

    let eq1 = Expression::add(vec![
        Expression::symbol(x.clone()),
        Expression::mul(vec![Expression::integer(-1), Expression::symbol(y.clone())]),
        Expression::integer(-1),
    ]);

    let eq2 = Expression::add(vec![
        Expression::mul(vec![Expression::integer(2), Expression::symbol(x.clone())]),
        Expression::mul(vec![Expression::integer(3), Expression::symbol(y.clone())]),
        Expression::integer(-13),
    ]);

    let solver = SystemSolver::new();
    let (_, explanation) = solver.solve_system_with_explanation(&[eq1, eq2], &[x, y]);

    assert!(
        has_step_containing(&explanation, "Verify") ||
        has_step_containing(&explanation, "verify") ||
        has_step_containing(&explanation, "Check") ||
        has_step_containing(&explanation, "check") ||
        has_step_containing(&explanation, "satisfied"),
        "Should verify the solution"
    );
}

#[test]
fn test_system_complete_flow() {
    let x = symbol!(x);
    let y = symbol!(y);

    let eq1 = Expression::add(vec![
        Expression::symbol(x.clone()),
        Expression::mul(vec![Expression::integer(-1), Expression::symbol(y.clone())]),
        Expression::integer(-1),
    ]);

    let eq2 = Expression::add(vec![
        Expression::mul(vec![Expression::integer(2), Expression::symbol(x.clone())]),
        Expression::mul(vec![Expression::integer(3), Expression::symbol(y.clone())]),
        Expression::integer(-13),
    ]);

    let solver = SystemSolver::new();
    let (_, explanation) = solver.solve_system_with_explanation(&[eq1, eq2], &[x, y]);

    assert!(
        explanation.steps.len() >= 6,
        "Should have at least 6 steps (introduction, method, solve, solution, verification)"
    );

    assert!(
        has_step_containing(&explanation, "system") ||
        has_step_containing(&explanation, "System"),
        "Should introduce the system"
    );

    assert!(
        has_step_containing(&explanation, "x = ") && has_step_containing(&explanation, "y = "),
        "Should show both x and y solutions"
    );
}

#[test]
fn test_polynomial_verification_step() {
    let x = symbol!(x);
    let equation = Expression::add(vec![
        Expression::pow(Expression::symbol(x.clone()), Expression::integer(3)),
        Expression::integer(-8),
    ]);

    let solver = PolynomialSolver::new();
    let (_, explanation) = solver.solve_with_explanation(&equation, &x);

    assert!(
        has_step_containing(&explanation, "Verif") ||
        has_step_containing(&explanation, "verif") ||
        has_step_containing(&explanation, "Check") ||
        has_step_containing(&explanation, "check"),
        "Should include verification step"
    );
}

#[test]
fn test_polynomial_strategy_explained() {
    let x = symbol!(x);
    let equation = Expression::add(vec![
        Expression::pow(Expression::symbol(x.clone()), Expression::integer(4)),
        Expression::mul(vec![Expression::integer(-5), Expression::pow(Expression::symbol(x.clone()), Expression::integer(2))]),
        Expression::integer(4),
    ]);

    let solver = PolynomialSolver::new();
    let (_, explanation) = solver.solve_with_explanation(&equation, &x);

    assert!(
        has_step_containing(&explanation, "Strategy") ||
        has_step_containing(&explanation, "strategy") ||
        has_step_containing(&explanation, "method") ||
        has_step_containing(&explanation, "Method"),
        "Should explain the solution strategy"
    );

    assert!(
        has_step_containing(&explanation, "quartic") ||
        has_step_containing(&explanation, "degree 4") ||
        has_step_containing(&explanation, "fourth"),
        "Should identify quartic equation"
    );
}
