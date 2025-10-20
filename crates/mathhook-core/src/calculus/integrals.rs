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
pub mod rational;
pub mod strategy;
// mod substitution;
// mod trigonometric;

pub use basic::BasicIntegrals;
pub use by_parts::IntegrationByParts;
// pub use definite::DefiniteIntegrals;
pub use educational::{
    explain_constant_rule, explain_definite_integral, explain_integration_by_parts,
    explain_power_rule, explain_sum_rule, explain_u_substitution,
};
pub use function_integrals::FunctionIntegrals;
pub use rational::{integrate_rational, is_rational_function};
// pub use substitution::IntegrationBySubstitution;
// pub use trigonometric::TrigonometricIntegrals;

use crate::core::{Expression, Symbol};
use strategy::integrate_with_strategy;

/// Trait for integration operations
pub trait Integration {
    /// Compute indefinite integral
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
    /// let result = expr.integrate(x);
    /// ```
    fn integrate(&self, variable: Symbol) -> Expression;

    /// Compute definite integral
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
    /// ```
    fn definite_integrate(
        &self,
        variable: Symbol,
        lower: Expression,
        upper: Expression,
    ) -> Expression;
}

impl Integration for Expression {
    fn integrate(&self, variable: Symbol) -> Expression {
        // Delegate to strategy dispatcher which orchestrates all integration techniques
        integrate_with_strategy(self, variable)
    }

    fn definite_integrate(
        &self,
        variable: Symbol,
        lower: Expression,
        upper: Expression,
    ) -> Expression {
        // Use core Expression::definite_integral constructor
        Expression::definite_integral(self.clone(), variable, lower, upper)
    }
}

/// Integration methods collection
pub struct IntegrationMethods;

impl IntegrationMethods {
    /// Attempt integration by parts
    ///
    /// Uses the IntegrationByParts module to attempt integration by parts.
    /// Falls back to symbolic representation if unable to integrate.
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
    /// ```
    pub fn by_parts(expr: &Expression, variable: Symbol) -> Expression {
        IntegrationByParts::integrate(expr, variable.clone())
            .unwrap_or_else(|| Expression::integral(expr.clone(), variable))
    }

    /// Attempt integration by substitution
    ///
    /// Integration by substitution is not yet fully implemented.
    /// Returns symbolic integral representation.
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
    /// ```
    pub fn substitution(expr: &Expression, variable: Symbol) -> Expression {
        Expression::integral(expr.clone(), variable)
    }
}
