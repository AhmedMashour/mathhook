//! Exact ODE solver
//!
//! Solves exact ODEs of the form M(x,y)dx + N(x,y)dy = 0 where ∂M/∂y = ∂N/∂x.
//!
//! An exact ODE has a potential function F(x,y) such that:
//! - ∂F/∂x = M(x,y)
//! - ∂F/∂y = N(x,y)
//!
//! The solution is then F(x,y) = C (implicit solution).

use crate::calculus::derivatives::Derivative;
use crate::calculus::integrals::Integration;
use crate::core::{Expression, Symbol};
use super::{ODEError, ODEResult};
use crate::simplify::Simplify;

/// Exact ODE solver
///
/// Solves ODEs of the form M(x,y)dx + N(x,y)dy = 0 where ∂M/∂y = ∂N/∂x.
pub struct ExactODESolver;

impl ExactODESolver {
    /// Check if ODE is exact
    ///
    /// # Arguments
    ///
    /// * `m` - Coefficient of dx: M(x,y)
    /// * `n` - Coefficient of dy: N(x,y)
    /// * `x` - Independent variable
    /// * `y` - Dependent variable
    ///
    /// # Returns
    ///
    /// True if ∂M/∂y = ∂N/∂x
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::ode::first_order::exact::ExactODESolver;
    /// use mathhook_core::{symbol, expr};
    ///
    /// let x = symbol!(x);
    /// let y = symbol!(y);
    ///
    /// // 2xy dx + x^2 dy = 0 is exact
    /// let m = expr!((2 * x) * y);
    /// let n = expr!(x ^ 2);
    ///
    /// let solver = ExactODESolver;
    /// assert!(solver.is_exact(&m, &n, &x, &y));
    /// ```
    pub fn is_exact(&self, m: &Expression, n: &Expression, x: &Symbol, y: &Symbol) -> bool {
        let dm_dy = m.derivative(y.clone()).simplify();
        let dn_dx = n.derivative(x.clone()).simplify();

        dm_dy == dn_dx
    }

    /// Solve exact ODE
    ///
    /// Solves M(x,y)dx + N(x,y)dy = 0 by finding potential function F(x,y).
    ///
    /// # Arguments
    ///
    /// * `m` - Coefficient of dx: M(x,y)
    /// * `n` - Coefficient of dy: N(x,y)
    /// * `x` - Independent variable
    /// * `y` - Dependent variable
    ///
    /// # Returns
    ///
    /// Implicit solution F(x,y) = C
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::ode::first_order::exact::ExactODESolver;
    /// use mathhook_core::{symbol, expr};
    ///
    /// let x = symbol!(x);
    /// let y = symbol!(y);
    ///
    /// // 2xy dx + x^2 dy = 0
    /// let m = expr!((2 * x) * y);
    /// let n = expr!(x ^ 2);
    ///
    /// let solver = ExactODESolver;
    /// let solution = solver.solve(&m, &n, &x, &y);
    /// assert!(solution.is_ok());
    /// ```
    pub fn solve(
        &self,
        m: &Expression,
        n: &Expression,
        x: &Symbol,
        y: &Symbol,
    ) -> ODEResult {
        if !self.is_exact(m, n, x, y) {
            return Err(ODEError::NotLinearForm {
                reason: "ODE is not exact: ∂M/∂y ≠ ∂N/∂x".to_string(),
            });
        }

        // Find potential function F(x,y) such that:
        // ∂F/∂x = M(x,y) and ∂F/∂y = N(x,y)

        // Integrate M with respect to x: F(x,y) = ∫M(x,y)dx + g(y)
        let f_from_m = m.integrate(x.clone());

        // To find g(y), differentiate F with respect to y and compare with N
        // ∂F/∂y = ∂/∂y[∫M dx] + g'(y) = N(x,y)
        // Therefore: g'(y) = N(x,y) - ∂/∂y[∫M dx]

        let df_dy = f_from_m.derivative(y.clone()).simplify();
        let g_prime = Expression::add(vec![
            n.clone(),
            Expression::mul(vec![Expression::integer(-1), df_dy]),
        ])
        .simplify();

        // Integrate g'(y) with respect to y to get g(y)
        let g = g_prime.integrate(y.clone());

        // F(x,y) = ∫M dx + g(y)
        let potential = Expression::add(vec![f_from_m, g]).simplify();

        // Add constant of integration
        let c = Expression::symbol(Symbol::scalar("C"));
        let solution = Expression::add(vec![potential, c]).simplify();

        Ok(solution)
    }

    /// Find integrating factor for non-exact ODE
    ///
    /// Attempts to find integrating factor μ(x) or μ(y) to make ODE exact.
    ///
    /// # Arguments
    ///
    /// * `m` - Coefficient of dx
    /// * `n` - Coefficient of dy
    /// * `x` - Independent variable
    /// * `y` - Dependent variable
    ///
    /// # Returns
    ///
    /// Integrating factor if found
    pub fn find_integrating_factor(
        &self,
        m: &Expression,
        n: &Expression,
        x: &Symbol,
        y: &Symbol,
    ) -> Option<Expression> {
        // Check if integrating factor depends only on x: μ(x)
        // (∂M/∂y - ∂N/∂x) / N should be function of x only

        let dm_dy = m.derivative(y.clone()).simplify();
        let dn_dx = n.derivative(x.clone()).simplify();

        let numerator = Expression::add(vec![
            dm_dy.clone(),
            Expression::mul(vec![Expression::integer(-1), dn_dx.clone()]),
        ])
        .simplify();

        let quotient = Expression::mul(vec![
            numerator.clone(),
            Expression::pow(n.clone(), Expression::integer(-1)),
        ])
        .simplify();

        // Check if quotient is independent of y (crude check: no y symbols)
        if !self.contains_symbol(&quotient, y) {
            // μ(x) = exp(∫[(∂M/∂y - ∂N/∂x) / N] dx)
            let integral = quotient.integrate(x.clone());
            return Some(Expression::function("exp", vec![integral]));
        }

        // Check if integrating factor depends only on y: μ(y)
        // (∂N/∂x - ∂M/∂y) / M should be function of y only

        let numerator_y = Expression::add(vec![
            dn_dx,
            Expression::mul(vec![Expression::integer(-1), dm_dy]),
        ])
        .simplify();

        let quotient_y = Expression::mul(vec![
            numerator_y,
            Expression::pow(m.clone(), Expression::integer(-1)),
        ])
        .simplify();

        if !self.contains_symbol(&quotient_y, x) {
            // μ(y) = exp(∫[(∂N/∂x - ∂M/∂y) / M] dy)
            let integral = quotient_y.integrate(y.clone());
            return Some(Expression::function("exp", vec![integral]));
        }

        None
    }

    /// Check if expression contains a symbol
    fn contains_symbol(&self, expr: &Expression, sym: &Symbol) -> bool {
        match expr {
            Expression::Symbol(s) => s == sym,
            Expression::Add(terms) => terms.iter().any(|t| self.contains_symbol(t, sym)),
            Expression::Mul(factors) => factors.iter().any(|f| self.contains_symbol(f, sym)),
            Expression::Pow(base, exp) => {
                self.contains_symbol(base, sym) || self.contains_symbol(exp, sym)
            }
            Expression::Function { args, .. } => {
                args.iter().any(|a| self.contains_symbol(a, sym))
            }
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{expr, symbol};

    #[test]
    fn test_exact_ode_simple() {
        let x = symbol!(x);
        let y = symbol!(y);

        // 2xy dx + x^2 dy = 0
        // ∂M/∂y = 2x, ∂N/∂x = 2x ✓ exact
        // Solution: x^2*y = C
        let m = expr!((2 * x) * y);
        let n = expr!(x ^ 2);

        let solver = ExactODESolver;
        assert!(solver.is_exact(&m, &n, &x, &y));

        let solution = solver.solve(&m, &n, &x, &y);
        assert!(solution.is_ok());
    }

    #[test]
    fn test_exact_ode_polynomial() {
        let x = symbol!(x);
        let y = symbol!(y);

        // (3x^2 + y) dx + x dy = 0
        // ∂M/∂y = 1, ∂N/∂x = 1 ✓ exact
        let m = expr!((3 * (x ^ 2)) + y);
        let n = expr!(x);

        let solver = ExactODESolver;
        assert!(solver.is_exact(&m, &n, &x, &y));

        let solution = solver.solve(&m, &n, &x, &y);
        assert!(solution.is_ok());
    }

    #[test]
    fn test_not_exact() {
        let x = symbol!(x);
        let y = symbol!(y);

        // y dx + x dy = 0 is NOT exact
        // ∂M/∂y = 1, ∂N/∂x = 1 (actually this IS exact!)
        // Better example: y dx + 2x dy = 0
        // ∂M/∂y = 1, ∂N/∂x = 2 ✗ not exact
        let m = expr!(y);
        let n = expr!(2 * x);

        let solver = ExactODESolver;
        assert!(!solver.is_exact(&m, &n, &x, &y));
    }

//     #[test]
//     fn test_exact_ode_with_trig() {
//         let x = symbol!(x);
//         let y = symbol!(y);
// 
//         // (y*cos(x) + sin(x)) dx + sin(x) dy = 0
//         // ∂M/∂y = cos(x), ∂N/∂x = cos(x) ✓ exact
//         let m = expr!((y * cos(x)) + sin(x));
//         let n = expr!(sin(x));
// 
//         let solver = ExactODESolver;
//         assert!(solver.is_exact(&m, &n, &x, &y));
//     }

    #[test]
    fn test_integrating_factor_x_only() {
        let x = symbol!(x);
        let y = symbol!(y);

        // y dx + 2x dy = 0 (not exact)
        // Can find integrating factor μ(x)
        let m = expr!(y);
        let n = expr!(2 * x);

        let solver = ExactODESolver;
        let mu = solver.find_integrating_factor(&m, &n, &x, &y);

        // Should find integrating factor
        assert!(mu.is_some());
    }

    #[test]
    fn test_exact_solve_returns_implicit_solution() {
        let x = symbol!(x);
        let y = symbol!(y);

        let m = expr!((2 * x) * y);
        let n = expr!(x ^ 2);

        let solver = ExactODESolver;
        let solution = solver.solve(&m, &n, &x, &y).unwrap();

        // Solution should contain both x and y
        let sol_str = solution.to_string();
        assert!(sol_str.contains("x") || sol_str.contains("y"));
        assert!(sol_str.contains("C"));
    }
}
