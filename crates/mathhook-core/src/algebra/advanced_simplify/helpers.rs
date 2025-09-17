//! Helper functions for advanced simplification

use crate::algebra::simplification::registry::SIMPLIFICATION_REGISTRY;
use crate::core::Expression;
use std::slice::from_ref;

impl Expression {
    /// Simplify factorial function using registry
    pub(super) fn compute_factorial(&self, arg: &Expression) -> Expression {
        SIMPLIFICATION_REGISTRY.simplify_function("factorial", from_ref(arg))
    }

    /// Simplify logarithm functions using registry
    pub(super) fn simplify_log_function(&self, args: &[Expression]) -> Expression {
        SIMPLIFICATION_REGISTRY.simplify_function("log", args)
    }

    /// Simplify natural logarithm functions using registry
    pub(super) fn simplify_ln_function(&self, args: &[Expression]) -> Expression {
        SIMPLIFICATION_REGISTRY.simplify_function("ln", args)
    }

    /// Check if a function name is trigonometric
    pub(super) fn is_trig_function(&self, name: &str) -> bool {
        matches!(
            name,
            "sin"
                | "cos"
                | "tan"
                | "csc"
                | "sec"
                | "cot"
                | "asin"
                | "acos"
                | "atan"
                | "sinh"
                | "cosh"
                | "tanh"
        )
    }

    /// Simplify trigonometric functions using registry
    pub(super) fn simplify_trig_function(&self, name: &str, args: &[Expression]) -> Expression {
        SIMPLIFICATION_REGISTRY.simplify_function(name, args)
    }

    /// Simplify square root function using registry
    pub(super) fn simplify_sqrt(&self, args: &[Expression]) -> Expression {
        SIMPLIFICATION_REGISTRY.simplify_function("sqrt", args)
    }

    /// Simplify absolute value function using registry
    pub(super) fn simplify_abs(&self, args: &[Expression]) -> Expression {
        SIMPLIFICATION_REGISTRY.simplify_function("abs", args)
    }

    /// Simplify exponential function using registry
    pub(super) fn simplify_exp(&self, args: &[Expression]) -> Expression {
        SIMPLIFICATION_REGISTRY.simplify_function("exp", args)
    }

    /// Simplify gamma function using registry
    pub(super) fn simplify_gamma(&self, args: &[Expression]) -> Expression {
        SIMPLIFICATION_REGISTRY.simplify_function("gamma", args)
    }

    /// Create factorial expression
    pub fn factorial(arg: Expression) -> Expression {
        Expression::function("factorial", vec![arg])
    }

    /// Create natural logarithm expression
    pub fn ln(arg: Expression) -> Expression {
        Expression::function("ln", vec![arg])
    }
}
