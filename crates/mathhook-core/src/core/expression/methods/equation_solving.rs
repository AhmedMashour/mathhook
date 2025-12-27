//! Equation solving methods for expressions
//!
//! This module provides methods for solving equations symbolically,
//! with configurable step-by-step explanations.
//!
//! # Auto-Detection vs Fast Path
//!
//! - `solve()` / `solve_with_steps()` - Auto-detect equation type via `EquationAnalyzer`
//! - Fast path methods (in `fast_path_solvers`) - Skip classification for known types
//!
//! For users who already know their equation type, see `fast_path_solvers` module for:
//! - `solve_linear()` - Linear equations (ax + b = 0)
//! - `solve_quadratic()` - Quadratic equations (ax^2 + bx + c = 0)
//! - `solve_polynomial()` - Polynomial equations (degree 3+)
//! - `solve_ode()` - Ordinary differential equations
//! - `solve_pde()` - Partial differential equations
//! - `solve_system()` - Systems of equations

use super::super::Expression;
use crate::algebra::equation_analyzer::SmartEquationSolver;
use crate::algebra::solvers::SolverResult;
use crate::core::Symbol;
use crate::educational::step_by_step::StepByStepExplanation;

impl Expression {
    /// Solve equation for a variable with auto-detection
    ///
    /// Solves the equation `self = 0` for the given variable.
    /// Automatically detects equation type (linear, quadratic, ODE, etc.) and
    /// routes to the appropriate solver. For performance-critical code where
    /// you already know the equation type, use the fast path methods instead:
    /// - `solve_linear()`
    /// - `solve_quadratic()`
    /// - `solve_polynomial()`
    /// - `solve_ode()`
    /// - `solve_pde()`
    /// - `solve_system()`
    ///
    /// # Arguments
    ///
    /// * `variable` - The variable to solve for
    ///
    /// # Returns
    ///
    /// The solver result containing solutions or error information
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{Expression, symbol};
    /// use mathhook_core::algebra::solvers::SolverResult;
    ///
    /// let x = symbol!(x);
    /// let equation = Expression::add(vec![
    ///     Expression::mul(vec![Expression::integer(2), Expression::symbol(x.clone())]),
    ///     Expression::integer(-6),
    /// ]);
    ///
    /// let result = equation.solve(&x);
    /// match result {
    ///     SolverResult::Single(solution) => {
    ///         assert_eq!(solution, Expression::integer(3));
    ///     }
    ///     _ => panic!("Expected single solution"),
    /// }
    /// ```
    pub fn solve(&self, variable: &Symbol) -> SolverResult {
        let (result, _explanation) = self.solve_with_steps(variable);
        result
    }

    /// Solve equation with step-by-step educational explanation
    ///
    /// Solves the equation `self = 0` for the given variable and generates
    /// an educational explanation showing each solving step. Use this when you need
    /// to teach or explain the solving process. For performance-critical code where
    /// you only need the answer, use `solve()` instead.
    ///
    /// # Arguments
    ///
    /// * `variable` - The variable to solve for
    ///
    /// # Returns
    ///
    /// A tuple of (solver result, step-by-step explanation)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{Expression, symbol};
    ///
    /// let x = symbol!(x);
    /// let equation = Expression::add(vec![
    ///     Expression::mul(vec![Expression::integer(2), Expression::symbol(x.clone())]),
    ///     Expression::integer(-6),
    /// ]);
    ///
    /// let (result, explanation) = equation.solve_with_steps(&x);
    /// // The explanation contains educational content showing how to solve the equation
    /// ```
    pub fn solve_with_steps(&self, variable: &Symbol) -> (SolverResult, StepByStepExplanation) {
        let solver = SmartEquationSolver::new();
        solver.solve_with_equation(self, variable)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::expr;
    use crate::symbol;

    #[test]
    fn test_solve_linear_equation() {
        let x = symbol!(x);
        let equation = Expression::add(vec![expr!(2 * x), expr!(-6)]);

        let result = equation.solve(&x);
        match result {
            SolverResult::Single(solution) => {
                assert_eq!(solution, expr!(3));
            }
            _ => panic!("Expected single solution"),
        }
    }

    #[test]
    fn test_solve_with_steps() {
        let x = symbol!(x);
        let equation = Expression::add(vec![expr!(2 * x), expr!(-6)]);

        let (result, _explanation) = equation.solve_with_steps(&x);
        match result {
            SolverResult::Single(solution) => {
                assert_eq!(solution, Expression::integer(3));
            }
            _ => panic!("Expected single solution"),
        }
    }
}
