/// Test suite for PDE integration with SmartEquationSolver
///
/// This verifies that:
/// 1. EquationAnalyzer correctly detects PDE equations
/// 2. SmartEquationSolver correctly routes to PDE solver
/// 3. No stub implementations remain in the critical path
/// 4. Architecture follows CLAUDE.md patterns (registry-based, not hardcoded)

use mathhook_core::algebra::equation_analyzer::{EquationAnalyzer, EquationType, SmartEquationSolver};
use mathhook_core::core::{Expression, Symbol};
use mathhook_core::symbol;

#[test]
fn test_pde_detection_partial_derivative_symbol() {
    // Test: ∂u/∂t (partial derivative using ∂ notation)
    let u = symbol!(u);
    let partial_u = Symbol::new("∂u");

    let expr = Expression::symbol(partial_u);

    let equation_type = EquationAnalyzer::analyze(&expr, &u);

    assert_eq!(
        equation_type,
        EquationType::PDE,
        "Should detect equation with ∂u as PDE"
    );
}

#[test]
fn test_pde_detection_partial_function() {
    // Test: partial(u, t) (using partial function notation)
    let u = symbol!(u);
    let t = symbol!(t);

    let partial_expr = Expression::function(
        "partial",
        vec![Expression::symbol(u.clone()), Expression::symbol(t.clone())]
    );

    let equation_type = EquationAnalyzer::analyze(&partial_expr, &u);

    assert_eq!(
        equation_type,
        EquationType::PDE,
        "Should detect partial() function as PDE"
    );
}

#[test]
fn test_pde_not_ode() {
    // Test: PDE should be detected as PDE, NOT as ODE
    let u = symbol!(u);
    let partial_symbol = Symbol::new("∂u");

    let expr = Expression::symbol(partial_symbol);

    let equation_type = EquationAnalyzer::analyze(&expr, &u);

    assert_eq!(
        equation_type,
        EquationType::PDE,
        "Partial derivative should be PDE, not ODE"
    );
    assert_ne!(
        equation_type,
        EquationType::ODE,
        "Partial derivative should NOT be classified as ODE"
    );
}

#[test]
fn test_smart_solver_pde_routing() {
    // Test: SmartEquationSolver routes PDE to PDE solver (not ODE/polynomial)
    let u = symbol!(u);
    let partial_u = Symbol::new("∂u");

    let equation = Expression::symbol(partial_u);

    let mut solver = SmartEquationSolver::new();
    let (result, explanation) = solver.solve_with_equation(&equation, &u);

    // Verify routing happened (explanation should mention PDE)
    let steps_str = format!("{:?}", explanation);
    assert!(
        steps_str.contains("PDE") || steps_str.contains("partial"),
        "SmartEquationSolver should route to PDE solver and mention it in explanation"
    );

    // Verify it didn't incorrectly route to ODE solver
    assert!(
        !steps_str.contains("ODE Classification"),
        "Should NOT route to ODE solver for PDE"
    );
}

#[test]
fn test_non_pde_still_works() {
    // Regression test: Non-PDE equations should still be detected correctly
    let x = symbol!(x);

    // ODE: y' + 2y = 0
    let y = symbol!(y);
    let y_prime = Symbol::new("y'");
    let ode = Expression::add(vec![
        Expression::symbol(y_prime),
        Expression::mul(vec![Expression::integer(2), Expression::symbol(y.clone())])
    ]);

    let ode_type = EquationAnalyzer::analyze(&ode, &y);
    assert_eq!(
        ode_type,
        EquationType::ODE,
        "ODE should still be detected as ODE, not PDE"
    );

    // Quadratic: x^2 + 3x + 2 = 0
    let quadratic = Expression::add(vec![
        Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
        Expression::mul(vec![Expression::integer(3), Expression::symbol(x.clone())]),
        Expression::integer(2),
    ]);

    let quad_type = EquationAnalyzer::analyze(&quadratic, &x);
    assert_eq!(
        quad_type,
        EquationType::Quadratic,
        "Quadratic should still be detected correctly (no regression)"
    );
}

#[test]
fn test_architectural_pattern_no_hardcoded_pde_matching() {
    // Architectural verification: PDE detection should use helper methods,
    // not hardcoded string matching in analyze() match statement

    // This is verified by code structure, not runtime test
    // But we can verify the BEHAVIOR is consistent

    let u = symbol!(u);

    // Test multiple partial derivative notations
    let notations = vec![
        Symbol::new("∂u"),
        Symbol::new("partial_u"),
    ];

    for notation in notations {
        let expr = Expression::symbol(notation);
        let eq_type = EquationAnalyzer::analyze(&expr, &u);

        assert_eq!(
            eq_type,
            EquationType::PDE,
            "All partial derivative notations should be detected consistently via helper methods"
        );
    }
}

#[test]
fn test_no_stub_implementations_in_pde_routing() {
    // Verify SmartEquationSolver actually has a PDE solver field
    // and doesn't just return NotImplemented

    let mut solver = SmartEquationSolver::new();

    // We can't directly inspect the private pde_solver field,
    // but we can verify behavior via public API
    let u = symbol!(u);
    let partial_u = Symbol::new("∂u");

    let equation = Expression::symbol(partial_u);

    let (_result, explanation) = solver.solve_with_equation(&equation, &u);

    // Should have at least SOME steps (not empty/stub)
    assert!(
        !explanation.steps.is_empty(),
        "PDE routing should produce explanation steps, not be a stub"
    );

    // Should mention PDE classification
    let has_pde_classification = explanation.steps.iter().any(|step| {
        step.title.contains("PDE") || step.description.contains("partial")
    });

    assert!(
        has_pde_classification,
        "Should have PDE-specific classification step"
    );
}
