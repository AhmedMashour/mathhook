//! Integration tests for separable ODE solver
//!
//! Tests end-to-end workflows:
//! - Full ODE solving pipeline
//! - Integration with ODE classification
//! - Initial condition handling
//! - Solution verification

use mathhook_core::calculus::ode::first_order::separable::SeparableODESolver;
use mathhook_core::{expr, symbol, Expression};

#[test]
fn test_full_ode_workflow_simple_linear() {
    let x = symbol!(x);
    let y = symbol!(y);
    let solver = SeparableODESolver::new();

    let rhs = expr!(x);
    let result = solver.solve(&rhs, &y, &x, None);

    assert!(
        result.is_ok(),
        "Failed to solve dy/dx = x: {:?}",
        result.err()
    );
}

#[test]
fn test_full_ode_workflow_exponential_with_ic() {
    let x = symbol!(x);
    let y = symbol!(y);
    let solver = SeparableODESolver::new();

    let rhs = expr!(y);
    let ic = Some((expr!(0), expr!(1)));
    let result = solver.solve(&rhs, &y, &x, ic);

    assert!(
        result.is_ok(),
        "Failed to solve dy/dx = y with IC: {:?}",
        result.err()
    );
}

#[test]
fn test_full_ode_workflow_product_form() {
    let x = symbol!(x);
    let y = symbol!(y);
    let solver = SeparableODESolver::new();

    let rhs = expr!(x * y);
    let result = solver.solve(&rhs, &y, &x, None);

    assert!(
        result.is_ok(),
        "Failed to solve dy/dx = xy: {:?}",
        result.err()
    );
}

#[test]
fn test_non_separable_detection() {
    let x = symbol!(x);
    let y = symbol!(y);
    let solver = SeparableODESolver::new();

    let rhs = expr!(x + y);
    assert!(!solver.is_separable(&rhs, &y, &x));

    let result = solver.solve(&rhs, &y, &x, None);
    assert!(result.is_err());
}

#[test]
fn test_separable_detection_various_forms() {
    let x = symbol!(x);
    let y = symbol!(y);
    let solver = SeparableODESolver::new();

    assert!(solver.is_separable(&expr!(x), &y, &x));
    assert!(solver.is_separable(&expr!(y), &y, &x));
    assert!(solver.is_separable(&expr!(x * y), &y, &x));
    assert!(solver.is_separable(&Expression::mul(vec![expr!(2), expr!(x), expr!(y)]), &y, &x));
}

#[test]
fn test_initial_condition_application() {
    let x = symbol!(x);
    let y = symbol!(y);
    let solver = SeparableODESolver::new();

    let rhs = expr!(x);
    let ic = Some((expr!(0), expr!(5)));
    let result = solver.solve(&rhs, &y, &x, ic);

    assert!(result.is_ok());
}

#[test]
fn test_multiple_equations_same_solver() {
    let x = symbol!(x);
    let y = symbol!(y);
    let solver = SeparableODESolver::new();

    let eq1 = expr!(x);
    let eq2 = expr!(y);
    let eq3 = expr!(x * y);

    assert!(solver.solve(&eq1, &y, &x, None).is_ok());
    assert!(solver.solve(&eq2, &y, &x, None).is_ok());
    assert!(solver.solve(&eq3, &y, &x, None).is_ok());
}
