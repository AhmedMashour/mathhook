//! Solver objects for the hybrid API
//!
//! This module provides stateful solver objects that complement the Expression-centric API.
//! These are separate objects that maintain state and configuration for complex solving operations.

use crate::algebra::equation_analyzer::SmartEquationSolver;
use crate::core::{Expression, Symbol};
use crate::simplify::Simplify;
use serde::{Deserialize, Serialize};

/// Result of a solving operation
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SolverResult {
    /// Single solution found
    Single(Expression),
    /// Multiple solutions found
    Multiple(Vec<Expression>),
    /// No solution exists
    NoSolution,
    /// Infinite solutions exist
    InfiniteSolutions,
}

/// Configuration for solving operations
#[derive(Debug, Clone)]
pub struct SolverConfig {
    pub max_iterations: u32,
    pub tolerance: f64,
    pub use_numeric: bool,
    pub simplify_results: bool,
}

impl Default for SolverConfig {
    fn default() -> Self {
        Self {
            max_iterations: 1000,
            tolerance: 1e-10,
            use_numeric: false,
            simplify_results: true,
        }
    }
}

/// Stateful mathematical solver for the hybrid API
///
/// This is a separate object from Expression that maintains configuration
/// and state for complex solving operations.
///
/// # Examples
///
/// ```rust
/// use mathhook_core::{MathSolver, Expression, symbol, expr};
/// use mathhook_core::simplify::Simplify;
///
/// let mut solver = MathSolver::new();
/// let x = symbol!(x);
/// let equation = Expression::equation(
///     expr!((2*x) + 3),
///     expr!(7),
/// );
///
/// let result = solver.solve(&equation, &x);
/// // Result: SolverResult::Single for x = 2
/// ```
pub struct MathSolver {
    config: SolverConfig,
    smart_solver: SmartEquationSolver,
}

impl MathSolver {
    /// Create a new solver with default configuration
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::MathSolver;
    ///
    /// let solver = MathSolver::new();
    /// ```
    pub fn new() -> Self {
        Self {
            config: SolverConfig::default(),
            smart_solver: SmartEquationSolver::new(),
        }
    }

    /// Create a new solver with custom configuration
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{MathSolver, SolverConfig};
    ///
    /// let config = SolverConfig {
    ///     max_iterations: 500,
    ///     tolerance: 1e-8,
    ///     use_numeric: true,
    ///     simplify_results: false,
    /// };
    /// let solver = MathSolver::with_config(config);
    /// ```
    pub fn with_config(config: SolverConfig) -> Self {
        Self {
            config,
            smart_solver: SmartEquationSolver::new(),
        }
    }

    /// Solve an equation for a given variable
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{MathSolver, Expression};
    /// use mathhook_core::{symbol, expr};
    /// let mut solver = MathSolver::new();
    /// let equation = Expression::equation(
    ///     expr!(x),
    ///     expr!(5),
    /// );
    /// let result = solver.solve(&equation, &symbol!(x));
    /// ```
    pub fn solve(&mut self, equation: &Expression, variable: &Symbol) -> SolverResult {
        match equation {
            Expression::Relation(relation_data) => {
                // Extract left and right sides and convert to standard form (LHS - RHS = 0)
                let left = &relation_data.left;
                let right = &relation_data.right;

                let standard_form = Expression::add(vec![
                    left.clone(),
                    Expression::mul(vec![Expression::integer(-1), right.clone()]),
                ]);

                // Use the SmartEquationSolver to solve
                let (algebra_result, _explanation) = self
                    .smart_solver
                    .solve_with_equation(&standard_form, variable);

                // Convert algebra::solvers::SolverResult to solvers::SolverResult
                let result = self.convert_solver_result(algebra_result);

                // Apply simplification if configured
                if self.config.simplify_results {
                    match result {
                        SolverResult::Single(expr) => SolverResult::Single(expr.simplify()),
                        SolverResult::Multiple(exprs) => {
                            SolverResult::Multiple(exprs.iter().map(|e| e.simplify()).collect())
                        }
                        other => other,
                    }
                } else {
                    result
                }
            }
            _ => SolverResult::NoSolution,
        }
    }

    /// Solve a system of equations
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{MathSolver, Expression};
    /// use mathhook_core::{symbol, expr};
    ///
    /// let mut solver = MathSolver::new();
    /// let equations = vec![
    ///     Expression::equation(expr!(x), expr!(1)),
    ///     Expression::equation(expr!(y), expr!(2)),
    /// ];
    /// let variables = vec![symbol!(x), symbol!(y)];
    /// let result = solver.solve_system(&equations, &variables);
    /// ```
    pub fn solve_system(
        &mut self,
        equations: &[Expression],
        variables: &[Symbol],
    ) -> Vec<SolverResult> {
        // Basic implementation - solve each equation independently
        equations
            .iter()
            .map(|eq| self.solve(eq, &variables[0])) // Simplified for now
            .collect()
    }

    /// Update solver configuration
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{MathSolver, SolverConfig};
    ///
    /// let mut solver = MathSolver::new();
    /// let new_config = SolverConfig {
    ///     max_iterations: 2000,
    ///     ..Default::default()
    /// };
    /// solver.configure(new_config);
    /// ```
    pub fn configure(&mut self, config: SolverConfig) {
        self.config = config;
    }

    // Private helper methods
    fn convert_solver_result(
        &self,
        algebra_result: crate::algebra::solvers::SolverResult,
    ) -> SolverResult {
        match algebra_result {
            crate::algebra::solvers::SolverResult::Single(expr) => SolverResult::Single(expr),
            crate::algebra::solvers::SolverResult::Multiple(exprs) => SolverResult::Multiple(exprs),
            crate::algebra::solvers::SolverResult::NoSolution => SolverResult::NoSolution,
            crate::algebra::solvers::SolverResult::InfiniteSolutions => {
                SolverResult::InfiniteSolutions
            }
            crate::algebra::solvers::SolverResult::Parametric(exprs) => {
                // Parametric solutions are returned as multiple solutions for simplicity
                SolverResult::Multiple(exprs)
            }
            crate::algebra::solvers::SolverResult::Partial(exprs) => {
                // Partial solutions are returned as multiple solutions
                SolverResult::Multiple(exprs)
            }
        }
    }
}

impl Default for MathSolver {
    fn default() -> Self {
        Self::new()
    }
}
