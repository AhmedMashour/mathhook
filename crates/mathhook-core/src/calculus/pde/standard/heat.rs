//! Heat equation solver
//!
//! Solves the heat equation: ∂u/∂t = α∇²u
//!
//! ⚠️ **CURRENT LIMITATION**: Returns solutions with symbolic Fourier coefficients
//! (A₁, A₂, A₃, ...). Numerical evaluation of these coefficients requires symbolic
//! integration, which is not yet implemented in MathHook.
//!
//! **What you get**: Correct solution structure `u(x,t) = Σ Aₙ sin(√λₙ x) exp(-λₙ α t)`
//! where λₙ are correctly computed eigenvalues
//!
//! **What's missing**: Actual values of Aₙ computed from initial conditions via
//! Fourier series expansion (requires symbolic integration)
//!
//! # Examples
//!
//! ```rust
//! // This returns a solution with correctly computed eigenvalues
//! // but symbolic coefficients A_1, A_2, A_3, ...
//! # use mathhook_core::calculus::pde::standard::heat::HeatEquationSolver;
//! # use mathhook_core::calculus::pde::types::{Pde, BoundaryCondition, InitialCondition, BoundaryLocation};
//! # use mathhook_core::{symbol, expr};
//! # let solver = HeatEquationSolver::new();
//! # let u = symbol!(u);
//! # let x = symbol!(x);
//! # let t = symbol!(t);
//! # let equation = expr!(u);
//! # let pde = Pde::new(equation, u, vec![x.clone(), t]);
//! # let alpha = expr!(1);
//! # let bc1 = BoundaryCondition::dirichlet(expr!(0), BoundaryLocation::Simple { variable: x.clone(), value: expr!(0) });
//! # let bc2 = BoundaryCondition::dirichlet(expr!(0), BoundaryLocation::Simple { variable: x, value: expr!(1) });
//! # let ic = InitialCondition::value(expr!(1));
//! let solution = solver.solve_heat_equation_1d(&pde, &alpha, &[bc1, bc2], &ic);
//! // solution.eigenvalues = [(π)², (2π)², (3π)², ...]  (correctly computed)
//! // solution.coefficients = [A_1, A_2, A_3, ...]  (symbolic, not computed)
//! ```
//!
//! Uses separation of variables and Fourier series for standard boundary conditions.

use crate::calculus::pde::common::{
    compute_dirichlet_1d_eigenvalues, create_symbolic_coefficients,
};
use crate::calculus::pde::registry::{PDEError, PDEResult, PDESolver};
use crate::calculus::pde::types::{BoundaryCondition, InitialCondition, PDESolution, Pde, PdeType};
use crate::core::{Expression, Symbol};

/// Solution to the heat equation (legacy type for backward compatibility)
#[deprecated(since = "0.1.0", note = "Use PDESolution instead")]
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
    /// * `initial_condition` - Initial temperature distribution
    ///
    /// # Mathematical Background
    ///
    /// Solution form: u(x,t) = Σ Aₙ sin(√λₙ x) exp(-λₙ α t)
    /// where λₙ are eigenvalues determined by boundary conditions and Aₙ are
    /// Fourier coefficients determined by initial condition.
    ///
    /// ⚠️ **LIMITATION**: Aₙ coefficients are symbolic placeholders. Computing actual
    /// values requires symbolic integration:
    ///
    /// Aₙ = (2/L) ∫₀ᴸ f(x) sin(√λₙ x) dx
    ///
    /// This feature requires the symbolic integration infrastructure (Phase 2).
    #[allow(deprecated, unused_variables)]
    pub fn solve_heat_equation_1d(
        &self,
        pde: &Pde,
        alpha: &Expression,
        boundary_conditions: &[BoundaryCondition],
        initial_condition: &InitialCondition,
    ) -> Result<HeatSolution, PDEError> {
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

        Ok(HeatSolution {
            solution,
            alpha: alpha.clone(),
            eigenvalues,
            coefficients,
        })
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
    #[allow(deprecated)]
    fn solve(&self, pde: &Pde) -> PDEResult {
        use crate::expr;

        let alpha = expr!(1);
        let ic = InitialCondition::value(expr!(1));

        let legacy_sol = self.solve_heat_equation_1d(pde, &alpha, &[], &ic)?;

        Ok(PDESolution::heat(
            legacy_sol.solution,
            legacy_sol.alpha,
            legacy_sol.eigenvalues,
            legacy_sol.coefficients,
        ))
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
        "Solves heat equation ∂u/∂t = α∇²u using separation of variables and Fourier series"
    }
}

impl Default for HeatEquationSolver {
    fn default() -> Self {
        Self::new()
    }
}

#[allow(deprecated, unused_variables)]
#[deprecated(
    since = "0.1.0",
    note = "Use HeatEquationSolver::new().solve() instead"
)]
pub fn solve_heat_equation_1d(
    pde: &Pde,
    alpha: &Expression,
    boundary_conditions: &[BoundaryCondition],
    initial_condition: &InitialCondition,
) -> Result<HeatSolution, String> {
    HeatEquationSolver::new()
        .solve_heat_equation_1d(pde, alpha, boundary_conditions, initial_condition)
        .map_err(|e| format!("{:?}", e))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::calculus::pde::types::BoundaryLocation;
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
    #[allow(deprecated)]
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
        assert_eq!(solution.alpha, alpha);
        assert!(!solution.eigenvalues.is_empty());
        assert!(!solution.coefficients.is_empty());
    }

    #[test]
    #[allow(deprecated)]
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
    #[allow(deprecated)]
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
        assert_eq!(solution.eigenvalues.len(), solution.coefficients.len());
    }

    #[test]
    #[allow(deprecated)]
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
            crate::calculus::pde::types::SolutionMetadata::Heat {
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
    #[allow(deprecated)]
    fn test_legacy_function() {
        let u = symbol!(u);
        let x = symbol!(x);
        let t = symbol!(t);
        let equation = expr!(u);
        let pde = Pde::new(equation, u, vec![x, t]);
        let alpha = expr!(1);
        let ic = InitialCondition::value(expr!(1));

        let result = solve_heat_equation_1d(&pde, &alpha, &[], &ic);
        assert!(result.is_ok());
    }
}
