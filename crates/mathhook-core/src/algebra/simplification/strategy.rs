//! Simplification Strategy Trait
//!
//! Defines the interface for function-specific simplification strategies.
//! Each function family can implement custom algebraic rewrite rules.

use crate::core::Expression;

/// Simplification strategy for a specific function or function family
///
/// Implements pattern-based algebraic rewrites (e.g., log(x^n) → n*log(x))
/// Distinct from FunctionProperties which store declarative mathematical data.
pub trait SimplificationStrategy: Send + Sync {
    /// Apply simplification rules to function call
    ///
    /// # Arguments
    ///
    /// * `args` - Function arguments to simplify
    ///
    /// # Returns
    ///
    /// Simplified expression (may be unchanged if no rules apply)
    ///
    /// # Examples
    ///
    /// Common simplification patterns:
    /// - log(1) → 0
    /// - log(x^n) → n*log(x)
    /// - sin(0) → 0
    fn simplify(&self, args: &[Expression]) -> Expression;

    /// Check if simplification strategy applies to given arguments
    ///
    /// Allows early rejection of inapplicable strategies
    fn applies_to(&self, args: &[Expression]) -> bool {
        !args.is_empty()
    }

    /// Get strategy name for debugging
    fn name(&self) -> &str {
        "UnnamedStrategy"
    }
}
