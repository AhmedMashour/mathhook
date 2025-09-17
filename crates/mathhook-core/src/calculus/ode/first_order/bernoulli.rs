//! Bernoulli equation solver
//!
//! Solves Bernoulli ODEs of the form dy/dx + p(x)y = q(x)y^n where n ≠ 0, 1.
//!
//! The Bernoulli equation is solved by the substitution v = y^(1-n), which transforms
//! it into a linear ODE in v:
//! dv/dx + (1-n)p(x)v = (1-n)q(x)

use super::{ODEError, ODEResult};
use crate::calculus::ode::first_order::linear::LinearFirstOrderSolver;
use crate::core::{Expression, Symbol};
use crate::simplify::Simplify;

/// Bernoulli equation solver
///
/// Solves ODEs of the form dy/dx + p(x)y = q(x)y^n where n ≠ 0, 1.
pub struct BernoulliODESolver;

impl BernoulliODESolver {
    /// Solve Bernoulli ODE
    ///
    /// Transforms Bernoulli equation to linear ODE via substitution v = y^(1-n).
    ///
    /// # Arguments
    ///
    /// * `p` - Coefficient function p(x)
    /// * `q` - Right-hand side coefficient q(x)
    /// * `n` - Power of y on right-hand side
    /// * `dependent` - Dependent variable (y)
    /// * `independent` - Independent variable (x)
    ///
    /// # Returns
    ///
    /// General solution y(x)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::calculus::ode::first_order::bernoulli::BernoulliODESolver;
    /// use mathhook_core::{symbol, expr};
    ///
    /// let x = symbol!(x);
    /// let y = symbol!(y);
    ///
    /// // dy/dx + y = x*y^2 (p(x) = 1, q(x) = x, n = 2)
    /// let solver = BernoulliODESolver;
    /// let solution = solver.solve(
    ///     &expr!(1),
    ///     &expr!(x),
    ///     &expr!(2),
    ///     &y,
    ///     &x
    /// );
    /// assert!(solution.is_ok());
    /// ```
    pub fn solve(
        &self,
        p: &Expression,
        q: &Expression,
        n: &Expression,
        dependent: &Symbol,
        independent: &Symbol,
    ) -> ODEResult {
        // Check if n = 0 or n = 1 (these are linear, not Bernoulli)
        let n_simplified = n.simplify();
        if n_simplified == Expression::integer(0) || n_simplified == Expression::integer(1) {
            return Err(ODEError::NotLinearForm {
                reason: "Bernoulli equation requires n ≠ 0, 1 (this is linear)".to_owned(),
            });
        }

        // Substitution: v = y^(1-n)
        // Then: dv/dx = (1-n)y^(-n) * dy/dx

        // Original equation: dy/dx + p(x)y = q(x)y^n
        // Multiply by (1-n)y^(-n): (1-n)y^(-n) dy/dx + (1-n)p(x)y^(1-n) = (1-n)q(x)
        // This becomes: dv/dx + (1-n)p(x)v = (1-n)q(x)

        // Compute (1-n)
        let one_minus_n = Expression::add(vec![
            Expression::integer(1),
            Expression::mul(vec![Expression::integer(-1), n.clone()]),
        ])
        .simplify();

        // New coefficients for linear ODE in v
        let p_prime = Expression::mul(vec![one_minus_n.clone(), p.clone()]).simplify();
        let q_prime = Expression::mul(vec![one_minus_n.clone(), q.clone()]).simplify();

        // Solve linear ODE: dv/dx + p'(x)v = q'(x)
        let linear_solver = LinearFirstOrderSolver;
        let v_solution = linear_solver.solve(&p_prime, &q_prime, dependent, independent, None)?;

        // Back-substitute: v = y^(1-n), so y = v^(1/(1-n))
        let exponent = Expression::pow(one_minus_n, Expression::integer(-1)).simplify();
        let y_solution = Expression::pow(v_solution, exponent).simplify();

        Ok(y_solution)
    }

    /// Detect if ODE is in Bernoulli form
    ///
    /// Checks if equation matches dy/dx + p(x)y = q(x)y^n
    ///
    /// # Arguments
    ///
    /// * `equation` - The ODE equation
    /// * `dependent` - Dependent variable (y)
    /// * `independent` - Independent variable (x)
    ///
    /// # Returns
    ///
    /// Optional (p, q, n) if equation is Bernoulli form
    pub fn detect_form(
        &self,
        _equation: &Expression,
        _dependent: &Symbol,
        _independent: &Symbol,
    ) -> Option<(Expression, Expression, Expression)> {
        // Pattern matching for Bernoulli form would go here
        // This is a placeholder for future enhancement
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{expr, symbol};

    #[test]
    fn test_bernoulli_n_equals_2() {
        let x = symbol!(x);
        let y = symbol!(y);

        // dy/dx + y = x*y^2 (standard Bernoulli with n=2)
        // Substitution: v = y^(-1), becomes dv/dx - v = -x
        let solver = BernoulliODESolver;
        let solution = solver.solve(&expr!(1), &expr!(x), &expr!(2), &y, &x);

        assert!(solution.is_ok());
        let sol = solution.unwrap();
        let sol_str = sol.to_string();
        // Solution should involve exponentials and x
        assert!(sol_str.contains("exp") || sol_str.contains("x"));
    }

    #[test]
    fn test_bernoulli_n_equals_3() {
        let x = symbol!(x);
        let y = symbol!(y);

        // dy/dx + 2y = y^3
        let solver = BernoulliODESolver;
        let solution = solver.solve(&expr!(2), &expr!(1), &expr!(3), &y, &x);

        assert!(solution.is_ok());
    }

    #[test]
    fn test_bernoulli_rejects_n_equals_0() {
        let x = symbol!(x);
        let y = symbol!(y);

        // n = 0 is linear, not Bernoulli
        let solver = BernoulliODESolver;
        let result = solver.solve(&expr!(1), &expr!(x), &expr!(0), &y, &x);

        assert!(result.is_err());
        if let Err(ODEError::NotLinearForm { reason }) = result {
            assert!(reason.contains("linear"));
        }
    }

    #[test]
    fn test_bernoulli_rejects_n_equals_1() {
        let x = symbol!(x);
        let y = symbol!(y);

        // n = 1 is linear, not Bernoulli
        let solver = BernoulliODESolver;
        let result = solver.solve(&expr!(1), &expr!(x), &expr!(1), &y, &x);

        assert!(result.is_err());
        if let Err(ODEError::NotLinearForm { reason }) = result {
            assert!(reason.contains("linear"));
        }
    }

    #[test]
    fn test_bernoulli_negative_n() {
        let x = symbol!(x);
        let y = symbol!(y);

        // dy/dx + y = y^(-1) (n = -1)
        let solver = BernoulliODESolver;
        let solution = solver.solve(&expr!(1), &expr!(1), &expr!(-1), &y, &x);

        assert!(solution.is_ok());
    }

    #[test]
    fn test_bernoulli_fractional_n() {
        let x = symbol!(x);
        let y = symbol!(y);

        // dy/dx + y = y^(1/2) (n = 1/2)
        let solver = BernoulliODESolver;
        let n = Expression::mul(vec![
            Expression::integer(1),
            Expression::pow(Expression::integer(2), Expression::integer(-1)),
        ]);
        let solution = solver.solve(&expr!(1), &expr!(1), &n, &y, &x);

        assert!(solution.is_ok());
    }

    #[test]
    fn test_bernoulli_solution_structure() {
        let x = symbol!(x);
        let y = symbol!(y);

        let solver = BernoulliODESolver;
        let solution = solver
            .solve(&expr!(1), &expr!(1), &expr!(2), &y, &x)
            .unwrap();

        // Solution should contain constant C
        let sol_str = solution.to_string();
        assert!(sol_str.contains("C"));
    }
}
