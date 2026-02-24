//! Low-level numerical evaluation trait system
//!
//! This module implements the two-level evaluation architecture following SymPy's proven design:
//!
//! 1. **Low-level `EvalNumeric` trait:** Type-specific numerical evaluation without substitution
//! 2. **High-level `evaluate()` method:** User-facing API with substitution, simplification, and control
//!
//! # Architecture
//!
//! The two-level design separates concerns:
//!
//! - `EvalNumeric::eval_numeric()`: Internal trait for numerical conversion (like SymPy's `_eval_evalf()`)
//! - `Expression::evaluate_with_context()`: Public API with substitution and context control (like SymPy's `evalf()`)
//!
//! This separation enables:
//! - Clear semantics for each expression type
//! - Explicit control over evaluation behavior
//! - Extensibility for custom types
//!
//! # Mathematical Background
//!
//! Numerical evaluation converts symbolic expressions to numerical form while preserving
//! mathematical correctness. For example:
//!
//! - `sin(π/2)` → `1.0` (exact symbolic evaluation)
//! - `sqrt(2)` → `1.4142135623730951` (numerical approximation with precision control)
//! - `x^2` (with x=3) → `9` (after substitution and evaluation)
use crate::core::number::Number;
use crate::core::Expression;
use crate::error::MathError;
use num_bigint::BigInt;
use num_rational::BigRational;
use std::collections::HashMap;

pub trait EvalNumeric {
    /// Evaluate expression to numerical form
    ///
    /// # Arguments
    ///
    /// * `precision` - Number of bits of precision for numerical operations (default: 53 for f64)
    ///
    /// # Returns
    ///
    /// Expression in numerical form (may contain Number, Complex, Matrix of numbers, etc.)
    ///
    /// # Errors
    ///
    /// Returns `MathError` for:
    /// - Domain violations (sqrt of negative, log of zero, etc.)
    /// - Undefined operations (0/0, inf-inf, etc.)
    /// - Numerical overflow/underflow
    ///
    /// # Implementation Requirements
    ///
    /// Implementations MUST:
    /// 1. Handle domain restrictions correctly (return error for invalid inputs)
    /// 2. Preserve mathematical correctness (exact evaluation when possible)
    /// 3. Use specified precision for floating-point operations
    /// 4. NOT perform variable substitution (that's `evaluate_with_context()`'s job)
    fn eval_numeric(&self, precision: u32) -> Result<Expression, MathError>;
}

/// Evaluation context
///
/// Controls how `Expression::evaluate_with_context()` behaves. Provides variable substitutions,
/// numerical evaluation control, and simplification options.
///
/// This mirrors SymPy's `evalf(subs={...}, ...)` high-level API.
///
/// # Two-Level Architecture
///
/// The context enables separation of concerns:
///
/// 1. **Variable substitution:** Replace symbols with values before evaluation
/// 2. **Simplification control:** Optionally simplify symbolically first
/// 3. **Numerical evaluation:** Convert to numerical form if requested
///
/// # Examples
///
/// ```rust
/// use mathhook_core::{expr, symbol};
/// use mathhook_core::core::expression::eval_numeric::EvalContext;
/// use std::collections::HashMap;
///
/// // Symbolic evaluation (no numerical conversion)
/// let ctx = EvalContext::symbolic();
/// assert!(!ctx.numeric);
/// assert!(ctx.variables.is_empty());
///
/// // Numerical evaluation with substitutions
/// let mut vars = HashMap::new();
/// vars.insert("x".to_string(), expr!(5));
/// let ctx = EvalContext::numeric(vars);
/// assert!(ctx.numeric);
/// assert_eq!(ctx.variables.len(), 1);
///
/// // Custom precision
/// let ctx = EvalContext::symbolic().with_precision(128);
/// assert_eq!(ctx.precision, 128);
/// ```
#[derive(Debug, Clone)]
pub struct EvalContext {
    /// Variable substitutions (symbol name → value)
    ///
    /// Before evaluation, all symbols matching these names will be replaced
    /// with the provided expressions. This enables parameterized evaluation.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use std::collections::HashMap;
    /// use mathhook_core::{expr, Expression};
    ///
    /// let mut vars = HashMap::new();
    /// vars.insert("x".to_string(), expr!(3));
    /// vars.insert("y".to_string(), expr!(4));
    /// // Now evaluating "x + y" will substitute → "3 + 4" → "7"
    /// ```
    pub variables: HashMap<String, Expression>,

    /// Whether to perform numerical evaluation (evalf-style)
    ///
    /// - `true`: Convert to numerical form using `eval_numeric()`
    /// - `false`: Keep symbolic form (only substitute variables)
    pub numeric: bool,

    /// Precision for numerical operations (bits)
    ///
    /// Controls accuracy of floating-point operations:
    /// - 53 bits: f64 precision (default)
    /// - 64 bits: Extended precision
    /// - 128+ bits: Arbitrary precision (future)
    ///
    /// Note: Current implementation uses f64, so precision >53 has no effect yet.
    /// Future versions will support arbitrary precision via `rug` or `mpc`.
    pub precision: u32,

    /// Whether to simplify symbolically before numerical evaluation
    ///
    /// - `true`: Call `simplify()` before `eval_numeric()` (recommended)
    /// - `false`: Evaluate directly without simplification
    ///
    /// Simplification often improves numerical stability by reducing expression complexity.
    pub simplify_first: bool,
}

impl EvalContext {
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
    /// ```rust
    /// use mathhook_core::{expr, symbol};
    /// use mathhook_core::core::expression::eval_numeric::EvalContext;
    ///
    /// let x = symbol!(x);
    /// let e = expr!((x ^ 2) + (2*x) + 1);
    ///
    /// let ctx = EvalContext::symbolic();
    /// let result = e.evaluate_with_context(&ctx).unwrap();
    /// // Result is still symbolic: x^2 + 2*x + 1
    /// ```
    pub fn symbolic() -> Self {
        Self {
            variables: HashMap::new(),
            numeric: false,
            precision: 53,
            simplify_first: false,
        }
    }

    /// Create context for numerical evaluation with substitutions
    ///
    /// Returns a context that substitutes variables and converts to numerical form.
    /// Simplification is enabled by default for numerical stability.
    ///
    /// # Arguments
    ///
    /// * `variables` - Map from symbol name to replacement expression
    ///
    /// # Returns
    ///
    /// Context with:
    /// - Provided variable substitutions
    /// - Numerical mode (numeric = true)
    /// - Default precision (53 bits for f64)
    /// - Pre-simplification enabled (simplify_first = true)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{expr, symbol};
    /// use mathhook_core::core::expression::eval_numeric::EvalContext;
    /// use std::collections::HashMap;
    ///
    /// let x = symbol!(x);
    /// let e = expr!((x ^ 2) + (2*x) + 1);
    ///
    /// let mut vars = HashMap::new();
    /// vars.insert("x".to_string(), expr!(3));
    ///
    /// let ctx = EvalContext::numeric(vars);
    /// let result = e.evaluate_with_context(&ctx).unwrap();
    /// // Result is numerical: 16 (= 3^2 + 2*3 + 1)
    /// ```
    pub fn numeric(variables: HashMap<String, Expression>) -> Self {
        Self {
            variables,
            numeric: true,
            precision: 53,
            simplify_first: true,
        }
    }

    /// Set precision for numerical operations (bits)
    ///
    /// Consumes self and returns a new context with the specified precision.
    ///
    /// # Arguments
    ///
    /// * `precision` - Number of bits of precision (53 for f64, 128+ for arbitrary precision)
    ///
    /// # Returns
    ///
    /// New context with updated precision
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::core::expression::eval_numeric::EvalContext;
    ///
    /// let ctx = EvalContext::symbolic().with_precision(128);
    /// assert_eq!(ctx.precision, 128);
    /// ```
    pub fn with_precision(mut self, precision: u32) -> Self {
        self.precision = precision;
        self
    }

    /// Control whether to simplify symbolically before numerical evaluation
    ///
    /// Consumes self and returns a new context with the specified simplification flag.
    ///
    /// # Arguments
    ///
    /// * `simplify` - Whether to call `simplify()` before `eval_numeric()`
    ///
    /// # Returns
    ///
    /// New context with updated simplification setting
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::core::expression::eval_numeric::EvalContext;
    ///
    /// // Disable simplification for performance
    /// let ctx = EvalContext::symbolic().with_simplify(false);
    /// assert!(!ctx.simplify_first);
    ///
    /// // Enable simplification for numerical stability
    /// let ctx = EvalContext::symbolic().with_simplify(true);
    /// assert!(ctx.simplify_first);
    /// ```
    pub fn with_simplify(mut self, simplify: bool) -> Self {
        self.simplify_first = simplify;
        self
    }
}

impl Default for EvalContext {
    /// Default context is symbolic (no numerical evaluation)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::core::expression::eval_numeric::EvalContext;
    ///
    /// let ctx = EvalContext::default();
    /// assert!(!ctx.numeric);
    /// assert!(ctx.variables.is_empty());
    /// ```
    fn default() -> Self {
        Self::symbolic()
    }
}

fn is_number_negative(n: &Number) -> bool {
    match n {
        Number::Integer(i) => *i < 0,
        Number::Float(f) => *f < 0.0,
        Number::BigInteger(bi) => **bi < BigInt::from(0),
        Number::Rational(r) => **r < BigRational::new(BigInt::from(0), BigInt::from(1)),
    }
}

impl EvalNumeric for Expression {
    fn eval_numeric(&self, _precision: u32) -> Result<Expression, MathError> {
        match self {
            Expression::Number(_) => Ok(self.clone()),
            Expression::Symbol(_) => Ok(self.clone()),

            Expression::Constant(c) => {
                use crate::core::MathConstant;
                match c {
                    MathConstant::Pi => Ok(Expression::float(std::f64::consts::PI)),
                    MathConstant::E => Ok(Expression::float(std::f64::consts::E)),
                    MathConstant::I => Ok(self.clone()),
                    MathConstant::Infinity => Ok(self.clone()),
                    MathConstant::NegativeInfinity => Ok(self.clone()),
                    MathConstant::Undefined => Ok(self.clone()),
                    MathConstant::GoldenRatio => {
                        Ok(Expression::float(MathConstant::GoldenRatio.to_f64()))
                    }
                    MathConstant::EulerGamma => {
                        Ok(Expression::float(MathConstant::EulerGamma.to_f64()))
                    }
                    MathConstant::TribonacciConstant => {
                        Ok(Expression::float(MathConstant::TribonacciConstant.to_f64()))
                    }
                }
            }

            Expression::Add(terms) => {
                let evaluated: Result<Vec<_>, _> =
                    terms.iter().map(|t| t.eval_numeric(_precision)).collect();
                Ok(Expression::add(evaluated?))
            }

            Expression::Mul(factors) => {
                let evaluated: Result<Vec<_>, _> =
                    factors.iter().map(|f| f.eval_numeric(_precision)).collect();
                Ok(Expression::mul(evaluated?))
            }

            Expression::Pow(base, exp) => {
                let base_eval = base.eval_numeric(_precision)?;
                let exp_eval = exp.eval_numeric(_precision)?;

                if base_eval.is_zero() {
                    if let Expression::Number(n) = &exp_eval {
                        if is_number_negative(n) {
                            return Err(MathError::DivisionByZero);
                        }
                    }
                }

                // Compute numerically when both operands are numeric
                match (&base_eval, &exp_eval) {
                    (
                        Expression::Number(Number::Float(b)),
                        Expression::Number(Number::Integer(n)),
                    ) => {
                        if let Ok(exp_i32) = i32::try_from(*n) {
                            let result = b.powi(exp_i32);
                            if result.is_finite() {
                                return Ok(Expression::float(result));
                            }
                        }
                        let result = b.powf(*n as f64);
                        if result.is_finite() {
                            return Ok(Expression::float(result));
                        }
                    }
                    (
                        Expression::Number(Number::Float(b)),
                        Expression::Number(Number::Float(e)),
                    ) => {
                        let result = b.powf(*e);
                        if result.is_finite() {
                            return Ok(Expression::float(result));
                        }
                    }
                    (
                        Expression::Number(Number::Integer(b)),
                        Expression::Number(Number::Float(e)),
                    ) => {
                        let result = (*b as f64).powf(*e);
                        if result.is_finite() {
                            return Ok(Expression::float(result));
                        }
                    }
                    _ => {}
                }

                Ok(Expression::pow(base_eval, exp_eval))
            }

            Expression::Function { name, args } => {
                let eval_args = args
                    .iter()
                    .map(|arg| arg.eval_numeric(_precision))
                    .collect::<Result<Vec<_>, _>>()?;

                if let Some(result) =
                    super::evaluation::evaluate_function_dispatch(name, &eval_args)
                {
                    return Ok(result);
                }

                Ok(Expression::function(name.clone(), eval_args))
            }

            Expression::Matrix(matrix) => {
                let (rows, cols) = matrix.dimensions();
                let mut new_rows = Vec::with_capacity(rows);

                for i in 0..rows {
                    let mut row = Vec::with_capacity(cols);
                    for j in 0..cols {
                        let element = matrix.get_element(i, j);
                        row.push(element.eval_numeric(_precision)?);
                    }
                    new_rows.push(row);
                }

                Ok(Expression::matrix(new_rows))
            }

            Expression::Set(elements) => {
                let evaluated: Result<Vec<_>, _> = elements
                    .iter()
                    .map(|e| e.eval_numeric(_precision))
                    .collect();
                Ok(Expression::set(evaluated?))
            }

            Expression::Complex(data) => {
                let real_eval = data.real.eval_numeric(_precision)?;
                let imag_eval = data.imag.eval_numeric(_precision)?;
                Ok(Expression::complex(real_eval, imag_eval))
            }

            Expression::Interval(interval) => {
                let start_eval = interval.start.eval_numeric(_precision)?;
                let end_eval = interval.end.eval_numeric(_precision)?;

                Ok(Expression::interval(
                    start_eval,
                    end_eval,
                    interval.start_inclusive,
                    interval.end_inclusive,
                ))
            }

            Expression::Piecewise(data) => {
                let mut new_pieces = Vec::with_capacity(data.pieces.len());

                for (expr, cond) in &data.pieces {
                    let expr_eval = expr.eval_numeric(_precision)?;
                    new_pieces.push((expr_eval, cond.clone()));
                }

                let default_eval = if let Some(ref default) = data.default {
                    Some(default.eval_numeric(_precision)?)
                } else {
                    None
                };

                Ok(Expression::piecewise(new_pieces, default_eval))
            }

            Expression::Relation(rel) => {
                let lhs_eval = rel.left.eval_numeric(_precision)?;
                let rhs_eval = rel.right.eval_numeric(_precision)?;

                Ok(Expression::relation(lhs_eval, rhs_eval, rel.relation_type))
            }

            Expression::Calculus(_) => Ok(self.clone()),

            Expression::MethodCall(_) => Ok(self.clone()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eval_context_symbolic() {
        let ctx = EvalContext::symbolic();
        assert!(!ctx.numeric);
        assert!(ctx.variables.is_empty());
        assert_eq!(ctx.precision, 53);
        assert!(!ctx.simplify_first);
    }

    #[test]
    fn test_eval_context_numeric() {
        let mut vars = HashMap::new();
        vars.insert("x".to_string(), Expression::integer(5));
        let ctx = EvalContext::numeric(vars);

        assert!(ctx.numeric);
        assert_eq!(ctx.variables.len(), 1);
        assert_eq!(ctx.precision, 53);
        assert!(ctx.simplify_first);
    }

    #[test]
    fn test_eval_context_with_precision() {
        let ctx = EvalContext::symbolic().with_precision(128);
        assert_eq!(ctx.precision, 128);
    }

    #[test]
    fn test_eval_context_with_simplify() {
        let ctx = EvalContext::symbolic().with_simplify(true);
        assert!(ctx.simplify_first);

        let ctx = EvalContext::symbolic().with_simplify(false);
        assert!(!ctx.simplify_first);
    }

    #[test]
    fn test_eval_context_default() {
        let ctx = EvalContext::default();
        assert!(!ctx.numeric);
        assert!(ctx.variables.is_empty());
    }

    #[test]
    fn test_eval_context_chaining() {
        let mut vars = HashMap::new();
        vars.insert("x".to_string(), Expression::integer(3));

        let ctx = EvalContext::numeric(vars)
            .with_precision(128)
            .with_simplify(false);

        assert!(ctx.numeric);
        assert_eq!(ctx.variables.len(), 1);
        assert_eq!(ctx.precision, 128);
        assert!(!ctx.simplify_first);
    }
}
