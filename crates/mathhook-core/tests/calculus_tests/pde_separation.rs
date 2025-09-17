//! Integration tests for PDE separation of variables
//!
//! Tests end-to-end workflows:
//! - Heat equation with Dirichlet BCs
//! - Wave equation with BCs
//! - Eigenvalue problem solving
//! - Fourier coefficient computation
//! - Complete series solution assembly

use mathhook_core::calculus::pde::common::eigenvalue_problem::solve_sturm_liouville;
use mathhook_core::calculus::pde::common::fourier_coefficients::compute_fourier_coefficients;
use mathhook_core::calculus::pde::separation_of_variables::{
    construct_series_solution, separate_variables,
};
use mathhook_core::calculus::pde::types::{BoundaryCondition, InitialCondition, Pde};
use mathhook_core::{expr, symbol, Expression};

#[test]
fn test_full_heat_equation_with_dirichlet_bcs() {
    let u = symbol!(u);
    let x = symbol!(x);
    let t = symbol!(t);
    let equation = expr!(u);
    let pde = Pde::new(equation, u, vec![x.clone(), t]);

    let bc_left = BoundaryCondition::dirichlet_at(x.clone(), expr!(0), expr!(0));
    let bc_right = BoundaryCondition::dirichlet_at(x, expr!(pi), expr!(0));
    let bcs = vec![bc_left, bc_right];

    let result = separate_variables(&pde, &bcs, &[]);

    assert!(
        result.is_ok(),
        "Failed to solve heat equation: {:?}",
        result.err()
    );
    let solution = result.unwrap();
    assert_eq!(solution.eigenvalues.len(), 10);
    assert_eq!(solution.eigenfunctions.len(), 10);
}

#[test]
fn test_full_heat_equation_with_ic() {
    let u = symbol!(u);
    let x = symbol!(x);
    let t = symbol!(t);
    let equation = expr!(u);
    let pde = Pde::new(equation, u, vec![x.clone(), t]);

    let bc_left = BoundaryCondition::dirichlet_at(x.clone(), expr!(0), expr!(0));
    let bc_right = BoundaryCondition::dirichlet_at(x.clone(), expr!(pi), expr!(0));
    let bcs = vec![bc_left, bc_right];

    let sin_x = Expression::function("sin", vec![Expression::symbol(x)]);
    let ic = InitialCondition::value(sin_x);
    let ics = vec![ic];

    let result = separate_variables(&pde, &bcs, &ics);

    assert!(result.is_ok());
    let solution = result.unwrap();
    assert_eq!(solution.coefficients.len(), 10);
}

#[test]
fn test_wave_equation_with_bcs() {
    let u = symbol!(u);
    let x = symbol!(x);
    let t = symbol!(t);
    let equation = expr!(u);
    let pde = Pde::new(equation, u, vec![x.clone(), t]);

    let bc_left = BoundaryCondition::dirichlet_at(x.clone(), expr!(0), expr!(0));
    let bc_right = BoundaryCondition::dirichlet_at(x, expr!(pi), expr!(0));
    let bcs = vec![bc_left, bc_right];

    let result = separate_variables(&pde, &bcs, &[]);

    assert!(result.is_ok());
}

#[test]
fn test_neumann_boundary_conditions() {
    let u = symbol!(u);
    let x = symbol!(x);
    let t = symbol!(t);
    let equation = expr!(u);
    let pde = Pde::new(equation, u, vec![x.clone(), t]);

    let bc_left = BoundaryCondition::neumann_at(x.clone(), expr!(0), expr!(0));
    let bc_right = BoundaryCondition::neumann_at(x, expr!(pi), expr!(0));
    let bcs = vec![bc_left, bc_right];

    let result = separate_variables(&pde, &bcs, &[]);

    assert!(result.is_ok());
    let solution = result.unwrap();
    assert_eq!(solution.eigenvalues.len(), 10);
}

#[test]
fn test_mixed_boundary_conditions() {
    let x = symbol!(x);
    let bc_left = BoundaryCondition::dirichlet_at(x.clone(), expr!(0), expr!(0));
    let bc_right = BoundaryCondition::neumann_at(x, expr!(pi), expr!(0));

    let result = solve_sturm_liouville(&bc_left, &bc_right, 5);

    assert!(result.is_ok());
    let solution = result.unwrap();
    assert_eq!(solution.eigenvalues.len(), 5);
    assert_eq!(solution.eigenfunctions.len(), 5);
}

#[test]
fn test_series_solution_construction() {
    let x = symbol!(x);
    let t = symbol!(t);

    let coefficients = vec![expr!(1), expr!(2)];
    let spatial = vec![
        Expression::function("sin", vec![Expression::symbol(x.clone())]),
        Expression::function(
            "sin",
            vec![Expression::mul(vec![
                Expression::integer(2),
                Expression::symbol(x),
            ])],
        ),
    ];
    let temporal = vec![
        Expression::function(
            "exp",
            vec![Expression::mul(vec![
                Expression::integer(-1),
                Expression::symbol(t.clone()),
            ])],
        ),
        Expression::function(
            "exp",
            vec![Expression::mul(vec![
                Expression::integer(-4),
                Expression::symbol(t),
            ])],
        ),
    ];

    let solution = construct_series_solution(&coefficients, &spatial, &temporal, 2);

    assert!(matches!(solution, Expression::Add(_)));
}

#[test]
fn test_fourier_coefficient_computation() {
    let x = symbol!(x);
    let sin_x = Expression::function("sin", vec![Expression::symbol(x.clone())]);
    let ic = InitialCondition::value(sin_x.clone());
    let eigenfunctions = vec![sin_x];
    let domain = (expr!(0), expr!(pi));

    let result = compute_fourier_coefficients(&ic, &eigenfunctions, &domain, &x);

    assert!(result.is_ok());
}

#[test]
fn test_eigenvalue_problem_dirichlet() {
    let x = symbol!(x);
    let bc_left = BoundaryCondition::dirichlet_at(x.clone(), expr!(0), expr!(0));
    let bc_right = BoundaryCondition::dirichlet_at(x, expr!(pi), expr!(0));

    let result = solve_sturm_liouville(&bc_left, &bc_right, 10);

    assert!(result.is_ok());
    let solution = result.unwrap();
    assert_eq!(solution.eigenvalues.len(), 10);
    assert_eq!(solution.eigenfunctions.len(), 10);
}

#[test]
fn test_eigenvalue_problem_neumann() {
    let x = symbol!(x);
    let bc_left = BoundaryCondition::neumann_at(x.clone(), expr!(0), expr!(0));
    let bc_right = BoundaryCondition::neumann_at(x, expr!(pi), expr!(0));

    let result = solve_sturm_liouville(&bc_left, &bc_right, 10);

    assert!(result.is_ok());
    let solution = result.unwrap();
    assert_eq!(solution.eigenvalues.len(), 10);
}
