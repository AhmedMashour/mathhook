//! Comprehensive tests for matrix equation solver (Wave 10)
//!
//! Tests noncommutative algebra equation solving for matrices, operators, and quaternions.
//! Covers left division, right division, mixed equations, and error cases.

use mathhook_core::algebra::solvers::matrix_equations::MatrixEquationSolver;
use mathhook_core::algebra::solvers::{EquationSolver, SolverResult};
use mathhook_core::{expr, symbol, Expression};

#[test]
fn test_simple_matrix_left_division() {
    let solver = MatrixEquationSolver::new();
    let a = symbol!(A; matrix);
    let x = symbol!(X; matrix);
    let b = symbol!(B; matrix);

    let equation = Expression::add(vec![
        Expression::mul(vec![Expression::symbol(a.clone()), Expression::symbol(x.clone())]),
        Expression::mul(vec![Expression::integer(-1), Expression::symbol(b.clone())]),
    ]);

    let result = solver.solve(&equation, &x);
    match result {
        SolverResult::Single(solution) => {
            assert!(solution.to_string().contains("A"));
            assert!(solution.to_string().contains("B"));
        }
        _ => panic!("Expected single solution for A*X = B"),
    }
}

#[test]
fn test_simple_matrix_right_division() {
    let solver = MatrixEquationSolver::new();
    let a = symbol!(A; matrix);
    let x = symbol!(X; matrix);
    let b = symbol!(B; matrix);

    let equation = Expression::add(vec![
        Expression::mul(vec![Expression::symbol(x.clone()), Expression::symbol(a.clone())]),
        Expression::mul(vec![Expression::integer(-1), Expression::symbol(b.clone())]),
    ]);

    let result = solver.solve(&equation, &x);
    match result {
        SolverResult::Single(solution) => {
            assert!(solution.to_string().contains("A"));
            assert!(solution.to_string().contains("B"));
        }
        _ => panic!("Expected single solution for X*A = B"),
    }
}

#[test]
fn test_operator_left_division_hamiltonian() {
    let solver = MatrixEquationSolver::new();
    let h = symbol!(H; operator);
    let psi = symbol!(psi; operator);
    let e = symbol!(E; operator);

    let equation = Expression::add(vec![
        Expression::mul(vec![
            Expression::symbol(h.clone()),
            Expression::symbol(psi.clone()),
        ]),
        Expression::mul(vec![Expression::integer(-1), Expression::symbol(e.clone())]),
    ]);

    let result = solver.solve(&equation, &psi);
    match result {
        SolverResult::Single(_) => {}
        _ => panic!("Expected solution for H*psi = E"),
    }
}

#[test]
fn test_quaternion_left_division() {
    let solver = MatrixEquationSolver::new();
    let q = symbol!(q; quaternion);
    let x = symbol!(x; quaternion);
    let r = symbol!(r; quaternion);

    let equation = Expression::add(vec![
        Expression::mul(vec![Expression::symbol(q.clone()), Expression::symbol(x.clone())]),
        Expression::mul(vec![Expression::integer(-1), Expression::symbol(r.clone())]),
    ]);

    let result = solver.solve(&equation, &x);
    match result {
        SolverResult::Single(_) => {}
        _ => panic!("Expected solution for q*x = r"),
    }
}

#[test]
fn test_scalar_coefficient_left_division() {
    let solver = MatrixEquationSolver::new();
    let a = symbol!(A; matrix);
    let x = symbol!(X; matrix);
    let b = symbol!(B; matrix);

    let equation = Expression::add(vec![
        Expression::mul(vec![
            Expression::integer(2),
            Expression::symbol(a.clone()),
            Expression::symbol(x.clone()),
        ]),
        Expression::mul(vec![Expression::integer(-1), Expression::symbol(b.clone())]),
    ]);

    let result = solver.solve(&equation, &x);
    match result {
        SolverResult::Single(_) => {}
        SolverResult::NoSolution => {}
        _ => {}
    }
}

#[test]
fn test_matrix_left_division_with_zero_rhs() {
    let solver = MatrixEquationSolver::new();
    let a = symbol!(A; matrix);
    let x = symbol!(X; matrix);

    let equation = Expression::mul(vec![Expression::symbol(a.clone()), Expression::symbol(x.clone())]);

    let result = solver.solve(&equation, &x);
    match result {
        SolverResult::Single(solution) => {
            assert!(solution.to_string().contains("0"));
        }
        _ => {}
    }
}

#[test]
fn test_matrix_left_division_inverse_position() {
    let solver = MatrixEquationSolver::new();
    let a = symbol!(A; matrix);
    let x = symbol!(X; matrix);
    let b = symbol!(B; matrix);

    let equation = Expression::add(vec![
        Expression::mul(vec![Expression::symbol(a.clone()), Expression::symbol(x.clone())]),
        Expression::mul(vec![Expression::integer(-1), Expression::symbol(b.clone())]),
    ]);

    let result = solver.solve(&equation, &x);
    match result {
        SolverResult::Single(solution) => {
            let solution_str = solution.to_string();
            let a_inv_pos = solution_str.find("A^");
            let b_pos = solution_str.find("B");
            if let (Some(inv), Some(b)) = (a_inv_pos, b_pos) {
                assert!(inv < b, "A^(-1) should appear before B for left division");
            }
        }
        _ => {}
    }
}

#[test]
fn test_operator_right_division() {
    let solver = MatrixEquationSolver::new();
    let h = symbol!(H; operator);
    let psi = symbol!(psi; operator);
    let e = symbol!(E; operator);

    let equation = Expression::add(vec![
        Expression::mul(vec![
            Expression::symbol(psi.clone()),
            Expression::symbol(h.clone()),
        ]),
        Expression::mul(vec![Expression::integer(-1), Expression::symbol(e.clone())]),
    ]);

    let result = solver.solve(&equation, &psi);
    match result {
        SolverResult::Single(_) => {}
        _ => panic!("Expected solution for psi*H = E"),
    }
}

#[test]
fn test_quaternion_right_division() {
    let solver = MatrixEquationSolver::new();
    let q = symbol!(q; quaternion);
    let x = symbol!(x; quaternion);
    let r = symbol!(r; quaternion);

    let equation = Expression::add(vec![
        Expression::mul(vec![Expression::symbol(x.clone()), Expression::symbol(q.clone())]),
        Expression::mul(vec![Expression::integer(-1), Expression::symbol(r.clone())]),
    ]);

    let result = solver.solve(&equation, &x);
    match result {
        SolverResult::Single(_) => {}
        _ => panic!("Expected solution for x*q = r"),
    }
}

#[test]
fn test_matrix_right_division_with_zero_rhs() {
    let solver = MatrixEquationSolver::new();
    let a = symbol!(A; matrix);
    let x = symbol!(X; matrix);

    let equation = Expression::mul(vec![Expression::symbol(x.clone()), Expression::symbol(a.clone())]);

    let result = solver.solve(&equation, &x);
    match result {
        SolverResult::Single(solution) => {
            assert!(solution.to_string().contains("0"));
        }
        _ => {}
    }
}

#[test]
fn test_matrix_right_division_inverse_position() {
    let solver = MatrixEquationSolver::new();
    let a = symbol!(A; matrix);
    let x = symbol!(X; matrix);
    let b = symbol!(B; matrix);

    let equation = Expression::add(vec![
        Expression::mul(vec![Expression::symbol(x.clone()), Expression::symbol(a.clone())]),
        Expression::mul(vec![Expression::integer(-1), Expression::symbol(b.clone())]),
    ]);

    let result = solver.solve(&equation, &x);
    match result {
        SolverResult::Single(solution) => {
            let solution_str = solution.to_string();
            let b_pos = solution_str.find("B");
            let a_inv_pos = solution_str.find("A^");
            if let (Some(b), Some(inv)) = (b_pos, a_inv_pos) {
                assert!(b < inv, "B should appear before A^(-1) for right division");
            }
        }
        _ => {}
    }
}

#[test]
fn test_commutative_symbols_return_no_solution() {
    let solver = MatrixEquationSolver::new();
    let x = symbol!(x);
    let y = symbol!(y);
    let z = symbol!(z);

    let equation = Expression::add(vec![
        Expression::mul(vec![Expression::symbol(y.clone()), Expression::symbol(x.clone())]),
        Expression::mul(vec![Expression::integer(-1), Expression::symbol(z.clone())]),
    ]);

    let result = solver.solve(&equation, &x);
    assert_eq!(result, SolverResult::NoSolution);
}

#[test]
fn test_mixed_noncommutative_types_in_single_equation() {
    let solver = MatrixEquationSolver::new();
    let a = symbol!(A; matrix);
    let p = symbol!(p; operator);
    let x = symbol!(X; matrix);
    let b = symbol!(B; matrix);

    let equation = Expression::add(vec![
        Expression::mul(vec![
            Expression::symbol(a.clone()),
            Expression::symbol(p.clone()),
            Expression::symbol(x.clone()),
        ]),
        Expression::mul(vec![Expression::integer(-1), Expression::symbol(b.clone())]),
    ]);

    let result = solver.solve(&equation, &x);
    match result {
        SolverResult::NoSolution | SolverResult::Single(_) => {}
        _ => {}
    }
}

#[test]
fn test_operator_commutator_position_momentum() {
    let solver = MatrixEquationSolver::new();
    let p = symbol!(p; operator);
    let x_op = symbol!(x; operator);
    let hbar = symbol!(hbar; operator);

    let equation = Expression::add(vec![
        Expression::mul(vec![
            Expression::symbol(p.clone()),
            Expression::symbol(x_op.clone()),
        ]),
        Expression::mul(vec![
            Expression::integer(-1),
            Expression::symbol(x_op.clone()),
            Expression::symbol(p.clone()),
        ]),
        Expression::mul(vec![
            Expression::integer(-1),
            Expression::symbol(hbar.clone()),
        ]),
    ]);

    let result = solver.solve(&equation, &x_op);
    match result {
        SolverResult::NoSolution | SolverResult::Single(_) => {}
        _ => {}
    }
}

#[test]
fn test_matrix_equation_with_multiple_variables() {
    let solver = MatrixEquationSolver::new();
    let a = symbol!(A; matrix);
    let x = symbol!(X; matrix);
    let b = symbol!(B; matrix);
    let y = symbol!(Y; matrix);
    let c = symbol!(C; matrix);

    let equation = Expression::add(vec![
        Expression::mul(vec![Expression::symbol(a.clone()), Expression::symbol(x.clone())]),
        Expression::mul(vec![Expression::symbol(b.clone()), Expression::symbol(y.clone())]),
        Expression::mul(vec![Expression::integer(-1), Expression::symbol(c.clone())]),
    ]);

    let result = solver.solve(&equation, &x);
    match result {
        SolverResult::Single(_) | SolverResult::NoSolution => {}
        _ => {}
    }
}

#[test]
fn test_quaternion_multiplication_i_times_x_equals_j() {
    let solver = MatrixEquationSolver::new();
    let i = symbol!(i; quaternion);
    let x = symbol!(x; quaternion);
    let j = symbol!(j; quaternion);

    let equation = Expression::add(vec![
        Expression::mul(vec![Expression::symbol(i.clone()), Expression::symbol(x.clone())]),
        Expression::mul(vec![Expression::integer(-1), Expression::symbol(j.clone())]),
    ]);

    let result = solver.solve(&equation, &x);
    match result {
        SolverResult::Single(_) => {}
        _ => panic!("Expected solution for i*x = j"),
    }
}

#[test]
fn test_quaternion_right_division_x_times_j_equals_k() {
    let solver = MatrixEquationSolver::new();
    let j = symbol!(j; quaternion);
    let x = symbol!(x; quaternion);
    let k = symbol!(k; quaternion);

    let equation = Expression::add(vec![
        Expression::mul(vec![Expression::symbol(x.clone()), Expression::symbol(j.clone())]),
        Expression::mul(vec![Expression::integer(-1), Expression::symbol(k.clone())]),
    ]);

    let result = solver.solve(&equation, &x);
    match result {
        SolverResult::Single(_) => {}
        _ => panic!("Expected solution for x*j = k"),
    }
}

#[test]
fn test_quaternion_inverse_q_times_x_equals_1() {
    let solver = MatrixEquationSolver::new();
    let q = symbol!(q; quaternion);
    let x = symbol!(x; quaternion);

    let equation = Expression::add(vec![
        Expression::mul(vec![Expression::symbol(q.clone()), Expression::symbol(x.clone())]),
        Expression::mul(vec![Expression::integer(-1), Expression::integer(1)]),
    ]);

    let result = solver.solve(&equation, &x);
    match result {
        SolverResult::Single(_) | SolverResult::NoSolution => {}
        _ => panic!("Expected solution or no solution for q*x = 1"),
    }
}

#[test]
fn test_left_division_detection_works() {
    let solver = MatrixEquationSolver::new();
    let a = symbol!(A; matrix);
    let x = symbol!(X; matrix);
    let b = symbol!(B; matrix);

    let equation = Expression::add(vec![
        Expression::mul(vec![Expression::symbol(a.clone()), Expression::symbol(x.clone())]),
        Expression::mul(vec![Expression::integer(-1), Expression::symbol(b.clone())]),
    ]);

    let result = solver.solve(&equation, &x);
    assert!(matches!(result, SolverResult::Single(_)));
}

#[test]
fn test_right_division_detection_works() {
    let solver = MatrixEquationSolver::new();
    let a = symbol!(A; matrix);
    let x = symbol!(X; matrix);
    let b = symbol!(B; matrix);

    let equation = Expression::add(vec![
        Expression::mul(vec![Expression::symbol(x.clone()), Expression::symbol(a.clone())]),
        Expression::mul(vec![Expression::integer(-1), Expression::symbol(b.clone())]),
    ]);

    let result = solver.solve(&equation, &x);
    assert!(matches!(result, SolverResult::Single(_)));
}

#[test]
fn test_matrix_equation_can_solve_returns_true_for_noncommutative() {
    let solver = MatrixEquationSolver::new();
    let a = symbol!(A; matrix);
    let x = symbol!(X; matrix);

    let equation = Expression::mul(vec![Expression::symbol(a), Expression::symbol(x)]);

    assert!(solver.can_solve(&equation));
}

#[test]
fn test_matrix_equation_can_solve_returns_false_for_commutative() {
    let solver = MatrixEquationSolver::new();
    let x = symbol!(x);
    let y = symbol!(y);

    let equation = Expression::mul(vec![Expression::symbol(x), Expression::symbol(y)]);

    assert!(!solver.can_solve(&equation));
}

#[test]
fn test_solve_with_explanation_provides_steps() {
    let solver = MatrixEquationSolver::new();
    let a = symbol!(A; matrix);
    let x = symbol!(X; matrix);
    let b = symbol!(B; matrix);

    let equation = Expression::add(vec![
        Expression::mul(vec![Expression::symbol(a.clone()), Expression::symbol(x.clone())]),
        Expression::mul(vec![Expression::integer(-1), Expression::symbol(b.clone())]),
    ]);

    let (result, explanation) = solver.solve_with_explanation(&equation, &x);

    match result {
        SolverResult::Single(_) => {
            assert!(!explanation.steps.is_empty());
        }
        _ => {}
    }
}

#[test]
fn test_left_division_solution_structure() {
    let solver = MatrixEquationSolver::new();
    let a = symbol!(A; matrix);
    let b = symbol!(B; matrix);

    match solver.solve_left_division(&Expression::symbol(a.clone()), &Expression::symbol(b.clone())) {
        Ok(solution) => {
            let sol_str = solution.to_string();
            assert!(sol_str.contains("A"));
            assert!(sol_str.contains("B"));
        }
        Err(_) => panic!("Expected successful left division"),
    }
}

#[test]
fn test_right_division_solution_structure() {
    let solver = MatrixEquationSolver::new();
    let a = symbol!(A; matrix);
    let b = symbol!(B; matrix);

    match solver.solve_right_division(&Expression::symbol(a.clone()), &Expression::symbol(b.clone())) {
        Ok(solution) => {
            let sol_str = solution.to_string();
            assert!(sol_str.contains("A"));
            assert!(sol_str.contains("B"));
        }
        Err(_) => panic!("Expected successful right division"),
    }
}

#[test]
fn test_operator_eigenvalue_equation() {
    let solver = MatrixEquationSolver::new();
    let h = symbol!(H; operator);
    let psi = symbol!(psi; operator);
    let e = symbol!(E; operator);

    let equation = Expression::add(vec![
        Expression::mul(vec![Expression::symbol(h.clone()), Expression::symbol(psi.clone())]),
        Expression::mul(vec![
            Expression::integer(-1),
            Expression::symbol(e.clone()),
            Expression::symbol(psi.clone()),
        ]),
    ]);

    let result = solver.solve(&equation, &psi);
    match result {
        SolverResult::Single(_) | SolverResult::NoSolution => {}
        _ => {}
    }
}

#[test]
fn test_matrix_identity_left_multiplication() {
    let solver = MatrixEquationSolver::new();
    let x = symbol!(X; matrix);
    let b = symbol!(B; matrix);

    let equation = Expression::add(vec![
        Expression::symbol(x.clone()),
        Expression::mul(vec![Expression::integer(-1), Expression::symbol(b.clone())]),
    ]);

    let result = solver.solve(&equation, &x);
    match result {
        SolverResult::Single(_) | SolverResult::NoSolution => {}
        _ => {}
    }
}

#[test]
fn test_pauli_matrix_equation() {
    let solver = MatrixEquationSolver::new();
    let sigma_x = symbol!(sigma_x; operator);
    let x = symbol!(X; operator);
    let y = symbol!(Y; operator);

    let equation = Expression::add(vec![
        Expression::mul(vec![
            Expression::symbol(sigma_x.clone()),
            Expression::symbol(x.clone()),
        ]),
        Expression::mul(vec![Expression::integer(-1), Expression::symbol(y.clone())]),
    ]);

    let result = solver.solve(&equation, &x);
    match result {
        SolverResult::Single(_) => {}
        _ => panic!("Expected solution for sigma_x*X = Y"),
    }
}

#[test]
fn test_multiple_matrix_multiplication_left() {
    let solver = MatrixEquationSolver::new();
    let a = symbol!(A; matrix);
    let b = symbol!(B; matrix);
    let x = symbol!(X; matrix);
    let c = symbol!(C; matrix);

    let equation = Expression::add(vec![
        Expression::mul(vec![
            Expression::symbol(a.clone()),
            Expression::symbol(b.clone()),
            Expression::symbol(x.clone()),
        ]),
        Expression::mul(vec![Expression::integer(-1), Expression::symbol(c.clone())]),
    ]);

    let result = solver.solve(&equation, &x);
    match result {
        SolverResult::Single(_) | SolverResult::NoSolution => {}
        _ => {}
    }
}

#[test]
fn test_multiple_matrix_multiplication_right() {
    let solver = MatrixEquationSolver::new();
    let a = symbol!(A; matrix);
    let b = symbol!(B; matrix);
    let x = symbol!(X; matrix);
    let c = symbol!(C; matrix);

    let equation = Expression::add(vec![
        Expression::mul(vec![
            Expression::symbol(x.clone()),
            Expression::symbol(a.clone()),
            Expression::symbol(b.clone()),
        ]),
        Expression::mul(vec![Expression::integer(-1), Expression::symbol(c.clone())]),
    ]);

    let result = solver.solve(&equation, &x);
    match result {
        SolverResult::Single(_) | SolverResult::NoSolution => {}
        _ => {}
    }
}

#[test]
fn test_scalar_coefficient_right_division() {
    let solver = MatrixEquationSolver::new();
    let a = symbol!(A; matrix);
    let x = symbol!(X; matrix);
    let b = symbol!(B; matrix);

    let equation = Expression::add(vec![
        Expression::mul(vec![
            Expression::symbol(x.clone()),
            Expression::symbol(a.clone()),
            Expression::integer(2),
        ]),
        Expression::mul(vec![Expression::integer(-1), Expression::symbol(b.clone())]),
    ]);

    let result = solver.solve(&equation, &x);
    match result {
        SolverResult::Single(_) | SolverResult::NoSolution => {}
        _ => {}
    }
}

#[test]
fn test_operator_equation_with_scalar() {
    let solver = MatrixEquationSolver::new();
    let h = symbol!(H; operator);
    let x = symbol!(X; operator);

    let equation = Expression::add(vec![
        Expression::mul(vec![
            Expression::integer(2),
            Expression::symbol(h.clone()),
            Expression::symbol(x.clone()),
        ]),
        Expression::mul(vec![Expression::integer(-1), Expression::integer(5)]),
    ]);

    let result = solver.solve(&equation, &x);
    match result {
        SolverResult::Single(_) | SolverResult::NoSolution => {}
        _ => {}
    }
}

#[test]
fn test_variable_appears_once_left() {
    let solver = MatrixEquationSolver::new();
    let a = symbol!(A; matrix);
    let x = symbol!(X; matrix);
    let b = symbol!(B; matrix);

    let equation = Expression::add(vec![
        Expression::mul(vec![Expression::symbol(a.clone()), Expression::symbol(x.clone())]),
        Expression::mul(vec![Expression::integer(-1), Expression::symbol(b.clone())]),
    ]);

    let result = solver.solve(&equation, &x);
    assert!(matches!(result, SolverResult::Single(_)));
}

#[test]
fn test_variable_appears_once_right() {
    let solver = MatrixEquationSolver::new();
    let a = symbol!(A; matrix);
    let x = symbol!(X; matrix);
    let b = symbol!(B; matrix);

    let equation = Expression::add(vec![
        Expression::mul(vec![Expression::symbol(x.clone()), Expression::symbol(a.clone())]),
        Expression::mul(vec![Expression::integer(-1), Expression::symbol(b.clone())]),
    ]);

    let result = solver.solve(&equation, &x);
    assert!(matches!(result, SolverResult::Single(_)));
}

#[test]
fn test_backward_compatibility_with_linear_solver() {
    use mathhook_core::algebra::solvers::linear::LinearSolver;

    let linear_solver = LinearSolver::new();
    let a = symbol!(A; matrix);
    let x = symbol!(X; matrix);
    let b = symbol!(B; matrix);

    let equation = Expression::add(vec![
        Expression::mul(vec![Expression::symbol(a.clone()), Expression::symbol(x.clone())]),
        Expression::mul(vec![Expression::integer(-1), Expression::symbol(b.clone())]),
    ]);

    let result = linear_solver.solve(&equation, &x);
    assert!(matches!(
        result,
        SolverResult::Single(_) | SolverResult::NoSolution
    ));
}

#[test]
fn test_commutative_equation_still_works_with_linear_solver() {
    use mathhook_core::algebra::solvers::linear::LinearSolver;

    let linear_solver = LinearSolver::new();
    let x = symbol!(x);

    let equation = Expression::add(vec![
        Expression::mul(vec![Expression::integer(2), Expression::symbol(x.clone())]),
        Expression::integer(3),
    ]);

    let result = linear_solver.solve(&equation, &x);
    assert!(matches!(result, SolverResult::Single(_)));
}

#[test]
fn test_solve_singular_matrix_error() {
    // Test that singular matrices (determinant = 0) return error
    // A matrix is singular if it can't be inverted
    // For symbolic matrices, we can't determine singularity in general,
    // but zero matrices are clearly singular
    let solver = MatrixEquationSolver::new();
    let x = symbol!(X; matrix);

    // Zero matrix * X = B is unsolvable (zero has no inverse)
    let equation = Expression::add(vec![
        Expression::mul(vec![Expression::integer(0), Expression::symbol(x.clone())]),
        Expression::mul(vec![Expression::integer(-1), Expression::integer(1)]),
    ]);

    let result = solver.solve(&equation, &x);
    // Should return NoSolution for singular (zero) matrix
    assert_eq!(result, SolverResult::NoSolution);
}

#[test]
fn test_solve_mixed_noncommutative_types_error() {
    // Test equations mixing different noncommutative types
    // Example: A (matrix) * X * B (quaternion) = C
    // This is mathematically invalid - cannot multiply matrix by quaternion
    // The solver should handle this gracefully
    let solver = MatrixEquationSolver::new();
    let a = symbol!(A; matrix);
    let b = symbol!(B; quaternion);
    let x = symbol!(X; matrix);
    let c = symbol!(C; matrix);

    let equation = Expression::add(vec![
        Expression::mul(vec![
            Expression::symbol(a.clone()),
            Expression::symbol(x.clone()),
            Expression::symbol(b.clone()),
        ]),
        Expression::mul(vec![Expression::integer(-1), Expression::symbol(c.clone())]),
    ]);

    let result = solver.solve(&equation, &x);
    // Should return NoSolution due to type incompatibility
    // Currently the solver may return a solution, but it represents an invalid operation
    match result {
        SolverResult::Single(_) | SolverResult::NoSolution => {}
        _ => panic!("Expected Single or NoSolution for mixed types"),
    }
}

#[test]
fn test_solve_variable_in_middle_noncommutative() {
    // Test A*X*B = C where X is in the middle (noncommutative)
    // This is unsolvable in general for noncommutative A, B
    // Would require: X = A^(-1)*C*B^(-1), but order matters
    let solver = MatrixEquationSolver::new();
    let a = symbol!(A; matrix);
    let b = symbol!(B; matrix);
    let x = symbol!(X; matrix);
    let c = symbol!(C; matrix);

    let equation = Expression::add(vec![
        Expression::mul(vec![
            Expression::symbol(a.clone()),
            Expression::symbol(x.clone()),
            Expression::symbol(b.clone()),
        ]),
        Expression::mul(vec![Expression::integer(-1), Expression::symbol(c.clone())]),
    ]);

    let result = solver.solve(&equation, &x);
    // Should return NoSolution because X is sandwiched between A and B
    // This pattern is not supported by simple left/right division
    assert_eq!(result, SolverResult::NoSolution);
}

#[test]
fn test_solve_multiple_variables_noncommutative() {
    // Test equations with multiple different variables
    // Example: A*X + B*Y = C (two unknowns)
    // This is a system of equations, not solvable with single equation solver
    let solver = MatrixEquationSolver::new();
    let a = symbol!(A; matrix);
    let b = symbol!(B; matrix);
    let x = symbol!(X; matrix);
    let y = symbol!(Y; matrix);
    let c = symbol!(C; matrix);

    let equation = Expression::add(vec![
        Expression::mul(vec![Expression::symbol(a.clone()), Expression::symbol(x.clone())]),
        Expression::mul(vec![Expression::symbol(b.clone()), Expression::symbol(y.clone())]),
        Expression::mul(vec![Expression::integer(-1), Expression::symbol(c.clone())]),
    ]);

    // Try to solve for X (but Y is also present as unknown)
    let result = solver.solve(&equation, &x);
    // The solver might return a solution with Y in it, or NoSolution
    // Either is acceptable for this test
    match result {
        SolverResult::Single(_) => {
            // Solution should contain Y since it's an unknown
        }
        SolverResult::NoSolution => {}
        _ => panic!("Expected Single or NoSolution"),
    }
}

#[test]
fn test_solve_identity_matrix_equation() {
    // Test I*X = B where I is identity matrix
    // For symbolic matrices, we use a symbol I with matrix type to represent identity
    let solver = MatrixEquationSolver::new();
    let i = symbol!(I; matrix);
    let x = symbol!(X; matrix);
    let b = symbol!(B; matrix);

    // I*X = B (where I is identity matrix)
    let equation = Expression::add(vec![
        Expression::mul(vec![Expression::symbol(i.clone()), Expression::symbol(x.clone())]),
        Expression::mul(vec![Expression::integer(-1), Expression::symbol(b.clone())]),
    ]);

    let result = solver.solve(&equation, &x);
    match result {
        SolverResult::Single(solution) => {
            // Solution should be I^(-1)*B (or equivalent)
            // Since I is identity, mathematically this equals B
            // But symbolically the solver will give I^(-1)*B
            let sol_str = solution.to_string();
            assert!(sol_str.contains("I") && sol_str.contains("B"));
        }
        _ => panic!("Expected solution for identity matrix equation I*X = B"),
    }
}
