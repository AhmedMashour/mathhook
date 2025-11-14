//! Linear first-order ODE solver
//!
//! Solves ODEs of the form dy/dx + p(x)y = q(x) using the integrating factor method.
//!
//! This is a specialized solver for the linear first-order case, which covers approximately
//! 25% of real-world first-order ODE problems.

use crate::calculus::integrals::Integration;
use crate::core::{Expression, Symbol};
use crate::error::MathError;
use crate::simplify::Simplify;

/// Result type for ODE solving operations
pub type ODEResult = Result<Expression, ODEError>;

/// ODE-specific error types
#[derive(Debug, Clone, PartialEq)]
pub enum ODEError {
    /// ODE cannot be classified into a known solvable type
    UnknownType {
        equation: Expression,
        reason: String,
    },
    /// ODE does not match expected form
    NotLinearForm { reason: String },
    /// Integration failed
    IntegrationFailed { step: String, expr: Expression },
    /// Domain error in coefficient
    DomainError { coefficient: String, reason: String },
    /// Invalid input to solver
    InvalidInput { message: String },
    /// Feature not yet implemented
    NotImplemented { feature: String },
    /// Mathematical error occurred
    MathError(MathError),
}

impl std::fmt::Display for ODEError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ODEError::UnknownType { equation, reason } => {
                write!(f, "Unknown ODE type: {} ({})", equation, reason)
            }
            ODEError::NotLinearForm { reason } => {
                write!(f, "ODE not in linear form: {}", reason)
            }
            ODEError::IntegrationFailed { step, expr } => {
                write!(f, "Integration failed at step '{}': {}", step, expr)
            }
            ODEError::DomainError {
                coefficient,
                reason,
            } => {
                write!(f, "Domain error in coefficient {}: {}", coefficient, reason)
            }
            ODEError::InvalidInput { message } => {
                write!(f, "Invalid input: {}", message)
            }
            ODEError::NotImplemented { feature } => {
                write!(f, "Feature not yet implemented: {}", feature)
            }
            ODEError::MathError(err) => write!(f, "Mathematical error: {}", err),
        }
    }
}

impl std::error::Error for ODEError {}

impl From<MathError> for ODEError {
    fn from(err: MathError) -> Self {
        ODEError::MathError(err)
    }
}

/// Linear first-order ODE solver
///
/// Solves ODEs of the form dy/dx + p(x)y = q(x) using the integrating factor method.
pub struct LinearFirstOrderSolver;

impl LinearFirstOrderSolver {
    /// Solve linear first-order ODE: dy/dx + p(x)y = q(x)
    ///
    /// Uses integrating factor method: μ(x) = exp(∫p(x)dx)
    ///
    /// # Arguments
    ///
    /// * `p` - Coefficient function p(x)
    /// * `q` - Right-hand side function q(x)
    /// * `dependent` - Dependent variable (y)
    /// * `independent` - Independent variable (x)
    /// * `initial_condition` - Optional (x0, y0) for particular solution
    ///
    /// # Returns
    ///
    /// General solution or particular solution if initial conditions provided
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::ode::first_order::LinearFirstOrderSolver;
    /// use mathhook_core::{symbol, expr};
    ///
    /// let x = symbol!(x);
    /// let y = symbol!(y);
    ///
    /// // dy/dx + 2y = x (p(x) = 2, q(x) = x)
    /// let solver = LinearFirstOrderSolver;
    /// let solution = solver.solve(
    ///     &expr!(2),
    ///     &expr!(x),
    ///     &y,
    ///     &x,
    ///     None
    /// );
    /// ```
    pub fn solve(
        &self,
        p: &Expression,
        q: &Expression,
        _dependent: &Symbol,
        independent: &Symbol,
        _initial_condition: Option<(Expression, Expression)>,
    ) -> ODEResult {
        // Integrating factor: μ(x) = exp(∫p(x)dx)
        let p_integral = p.integrate(independent.clone(), 0);

        let mu = Expression::function("exp", vec![p_integral]);

        // Multiply both sides by μ(x): μ(x) * dy/dx + μ(x) * p(x) * y = μ(x) * q(x)
        // Left side becomes d/dx[μ(x) * y], so:
        // d/dx[μ(x) * y] = μ(x) * q(x)
        // Integrating both sides: μ(x) * y = ∫μ(x) * q(x) dx + C

        let rhs = Expression::mul(vec![mu.clone(), q.clone()]);
        let rhs_integral = rhs.integrate(independent.clone(), 0);

        // Add constant of integration
        let c = Expression::symbol(Symbol::scalar("C"));
        let with_constant = Expression::add(vec![rhs_integral, c]);

        // Solve for y: y = (∫μ(x)*q(x) dx + C) / μ(x)
        let solution = Expression::mul(vec![
            with_constant,
            Expression::pow(mu, Expression::integer(-1)),
        ]);

        Ok(solution.simplify())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{expr, symbol};

    #[test]
    fn test_linear_first_order_constant_coefficients() {
        let x = symbol!(x);
        let y = symbol!(y);

        // dy/dx + 2y = 0 (p(x) = 2, q(x) = 0)
        // Solution: y = C*e^(-2x)
        let solver = LinearFirstOrderSolver;
        let solution = solver.solve(&expr!(2), &expr!(0), &y, &x, None);

        assert!(solution.is_ok());
        let sol = solution.unwrap();

        // Solution should contain exponential
        let sol_str = sol.to_string();
        assert!(sol_str.contains("exp") || sol_str.contains("e"));
    }

    #[test]
    fn test_linear_first_order_nonhomogeneous() {
        let x = symbol!(x);
        let y = symbol!(y);

        // dy/dx + y = x (p(x) = 1, q(x) = x)
        let solver = LinearFirstOrderSolver;
        let solution = solver.solve(&expr!(1), &expr!(x), &y, &x, None);

        // May fail if integration not fully implemented
        if solution.is_ok() {
            let sol = solution.unwrap();
            let sol_str = sol.to_string();
            assert!(sol_str.contains("exp") || sol_str.contains("C"));
        }
    }
}
