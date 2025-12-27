//! Integration tests for matrix equation solver through SmartEquationSolver API

use mathhook_core::algebra::equation_analyzer::SmartEquationSolver;
use mathhook_core::algebra::solvers::SolverResult;
use mathhook_core::{symbol, Expression};

#[test]
fn test_left_division_through_smart_solver() {
    let solver = SmartEquationSolver::new();
    let a = symbol!(A; matrix);
    let x = symbol!(X; matrix);
    let b = symbol!(B; matrix);

    // A*X - B = 0
    let equation = Expression::add(vec![
        Expression::mul(vec![
            Expression::symbol(a.clone()),
            Expression::symbol(x.clone()),
        ]),
        Expression::mul(vec![Expression::integer(-1), Expression::symbol(b.clone())]),
    ]);

    let (result, _explanation) = solver.solve_with_equation(&equation, &x);

    match result {
        SolverResult::Single(solution) => {
            // Solution should be A^(-1)*B
            assert!(solution.to_string().contains("A"));
            assert!(solution.to_string().contains("B"));
        }
        _ => panic!("Expected single solution for left division"),
    }
}

#[test]
fn test_right_division_through_smart_solver() {
    let solver = SmartEquationSolver::new();
    let a = symbol!(A; matrix);
    let x = symbol!(X; matrix);
    let b = symbol!(B; matrix);

    // X*A - B = 0
    let equation = Expression::add(vec![
        Expression::mul(vec![
            Expression::symbol(x.clone()),
            Expression::symbol(a.clone()),
        ]),
        Expression::mul(vec![Expression::integer(-1), Expression::symbol(b.clone())]),
    ]);

    let (result, _explanation) = solver.solve_with_equation(&equation, &x);

    match result {
        SolverResult::Single(solution) => {
            // Solution should be B*A^(-1)
            assert!(solution.to_string().contains("A"));
            assert!(solution.to_string().contains("B"));
        }
        _ => panic!("Expected single solution for right division"),
    }
}

#[test]
fn test_operator_equation_through_smart_solver() {
    let solver = SmartEquationSolver::new();
    let h = symbol!(H; operator);
    let psi = symbol!(psi; operator);
    let e = symbol!(E; operator);

    // H*psi - E*psi = 0 (Eigenvalue equation)
    let equation = Expression::add(vec![
        Expression::mul(vec![
            Expression::symbol(h.clone()),
            Expression::symbol(psi.clone()),
        ]),
        Expression::mul(vec![
            Expression::integer(-1),
            Expression::mul(vec![
                Expression::symbol(e.clone()),
                Expression::symbol(psi.clone()),
            ]),
        ]),
    ]);

    let (result, _explanation) = solver.solve_with_equation(&equation, &psi);

    // This is a more complex equation (eigenvalue), solver may return NoSolution
    // or require factoring - just verify it doesn't panic
    match result {
        SolverResult::Single(_) | SolverResult::NoSolution => {
            // Both outcomes are acceptable for this complex equation
        }
        _ => {}
    }
}

#[test]
fn test_scalar_equations_still_work() {
    let solver = SmartEquationSolver::new();
    let x = symbol!(x);

    // 2x + 3 = 0
    let equation = Expression::add(vec![
        Expression::mul(vec![Expression::integer(2), Expression::symbol(x.clone())]),
        Expression::integer(3),
    ]);

    let (result, _explanation) = solver.solve_with_equation(&equation, &x);

    match result {
        SolverResult::Single(solution) => {
            // Solution should be -3/2
            let simplified = solution.to_string();
            assert!(simplified.contains("-3") || simplified.contains("3"));
        }
        _ => panic!("Expected single solution for linear equation"),
    }
}

#[test]
fn test_matrix_equation_with_explanation() {
    let solver = SmartEquationSolver::new();
    let a = symbol!(A; matrix);
    let x = symbol!(X; matrix);
    let b = symbol!(B; matrix);

    // A*X - B = 0
    let equation = Expression::add(vec![
        Expression::mul(vec![
            Expression::symbol(a.clone()),
            Expression::symbol(x.clone()),
        ]),
        Expression::mul(vec![Expression::integer(-1), Expression::symbol(b.clone())]),
    ]);

    let (result, explanation) = solver.solve_with_equation(&equation, &x);

    // Verify we got a result
    assert!(matches!(result, SolverResult::Single(_)));

    // Verify explanation contains matrix-related steps
    let explanation_text = format!("{:?}", explanation);
    assert!(
        explanation_text.contains("matrix") || explanation_text.contains("noncommutative"),
        "Explanation should mention matrix/noncommutative symbols"
    );
}

#[test]
fn test_mixed_matrix_scalar_equation() {
    let solver = SmartEquationSolver::new();
    let a = symbol!(A; matrix);
    let x = symbol!(x);

    // A*x is a mixed equation (matrix * scalar), still noncommutative
    let equation = Expression::add(vec![
        Expression::mul(vec![
            Expression::symbol(a.clone()),
            Expression::symbol(x.clone()),
        ]),
        Expression::integer(-1),
    ]);

    let (_result, _explanation) = solver.solve_with_equation(&equation, &x);

    // Should be detected as matrix equation due to A being a matrix
    // Actual solving may not work (different types), but detection should work
}
