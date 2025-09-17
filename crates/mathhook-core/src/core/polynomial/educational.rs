//! Polynomial Educational Module
//!
//! Provides step-by-step explanations for polynomial operations.
//! Integrates with the main educational framework to provide
//! detailed, pedagogically sound explanations for:
//!
//! - Polynomial division (long division algorithm)
//! - GCD computation (Euclidean algorithm)
//! - Factorization steps
//! - Resultant and discriminant computation
//!
//! # Architecture
//!
//! This module provides the `PolynomialEducational` trait which can be
//! implemented by polynomial expressions to generate educational explanations.
//! The explanations follow the existing `Step` and `StepByStepExplanation`
//! structures from the main educational module.

use crate::core::{Expression, Symbol};
use crate::educational::step_by_step::{Step, StepByStepExplanation};

pub mod division;
pub mod factorization;
pub mod gcd;

mod impl_trait;

#[cfg(test)]
mod tests;

/// Trait for generating educational explanations for polynomial operations
///
/// This trait provides methods to generate step-by-step explanations
/// for various polynomial algorithms, suitable for educational purposes.
pub trait PolynomialEducational {
    /// Generate step-by-step explanation for polynomial division
    ///
    /// Explains the polynomial long division algorithm step by step,
    /// showing how the quotient and remainder are computed.
    ///
    /// # Arguments
    ///
    /// * `divisor` - The polynomial to divide by
    /// * `var` - The variable to treat as the polynomial indeterminate
    ///
    /// # Returns
    ///
    /// A `StepByStepExplanation` detailing each step of the division
    fn explain_poly_division(&self, divisor: &Expression, var: &Symbol) -> StepByStepExplanation;

    /// Generate step-by-step explanation for GCD computation
    ///
    /// Explains the Euclidean algorithm for polynomial GCD,
    /// showing each division and remainder step.
    ///
    /// # Arguments
    ///
    /// * `other` - The other polynomial
    ///
    /// # Returns
    ///
    /// A `StepByStepExplanation` detailing each step of GCD computation
    fn explain_poly_gcd(&self, other: &Expression) -> StepByStepExplanation;

    /// Generate step-by-step explanation for factorization
    ///
    /// Explains the factorization process, including content extraction
    /// and common factor identification.
    ///
    /// # Arguments
    ///
    /// * `var` - The variable to factor with respect to
    ///
    /// # Returns
    ///
    /// A `StepByStepExplanation` detailing the factorization steps
    fn explain_poly_factorization(&self, var: &Symbol) -> StepByStepExplanation;
}

/// Helper function to create a StepByStepExplanation from components
pub(crate) fn create_explanation(
    initial: Expression,
    final_expr: Expression,
    steps: Vec<Step>,
) -> StepByStepExplanation {
    let total_steps = steps.len().saturating_sub(2);
    let rules_used: Vec<String> = steps.iter().map(|s| s.rule_applied.clone()).collect();

    StepByStepExplanation {
        initial_expression: initial,
        final_expression: final_expr,
        steps,
        total_steps,
        rules_used,
    }
}
