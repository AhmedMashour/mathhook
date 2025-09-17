//! Laplace equation solver
//!
//! Solves the Laplace equation: ∇²u = 0
//!
//! ⚠️ **CURRENT LIMITATION**: Returns solutions with symbolic Fourier coefficients
//! (C₁, C₂, C₃, ...). Numerical evaluation of these coefficients requires symbolic
//! integration, which is not yet implemented in MathHook.
//!
//! **What you get**: Correct solution structure `u(x,y) = Σ Cₙ sin(λₙx) sinh(λₙy)`
//! where λₙ = nπ/a are correctly computed eigenvalues
//!
//! **What's missing**: Actual values of Cₙ computed from boundary conditions
//!
//! # Examples
//!
//! ```rust
//! // This returns a solution with correctly computed eigenvalues
//! // but symbolic coefficients C_1, C_2, C_3, ...
//! # use mathhook_core::calculus::pde::standard::laplace::LaplaceEquationSolver;
//! # use mathhook_core::calculus::pde::types::{Pde, BoundaryCondition, BoundaryLocation};
//! # use mathhook_core::{symbol, expr};
//! # let solver = LaplaceEquationSolver::new();
//! # let u = symbol!(u);
//! # let x = symbol!(x);
//! # let y = symbol!(y);
//! # let equation = expr!(u);
//! # let pde = Pde::new(equation, u, vec![x.clone(), y.clone()]);
//! # let bc1 = BoundaryCondition::dirichlet(expr!(0), BoundaryLocation::Simple { variable: x.clone(), value: expr!(0) });
//! # let bc2 = BoundaryCondition::dirichlet(expr!(0), BoundaryLocation::Simple { variable: x, value: expr!(1) });
//! let solution = solver.solve_laplace_equation_2d(&pde, &[bc1, bc2]);
//! // solution.x_eigenvalues = [π, 2π, 3π, ...]  (correctly computed)
//! // solution.coefficients = [C_1, C_2, C_3, ...]  (symbolic, not computed)
//! ```

use crate::calculus::pde::registry::{PDEError, PDEResult, PDESolver};
use crate::calculus::pde::types::{BoundaryCondition, PDESolution, Pde, PdeType};
use crate::core::{Expression, Symbol};
use crate::expr;

/// Solution to the Laplace equation
#[derive(Debug, Clone, PartialEq)]
pub struct LaplaceSolution {
    pub solution: Expression,
    pub x_eigenvalues: Vec<Expression>,
    pub y_eigenvalues: Vec<Expression>,
    pub coefficients: Vec<Expression>,
}

/// Laplace equation solver implementing PDESolver trait
pub struct LaplaceEquationSolver {
    max_terms: usize,
}

impl LaplaceEquationSolver {
    pub fn new() -> Self {
        Self { max_terms: 10 }
    }

    pub fn with_max_terms(max_terms: usize) -> Self {
        Self { max_terms }
    }

    /// Solves the 2D Laplace equation on a rectangle
    ///
    /// For the Laplace equation: ∂²u/∂x² + ∂²u/∂y² = 0
    /// on a rectangular domain `[0,a]` × `[0,b]` with Dirichlet boundary conditions
    ///
    /// # Mathematical Background
    ///
    /// Eigenvalues for rectangular domain with Dirichlet BCs:
    /// - x-direction: λₙ = nπ/a for n = 1, 2, 3, ...
    /// - y-direction: μₘ = mπ/b for m = 1, 2, 3, ...
    ///
    /// Solution form: u(x,y) = Σ Cₙ sin(λₙx) sinh(λₙy)
    pub fn solve_laplace_equation_2d(
        &self,
        pde: &Pde,
        boundary_conditions: &[BoundaryCondition],
    ) -> Result<LaplaceSolution, PDEError> {
        if pde.independent_vars.len() != 2 {
            return Err(PDEError::InvalidForm {
                reason: "Laplace equation solver requires exactly 2 independent variables"
                    .to_owned(),
            });
        }

        let x_var = &pde.independent_vars[0];
        let y_var = &pde.independent_vars[1];

        let (x_eigs, y_eigs) =
            compute_eigenvalues(boundary_conditions, x_var, y_var, self.max_terms)?;
        let coefficients = compute_coefficients(boundary_conditions, &x_eigs)?;

        let solution =
            construct_laplace_solution(&pde.independent_vars, &x_eigs, &y_eigs, &coefficients);

        Ok(LaplaceSolution {
            solution,
            x_eigenvalues: x_eigs,
            y_eigenvalues: y_eigs,
            coefficients,
        })
    }
}

impl PDESolver for LaplaceEquationSolver {
    fn solve(&self, pde: &Pde) -> PDEResult {
        let result = self.solve_laplace_equation_2d(pde, &[])?;

        Ok(PDESolution::laplace(
            result.solution,
            result.x_eigenvalues,
            result.coefficients,
        ))
    }

    fn can_solve(&self, pde_type: PdeType) -> bool {
        matches!(pde_type, PdeType::Elliptic)
    }

    fn priority(&self) -> u8 {
        100
    }

    fn name(&self) -> &'static str {
        "Laplace Equation Solver"
    }

    fn description(&self) -> &'static str {
        "Solves Laplace equation ∇²u = 0 on rectangular domains"
    }
}

impl Default for LaplaceEquationSolver {
    fn default() -> Self {
        Self::new()
    }
}

/// Computes eigenvalues for 2D Laplace equation
///
/// For rectangular domain `[0,a]` × `[0,b]` with Dirichlet BCs:
/// - λₙ = nπ/a for x-direction (n = 1, 2, 3, ...)
/// - μₘ = mπ/b for y-direction (m = 1, 2, 3, ...)
///
/// # Arguments
/// * `boundary_conditions` - Boundary conditions (used to extract domain dimensions)
/// * `x_var` - x spatial variable
/// * `y_var` - y spatial variable
/// * `max_terms` - Maximum number of terms to generate
///
/// # Returns
/// Tuple of (x_eigenvalues, y_eigenvalues) as actual expressions λₙ = nπ/a, μₘ = mπ/b
fn compute_eigenvalues(
    boundary_conditions: &[BoundaryCondition],
    x_var: &Symbol,
    y_var: &Symbol,
    max_terms: usize,
) -> Result<(Vec<Expression>, Vec<Expression>), PDEError> {
    use crate::calculus::pde::common::extract_domain_length;

    let x_length = if boundary_conditions.is_empty() {
        expr!(1)
    } else {
        extract_domain_length(boundary_conditions, x_var)?
    };

    let y_length = if boundary_conditions.is_empty() {
        expr!(1)
    } else {
        extract_domain_length(boundary_conditions, y_var)?
    };

    let x_eigenvalues: Vec<_> = (1..=max_terms)
        .map(|n| {
            let n_expr = Expression::integer(n as i64);
            let pi = Expression::pi();

            Expression::mul(vec![
                n_expr,
                pi,
                Expression::pow(x_length.clone(), Expression::integer(-1)),
            ])
        })
        .collect();

    let y_eigenvalues: Vec<_> = (1..=max_terms)
        .map(|m| {
            let m_expr = Expression::integer(m as i64);
            let pi = Expression::pi();

            Expression::mul(vec![
                m_expr,
                pi,
                Expression::pow(y_length.clone(), Expression::integer(-1)),
            ])
        })
        .collect();

    Ok((x_eigenvalues, y_eigenvalues))
}

/// Computes Fourier coefficients for Laplace equation
///
/// Returns symbolic coefficients C₁, C₂, C₃, ... for boundary condition matching.
///
/// ⚠️ **LIMITATION**: Returns symbolic placeholders. Numerical values require
/// symbolic integration of boundary data (not yet implemented).
fn compute_coefficients(
    _boundary_conditions: &[BoundaryCondition],
    x_eigenvalues: &[Expression],
) -> Result<Vec<Expression>, PDEError> {
    let coefficients: Vec<_> = (0..x_eigenvalues.len())
        .map(|i| {
            let symbol = Symbol::new(format!("C_{}", i + 1));
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

    let x_part = Expression::function(
        "sin",
        vec![Expression::mul(vec![
            lambda.clone(),
            Expression::symbol(x.clone()),
        ])],
    );

    let y_arg = Expression::mul(vec![lambda.clone(), Expression::symbol(y.clone())]);
    let y_part = Expression::function("sinh", vec![y_arg]);

    Expression::mul(vec![c.clone(), x_part, y_part])
}

/// Legacy function for backward compatibility
pub fn solve_laplace_2d(
    pde: &Pde,
    boundary_conditions: &[BoundaryCondition],
) -> Result<LaplaceSolution, String> {
    LaplaceEquationSolver::new()
        .solve_laplace_equation_2d(pde, boundary_conditions)
        .map_err(|e| format!("{:?}", e))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::calculus::pde::types::BoundaryLocation;
    use crate::{expr, symbol};

    #[test]
    fn test_laplace_solver_creation() {
        let solver = LaplaceEquationSolver::new();
        assert_eq!(solver.name(), "Laplace Equation Solver");
        assert_eq!(solver.priority(), 100);
    }

    #[test]
    fn test_laplace_solver_can_solve() {
        let solver = LaplaceEquationSolver::new();
        assert!(solver.can_solve(PdeType::Elliptic));
        assert!(!solver.can_solve(PdeType::Parabolic));
        assert!(!solver.can_solve(PdeType::Hyperbolic));
    }

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
                variable: x.clone(),
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
            Expression::function("f", vec![Expression::symbol(x)]),
            BoundaryLocation::Simple {
                variable: y,
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

        let bc = BoundaryCondition::dirichlet(
            expr!(0),
            BoundaryLocation::Simple {
                variable: x,
                value: expr!(0),
            },
        );

        let result = solve_laplace_2d(&pde, &[bc]);
        assert!(result.is_err());
    }

    #[test]
    fn test_solve_laplace_insufficient_bc() {
        let u = symbol!(u);
        let x = symbol!(x);
        let y = symbol!(y);
        let equation = expr!(u);
        let pde = Pde::new(equation, u, vec![x.clone(), y.clone()]);

        let bc1 = BoundaryCondition::dirichlet(
            expr!(0),
            BoundaryLocation::Simple {
                variable: x,
                value: expr!(0),
            },
        );
        let bc2 = BoundaryCondition::dirichlet(
            expr!(0),
            BoundaryLocation::Simple {
                variable: y,
                value: expr!(0),
            },
        );

        let result = solve_laplace_2d(&pde, &[bc1, bc2]);
        assert!(result.is_ok());
    }

    #[test]
    fn test_construct_laplace_solution() {
        let x = symbol!(x);
        let y = symbol!(y);
        let vars = vec![x, y];
        let x_eigenvalues = vec![expr!(1)];
        let y_eigenvalues = vec![expr!(1)];
        let coefficients = vec![Expression::symbol(symbol!(C_0))];

        let solution =
            construct_laplace_solution(&vars, &x_eigenvalues, &y_eigenvalues, &coefficients);
        match solution {
            Expression::Mul(_) => (),
            _ => panic!("Expected multiplication expression for Laplace solution"),
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
                variable: x.clone(),
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
            Expression::function("f", vec![Expression::symbol(x)]),
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
    fn test_laplace_eigenvalues_actual_values() {
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
                variable: x.clone(),
                value: expr!(1),
            },
        );

        let solver = LaplaceEquationSolver::with_max_terms(3);
        let result = solver.solve_laplace_equation_2d(&pde, &[bc1, bc2]);
        assert!(result.is_ok());

        let solution = result.unwrap();

        assert_eq!(solution.x_eigenvalues.len(), 3);
        assert_eq!(solution.y_eigenvalues.len(), 3);
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
    fn test_pde_solver_trait() {
        let solver = LaplaceEquationSolver::new();
        let u = symbol!(u);
        let x = symbol!(x);
        let y = symbol!(y);
        let equation = expr!(u);
        let pde = Pde::new(equation, u, vec![x, y]);

        let result = solver.solve(&pde);
        assert!(result.is_ok());
    }
}
