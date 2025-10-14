//! Tests for polynomial solver

use super::PolynomialSolver;
use crate::algebra::solvers::EquationSolver;
use crate::algebra::solvers::SolverResult;
use crate::core::Expression;
use crate::symbol;

/// Verify that a root actually solves the equation
fn verify_root_solves_equation(
    equation: &Expression,
    variable: &crate::core::Symbol,
    root: &Expression,
) -> bool {
    let solver = PolynomialSolver::new();
    let result = solver.evaluate_polynomial_at(equation, variable, root);
    result.is_zero()
}

#[test]
fn test_cubic_x_cubed_minus_8() {
    let x = symbol!(x);
    let equation = Expression::add(vec![
        Expression::pow(Expression::symbol(x.clone()), Expression::integer(3)),
        Expression::integer(-8),
    ]);

    let solver = PolynomialSolver::new();
    let result = solver.solve(&equation, &x);

    match result {
        SolverResult::Partial(roots) => {
            assert_eq!(roots.len(), 1, "Should find 1 real root");
            assert_eq!(roots[0], Expression::integer(2), "Real root should be 2");

            for root in &roots {
                assert!(
                    verify_root_solves_equation(&equation, &x, root),
                    "Root {:?} does not solve the equation",
                    root
                );
            }
        }
        _ => panic!("Expected Partial result with real root for cubic equation"),
    }
}

#[test]
fn test_cubic_partial_solution_returns_valid_roots() {
    let x = symbol!(x);
    let equation = Expression::add(vec![
        Expression::pow(Expression::symbol(x.clone()), Expression::integer(3)),
        Expression::mul(vec![Expression::integer(2), Expression::symbol(x.clone())]),
        Expression::integer(1),
    ]);

    let solver = PolynomialSolver::new();
    let result = solver.solve(&equation, &x);

    match result {
        SolverResult::Partial(roots) | SolverResult::Multiple(roots) => {
            assert!(!roots.is_empty(), "Should find at least some roots");

            for root in &roots {
                assert!(
                    verify_root_solves_equation(&equation, &x, root),
                    "Root {:?} does not solve the equation",
                    root
                );
            }
        }
        SolverResult::NoSolution => {
            // Acceptable if rational root theorem finds no roots
        }
        _ => panic!("Unexpected solver result type"),
    }
}

#[test]
fn test_quartic_x_fourth_minus_16() {
    let x = symbol!(x);
    let equation = Expression::add(vec![
        Expression::pow(Expression::symbol(x.clone()), Expression::integer(4)),
        Expression::integer(-16),
    ]);

    let solver = PolynomialSolver::new();
    let result = solver.solve(&equation, &x);

    match result {
        SolverResult::Partial(roots) => {
            assert_eq!(roots.len(), 2, "Should find 2 real roots");

            assert!(roots.contains(&Expression::integer(2)), "Should include root 2");
            assert!(roots.contains(&Expression::integer(-2)), "Should include root -2");

            for root in &roots {
                assert!(
                    verify_root_solves_equation(&equation, &x, root),
                    "Root {:?} does not solve the equation",
                    root
                );
            }
        }
        _ => panic!("Expected Partial result with real roots for quartic equation"),
    }
}

#[test]
fn test_no_fake_roots_in_output() {
    let x = symbol!(x);
    let equation = Expression::add(vec![
        Expression::pow(Expression::symbol(x.clone()), Expression::integer(3)),
        Expression::mul(vec![Expression::integer(5), Expression::symbol(x.clone())]),
        Expression::integer(7),
    ]);

    let solver = PolynomialSolver::new();
    let result = solver.solve(&equation, &x);

    match result {
        SolverResult::Multiple(roots) | SolverResult::Partial(roots) => {
            for root in &roots {
                match root {
                    Expression::Function { name, args } if name == "complex" => {
                        if args.len() == 2 {
                            let is_zero_one = match (&args[0], &args[1]) {
                                (Expression::Number(crate::core::Number::Integer(0)), Expression::Number(crate::core::Number::Integer(1))) => true,
                                _ => false,
                            };
                            assert!(
                                !is_zero_one,
                                "Invalid root complex(0, 1) detected - this is an error"
                            );
                        }
                    }
                    _ => {}
                }

                assert!(
                    verify_root_solves_equation(&equation, &x, root),
                    "Root {:?} does not solve the equation - likely a fake root",
                    root
                );
            }
        }
        SolverResult::NoSolution => {
            // Acceptable - rational root theorem may not find roots
        }
        _ => {}
    }
}

#[test]
fn test_partial_result_documented() {
    let x = symbol!(x);
    let equation = Expression::add(vec![
        Expression::pow(Expression::symbol(x.clone()), Expression::integer(3)),
        Expression::mul(vec![Expression::integer(2), Expression::pow(Expression::symbol(x.clone()), Expression::integer(2))]),
        Expression::integer(1),
    ]);

    let solver = PolynomialSolver::new();
    let result = solver.solve(&equation, &x);

    match &result {
        SolverResult::Partial(roots) => {
            assert!(!roots.is_empty(), "Partial should have at least one root");
            for root in roots {
                assert!(
                    verify_root_solves_equation(&equation, &x, root),
                    "Partial solution root {:?} must solve the equation",
                    root
                );
            }
        }
        SolverResult::Multiple(roots) => {
            for root in roots {
                assert!(
                    verify_root_solves_equation(&equation, &x, root),
                    "Multiple solution root {:?} must solve the equation",
                    root
                );
            }
        }
        SolverResult::NoSolution => {}
        _ => panic!("Unexpected result type: {:?}", result),
    }
}
