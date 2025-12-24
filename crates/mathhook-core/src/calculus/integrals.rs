//! Integration operations and methods
//!
//! Implements symbolic integration including basic antiderivatives,
//! integration by parts, substitution, trigonometric integrals,
//! and definite integrals. Utilizes the existing Expression::Calculus
//! infrastructure and Expression::function support.

mod basic;
pub mod by_parts;
// mod definite;
pub mod educational;
mod function_integrals;
pub mod numerical;
pub mod rational;
pub mod risch;
pub mod strategy;
pub mod substitution;
pub mod table;
pub mod trigonometric;

pub use basic::BasicIntegrals;
pub use by_parts::IntegrationByParts;
// pub use definite::DefiniteIntegrals;
pub use educational::{
    explain_constant_rule, explain_definite_integral, explain_integration_by_parts,
    explain_power_rule, explain_sum_rule, explain_u_substitution,
};
pub use function_integrals::FunctionIntegrals;
pub use numerical::{
    AdaptiveSimpson, GaussianQuadrature, IntegrationConfig, IntegrationResult, NumericalIntegrator,
    RombergIntegration,
};
pub use rational::{integrate_rational, is_rational_function};
pub use substitution::try_substitution;
pub use trigonometric::try_trigonometric_integration;

use crate::core::{Expression, Symbol};
use crate::error::MathError;
use std::collections::HashMap;
use strategy::integrate_with_strategy;

/// Trait for integration operations
/// Added `depth` parameter to prevent infinite recursion in integration by parts.
/// The `depth` parameter tracks recursion depth and enables maximum depth limiting.
///
/// Default depth is 0 (top-level call). Internal implementations increment this
/// to prevent stack overflow in cases where simplification fails.
pub trait Integration {
    /// Compute indefinite integral with recursion depth tracking
    ///
    /// Returns an unevaluated integral if no closed form exists.
    ///
    /// # Arguments
    ///
    /// * `variable` - The variable to integrate with respect to
    /// * `depth` - Current recursion depth (default: 0)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::calculus::integrals::IntegrationMethods;
    /// use mathhook_core::Expression;
    /// use mathhook_core::calculus::integrals::Integration;
    /// use mathhook_core::symbol;
    ///
    /// let x = symbol!(x);
    /// let expr = Expression::pow(Expression::symbol(x.clone()), Expression::integer(2));
    /// let result = expr.integrate(x, 0);
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    fn integrate(&self, variable: Symbol, depth: usize) -> Expression;

    /// Compute definite integral
    ///
    /// # Errors
    ///
    /// Returns `MathError::DomainError` when evaluation at bounds fails.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::Expression;
    /// use mathhook_core::symbol;
    /// use mathhook_core::calculus::integrals::Integration;
    ///
    /// let x = symbol!(x);
    /// let expr = Expression::symbol(x.clone());
    /// let lower = Expression::integer(0);
    /// let upper = Expression::integer(1);
    /// let result = expr.definite_integrate(x, lower, upper);
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    fn definite_integrate(
        &self,
        variable: Symbol,
        lower: Expression,
        upper: Expression,
    ) -> Result<Expression, MathError>;
}

impl Integration for Expression {
    fn integrate(&self, variable: Symbol, depth: usize) -> Expression {
        integrate_with_strategy(self, variable, depth)
    }

    fn definite_integrate(
        &self,
        variable: Symbol,
        lower: Expression,
        upper: Expression,
    ) -> Result<Expression, MathError> {
        let antiderivative = self.integrate(variable.clone(), 0);

        let mut substitutions = HashMap::new();
        substitutions.insert(variable.name().to_string(), upper.clone());
        let upper_val = antiderivative.clone().substitute(&substitutions);

        substitutions.insert(variable.name().to_string(), lower.clone());
        let lower_val = antiderivative.substitute(&substitutions);

        Ok(Expression::add(vec![
            upper_val,
            Expression::mul(vec![Expression::integer(-1), lower_val]),
        ]))
    }
}

/// Integration methods collection
pub struct IntegrationMethods;

impl IntegrationMethods {
    /// Attempt integration by parts
    ///
    /// Uses the IntegrationByParts module to attempt integration by parts.
    ///
    /// # Errors
    ///
    /// Returns `MathError::NotImplemented` if integration by parts cannot be applied
    /// or if no antiderivative can be found.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::Expression;
    /// use mathhook_core::symbol;
    /// use mathhook_core::calculus::integrals::IntegrationMethods;
    ///
    /// let x = symbol!(x);
    /// let expr = Expression::mul(vec![
    ///     Expression::symbol(x.clone()),
    ///     Expression::function("exp", vec![Expression::symbol(x.clone())])
    /// ]);
    /// let result = IntegrationMethods::by_parts(&expr, x);
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn by_parts(expr: &Expression, variable: Symbol) -> Result<Expression, MathError> {
        IntegrationByParts::integrate(expr, variable.clone(), 0).ok_or_else(|| {
            MathError::NotImplemented {
                feature: format!("integration by parts for {}", expr),
            }
        })
    }

    /// Attempt integration by substitution
    ///
    /// Uses u-substitution to integrate composite functions f(g(x)) where
    /// the derivative g'(x) appears in the integrand.
    ///
    /// # Errors
    ///
    /// Returns `MathError::NotImplemented` if no suitable substitution can be found.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{Expression, Symbol};
    /// use mathhook_core::symbol;
    /// use mathhook_core::calculus::integrals::IntegrationMethods;
    ///
    /// let x = symbol!(x);
    /// let expr = Expression::function("sin", vec![
    ///     Expression::pow(Expression::symbol(x.clone()), Expression::integer(2))
    /// ]);
    /// let result = IntegrationMethods::substitution(&expr, x);
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn substitution(expr: &Expression, variable: Symbol) -> Result<Expression, MathError> {
        try_substitution(expr, &variable, 0).ok_or_else(|| MathError::NotImplemented {
            feature: format!("u-substitution for {}", expr),
        })
    }
}
