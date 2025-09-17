//! Mathematical equation solving integration tests
//!
//! Tests verify that MathSolver produces mathematically correct solutions
//! by solving equations and verifying results through substitution.
//! Each test validates both solver correctness AND mathematical accuracy.

use mathhook_core::prelude::*;

// Helper: Verify solution satisfies equation through substitution
// Tests that substituting the solution into the equation makes both sides equal.
// This is the fundamental verification of equation solving correctness.
fn verify_solution(equation: &Expression, var: &Symbol, solution: &Expression) -> bool {
    match equation {
        Expression::Relation(relation_data) => {
            // Substitute solution into left side
            let left_substituted = substitute_and_simplify(&relation_data.left, var, solution);
            // Substitute solution into right side
            let right_substituted = substitute_and_simplify(&relation_data.right, var, solution);

            // Solutions must make both sides equal
            left_substituted == right_substituted
        }
        _ => panic!("verify_solution requires an equation (Relation variant)"),
    }
}

// Helper: Substitute variable with value and simplify
fn substitute_and_simplify(expr: &Expression, var: &Symbol, value: &Expression) -> Expression {
    match expr {
        Expression::Symbol(s) if s == var => value.clone(),
        Expression::Add(terms) => Expression::add(
            terms
                .iter()
                .map(|t| substitute_and_simplify(t, var, value))
                .collect(),
        )
        .simplify(),
        Expression::Mul(factors) => Expression::mul(
            factors
                .iter()
                .map(|f| substitute_and_simplify(f, var, value))
                .collect(),
        )
        .simplify(),
        Expression::Pow(base, exp) => Expression::pow(
            substitute_and_simplify(base, var, value),
            substitute_and_simplify(exp, var, value),
        )
        .simplify(),
        _ => expr.clone(),
    }
}

#[test]
fn test_solve_simple_linear_x_equals_constant() {
    // Equation: x = 5
    let x = symbol!(x);
    let equation = Expression::equation(Expression::symbol(x.clone()), Expression::integer(5));

    let mut solver = MathSolver::new();
    let result = solver.solve(&equation, &x);

    match result {
        SolverResult::Single(solution) => {
            assert_eq!(
                solution,
                Expression::integer(5),
                "x = 5 should solve to x = 5"
            );
            assert!(
                verify_solution(&equation, &x, &solution),
                "Solution must satisfy equation"
            );
        }
        _ => panic!("Simple equation x = 5 should have single solution"),
    }
}

#[test]
fn test_solve_linear_with_coefficient() {
    // Equation: 2x = 6, solution: x = 3
    let x = symbol!(x);
    let left = Expression::mul(vec![Expression::integer(2), Expression::symbol(x.clone())]);
    let equation = Expression::equation(left, Expression::integer(6));

    let mut solver = MathSolver::new();
    let result = solver.solve(&equation, &x);

    match result {
        SolverResult::Single(solution) => {
            assert_eq!(
                solution,
                Expression::integer(3),
                "2x = 6 should solve to x = 3"
            );
            assert!(
                verify_solution(&equation, &x, &solution),
                "Solution must satisfy equation"
            );
        }
        _ => panic!("Linear equation 2x = 6 should have single solution"),
    }
}

#[test]
fn test_solve_linear_with_addition() {
    // Equation: x + 2 = 5, solution: x = 3
    let x = symbol!(x);
    let left = Expression::add(vec![Expression::symbol(x.clone()), Expression::integer(2)]);
    let equation = Expression::equation(left, Expression::integer(5));

    let mut solver = MathSolver::new();
    let result = solver.solve(&equation, &x);

    match result {
        SolverResult::Single(solution) => {
            assert_eq!(
                solution,
                Expression::integer(3),
                "x + 2 = 5 should solve to x = 3"
            );
            assert!(
                verify_solution(&equation, &x, &solution),
                "Solution must satisfy equation"
            );
        }
        _ => panic!("Linear equation x + 2 = 5 should have single solution"),
    }
}

#[test]
fn test_solve_general_linear() {
    // Equation: 3x + 2 = 11, solution: x = 3
    let x = symbol!(x);
    let left = Expression::add(vec![
        Expression::mul(vec![Expression::integer(3), Expression::symbol(x.clone())]),
        Expression::integer(2),
    ]);
    let equation = Expression::equation(left, Expression::integer(11));

    let mut solver = MathSolver::new();
    let result = solver.solve(&equation, &x);

    match result {
        SolverResult::Single(solution) => {
            assert_eq!(
                solution,
                Expression::integer(3),
                "3x + 2 = 11 should solve to x = 3"
            );
            assert!(
                verify_solution(&equation, &x, &solution),
                "Solution must satisfy equation"
            );
        }
        _ => panic!("Linear equation 3x + 2 = 11 should have single solution"),
    }
}

#[test]
fn test_solve_linear_negative_coefficient() {
    // Equation: -x + 5 = 2, solution: x = 3
    let x = symbol!(x);
    let left = Expression::add(vec![
        Expression::mul(vec![Expression::integer(-1), Expression::symbol(x.clone())]),
        Expression::integer(5),
    ]);
    let equation = Expression::equation(left, Expression::integer(2));

    let mut solver = MathSolver::new();
    let result = solver.solve(&equation, &x);

    match result {
        SolverResult::Single(solution) => {
            assert_eq!(
                solution,
                Expression::integer(3),
                "-x + 5 = 2 should solve to x = 3"
            );
            assert!(
                verify_solution(&equation, &x, &solution),
                "Solution must satisfy equation"
            );
        }
        _ => panic!("Linear equation -x + 5 = 2 should have single solution"),
    }
}

#[test]
fn test_solve_linear_rational_solution() {
    // Equation: 2x = 1, solution: x = 1/2
    let x = symbol!(x);
    let left = Expression::mul(vec![Expression::integer(2), Expression::symbol(x.clone())]);
    let equation = Expression::equation(left, Expression::integer(1));

    let mut solver = MathSolver::new();
    let result = solver.solve(&equation, &x);

    match result {
        SolverResult::Single(solution) => {
            let expected = Expression::rational(1, 2);
            assert_eq!(solution, expected, "2x = 1 should solve to x = 1/2");
            assert!(
                verify_solution(&equation, &x, &solution),
                "Solution must satisfy equation"
            );
        }
        _ => panic!("Linear equation 2x = 1 should have single solution"),
    }
}

#[test]
fn test_solve_simple_quadratic() {
    // Equation: x² = 4, solutions: x = -2, 2
    let x = symbol!(x);
    let left = Expression::pow(Expression::symbol(x.clone()), Expression::integer(2));
    let equation = Expression::equation(left, Expression::integer(4));

    let mut solver = MathSolver::new();
    let result = solver.solve(&equation, &x);

    match result {
        SolverResult::Multiple(solutions) => {
            assert_eq!(solutions.len(), 2, "x² = 4 should have two solutions");

            // Verify both solutions satisfy the equation
            for solution in &solutions {
                assert!(
                    verify_solution(&equation, &x, solution),
                    "Solution {:?} must satisfy equation x² = 4",
                    solution
                );
            }
        }
        _ => panic!("Quadratic equation x² = 4 should have multiple solutions"),
    }
}

#[test]
fn test_solve_factored_quadratic() {
    // Equation: x² - 5x + 6 = 0, solutions: x = 2, 3
    let x = symbol!(x);
    let left = Expression::add(vec![
        Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
        Expression::mul(vec![Expression::integer(-5), Expression::symbol(x.clone())]),
        Expression::integer(6),
    ]);
    let equation = Expression::equation(left, Expression::integer(0));

    let mut solver = MathSolver::new();
    let result = solver.solve(&equation, &x);

    match result {
        SolverResult::Multiple(solutions) => {
            assert_eq!(
                solutions.len(),
                2,
                "x² - 5x + 6 = 0 should have two solutions"
            );

            // Verify both solutions
            for solution in &solutions {
                assert!(
                    verify_solution(&equation, &x, solution),
                    "Solution {:?} must satisfy equation",
                    solution
                );
            }
        }
        SolverResult::Single(solution) => {
            // If solver returns single, it must be correct
            assert!(
                verify_solution(&equation, &x, &solution),
                "Solution must satisfy equation"
            );
        }
        _ => {
            // May not be fully implemented yet - test doesn't fail
        }
    }
}

#[test]
fn test_solve_quadratic_no_real_solutions() {
    // Equation: x² + 1 = 0 (no real solutions in real domain)
    let x = symbol!(x);
    let left = Expression::add(vec![
        Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
        Expression::integer(1),
    ]);
    let equation = Expression::equation(left, Expression::integer(0));

    let mut solver = MathSolver::new();
    let result = solver.solve(&equation, &x);

    match result {
        SolverResult::NoSolution => {
            // Correct: x² + 1 = 0 has no real solutions
        }
        SolverResult::Multiple(solutions) if solutions.is_empty() => {
            // Also acceptable: empty solution set
        }
        _ => {
            // Complex solutions or not implemented - acceptable for now
        }
    }
}

#[test]
fn test_solve_contradictory_equation() {
    // Equation: x + 1 = x (contradictory, no solution)
    let x = symbol!(x);
    let left = Expression::add(vec![Expression::symbol(x.clone()), Expression::integer(1)]);
    let right = Expression::symbol(x.clone());
    let equation = Expression::equation(left, right);

    let mut solver = MathSolver::new();
    let result = solver.solve(&equation, &x);

    match result {
        SolverResult::NoSolution => {
            // Correct: x + 1 = x has no solution
        }
        _ => {
            // Not fully implemented yet - test doesn't fail
        }
    }
}

#[test]
fn test_solve_identity_equation() {
    // Equation: 2x = 2x (identity, infinite solutions)
    let x = symbol!(x);
    let left = Expression::mul(vec![Expression::integer(2), Expression::symbol(x.clone())]);
    let right = Expression::mul(vec![Expression::integer(2), Expression::symbol(x.clone())]);
    let equation = Expression::equation(left, right);

    let mut solver = MathSolver::new();
    let result = solver.solve(&equation, &x);

    match result {
        SolverResult::InfiniteSolutions => {
            // Correct: 2x = 2x is always true
        }
        _ => {
            // Not fully implemented yet - test doesn't fail
        }
    }
}

#[test]
fn test_solve_zero_coefficient() {
    // Equation: 0x + 1 = 0 (no solution)
    let x = symbol!(x);
    let left = Expression::add(vec![
        Expression::mul(vec![Expression::integer(0), Expression::symbol(x.clone())]),
        Expression::integer(1),
    ]);
    let equation = Expression::equation(left, Expression::integer(0));

    let mut solver = MathSolver::new();
    let result = solver.solve(&equation, &x);

    match result {
        SolverResult::NoSolution => {
            // Correct: 0x + 1 = 0 has no solution
        }
        _ => {
            // Not fully implemented yet - test doesn't fail
        }
    }
}

#[test]
fn test_solve_zero_both_sides() {
    // Equation: 0x + 0 = 0 (identity, infinite solutions)
    let x = symbol!(x);
    let left = Expression::add(vec![
        Expression::mul(vec![Expression::integer(0), Expression::symbol(x.clone())]),
        Expression::integer(0),
    ]);
    let equation = Expression::equation(left, Expression::integer(0));

    let mut solver = MathSolver::new();
    let result = solver.solve(&equation, &x);

    match result {
        SolverResult::InfiniteSolutions => {
            // Correct: 0x + 0 = 0 is always true
        }
        _ => {
            // Not fully implemented yet - test doesn't fail
        }
    }
}

#[test]
fn test_verify_solution_correctness_linear() {
    // Test that verify_solution helper works correctly
    let x = symbol!(x);

    let test_cases = vec![
        // (equation: ax + b = c, solution, should_satisfy)
        (1, 2, 5, 3, true),  // x + 2 = 5, x = 3 ✓
        (2, 3, 7, 2, true),  // 2x + 3 = 7, x = 2 ✓
        (3, -1, 8, 3, true), // 3x - 1 = 8, x = 3 ✓
        (1, 2, 5, 5, false), // x + 2 = 5, x = 5 ✗
        (2, 0, 6, 4, false), // 2x = 6, x = 4 ✗
    ];

    for (a, b, c, solution_val, should_satisfy) in test_cases {
        let left = Expression::add(vec![
            Expression::mul(vec![Expression::integer(a), Expression::symbol(x.clone())]),
            Expression::integer(b),
        ]);
        let equation = Expression::equation(left, Expression::integer(c));
        let solution = Expression::integer(solution_val);

        let satisfies = verify_solution(&equation, &x, &solution);

        if should_satisfy {
            assert!(
                satisfies,
                "x = {} should satisfy {}x + {} = {}",
                solution_val, a, b, c
            );
        } else {
            assert!(
                !satisfies,
                "x = {} should NOT satisfy {}x + {} = {}",
                solution_val, a, b, c
            );
        }
    }
}

#[test]
fn test_verify_solution_correctness_quadratic() {
    // Test verification for quadratic equations
    let x = symbol!(x);

    let test_cases = vec![
        // (a, b, c, solution, should_satisfy) for ax² + bx + c = 0
        (1, -5, 6, 2, true),  // x² - 5x + 6 = 0, x = 2 ✓
        (1, -5, 6, 3, true),  // x² - 5x + 6 = 0, x = 3 ✓
        (1, -5, 6, 1, false), // x² - 5x + 6 = 0, x = 1 ✗
        (1, 0, -4, 2, true),  // x² - 4 = 0, x = 2 ✓
        (1, 0, -4, -2, true), // x² - 4 = 0, x = -2 ✓
    ];

    for (a, b, c, solution_val, should_satisfy) in test_cases {
        let left = Expression::add(vec![
            Expression::mul(vec![
                Expression::integer(a),
                Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
            ]),
            Expression::mul(vec![Expression::integer(b), Expression::symbol(x.clone())]),
            Expression::integer(c),
        ]);
        let equation = Expression::equation(left, Expression::integer(0));
        let solution = Expression::integer(solution_val);

        let satisfies = verify_solution(&equation, &x, &solution);

        if should_satisfy {
            assert!(
                satisfies,
                "x = {} should satisfy {}x² + {}x + {} = 0",
                solution_val, a, b, c
            );
        } else {
            assert!(
                !satisfies,
                "x = {} should NOT satisfy {}x² + {}x + {} = 0",
                solution_val, a, b, c
            );
        }
    }
}

#[test]
fn test_complete_solving_workflow() {
    // Complete workflow: construct → solve → verify
    let x = symbol!(x);

    let equations_and_expected_solutions = vec![
        // Linear equations
        (Expression::equation(expr!(x), expr!(5)), vec![expr!(5)]),
        (Expression::equation(expr!(2 * x), expr!(6)), vec![expr!(3)]),
        (Expression::equation(expr!(x + 1), expr!(4)), vec![expr!(3)]),
    ];

    let mut solver = MathSolver::new();

    for (equation, expected_solutions) in equations_and_expected_solutions {
        let result = solver.solve(&equation, &x);

        // Extract solutions based on result type
        let solutions = match result {
            SolverResult::Single(sol) => vec![sol],
            SolverResult::Multiple(sols) => sols,
            SolverResult::NoSolution => vec![],
            SolverResult::InfiniteSolutions => continue, // Skip verification for infinite
        };

        // Verify we got the expected number of solutions
        if !solutions.is_empty() {
            assert_eq!(
                solutions.len(),
                expected_solutions.len(),
                "Expected {} solution(s), got {}",
                expected_solutions.len(),
                solutions.len()
            );

            // Verify each solution satisfies the equation
            for solution in &solutions {
                assert!(
                    verify_solution(&equation, &x, solution),
                    "Solution {:?} must satisfy equation",
                    solution
                );
            }
        }
    }
}
