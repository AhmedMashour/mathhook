//! Expression module for MathHook Node.js bindings
//!
//! This module was automatically extracted from lib.rs using syn-based refactoring.
use crate::types::{
    JsStep, JsStepByStepExplanation, LUDecompositionResult, QRDecompositionResult,
    SVDDecompositionResult,
};
use mathhook_core::algebra::collect::Collect;
use mathhook_core::algebra::expand::Expand;
use mathhook_core::algebra::factor::Factor;
use mathhook_core::algebra::polynomial_advanced::AdvancedPolynomial;
use mathhook_core::calculus::derivatives::Derivative;
use mathhook_core::calculus::integrals::Integration;
use mathhook_core::calculus::limits::Limits;
use mathhook_core::calculus::series::SeriesExpansion;
use mathhook_core::educational::step_by_step::{Step, StepByStep, StepByStepExplanation};
use mathhook_core::matrices::operations::MatrixOperations;
use mathhook_core::parser::config::ParserConfig;
use mathhook_core::parser::Parser;
use mathhook_core::{Expression, Simplify, Symbol};
use napi::bindgen_prelude::*;
use napi_derive::napi;
use std::collections::HashMap;
/// JavaScript wrapper for Expression
#[napi]
pub struct JsExpression {
    pub(crate) inner: Expression,
}
#[napi]
impl JsExpression {
    /// Create a new expression from an integer
    ///
    /// # Examples
    ///
    /// ```javascript
    /// const expr = JsExpression.integer(42);
    /// ```
    #[napi(factory)]
    pub fn integer(value: i64) -> Self {
        Self {
            inner: Expression::integer(value),
        }
    }
    /// Create a new expression from a symbol
    ///
    /// # Examples
    ///
    /// ```javascript
    /// const x = JsExpression.symbol("x");
    /// const alpha = JsExpression.symbol("α");
    /// ```
    #[napi(factory)]
    pub fn symbol(name: String) -> Self {
        Self {
            inner: Expression::symbol(name),
        }
    }
    /// Add two expressions with auto-conversion from numbers
    ///
    /// Accepts either another expression or a number (automatically converted).
    ///
    /// # Examples
    ///
    /// ```javascript
    /// const x = JsExpression.symbol("x");
    /// const result1 = x.add(JsExpression.integer(2));  // Expression
    /// const result2 = x.add(2);                         // Number auto-converted
    /// ```
    #[napi]
    pub fn add(&self, other: Either<&JsExpression, f64>) -> JsExpression {
        let other_expr = match other {
            Either::A(expr) => expr.inner.clone(),
            Either::B(num) => {
                if num.fract() == 0.0 {
                    Expression::integer(num as i64)
                } else {
                    Expression::float(num)
                }
            }
        };
        JsExpression {
            inner: Expression::add(vec![self.inner.clone(), other_expr]),
        }
    }
    /// Multiply two expressions with auto-conversion from numbers
    ///
    /// Accepts either another expression or a number (automatically converted).
    ///
    /// # Examples
    ///
    /// ```javascript
    /// const x = JsExpression.symbol("x");
    /// const result1 = x.multiply(JsExpression.integer(3));  // Expression
    /// const result2 = x.multiply(3);                         // Number auto-converted
    /// ```
    #[napi]
    pub fn multiply(&self, other: Either<&JsExpression, f64>) -> JsExpression {
        let other_expr = match other {
            Either::A(expr) => expr.inner.clone(),
            Either::B(num) => {
                if num.fract() == 0.0 {
                    Expression::integer(num as i64)
                } else {
                    Expression::float(num)
                }
            }
        };
        JsExpression {
            inner: Expression::mul(vec![self.inner.clone(), other_expr]),
        }
    }
    /// Raise expression to a power with auto-conversion from numbers
    ///
    /// Accepts either another expression or a number (automatically converted).
    ///
    /// # Examples
    ///
    /// ```javascript
    /// const x = JsExpression.symbol("x");
    /// const result1 = x.pow(JsExpression.integer(2));  // Expression
    /// const result2 = x.pow(2);                         // Number auto-converted
    /// ```
    #[napi]
    pub fn pow(&self, exponent: Either<&JsExpression, f64>) -> JsExpression {
        let exp_expr = match exponent {
            Either::A(expr) => expr.inner.clone(),
            Either::B(num) => {
                if num.fract() == 0.0 {
                    Expression::integer(num as i64)
                } else {
                    Expression::float(num)
                }
            }
        };
        JsExpression {
            inner: Expression::pow(self.inner.clone(), exp_expr),
        }
    }
    /// Simplify the expression
    ///
    /// # Examples
    ///
    /// ```javascript
    /// const expr = JsExpression.integer(2).add(JsExpression.integer(3));
    /// const simplified = expr.simplify();
    /// ```
    #[napi]
    pub fn simplify(&self) -> JsExpression {
        JsExpression {
            inner: self.inner.clone().simplify(),
        }
    }
    /// Parse a mathematical expression from string with automatic language detection
    ///
    /// The parser automatically detects the mathematical language (LaTeX, Wolfram, or simple notation)
    /// and parses accordingly.
    ///
    /// # Examples
    ///
    /// ```javascript
    /// const expr1 = JsExpression.parse("2*x + sin(y)");        // Simple notation
    /// const expr2 = JsExpression.parse("\\frac{x^2}{2}");      // LaTeX auto-detected
    /// const expr3 = JsExpression.parse("Sin[x] + Cos[y]");     // Wolfram auto-detected
    /// ```
    #[napi(factory)]
    pub fn parse(input: String) -> Result<JsExpression> {
        let parser = Parser::new(&ParserConfig::default());
        match parser.parse(&input) {
            Ok(expr) => Ok(JsExpression { inner: expr }),
            Err(e) => Err(Error::new(
                Status::InvalidArg,
                format!("Parse error: {}", e),
            )),
        }
    }
    /// Convert expression to LaTeX format
    ///
    /// # Examples
    ///
    /// ```javascript
    /// const expr = JsExpression.symbol("x").pow(JsExpression.integer(2));
    /// const latex = expr.toLatex();  // Returns "x^{2}"
    /// ```
    #[napi]
    pub fn to_latex(&self) -> String {
        use mathhook_core::formatter::LaTeXFormatter;
        self.inner
            .to_latex(None)
            .unwrap_or_else(|e| format!("Error: {}", e))
    }
    /// Convert expression to simple mathematical notation
    ///
    /// # Examples
    ///
    /// ```javascript
    /// const expr = JsExpression.symbol("x").pow(JsExpression.integer(2));
    /// const simple = expr.toSimple();  // Returns "x^2"
    /// ```
    #[napi]
    pub fn to_simple(&self) -> String {
        use mathhook_core::formatter::simple::{SimpleContext, SimpleFormatter};
        self.inner
            .to_simple(&SimpleContext::default())
            .unwrap_or_else(|e| format!("Error: {}", e))
    }
    /// Convert expression to Wolfram Language format
    ///
    /// # Examples
    ///
    /// ```javascript
    /// const expr = JsExpression.function("sin", [JsExpression.symbol("x")]);
    /// const wolfram = expr.toWolfram();  // Returns "Sin[x]"
    /// ```
    #[napi]
    pub fn to_wolfram(&self) -> String {
        use mathhook_core::formatter::wolfram::{WolframContext, WolframFormatter};
        self.inner
            .to_wolfram(&WolframContext::default())
            .unwrap_or_else(|e| format!("Error: {}", e))
    }
    /// Create a function expression
    ///
    /// # Examples
    ///
    /// ```javascript
    /// const x = JsExpression.symbol("x");
    /// const sinX = JsExpression.function("sin", [x]);
    /// ```
    #[napi(factory)]
    pub fn function(name: String, args: Vec<&JsExpression>) -> JsExpression {
        let inner_args: Vec<Expression> = args.iter().map(|arg| arg.inner.clone()).collect();
        JsExpression {
            inner: Expression::function(name, inner_args),
        }
    }
    /// Create an equation (equality relation)
    ///
    /// # Examples
    ///
    /// ```javascript
    /// const x = JsExpression.symbol("x");
    /// const five = JsExpression.integer(5);
    /// const equation = JsExpression.equation(x, five);
    /// ```
    #[napi(factory)]
    pub fn equation(left: &JsExpression, right: &JsExpression) -> JsExpression {
        JsExpression {
            inner: Expression::equation(left.inner.clone(), right.inner.clone()),
        }
    }
    /// Compute the derivative with respect to a variable
    ///
    /// # Examples
    ///
    /// ```javascript
    /// const x = JsExpression.symbol("x");
    /// const expr = x.pow(JsExpression.integer(2));
    /// const derivative = expr.derivative("x");
    /// ```
    #[napi]
    pub fn derivative(&self, variable: String) -> Result<JsExpression> {
        let symbol = Symbol::new(variable);
        Ok(JsExpression {
            inner: self.inner.derivative(symbol),
        })
    }
    /// Compute nth derivative
    ///
    /// # Examples
    ///
    /// ```javascript
    /// const x = JsExpression.symbol("x");
    /// const expr = x.pow(JsExpression.integer(3));
    /// const secondDerivative = expr.nthDerivative("x", 2);
    /// ```
    #[napi]
    pub fn nth_derivative(&self, variable: String, order: u32) -> Result<JsExpression> {
        let symbol = Symbol::new(variable);
        Ok(JsExpression {
            inner: self.inner.nth_derivative(symbol, order),
        })
    }
    /// Check if expression is differentiable
    ///
    /// # Examples
    ///
    /// ```javascript
    /// const x = JsExpression.symbol("x");
    /// const expr = x.pow(JsExpression.integer(2));
    /// const canDiff = expr.isDifferentiable();
    /// ```
    #[napi]
    pub fn is_differentiable(&self) -> bool {
        true
    }
    /// Compute indefinite integral
    ///
    /// # Examples
    ///
    /// ```javascript
    /// const x = JsExpression.symbol("x");
    /// const expr = x.pow(JsExpression.integer(2));
    /// const integral = expr.integrate("x");
    /// ```
    #[napi]
    pub fn integrate(&self, variable: String) -> Result<JsExpression> {
        let symbol = Symbol::new(variable);
        Ok(JsExpression {
            inner: self.inner.integrate(symbol, 0),
        })
    }
    /// Compute definite integral
    ///
    /// # Examples
    ///
    /// ```javascript
    /// const x = JsExpression.symbol("x");
    /// const expr = x;
    /// const lower = JsExpression.integer(0);
    /// const upper = JsExpression.integer(1);
    /// const result = expr.integrateDefinite("x", lower, upper);
    /// ```
    #[napi]
    pub fn integrate_definite(
        &self,
        variable: String,
        lower: &JsExpression,
        upper: &JsExpression,
    ) -> Result<JsExpression> {
        let symbol = Symbol::new(variable);
        let indefinite = self.inner.integrate(symbol.clone(), 0);
        let mut upper_subs = HashMap::new();
        upper_subs.insert(symbol.name().to_string(), upper.inner.clone());
        let f_upper = indefinite.substitute(&upper_subs);
        let mut lower_subs = HashMap::new();
        lower_subs.insert(symbol.name().to_string(), lower.inner.clone());
        let f_lower = indefinite.substitute(&lower_subs);
        Ok(JsExpression {
            inner: Expression::add(vec![
                f_upper,
                Expression::mul(vec![Expression::integer(-1), f_lower]),
            ]),
        })
    }
    /// Expand the expression
    ///
    /// # Examples
    ///
    /// ```javascript
    /// const x = JsExpression.symbol("x");
    /// const expr = x.add(JsExpression.integer(1)).pow(JsExpression.integer(2));
    /// const expanded = expr.expand();
    /// ```
    #[napi]
    pub fn expand(&self) -> JsExpression {
        JsExpression {
            inner: self.inner.expand(),
        }
    }
    /// Factor the expression
    ///
    /// # Examples
    ///
    /// ```javascript
    /// const x = JsExpression.symbol("x");
    /// const expr = x.pow(JsExpression.integer(2)).add(x.multiply(JsExpression.integer(2)));
    /// const factored = expr.factor();
    /// ```
    #[napi]
    pub fn factor(&self) -> JsExpression {
        JsExpression {
            inner: self.inner.factor(),
        }
    }
    /// Collect terms with respect to a variable
    ///
    /// # Examples
    ///
    /// ```javascript
    /// const x = JsExpression.symbol("x");
    /// const expr = x.add(x).add(JsExpression.integer(1));
    /// const collected = expr.collect("x");
    /// ```
    #[napi]
    pub fn collect(&self, variable: String) -> JsExpression {
        let symbol = Symbol::new(variable);
        JsExpression {
            inner: self.inner.collect(&symbol),
        }
    }
    /// Substitute variables with expressions
    ///
    /// # Examples
    ///
    /// ```javascript
    /// const x = JsExpression.symbol("x");
    /// const y = JsExpression.symbol("y");
    /// const expr = x.add(y);
    /// const result = expr.substitute({x: JsExpression.integer(2), y: JsExpression.integer(3)});
    /// ```
    #[napi]
    pub fn substitute(&self, substitutions: HashMap<String, &JsExpression>) -> JsExpression {
        let mut subs: HashMap<String, Expression> = HashMap::new();
        for (key, value) in substitutions {
            subs.insert(key, value.inner.clone());
        }
        JsExpression {
            inner: self.inner.substitute(&subs),
        }
    }
    /// Evaluate the expression
    ///
    /// # Examples
    ///
    /// ```javascript
    /// const expr = JsExpression.integer(2).add(JsExpression.integer(3));
    /// const result = expr.evaluate();
    /// ```
    #[napi]
    pub fn evaluate(&self) -> JsExpression {
        JsExpression {
            inner: self.inner.evaluate().unwrap_or_else(|_| self.inner.clone()),
        }
    }
    /// Evaluate expression with context (substitution + computation)
    ///
    /// Provides full control over evaluation behavior including variable substitution,
    /// numerical conversion, precision, and simplification.
    ///
    /// # Arguments
    ///
    /// * `context` - EvalContext controlling evaluation behavior
    ///
    /// # Returns
    ///
    /// Evaluated expression (numerical or symbolic based on context settings)
    ///
    /// # Throws
    ///
    /// Error if evaluation encounters domain errors (sqrt of negative, etc.)
    ///
    /// # Examples
    ///
    /// ```typescript
    /// import { Expression, Symbol, EvalContext } from 'mathhook';
    ///
    /// const x = new Symbol('x');
    /// const expr = x.pow(Expression.integer(2)).add(Expression.integer(1));
    ///
    /// // Evaluate at x = 3 (numerical)
    /// const ctx = EvalContext.numeric({ x: Expression.integer(3) });
    /// const result = expr.evaluateWithContext(ctx);
    /// // result == Expression.integer(10)
    ///
    /// // Symbolic evaluation (no numerical conversion)
    /// const ctxSymbolic = EvalContext.symbolic();
    /// const resultSymbolic = expr.evaluateWithContext(ctxSymbolic);
    /// // Result stays symbolic: x^2 + 1
    ///
    /// // Custom precision
    /// const ctxPrecise = EvalContext.numeric({ x: Expression.float(3.14159) })
    ///   .withPrecision(128);
    /// const resultPrecise = expr.evaluateWithContext(ctxPrecise);
    /// ```
    #[napi]
    pub fn evaluate_with_context(
        &self,
        context: &crate::eval_context::EvalContext,
    ) -> Result<JsExpression> {
        match self.inner.evaluate_with_context(&context.inner) {
            Ok(result) => Ok(JsExpression { inner: result }),
            Err(e) => Err(napi::Error::new(
                napi::Status::GenericFailure,
                format!("Evaluation error: {}", e),
            )),
        }
    }
    /// Create a rational number
    ///
    /// # Examples
    ///
    /// ```javascript
    /// const half = JsExpression.rational(1, 2);
    /// ```
    #[napi(factory)]
    pub fn rational(numerator: i64, denominator: i64) -> JsExpression {
        JsExpression {
            inner: Expression::rational(numerator, denominator),
        }
    }
    /// Create a floating point number
    ///
    /// # Examples
    ///
    /// ```javascript
    /// const pi = JsExpression.float(3.14159);
    /// ```
    #[napi(factory)]
    pub fn float(value: f64) -> JsExpression {
        JsExpression {
            inner: Expression::float(value),
        }
    }
    /// Create a complex number
    ///
    /// # Examples
    ///
    /// ```javascript
    /// const z = JsExpression.complex(JsExpression.integer(3), JsExpression.integer(4));
    /// ```
    #[napi(factory)]
    pub fn complex(real: &JsExpression, imag: &JsExpression) -> JsExpression {
        JsExpression {
            inner: Expression::complex(real.inner.clone(), imag.inner.clone()),
        }
    }
    /// Create the constant π
    ///
    /// # Examples
    ///
    /// ```javascript
    /// const pi = JsExpression.pi();
    /// ```
    #[napi(factory)]
    pub fn pi() -> JsExpression {
        JsExpression {
            inner: Expression::pi(),
        }
    }
    /// Create Euler's number e
    ///
    /// # Examples
    ///
    /// ```javascript
    /// const e = JsExpression.e();
    /// ```
    #[napi(factory)]
    pub fn e() -> JsExpression {
        JsExpression {
            inner: Expression::e(),
        }
    }
    /// Create the imaginary unit i
    ///
    /// # Examples
    ///
    /// ```javascript
    /// const i = JsExpression.i();
    /// ```
    #[napi(factory)]
    pub fn i() -> JsExpression {
        JsExpression {
            inner: Expression::i(),
        }
    }
    /// Create the golden ratio φ
    ///
    /// # Examples
    ///
    /// ```javascript
    /// const phi = JsExpression.goldenRatio();
    /// ```
    #[napi(factory)]
    pub fn golden_ratio() -> JsExpression {
        JsExpression {
            inner: Expression::golden_ratio(),
        }
    }
    /// Create Euler's gamma constant
    ///
    /// # Examples
    ///
    /// ```javascript
    /// const gamma = JsExpression.eulerGamma();
    /// ```
    #[napi(factory)]
    pub fn euler_gamma() -> JsExpression {
        JsExpression {
            inner: Expression::euler_gamma(),
        }
    }
    /// Subtract two expressions with auto-conversion from numbers
    ///
    /// Accepts either another expression or a number (automatically converted).
    ///
    /// # Examples
    ///
    /// ```javascript
    /// const x = JsExpression.symbol("x");
    /// const result1 = x.subtract(JsExpression.symbol("y"));  // Expression
    /// const result2 = x.subtract(3);                          // Number auto-converted
    /// ```
    #[napi]
    pub fn subtract(&self, other: Either<&JsExpression, f64>) -> JsExpression {
        let other_expr = match other {
            Either::A(expr) => expr.inner.clone(),
            Either::B(num) => {
                if num.fract() == 0.0 {
                    Expression::integer(num as i64)
                } else {
                    Expression::float(num)
                }
            }
        };
        JsExpression {
            inner: Expression::add(vec![
                self.inner.clone(),
                Expression::mul(vec![Expression::integer(-1), other_expr]),
            ]),
        }
    }
    /// Divide two expressions with auto-conversion from numbers
    ///
    /// Accepts either another expression or a number (automatically converted).
    ///
    /// # Examples
    ///
    /// ```javascript
    /// const x = JsExpression.symbol("x");
    /// const result1 = x.divide(JsExpression.symbol("y"));  // Expression
    /// const result2 = x.divide(2);                          // Number auto-converted
    /// ```
    #[napi]
    pub fn divide(&self, other: Either<&JsExpression, f64>) -> JsExpression {
        let other_expr = match other {
            Either::A(expr) => expr.inner.clone(),
            Either::B(num) => {
                if num.fract() == 0.0 {
                    Expression::integer(num as i64)
                } else {
                    Expression::float(num)
                }
            }
        };
        JsExpression {
            inner: Expression::mul(vec![
                self.inner.clone(),
                Expression::pow(other_expr, Expression::integer(-1)),
            ]),
        }
    }
    /// Negate the expression
    ///
    /// # Examples
    ///
    /// ```javascript
    /// const x = JsExpression.symbol("x");
    /// const negX = x.negate();
    /// ```
    #[napi]
    pub fn negate(&self) -> JsExpression {
        JsExpression {
            inner: Expression::mul(vec![Expression::integer(-1), self.inner.clone()]),
        }
    }
    /// Get step-by-step explanation of simplification
    ///
    /// # Examples
    ///
    /// ```javascript
    /// const expr = JsExpression.integer(2).add(JsExpression.integer(3));
    /// const explanation = expr.explainSimplification();
    /// ```
    #[napi]
    pub fn explain_simplification(&self) -> JsStepByStepExplanation {
        let explanation = self.inner.explain_simplification();
        JsStepByStepExplanation {
            steps: explanation
                .steps
                .iter()
                .map(|step| JsStep {
                    title: step.title.clone(),
                    description: step.description.clone(),
                    before: format!("{}", explanation.initial_expression),
                    after: format!("{}", step.expression),
                })
                .collect(),
        }
    }
    /// Get step-by-step explanation of derivative
    ///
    /// # Examples
    ///
    /// ```javascript
    /// const x = JsExpression.symbol("x");
    /// const expr = x.pow(JsExpression.integer(2));
    /// const explanation = expr.derivativeWithSteps("x");
    /// ```
    #[napi]
    pub fn derivative_with_steps(&self, variable: String) -> JsStepByStepExplanation {
        let symbol = Symbol::new(variable);
        let derivative_result = self.inner.derivative(symbol.clone());
        let steps = vec![
            Step {
                title: "Given Expression".to_string(),
                description: format!("Differentiate with respect to {}", symbol.name()),
                expression: self.inner.clone(),
                rule_applied: "Initial".to_string(),
                latex: None,
            },
            Step {
                title: "Apply Derivative Rules".to_string(),
                description: "Apply appropriate differentiation rules".to_string(),
                expression: derivative_result.clone(),
                rule_applied: "Derivative".to_string(),
                latex: None,
            },
        ];
        let explanation = StepByStepExplanation {
            initial_expression: self.inner.clone(),
            final_expression: derivative_result,
            steps: steps.clone(),
            total_steps: 2,
            rules_used: vec!["Initial".to_string(), "Derivative".to_string()],
        };
        JsStepByStepExplanation {
            steps: explanation
                .steps
                .iter()
                .map(|step| JsStep {
                    title: step.title.clone(),
                    description: step.description.clone(),
                    before: format!("{}", explanation.initial_expression),
                    after: format!("{}", step.expression),
                })
                .collect(),
        }
    }
    /// Compute limit of expression as variable approaches a value
    ///
    /// # Examples
    ///
    /// ```javascript
    /// const x = JsExpression.symbol("x");
    /// const expr = x.divide(x);
    /// const lim = expr.limit("x", JsExpression.integer(0));
    /// ```
    #[napi]
    pub fn limit(&self, variable: String, value: &JsExpression) -> Result<JsExpression> {
        let symbol = Symbol::new(variable);
        Ok(JsExpression {
            inner: self.inner.limit(&symbol, &value.inner),
        })
    }
    /// Compute limit as variable approaches infinity
    ///
    /// # Examples
    ///
    /// ```javascript
    /// const x = JsExpression.symbol("x");
    /// const expr = x.divide(x.pow(JsExpression.integer(2)));
    /// const lim = expr.limitInfinity("x");
    /// ```
    #[napi]
    pub fn limit_infinity(&self, variable: String) -> Result<JsExpression> {
        let symbol = Symbol::new(variable);
        Ok(JsExpression {
            inner: self.inner.limit_at_infinity(&symbol),
        })
    }
    /// Compute Taylor/Maclaurin series expansion
    ///
    /// # Examples
    ///
    /// ```javascript
    /// const x = JsExpression.symbol("x");
    /// const sinx = JsExpression.function("sin", [x]);
    /// const series = sinx.series("x", JsExpression.integer(0), 5);
    /// ```
    #[napi]
    pub fn series(
        &self,
        variable: String,
        point: &JsExpression,
        order: u32,
    ) -> Result<JsExpression> {
        let symbol = Symbol::new(variable);
        Ok(JsExpression {
            inner: self.inner.taylor_series(&symbol, &point.inner, order),
        })
    }
    /// Compute partial derivative with respect to multiple variables
    ///
    /// # Examples
    ///
    /// ```javascript
    /// const x = JsExpression.symbol("x");
    /// const y = JsExpression.symbol("y");
    /// const expr = x.multiply(y);
    /// const partial = expr.partialDerivative(["x", "y"]);
    /// ```
    #[napi]
    pub fn partial_derivative(&self, variables: Vec<String>) -> Result<JsExpression> {
        let mut result = self.inner.clone();
        for var in variables {
            let symbol = Symbol::new(var);
            result = result.derivative(symbol);
        }
        Ok(JsExpression { inner: result })
    }
    /// Create a matrix from rows
    ///
    /// # Examples
    ///
    /// ```javascript
    /// const a = JsExpression.integer(1);
    /// const b = JsExpression.integer(2);
    /// const c = JsExpression.integer(3);
    /// const d = JsExpression.integer(4);
    /// const m = JsExpression.matrix([[a, b], [c, d]]);
    /// ```
    #[napi(factory)]
    pub fn matrix(rows: Vec<Vec<&JsExpression>>) -> JsExpression {
        let inner_rows: Vec<Vec<Expression>> = rows
            .into_iter()
            .map(|row| row.into_iter().map(|expr| expr.inner.clone()).collect())
            .collect();
        JsExpression {
            inner: Expression::matrix(inner_rows),
        }
    }
    /// Create an identity matrix
    ///
    /// # Examples
    ///
    /// ```javascript
    /// const identity = JsExpression.identityMatrix(3);
    /// ```
    #[napi(factory)]
    pub fn identity_matrix(size: u32) -> JsExpression {
        JsExpression {
            inner: Expression::identity_matrix(size as usize),
        }
    }
    /// Create a zero matrix
    ///
    /// # Examples
    ///
    /// ```javascript
    /// const zeros = JsExpression.zeroMatrix(2, 3);
    /// ```
    #[napi(factory)]
    pub fn zero_matrix(rows: u32, cols: u32) -> JsExpression {
        JsExpression {
            inner: Expression::zero_matrix(rows as usize, cols as usize),
        }
    }
    /// Compute matrix determinant
    ///
    /// # Examples
    ///
    /// ```javascript
    /// const m = JsExpression.identityMatrix(2);
    /// const det = m.determinant();
    /// ```
    #[napi]
    pub fn determinant(&self) -> Result<JsExpression> {
        Ok(JsExpression {
            inner: self.inner.matrix_determinant(),
        })
    }
    /// Compute matrix inverse
    ///
    /// # Examples
    ///
    /// ```javascript
    /// const m = JsExpression.identityMatrix(2);
    /// const inv = m.inverse();
    /// ```
    #[napi]
    pub fn inverse(&self) -> Result<JsExpression> {
        Ok(JsExpression {
            inner: self.inner.matrix_inverse(),
        })
    }
    /// Compute matrix transpose
    ///
    /// # Examples
    ///
    /// ```javascript
    /// const a = JsExpression.integer(1);
    /// const b = JsExpression.integer(2);
    /// const m = JsExpression.matrix([[a, b]]);
    /// const t = m.transpose();
    /// ```
    #[napi]
    pub fn transpose(&self) -> JsExpression {
        JsExpression {
            inner: self.inner.matrix_transpose(),
        }
    }
    /// Perform LU decomposition with partial pivoting
    ///
    /// Decomposes matrix A into PA = LU where:
    /// - P is a permutation matrix (optional, only when pivoting is needed)
    /// - L is lower triangular with 1s on diagonal
    /// - U is upper triangular
    ///
    /// # Examples
    ///
    /// ```javascript
    /// const matrix = JsExpression.matrix([[2, 1, 1], [4, 3, 3], [8, 7, 9]]);
    /// const lu = matrix.luDecomposition();
    /// // lu.l = Lower triangular matrix
    /// // lu.u = Upper triangular matrix
    /// // lu.p = Permutation matrix (if pivoting was needed)
    /// ```
    #[napi]
    pub fn lu_decomposition(&self) -> Result<LUDecompositionResult> {
        match &self.inner {
            Expression::Matrix(matrix) => {
                if let Some(decomp) = matrix.lu_decomposition() {
                    Ok(LUDecompositionResult {
                        l: JsExpression {
                            inner: Expression::Matrix(Box::new(decomp.l)),
                        },
                        u: JsExpression {
                            inner: Expression::Matrix(Box::new(decomp.u)),
                        },
                        p: decomp.p.map(|p| JsExpression {
                            inner: Expression::Matrix(Box::new(p)),
                        }),
                    })
                } else {
                    Err(Error::new(
                        Status::GenericFailure,
                        "LU decomposition failed (matrix may be singular)",
                    ))
                }
            }
            _ => Err(Error::new(
                Status::InvalidArg,
                "luDecomposition() requires a matrix expression",
            )),
        }
    }
    /// Perform QR decomposition using Gram-Schmidt process
    ///
    /// Decomposes matrix A into A = QR where:
    /// - Q is orthogonal (Q^T * Q = I)
    /// - R is upper triangular
    ///
    /// # Examples
    ///
    /// ```javascript
    /// const matrix = JsExpression.matrix([[1, 1, 0], [1, 0, 1], [0, 1, 1]]);
    /// const qr = matrix.qrDecomposition();
    /// // qr.q = Orthogonal matrix
    /// // qr.r = Upper triangular matrix
    /// ```
    #[napi]
    pub fn qr_decomposition(&self) -> Result<QRDecompositionResult> {
        match &self.inner {
            Expression::Matrix(matrix) => {
                if let Some(decomp) = matrix.qr_decomposition() {
                    Ok(QRDecompositionResult {
                        q: JsExpression {
                            inner: Expression::Matrix(Box::new(decomp.q)),
                        },
                        r: JsExpression {
                            inner: Expression::Matrix(Box::new(decomp.r)),
                        },
                    })
                } else {
                    Err(Error::new(
                        Status::GenericFailure,
                        "QR decomposition failed (matrix may be rank-deficient)",
                    ))
                }
            }
            _ => Err(Error::new(
                Status::InvalidArg,
                "qrDecomposition() requires a matrix expression",
            )),
        }
    }
    /// Perform SVD (Singular Value Decomposition)
    ///
    /// Decomposes matrix A into A = UΣV^T where:
    /// - U contains left singular vectors (orthogonal)
    /// - Σ contains singular values (diagonal, non-negative)
    /// - V^T contains right singular vectors (orthogonal)
    ///
    /// # Examples
    ///
    /// ```javascript
    /// const matrix = JsExpression.matrix([[1, 2], [3, 4], [5, 6]]);
    /// const svd = matrix.svdDecomposition();
    /// // svd.u = Left singular vectors
    /// // svd.sigma = Singular values (diagonal matrix)
    /// // svd.vt = Right singular vectors (transposed)
    /// ```
    #[napi]
    pub fn svd_decomposition(&self) -> Result<SVDDecompositionResult> {
        match &self.inner {
            Expression::Matrix(matrix) => {
                if let Some(decomp) = matrix.svd_decomposition() {
                    Ok(SVDDecompositionResult {
                        u: JsExpression {
                            inner: Expression::Matrix(Box::new(decomp.u)),
                        },
                        sigma: JsExpression {
                            inner: Expression::Matrix(Box::new(decomp.sigma)),
                        },
                        vt: JsExpression {
                            inner: Expression::Matrix(Box::new(decomp.vt)),
                        },
                    })
                } else {
                    Err(Error::new(
                        Status::GenericFailure,
                        "SVD decomposition failed",
                    ))
                }
            }
            _ => Err(Error::new(
                Status::InvalidArg,
                "svdDecomposition() requires a matrix expression",
            )),
        }
    }
    /// Perform Cholesky decomposition for positive definite matrices
    ///
    /// Decomposes symmetric positive definite matrix A into A = LL^T where:
    /// - L is lower triangular with positive diagonal elements
    ///
    /// # Examples
    ///
    /// ```javascript
    /// const matrix = JsExpression.matrix([[4, 2, 1], [2, 3, 0], [1, 0, 2]]);
    /// const cholesky = matrix.choleskyDecomposition();
    /// // Returns lower triangular matrix L such that A = L * L^T
    /// ```
    #[napi]
    pub fn cholesky_decomposition(&self) -> Result<JsExpression> {
        match &self.inner {
            Expression::Matrix(matrix) => {
                if let Some(decomp) = matrix.cholesky_decomposition() {
                    Ok(JsExpression {
                        inner: Expression::Matrix(Box::new(decomp.l)),
                    })
                } else {
                    Err(Error::new(
                        Status::GenericFailure,
                        "Cholesky decomposition failed (matrix may not be positive definite)",
                    ))
                }
            }
            _ => Err(Error::new(
                Status::InvalidArg,
                "choleskyDecomposition() requires a matrix expression",
            )),
        }
    }
    /// String representation
    ///
    /// Returns a human-readable string representation of the expression
    /// using standard mathematical notation.
    ///
    /// # Examples
    ///
    /// ```javascript
    /// const x = JsExpression.symbol("x");
    /// const expr = x.pow(JsExpression.integer(2));
    /// console.log(expr.toString()); // "x^2"
    /// ```
    #[napi]
    #[allow(clippy::inherent_to_string)]
    pub fn to_string(&self) -> String {
        format!("{}", self.inner)
    }
}
/// Advanced polynomial operations
#[napi]
impl JsExpression {
    /// Compute resultant of two polynomials for elimination
    ///
    /// The resultant is a polynomial in the coefficients that vanishes if and only if
    /// the two input polynomials have a common root.
    ///
    /// # Arguments
    ///
    /// * `p1` - First polynomial
    /// * `p2` - Second polynomial
    /// * `variable` - Variable to eliminate
    ///
    /// # Examples
    ///
    /// ```javascript
    /// const x = JsExpression.symbol("x");
    /// const p1 = x.pow(JsExpression.integer(2)).subtract(JsExpression.integer(1));
    /// const p2 = x.subtract(JsExpression.integer(1));
    /// const res = JsExpression.resultant(p1, p2, "x");
    /// // Returns 0 (polynomials share root x=1)
    /// ```
    #[napi(factory)]
    pub fn resultant(p1: &JsExpression, p2: &JsExpression, variable: String) -> JsExpression {
        let symbol = Symbol::new(variable);
        JsExpression {
            inner: p1.inner.polynomial_resultant(&p2.inner, &symbol),
        }
    }
    /// Compute discriminant of a polynomial
    ///
    /// The discriminant is a polynomial expression of the coefficients that
    /// is zero if and only if the polynomial has a repeated root.
    ///
    /// # Arguments
    ///
    /// * `poly` - Polynomial expression
    /// * `variable` - Main variable
    ///
    /// # Examples
    ///
    /// ```javascript
    /// const x = JsExpression.symbol("x");
    /// const poly = x.pow(JsExpression.integer(2)).add(x.multiply(JsExpression.integer(2))).add(JsExpression.integer(1));
    /// const disc = JsExpression.discriminant(poly, "x");
    /// // Returns 0 for (x+1)² which has repeated root
    /// ```
    #[napi(factory)]
    pub fn discriminant(poly: &JsExpression, variable: String) -> JsExpression {
        let symbol = Symbol::new(variable);
        JsExpression {
            inner: poly.inner.polynomial_discriminant(&symbol),
        }
    }
    /// Get polynomial degree with respect to a variable
    ///
    /// # Examples
    ///
    /// ```javascript
    /// const x = JsExpression.symbol("x");
    /// const poly = x.pow(JsExpression.integer(3)).add(x);
    /// const degree = poly.polynomialDegree("x");
    /// // Returns 3
    /// ```
    #[napi]
    pub fn polynomial_degree(&self, variable: String) -> Option<i32> {
        let symbol = Symbol::new(variable);
        self.inner.polynomial_degree(&symbol).map(|d| d as i32)
    }
    /// Get leading coefficient of polynomial
    ///
    /// # Examples
    ///
    /// ```javascript
    /// const x = JsExpression.symbol("x");
    /// const poly = JsExpression.integer(2).multiply(x.pow(JsExpression.integer(2))).add(x);
    /// const lc = poly.polynomialLeadingCoefficient("x");
    /// // Returns 2
    /// ```
    #[napi]
    pub fn polynomial_leading_coefficient(&self, variable: String) -> JsExpression {
        let symbol = Symbol::new(variable);
        JsExpression {
            inner: self.inner.polynomial_leading_coefficient(&symbol),
        }
    }
    /// Get content (GCD of coefficients) of polynomial
    ///
    /// # Examples
    ///
    /// ```javascript
    /// const x = JsExpression.symbol("x");
    /// const poly = JsExpression.integer(6).multiply(x.pow(JsExpression.integer(2))).add(JsExpression.integer(9).multiply(x));
    /// const content = poly.polynomialContent();
    /// // Returns 3 (GCD of 6 and 9)
    /// ```
    #[napi]
    pub fn polynomial_content(&self) -> JsExpression {
        JsExpression {
            inner: self.inner.polynomial_content(),
        }
    }
    /// Get primitive part of polynomial (polynomial / content)
    ///
    /// # Examples
    ///
    /// ```javascript
    /// const x = JsExpression.symbol("x");
    /// const poly = JsExpression.integer(6).multiply(x.pow(JsExpression.integer(2))).add(JsExpression.integer(9).multiply(x));
    /// const primitive = poly.polynomialPrimitivePart();
    /// // Returns 2x² + 3x (original / 3)
    /// ```
    #[napi]
    pub fn polynomial_primitive_part(&self) -> JsExpression {
        JsExpression {
            inner: self.inner.polynomial_primitive_part(),
        }
    }
}
