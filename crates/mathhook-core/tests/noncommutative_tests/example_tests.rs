//! Example Validation Tests
//!
//! Tests verifying real-world example use cases are correct.
//! Covers: Quantum mechanics, matrix algebra, quaternion rotations.

use mathhook_core::algebra::solvers::matrix_equations::MatrixEquationSolver;
use mathhook_core::algebra::solvers::{EquationSolver, SolverResult};
use mathhook_core::formatter::latex::LaTeXContext;
use mathhook_core::formatter::LaTeXFormatter;
use mathhook_core::{symbol, Expression};

#[test]
fn test_quantum_mechanics_example_concepts() {
    let x = symbol!(x; operator);
    let p = symbol!(p; operator);

    let xp = Expression::mul(vec![
        Expression::symbol(x.clone()),
        Expression::symbol(p.clone()),
    ]);

    let px = Expression::mul(vec![
        Expression::symbol(p.clone()),
        Expression::symbol(x.clone()),
    ]);

    let xp_str = xp.to_string();
    let px_str = px.to_string();

    assert_ne!(
        xp_str, px_str,
        "Operator multiplication should preserve order"
    );

    let xp_latex = xp.to_latex(LaTeXContext::default()).unwrap();
    let px_latex = px.to_latex(LaTeXContext::default()).unwrap();

    assert!(xp_latex.contains(r"\hat{x}") && xp_latex.contains(r"\hat{p}"));
    assert!(px_latex.contains(r"\hat{p}") && px_latex.contains(r"\hat{x}"));
}

#[test]
fn test_matrix_algebra_example_concepts() {
    let a = symbol!(A; matrix);
    let x = symbol!(X; matrix);
    let b = symbol!(B; matrix);

    let ax_eq = Expression::add(vec![
        Expression::mul(vec![
            Expression::symbol(a.clone()),
            Expression::symbol(x.clone()),
        ]),
        Expression::mul(vec![Expression::integer(-1), Expression::symbol(b.clone())]),
    ]);

    let solver = MatrixEquationSolver::new();
    let result = solver.solve(&ax_eq, &x);

    assert!(
        matches!(result, SolverResult::Single(_)),
        "Should solve A*X = B"
    );

    let xa_eq = Expression::add(vec![
        Expression::mul(vec![Expression::symbol(x.clone()), Expression::symbol(a)]),
        Expression::mul(vec![Expression::integer(-1), Expression::symbol(b)]),
    ]);

    let result_right = solver.solve(&xa_eq, &x);

    assert!(
        matches!(result_right, SolverResult::Single(_)),
        "Should solve X*A = B"
    );
}

#[test]
fn test_quaternion_rotations_example_concepts() {
    let i = symbol!(i; quaternion);
    let j = symbol!(j; quaternion);
    let k = symbol!(k; quaternion);

    let ij = Expression::mul(vec![
        Expression::symbol(i.clone()),
        Expression::symbol(j.clone()),
    ]);

    let ji = Expression::mul(vec![
        Expression::symbol(j.clone()),
        Expression::symbol(i.clone()),
    ]);

    let ij_str = ij.to_string();
    let ji_str = ji.to_string();

    assert_ne!(ij_str, ji_str, "Quaternion multiplication order matters");

    let jk = Expression::mul(vec![Expression::symbol(j), Expression::symbol(k.clone())]);

    let ki = Expression::mul(vec![Expression::symbol(k), Expression::symbol(i.clone())]);

    assert!(!jk.to_string().is_empty());
    assert!(!ki.to_string().is_empty());
}
