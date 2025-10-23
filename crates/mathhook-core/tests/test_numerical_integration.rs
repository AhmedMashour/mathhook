//! Integration tests for numerical methods with SmartEquationSolver
//!
//! Tests numerical equation detection and routing through the public API to verify
//! complete integration between equation analysis and solver dispatch.

use mathhook_core::algebra::equation_analyzer::{EquationAnalyzer, EquationType, SmartEquationSolver};
use mathhook_core::algebra::solvers::SolverResult;
use mathhook_core::{symbol, Expression};

#[test]
fn test_numerical_detection_quintic() {
    let x = symbol!(x);

    // Quintic equation: x^5 - x - 1 = 0
    let quintic = Expression::add(vec![
        Expression::pow(Expression::symbol(x.clone()), Expression::integer(5)),
        Expression::mul(vec![Expression::integer(-1), Expression::symbol(x.clone())]),
        Expression::integer(-1),
    ]);

    // Should be detected as numerical
    assert_eq!(
        EquationAnalyzer::analyze(&quintic, &x),
        EquationType::Numerical,
        "Quintic polynomial should be detected as numerical equation"
    );
}

#[test]
fn test_numerical_detection_transcendental_mixed() {
    let x = symbol!(x);

    // cos(x) - x = 0 (mixed transcendental and polynomial)
    let equation = Expression::add(vec![
        Expression::function("cos", vec![Expression::symbol(x.clone())]),
        Expression::mul(vec![Expression::integer(-1), Expression::symbol(x.clone())]),
    ]);

    // Should be detected as numerical
    assert_eq!(
        EquationAnalyzer::analyze(&equation, &x),
        EquationType::Numerical,
        "Mixed transcendental-polynomial should be detected as numerical"
    );
}

#[test]
fn test_numerical_detection_high_degree() {
    let x = symbol!(x);

    // x^6 + x - 10 = 0 (degree 6 polynomial)
    let high_degree = Expression::add(vec![
        Expression::pow(Expression::symbol(x.clone()), Expression::integer(6)),
        Expression::symbol(x.clone()),
        Expression::integer(-10),
    ]);

    // Should be detected as numerical (degree > 4)
    assert_eq!(
        EquationAnalyzer::analyze(&high_degree, &x),
        EquationType::Numerical,
        "Polynomial degree > 4 should be detected as numerical"
    );
}

#[test]
fn test_numerical_solver_routing() {
    let x = symbol!(x);

    // x^5 - x - 1 = 0
    let quintic = Expression::add(vec![
        Expression::pow(Expression::symbol(x.clone()), Expression::integer(5)),
        Expression::mul(vec![Expression::integer(-1), Expression::symbol(x.clone())]),
        Expression::integer(-1),
    ]);

    let mut solver = SmartEquationSolver::new();
    let (_result, explanation) = solver.solve_with_equation(&quintic, &x);

    // Verify solver routing
    assert!(
        explanation
            .steps
            .iter()
            .any(|s| s.description.contains("numerical methods")
                || s.description.contains("Newton-Raphson")),
        "Should explain numerical method requirement"
    );

    // Verify educational steps include equation analysis
    assert!(
        explanation
            .steps
            .iter()
            .any(|s| s.title == "Equation Analysis"),
        "Should include equation analysis step"
    );

    assert!(
        explanation
            .steps
            .iter()
            .any(|s| s.title == "Solver Selection"),
        "Should include solver selection step"
    );
}

#[test]
fn test_numerical_vs_symbolic_boundary() {
    let x = symbol!(x);

    // Quartic (degree 4) should NOT be numerical (symbolic solver available)
    let quartic = Expression::add(vec![
        Expression::pow(Expression::symbol(x.clone()), Expression::integer(4)),
        Expression::mul(vec![Expression::integer(-1), Expression::symbol(x.clone())]),
        Expression::integer(-1),
    ]);

    assert_eq!(
        EquationAnalyzer::analyze(&quartic, &x),
        EquationType::Quartic,
        "Quartic should use symbolic solver"
    );

    // Quintic (degree 5) should be numerical
    let quintic = Expression::add(vec![
        Expression::pow(Expression::symbol(x.clone()), Expression::integer(5)),
        Expression::mul(vec![Expression::integer(-1), Expression::symbol(x.clone())]),
        Expression::integer(-1),
    ]);

    assert_eq!(
        EquationAnalyzer::analyze(&quintic, &x),
        EquationType::Numerical,
        "Quintic should use numerical solver"
    );
}

#[test]
fn test_transcendental_equation_detection() {
    let x = symbol!(x);

    // Pure transcendental without polynomial should be Transcendental, not Numerical
    let pure_trig = Expression::function("sin", vec![Expression::symbol(x.clone())]);
    assert_eq!(
        EquationAnalyzer::analyze(&pure_trig, &x),
        EquationType::Transcendental,
        "Pure sin(x) should be transcendental"
    );

    // Mixed transcendental-polynomial should be Numerical
    let mixed = Expression::add(vec![
        Expression::function("sin", vec![Expression::symbol(x.clone())]),
        Expression::symbol(x.clone()),
    ]);
    assert_eq!(
        EquationAnalyzer::analyze(&mixed, &x),
        EquationType::Numerical,
        "sin(x) + x should be numerical"
    );
}

#[test]
fn test_numerical_solver_educational_output() {
    let x = symbol!(x);

    // Simple numerical equation
    let equation = Expression::add(vec![
        Expression::pow(Expression::symbol(x.clone()), Expression::integer(5)),
        Expression::integer(-32),
    ]);

    let mut solver = SmartEquationSolver::new();
    let (_result, explanation) = solver.solve_with_equation(&equation, &x);

    // Verify comprehensive educational output
    let step_titles: Vec<&str> = explanation.steps.iter().map(|s| s.title.as_str()).collect();

    assert!(
        step_titles.contains(&"Equation Analysis"),
        "Should include equation analysis"
    );
    assert!(
        step_titles.contains(&"Solver Selection"),
        "Should include solver selection"
    );

    // Should mention numerical methods
    assert!(
        explanation
            .steps
            .iter()
            .any(|s| s.description.contains("numerical")
                || s.description.contains("Newton-Raphson")),
        "Should explain numerical method approach"
    );
}
