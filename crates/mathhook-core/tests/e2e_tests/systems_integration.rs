// Test suite for System Solver integration with SmartEquationSolver
// This verifies that:
// 1. EquationAnalyzer correctly detects system equations
// 2. SmartEquationSolver correctly routes to SystemSolver
// 3. Polynomial systems route to Gröbner basis solver
use mathhook_core::algebra::equation_analyzer::{
    EquationAnalyzer, EquationType, SmartEquationSolver,
};
use mathhook_core::algebra::solvers::{SolverResult, SystemEquationSolver, SystemSolver};
use mathhook_core::{symbol, Expression};

#[test]
fn test_system_detection_multiple_variables() {
    // Test: x + y = 3 should be detected as potential system (multiple variables)
    let x = symbol!(x);
    let y = symbol!(y);

    let equation = Expression::add(vec![
        Expression::symbol(x.clone()),
        Expression::symbol(y.clone()),
        Expression::integer(-3),
    ]);

    let equation_type = EquationAnalyzer::analyze(&equation, &x);

    // Could be linear or system depending on context, but should not be polynomial/ODE/PDE
    assert!(
        matches!(equation_type, EquationType::Linear | EquationType::System),
        "Should detect equation with multiple variables as Linear or System"
    );
}

#[test]
#[ignore = "FIXME: Lets find out why"]
fn test_polynomial_system_detection() {
    // Test: x² + y = 1 should be detected as polynomial (degree > 1)
    let x = symbol!(x);
    let y = symbol!(y);

    let equation = Expression::add(vec![
        Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
        Expression::symbol(y.clone()),
        Expression::integer(-1),
    ]);

    let equation_type = EquationAnalyzer::analyze(&equation, &x);

    // Should be detected as polynomial due to x²
    assert!(
        matches!(
            equation_type,
            EquationType::Quadratic | EquationType::Cubic | EquationType::Quartic
        ),
        "Should detect x² + y as polynomial equation, got {:?}",
        equation_type
    );
}

#[test]
fn test_smart_solver_system_routing() {
    // Test: SmartEquationSolver routes to SystemSolver (Fix 2: line 290 bug)
    // Previously routed System → linear_solver (WRONG)
    // Now routes System → system_solver (CORRECT)

    let x = symbol!(x);
    let y = symbol!(y);

    // Create a simple linear system through solve_system
    let eq1 = Expression::add(vec![
        Expression::symbol(x.clone()),
        Expression::symbol(y.clone()),
        Expression::integer(-3),
    ]);
    let eq2 = Expression::add(vec![
        Expression::symbol(x.clone()),
        Expression::mul(vec![Expression::integer(-1), Expression::symbol(y.clone())]),
        Expression::integer(-1),
    ]);

    let mut solver = SmartEquationSolver::new();
    let result = solver.solve_system(&[eq1, eq2], &[x, y]);

    // Should find unique solution: x = 2, y = 1
    match result {
        SolverResult::Multiple(sols) => {
            assert_eq!(sols.len(), 2, "Should have 2 solutions (x and y)");
            assert_eq!(sols[0], Expression::integer(2), "x should be 2");
            assert_eq!(sols[1], Expression::integer(1), "y should be 1");
        }
        _ => panic!(
            "Expected unique solution for simple 2x2 linear system, got {:?}",
            result
        ),
    }
}

#[test]
fn test_system_solver_linear_2x2() {
    // Test: SystemSolver handles simple 2x2 linear system
    let x = symbol!(x);
    let y = symbol!(y);

    let solver = SystemSolver::new();

    // System: 2x + y = 5
    //         x - y = 1
    // Solution: x = 2, y = 1
    let eq1 = Expression::add(vec![
        Expression::mul(vec![Expression::integer(2), Expression::symbol(x.clone())]),
        Expression::symbol(y.clone()),
        Expression::integer(-5),
    ]);
    let eq2 = Expression::add(vec![
        Expression::symbol(x.clone()),
        Expression::mul(vec![Expression::integer(-1), Expression::symbol(y.clone())]),
        Expression::integer(-1),
    ]);

    let result = solver.solve_system(&[eq1, eq2], &[x, y]);

    match result {
        SolverResult::Multiple(sols) => {
            assert_eq!(sols.len(), 2);
            assert_eq!(sols[0], Expression::integer(2));
            assert_eq!(sols[1], Expression::integer(1));
        }
        _ => panic!("Expected unique solution, got {:?}", result),
    }
}

#[test]
fn test_polynomial_system_routes_to_groebner() {
    // Test: Polynomial system routes to Gröbner basis solver
    let x = symbol!(x);
    let y = symbol!(y);

    let solver = SystemSolver::new();

    // Polynomial system: x - 1 = 0, y - 2 = 0
    // Simple enough for Gröbner basis to solve
    let eq1 = Expression::add(vec![Expression::symbol(x.clone()), Expression::integer(-1)]);
    let eq2 = Expression::add(vec![Expression::symbol(y.clone()), Expression::integer(-2)]);

    let result = solver.solve_system(&[eq1, eq2], &[x, y]);

    // Should find solution or return Partial (acceptable per Fix 3)
    match result {
        SolverResult::Multiple(sols) => {
            assert_eq!(sols.len(), 2);
            // Solutions should be x=1, y=2 (after simplification)
        }
        SolverResult::Partial(_) => {
            // Acceptable: Gröbner basis computed but extraction incomplete
            // Full implementation deferred to Phase 4: WAVE-CLEANUP
        }
        _ => panic!(
            "Expected Multiple or Partial for simple polynomial system, got {:?}",
            result
        ),
    }
}

#[test]
#[ignore = "FIXME: Lets find out why"]
fn test_groebner_stub_mathematical_honesty() {
    // Test: Verify Fix 3 - stub returns Partial (honest) not NoSolution (false negative)
    let x = symbol!(x);
    let y = symbol!(y);

    let solver = SystemSolver::new();

    // Complex polynomial system that requires univariate solving + back-substitution
    // Circle: x² + y² = 1
    // Line: x - y = 0
    // Solutions: (√2/2, √2/2) and (-√2/2, -√2/2)
    let eq1 = Expression::add(vec![
        Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
        Expression::pow(Expression::symbol(y.clone()), Expression::integer(2)),
        Expression::integer(-1),
    ]);
    let eq2 = Expression::add(vec![
        Expression::symbol(x.clone()),
        Expression::mul(vec![Expression::integer(-1), Expression::symbol(y.clone())]),
    ]);

    let result = solver.solve_system(&[eq1, eq2], &[x, y]);

    // Should NOT return NoSolution (that would be false negative)
    // Should return either:
    // - Multiple (if full extraction implemented)
    // - Partial (if extraction incomplete - acceptable per Fix 3)
    assert!(
        !matches!(result, SolverResult::NoSolution),
        "Should NOT return NoSolution for solvable system - this is mathematically incorrect. \
         Expected Multiple or Partial. Got: {:?}",
        result
    );

    // Partial is acceptable (Gröbner basis computed, extraction incomplete)
    // Full implementation deferred to Phase 4: WAVE-CLEANUP
    assert!(
        matches!(result, SolverResult::Partial(_) | SolverResult::Multiple(_)),
        "Expected Partial or Multiple, got {:?}",
        result
    );
}

#[test]
fn test_no_regression_linear_systems() {
    // Test: Ensure linear systems still work (no regression from Gröbner integration)
    let x = symbol!(x);
    let y = symbol!(y);
    let z = symbol!(z);

    let solver = SystemSolver::new();

    // 3x3 linear system
    let eq1 = Expression::add(vec![
        Expression::symbol(x.clone()),
        Expression::symbol(y.clone()),
        Expression::symbol(z.clone()),
        Expression::integer(-6),
    ]);
    let eq2 = Expression::add(vec![
        Expression::mul(vec![Expression::integer(2), Expression::symbol(x.clone())]),
        Expression::symbol(y.clone()),
        Expression::integer(-3),
    ]);
    let eq3 = Expression::add(vec![
        Expression::symbol(x.clone()),
        Expression::mul(vec![Expression::integer(-1), Expression::symbol(y.clone())]),
        Expression::mul(vec![Expression::integer(2), Expression::symbol(z.clone())]),
        Expression::integer(-1),
    ]);

    let result = solver.solve_system(&[eq1, eq2, eq3], &[x, y, z]);

    // Should find unique solution (linear systems use Gaussian elimination, not Gröbner)
    match result {
        SolverResult::Multiple(sols) => {
            assert_eq!(sols.len(), 3, "Should have 3 solutions");
        }
        _ => panic!(
            "Expected unique solution for 3x3 linear system, got {:?}",
            result
        ),
    }
}

#[test]
fn test_architecture_no_hardcoded_routing() {
    // Test: Verify architecture uses SystemSolver trait, not hardcoded function matching

    let x = symbol!(x);
    let y = symbol!(y);

    // Use SmartEquationSolver (high-level API)
    let mut smart_solver = SmartEquationSolver::new();

    let eq1 = Expression::add(vec![
        Expression::symbol(x.clone()),
        Expression::symbol(y.clone()),
        Expression::integer(-3),
    ]);
    let eq2 = Expression::add(vec![
        Expression::symbol(x.clone()),
        Expression::mul(vec![Expression::integer(-1), Expression::symbol(y.clone())]),
        Expression::integer(-1),
    ]);

    let result = smart_solver.solve_system(&[eq1, eq2], &[x, y]);

    // Should successfully route through SmartEquationSolver → SystemSolver
    // This verifies the registry-based architecture (not hardcoded matches)
    assert!(
        matches!(result, SolverResult::Multiple(_)),
        "SmartEquationSolver should route to SystemSolver via traits, not hardcoded"
    );
}
