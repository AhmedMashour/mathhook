//! Core evaluation methods for expressions
//!
//! Contains the primary evaluation logic with domain checking:
//! - `evaluate()` - main evaluation with domain validation
//! - `evaluate_with_context()` - evaluation with variable substitution
//! - `evaluate_to_f64()` - conversion to f64
//! - `try_extract_numeric_value()` - helper for numeric extraction

use super::super::eval_numeric::EvalContext;
use super::super::Expression;
use crate::core::constants::EPSILON;
use crate::core::Number;
use crate::simplify::Simplify;
use num_traits::ToPrimitive;

impl Expression {
    /// Evaluate expression with domain checking
    ///
    /// Computes numerical values from expressions while validating mathematical
    /// domain constraints. Returns `Result<Expression, MathError>` to handle
    /// domain violations gracefully.
    ///
    /// # Evaluation vs Simplification
    ///
    /// **Use `evaluate()` when:** You need numerical results with domain validation
    /// **Use `simplify()` when:** You need algebraic reduction without domain checking
    /// **Use `evaluate_with_context()` when:** You need variable substitution + computation
    ///
    /// # Domain Constraints Checked
    ///
    /// - `sqrt(x)`: Requires x >= 0 in real domain
    /// - `log(x)`: Requires x > 0 (pole at 0)
    /// - `tan(x)`: Has poles at π/2 + nπ
    /// - `arcsin(x)`, `arccos(x)`: Require |x| <= 1 in real domain
    /// - `csc(x)`, `sec(x)`, `cot(x)`: Have poles where sin/cos/tan = 0
    /// - Division by zero: Checked in `x/y` and `x^(-n)` for n > 0
    ///
    /// # Returns
    ///
    /// - `Ok(Expression)`: Evaluated result (numerical or symbolic if can't evaluate)
    /// - `Err(MathError::DomainError)`: Domain constraint violated
    /// - `Err(MathError::DivisionByZero)`: Division by zero detected
    ///
    /// # Examples
    ///
    /// ## Successful Evaluation
    ///
    /// ```rust
    /// use mathhook_core::{Expression, MathError};
    ///
    /// // Constants evaluate to numbers
    /// let sum = Expression::add(vec![Expression::integer(2), Expression::integer(3)]);
    /// assert_eq!(sum.evaluate().unwrap(), Expression::integer(5));
    ///
    /// let product = Expression::mul(vec![Expression::integer(2), Expression::integer(3), Expression::integer(4)]);
    /// assert_eq!(product.evaluate().unwrap(), Expression::integer(24));
    ///
    /// // Special values
    /// let sin_zero = Expression::function("sin".to_string(), vec![Expression::integer(0)]);
    /// assert_eq!(sin_zero.evaluate().unwrap(), Expression::integer(0));
    /// ```
    ///
    /// ## Domain Errors
    ///
    /// ```rust,ignore
    /// use mathhook_core::{expr, Expression, MathError};
    ///
    /// // sqrt requires non-negative input
    /// let sqrt_neg = Expression::function("sqrt".to_string(), vec![Expression::integer(-1)]);
    /// assert!(matches!(
    ///     sqrt_neg.evaluate(),
    ///     Err(MathError::Pole { .. })
    /// ));
    ///
    /// // log has pole at 0
    /// assert!(matches!(
    ///     expr!(log(0)).evaluate(),
    ///     Err(MathError::Pole { .. })
    /// ));
    ///
    /// // Division by zero
    /// assert!(matches!(
    ///     expr!(1 / 0).evaluate(),
    ///     Err(MathError::DivisionByZero)
    /// ));
    /// ```
    ///
    /// ## Symbolic Results (No Variables to Substitute)
    ///
    /// ```rust
    /// use mathhook_core::{expr, symbol};
    /// use mathhook_core::simplify::Simplify;
    ///
    /// let x = symbol!(x);
    ///
    /// // Can't evaluate without variable value - returns simplified symbolic
    /// let result = expr!(x + 1).evaluate().unwrap();
    /// assert_eq!(result, expr!(x + 1).simplify());
    ///
    /// // For variable substitution, use evaluate_with_context() instead
    /// ```
    ///
    /// ## Handling Errors
    ///
    /// ```rust
    /// use mathhook_core::{Expression, MathError};
    ///
    /// let sqrt_neg = Expression::function("sqrt".to_string(), vec![Expression::integer(-1)]);
    /// match sqrt_neg.evaluate() {
    ///     Ok(result) => println!("Result: {}", result),
    ///     Err(MathError::DomainError { operation, value, reason }) => {
    ///         eprintln!("Domain error in {}: {} ({})", operation, value, reason);
    ///     }
    ///     Err(e) => eprintln!("Other error: {:?}", e),
    /// }
    /// ```
    pub fn evaluate(&self) -> Result<Expression, crate::MathError> {
        use crate::MathError;
        use std::f64::consts::PI;

        match self {
            Expression::Number(_) => Ok(self.simplify()),

            Expression::Symbol(_) => Ok(self.simplify()),

            Expression::Constant(_) => Ok(self.simplify()),

            Expression::Add(terms) => {
                let evaluated_terms: Result<Vec<Expression>, MathError> =
                    terms.iter().map(|t| t.evaluate()).collect();
                Ok(Expression::add(evaluated_terms?).simplify())
            }

            Expression::Mul(factors) => {
                let evaluated_factors: Result<Vec<Expression>, MathError> =
                    factors.iter().map(|f| f.evaluate()).collect();
                Ok(Expression::mul(evaluated_factors?).simplify())
            }

            Expression::Pow(base, exp) => {
                let eval_base = base.evaluate()?;
                let eval_exp = exp.evaluate()?;

                if eval_base.is_zero_fast() {
                    if let Some(exp_value) = Self::try_extract_numeric_value(&eval_exp) {
                        if exp_value < 0.0 {
                            return Err(MathError::DivisionByZero);
                        }
                    }
                }

                Ok(Expression::pow(eval_base, eval_exp).simplify())
            }

            Expression::Function { name, args } => {
                if name.as_ref() == "undefined" {
                    return Err(MathError::DivisionByZero);
                }

                let evaluated_args: Result<Vec<Expression>, MathError> =
                    args.iter().map(|arg| arg.evaluate()).collect();
                let evaluated_args = evaluated_args?;

                match name.as_ref() {
                    "sqrt" => {
                        if let Some(arg) = evaluated_args.first() {
                            if let Some(value) = Self::try_extract_numeric_value(arg) {
                                if value < 0.0 {
                                    return Err(MathError::DomainError {
                                        operation: "sqrt".to_owned(),
                                        value: arg.clone(),
                                        reason: "sqrt requires non-negative input in real domain"
                                            .to_owned(),
                                    });
                                }
                            }
                        }
                    }
                    "log" | "ln" => {
                        if let Some(arg) = evaluated_args.first() {
                            if let Some(value) = Self::try_extract_numeric_value(arg) {
                                if value.abs() < EPSILON {
                                    return Err(MathError::Pole {
                                        function: name.to_string(),
                                        at: arg.clone(),
                                    });
                                } else if value < 0.0 {
                                    return Err(MathError::BranchCut {
                                        function: name.to_string(),
                                        value: arg.clone(),
                                    });
                                }
                            }
                        }
                    }
                    "tan" => {
                        if let Some(arg) = evaluated_args.first() {
                            if let Some(value) = Self::try_extract_numeric_value(arg) {
                                let normalized = value.rem_euclid(PI);
                                if (normalized - PI / 2.0).abs() < 1e-10 {
                                    return Err(MathError::Pole {
                                        function: "tan".to_owned(),
                                        at: arg.clone(),
                                    });
                                }
                            }
                        }
                    }
                    "arcsin" | "asin" => {
                        if let Some(arg) = evaluated_args.first() {
                            if let Some(value) = Self::try_extract_numeric_value(arg) {
                                if !(-1.0..=1.0).contains(&value) {
                                    return Err(MathError::DomainError {
                                        operation: "arcsin".to_owned(),
                                        value: arg.clone(),
                                        reason: "arcsin requires input in [-1, 1] in real domain"
                                            .to_owned(),
                                    });
                                }
                            }
                        }
                    }
                    "arccos" | "acos" => {
                        if let Some(arg) = evaluated_args.first() {
                            if let Some(value) = Self::try_extract_numeric_value(arg) {
                                if !(-1.0..=1.0).contains(&value) {
                                    return Err(MathError::DomainError {
                                        operation: "arccos".to_owned(),
                                        value: arg.clone(),
                                        reason: "arccos requires input in [-1, 1] in real domain"
                                            .to_owned(),
                                    });
                                }
                            }
                        }
                    }
                    "csc" => {
                        if let Some(arg) = evaluated_args.first() {
                            if let Some(value) = Self::try_extract_numeric_value(arg) {
                                let normalized = value.rem_euclid(PI);
                                if normalized.abs() < 1e-10 {
                                    return Err(MathError::Pole {
                                        function: "csc".to_owned(),
                                        at: arg.clone(),
                                    });
                                }
                            }
                        }
                    }
                    "sec" => {
                        if let Some(arg) = evaluated_args.first() {
                            if let Some(value) = Self::try_extract_numeric_value(arg) {
                                let normalized = value.rem_euclid(PI);
                                if (normalized - PI / 2.0).abs() < 1e-10 {
                                    return Err(MathError::Pole {
                                        function: "sec".to_owned(),
                                        at: arg.clone(),
                                    });
                                }
                            }
                        }
                    }
                    _ => {}
                }

                if let Some(result) =
                    super::dispatch::evaluate_function_dispatch(name, &evaluated_args)
                {
                    Ok(result)
                } else {
                    Ok(Expression::function(name.clone(), evaluated_args).simplify())
                }
            }

            Expression::Matrix(_) => Ok(self.simplify()),

            Expression::Set(_) => Ok(self.simplify()),

            Expression::Complex(_) => Ok(self.simplify()),

            Expression::Interval(_) => Ok(self.simplify()),

            Expression::Piecewise(_) => Ok(self.simplify()),

            Expression::Relation(_) => Ok(self.simplify()),

            Expression::Calculus(_) => Ok(self.simplify()),

            Expression::MethodCall(_) => Ok(self.simplify()),
        }
    }

    /// Extract numeric value from expression as f64 for domain checking
    pub(crate) fn try_extract_numeric_value(expr: &Expression) -> Option<f64> {
        match expr {
            Expression::Number(Number::Integer(i)) => Some(*i as f64),
            Expression::Number(Number::Float(f)) => Some(*f),
            Expression::Number(Number::Rational(r)) => {
                let num_float = r.numer().to_f64()?;
                let denom_float = r.denom().to_f64()?;
                Some(num_float / denom_float)
            }
            Expression::Number(Number::BigInteger(bi)) => bi.to_f64(),
            _ => None,
        }
    }

    /// High-level evaluation with context
    ///
    /// This is the PRIMARY user-facing evaluation method following SymPy's two-level architecture.
    /// It handles:
    /// 1. Variable substitution
    /// 2. Optional symbolic simplification
    /// 3. Optional numerical evaluation
    ///
    /// This mirrors SymPy's `evalf(subs={...}, ...)` high-level API.
    ///
    /// # Arguments
    ///
    /// * `context` - Evaluation context (variables, numeric mode, precision, etc.)
    ///
    /// # Returns
    ///
    /// Evaluated expression
    ///
    /// # Errors
    ///
    /// Returns `MathError` for domain violations, undefined operations, etc.
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// use mathhook_core::{expr, symbol};
    /// use mathhook_core::core::expression::eval_numeric::EvalContext;
    /// use std::collections::HashMap;
    ///
    /// // Symbolic evaluation (no substitution)
    /// let x = symbol!(x);
    /// let f = expr!(x ^ 2);
    /// let result = f.evaluate_with_context(&EvalContext::symbolic()).unwrap();
    /// assert_eq!(result, expr!(x ^ 2)); // Unchanged
    ///
    /// // Numerical evaluation with substitution
    /// let mut vars = HashMap::new();
    /// vars.insert("x".to_string(), expr!(3));
    /// let ctx = EvalContext::numeric(vars);
    /// let result = f.evaluate_with_context(&ctx).unwrap();
    /// assert_eq!(result, expr!(9));
    /// ```
    pub fn evaluate_with_context(
        &self,
        context: &EvalContext,
    ) -> Result<Expression, crate::MathError> {
        let substituted = if context.variables.is_empty() {
            self.clone()
        } else {
            self.substitute(&context.variables)
        };

        let simplified = if context.simplify_first {
            substituted.simplify()
        } else {
            substituted
        };

        if context.numeric {
            use crate::core::expression::eval_numeric::EvalNumeric;
            simplified.eval_numeric(context.precision)
        } else {
            Ok(simplified)
        }
    }

    /// Convert evaluated expression to f64
    ///
    /// First evaluates the expression, then attempts to convert the result to f64.
    /// Returns error if the result is non-numerical (symbolic).
    ///
    /// # Returns
    ///
    /// f64 value if expression evaluates to a number
    ///
    /// # Errors
    ///
    /// Returns `MathError::NonNumericalResult` if evaluation produces symbolic expression
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// use mathhook_core::{expr, symbol};
    ///
    /// // Numerical expression
    /// let e = expr!(2 + 3);
    /// assert_eq!(e.evaluate_to_f64().unwrap(), 5.0);
    ///
    /// // Symbolic expression fails
    /// let x = symbol!(x);
    /// assert!(x.evaluate_to_f64().is_err());
    /// ```
    pub fn evaluate_to_f64(&self) -> Result<f64, crate::MathError> {
        let evaluated = self.evaluate()?;
        match evaluated {
            Expression::Number(n) => match n {
                Number::Integer(i) => Ok(i as f64),
                Number::Float(f) => Ok(f),
                Number::BigInteger(bi) => Ok(bi.to_f64().unwrap_or(f64::INFINITY)),
                Number::Rational(r) => {
                    r.to_f64().ok_or_else(|| crate::MathError::NumericOverflow {
                        operation: "rational to f64 conversion".to_owned(),
                    })
                }
            },
            _ => Err(crate::MathError::NonNumericalResult {
                expression: evaluated.clone(),
            }),
        }
    }
}
