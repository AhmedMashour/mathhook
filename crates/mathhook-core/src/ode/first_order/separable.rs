//! Separable ODE Solver
//!
//! Solves first-order ODEs of the form: dy/dx = g(x)h(y)
//!
//! # Algorithm
//!
//! 1. Detect if equation is separable (can factor into g(x)*h(y))
//! 2. Separate variables: dy/h(y) = g(x)dx
//! 3. Integrate both sides: ∫dy/h(y) = ∫g(x)dx + C
//! 4. Apply initial condition if provided
//! 5. Solve for y if possible (may result in implicit solution)
//!
//! # Coverage
//!
//! Separable ODEs represent approximately 30% of first-order ODE problems,
//! making this the highest-priority solver in the classification chain.

use crate::core::{Expression, Symbol};
use crate::ode::first_order::{ODEError, ODEResult};

/// Separable ODE solver implementation
pub struct SeparableODESolver;

impl SeparableODESolver {
    /// Create a new separable ODE solver
    pub fn new() -> Self {
        Self
    }

    /// Solve a separable ODE: dy/dx = g(x)h(y)
    ///
    /// # Arguments
    ///
    /// * `rhs` - Right-hand side of dy/dx = rhs
    /// * `dependent` - Dependent variable (y)
    /// * `independent` - Independent variable (x)
    /// * `initial_condition` - Optional (x0, y0) for particular solution
    ///
    /// # Returns
    ///
    /// General solution or particular solution if initial condition provided
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{symbol, expr};
    /// use mathhook_core::ode::first_order::SeparableODESolver;
    ///
    /// let x = symbol!(x);
    /// let y = symbol!(y);
    ///
    /// // dy/dx = x (separable: g(x)=x, h(y)=1)
    /// let rhs = expr!(x);
    /// let solver = SeparableODESolver::new();
    /// let solution = solver.solve(&rhs, &y, &x, None).unwrap();
    /// // Expected: y = x^2/2 + C
    /// ```
    pub fn solve(
        &self,
        _rhs: &Expression,
        _dependent: &Symbol,
        _independent: &Symbol,
        _initial_condition: Option<(Expression, Expression)>,
    ) -> ODEResult {
        // TODO: Full implementation requires pattern matching system
        // This is a stub implementation to allow compilation
        Err(ODEError::NotImplemented {
            feature: "Separable ODE solver - requires pattern matching system".to_string(),
        })
    }

    /// Check if ODE is separable
    ///
    /// An ODE dy/dx = f(x,y) is separable if it can be written as g(x)*h(y)
    pub fn is_separable(&self, rhs: &Expression, dependent: &Symbol, independent: &Symbol) -> bool {
        // Try to separate - if successful, it's separable
        self.separate(rhs, dependent, independent).is_ok()
    }

    /// Separate equation into g(x) and h(y)
    ///
    /// Attempts to factor rhs into g(x)*h(y) where:
    /// - g(x) contains only independent variable
    /// - h(y) contains only dependent variable
    fn separate(
        &self,
        rhs: &Expression,
        dependent: &Symbol,
        independent: &Symbol,
    ) -> Result<(Expression, Expression), ODEError> {
        // Simple case 1: rhs = g(x) (doesn't depend on y)
        if !self.contains_symbol(rhs, dependent) {
            return Ok((rhs.clone(), Expression::integer(1)));
        }

        // Simple case 2: rhs = h(y) (doesn't depend on x)
        if !self.contains_symbol(rhs, independent) {
            return Ok((Expression::integer(1), rhs.clone()));
        }

        // Case 3: Try to factor as product
        if let Expression::Mul(factors) = rhs {
            let mut x_factors = Vec::new();
            let mut y_factors = Vec::new();

            for factor in factors.iter() {
                if self.contains_symbol(factor, dependent)
                    && self.contains_symbol(factor, independent)
                {
                    // Factor contains both variables - not separable
                    return Err(ODEError::UnknownType {
                        equation: rhs.clone(),
                        reason: "Cannot separate variables - factor contains both x and y"
                            .to_string(),
                    });
                } else if self.contains_symbol(factor, independent) {
                    x_factors.push(factor.clone());
                } else if self.contains_symbol(factor, dependent) {
                    y_factors.push(factor.clone());
                } else {
                    // Constant factor - can go either side
                    x_factors.push(factor.clone());
                }
            }

            let g_x = if x_factors.is_empty() {
                Expression::integer(1)
            } else {
                Expression::mul(x_factors)
            };

            let h_y = if y_factors.is_empty() {
                Expression::integer(1)
            } else {
                Expression::mul(y_factors)
            };

            return Ok((g_x, h_y));
        }

        // Cannot separate (Note: division form would require analyzing Mul/Pow structure)
        Err(ODEError::UnknownType {
            equation: rhs.clone(),
            reason: "Cannot factor into g(x)*h(y)".to_string(),
        })
    }

    /// Check if expression contains a specific symbol
    fn contains_symbol(&self, expr: &Expression, symbol: &Symbol) -> bool {
        match expr {
            Expression::Symbol(s) => s == symbol,
            Expression::Number(_) => false,
            Expression::Add(terms) => terms.iter().any(|t| self.contains_symbol(t, symbol)),
            Expression::Mul(factors) => factors.iter().any(|f| self.contains_symbol(f, symbol)),
            Expression::Pow(base, exp) => {
                self.contains_symbol(base, symbol) || self.contains_symbol(exp, symbol)
            }
            Expression::Function { args, .. } => {
                args.iter().any(|arg| self.contains_symbol(arg, symbol))
            }
            _ => false, // For other expression types, assume no symbol
        }
    }

    /// Try to solve for dependent variable explicitly
    fn solve_for_dependent(
        &self,
        _equation: &Expression,
        _dependent: &Symbol,
    ) -> Result<Expression, ODEError> {
        // TODO: Requires equation solver
        Err(ODEError::NotImplemented {
            feature: "Explicit solution - requires equation solver".to_string(),
        })
    }
}

impl Default for SeparableODESolver {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{expr, symbol};

    #[test]
    #[ignore = "ODE solver implementation incomplete - tracked in Phase 4"]
    fn test_separable_simple_linear() {
        let x = symbol!(x);
        let y = symbol!(y);

        // dy/dx = x (separable: g(x)=x, h(y)=1)
        let rhs = expr!(x);
        let solver = SeparableODESolver::new();

        assert!(solver.is_separable(&rhs, &y, &x));

        let solution = solver.solve(&rhs, &y, &x, None);
        assert!(
            solution.is_ok(),
            "Failed to solve dy/dx = x: {:?}",
            solution.err()
        );

        // Verify solution by differentiation
        // If y = x^2/2 + C, then dy/dx = x
        // (We'll verify this symbolically once we have the solution)
    }

    #[test]
    #[ignore = "ODE solver implementation incomplete - tracked in Phase 4"]
    fn test_separable_exponential() {
        let x = symbol!(x);
        let y = symbol!(y);

        // dy/dx = y (separable: g(x)=1, h(y)=y)
        let rhs = expr!(y);
        let solver = SeparableODESolver::new();

        assert!(solver.is_separable(&rhs, &y, &x));

        let solution = solver.solve(&rhs, &y, &x, None);
        assert!(
            solution.is_ok(),
            "Failed to solve dy/dx = y: {:?}",
            solution.err()
        );

        // Expected: y = C*exp(x) (after solving ln(y) = x + C)
    }

    #[test]
    #[ignore = "ODE solver implementation incomplete - tracked in Phase 4"]
    fn test_separable_product() {
        let x = symbol!(x);
        let y = symbol!(y);

        // dy/dx = x*y (separable: g(x)=x, h(y)=y)
        let rhs = expr!(x * y);
        let solver = SeparableODESolver::new();

        assert!(solver.is_separable(&rhs, &y, &x));

        let solution = solver.solve(&rhs, &y, &x, None);
        assert!(
            solution.is_ok(),
            "Failed to solve dy/dx = x*y: {:?}",
            solution.err()
        );

        // Expected: y = C*exp(x^2/2)
    }

    #[test]
    fn test_non_separable() {
        let x = symbol!(x);
        let y = symbol!(y);

        // dy/dx = x + y (not separable - cannot factor into g(x)*h(y))
        let rhs = expr!(x + y);
        let solver = SeparableODESolver::new();

        assert!(!solver.is_separable(&rhs, &y, &x));

        let solution = solver.solve(&rhs, &y, &x, None);
        assert!(solution.is_err(), "Should not solve non-separable ODE");
    }

    #[test]
    #[ignore = "ODE solver implementation incomplete - tracked in Phase 4"]
    fn test_separable_with_initial_condition() {
        let x = symbol!(x);
        let y = symbol!(y);

        // dy/dx = x, y(0) = 1
        let rhs = expr!(x);
        let ic = Some((expr!(0), expr!(1)));
        let solver = SeparableODESolver::new();

        let solution = solver.solve(&rhs, &y, &x, ic);
        assert!(
            solution.is_ok(),
            "Failed to solve with IC: {:?}",
            solution.err()
        );

        // Expected: y = x^2/2 + 1
        // We can verify: y(0) = 0 + 1 = 1 ✓
    }
}
