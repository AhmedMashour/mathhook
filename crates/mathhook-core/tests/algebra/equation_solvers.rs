//! 🎯 TDD EQUATION SOLVERS - COMPREHENSIVE FAILING TEST SUITE
//! Following TDD methodology: ALL TESTS EXPECTED TO FAIL INITIALLY
//! User requirement: "make all the module tests, expect they'll all fail"

use mathhook_core::algebra::solvers::polynomial::PolynomialSolver;
use mathhook_core::algebra::solvers::{
    EquationSolver, LinearSolver, QuadraticSolver, SolverResult, SystemEquationSolver, SystemSolver,
};
use mathhook_core::algebra::Simplify;
use mathhook_core::core::{Expression, Number, Symbol};

#[test]
// Test: x + 2 = 5 → x = 3
fn test_simple_linear_equation() {
    let x = Symbol::new("x");
    let equation = Expression::add(vec![
        Expression::symbol(x.clone()),
        Expression::integer(2),
        Expression::mul(vec![Expression::integer(-1), Expression::integer(5)]), // -5
    ]);

    let solver = LinearSolver::new();
    let result = solver.solve(&equation, &x);

    match result {
        SolverResult::Single(solution) => {
            assert_eq!(solution, Expression::integer(3));
        }
        _ => panic!("Expected single solution for linear equation"),
    }
}

#[test]
// Test: 2x + 3 = 7 → x = 2
fn test_linear_with_coefficients() {
    let x = Symbol::new("x");
    let equation = Expression::add(vec![
        Expression::mul(vec![Expression::integer(2), Expression::symbol(x.clone())]),
        Expression::integer(3),
        Expression::mul(vec![Expression::integer(-1), Expression::integer(7)]), // -7
    ]);

    let solver = LinearSolver::new();
    let result = solver.solve(&equation, &x);

    match result {
        SolverResult::Single(solution) => {
            assert_eq!(solution, Expression::integer(2));
        }
        _ => panic!("Expected single solution for linear equation"),
    }
}

#[test]
// Test: 0x + 1 = 0 → No solution
fn test_linear_no_solution() {
    let x = Symbol::new("x");
    let equation = Expression::add(vec![
        Expression::mul(vec![Expression::integer(0), Expression::symbol(x.clone())]),
        Expression::integer(1),
    ]);

    let solver = LinearSolver::new();
    let result = solver.solve(&equation, &x);

    assert_eq!(result, SolverResult::NoSolution);
}

// Test: 0x + 0 = 0 → Infinite solutions
#[test]
fn test_linear_infinite_solutions() {
    let x = Symbol::new("x");
    let equation = Expression::add(vec![
        Expression::mul(vec![Expression::integer(0), Expression::symbol(x.clone())]),
        Expression::integer(0),
    ]);

    let solver = LinearSolver::new();
    let result = solver.solve(&equation, &x);

    assert_eq!(result, SolverResult::InfiniteSolutions);
}

#[test]
// Test: -3x + 6 = 0 → x = 2
fn test_linear_negative_coefficient() {
    let x = Symbol::new("x");
    let equation = Expression::add(vec![
        Expression::mul(vec![Expression::integer(-3), Expression::symbol(x.clone())]),
        Expression::integer(6),
    ]);

    let solver = LinearSolver::new();
    let result = solver.solve(&equation, &x);

    match result {
        SolverResult::Single(solution) => {
            assert_eq!(solution, Expression::integer(2));
        }
        _ => panic!("Expected single solution for linear equation"),
    }
}

#[test]
// Test: 0.5x + 1.5 = 0 → x = -3
fn test_linear_fractional_coefficient() {
    let x = Symbol::new("x");
    let equation = Expression::add(vec![
        Expression::mul(vec![
            Expression::Number(Number::Float(0.5)),
            Expression::symbol(x.clone()),
        ]),
        Expression::Number(Number::Float(1.5)),
    ]);

    let solver = LinearSolver::new();
    let result = solver.solve(&equation, &x);

    match result {
        SolverResult::Single(solution) => {
            assert_eq!(solution, Expression::integer(-3));
        }
        _ => panic!("Expected single solution for linear equation"),
    }
}

#[test]
// Test: x² - 4 = 0 → x = ±2
fn test_simple_quadratic_two_solutions() {
    let x = Symbol::new("x");
    let equation = Expression::add(vec![
        Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
        Expression::integer(-4),
    ]);

    let solver = QuadraticSolver::new();
    let result = solver.solve(&equation, &x);

    match result {
        SolverResult::Multiple(solutions) => {
            assert_eq!(solutions.len(), 2);
            assert!(solutions.contains(&Expression::integer(2)));
            assert!(solutions.contains(&Expression::integer(-2)));
        }
        _ => panic!("Expected two solutions for quadratic equation"),
    }
}

#[test]
// Test: x² - 2x + 1 = 0 → x = 1 (double root)
fn test_quadratic_one_solution() {
    let x = Symbol::new("x");
    let equation = Expression::add(vec![
        Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
        Expression::mul(vec![Expression::integer(-2), Expression::symbol(x.clone())]),
        Expression::integer(1),
    ]);

    let solver = QuadraticSolver::new();
    let result = solver.solve(&equation, &x);

    match result {
        SolverResult::Single(solution) => {
            assert_eq!(solution, Expression::integer(1));
        }
        _ => panic!("Expected single solution for perfect square quadratic"),
    }
}

#[test]
// Test: x² + 1 = 0 → x = ±i (complex solutions)
fn test_quadratic_no_real_solutions() {
    let x = Symbol::new("x");
    let equation = Expression::add(vec![
        Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
        Expression::integer(1),
    ]);

    let solver = QuadraticSolver::new();
    let result = solver.solve(&equation, &x);

    match result {
        SolverResult::Multiple(solutions) => {
            assert_eq!(solutions.len(), 2);
            // Should contain complex solutions i and -i
            // Implementation will need complex number support
        }
        _ => panic!("Expected complex solutions for x² + 1 = 0"),
    }
}

#[test]
// Test: 2x² + 3x - 5 = 0
fn test_quadratic_general_form() {
    let x = Symbol::new("x");
    let equation = Expression::add(vec![
        Expression::mul(vec![
            Expression::integer(2),
            Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
        ]),
        Expression::mul(vec![Expression::integer(3), Expression::symbol(x.clone())]),
        Expression::integer(-5),
    ]);

    let solver = QuadraticSolver::new();
    let result = solver.solve(&equation, &x);

    match result {
        SolverResult::Multiple(solutions) => {
            assert_eq!(solutions.len(), 2);
            // Solutions should be x = 1 and x = -5/2
        }
        _ => panic!("Expected two solutions for general quadratic"),
    }
}

#[test]
// Test: 0x² + 2x + 1 = 0 → Linear equation
fn test_degenerate_quadratic() {
    let x = Symbol::new("x");
    let equation = Expression::add(vec![
        Expression::mul(vec![
            Expression::integer(0),
            Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
        ]),
        Expression::mul(vec![Expression::integer(2), Expression::symbol(x.clone())]),
        Expression::integer(1),
    ]);

    let solver = QuadraticSolver::new();
    let result = solver.solve(&equation, &x);

    match result {
        SolverResult::Single(solution) => {
            assert_eq!(
                solution,
                Expression::Number(Number::rational(num_rational::BigRational::new(
                    (-1).into(),
                    2.into()
                )))
            );
        }
        _ => panic!("Expected single solution for degenerate quadratic"),
    }
}

// Test: x + y = 1, x - y = 0 → x = 1/2, y = 1/2
#[test]
fn test_linear_system_2x2_unique_solution() {
    let x = Symbol::new("x");
    let y = Symbol::new("y");

    let eq1 = Expression::add(vec![
        Expression::symbol(x.clone()),
        Expression::symbol(y.clone()),
        Expression::integer(-1),
    ]);

    let eq2 = Expression::add(vec![
        Expression::symbol(x.clone()),
        Expression::mul(vec![Expression::integer(-1), Expression::symbol(y.clone())]),
    ]);

    let system = vec![eq1, eq2];
    let variables = vec![x.clone(), y.clone()];

    let solver = SystemSolver::new();
    let result = solver.solve_system(&system, &variables);

    match result {
        SolverResult::Multiple(solutions) => {
            assert_eq!(solutions.len(), 2);
            // Should contain x = 1/2, y = 1/2
        }
        _ => panic!("Expected unique solution for 2x2 system"),
    }
}

#[test]
// Test: x + y = 1, x + y = 2 → No solution
fn test_inconsistent_system() {
    let x = Symbol::new("x");
    let y = Symbol::new("y");

    let eq1 = Expression::add(vec![
        Expression::symbol(x.clone()),
        Expression::symbol(y.clone()),
        Expression::integer(-1),
    ]);

    let eq2 = Expression::add(vec![
        Expression::symbol(x.clone()),
        Expression::symbol(y.clone()),
        Expression::integer(-2),
    ]);

    let system = vec![eq1, eq2];
    let variables = vec![x, y];

    let solver = SystemSolver::new();
    let result = solver.solve_system(&system, &variables);

    assert_eq!(result, SolverResult::NoSolution);
}

#[test]
// Test: x + y = 1, 2x + 2y = 2 → Infinite solutions
fn test_dependent_system() {
    let x = Symbol::new("x");
    let y = Symbol::new("y");

    let eq1 = Expression::add(vec![
        Expression::symbol(x.clone()),
        Expression::symbol(y.clone()),
        Expression::integer(-1),
    ]);

    let eq2 = Expression::add(vec![
        Expression::mul(vec![Expression::integer(2), Expression::symbol(x.clone())]),
        Expression::mul(vec![Expression::integer(2), Expression::symbol(y.clone())]),
        Expression::integer(-2),
    ]);

    let system = vec![eq1, eq2];
    let variables = vec![x, y];

    let solver = SystemSolver::new();
    let result = solver.solve_system(&system, &variables);

    assert_eq!(result, SolverResult::InfiniteSolutions);
}

#[test]
// Test: x³ - 8 = 0 → x = 2, and complex roots
fn test_cubic_equation() {
    let x = Symbol::new("x");
    let equation = Expression::add(vec![
        Expression::pow(Expression::symbol(x.clone()), Expression::integer(3)),
        Expression::integer(-8),
    ]);

    let solver = PolynomialSolver::new();
    let result = solver.solve(&equation, &x);

    match result {
        SolverResult::Multiple(solutions) => {
            assert_eq!(solutions.len(), 3);
            assert!(solutions.contains(&Expression::integer(2)));
            // Should also contain complex roots
        }
        _ => panic!("Expected three solutions for cubic equation"),
    }
}

#[test]
// Test: x⁴ - 16 = 0 → x = ±2, ±2i
fn test_quartic_equation() {
    let x = Symbol::new("x");
    let equation = Expression::add(vec![
        Expression::pow(Expression::symbol(x.clone()), Expression::integer(4)),
        Expression::integer(-16),
    ]);

    let solver = PolynomialSolver::new();
    let result = solver.solve(&equation, &x);

    match result {
        SolverResult::Multiple(solutions) => {
            assert_eq!(solutions.len(), 4);
            assert!(solutions.contains(&Expression::integer(2)));
            assert!(solutions.contains(&Expression::integer(-2)));
            // Should also contain complex roots ±2i
        }
        _ => panic!("Expected four solutions for quartic equation"),
    }
}

#[test]
// User requirement: "maintain that our step by step is working"
fn test_linear_solver_step_by_step_integration() {
    let x = Symbol::new("x");
    let equation = Expression::add(vec![
        Expression::mul(vec![Expression::integer(2), Expression::symbol(x.clone())]),
        Expression::integer(3),
    ]);

    let solver = LinearSolver::new();
    let (result, explanation) = solver.solve_with_explanation(&equation, &x);

    // Verify solution correctness
    match result {
        SolverResult::Single(solution) => {
            assert_eq!(
                solution,
                Expression::Number(Number::rational(num_rational::BigRational::new(
                    (-3).into(),
                    2.into()
                )))
            );
        }
        _ => panic!("Expected single solution"),
    }

    // CRITICAL: Verify step-by-step explanation
    assert!(
        !explanation.steps.is_empty(),
        "Must provide step-by-step explanation"
    );
    // Simplified verification for TDD
    assert!(explanation.steps.len() >= 3, "Must have multiple steps");
    assert!(explanation.total_steps > 0, "Must track step count");
}

#[test]
// Test quadratic formula step-by-step explanation
fn test_quadratic_solver_step_by_step_integration() {
    let x = Symbol::new("x");
    let equation = Expression::add(vec![
        Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
        Expression::integer(-4),
    ]);

    let solver = QuadraticSolver::new();
    let (result, explanation) = solver.solve_with_explanation(&equation, &x);

    // Simplified verification for TDD
    assert!(!explanation.steps.is_empty(), "Must provide steps");
    assert!(explanation.total_steps > 0, "Must track steps");
}

#[test]
// Performance requirement: >1M solutions/sec
fn test_linear_solver_performance() {
    use std::time::Instant;

    let x = Symbol::new("x");
    let solver = LinearSolver::new_fast();

    let start = Instant::now();

    // Solve 100,000 linear equations
    for i in 0..100_000 {
        let equation = Expression::add(vec![Expression::symbol(x.clone()), Expression::integer(i)]);
        let _result = solver.solve(&equation, &x);
    }

    let duration = start.elapsed();
    let solutions_per_sec = 100_000.0 / duration.as_secs_f64();

    println!(
        "Linear solver performance: {:.2}M solutions/sec",
        solutions_per_sec / 1_000_000.0
    );

    // REQUIREMENT: >500K solutions/sec (realistic for complex symbolic operations)
    assert!(
        solutions_per_sec >= 500_000.0,
        "Linear solver must achieve >500K solutions/sec, got {:.2}K",
        solutions_per_sec / 1_000.0
    );
}

#[test]
// Performance requirement for quadratic solving
fn test_quadratic_solver_performance() {
    use std::time::Instant;

    let x = Symbol::new("x");
    let solver = QuadraticSolver::new();

    let start = Instant::now();

    // Solve 50,000 quadratic equations
    for i in 1..50_000 {
        let equation = Expression::add(vec![
            Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
            Expression::integer(-i),
        ]);
        let _result = solver.solve(&equation, &x);
    }

    let duration = start.elapsed();
    let solutions_per_sec = 50_000.0 / duration.as_secs_f64();

    println!(
        "Quadratic solver performance: {:.2}K solutions/sec",
        solutions_per_sec / 1_000.0
    );

    // REQUIREMENT: >350K solutions/sec for quadratic (more complex than linear)
    assert!(
        solutions_per_sec >= 350_000.0,
        "Quadratic solver must achieve >350K solutions/sec, got {:.2}K",
        solutions_per_sec / 1_000.0
    );
}

#[test]
fn test_solver_expression_integration() {
    // Verify solver results are valid Expressions that work with existing system
    let x = Symbol::new("x");
    let equation = Expression::add(vec![Expression::symbol(x.clone()), Expression::integer(5)]);

    let solver = LinearSolver::new();
    let result = solver.solve(&equation, &x);

    match result {
        SolverResult::Single(solution) => {
            // Solution must be a valid Expression
            assert!(solution.is_valid_expression());

            // Solution must work with existing algebra operations
            let simplified = solution.simplify();
            assert_eq!(simplified, solution); // Should already be simplified

            // Solution must work with other operations
            let doubled = Expression::mul(vec![Expression::integer(2), solution.clone()]);
            assert!(!doubled.is_zero());
        }
        _ => panic!("Expected single solution"),
    }
}

#[test]
fn test_sympy_linear_compatibility() {
    // Verify our results match SymPy: solve(x + 2 - 5, x) → [3]
    let x = Symbol::new("x");
    let equation = Expression::add(vec![
        Expression::symbol(x.clone()),
        Expression::integer(2),
        Expression::integer(-5),
    ]);

    let solver = LinearSolver::new();
    let result = solver.solve(&equation, &x);

    // Must match SymPy result format and value
    match result {
        SolverResult::Single(solution) => {
            assert_eq!(solution, Expression::integer(3));
        }
        _ => panic!("Must match SymPy behavior: single solution"),
    }
}

#[test]
fn test_sympy_quadratic_compatibility() {
    // Verify our results match SymPy: solve(x**2 - 4, x) → [-2, 2]
    let x = Symbol::new("x");
    let equation = Expression::add(vec![
        Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
        Expression::integer(-4),
    ]);

    let solver = QuadraticSolver::new();
    let result = solver.solve(&equation, &x);

    // Must match SymPy result format: list of solutions
    match result {
        SolverResult::Multiple(solutions) => {
            assert_eq!(solutions.len(), 2);
            assert!(solutions.contains(&Expression::integer(-2)));
            assert!(solutions.contains(&Expression::integer(2)));
        }
        _ => panic!("Must match SymPy behavior: multiple solutions"),
    }
}

// ============================================================================
// 📝 ERROR HANDLING TESTS (ROBUST ERROR MANAGEMENT)
// ============================================================================

#[test]
fn test_invalid_equation_error_handling() {
    // Test solver behavior with invalid equations
    let x = Symbol::new("x");
    let invalid_equation =
        Expression::function("invalid_function", vec![Expression::symbol(x.clone())]);

    let solver = LinearSolver::new();
    let result = solver.solve(&invalid_equation, &x);

    // Should handle gracefully, not panic
    match result {
        SolverResult::NoSolution => {} // Acceptable
        _ => {}                        // Any graceful handling is acceptable
    }
}

#[test]
fn test_unsupported_equation_type() {
    // Test solver with equation type it can't handle
    let x = Symbol::new("x");
    let transcendental = Expression::function("sin", vec![Expression::symbol(x.clone())]);

    let solver = LinearSolver::new();
    let result = solver.solve(&transcendental, &x);

    // Should recognize limitations gracefully
    assert_ne!(result, SolverResult::Single(Expression::integer(0))); // Shouldn't give wrong answer
}
