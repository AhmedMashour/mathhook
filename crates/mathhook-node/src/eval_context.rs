//! EvalContext module for MathHook Node.js bindings
//!
//! Provides the EvalContext class for controlling expression evaluation behavior.

use mathhook_core::core::expression::eval_numeric::EvalContext as CoreEvalContext;
use napi_derive::napi;
use std::collections::HashMap;

use crate::expression::JsExpression;

/// Evaluation context for controlling evaluation behavior
///
/// Controls how `evaluateWithContext()` behaves. Provides variable substitutions,
/// numerical evaluation control, and simplification options.
///
/// # Examples
///
/// ```typescript
/// import { EvalContext, Expression } from 'mathhook';
///
/// // Symbolic evaluation (no numerical conversion)
/// const ctx = EvalContext.symbolic();
/// const result = expr.evaluateWithContext(ctx);
///
/// // Numerical evaluation with substitutions
/// const ctx = EvalContext.numeric({ x: Expression.integer(3) });
/// const result = expr.evaluateWithContext(ctx);
///
/// // Custom configuration
/// const ctx = new EvalContext({
///   variables: { x: Expression.integer(5) },
///   numeric: true,
///   precision: 128,
///   simplifyFirst: true
/// });
/// ```
#[napi]
pub struct EvalContext {
    pub(crate) inner: CoreEvalContext,
}

/// Options for creating an EvalContext
#[napi(object)]
pub struct EvalContextOptions {
    /// Whether to perform numerical evaluation (default: true)
    pub numeric: Option<bool>,
    /// Number of bits of precision for numerical operations (default: 53 for f64)
    pub precision: Option<u32>,
    /// Whether to simplify before evaluation (default: true)
    pub simplify_first: Option<bool>,
}

#[napi]
impl EvalContext {
    /// Create a new evaluation context
    ///
    /// # Arguments
    ///
    /// * `options` - Configuration options for evaluation
    ///   - `numeric` - Whether to perform numerical evaluation (default: true)
    ///   - `precision` - Number of bits of precision for numerical operations (default: 53 for f64)
    ///   - `simplifyFirst` - Whether to simplify before evaluation (default: true)
    ///
    /// # Examples
    ///
    /// ```typescript
    /// import { EvalContext, Expression } from 'mathhook';
    ///
    /// // Default: numerical evaluation with simplification
    /// const ctx = new EvalContext({});
    ///
    /// // Custom precision
    /// const ctx = new EvalContext({ precision: 128 });
    ///
    /// // Symbolic mode (no numerical conversion)
    /// const ctx = new EvalContext({ numeric: false });
    /// ```
    #[napi(constructor)]
    pub fn new(options: Option<EvalContextOptions>) -> Self {
        let opts = options.unwrap_or(EvalContextOptions {
            numeric: Some(true),
            precision: Some(53),
            simplify_first: Some(true),
        });

        Self {
            inner: CoreEvalContext {
                variables: HashMap::new(),
                numeric: opts.numeric.unwrap_or(true),
                precision: opts.precision.unwrap_or(53),
                simplify_first: opts.simplify_first.unwrap_or(true),
            },
        }
    }

    /// Create context for symbolic evaluation (no numerical conversion)
    ///
    /// Returns a context that performs variable substitution but keeps expressions
    /// in symbolic form. No numerical evaluation is performed.
    ///
    /// # Returns
    ///
    /// Context with:
    /// - No variable substitutions
    /// - Symbolic mode (numeric = false)
    /// - Default precision (53 bits)
    /// - No pre-simplification
    ///
    /// # Examples
    ///
    /// ```typescript
    /// import { EvalContext, Expression, Symbol } from 'mathhook';
    ///
    /// const x = new Symbol('x');
    /// const expr = x.pow(Expression.integer(2)).add(Expression.integer(1));
    ///
    /// const ctx = EvalContext.symbolic();
    /// const result = expr.evaluateWithContext(ctx);
    /// // Result is still symbolic: x^2 + 1
    /// ```
    #[napi(factory)]
    pub fn symbolic() -> Self {
        Self {
            inner: CoreEvalContext::symbolic(),
        }
    }

    /// Create context for numerical evaluation with substitutions
    ///
    /// Returns a context that substitutes variables and converts to numerical form.
    /// Simplification is enabled by default for numerical stability.
    ///
    /// # Arguments
    ///
    /// * `variables` - Array of [name, expression] pairs for variable substitution
    ///
    /// # Returns
    ///
    /// Context with:
    /// - Provided variable substitutions
    /// - Numerical mode (numeric = true)
    /// - Default precision (53 bits for f64)
    /// - Pre-simplification enabled (simplifyFirst = true)
    ///
    /// # Examples
    ///
    /// ```typescript
    /// import { EvalContext, Expression, Symbol } from 'mathhook';
    ///
    /// const x = new Symbol('x');
    /// const expr = x.pow(Expression.integer(2)).add(Expression.integer(1));
    ///
    /// // Evaluate at x = 3
    /// const ctx = EvalContext.numeric([["x", Expression.integer(3)]]);
    /// const result = expr.evaluateWithContext(ctx);
    /// // Result: 10 (numerical)
    /// ```
    #[napi(factory)]
    pub fn numeric(variables: Vec<(String, &JsExpression)>) -> Self {
        let core_variables = variables
            .into_iter()
            .map(|(k, v)| (k, v.inner.clone()))
            .collect();

        Self {
            inner: CoreEvalContext::numeric(core_variables),
        }
    }

    /// Set precision for numerical operations
    ///
    /// # Arguments
    ///
    /// * `precision` - Number of bits of precision (53 for f64, higher for future arbitrary precision)
    ///
    /// # Returns
    ///
    /// Self with updated precision (for method chaining)
    ///
    /// # Examples
    ///
    /// ```typescript
    /// const ctx = EvalContext.symbolic().withPrecision(128);
    /// ```
    #[napi]
    pub fn with_precision(&mut self, precision: u32) -> Self {
        self.inner.precision = precision;
        Self {
            inner: self.inner.clone(),
        }
    }

    /// Set simplification behavior
    ///
    /// # Arguments
    ///
    /// * `simplifyFirst` - Whether to simplify before evaluation
    ///
    /// # Returns
    ///
    /// Self with updated simplification setting (for method chaining)
    ///
    /// # Examples
    ///
    /// ```typescript
    /// const ctx = new EvalContext({}).withSimplifyFirst(false);
    /// ```
    #[napi]
    pub fn with_simplify_first(&mut self, simplify_first: bool) -> Self {
        self.inner.simplify_first = simplify_first;
        Self {
            inner: self.inner.clone(),
        }
    }
}
