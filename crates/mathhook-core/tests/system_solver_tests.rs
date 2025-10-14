//! Comprehensive tests for NxN linear system solver
//!
//! Tests cover:
//! - 2x2, 3x3, 4x4, and 5x5 systems
//! - Unique solutions
//! - No solution (inconsistent systems)
//! - Infinite solutions (dependent systems)
//! - Edge cases (zero rows, identity matrix, diagonal systems)

use mathhook_core::algebra::solvers::{SolverResult, SystemEquationSolver, SystemSolver};
use mathhook_core::{symbol, Expression};

#[test]
fn test_2x2_unique_solution() {
    let solver = SystemSolver::new();
    let x = symbol!(x);
    let y = symbol!(y);

    // System: 2x + y = 5, x - y = 1
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
        SolverResult::Multiple(solutions) => {
            assert_eq!(solutions.len(), 2);
            assert_eq!(solutions[0], Expression::integer(2));
            assert_eq!(solutions[1], Expression::integer(1));
        }
        _ => panic!("Expected unique solution, got {:?}", result),
    }
}

#[test]
fn test_3x3_unique_solution() {
    let solver = SystemSolver::new();
    let x = symbol!(x);
    let y = symbol!(y);
    let z = symbol!(z);

    // System: x + y + z = 6, 2x - y + z = 3, x + 2y - z = 3
    // Solution: x = 9/7, y = 15/7, z = 18/7
    let eq1 = Expression::add(vec![
        Expression::symbol(x.clone()),
        Expression::symbol(y.clone()),
        Expression::symbol(z.clone()),
        Expression::integer(-6),
    ]);
    let eq2 = Expression::add(vec![
        Expression::mul(vec![Expression::integer(2), Expression::symbol(x.clone())]),
        Expression::mul(vec![Expression::integer(-1), Expression::symbol(y.clone())]),
        Expression::symbol(z.clone()),
        Expression::integer(-3),
    ]);
    let eq3 = Expression::add(vec![
        Expression::symbol(x.clone()),
        Expression::mul(vec![Expression::integer(2), Expression::symbol(y.clone())]),
        Expression::mul(vec![Expression::integer(-1), Expression::symbol(z.clone())]),
        Expression::integer(-3),
    ]);

    let result = solver.solve_system(&[eq1, eq2, eq3], &[x, y, z]);

    match result {
        SolverResult::Multiple(solutions) => {
            assert_eq!(solutions.len(), 3);
            assert_eq!(solutions[0], Expression::rational(9, 7));
            assert_eq!(solutions[1], Expression::rational(15, 7));
            assert_eq!(solutions[2], Expression::rational(18, 7));
        }
        _ => panic!("Expected unique solution, got {:?}", result),
    }
}

#[test]
fn test_3x3_unique_solution_fractional() {
    let solver = SystemSolver::new();
    let x = symbol!(x);
    let y = symbol!(y);
    let z = symbol!(z);

    // System: 2x + 3y + z = 8, x - y + 2z = 3, 3x + y - z = 4
    let eq1 = Expression::add(vec![
        Expression::mul(vec![Expression::integer(2), Expression::symbol(x.clone())]),
        Expression::mul(vec![Expression::integer(3), Expression::symbol(y.clone())]),
        Expression::symbol(z.clone()),
        Expression::integer(-8),
    ]);
    let eq2 = Expression::add(vec![
        Expression::symbol(x.clone()),
        Expression::mul(vec![Expression::integer(-1), Expression::symbol(y.clone())]),
        Expression::mul(vec![Expression::integer(2), Expression::symbol(z.clone())]),
        Expression::integer(-3),
    ]);
    let eq3 = Expression::add(vec![
        Expression::mul(vec![Expression::integer(3), Expression::symbol(x.clone())]),
        Expression::symbol(y.clone()),
        Expression::mul(vec![Expression::integer(-1), Expression::symbol(z.clone())]),
        Expression::integer(-4),
    ]);

    let result = solver.solve_system(&[eq1, eq2, eq3], &[x, y, z]);

    match result {
        SolverResult::Multiple(solutions) => {
            assert_eq!(solutions.len(), 3);
            assert_eq!(solutions[0], Expression::rational(32, 23));
            assert_eq!(solutions[1], Expression::rational(29, 23));
            assert_eq!(solutions[2], Expression::rational(33, 23));
        }
        _ => panic!("Expected unique solution, got {:?}", result),
    }
}

#[test]
fn test_4x4_unique_solution() {
    let solver = SystemSolver::new();
    let x = symbol!(x);
    let y = symbol!(y);
    let z = symbol!(z);
    let w = symbol!(w);

    // System: x + y + z + w = 10, 2x + y - z + w = 5, x - 2y + 3z + w = 9, x + y + z - 2w = -1
    // Solution: x = 1, y = 7/3, z = 3, w = 11/3
    let eq1 = Expression::add(vec![
        Expression::symbol(x.clone()),
        Expression::symbol(y.clone()),
        Expression::symbol(z.clone()),
        Expression::symbol(w.clone()),
        Expression::integer(-10),
    ]);
    let eq2 = Expression::add(vec![
        Expression::mul(vec![Expression::integer(2), Expression::symbol(x.clone())]),
        Expression::symbol(y.clone()),
        Expression::mul(vec![Expression::integer(-1), Expression::symbol(z.clone())]),
        Expression::symbol(w.clone()),
        Expression::integer(-5),
    ]);
    let eq3 = Expression::add(vec![
        Expression::symbol(x.clone()),
        Expression::mul(vec![Expression::integer(-2), Expression::symbol(y.clone())]),
        Expression::mul(vec![Expression::integer(3), Expression::symbol(z.clone())]),
        Expression::symbol(w.clone()),
        Expression::integer(-9),
    ]);
    let eq4 = Expression::add(vec![
        Expression::symbol(x.clone()),
        Expression::symbol(y.clone()),
        Expression::symbol(z.clone()),
        Expression::mul(vec![Expression::integer(-2), Expression::symbol(w.clone())]),
        Expression::integer(1),
    ]);

    let result = solver.solve_system(&[eq1, eq2, eq3, eq4], &[x, y, z, w]);

    match result {
        SolverResult::Multiple(solutions) => {
            assert_eq!(solutions.len(), 4);
            assert_eq!(solutions[0], Expression::integer(1));
            assert_eq!(solutions[1], Expression::rational(7, 3));
            assert_eq!(solutions[2], Expression::integer(3));
            assert_eq!(solutions[3], Expression::rational(11, 3));
        }
        _ => panic!("Expected unique solution, got {:?}", result),
    }
}

#[test]
fn test_3x3_identity_system() {
    let solver = SystemSolver::new();
    let x = symbol!(x);
    let y = symbol!(y);
    let z = symbol!(z);

    // System: x = 1, y = 2, z = 3 (identity matrix)
    let eq1 = Expression::add(vec![Expression::symbol(x.clone()), Expression::integer(-1)]);
    let eq2 = Expression::add(vec![Expression::symbol(y.clone()), Expression::integer(-2)]);
    let eq3 = Expression::add(vec![Expression::symbol(z.clone()), Expression::integer(-3)]);

    let result = solver.solve_system(&[eq1, eq2, eq3], &[x, y, z]);

    match result {
        SolverResult::Multiple(solutions) => {
            assert_eq!(solutions.len(), 3);
            assert_eq!(solutions[0], Expression::integer(1));
            assert_eq!(solutions[1], Expression::integer(2));
            assert_eq!(solutions[2], Expression::integer(3));
        }
        _ => panic!("Expected unique solution, got {:?}", result),
    }
}

#[test]
fn test_3x3_diagonal_system() {
    let solver = SystemSolver::new();
    let x = symbol!(x);
    let y = symbol!(y);
    let z = symbol!(z);

    // System: 2x = 4, 3y = 9, 4z = 8
    let eq1 = Expression::add(vec![
        Expression::mul(vec![Expression::integer(2), Expression::symbol(x.clone())]),
        Expression::integer(-4),
    ]);
    let eq2 = Expression::add(vec![
        Expression::mul(vec![Expression::integer(3), Expression::symbol(y.clone())]),
        Expression::integer(-9),
    ]);
    let eq3 = Expression::add(vec![
        Expression::mul(vec![Expression::integer(4), Expression::symbol(z.clone())]),
        Expression::integer(-8),
    ]);

    let result = solver.solve_system(&[eq1, eq2, eq3], &[x, y, z]);

    match result {
        SolverResult::Multiple(solutions) => {
            assert_eq!(solutions.len(), 3);
            assert_eq!(solutions[0], Expression::integer(2));
            assert_eq!(solutions[1], Expression::integer(3));
            assert_eq!(solutions[2], Expression::integer(2));
        }
        _ => panic!("Expected unique solution, got {:?}", result),
    }
}

#[test]
fn test_3x3_with_negative_coefficients() {
    let solver = SystemSolver::new();
    let x = symbol!(x);
    let y = symbol!(y);
    let z = symbol!(z);

    // System: -x + 2y - z = -5, 2x - y + 3z = 9, -3x + y - 2z = -7
    // Solution: x = 3/8, y = -9/8, z = 19/8
    let eq1 = Expression::add(vec![
        Expression::mul(vec![Expression::integer(-1), Expression::symbol(x.clone())]),
        Expression::mul(vec![Expression::integer(2), Expression::symbol(y.clone())]),
        Expression::mul(vec![Expression::integer(-1), Expression::symbol(z.clone())]),
        Expression::integer(5),
    ]);
    let eq2 = Expression::add(vec![
        Expression::mul(vec![Expression::integer(2), Expression::symbol(x.clone())]),
        Expression::mul(vec![Expression::integer(-1), Expression::symbol(y.clone())]),
        Expression::mul(vec![Expression::integer(3), Expression::symbol(z.clone())]),
        Expression::integer(-9),
    ]);
    let eq3 = Expression::add(vec![
        Expression::mul(vec![Expression::integer(-3), Expression::symbol(x.clone())]),
        Expression::symbol(y.clone()),
        Expression::mul(vec![Expression::integer(-2), Expression::symbol(z.clone())]),
        Expression::integer(7),
    ]);

    let result = solver.solve_system(&[eq1, eq2, eq3], &[x, y, z]);

    match result {
        SolverResult::Multiple(solutions) => {
            assert_eq!(solutions.len(), 3);
            assert_eq!(solutions[0], Expression::rational(3, 8));
            assert_eq!(solutions[1], Expression::rational(-9, 8));
            assert_eq!(solutions[2], Expression::rational(19, 8));
        }
        _ => panic!("Expected unique solution, got {:?}", result),
    }
}

#[test]
fn test_2x2_no_solution() {
    let solver = SystemSolver::new();
    let x = symbol!(x);
    let y = symbol!(y);

    // System: x + y = 1, x + y = 2
    // Inconsistent (no solution)
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

    let result = solver.solve_system(&[eq1, eq2], &[x, y]);

    assert!(
        matches!(result, SolverResult::NoSolution),
        "Expected no solution for inconsistent system"
    );
}

#[test]
fn test_2x2_infinite_solutions() {
    let solver = SystemSolver::new();
    let x = symbol!(x);
    let y = symbol!(y);

    // System: 2x + y = 3, 4x + 2y = 6
    // Dependent (infinite solutions)
    let eq1 = Expression::add(vec![
        Expression::mul(vec![Expression::integer(2), Expression::symbol(x.clone())]),
        Expression::symbol(y.clone()),
        Expression::integer(-3),
    ]);
    let eq2 = Expression::add(vec![
        Expression::mul(vec![Expression::integer(4), Expression::symbol(x.clone())]),
        Expression::mul(vec![Expression::integer(2), Expression::symbol(y.clone())]),
        Expression::integer(-6),
    ]);

    let result = solver.solve_system(&[eq1, eq2], &[x, y]);

    assert!(
        matches!(result, SolverResult::InfiniteSolutions),
        "Expected infinite solutions for dependent system"
    );
}

#[test]
fn test_3x3_no_solution() {
    let solver = SystemSolver::new();
    let x = symbol!(x);
    let y = symbol!(y);
    let z = symbol!(z);

    // System: x + y + z = 1, 2x + 2y + 2z = 3, x + y + z = 1
    let eq1 = Expression::add(vec![
        Expression::symbol(x.clone()),
        Expression::symbol(y.clone()),
        Expression::symbol(z.clone()),
        Expression::integer(-1),
    ]);
    let eq2 = Expression::add(vec![
        Expression::mul(vec![Expression::integer(2), Expression::symbol(x.clone())]),
        Expression::mul(vec![Expression::integer(2), Expression::symbol(y.clone())]),
        Expression::mul(vec![Expression::integer(2), Expression::symbol(z.clone())]),
        Expression::integer(-3),
    ]);
    let eq3 = Expression::add(vec![
        Expression::symbol(x.clone()),
        Expression::symbol(y.clone()),
        Expression::symbol(z.clone()),
        Expression::integer(-1),
    ]);

    let result = solver.solve_system(&[eq1, eq2, eq3], &[x, y, z]);

    assert!(
        matches!(result, SolverResult::NoSolution),
        "Expected no solution for inconsistent 3x3 system"
    );
}

#[test]
fn test_5x5_unique_solution() {
    let solver = SystemSolver::new();
    let x1 = symbol!(x1);
    let x2 = symbol!(x2);
    let x3 = symbol!(x3);
    let x4 = symbol!(x4);
    let x5 = symbol!(x5);

    // System: x1 + x2 + x3 + x4 + x5 = 15, etc.
    // Solution: x1 = 1, x2 = 2, x3 = 3, x4 = 4, x5 = 5
    let eq1 = Expression::add(vec![
        Expression::symbol(x1.clone()),
        Expression::symbol(x2.clone()),
        Expression::symbol(x3.clone()),
        Expression::symbol(x4.clone()),
        Expression::symbol(x5.clone()),
        Expression::integer(-15),
    ]);
    let eq2 = Expression::add(vec![
        Expression::mul(vec![Expression::integer(2), Expression::symbol(x1.clone())]),
        Expression::symbol(x2.clone()),
        Expression::symbol(x3.clone()),
        Expression::symbol(x4.clone()),
        Expression::symbol(x5.clone()),
        Expression::integer(-16),
    ]);
    let eq3 = Expression::add(vec![
        Expression::symbol(x1.clone()),
        Expression::mul(vec![Expression::integer(2), Expression::symbol(x2.clone())]),
        Expression::symbol(x3.clone()),
        Expression::symbol(x4.clone()),
        Expression::symbol(x5.clone()),
        Expression::integer(-17),
    ]);
    let eq4 = Expression::add(vec![
        Expression::symbol(x1.clone()),
        Expression::symbol(x2.clone()),
        Expression::mul(vec![Expression::integer(2), Expression::symbol(x3.clone())]),
        Expression::symbol(x4.clone()),
        Expression::symbol(x5.clone()),
        Expression::integer(-18),
    ]);
    let eq5 = Expression::add(vec![
        Expression::symbol(x1.clone()),
        Expression::symbol(x2.clone()),
        Expression::symbol(x3.clone()),
        Expression::mul(vec![Expression::integer(2), Expression::symbol(x4.clone())]),
        Expression::symbol(x5.clone()),
        Expression::integer(-19),
    ]);

    let result = solver.solve_system(&[eq1, eq2, eq3, eq4, eq5], &[x1, x2, x3, x4, x5]);

    match result {
        SolverResult::Multiple(solutions) => {
            assert_eq!(solutions.len(), 5);
            assert_eq!(solutions[0], Expression::integer(1));
            assert_eq!(solutions[1], Expression::integer(2));
            assert_eq!(solutions[2], Expression::integer(3));
            assert_eq!(solutions[3], Expression::integer(4));
            assert_eq!(solutions[4], Expression::integer(5));
        }
        _ => panic!("Expected unique solution for 5x5 system, got {:?}", result),
    }
}

#[test]
fn test_empty_system() {
    let solver = SystemSolver::new();
    let result = solver.solve_system(&[], &[]);

    assert!(
        matches!(result, SolverResult::NoSolution),
        "Expected no solution for empty system"
    );
}

#[test]
fn test_non_square_system() {
    let solver = SystemSolver::new();
    let x = symbol!(x);
    let y = symbol!(y);
    let z = symbol!(z);

    // 2 equations, 3 variables (underdetermined)
    let eq1 = Expression::add(vec![
        Expression::symbol(x.clone()),
        Expression::symbol(y.clone()),
        Expression::symbol(z.clone()),
        Expression::integer(-1),
    ]);
    let eq2 = Expression::add(vec![
        Expression::symbol(x.clone()),
        Expression::symbol(y.clone()),
        Expression::symbol(z.clone()),
        Expression::integer(-2),
    ]);

    let result = solver.solve_system(&[eq1, eq2], &[x, y, z]);

    assert!(
        matches!(result, SolverResult::NoSolution),
        "Expected no solution for non-square system"
    );
}

#[test]
fn test_3x3_requires_pivoting() {
    let solver = SystemSolver::new();
    let x = symbol!(x);
    let y = symbol!(y);
    let z = symbol!(z);

    // System that requires row pivoting: 0x + y + z = 3, x + y + z = 6, x + 0y + z = 4
    // Solution: x = 3, y = 2, z = 1
    let eq1 = Expression::add(vec![
        Expression::symbol(y.clone()),
        Expression::symbol(z.clone()),
        Expression::integer(-3),
    ]);
    let eq2 = Expression::add(vec![
        Expression::symbol(x.clone()),
        Expression::symbol(y.clone()),
        Expression::symbol(z.clone()),
        Expression::integer(-6),
    ]);
    let eq3 = Expression::add(vec![
        Expression::symbol(x.clone()),
        Expression::symbol(z.clone()),
        Expression::integer(-4),
    ]);

    let result = solver.solve_system(&[eq1, eq2, eq3], &[x, y, z]);

    match result {
        SolverResult::Multiple(solutions) => {
            assert_eq!(solutions.len(), 3);
            assert_eq!(solutions[0], Expression::integer(3));
            assert_eq!(solutions[1], Expression::integer(2));
            assert_eq!(solutions[2], Expression::integer(1));
        }
        _ => panic!("Expected unique solution with pivoting, got {:?}", result),
    }
}

#[test]
fn test_3x3_with_large_coefficients() {
    let solver = SystemSolver::new();
    let x = symbol!(x);
    let y = symbol!(y);
    let z = symbol!(z);

    // System: 10x + 5y + 2z = 44, 3x + 2y + z = 17, 5x + 3y + 2z = 26
    // Solution: x = 2/3, y = 22/3, z = 1/3
    let eq1 = Expression::add(vec![
        Expression::mul(vec![Expression::integer(10), Expression::symbol(x.clone())]),
        Expression::mul(vec![Expression::integer(5), Expression::symbol(y.clone())]),
        Expression::mul(vec![Expression::integer(2), Expression::symbol(z.clone())]),
        Expression::integer(-44),
    ]);
    let eq2 = Expression::add(vec![
        Expression::mul(vec![Expression::integer(3), Expression::symbol(x.clone())]),
        Expression::mul(vec![Expression::integer(2), Expression::symbol(y.clone())]),
        Expression::symbol(z.clone()),
        Expression::integer(-17),
    ]);
    let eq3 = Expression::add(vec![
        Expression::mul(vec![Expression::integer(5), Expression::symbol(x.clone())]),
        Expression::mul(vec![Expression::integer(3), Expression::symbol(y.clone())]),
        Expression::mul(vec![Expression::integer(2), Expression::symbol(z.clone())]),
        Expression::integer(-26),
    ]);

    let result = solver.solve_system(&[eq1, eq2, eq3], &[x, y, z]);

    match result {
        SolverResult::Multiple(solutions) => {
            assert_eq!(solutions.len(), 3);
            assert_eq!(solutions[0], Expression::rational(2, 3));
            assert_eq!(solutions[1], Expression::rational(22, 3));
            assert_eq!(solutions[2], Expression::rational(1, 3));
        }
        _ => panic!("Expected unique solution, got {:?}", result),
    }
}
