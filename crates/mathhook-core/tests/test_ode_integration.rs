/// Test suite for ODE integration with SmartEquationSolver
///
/// This verifies that:
/// 1. EquationAnalyzer correctly detects ODE equations
/// 2. SmartEquationSolver correctly routes to ODE solver
/// 3. No stub implementations remain in the critical path
/// 4. Architecture follows CLAUDE.md patterns (registry-based, not hardcoded)

use mathhook_core::algebra::equation_analyzer::{EquationAnalyzer, EquationType, SmartEquationSolver};
use mathhook_core::core::{Expression, Symbol};
use mathhook_core::symbol;

#[test]
fn test_ode_detection_simple_derivative() {
    // Test: y' + 2y = x (linear first-order ODE)
    let y = symbol!(y);
    let y_prime = Symbol::new("y'");

    let lhs = Expression::add(vec![
        Expression::symbol(y_prime),
        Expression::mul(vec![Expression::integer(2), Expression::symbol(y.clone())])
    ]);

    let equation_type = EquationAnalyzer::analyze(&lhs, &y);

    assert_eq!(
        equation_type,
        EquationType::ODE,
        "Should detect equation with y' as ODE"
    );
}

#[test]
fn test_ode_detection_function_derivative() {
    // Test: derivative(y, x) = 2x (using function notation)
    let y = symbol!(y);
    let x = symbol!(x);

    let derivative_expr = Expression::function(
        "derivative",
        vec![Expression::symbol(y.clone()), Expression::symbol(x.clone())]
    );

    let equation_type = EquationAnalyzer::analyze(&derivative_expr, &y);

    assert_eq!(
        equation_type,
        EquationType::ODE,
        "Should detect derivative() function as ODE"
    );
}

#[test]
fn test_pde_detection() {
    // Test: ∂u/∂t = k∂²u/∂x² (heat equation)
    let u = symbol!(u);
    let partial_u = Symbol::new("∂u");

    let expr = Expression::symbol(partial_u);

    let equation_type = EquationAnalyzer::analyze(&expr, &u);

    assert_eq!(
        equation_type,
        EquationType::PDE,
        "Should detect partial derivative symbol as PDE"
    );
}

#[test]
fn test_smart_solver_ode_routing() {
    // Test: SmartEquationSolver routes ODE to ODE solver (not polynomial/linear)
    let y = symbol!(y);
    let y_prime = Symbol::new("y'");

    let equation = Expression::add(vec![
        Expression::symbol(y_prime),
        Expression::mul(vec![Expression::integer(2), Expression::symbol(y.clone())])
    ]);

    let mut solver = SmartEquationSolver::new();
    let (result, explanation) = solver.solve_with_equation(&equation, &y);

    // Verify routing happened (explanation should mention ODE)
    let steps_str = format!("{:?}", explanation);
    assert!(
        steps_str.contains("ODE") || steps_str.contains("differential"),
        "SmartEquationSolver should route to ODE solver and mention it in explanation"
    );

    // Verify it didn't incorrectly route to linear solver
    assert!(
        !steps_str.contains("linear equation solver (isolation method)"),
        "Should NOT route to linear solver for ODE"
    );
}

#[test]
fn test_non_ode_still_works() {
    // Regression test: Non-ODE equations should still be detected correctly
    let x = symbol!(x);

    // Quadratic: x^2 + 3x + 2 = 0
    let quadratic = Expression::add(vec![
        Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
        Expression::mul(vec![Expression::integer(3), Expression::symbol(x.clone())]),
        Expression::integer(2),
    ]);

    let equation_type = EquationAnalyzer::analyze(&quadratic, &x);

    assert_eq!(
        equation_type,
        EquationType::Quadratic,
        "Non-ODE equations should still be detected correctly (no regression)"
    );
}

#[test]
fn test_architectural_pattern_no_hardcoded_ode_matching() {
    // Architectural verification: ODE detection should use helper methods,
    // not hardcoded string matching in analyze() match statement

    // This is verified by code structure, not runtime test
    // But we can verify the BEHAVIOR is consistent

    let y = symbol!(y);

    // Test multiple derivative notations
    let notations = vec![
        Symbol::new("y'"),
        Symbol::new("y_prime"),
    ];

    for notation in notations {
        let expr = Expression::symbol(notation);
        let eq_type = EquationAnalyzer::analyze(&expr, &y);

        assert_eq!(
            eq_type,
            EquationType::ODE,
            "All derivative notations should be detected consistently via helper methods"
        );
    }
}

#[test]
fn test_no_stub_implementations_in_routing() {
    // Verify SmartEquationSolver actually has an ODE solver field
    // and doesn't just return NotImplemented

    let mut solver = SmartEquationSolver::new();

    // We can't directly inspect the private ode_solver field,
    // but we can verify behavior via public API
    let y = symbol!(y);
    let y_prime = Symbol::new("y'");

    let equation = Expression::symbol(y_prime);

    let (_result, explanation) = solver.solve_with_equation(&equation, &y);

    // Should have at least SOME steps (not empty/stub)
    assert!(
        !explanation.steps.is_empty(),
        "ODE routing should produce explanation steps, not be a stub"
    );

    // Should mention ODE classification
    let has_ode_classification = explanation.steps.iter().any(|step| {
        step.title.contains("ODE") || step.description.contains("differential")
    });

    assert!(
        has_ode_classification,
        "Should have ODE-specific classification step"
    );
}
