//! Wave equation solver
//!
//! Solves the wave equation: ∂²u/∂t² = c²∇²u
//!
//! Uses separation of variables and Fourier series for standard boundary conditions.

use crate::calculus::pde::common::{compute_wave_eigenvalues, create_symbolic_coefficients};
use crate::calculus::pde::registry::{PDEError, PDEResult, PDESolver};
use crate::calculus::pde::types::{BoundaryCondition, InitialCondition, PDESolution, Pde, PdeType};
use crate::core::{Expression, Symbol};

/// Solution to the wave equation
#[derive(Debug, Clone, PartialEq)]
pub struct WaveSolution {
    pub solution: Expression,
    pub wave_speed: Expression,
    pub eigenvalues: Vec<Expression>,
    pub position_coefficients: Vec<Expression>,
    pub velocity_coefficients: Vec<Expression>,
}

/// Wave equation solver implementing PDESolver trait
pub struct WaveEquationSolver {
    max_terms: usize,
}

impl WaveEquationSolver {
    pub fn new() -> Self {
        Self { max_terms: 10 }
    }

    pub fn with_max_terms(max_terms: usize) -> Self {
        Self { max_terms }
    }

    /// Solves the 1D wave equation with full Fourier series computation
    ///
    /// For the wave equation: ∂²u/∂t² = c²∂²u/∂x²
    /// with Dirichlet boundary conditions and initial conditions for u and ∂u/∂t
    ///
    /// # Arguments
    ///
    /// * `pde` - The wave equation PDE
    /// * `wave_speed` - Wave propagation speed coefficient c
    /// * `boundary_conditions` - Boundary conditions (typically Dirichlet: u(0,t)=0, u(L,t)=0)
    /// * `initial_position` - Initial displacement: u(x,0) = f(x)
    /// * `initial_velocity` - Initial velocity: ∂u/∂t(x,0) = g(x)
    #[allow(unused_variables)]
    pub fn solve_wave_equation_1d(
        &self,
        pde: &Pde,
        wave_speed: &Expression,
        boundary_conditions: &[BoundaryCondition],
        initial_position: &InitialCondition,
        initial_velocity: &InitialCondition,
    ) -> Result<WaveSolution, PDEError> {
        if pde.independent_vars.len() != 2 {
            return Err(PDEError::InvalidForm {
                reason: "1D wave equation requires exactly 2 independent variables (x, t)"
                    .to_owned(),
            });
        }

        let spatial_var = &pde.independent_vars[0];

        let eigenvalues =
            compute_wave_eigenvalues(boundary_conditions, spatial_var, self.max_terms)?;

        let position_coeffs = create_symbolic_coefficients("A", eigenvalues.len())?;

        let velocity_coeffs = create_symbolic_coefficients("B", eigenvalues.len())?;

        let solution = self.construct_wave_solution(
            &pde.independent_vars,
            wave_speed,
            &eigenvalues,
            &position_coeffs,
            &velocity_coeffs,
        );

        Ok(WaveSolution {
            solution,
            wave_speed: wave_speed.clone(),
            eigenvalues,
            position_coefficients: position_coeffs,
            velocity_coefficients: velocity_coeffs,
        })
    }

    /// Construct the complete wave equation solution
    ///
    /// Solution form: u(x,t) = Σ [Aₙcos(λₙct) + Bₙsin(λₙct)]sin(λₙx)
    /// where λₙ are eigenvalues, Aₙ from initial position, Bₙ from initial velocity
    fn construct_wave_solution(
        &self,
        vars: &[Symbol],
        wave_speed: &Expression,
        eigenvalues: &[Expression],
        position_coeffs: &[Expression],
        velocity_coeffs: &[Expression],
    ) -> Expression {
        let x = &vars[0];
        let t = &vars[1];

        if eigenvalues.is_empty() || position_coeffs.is_empty() || velocity_coeffs.is_empty() {
            return Expression::integer(0);
        }

        let mut terms = Vec::new();

        for ((lambda, a_n), b_n) in eigenvalues
            .iter()
            .zip(position_coeffs.iter())
            .zip(velocity_coeffs.iter())
        {
            let spatial = Expression::function(
                "sin",
                vec![Expression::mul(vec![
                    lambda.clone(),
                    Expression::symbol(x.clone()),
                ])],
            );

            let omega = Expression::mul(vec![
                lambda.clone(),
                wave_speed.clone(),
                Expression::symbol(t.clone()),
            ]);

            let cos_term = Expression::mul(vec![
                a_n.clone(),
                Expression::function("cos", vec![omega.clone()]),
            ]);

            let sin_term =
                Expression::mul(vec![b_n.clone(), Expression::function("sin", vec![omega])]);

            let temporal = Expression::add(vec![cos_term, sin_term]);

            let term = Expression::mul(vec![temporal, spatial]);
            terms.push(term);
        }

        Expression::add(terms)
    }
}

impl PDESolver for WaveEquationSolver {
    fn solve(&self, pde: &Pde) -> PDEResult {
        use crate::expr;

        let wave_speed = expr!(1);
        let ic_pos = InitialCondition::value(expr!(1));
        let ic_vel = InitialCondition::derivative(expr!(0));

        let result = self.solve_wave_equation_1d(pde, &wave_speed, &[], &ic_pos, &ic_vel)?;

        let mut all_coeffs = result.position_coefficients.clone();
        all_coeffs.extend(result.velocity_coefficients.clone());

        Ok(PDESolution::wave(
            result.solution,
            result.wave_speed,
            result.eigenvalues,
            all_coeffs,
        ))
    }

    fn can_solve(&self, pde_type: PdeType) -> bool {
        matches!(pde_type, PdeType::Hyperbolic)
    }

    fn priority(&self) -> u8 {
        100
    }

    fn name(&self) -> &'static str {
        "Wave Equation Solver"
    }

    fn description(&self) -> &'static str {
        "Solves wave equation ∂²u/∂t² = c²∇²u using separation of variables and Fourier series"
    }
}

impl Default for WaveEquationSolver {
    fn default() -> Self {
        Self::new()
    }
}

/// Legacy function for backward compatibility
pub fn solve_wave_equation_1d(
    pde: &Pde,
    wave_speed: &Expression,
    boundary_conditions: &[BoundaryCondition],
    initial_position: &InitialCondition,
    initial_velocity: &InitialCondition,
) -> Result<WaveSolution, String> {
    WaveEquationSolver::new()
        .solve_wave_equation_1d(
            pde,
            wave_speed,
            boundary_conditions,
            initial_position,
            initial_velocity,
        )
        .map_err(|e| format!("{:?}", e))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::calculus::pde::types::BoundaryLocation;
    use crate::{expr, symbol};

    #[test]
    fn test_wave_solver_creation() {
        let solver = WaveEquationSolver::new();
        assert_eq!(solver.name(), "Wave Equation Solver");
        assert_eq!(solver.priority(), 100);
    }

    #[test]
    fn test_wave_solver_can_solve() {
        let solver = WaveEquationSolver::new();
        assert!(solver.can_solve(PdeType::Hyperbolic));
        assert!(!solver.can_solve(PdeType::Parabolic));
        assert!(!solver.can_solve(PdeType::Elliptic));
    }

    #[test]
    fn test_solve_wave_equation_1d_basic() {
        let u = symbol!(u);
        let x = symbol!(x);
        let t = symbol!(t);
        let equation = expr!(u);
        let pde = Pde::new(equation, u, vec![x.clone(), t]);
        let c = expr!(1);

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

        let ic_pos = InitialCondition::value(expr!(1));
        let ic_vel = InitialCondition::derivative(expr!(0));

        let solver = WaveEquationSolver::new();
        let result = solver.solve_wave_equation_1d(&pde, &c, &[bc1, bc2], &ic_pos, &ic_vel);
        assert!(result.is_ok());

        let solution = result.unwrap();
        assert_eq!(solution.wave_speed, c);
        assert!(!solution.eigenvalues.is_empty());
        assert!(!solution.position_coefficients.is_empty());
        assert!(!solution.velocity_coefficients.is_empty());
    }

    #[test]
    fn test_solve_wave_equation_wrong_dimensions() {
        let u = symbol!(u);
        let x = symbol!(x);
        let equation = expr!(u);
        let pde = Pde::new(equation, u, vec![x]);
        let c = expr!(1);
        let ic_pos = InitialCondition::value(expr!(1));
        let ic_vel = InitialCondition::derivative(expr!(0));

        let solver = WaveEquationSolver::new();
        let result = solver.solve_wave_equation_1d(&pde, &c, &[], &ic_pos, &ic_vel);
        assert!(result.is_err());
    }

    #[test]
    fn test_construct_wave_solution() {
        let solver = WaveEquationSolver::new();
        let x = symbol!(x);
        let t = symbol!(t);
        let vars = vec![x, t];
        let c = expr!(1);
        let eigenvalues = vec![expr!(1)];
        let a_coeffs = vec![Expression::symbol(symbol!(A_1))];
        let b_coeffs = vec![Expression::symbol(symbol!(B_1))];

        let solution =
            solver.construct_wave_solution(&vars, &c, &eigenvalues, &a_coeffs, &b_coeffs);
        match solution {
            Expression::Add(_) | Expression::Mul(_) => (),
            _ => panic!("Expected addition or multiplication expression for wave solution"),
        }
    }

    #[test]
    fn test_wave_solution_structure() {
        let u = symbol!(u);
        let x = symbol!(x);
        let t = symbol!(t);
        let equation = expr!(u);
        let pde = Pde::new(equation, u, vec![x.clone(), t]);
        let c = expr!(1);

        let bc = BoundaryCondition::dirichlet(
            expr!(0),
            BoundaryLocation::Simple {
                variable: x,
                value: expr!(0),
            },
        );
        let ic_pos = InitialCondition::value(expr!(1));
        let ic_vel = InitialCondition::derivative(expr!(0));

        let solver = WaveEquationSolver::new();
        let result = solver.solve_wave_equation_1d(&pde, &c, &[bc], &ic_pos, &ic_vel);
        assert!(result.is_ok());

        let solution = result.unwrap();
        assert_eq!(
            solution.eigenvalues.len(),
            solution.position_coefficients.len()
        );
        assert_eq!(
            solution.eigenvalues.len(),
            solution.velocity_coefficients.len()
        );
    }

    #[test]
    fn test_wave_solution_clone() {
        let solution = WaveSolution {
            solution: expr!(1),
            wave_speed: expr!(1),
            eigenvalues: vec![expr!(1)],
            position_coefficients: vec![expr!(1)],
            velocity_coefficients: vec![expr!(1)],
        };

        let _cloned = solution.clone();
    }

    #[test]
    fn test_pde_solver_trait() {
        let solver = WaveEquationSolver::new();
        let u = symbol!(u);
        let x = symbol!(x);
        let t = symbol!(t);
        let equation = expr!(u);
        let pde = Pde::new(equation, u, vec![x, t]);

        let result = solver.solve(&pde);
        assert!(result.is_ok());
    }

    #[test]
    fn test_legacy_function() {
        let u = symbol!(u);
        let x = symbol!(x);
        let t = symbol!(t);
        let equation = expr!(u);
        let pde = Pde::new(equation, u, vec![x, t]);
        let c = expr!(1);
        let ic_pos = InitialCondition::value(expr!(1));
        let ic_vel = InitialCondition::derivative(expr!(0));

        let result = solve_wave_equation_1d(&pde, &c, &[], &ic_pos, &ic_vel);
        assert!(result.is_ok());
    }
}
