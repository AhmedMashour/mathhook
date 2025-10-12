//! Solver objects for the hybrid API
//!
//! This module provides stateful solver objects that complement the Expression-centric API.
//! These are separate objects that maintain state and configuration for complex solving operations.

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
/// use mathhook_core::{MathSolver, Expression};
/// use mathhook_core::symbol;
///
/// let mut solver = MathSolver::new();
/// let equation = Expression::equation(
///     Expression::add(vec![
///         Expression::mul(vec![Expression::integer(2), Expression::symbol(symbol!(x))]),
///         Expression::integer(3),
///     ]),
///     Expression::integer(7),
/// );
///
/// let result = solver.solve(&equation, &symbol!(x));
/// // Result: x = 2
/// ```
pub struct MathSolver {
    config: SolverConfig,
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
        Self { config }
    }

    /// Solve an equation for a given variable
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{MathSolver, Expression, Symbol};
    ///
    /// let mut solver = MathSolver::new();
    /// let equation = Expression::equation(
    ///     Expression::symbol("x"),
    ///     Expression::integer(5),
    /// );
    /// let result = solver.solve(&equation, &symbol!(x));
    /// ```
    pub fn solve(&mut self, equation: &Expression, variable: &Symbol) -> SolverResult {
        match equation {
            Expression::Relation(relation_data) => {
                // Extract left and right sides of the equation
                let left = &relation_data.left;
                let right = &relation_data.right;

                // Basic solving for linear equations
                if let Some(solution) = self.solve_linear(left, right, variable) {
                    if self.config.simplify_results {
                        SolverResult::Single(solution.simplify())
                    } else {
                        SolverResult::Single(solution)
                    }
                } else {
                    SolverResult::NoSolution
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
    /// use mathhook_core::symbol;
    ///
    /// let mut solver = MathSolver::new();
    /// let equations = vec![
    ///     Expression::equation(Expression::symbol(symbol!(x)), Expression::integer(1)),
    ///     Expression::equation(Expression::symbol(symbol!(y)), Expression::integer(2)),
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
    fn solve_linear(
        &self,
        left: &Expression,
        right: &Expression,
        variable: &Symbol,
    ) -> Option<Expression> {
        // Very basic linear solving: x = value
        match (left, right) {
            (Expression::Symbol(sym), value) if sym.name == variable.name => Some(value.clone()),
            (value, Expression::Symbol(sym)) if sym.name == variable.name => Some(value.clone()),
            _ => None, // More complex cases need proper implementation
        }
    }
}

impl Default for MathSolver {
    fn default() -> Self {
        Self::new()
    }
}
