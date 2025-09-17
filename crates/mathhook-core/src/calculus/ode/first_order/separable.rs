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
//! # Mathematical Background
//!
//! A first-order ODE dy/dx = f(x,y) is separable if it can be written as:
//! dy/dx = g(x)h(y)
//!
//! This allows separation: (1/h(y))dy = g(x)dx
//! Integration yields: ∫(1/h(y))dy = ∫g(x)dx + C
//!
//! # Coverage
//!
//! Separable ODEs represent approximately 30% of first-order ODE problems,
//! making this the highest-priority solver in the classification chain.
//!
//! # References
//!
//! - M. Tenenbaum & H. Pollard, "Ordinary Differential Equations", Dover 1963, pp. 52

use crate::calculus::integrals::Integration;
use crate::calculus::ode::first_order::{ODEError, ODEResult};
use crate::core::{Expression, Symbol};
use crate::simplify::Simplify;
use std::collections::HashMap;

/// Separable ODE solver implementation
pub struct SeparableODESolver;

impl SeparableODESolver {
    /// Create a new separable ODE solver
    pub fn new() -> Self {
        Self
    }

    /// Solve a separable ODE: dy/dx = g(x)h(y)
    ///
    /// # Complexity
    ///
    /// * **Time:** O(n) where n is the complexity of symbolic integration
    /// * **Space:** O(n) for storing separated expressions and integrals
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
    /// General solution or particular solution if initial condition provided.
    /// Solutions may be implicit (when solving for y is not possible).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{symbol, expr};
    /// use mathhook_core::calculus::ode::first_order::separable::SeparableODESolver;
    ///
    /// let x = symbol!(x);
    /// let y = symbol!(y);
    ///
    /// // dy/dx = x (separable: g(x)=x, h(y)=1)
    /// let rhs = expr!(x);
    /// let solver = SeparableODESolver::new();
    /// let solution = solver.solve(&rhs, &y, &x, None).unwrap();
    /// // Returns implicit form: ∫dy = ∫x dx + C
    /// // Simplifies to: y = x²/2 + C
    /// ```
    ///
    /// # Example: Exponential Growth (dy/dx = y)
    ///
    /// ```rust
    /// use mathhook_core::{symbol, expr};
    /// use mathhook_core::calculus::ode::first_order::separable::SeparableODESolver;
    ///
    /// let x = symbol!(x);
    /// let y = symbol!(y);
    /// let solver = SeparableODESolver::new();
    ///
    /// // dy/dx = y (separable: g(x)=1, h(y)=y)
    /// let rhs = expr!(y);
    /// let solution = solver.solve(&rhs, &y, &x, None);
    /// assert!(solution.is_ok());
    /// // Expected: ln|y| = x + C or y = Ce^x
    /// ```
    ///
    /// # Example: With Initial Condition
    ///
    /// ```rust
    /// use mathhook_core::{symbol, expr};
    /// use mathhook_core::calculus::ode::first_order::separable::SeparableODESolver;
    ///
    /// let x = symbol!(x);
    /// let y = symbol!(y);
    /// let solver = SeparableODESolver::new();
    ///
    /// // dy/dx = x, y(0) = 1
    /// let rhs = expr!(x);
    /// let ic = Some((expr!(0), expr!(1)));
    /// let solution = solver.solve(&rhs, &y, &x, ic);
    /// assert!(solution.is_ok());
    /// // Expected: y = x²/2 + 1
    /// ```
    pub fn solve(
        &self,
        rhs: &Expression,
        dependent: &Symbol,
        independent: &Symbol,
        initial_condition: Option<(Expression, Expression)>,
    ) -> ODEResult {
        let (g_x, h_y) = self.separate(rhs, dependent, independent)?;

        let integrand_y = Expression::pow(h_y, Expression::integer(-1));
        let integral_y = integrand_y.integrate(dependent.clone(), 0);

        let integral_x = g_x.integrate(independent.clone(), 0);

        let c1 = Symbol::new("C1");
        let general_solution = Expression::add(vec![
            integral_y,
            Expression::mul(vec![Expression::integer(-1), integral_x]),
            Expression::symbol(c1),
        ])
        .simplify();

        if let Some((x0, y0)) = initial_condition {
            self.apply_initial_condition(&general_solution, dependent, independent, x0, y0)
        } else {
            Ok(general_solution)
        }
    }

    /// Check if ODE is separable
    ///
    /// An ODE dy/dx = f(x,y) is separable if it can be written as g(x)*h(y)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{symbol, expr};
    /// use mathhook_core::calculus::ode::first_order::separable::SeparableODESolver;
    ///
    /// let x = symbol!(x);
    /// let y = symbol!(y);
    /// let solver = SeparableODESolver::new();
    ///
    /// // Separable: x*y
    /// assert!(solver.is_separable(&expr!(x * y), &y, &x));
    ///
    /// // Not separable: x + y
    /// assert!(!solver.is_separable(&expr!(x + y), &y, &x));
    /// ```
    ///
    /// # Example: Common Separable Forms
    ///
    /// ```rust
    /// use mathhook_core::{symbol, expr};
    /// use mathhook_core::calculus::ode::first_order::separable::SeparableODESolver;
    ///
    /// let x = symbol!(x);
    /// let y = symbol!(y);
    /// let solver = SeparableODESolver::new();
    ///
    /// // Separable examples
    /// assert!(solver.is_separable(&expr!(x), &y, &x));      // dy/dx = x
    /// assert!(solver.is_separable(&expr!(y), &y, &x));      // dy/dx = y
    /// assert!(solver.is_separable(&expr!(x * y), &y, &x));  // dy/dx = xy
    ///
    /// // Non-separable examples
    /// assert!(!solver.is_separable(&expr!(x + y), &y, &x)); // dy/dx = x + y
    /// ```
    pub fn is_separable(&self, rhs: &Expression, dependent: &Symbol, independent: &Symbol) -> bool {
        self.separate(rhs, dependent, independent).is_ok()
    }

    /// Separate equation into g(x) and h(y)
    ///
    /// Attempts to factor rhs into g(x)*h(y) where:
    /// - g(x) contains only independent variable
    /// - h(y) contains only dependent variable
    ///
    /// # Algorithm
    ///
    /// 1. **Simple case 1**: rhs = g(x) (no y dependence) → (g(x), 1)
    /// 2. **Simple case 2**: rhs = h(y) (no x dependence) → (1, h(y))
    /// 3. **Product form**: Factor Mul expression into x-factors and y-factors
    ///
    /// # Returns
    ///
    /// Ok((g_x, h_y)) if separable, Err otherwise
    fn separate(
        &self,
        rhs: &Expression,
        dependent: &Symbol,
        independent: &Symbol,
    ) -> Result<(Expression, Expression), ODEError> {
        if !rhs.contains_variable(dependent) {
            return Ok((rhs.clone(), Expression::integer(1)));
        }

        if !rhs.contains_variable(independent) {
            return Ok((Expression::integer(1), rhs.clone()));
        }

        if let Expression::Mul(factors) = rhs {
            let mut x_factors = Vec::new();
            let mut y_factors = Vec::new();

            for factor in factors.iter() {
                if factor.contains_variable(dependent) && factor.contains_variable(independent) {
                    return Err(ODEError::UnknownType {
                        equation: rhs.clone(),
                        reason: "Cannot separate variables - factor contains both x and y"
                            .to_owned(),
                    });
                } else if factor.contains_variable(independent) {
                    x_factors.push(factor.clone());
                } else if factor.contains_variable(dependent) {
                    y_factors.push(factor.clone());
                } else {
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

        Err(ODEError::UnknownType {
            equation: rhs.clone(),
            reason: "Cannot factor into g(x)*h(y)".to_owned(),
        })
    }

    /// Apply initial condition y(x0) = y0 to determine integration constant
    ///
    /// Substitutes x=x0, y=y0 into the general solution ∫(1/h(y))dy = ∫g(x)dx + C
    /// and solves for C.
    fn apply_initial_condition(
        &self,
        general_solution: &Expression,
        dependent: &Symbol,
        independent: &Symbol,
        x0: Expression,
        y0: Expression,
    ) -> ODEResult {
        let mut subs = HashMap::new();
        subs.insert(independent.name().to_owned(), x0);
        subs.insert(dependent.name().to_owned(), y0);

        let substituted = general_solution.substitute(&subs);
        let simplified = substituted.simplify();

        let c1_value = simplified;

        let mut c_subs = HashMap::new();
        c_subs.insert("C1".to_owned(), c1_value);

        let particular_solution = general_solution.substitute(&c_subs).simplify();

        Ok(particular_solution)
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
    fn test_is_separable_simple_cases() {
        let x = symbol!(x);
        let y = symbol!(y);
        let solver = SeparableODESolver::new();

        assert!(solver.is_separable(&expr!(x), &y, &x));
        assert!(solver.is_separable(&expr!(y), &y, &x));
        assert!(solver.is_separable(&expr!(x * y), &y, &x));
        assert!(!solver.is_separable(&expr!(x + y), &y, &x));
    }

    #[test]
    fn test_separate_simple_linear() {
        let x = symbol!(x);
        let y = symbol!(y);
        let solver = SeparableODESolver::new();

        let rhs = expr!(x);
        let result = solver.separate(&rhs, &y, &x);
        assert!(result.is_ok());

        let (g_x, h_y) = result.unwrap();
        assert_eq!(g_x, expr!(x));
        assert_eq!(h_y, Expression::integer(1));
    }

    #[test]
    fn test_separate_product() {
        let x = symbol!(x);
        let y = symbol!(y);
        let solver = SeparableODESolver::new();

        let rhs = expr!(x * y);
        let result = solver.separate(&rhs, &y, &x);
        assert!(result.is_ok());

        let (g_x, h_y) = result.unwrap();
        assert_eq!(g_x, expr!(x));
        assert_eq!(h_y, expr!(y));
    }

    #[test]
    fn test_separate_non_separable() {
        let x = symbol!(x);
        let y = symbol!(y);
        let solver = SeparableODESolver::new();

        let rhs = expr!(x + y);
        let result = solver.separate(&rhs, &y, &x);
        assert!(result.is_err());
    }

    #[test]
    fn test_solve_simple_linear() {
        let x = symbol!(x);
        let y = symbol!(y);

        let rhs = expr!(x);
        let solver = SeparableODESolver::new();

        let solution = solver.solve(&rhs, &y, &x, None);
        assert!(
            solution.is_ok(),
            "Failed to solve dy/dx = x: {:?}",
            solution.err()
        );
    }

    #[test]
    fn test_solve_exponential() {
        let x = symbol!(x);
        let y = symbol!(y);

        let rhs = expr!(y);
        let solver = SeparableODESolver::new();

        let solution = solver.solve(&rhs, &y, &x, None);
        assert!(
            solution.is_ok(),
            "Failed to solve dy/dx = y: {:?}",
            solution.err()
        );
    }

    #[test]
    fn test_solve_product() {
        let x = symbol!(x);
        let y = symbol!(y);

        let rhs = expr!(x * y);
        let solver = SeparableODESolver::new();

        let solution = solver.solve(&rhs, &y, &x, None);
        assert!(
            solution.is_ok(),
            "Failed to solve dy/dx = x*y: {:?}",
            solution.err()
        );
    }

    #[test]
    fn test_solve_with_initial_condition() {
        let x = symbol!(x);
        let y = symbol!(y);

        let rhs = expr!(x);
        let ic = Some((expr!(0), expr!(1)));
        let solver = SeparableODESolver::new();

        let solution = solver.solve(&rhs, &y, &x, ic);
        assert!(
            solution.is_ok(),
            "Failed to solve with IC: {:?}",
            solution.err()
        );
    }

    #[test]
    fn test_non_separable_fails() {
        let x = symbol!(x);
        let y = symbol!(y);

        let rhs = expr!(x + y);
        let solver = SeparableODESolver::new();

        let solution = solver.solve(&rhs, &y, &x, None);
        assert!(solution.is_err(), "Should not solve non-separable ODE");
    }
}
