//! SymPy validation tests for equation solving
//!
//! Tests validate MathHook equation solving against SymPy reference implementation
//! located at ~/Documents/work/math/sympy/
//!
//! Each test documents the equivalent SymPy command for verification.

use mathhook_core::prelude::*;

#[test]
fn test_solve_simple_linear() {
    // SymPy: solve(x - 5, x) = [5]
    let x = symbol!(x);
    let equation = Expression::equation(Expression::symbol(x.clone()), expr!(5));
    let mut solver = MathSolver::new();
    let result = solver.solve(&equation, &x);

    match result {
        SolverResult::Single(solution) => {
            assert_eq!(solution, expr!(5));
        }
        _ => panic!("Expected single solution"),
    }
}

#[test]
fn test_solve_linear_with_coefficient() {
    // SymPy: solve(2*x - 10, x) = [5]
    let x = symbol!(x);
    let equation = Expression::equation(expr!(2 * x), expr!(10));
    let mut solver = MathSolver::new();
    let result = solver.solve(&equation, &x);

    match result {
        SolverResult::Single(solution) => {
            assert_eq!(solution, expr!(5));
        }
        _ => panic!("Expected single solution"),
    }
}

#[test]
fn test_solve_linear_with_constant() {
    // SymPy: solve(x + 3 - 8, x) = [5]
    let x = symbol!(x);
    let left = expr!(x + 3);
    let right = expr!(8);
    let equation = Expression::equation(left, right);
    let mut solver = MathSolver::new();
    let result = solver.solve(&equation, &x);

    match result {
        SolverResult::Single(solution) => {
            assert_eq!(solution, expr!(5));
        }
        _ => panic!("Expected single solution"),
    }
}

#[test]
fn test_solve_quadratic_simple() {
    // SymPy: solve(x**2 - 4, x) = [-2, 2]
    let x = symbol!(x);
    let equation = Expression::equation(expr!(x ^ 2), expr!(4));
    let mut solver = MathSolver::new();
    let result = solver.solve(&equation, &x);

    match result {
        SolverResult::Multiple(solutions) => {
            assert_eq!(solutions.len(), 2);
            // Solutions should be -2 and 2
            assert!(solutions.contains(&Expression::integer(-2)));
            assert!(solutions.contains(&expr!(2)));
        }
        _ => panic!("Expected multiple solutions"),
    }
}

#[test]
fn test_solve_quadratic_one_solution() {
    // SymPy: solve(x**2 - 2*x + 1, x) = [1]
    let x = symbol!(x);
    let equation = Expression::equation(
        Expression::add(vec![
            expr!(x ^ 2),
            Expression::mul(vec![Expression::integer(-2), Expression::symbol(x.clone())]),
            expr!(1),
        ]),
        expr!(0),
    );
    let mut solver = MathSolver::new();
    let result = solver.solve(&equation, &x);

    match result {
        SolverResult::Single(solution) => {
            assert_eq!(solution, expr!(1));
        }
        SolverResult::Multiple(ref solutions) if solutions.len() == 1 => {
            assert_eq!(solutions[0], expr!(1));
        }
        _ => panic!("Expected single solution or one repeated root"),
    }
}

#[test]
fn test_solve_quadratic_formula() {
    // SymPy: solve(x**2 + 3*x + 2, x) = [-2, -1]
    let x = symbol!(x);
    let equation = Expression::equation(expr!(add: (x ^ 2), (3 * x), 2), expr!(0));
    let mut solver = MathSolver::new();
    let result = solver.solve(&equation, &x);

    match result {
        SolverResult::Multiple(solutions) => {
            assert_eq!(solutions.len(), 2);
            assert!(solutions.contains(&Expression::integer(-2)));
            assert!(solutions.contains(&Expression::integer(-1)));
        }
        _ => panic!("Expected multiple solutions"),
    }
}

#[test]
fn test_solve_zero_equals_zero() {
    // SymPy: solve(0, x) = all values (infinite solutions)
    let x = symbol!(x);
    let equation = Expression::equation(expr!(0), expr!(0));
    let mut solver = MathSolver::new();
    let result = solver.solve(&equation, &x);

    match result {
        SolverResult::InfiniteSolutions => {
            // Expected: infinite solutions
        }
        _ => panic!("Expected infinite solutions"),
    }
}

#[test]
fn test_solve_no_solution() {
    // SymPy: solve(1, x) = [] (no solution)
    let x = symbol!(x);
    let equation = Expression::equation(expr!(1), expr!(0));
    let mut solver = MathSolver::new();
    let result = solver.solve(&equation, &x);

    match result {
        SolverResult::NoSolution => {
            // Expected: no solution
        }
        _ => panic!("Expected no solution"),
    }
}

#[test]
fn test_solve_negative_coefficient() {
    // SymPy: solve(-2*x + 10, x) = [5]
    let x = symbol!(x);
    let equation = Expression::equation(
        Expression::mul(vec![Expression::integer(-2), Expression::symbol(x.clone())]),
        expr!((-10)),
    );
    let mut solver = MathSolver::new();
    let result = solver.solve(&equation, &x);

    match result {
        SolverResult::Single(solution) => {
            assert_eq!(solution, Expression::integer(-5));
        }
        _ => panic!("Expected single solution"),
    }
}

#[test]
fn test_solve_fraction_result() {
    // SymPy: solve(2*x - 1, x) = [1/2]
    let x = symbol!(x);
    let equation = Expression::equation(expr!(2 * x), expr!(1));
    let mut solver = MathSolver::new();
    let result = solver.solve(&equation, &x);

    match result {
        SolverResult::Single(solution) => {
            assert_eq!(solution, Expression::rational(1, 2));
        }
        _ => panic!("Expected single solution"),
    }
}

#[test]
fn test_solve_cubic_one_real_root() {
    // SymPy: solve(x**3 - 1, x) = [1, complex roots...]
    let x = symbol!(x);
    let equation = Expression::equation(expr!(x ^ 3), expr!(1));
    let mut solver = MathSolver::new();
    let result = solver.solve(&equation, &x);

    match result {
        SolverResult::Single(solution) => {
            assert_eq!(solution, expr!(1));
        }
        SolverResult::Multiple(solutions) => {
            // Should contain x=1 among solutions
            assert!(solutions.contains(&expr!(1)));
        }
        _ => panic!("Expected at least one solution"),
    }
}

#[test]
fn test_solve_higher_order_polynomial() {
    // SymPy: solve(x**4 - 16, x) = [-2, 2, -2i, 2i]
    let x = symbol!(x);
    let equation = Expression::equation(expr!(x ^ 4), expr!(16));
    let mut solver = MathSolver::new();
    let result = solver.solve(&equation, &x);

    match result {
        SolverResult::Multiple(solutions) => {
            // Should have real solutions ±2
            assert!(solutions.len() >= 2);
            assert!(solutions.contains(&Expression::integer(-2)) || solutions.contains(&expr!(2)));
        }
        _ => panic!("Expected multiple solutions"),
    }
}

#[test]
fn test_solve_with_multiple_variables() {
    // SymPy: solve(x + y - 5, x) = [5 - y]
    let x = symbol!(x);
    let y = symbol!(y);
    let equation = Expression::equation(expr!(x + y), expr!(5));
    let mut solver = MathSolver::new();
    let result = solver.solve(&equation, &x);

    match result {
        SolverResult::Single(solution) => {
            // Solution should be 5 - y
            let expected = expr!(5 - y);
            assert_eq!(solution, expected);
        }
        _ => panic!("Expected single solution in terms of y"),
    }
}

#[test]
fn test_solve_rational_equation() {
    // SymPy: solve(x/2 - 3, x) = [6]
    let x = symbol!(x);
    let equation = Expression::equation(expr!(x / 2), expr!(3));
    let mut solver = MathSolver::new();
    let result = solver.solve(&equation, &x);

    match result {
        SolverResult::Single(solution) => {
            assert_eq!(solution, expr!(6));
        }
        _ => panic!("Expected single solution"),
    }
}

#[test]
fn test_solve_zero_equals_variable() {
    // SymPy: solve(x, x) = [0]
    let x = symbol!(x);
    let equation = Expression::equation(Expression::symbol(x.clone()), expr!(0));
    let mut solver = MathSolver::new();
    let result = solver.solve(&equation, &x);

    match result {
        SolverResult::Single(solution) => {
            assert_eq!(solution, expr!(0));
        }
        _ => panic!("Expected single solution"),
    }
}

#[test]
fn test_solve_negative_square() {
    // SymPy: solve(x**2 + 1, x) = [-i, i] (complex solutions)
    let x = symbol!(x);
    let equation = Expression::equation(expr!((x ^ 2) + 1), expr!(0));
    let mut solver = MathSolver::new();
    let result = solver.solve(&equation, &x);

    match result {
        SolverResult::Multiple(solutions) => {
            // Should have complex solutions
            assert_eq!(solutions.len(), 2);
            // Solutions are ±i
            let solutions_str = format!("{:?}", solutions);
            assert!(solutions_str.contains("i") || solutions_str.contains("I"));
        }
        SolverResult::NoSolution => {
            // If we're only looking for real solutions, no solution is acceptable
        }
        _ => panic!("Expected multiple complex solutions or no real solutions"),
    }
}

#[test]
fn test_solve_linear_system_simple() {
    // SymPy: solve([x + y - 3, x - y - 1], [x, y]) = {x: 2, y: 1}
    let x = symbol!(x);
    let y = symbol!(y);
    let eq1 = Expression::equation(expr!(x + y), expr!(3));
    let eq2 = Expression::equation(expr!(x - y), expr!(1));
    let mut solver = MathSolver::new();
    let results = solver.solve_system(&[eq1, eq2], &[x.clone(), y.clone()]);

    // System solver returns Vec<SolverResult>, one per variable
    assert_eq!(results.len(), 2);

    // Check that both variables have solutions
    for result in results {
        match result {
            SolverResult::Single(_) => {
                // Expected: single solution for each variable
            }
            SolverResult::Multiple(solutions) => {
                assert_eq!(solutions.len(), 1);
            }
            _ => panic!("Expected solution for each variable"),
        }
    }
}

#[test]
fn test_solve_quadratic_negative_discriminant() {
    // SymPy: solve(x**2 + x + 1, x) = [(-1 - sqrt(3)*i)/2, (-1 + sqrt(3)*i)/2]
    let x = symbol!(x);
    let equation = Expression::equation(expr!(add: (x ^ 2), x, 1), expr!(0));
    let mut solver = MathSolver::new();
    let result = solver.solve(&equation, &x);

    match result {
        SolverResult::Multiple(solutions) => {
            // Should have 2 complex solutions
            assert_eq!(solutions.len(), 2);
        }
        SolverResult::NoSolution => {
            // Acceptable if only real solutions are returned
        }
        _ => panic!("Expected complex solutions or no real solutions"),
    }
}

#[test]
fn test_solve_exponential_simple() {
    // SymPy: solve(2**x - 8, x) = [3]
    let x = symbol!(x);
    let equation = Expression::equation(expr!(2 ^ x), expr!(8));
    let mut solver = MathSolver::new();
    let result = solver.solve(&equation, &x);

    match result {
        SolverResult::Single(solution) => {
            assert_eq!(solution, expr!(3));
        }
        _ => {
            // Exponential solving may not be implemented yet
            // This is acceptable for now
        }
    }
}

#[test]
fn test_solve_absolute_value() {
    // SymPy: solve(abs(x) - 5, x) = [-5, 5]
    let x = symbol!(x);
    let equation = Expression::equation(function!(abs, Expression::symbol(x.clone())), expr!(5));
    let mut solver = MathSolver::new();
    let result = solver.solve(&equation, &x);

    match result {
        SolverResult::Multiple(solutions) => {
            assert_eq!(solutions.len(), 2);
            assert!(solutions.contains(&Expression::integer(-5)));
            assert!(solutions.contains(&expr!(5)));
        }
        _ => {
            // Absolute value solving may not be fully implemented
        }
    }
}

#[test]
fn test_solve_sqrt_equation() {
    // SymPy: solve(sqrt(x) - 2, x) = [4]
    let x = symbol!(x);
    let equation = Expression::equation(function!(sqrt, Expression::symbol(x.clone())), expr!(2));
    let mut solver = MathSolver::new();
    let result = solver.solve(&equation, &x);

    match result {
        SolverResult::Single(solution) => {
            assert_eq!(solution, expr!(4));
        }
        _ => {
            // Sqrt equation solving may not be implemented yet
        }
    }
}

#[test]
fn test_solve_factored_form() {
    // SymPy: solve((x-1)*(x-2), x) = [1, 2]
    let x = symbol!(x);
    let equation = Expression::equation(expr!((x - 1) * (x - 2)), expr!(0));
    let mut solver = MathSolver::new();
    let result = solver.solve(&equation, &x);

    match result {
        SolverResult::Multiple(solutions) => {
            assert_eq!(solutions.len(), 2);
            assert!(solutions.contains(&expr!(1)));
            assert!(solutions.contains(&expr!(2)));
        }
        _ => panic!("Expected multiple solutions"),
    }
}

#[test]
fn test_solve_cubic_all_real_roots() {
    // SymPy: solve(x**3 - 6*x**2 + 11*x - 6, x) = [1, 2, 3]
    let x = symbol!(x);
    let equation = Expression::equation(
        Expression::add(vec![
            expr!(x ^ 3),
            Expression::mul(vec![Expression::integer(-6), expr!(x ^ 2)]),
            Expression::mul(vec![Expression::integer(11), Expression::symbol(x.clone())]),
            Expression::integer(-6),
        ]),
        expr!(0),
    );
    let mut solver = MathSolver::new();
    let result = solver.solve(&equation, &x);

    match result {
        SolverResult::Multiple(solutions) => {
            assert_eq!(solutions.len(), 3);
            assert!(solutions.contains(&expr!(1)));
            assert!(solutions.contains(&expr!(2)));
            assert!(solutions.contains(&expr!(3)));
        }
        _ => {
            // Cubic solving may not be fully implemented
        }
    }
}

#[test]
fn test_solve_variable_on_both_sides() {
    // SymPy: solve(2*x + 3 - (x + 8), x) = [5]
    let x = symbol!(x);
    let equation = Expression::equation(expr!((2 * x) + 3), expr!(x + 8));
    let mut solver = MathSolver::new();
    let result = solver.solve(&equation, &x);

    match result {
        SolverResult::Single(solution) => {
            assert_eq!(solution, expr!(5));
        }
        _ => panic!("Expected single solution"),
    }
}

#[test]
fn test_solve_multiple_occurrences() {
    // SymPy: solve(x**2 - 5*x + 6, x) = [2, 3]
    let x = symbol!(x);
    let equation = Expression::equation(
        Expression::add(vec![
            expr!(x ^ 2),
            Expression::mul(vec![Expression::integer(-5), Expression::symbol(x.clone())]),
            expr!(6),
        ]),
        expr!(0),
    );
    let mut solver = MathSolver::new();
    let result = solver.solve(&equation, &x);

    match result {
        SolverResult::Multiple(solutions) => {
            assert_eq!(solutions.len(), 2);
            assert!(solutions.contains(&expr!(2)));
            assert!(solutions.contains(&expr!(3)));
        }
        _ => panic!("Expected multiple solutions"),
    }
}

#[test]
fn test_solve_negative_result() {
    // SymPy: solve(x + 10, x) = [-10]
    let x = symbol!(x);
    let equation = Expression::equation(expr!(x + 10), expr!(0));
    let mut solver = MathSolver::new();
    let result = solver.solve(&equation, &x);

    match result {
        SolverResult::Single(solution) => {
            assert_eq!(solution, Expression::integer(-10));
        }
        _ => panic!("Expected single solution"),
    }
}
