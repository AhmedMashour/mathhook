//! Simplification Registry
//!
//! Global registry of simplification strategies for all functions.
//! Provides O(1) lookup for function-specific algebraic rewrite rules.

use super::elementary::{
    AbsSimplificationStrategy, ExpSimplificationStrategy, SqrtSimplificationStrategy,
};
use super::logarithmic::{LogarithmSimplificationStrategy, NaturalLogSimplificationStrategy};
use super::special::{FactorialSimplificationStrategy, GammaSimplificationStrategy};
use super::strategy::SimplificationStrategy;
use super::trigonometric::{
    CosSimplificationStrategy, GenericTrigSimplificationStrategy, SinSimplificationStrategy,
    TanSimplificationStrategy,
};
use crate::core::Expression;
use once_cell::sync::Lazy;
use std::collections::HashMap;

/// Global simplification strategy registry
///
/// Lazy initialization ensures zero startup cost while providing
/// universal access to simplification strategies.
pub static SIMPLIFICATION_REGISTRY: Lazy<SimplificationRegistry> =
    Lazy::new(SimplificationRegistry::new);

/// Simplification registry for all functions
///
/// Stores and provides access to function-specific simplification strategies.
/// Separate from FunctionProperties (which stores declarative mathematical data).
pub struct SimplificationRegistry {
    /// Function name → Simplification strategy mapping
    strategies: HashMap<String, Box<dyn SimplificationStrategy>>,
}

impl Default for SimplificationRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl SimplificationRegistry {
    /// Create new simplification registry
    ///
    /// Initializes with all built-in simplification strategies.
    pub fn new() -> Self {
        let mut registry = Self {
            strategies: HashMap::with_capacity(32),
        };

        registry.initialize_logarithmic_strategies();
        registry.initialize_trigonometric_strategies();
        registry.initialize_elementary_strategies();
        registry.initialize_special_strategies();

        registry
    }

    /// Register simplification strategy for function
    ///
    /// # Arguments
    ///
    /// * `name` - Function name (e.g., "log", "sin")
    /// * `strategy` - Simplification strategy implementation
    pub fn register(&mut self, name: &str, strategy: Box<dyn SimplificationStrategy>) {
        self.strategies.insert(name.to_owned(), strategy);
    }

    /// Get simplification strategy for function
    ///
    /// # Arguments
    ///
    /// * `name` - Function name to look up
    ///
    /// # Returns
    ///
    /// Strategy if registered, None otherwise
    #[inline(always)]
    pub fn get_strategy(&self, name: &str) -> Option<&dyn SimplificationStrategy> {
        self.strategies.get(name).map(|s| &**s)
    }

    /// Simplify function call using registered strategy
    ///
    /// # Arguments
    ///
    /// * `name` - Function name
    /// * `args` - Function arguments
    ///
    /// # Returns
    ///
    /// Simplified expression (unchanged if no strategy registered)
    ///
    /// # Examples
    ///
    /// ```
    /// use mathhook_core::algebra::simplification::registry::SIMPLIFICATION_REGISTRY;
    /// use mathhook_core::expr;
    ///
    /// let result = SIMPLIFICATION_REGISTRY.simplify_function("log", &[expr!(1)]);
    /// assert_eq!(result, expr!(0));  // log(1) = 0
    /// ```
    pub fn simplify_function(&self, name: &str, args: &[Expression]) -> Expression {
        if let Some(strategy) = self.get_strategy(name) {
            if strategy.applies_to(args) {
                strategy.simplify(args)
            } else {
                Expression::function(name, args.to_vec())
            }
        } else {
            Expression::function(name, args.to_vec())
        }
    }

    /// Check if function has simplification strategy registered
    #[inline(always)]
    pub fn has_strategy(&self, name: &str) -> bool {
        self.strategies.contains_key(name)
    }

    /// Get count of registered strategies (for debugging)
    pub fn strategy_count(&self) -> usize {
        self.strategies.len()
    }

    /// List all registered function names (for debugging)
    pub fn list_functions(&self) -> Vec<String> {
        self.strategies.keys().cloned().collect()
    }

    fn initialize_logarithmic_strategies(&mut self) {
        self.register("log", Box::new(LogarithmSimplificationStrategy));
        self.register("ln", Box::new(NaturalLogSimplificationStrategy));
    }

    fn initialize_trigonometric_strategies(&mut self) {
        self.register("sin", Box::new(SinSimplificationStrategy));
        self.register("cos", Box::new(CosSimplificationStrategy));
        self.register("tan", Box::new(TanSimplificationStrategy));

        self.register(
            "csc",
            Box::new(GenericTrigSimplificationStrategy::new("csc")),
        );
        self.register(
            "sec",
            Box::new(GenericTrigSimplificationStrategy::new("sec")),
        );
        self.register(
            "cot",
            Box::new(GenericTrigSimplificationStrategy::new("cot")),
        );

        self.register(
            "asin",
            Box::new(GenericTrigSimplificationStrategy::new("asin")),
        );
        self.register(
            "acos",
            Box::new(GenericTrigSimplificationStrategy::new("acos")),
        );
        self.register(
            "atan",
            Box::new(GenericTrigSimplificationStrategy::new("atan")),
        );

        self.register(
            "sinh",
            Box::new(GenericTrigSimplificationStrategy::new("sinh")),
        );
        self.register(
            "cosh",
            Box::new(GenericTrigSimplificationStrategy::new("cosh")),
        );
        self.register(
            "tanh",
            Box::new(GenericTrigSimplificationStrategy::new("tanh")),
        );
    }

    fn initialize_elementary_strategies(&mut self) {
        self.register("sqrt", Box::new(SqrtSimplificationStrategy));
        self.register("abs", Box::new(AbsSimplificationStrategy));
        self.register("exp", Box::new(ExpSimplificationStrategy));
    }

    fn initialize_special_strategies(&mut self) {
        self.register("gamma", Box::new(GammaSimplificationStrategy));
        self.register("factorial", Box::new(FactorialSimplificationStrategy));
    }
}

/// Get global simplification registry
///
/// Provides universal access to simplification strategies throughout MathHook
#[inline(always)]
pub fn get_simplification_registry() -> &'static SimplificationRegistry {
    &SIMPLIFICATION_REGISTRY
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_registry_initialization() {
        let registry = SimplificationRegistry::new();

        // Registry should initialize successfully with all strategies
        assert_eq!(registry.strategy_count(), 19);

        // Should have expected capacity
        assert!(registry.strategies.capacity() >= 32);
    }

    #[test]
    fn test_registry_has_strategies() {
        let registry = SimplificationRegistry::new();

        assert!(registry.has_strategy("log"));
        assert!(registry.has_strategy("ln"));
        assert!(registry.has_strategy("sin"));
        assert!(registry.has_strategy("cos"));
        assert!(registry.has_strategy("tan"));
        assert!(registry.has_strategy("sqrt"));
        assert!(registry.has_strategy("abs"));
        assert!(registry.has_strategy("exp"));
        assert!(registry.has_strategy("gamma"));
        assert!(registry.has_strategy("factorial"));
    }

    #[test]
    fn test_global_registry_access() {
        let _registry = get_simplification_registry();

        let result = SIMPLIFICATION_REGISTRY.simplify_function("unknown", &[]);

        assert!(matches!(result, Expression::Function { .. }));
    }

    #[test]
    fn test_simplify_log_of_one() {
        use crate::expr;

        let result = SIMPLIFICATION_REGISTRY.simplify_function("log", &[expr!(1)]);
        assert_eq!(result, expr!(0));
    }

    #[test]
    fn test_simplify_ln_of_one() {
        use crate::expr;

        let result = SIMPLIFICATION_REGISTRY.simplify_function("ln", &[expr!(1)]);
        assert_eq!(result, expr!(0));
    }

    #[test]
    fn test_simplify_sin_of_zero() {
        use crate::expr;

        let result = SIMPLIFICATION_REGISTRY.simplify_function("sin", &[expr!(0)]);
        assert_eq!(result, expr!(0));
    }

    #[test]
    fn test_simplify_cos_of_zero() {
        use crate::expr;

        let result = SIMPLIFICATION_REGISTRY.simplify_function("cos", &[expr!(0)]);
        assert_eq!(result, expr!(1));
    }

    #[test]
    fn test_simplify_factorial_of_zero() {
        use crate::expr;

        let result = SIMPLIFICATION_REGISTRY.simplify_function("factorial", &[expr!(0)]);
        assert_eq!(result, expr!(1));
    }

    #[test]
    fn test_simplify_factorial_of_five() {
        use crate::expr;

        let result = SIMPLIFICATION_REGISTRY.simplify_function("factorial", &[expr!(5)]);
        assert_eq!(result, expr!(120));
    }
}
