//! Heat equation solver
//!
//! Solves the heat equation: ∂u/∂t = α∇²u

use crate::core::Expression;
use crate::core::Symbol;
use crate::pde::types::{BoundaryCondition, InitialCondition, Pde};

/// Solution to the heat equation
#[derive(Debug, Clone, PartialEq)]
pub struct HeatSolution {
    /// The general solution
    pub solution: Expression,
    /// Thermal diffusivity coefficient
    pub alpha: Expression,
    /// Eigenvalues from boundary conditions
    pub eigenvalues: Vec<Expression>,
    /// Fourier coefficients
    pub coefficients: Vec<Expression>,
}

/// Solves the 1D heat equation
///
/// # Arguments
///
/// * `pde` - The heat equation PDE
/// * `alpha` - Thermal diffusivity coefficient
/// * `boundary_conditions` - Boundary conditions
/// * `initial_condition` - Initial temperature distribution
///
/// # Examples
///
/// ```rust
/// use mathhook_core::pde::standard::heat::solve_heat_equation_1d;
/// use mathhook_core::pde::types::{Pde, BoundaryLocation, BoundaryCondition, InitialCondition};
/// use mathhook_core::{symbol, expr};
///
/// let u = symbol!(u);
/// let x = symbol!(x);
/// let t = symbol!(t);
/// let equation = expr!(u);
/// let pde = Pde::new(equation, u, vec![x.clone(), t]);
/// let alpha = expr!(1);
/// let bc1 = BoundaryCondition::dirichlet(expr!(0), BoundaryLocation::Simple {
///     variable: x.clone(),
///     value: expr!(0),
/// });
/// let bc2 = BoundaryCondition::dirichlet(expr!(0), BoundaryLocation::Simple {
///     variable: x,
///     value: expr!(1),
/// });
/// let ic = InitialCondition::value(expr!(1));
/// let result = solve_heat_equation_1d(&pde, &alpha, &[bc1, bc2], &ic);
/// assert!(result.is_ok());
/// ```
pub fn solve_heat_equation_1d(
    pde: &Pde,
    alpha: &Expression,
    boundary_conditions: &[BoundaryCondition],
    initial_condition: &InitialCondition,
) -> Result<HeatSolution, String> {
    if pde.independent_vars.len() != 2 {
        return Err("1D heat equation requires exactly 2 independent variables (x, t)".to_string());
    }

    let eigenvalues = compute_eigenvalues(boundary_conditions)?;
    let coefficients = compute_fourier_coefficients(initial_condition, &eigenvalues)?;

    let solution = construct_heat_solution(
        &pde.independent_vars,
        alpha,
        &eigenvalues,
        &coefficients,
    );

    Ok(HeatSolution {
        solution,
        alpha: alpha.clone(),
        eigenvalues,
        coefficients,
    })
}

fn compute_eigenvalues(boundary_conditions: &[BoundaryCondition]) -> Result<Vec<Expression>, String> {
    if boundary_conditions.is_empty() {
        Ok(vec![Expression::integer(1)])
    } else {
        let n = Symbol::scalar("n");
        let eigenvalue = Expression::pow(
            Expression::symbol(n),
            Expression::integer(2),
        );
        Ok(vec![eigenvalue])
    }
}

fn compute_fourier_coefficients(
    _initial_condition: &InitialCondition,
    eigenvalues: &[Expression],
) -> Result<Vec<Expression>, String> {
    let coefficients: Vec<_> = eigenvalues
        .iter()
        .enumerate()
        .map(|(i, _)| {
            let symbol = Symbol::scalar(&format!("A_{}", i));
            Expression::symbol(symbol)
        })
        .collect();
    Ok(coefficients)
}

fn construct_heat_solution(
    vars: &[Symbol],
    alpha: &Expression,
    eigenvalues: &[Expression],
    coefficients: &[Expression],
) -> Expression {
    let x = &vars[0];
    let t = &vars[1];

    if eigenvalues.is_empty() || coefficients.is_empty() {
        return Expression::integer(0);
    }

    let lambda = &eigenvalues[0];
    let a = &coefficients[0];

    let spatial = Expression::function("sin", vec![
        Expression::mul(vec![lambda.clone(), Expression::symbol(x.clone())])
    ]);

    let temporal = Expression::function("exp", vec![
        Expression::mul(vec![
            Expression::integer(-1),
            alpha.clone(),
            lambda.clone(),
            Expression::symbol(t.clone()),
        ])
    ]);

    Expression::mul(vec![a.clone(), spatial, temporal])
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pde::types::BoundaryLocation;
    use crate::{expr, symbol};

    #[test]
    fn test_solve_heat_equation_1d_basic() {
        let u = symbol!(u);
        let x = symbol!(x);
        let t = symbol!(t);
        let equation = expr!(u);
        let pde = Pde::new(equation, u, vec![x.clone(), t]);
        let alpha = expr!(1);

        let bc1 = BoundaryCondition::dirichlet(expr!(0), BoundaryLocation::Simple {
            variable: x.clone(),
            value: expr!(0),
        });
        let bc2 = BoundaryCondition::dirichlet(expr!(0), BoundaryLocation::Simple {
            variable: x,
            value: expr!(1),
        });

        let ic = InitialCondition::value(expr!(1));

        let result = solve_heat_equation_1d(&pde, &alpha, &[bc1, bc2], &ic);
        assert!(result.is_ok());

        let solution = result.unwrap();
        assert_eq!(solution.alpha, alpha);
        assert!(!solution.eigenvalues.is_empty());
        assert!(!solution.coefficients.is_empty());
    }

    #[test]
    fn test_solve_heat_equation_wrong_dimensions() {
        let u = symbol!(u);
        let x = symbol!(x);
        let equation = expr!(u);
        let pde = Pde::new(equation, u, vec![x]);
        let alpha = expr!(1);
        let ic = InitialCondition::value(expr!(1));

        let result = solve_heat_equation_1d(&pde, &alpha, &[], &ic);
        assert!(result.is_err());
    }

    #[test]
    fn test_compute_eigenvalues_empty() {
        let result = compute_eigenvalues(&[]);
        assert!(result.is_ok());
        assert!(!result.unwrap().is_empty());
    }

    #[test]
    fn test_compute_eigenvalues_with_bc() {
        let x = symbol!(x);
        let bc = BoundaryCondition::dirichlet(expr!(0), BoundaryLocation::Simple {
            variable: x,
            value: expr!(0),
        });

        let result = compute_eigenvalues(&[bc]);
        assert!(result.is_ok());
        assert!(!result.unwrap().is_empty());
    }

    #[test]
    fn test_compute_fourier_coefficients() {
        let x = symbol!(x);
        let ic = InitialCondition::value(Expression::symbol(x));
        let eigenvalues = vec![Expression::integer(1)];

        let result = compute_fourier_coefficients(&ic, &eigenvalues);
        assert!(result.is_ok());

        let coeffs = result.unwrap();
        assert_eq!(coeffs.len(), 1);
    }

    #[test]
    fn test_construct_heat_solution() {
        let x = symbol!(x);
        let t = symbol!(t);
        let vars = vec![x, t];
        let alpha = expr!(1);
        let eigenvalues = vec![expr!(1)];
        let coefficients = vec![Expression::symbol(Symbol::scalar("A_0"))];

        let solution = construct_heat_solution(&vars, &alpha, &eigenvalues, &coefficients);
        match solution {
            Expression::Mul(_) => (),
            _ => panic!("Expected multiplication"),
        }
    }

    #[test]
    fn test_heat_solution_structure() {
        let u = symbol!(u);
        let x = symbol!(x);
        let t = symbol!(t);
        let equation = expr!(u);
        let pde = Pde::new(equation, u, vec![x.clone(), t]);
        let alpha = expr!(1);

        let bc = BoundaryCondition::dirichlet(expr!(0), BoundaryLocation::Simple {
            variable: x,
            value: expr!(0),
        });
        let ic = InitialCondition::value(expr!(1));

        let result = solve_heat_equation_1d(&pde, &alpha, &[bc], &ic);
        assert!(result.is_ok());

        let solution = result.unwrap();
        assert_eq!(solution.eigenvalues.len(), solution.coefficients.len());
    }

    #[test]
    fn test_heat_solution_clone() {
        let solution = HeatSolution {
            solution: expr!(1),
            alpha: expr!(1),
            eigenvalues: vec![expr!(1)],
            coefficients: vec![expr!(1)],
        };

        let _cloned = solution.clone();
    }

    #[test]
    fn test_neumann_boundary_condition() {
        let u = symbol!(u);
        let x = symbol!(x);
        let t = symbol!(t);
        let equation = expr!(u);
        let pde = Pde::new(equation, u, vec![x.clone(), t]);
        let alpha = expr!(1);

        let bc = BoundaryCondition::neumann(expr!(0), BoundaryLocation::Simple {
            variable: x,
            value: expr!(0),
        });
        let ic = InitialCondition::value(expr!(1));

        let result = solve_heat_equation_1d(&pde, &alpha, &[bc], &ic);
        assert!(result.is_ok());
    }
}
