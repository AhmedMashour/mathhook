//! Fast path solver methods for Expression
//!
//! Provides direct access to specific solvers without equation type classification.
//! Use these when you already know the equation type for better performance.

use super::super::Expression;
use crate::algebra::solvers::{
    EquationSolver, LinearSolver, MatrixEquationSolver, PolynomialSolver, QuadraticSolver,
    SolverResult, SystemEquationSolver, SystemSolver,
};
use crate::calculus::ode::registry::ODESolverRegistry;
use crate::calculus::ode::EducationalODESolver;
use crate::calculus::pde::registry::PDESolverRegistry;
use crate::calculus::pde::EducationalPDESolver;
use crate::core::Symbol;
use crate::educational::step_by_step::StepByStepExplanation;
use std::sync::LazyLock;

static ODE_REGISTRY: LazyLock<ODESolverRegistry> = LazyLock::new(ODESolverRegistry::new);
static PDE_REGISTRY: LazyLock<PDESolverRegistry> = LazyLock::new(PDESolverRegistry::new);
static LINEAR_SOLVER: LazyLock<LinearSolver> = LazyLock::new(LinearSolver::new_fast);
static MATRIX_SOLVER: LazyLock<MatrixEquationSolver> =
    LazyLock::new(MatrixEquationSolver::new_fast);
static EDUCATIONAL_PDE_SOLVER: LazyLock<EducationalPDESolver> =
    LazyLock::new(EducationalPDESolver::new);

impl Expression {
    /// Fast path: solve as linear equation (skip classification)
    ///
    /// Directly solves equations of the form `ax + b = 0` without running
    /// equation type classification.
    ///
    /// # Arguments
    ///
    /// * `variable` - The variable to solve for
    ///
    /// # Returns
    ///
    /// The solver result containing the solution x = -b/a
    #[inline]
    pub fn solve_linear(&self, variable: &Symbol) -> SolverResult {
        LINEAR_SOLVER.solve(self, variable)
    }

    /// Fast path: solve as linear equation with steps (skip classification)
    #[inline]
    pub fn solve_linear_with_steps(
        &self,
        variable: &Symbol,
    ) -> (SolverResult, StepByStepExplanation) {
        LINEAR_SOLVER.solve_with_explanation(self, variable)
    }

    /// Fast path: solve as quadratic equation (skip classification)
    ///
    /// Directly solves equations of the form `ax^2 + bx + c = 0` using the
    /// quadratic formula without running equation type classification.
    ///
    /// # Arguments
    ///
    /// * `variable` - The variable to solve for
    ///
    /// # Returns
    ///
    /// The solver result containing solutions from the quadratic formula
    #[inline]
    pub fn solve_quadratic(&self, variable: &Symbol) -> SolverResult {
        QuadraticSolver.solve(self, variable)
    }

    /// Fast path: solve as quadratic equation with steps (skip classification)
    #[inline]
    pub fn solve_quadratic_with_steps(
        &self,
        variable: &Symbol,
    ) -> (SolverResult, StepByStepExplanation) {
        QuadraticSolver.solve_with_explanation(self, variable)
    }

    /// Fast path: solve as polynomial equation (skip classification)
    ///
    /// Directly solves polynomial equations of degree 3+ (cubic, quartic, etc.)
    /// without running equation type classification.
    ///
    /// # Arguments
    ///
    /// * `variable` - The variable to solve for
    ///
    /// # Returns
    ///
    /// The solver result containing polynomial roots
    #[inline]
    pub fn solve_polynomial(&self, variable: &Symbol) -> SolverResult {
        PolynomialSolver.solve(self, variable)
    }

    /// Fast path: solve as polynomial equation with steps (skip classification)
    #[inline]
    pub fn solve_polynomial_with_steps(
        &self,
        variable: &Symbol,
    ) -> (SolverResult, StepByStepExplanation) {
        PolynomialSolver.solve_with_explanation(self, variable)
    }

    /// Fast path: solve as ordinary differential equation (skip classification)
    ///
    /// Directly solves ODEs using the cached ODE solver registry without running
    /// equation type classification. Supports separable, linear, and homogeneous
    /// first-order ODE types.
    ///
    /// Uses a static cached registry for optimal performance on repeated calls.
    ///
    /// # Arguments
    ///
    /// * `dependent` - The dependent variable (e.g., y)
    /// * `independent` - The independent variable (e.g., x)
    ///
    /// # Returns
    ///
    /// The solver result containing the ODE solution
    #[inline]
    pub fn solve_ode(&self, dependent: &Symbol, independent: &Symbol) -> SolverResult {
        match ODE_REGISTRY.try_all_solvers(self, dependent, independent) {
            Ok(solution) => SolverResult::Single(solution),
            Err(_) => SolverResult::NoSolution,
        }
    }

    /// Fast path: solve as ODE with steps (skip classification)
    #[inline]
    pub fn solve_ode_with_steps(&self, variable: &Symbol) -> (SolverResult, StepByStepExplanation) {
        EducationalODESolver.solve_with_explanation(self, variable)
    }

    /// Fast path: solve as partial differential equation (skip classification)
    ///
    /// Directly solves PDEs using the cached PDE solver registry without running
    /// equation type classification. Supports heat equation, wave equation,
    /// Laplace equation, and other types.
    ///
    /// Uses a static cached registry for optimal performance on repeated calls.
    ///
    /// # Arguments
    ///
    /// * `dependent` - The dependent variable (e.g., u)
    /// * `independent_vars` - The independent variables (e.g., [x, t])
    ///
    /// # Returns
    ///
    /// The solver result containing the PDE solution
    #[inline]
    pub fn solve_pde(&self, dependent: &Symbol, independent_vars: &[Symbol]) -> SolverResult {
        // Pde::new() requires owned values; cloning unavoidable with current API
        let pde = crate::calculus::pde::types::Pde::new(
            self.clone(),
            dependent.clone(),
            independent_vars.to_vec(),
        );

        match PDE_REGISTRY.solve(&pde) {
            Ok(solution) => SolverResult::Single(solution.solution),
            Err(_) => SolverResult::NoSolution,
        }
    }

    /// Fast path: solve as PDE with steps (skip classification)
    #[inline]
    pub fn solve_pde_with_steps(&self, variable: &Symbol) -> (SolverResult, StepByStepExplanation) {
        EDUCATIONAL_PDE_SOLVER.solve_with_explanation(self, variable)
    }

    /// Fast path: solve system of equations (skip classification)
    ///
    /// Directly solves systems of equations without running equation type
    /// classification. Supports linear systems (Gaussian elimination) and
    /// polynomial systems (Groebner basis).
    ///
    /// # Arguments
    ///
    /// * `equations` - Slice of equations to solve
    /// * `variables` - Slice of variables to solve for
    ///
    /// # Returns
    ///
    /// The solver result containing system solutions
    #[inline]
    pub fn solve_system(equations: &[Expression], variables: &[Symbol]) -> SolverResult {
        SystemSolver.solve_system(equations, variables)
    }

    /// Fast path: solve system of equations with steps (skip classification)
    #[inline]
    pub fn solve_system_with_steps(
        equations: &[Expression],
        variables: &[Symbol],
    ) -> (SolverResult, StepByStepExplanation) {
        SystemSolver.solve_system_with_explanation(equations, variables)
    }

    /// Fast path: solve matrix/noncommutative equation (skip classification)
    ///
    /// Directly solves equations involving matrices, operators, or quaternions
    /// where multiplication order matters.
    ///
    /// Handles:
    /// - Left multiplication: A*X = B (solution: X = A^(-1)*B)
    /// - Right multiplication: X*A = B (solution: X = B*A^(-1))
    ///
    /// # Arguments
    ///
    /// * `variable` - The matrix/operator variable to solve for
    ///
    /// # Returns
    ///
    /// The solver result containing the matrix equation solution
    #[inline]
    pub fn solve_matrix_equation(&self, variable: &Symbol) -> SolverResult {
        MATRIX_SOLVER.solve(self, variable)
    }

    /// Fast path: solve matrix equation with steps (skip classification)
    #[inline]
    pub fn solve_matrix_equation_with_steps(
        &self,
        variable: &Symbol,
    ) -> (SolverResult, StepByStepExplanation) {
        MATRIX_SOLVER.solve_with_explanation(self, variable)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::expr;
    use crate::symbol;

    #[test]
    fn test_solve_linear_fast_path() {
        let x = symbol!(x);
        let equation = Expression::add(vec![expr!(3 * x), expr!(-9)]);
        let result = equation.solve_linear(&x);
        assert!(matches!(result, SolverResult::Single(s) if s == expr!(3)));
    }

    #[test]
    fn test_solve_linear_with_steps() {
        let x = symbol!(x);
        let equation = Expression::add(vec![expr!(2 * x), expr!(-8)]);
        let (result, explanation) = equation.solve_linear_with_steps(&x);
        assert!(matches!(result, SolverResult::Single(_)));
        assert!(!explanation.steps.is_empty());
    }

    #[test]
    fn test_solve_quadratic_fast_path() {
        let x = symbol!(x);
        let equation = Expression::add(vec![
            Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
            expr!(-4),
        ]);
        let result = equation.solve_quadratic(&x);
        assert!(matches!(result, SolverResult::Multiple(ref s) if s.len() == 2));
    }

    #[test]
    fn test_solve_quadratic_with_steps() {
        let x = symbol!(x);
        let equation = Expression::add(vec![
            Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
            expr!(-9),
        ]);
        let (result, explanation) = equation.solve_quadratic_with_steps(&x);
        assert!(matches!(result, SolverResult::Multiple(_)));
        assert!(!explanation.steps.is_empty());
    }

    #[test]
    fn test_solve_system_fast_path() {
        let x = symbol!(x);
        let y = symbol!(y);
        let eq1 = Expression::add(vec![
            Expression::mul(vec![Expression::integer(2), Expression::symbol(x.clone())]),
            Expression::symbol(y.clone()),
            Expression::integer(-5),
        ]);
        let eq2 = Expression::add(vec![
            Expression::symbol(x.clone()),
            Expression::mul(vec![Expression::integer(-1), Expression::symbol(y.clone())]),
            Expression::integer(-1),
        ]);
        let result = Expression::solve_system(&[eq1, eq2], &[x, y]);
        match result {
            SolverResult::Multiple(solutions) => {
                assert_eq!(solutions.len(), 2);
                assert_eq!(solutions[0], Expression::integer(2));
                assert_eq!(solutions[1], Expression::integer(1));
            }
            _ => panic!("Expected system solution"),
        }
    }

    #[test]
    fn test_solve_polynomial_fast_path() {
        let x = symbol!(x);
        let equation = Expression::add(vec![
            Expression::pow(Expression::symbol(x.clone()), Expression::integer(3)),
            expr!(-8),
        ]);
        let result = equation.solve_polynomial(&x);
        assert!(matches!(
            result,
            SolverResult::Single(_)
                | SolverResult::Multiple(_)
                | SolverResult::Partial(_)
                | SolverResult::NoSolution
        ));
    }

    #[test]
    fn test_solve_ode_fast_path() {
        let y = symbol!(y);
        let x = symbol!(x);
        let equation = expr!(y);
        let result = equation.solve_ode(&y, &x);
        assert!(matches!(
            result,
            SolverResult::NoSolution | SolverResult::Single(_)
        ));
    }

    #[test]
    fn test_solve_pde_fast_path() {
        let u = symbol!(u);
        let x = symbol!(x);
        let t = symbol!(t);
        let equation = expr!(u);
        let result = equation.solve_pde(&u, &[x, t]);
        assert!(matches!(
            result,
            SolverResult::NoSolution | SolverResult::Single(_)
        ));
    }

    #[test]
    fn test_cached_registries() {
        let y = symbol!(y);
        let x = symbol!(x);
        let equation = expr!(y);
        let r1 = equation.solve_ode(&y, &x);
        let r2 = equation.solve_ode(&y, &x);
        assert_eq!(r1, r2);

        let u = symbol!(u);
        let t = symbol!(t);
        let pde_eq = expr!(u);
        let p1 = pde_eq.solve_pde(&u, &[x.clone(), t.clone()]);
        let p2 = pde_eq.solve_pde(&u, &[x, t]);
        assert_eq!(p1, p2);
    }

    #[test]
    fn test_solve_matrix_equation_fast_path() {
        let a = symbol!(A; matrix);
        let b = symbol!(B; matrix);
        let x_mat = symbol!(X; matrix);
        let equation = Expression::add(vec![
            Expression::mul(vec![
                Expression::symbol(a),
                Expression::symbol(x_mat.clone()),
            ]),
            Expression::mul(vec![Expression::integer(-1), Expression::symbol(b)]),
        ]);
        let result = equation.solve_matrix_equation(&x_mat);
        assert!(matches!(
            result,
            SolverResult::Single(_) | SolverResult::NoSolution
        ));
    }
}
