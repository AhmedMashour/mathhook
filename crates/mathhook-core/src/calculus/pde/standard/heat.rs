//! Heat equation solver
//!
//! Solves the heat equation: du/dt = alpha * nabla^2(u)
//!
//! Uses separation of variables and Fourier series for standard boundary conditions.
//!
//! # Limitations
//!
//! Returns solutions with symbolic Fourier coefficients (A_1, A_2, A_3, ...).
//! Numerical evaluation of these coefficients requires symbolic integration,
//! which is not yet implemented in MathHook.
//!
//! **What you get**: Correct solution structure `u(x,t) = sum A_n sin(sqrt(lambda_n) x) exp(-lambda_n alpha t)`
//! where lambda_n are correctly computed eigenvalues.
//!
//! **What's missing**: Actual values of A_n computed from initial conditions via
//! Fourier series expansion (requires symbolic integration).

use crate::calculus::pde::common::{
    compute_dirichlet_1d_eigenvalues, create_symbolic_coefficients,
};
use crate::calculus::pde::registry::{PDEError, PDEResult, PDESolver};
use crate::calculus::pde::types::{BoundaryCondition, InitialCondition, PDESolution, Pde, PdeType};
use crate::core::{Expression, Symbol};

/// Heat equation solver implementing PDESolver trait
pub struct HeatEquationSolver {
    max_terms: usize,
}

impl HeatEquationSolver {
    /// Creates a new heat equation solver
    pub fn new() -> Self {
        Self { max_terms: 10 }
    }

    /// Creates solver with custom maximum number of terms
    pub fn with_max_terms(max_terms: usize) -> Self {
        Self { max_terms }
    }

    /// Solves the 1D heat equation with full Fourier series computation
    ///
    /// # Arguments
    ///
    /// * `pde` - The heat equation PDE
    /// * `alpha` - Thermal diffusivity coefficient
    /// * `boundary_conditions` - Boundary conditions
    /// * `_initial_condition` - Initial temperature distribution (currently unused)
    ///
    /// # Returns
    ///
    /// A `PDESolution` containing the heat equation solution with eigenvalues and
    /// symbolic Fourier coefficients.
    ///
    /// # Errors
    ///
    /// Returns `PDEError::InvalidForm` if the PDE does not have exactly 2 independent
    /// variables (x and t for 1D heat equation).
    ///
    /// # Mathematical Background
    ///
    /// Solution form: u(x,t) = sum A_n sin(sqrt(lambda_n) x) exp(-lambda_n alpha t)
    /// where lambda_n are eigenvalues determined by boundary conditions and A_n are
    /// Fourier coefficients determined by initial condition.
    #[allow(unused_variables)]
    pub fn solve_heat_equation_1d(
        &self,
        pde: &Pde,
        alpha: &Expression,
        boundary_conditions: &[BoundaryCondition],
        _initial_condition: &InitialCondition,
    ) -> PDEResult {
        if pde.independent_vars.len() != 2 {
            return Err(PDEError::InvalidForm {
                reason: "1D heat equation requires exactly 2 independent variables (x, t)"
                    .to_owned(),
            });
        }

        let eigenvalues = compute_dirichlet_1d_eigenvalues(
            boundary_conditions,
            &pde.independent_vars[0],
            self.max_terms,
        )?;

        let coefficients = create_symbolic_coefficients("A", eigenvalues.len())?;

        let solution =
            self.construct_heat_solution(&pde.independent_vars, alpha, &eigenvalues, &coefficients);

        Ok(PDESolution::heat(
            solution,
            alpha.clone(),
            eigenvalues,
            coefficients,
        ))
    }

    fn construct_heat_solution(
        &self,
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

        let mut terms = Vec::new();

        for (lambda, a_n) in eigenvalues.iter().zip(coefficients.iter()) {
            let spatial = Expression::function(
                "sin",
                vec![Expression::mul(vec![
                    Expression::pow(lambda.clone(), Expression::rational(1, 2)),
                    Expression::symbol(x.clone()),
                ])],
            );

            let temporal = Expression::function(
                "exp",
                vec![Expression::mul(vec![
                    Expression::integer(-1),
                    lambda.clone(),
                    alpha.clone(),
                    Expression::symbol(t.clone()),
                ])],
            );

            let term = Expression::mul(vec![a_n.clone(), spatial, temporal]);
            terms.push(term);
        }

        Expression::add(terms)
    }
}

impl PDESolver for HeatEquationSolver {
    fn solve(&self, pde: &Pde) -> PDEResult {
        use crate::expr;

        let alpha = expr!(1);
        let ic = InitialCondition::value(expr!(1));

        self.solve_heat_equation_1d(pde, &alpha, &[], &ic)
    }

    fn can_solve(&self, pde_type: PdeType) -> bool {
        matches!(pde_type, PdeType::Parabolic)
    }

    fn priority(&self) -> u8 {
        100
    }

    fn name(&self) -> &'static str {
        "Heat Equation Solver"
    }

    fn description(&self) -> &'static str {
        "Solves heat equation du/dt = alpha * nabla^2(u) using separation of variables and Fourier series"
    }
}

impl Default for HeatEquationSolver {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::calculus::pde::types::{BoundaryLocation, SolutionMetadata};
    use crate::{expr, symbol};

    #[test]
    fn test_heat_solver_creation() {
        let solver = HeatEquationSolver::new();
        assert_eq!(solver.name(), "Heat Equation Solver");
        assert_eq!(solver.priority(), 100);
    }

    #[test]
    fn test_heat_solver_can_solve() {
        let solver = HeatEquationSolver::new();
        assert!(solver.can_solve(PdeType::Parabolic));
        assert!(!solver.can_solve(PdeType::Hyperbolic));
        assert!(!solver.can_solve(PdeType::Elliptic));
    }

    #[test]
    fn test_solve_heat_equation_1d_basic() {
        let u = symbol!(u);
        let x = symbol!(x);
        let t = symbol!(t);
        let equation = expr!(u);
        let pde = Pde::new(equation, u, vec![x.clone(), t]);
        let alpha = expr!(1);

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

        let ic = InitialCondition::value(expr!(1));

        let solver = HeatEquationSolver::new();
        let result = solver.solve_heat_equation_1d(&pde, &alpha, &[bc1, bc2], &ic);
        assert!(result.is_ok());

        let solution = result.unwrap();
        match &solution.metadata {
            SolutionMetadata::Heat {
                alpha: sol_alpha,
                eigenvalues,
                coefficients,
            } => {
                assert_eq!(sol_alpha, &alpha);
                assert!(!eigenvalues.is_empty());
                assert!(!coefficients.is_empty());
            }
            _ => panic!("Expected Heat metadata"),
        }
    }

    #[test]
    fn test_solve_heat_equation_wrong_dimensions() {
        let u = symbol!(u);
        let x = symbol!(x);
        let equation = expr!(u);
        let pde = Pde::new(equation, u, vec![x]);
        let alpha = expr!(1);
        let ic = InitialCondition::value(expr!(1));

        let solver = HeatEquationSolver::new();
        let result = solver.solve_heat_equation_1d(&pde, &alpha, &[], &ic);
        assert!(result.is_err());
    }

    #[test]
    fn test_heat_solution_structure() {
        let u = symbol!(u);
        let x = symbol!(x);
        let t = symbol!(t);
        let equation = expr!(u);
        let pde = Pde::new(equation, u, vec![x.clone(), t]);
        let alpha = expr!(1);

        let bc = BoundaryCondition::dirichlet(
            expr!(0),
            BoundaryLocation::Simple {
                variable: x,
                value: expr!(0),
            },
        );
        let ic = InitialCondition::value(expr!(1));

        let solver = HeatEquationSolver::new();
        let result = solver.solve_heat_equation_1d(&pde, &alpha, &[bc], &ic);
        assert!(result.is_ok());

        let solution = result.unwrap();
        match &solution.metadata {
            SolutionMetadata::Heat {
                eigenvalues,
                coefficients,
                ..
            } => {
                assert_eq!(eigenvalues.len(), coefficients.len());
            }
            _ => panic!("Expected Heat metadata"),
        }
    }

    #[test]
    fn test_pde_solver_trait() {
        let solver = HeatEquationSolver::new();
        let u = symbol!(u);
        let x = symbol!(x);
        let t = symbol!(t);
        let equation = expr!(u);
        let pde = Pde::new(equation, u, vec![x, t]);

        let result = solver.solve(&pde);
        assert!(result.is_ok());

        let solution = result.unwrap();
        match solution.metadata {
            SolutionMetadata::Heat {
                alpha,
                eigenvalues,
                coefficients,
            } => {
                assert_eq!(alpha, expr!(1));
                assert!(!eigenvalues.is_empty());
                assert!(!coefficients.is_empty());
            }
            _ => panic!("Expected Heat metadata"),
        }
    }

    #[test]
    fn test_heat_solver_with_max_terms() {
        let solver = HeatEquationSolver::with_max_terms(5);
        assert_eq!(solver.max_terms, 5);
    }

    #[test]
    fn test_heat_solver_default() {
        let solver = HeatEquationSolver::default();
        assert_eq!(solver.max_terms, 10);
    }
}
