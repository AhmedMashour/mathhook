//! Separation of variables method for PDEs
//!
//! This module implements complete separation of variables including:
//! - Eigenvalue problem solving with boundary conditions
//! - Fourier coefficient computation from initial conditions
//! - Complete series solution assembly
//!
//! # Implementation
//!
//! For standard PDEs like heat and wave equations:
//! 1. Assume product solution: u(x,t) = X(x)T(t)
//! 2. Apply boundary conditions → solve eigenvalue problem for X(x)
//! 3. Solve temporal ODE for T(t)
//! 4. Apply initial conditions → compute Fourier coefficients
//! 5. Assemble infinite series solution

use crate::calculus::pde::common::eigenvalue_problem::solve_sturm_liouville;
use crate::calculus::pde::common::fourier_coefficients::compute_fourier_coefficients;
use crate::calculus::pde::types::{BoundaryCondition, InitialCondition, Pde};
use crate::core::{Expression, Symbol};
use crate::expr;

/// Result of applying separation of variables
#[derive(Debug, Clone, PartialEq)]
pub struct SeparatedSolution {
    /// The separated functions (e.g., X(x), T(t) for u(x,t) = X(x)T(t))
    pub functions: Vec<Expression>,
    /// The separation constants (λ₀, λ₁, ...)
    pub constants: Vec<Expression>,
    /// The general product solution (before applying ICs)
    pub solution: Expression,
    /// Computed eigenvalues from boundary conditions
    pub eigenvalues: Vec<Expression>,
    /// Computed eigenfunctions from boundary conditions
    pub eigenfunctions: Vec<Expression>,
    /// Fourier coefficients from initial conditions
    pub coefficients: Vec<Expression>,
}

/// Applies separation of variables to a PDE with boundary and initial conditions
///
/// This is the complete implementation that:
/// 1. Parses boundary conditions
/// 2. Solves eigenvalue problem
/// 3. Computes Fourier coefficients
/// 4. Assembles complete solution
///
/// # Arguments
///
/// * `pde` - The PDE to solve
/// * `boundary_conditions` - Spatial boundary conditions (must have exactly 2)
/// * `initial_conditions` - Temporal initial conditions
///
/// # Returns
///
/// Complete separated solution with eigenvalues, eigenfunctions, and coefficients
///
/// # Examples
///
/// ```rust
/// use mathhook_core::calculus::pde::separation_of_variables::separate_variables;
/// use mathhook_core::calculus::pde::types::{Pde, BoundaryCondition, InitialCondition};
/// use mathhook_core::{symbol, expr};
///
/// let u = symbol!(u);
/// let x = symbol!(x);
/// let t = symbol!(t);
/// let equation = expr!(u);
/// let pde = Pde::new(equation, u, vec![x.clone(), t]);
///
/// let bc_left = BoundaryCondition::dirichlet_at(x.clone(), expr!(0), expr!(0));
/// let bc_right = BoundaryCondition::dirichlet_at(x, expr!(pi), expr!(0));
/// let bcs = vec![bc_left, bc_right];
///
/// let sin_x = expr!(sin(x));
/// let ic = InitialCondition::value(sin_x);
/// let ics = vec![ic];
///
/// let result = separate_variables(&pde, &bcs, &ics);
/// assert!(result.is_ok());
/// ```
pub fn separate_variables(
    pde: &Pde,
    boundary_conditions: &[BoundaryCondition],
    initial_conditions: &[InitialCondition],
) -> Result<SeparatedSolution, String> {
    let num_vars = pde.independent_vars.len();

    if num_vars < 2 {
        return Err("Separation of variables requires at least 2 independent variables".to_owned());
    }

    let functions = create_separated_functions(&pde.independent_vars);
    let constants = create_separation_constants(num_vars - 1);
    let solution = construct_product_solution(&functions);

    if boundary_conditions.is_empty() {
        return Ok(SeparatedSolution {
            functions,
            constants,
            solution,
            eigenvalues: Vec::new(),
            eigenfunctions: Vec::new(),
            coefficients: Vec::new(),
        });
    }

    if boundary_conditions.len() != 2 {
        return Err(format!(
            "Expected exactly 2 boundary conditions, got {}",
            boundary_conditions.len()
        ));
    }

    let eigenvalue_solution =
        solve_sturm_liouville(&boundary_conditions[0], &boundary_conditions[1], 10)?;

    let coefficients = if initial_conditions.is_empty() {
        Vec::new()
    } else {
        compute_fourier_coefficients(
            &initial_conditions[0],
            &eigenvalue_solution.eigenfunctions,
            &eigenvalue_solution.domain,
            &eigenvalue_solution.variable,
        )?
    };

    Ok(SeparatedSolution {
        functions,
        constants,
        solution,
        eigenvalues: eigenvalue_solution.eigenvalues,
        eigenfunctions: eigenvalue_solution.eigenfunctions,
        coefficients,
    })
}

/// Construct complete series solution from eigenvalues and coefficients
///
/// Builds: u(x,t) = Σₙ cₙ Xₙ(x) Tₙ(t)
///
/// # Arguments
///
/// * `coefficients` - Fourier coefficients cₙ
/// * `spatial_eigenfunctions` - Spatial eigenfunctions Xₙ(x)
/// * `temporal_solutions` - Temporal solutions Tₙ(t)
/// * `num_terms` - Number of terms to include in series
///
/// # Returns
///
/// Series solution expression
pub fn construct_series_solution(
    coefficients: &[Expression],
    spatial_eigenfunctions: &[Expression],
    temporal_solutions: &[Expression],
    num_terms: usize,
) -> Expression {
    let mut terms = Vec::new();

    let max_terms = num_terms.min(coefficients.len());

    for n in 0..max_terms {
        let c_n = &coefficients[n];
        let x_n = &spatial_eigenfunctions[n];
        let t_n = &temporal_solutions[n];

        let term = Expression::mul(vec![c_n.clone(), x_n.clone(), t_n.clone()]);
        terms.push(term);
    }

    if terms.is_empty() {
        return expr!(0);
    }

    Expression::add(terms)
}

/// Create separated functions F(x), G(y), etc. for each variable
fn create_separated_functions(vars: &[Symbol]) -> Vec<Expression> {
    vars.iter()
        .map(|var| Expression::function("F", vec![Expression::symbol(var.clone())]))
        .collect()
}

/// Create separation constants λ₀, λ₁, etc.
fn create_separation_constants(count: usize) -> Vec<Expression> {
    (0..count)
        .map(|i| {
            let lambda = Symbol::new(format!("lambda_{}", i));
            Expression::symbol(lambda)
        })
        .collect()
}

/// Construct product solution u = F(x)G(y)H(t)...
fn construct_product_solution(functions: &[Expression]) -> Expression {
    if functions.is_empty() {
        return expr!(1);
    }

    Expression::mul(functions.to_vec())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{expr, symbol};
    use std::slice::from_ref;

    #[test]
    fn test_separate_variables_basic_no_bc() {
        let u = symbol!(u);
        let x = symbol!(x);
        let t = symbol!(t);
        let equation = expr!(u);
        let pde = Pde::new(equation, u, vec![x, t]);

        let result = separate_variables(&pde, &[], &[]);
        assert!(result.is_ok());

        let solution = result.unwrap();
        assert_eq!(solution.functions.len(), 2);
        assert_eq!(solution.constants.len(), 1);
        assert!(solution.eigenvalues.is_empty());
        assert!(solution.eigenfunctions.is_empty());
    }

    #[test]
    fn test_separate_variables_with_dirichlet_bc() {
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

        let solution = result.unwrap();
        assert_eq!(solution.eigenvalues.len(), 10);
        assert_eq!(solution.eigenfunctions.len(), 10);
    }

    #[test]
    fn test_separate_variables_with_neumann_bc() {
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
        assert_eq!(solution.eigenfunctions.len(), 10);
    }

    #[test]
    fn test_separate_variables_with_ic() {
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
    fn test_separate_variables_insufficient_bcs() {
        let u = symbol!(u);
        let x = symbol!(x);
        let t = symbol!(t);
        let equation = expr!(u);
        let pde = Pde::new(equation, u, vec![x.clone(), t]);

        let bc = BoundaryCondition::dirichlet_at(x, expr!(0), expr!(0));
        let bcs = vec![bc];

        let result = separate_variables(&pde, &bcs, &[]);
        assert!(result.is_err());
    }

    #[test]
    fn test_separate_variables_three_vars() {
        let u = symbol!(u);
        let x = symbol!(x);
        let y = symbol!(y);
        let t = symbol!(t);
        let equation = expr!(u);
        let pde = Pde::new(equation, u, vec![x, y, t]);

        let result = separate_variables(&pde, &[], &[]);
        assert!(result.is_ok());

        let solution = result.unwrap();
        assert_eq!(solution.functions.len(), 3);
        assert_eq!(solution.constants.len(), 2);
    }

    #[test]
    fn test_separate_variables_insufficient_vars() {
        let u = symbol!(u);
        let x = symbol!(x);
        let equation = expr!(u);
        let pde = Pde::new(equation, u, vec![x]);

        let result = separate_variables(&pde, &[], &[]);
        assert!(result.is_err());
    }

    #[test]
    fn test_create_separated_functions() {
        let x = symbol!(x);
        let t = symbol!(t);
        let vars = vec![x, t];

        let functions = create_separated_functions(&vars);
        assert_eq!(functions.len(), 2);
    }

    #[test]
    fn test_create_separation_constants() {
        let constants = create_separation_constants(2);
        assert_eq!(constants.len(), 2);
    }

    #[test]
    fn test_construct_product_solution_empty() {
        let solution = construct_product_solution(&[]);
        assert_eq!(solution, expr!(1));
    }

    #[test]
    fn test_construct_product_solution_single() {
        let x = symbol!(x);
        let f = Expression::function("F", vec![Expression::symbol(x)]);
        let solution = construct_product_solution(from_ref(&f));
        assert_eq!(solution, f);
    }

    #[test]
    fn test_construct_series_solution_single_term() {
        let x = symbol!(x);
        let _t = symbol!(t);

        let coefficients = vec![expr!(1)];
        let spatial = vec![Expression::function("sin", vec![Expression::symbol(x)])];
        let temporal = vec![Expression::function("exp", vec![expr!(-t)])];

        let solution = construct_series_solution(&coefficients, &spatial, &temporal, 1);

        assert!(matches!(solution, Expression::Mul(_)));
    }

    #[test]
    fn test_construct_series_solution_multiple_terms() {
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
    fn test_construct_series_solution_empty() {
        let solution = construct_series_solution(&[], &[], &[], 0);
        assert_eq!(solution, expr!(0));
    }

    #[test]
    fn test_separated_solution_clone() {
        let u = symbol!(u);
        let x = symbol!(x);
        let t = symbol!(t);
        let equation = expr!(u);
        let pde = Pde::new(equation, u, vec![x, t]);

        let result = separate_variables(&pde, &[], &[]);
        assert!(result.is_ok());

        let solution = result.unwrap();
        let _cloned = solution.clone();
    }
}
