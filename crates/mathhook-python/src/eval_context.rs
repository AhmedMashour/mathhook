//! EvalContext module for MathHook Python bindings
//!
//! Provides the EvalContext class for controlling expression evaluation behavior.

use mathhook_core::core::expression::eval_numeric::EvalContext as CoreEvalContext;
use pyo3::prelude::*;
use std::collections::HashMap;

use crate::expression::PyExpression;

/// Evaluation context for controlling evaluation behavior
///
/// Controls how `evaluate_with_context()` behaves. Provides variable substitutions,
/// numerical evaluation control, and simplification options.
///
/// # Examples
///
/// ```python
/// from mathhook import EvalContext, Expression
///
/// # Symbolic evaluation (no numerical conversion)
/// ctx = EvalContext.symbolic()
/// result = expr.evaluate_with_context(ctx)
///
/// # Numerical evaluation with substitutions
/// ctx = EvalContext.numeric({"x": Expression.integer(3)})
/// result = expr.evaluate_with_context(ctx)
///
/// # Custom configuration
/// ctx = EvalContext(
///     variables={"x": Expression.integer(5)},
///     numeric=True,
///     precision=128,
///     simplify_first=True
/// )
/// ```
#[pyclass]
#[derive(Clone)]
pub struct EvalContext {
    pub(crate) inner: CoreEvalContext,
}

#[pymethods]
impl EvalContext {
    /// Create a new evaluation context
    ///
    /// # Arguments
    ///
    /// * `variables` - Dictionary mapping variable names to expressions (default: empty)
    /// * `numeric` - Whether to perform numerical evaluation (default: True)
    /// * `precision` - Number of bits of precision for numerical operations (default: 53 for f64)
    /// * `simplify_first` - Whether to simplify before evaluation (default: True)
    ///
    /// # Examples
    ///
    /// ```python
    /// from mathhook import EvalContext, Expression
    ///
    /// # Default: numerical evaluation with simplification
    /// ctx = EvalContext()
    ///
    /// # With variable substitutions
    /// ctx = EvalContext(variables={"x": Expression.integer(3)})
    ///
    /// # Custom precision
    /// ctx = EvalContext(precision=128)
    ///
    /// # Symbolic mode (no numerical conversion)
    /// ctx = EvalContext(numeric=False)
    /// ```
    #[new]
    #[pyo3(signature = (variables=None, numeric=true, precision=53, simplify_first=true))]
    pub fn new(
        variables: Option<HashMap<String, PyExpression>>,
        numeric: bool,
        precision: u32,
        simplify_first: bool,
    ) -> Self {
        let core_variables = variables
            .unwrap_or_default()
            .into_iter()
            .map(|(k, v)| (k, v.inner))
            .collect();

        Self {
            inner: CoreEvalContext {
                variables: core_variables,
                numeric,
                precision,
                simplify_first,
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
    /// - Symbolic mode (numeric = False)
    /// - Default precision (53 bits)
    /// - No pre-simplification
    ///
    /// # Examples
    ///
    /// ```python
    /// from mathhook import EvalContext, Expression, Symbol
    ///
    /// x = Symbol('x')
    /// expr = x**2 + 2*x + 1
    ///
    /// ctx = EvalContext.symbolic()
    /// result = expr.evaluate_with_context(ctx)
    /// # Result is still symbolic: x^2 + 2*x + 1
    /// ```
    #[staticmethod]
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
    /// * `variables` - Dictionary mapping variable names to expressions
    ///
    /// # Returns
    ///
    /// Context with:
    /// - Provided variable substitutions
    /// - Numerical mode (numeric = True)
    /// - Default precision (53 bits for f64)
    /// - Pre-simplification enabled (simplify_first = True)
    ///
    /// # Examples
    ///
    /// ```python
    /// from mathhook import EvalContext, Expression, Symbol
    ///
    /// x = Symbol('x')
    /// expr = x**2 + 2*x + 1
    ///
    /// # Evaluate at x = 3
    /// ctx = EvalContext.numeric({"x": Expression.integer(3)})
    /// result = expr.evaluate_with_context(ctx)
    /// # Result: 16 (numerical)
    /// ```
    #[staticmethod]
    pub fn numeric(variables: HashMap<String, PyExpression>) -> Self {
        let core_variables = variables.into_iter().map(|(k, v)| (k, v.inner)).collect();

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
    /// ```python
    /// ctx = EvalContext.symbolic().with_precision(128)
    /// ```
    pub fn with_precision(&mut self, precision: u32) -> Self {
        self.inner.precision = precision;
        self.clone()
    }

    /// Set simplification behavior
    ///
    /// # Arguments
    ///
    /// * `simplify_first` - Whether to simplify before evaluation
    ///
    /// # Returns
    ///
    /// Self with updated simplification setting (for method chaining)
    ///
    /// # Examples
    ///
    /// ```python
    /// ctx = EvalContext().with_simplify_first(False)
    /// ```
    pub fn with_simplify_first(&mut self, simplify_first: bool) -> Self {
        self.inner.simplify_first = simplify_first;
        self.clone()
    }

    /// String representation
    fn __repr__(&self) -> String {
        format!(
            "EvalContext(numeric={}, precision={}, simplify_first={}, variables={})",
            self.inner.numeric,
            self.inner.precision,
            self.inner.simplify_first,
            self.inner.variables.len()
        )
    }

    /// String representation
    fn __str__(&self) -> String {
        self.__repr__()
    }
}
