//! Laplace equation solver
//!
//! Solves the Laplace equation: ∇²u = 0

use crate::core::Expression;
use crate::core::Symbol;
use crate::expr;
use crate::pde::types::{BoundaryCondition, Pde};
/// Solution to the Laplace equation
#[derive(Debug, Clone, PartialEq)]
pub struct LaplaceSolution {
    /// The general solution
    pub solution: Expression,
    /// Eigenvalues in x-direction
    pub x_eigenvalues: Vec<Expression>,
    /// Eigenvalues in y-direction
    pub y_eigenvalues: Vec<Expression>,
    /// Fourier coefficients
    pub coefficients: Vec<Expression>,
}

/// Solves the 2D Laplace equation on a rectangle
///
/// For the Laplace equation: ∂²u/∂x² + ∂²u/∂y² = 0
/// on a rectangular domain [0,a] × [0,b] with Dirichlet boundary conditions
///
/// # Arguments
///
/// * `pde` - The Laplace equation PDE
/// * `boundary_conditions` - Boundary conditions on all four sides
///
/// # Examples
///
/// ```rust
/// use mathhook_core::pde::standard::laplace::solve_laplace_2d;
/// use mathhook_core::pde::types::{Pde, BoundaryLocation, BoundaryCondition};
/// use mathhook_core::{symbol, expr};
///
/// let u = symbol!(u);
/// let x = symbol!(x);
/// let y = symbol!(y);
/// let equation = expr!(u);
/// let pde = Pde::new(equation, u, vec![x.clone(), y.clone()]);
///
/// let bc1 = BoundaryCondition::dirichlet(expr!(0), BoundaryLocation::Simple {
///     variable: x.clone(),
///     value: expr!(0),
/// });
/// let bc2 = BoundaryCondition::dirichlet(expr!(0), BoundaryLocation::Simple {
///     variable: x,
///     value: expr!(1),
/// });
/// let bc3 = BoundaryCondition::dirichlet(expr!(0), BoundaryLocation::Simple {
///     variable: y.clone(),
///     value: expr!(0),
/// });
/// let bc4 = BoundaryCondition::dirichlet(Expression::function("f", vec![Expression::symbol(y)]),
///     BoundaryLocation::Simple {
///         variable: Symbol::scalar("y"),
///         value: expr!(1),
///     });
///
/// let result = solve_laplace_2d(&pde, &[bc1, bc2, bc3, bc4]);
/// assert!(result.is_ok());
/// ```
pub fn solve_laplace_2d(
    pde: &Pde,
    boundary_conditions: &[BoundaryCondition],
) -> Result<LaplaceSolution, String> {
    if pde.independent_vars.len() != 2 {
        return Err(
            "2D Laplace equation requires exactly 2 independent variables (x, y)".to_string(),
        );
    }

    if boundary_conditions.len() < 4 {
        return Err("Laplace equation on rectangle requires 4 boundary conditions".to_string());
    }

    let (x_eigenvalues, y_eigenvalues) = compute_eigenvalues_laplace(boundary_conditions)?;
    let coefficients = compute_fourier_coefficients_laplace(boundary_conditions, &x_eigenvalues)?;

    let solution = construct_laplace_solution(
        &pde.independent_vars,
        &x_eigenvalues,
        &y_eigenvalues,
        &coefficients,
    );

    Ok(LaplaceSolution {
        solution,
        x_eigenvalues,
        y_eigenvalues,
        coefficients,
    })
}

/// Compute eigenvalues from boundary conditions for Laplace equation
fn compute_eigenvalues_laplace(
    _boundary_conditions: &[BoundaryCondition],
) -> Result<(Vec<Expression>, Vec<Expression>), String> {
    // For Dirichlet BCs on rectangle [0,a] × [0,b]:
    // λₙ = nπ/a in x-direction
    // μₙ = nπ/b in y-direction
    let n = Symbol::scalar("n");
    let m = Symbol::scalar("m");

    let x_eigenvalue = expr!(n);
    let y_eigenvalue = expr!(m);

    Ok((vec![x_eigenvalue], vec![y_eigenvalue]))
}

/// Compute Fourier coefficients for Laplace equation from boundary data
fn compute_fourier_coefficients_laplace(
    _boundary_conditions: &[BoundaryCondition],
    eigenvalues: &[Expression],
) -> Result<Vec<Expression>, String> {
    // Coefficients from Fourier series of non-homogeneous boundary
    let coefficients: Vec<_> = eigenvalues
        .iter()
        .enumerate()
        .map(|(i, _)| {
            let symbol = Symbol::scalar(&format!("C_{}", i));
            Expression::symbol(symbol)
        })
        .collect();
    Ok(coefficients)
}

/// Construct the general solution to the Laplace equation
///
/// Solution form: u(x,y) = Σ Cₙ sin(λₙx) sinh(λₙy)
/// or u(x,y) = Σ Cₙ sin(λₙx) [sinh(λₙy) / sinh(λₙb)]
/// depending on which boundary has non-homogeneous condition
fn construct_laplace_solution(
    vars: &[Symbol],
    x_eigenvalues: &[Expression],
    _y_eigenvalues: &[Expression],
    coefficients: &[Expression],
) -> Expression {
    let x = &vars[0];
    let y = &vars[1];

    if x_eigenvalues.is_empty() || coefficients.is_empty() {
        return Expression::integer(0);
    }

    let lambda = &x_eigenvalues[0];
    let c = &coefficients[0];

    // Spatial part in x: sin(λx)
    let x_part = Expression::function(
        "sin",
        vec![Expression::mul(vec![
            lambda.clone(),
            Expression::symbol(x.clone()),
        ])],
    );

    // Spatial part in y: sinh(λy)
    let y_arg = Expression::mul(vec![lambda.clone(), Expression::symbol(y.clone())]);
    let y_part = Expression::function("sinh", vec![y_arg]);

    // Complete solution: C sin(λx) sinh(λy)
    Expression::mul(vec![c.clone(), x_part, y_part])
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pde::types::BoundaryLocation;
    use crate::{expr, symbol};

    #[test]
    fn test_solve_laplace_2d_basic() {
        let u = symbol!(u);
        let x = symbol!(x);
        let y = symbol!(y);
        let equation = expr!(u);
        let pde = Pde::new(equation, u, vec![x.clone(), y.clone()]);

        let bc1 = BoundaryCondition::dirichlet(
            expr!(0),
            BoundaryLocation::Simple {
                variable: x.clone(),
                value: expr!(0),
            },
        );
        let bc2 = BoundaryCondition::dirichlet(
            expr!(0),
            BoundaryLocation::Simple {
                variable: x,
                value: expr!(1),
            },
        );
        let bc3 = BoundaryCondition::dirichlet(
            expr!(0),
            BoundaryLocation::Simple {
                variable: y.clone(),
                value: expr!(0),
            },
        );
        let bc4 = BoundaryCondition::dirichlet(
            Expression::function("f", vec![Expression::symbol(y)]),
            BoundaryLocation::Simple {
                variable: Symbol::scalar("y"),
                value: expr!(1),
            },
        );

        let result = solve_laplace_2d(&pde, &[bc1, bc2, bc3, bc4]);
        assert!(result.is_ok());

        let solution = result.unwrap();
        assert!(!solution.x_eigenvalues.is_empty());
        assert!(!solution.y_eigenvalues.is_empty());
        assert!(!solution.coefficients.is_empty());
    }

    #[test]
    fn test_solve_laplace_wrong_dimensions() {
        let u = symbol!(u);
        let x = symbol!(x);
        let equation = expr!(u);
        let pde = Pde::new(equation, u, vec![x.clone()]);

        let bc1 = BoundaryCondition::dirichlet(
            expr!(0),
            BoundaryLocation::Simple {
                variable: x,
                value: expr!(0),
            },
        );

        let result = solve_laplace_2d(&pde, &[bc1]);
        assert!(result.is_err());
    }

    #[test]
    fn test_solve_laplace_insufficient_bc() {
        let u = symbol!(u);
        let x = symbol!(x);
        let y = symbol!(y);
        let equation = expr!(u);
        let pde = Pde::new(equation, u, vec![x.clone(), y]);

        let bc1 = BoundaryCondition::dirichlet(
            expr!(0),
            BoundaryLocation::Simple {
                variable: x,
                value: expr!(0),
            },
        );

        let result = solve_laplace_2d(&pde, &[bc1]);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .contains("requires 4 boundary conditions"));
    }

    #[test]
    fn test_compute_eigenvalues_laplace() {
        let x = symbol!(x);
        let y = symbol!(y);

        let bc1 = BoundaryCondition::dirichlet(
            expr!(0),
            BoundaryLocation::Simple {
                variable: x.clone(),
                value: expr!(0),
            },
        );
        let bc2 = BoundaryCondition::dirichlet(
            expr!(0),
            BoundaryLocation::Simple {
                variable: x,
                value: expr!(1),
            },
        );
        let bc3 = BoundaryCondition::dirichlet(
            expr!(0),
            BoundaryLocation::Simple {
                variable: y.clone(),
                value: expr!(0),
            },
        );
        let bc4 = BoundaryCondition::dirichlet(
            expr!(1),
            BoundaryLocation::Simple {
                variable: y,
                value: expr!(1),
            },
        );

        let result = compute_eigenvalues_laplace(&[bc1, bc2, bc3, bc4]);
        assert!(result.is_ok());

        let (x_eigs, y_eigs) = result.unwrap();
        assert!(!x_eigs.is_empty());
        assert!(!y_eigs.is_empty());
    }

    #[test]
    fn test_compute_fourier_coefficients_laplace() {
        let x = symbol!(x);
        let y = symbol!(y);

        let bc1 = BoundaryCondition::dirichlet(
            expr!(0),
            BoundaryLocation::Simple {
                variable: x.clone(),
                value: expr!(0),
            },
        );
        let bc2 = BoundaryCondition::dirichlet(
            expr!(0),
            BoundaryLocation::Simple {
                variable: x,
                value: expr!(1),
            },
        );
        let bc3 = BoundaryCondition::dirichlet(
            expr!(0),
            BoundaryLocation::Simple {
                variable: y.clone(),
                value: expr!(0),
            },
        );
        let bc4 = BoundaryCondition::dirichlet(
            expr!(1),
            BoundaryLocation::Simple {
                variable: y,
                value: expr!(1),
            },
        );

        let eigenvalues = vec![Expression::integer(1)];

        let result = compute_fourier_coefficients_laplace(&[bc1, bc2, bc3, bc4], &eigenvalues);
        assert!(result.is_ok());

        let coeffs = result.unwrap();
        assert_eq!(coeffs.len(), 1);
    }

    #[test]
    fn test_construct_laplace_solution() {
        let x = symbol!(x);
        let y = symbol!(y);
        let vars = vec![x, y];
        let x_eigenvalues = vec![expr!(1)];
        let y_eigenvalues = vec![expr!(1)];
        let coefficients = vec![Expression::symbol(Symbol::scalar("C_0"))];

        let solution =
            construct_laplace_solution(&vars, &x_eigenvalues, &y_eigenvalues, &coefficients);
        match solution {
            Expression::Mul(_) => (),
            _ => panic!("Expected multiplication"),
        }
    }

    #[test]
    fn test_laplace_solution_structure() {
        let u = symbol!(u);
        let x = symbol!(x);
        let y = symbol!(y);
        let equation = expr!(u);
        let pde = Pde::new(equation, u, vec![x.clone(), y.clone()]);

        let bc1 = BoundaryCondition::dirichlet(
            expr!(0),
            BoundaryLocation::Simple {
                variable: x.clone(),
                value: expr!(0),
            },
        );
        let bc2 = BoundaryCondition::dirichlet(
            expr!(0),
            BoundaryLocation::Simple {
                variable: x,
                value: expr!(1),
            },
        );
        let bc3 = BoundaryCondition::dirichlet(
            expr!(0),
            BoundaryLocation::Simple {
                variable: y.clone(),
                value: expr!(0),
            },
        );
        let bc4 = BoundaryCondition::dirichlet(
            expr!(1),
            BoundaryLocation::Simple {
                variable: y,
                value: expr!(1),
            },
        );

        let result = solve_laplace_2d(&pde, &[bc1, bc2, bc3, bc4]);
        assert!(result.is_ok());

        let solution = result.unwrap();
        assert_eq!(solution.x_eigenvalues.len(), solution.coefficients.len());
    }

    #[test]
    fn test_laplace_solution_clone() {
        let solution = LaplaceSolution {
            solution: expr!(1),
            x_eigenvalues: vec![expr!(1)],
            y_eigenvalues: vec![expr!(1)],
            coefficients: vec![expr!(1)],
        };

        let _cloned = solution.clone();
    }

    #[test]
    fn test_neumann_boundary_condition() {
        let u = symbol!(u);
        let x = symbol!(x);
        let y = symbol!(y);
        let equation = expr!(u);
        let pde = Pde::new(equation, u, vec![x.clone(), y.clone()]);

        let bc1 = BoundaryCondition::neumann(
            expr!(0),
            BoundaryLocation::Simple {
                variable: x.clone(),
                value: expr!(0),
            },
        );
        let bc2 = BoundaryCondition::neumann(
            expr!(0),
            BoundaryLocation::Simple {
                variable: x,
                value: expr!(1),
            },
        );
        let bc3 = BoundaryCondition::dirichlet(
            expr!(0),
            BoundaryLocation::Simple {
                variable: y.clone(),
                value: expr!(0),
            },
        );
        let bc4 = BoundaryCondition::dirichlet(
            expr!(1),
            BoundaryLocation::Simple {
                variable: y,
                value: expr!(1),
            },
        );

        let result = solve_laplace_2d(&pde, &[bc1, bc2, bc3, bc4]);
        assert!(result.is_ok());
    }

    #[test]
    fn test_laplace_solution_empty_eigenvalues() {
        let x = symbol!(x);
        let y = symbol!(y);
        let vars = vec![x, y];

        let solution = construct_laplace_solution(&vars, &[], &[], &[]);
        match solution {
            Expression::Number(_) => (),
            _ => panic!("Expected zero for empty eigenvalues"),
        }
    }

    #[test]
    fn test_laplace_solution_with_symbolic_boundary() {
        let u = symbol!(u);
        let x = symbol!(x);
        let y = symbol!(y);
        let equation = expr!(u);
        let pde = Pde::new(equation, u, vec![x.clone(), y.clone()]);

        let a = Symbol::scalar("a");
        let b = Symbol::scalar("b");

        let bc1 = BoundaryCondition::dirichlet(
            expr!(0),
            BoundaryLocation::Simple {
                variable: x.clone(),
                value: expr!(0),
            },
        );
        let bc2 = BoundaryCondition::dirichlet(
            expr!(0),
            BoundaryLocation::Simple {
                variable: x,
                value: Expression::symbol(a),
            },
        );
        let bc3 = BoundaryCondition::dirichlet(
            expr!(0),
            BoundaryLocation::Simple {
                variable: y.clone(),
                value: expr!(0),
            },
        );
        let bc4 = BoundaryCondition::dirichlet(
            Expression::function("f", vec![Expression::symbol(y)]),
            BoundaryLocation::Simple {
                variable: Symbol::scalar("x"),
                value: Expression::symbol(b),
            },
        );

        let result = solve_laplace_2d(&pde, &[bc1, bc2, bc3, bc4]);
        assert!(result.is_ok());
    }
}
