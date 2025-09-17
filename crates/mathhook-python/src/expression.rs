//! Expression module for MathHook Python bindings
//!
//! This module was automatically extracted from lib.rs using syn-based refactoring.
use crate::helpers::{sympify_python, PRINT_CONFIG};
use crate::types::{PyPattern, PyStep, PyStepByStepExplanation};
use mathhook_core::algebra::polynomial_advanced::AdvancedPolynomial;
use mathhook_core::algebra::{Collect, Expand, Factor};
use mathhook_core::calculus::derivatives::{Derivative, DerivativeWithSteps};
use mathhook_core::calculus::integrals::Integration;
use mathhook_core::calculus::{Limits, SeriesExpansion};
use mathhook_core::educational::StepByStep;
use mathhook_core::matrices::MatrixOperations;
use mathhook_core::parser::config::ParserConfig;
use mathhook_core::prelude::*;
use mathhook_core::{Expression, Parser, Simplify, Symbol};
use pyo3::prelude::*;
use std::collections::HashMap;
/// Python wrapper for Expression
#[pyclass]
#[derive(Clone)]
pub struct PyExpression {
    pub(crate) inner: Expression,
}
#[pymethods]
impl PyExpression {
    /// Create a new expression from an integer
    ///
    /// # Examples
    ///
    /// ```python
    /// expr = PyExpression.integer(42)
    /// ```
    #[staticmethod]
    pub fn integer(value: i64) -> Self {
        Self {
            inner: Expression::integer(value),
        }
    }
    /// Create a new expression from a symbol
    ///
    /// # Examples
    ///
    /// ```python
    /// x = PyExpression.symbol("x")
    /// alpha = PyExpression.symbol("α")
    /// ```
    #[staticmethod]
    pub fn symbol(name: &str) -> Self {
        Self {
            inner: Expression::symbol(name),
        }
    }
    /// Add two expressions with auto-conversion
    ///
    /// Accepts Expression, int, float, or string. Enables method chaining.
    ///
    /// # Arguments
    ///
    /// * `other` - Expression, int, float, or string to add
    ///
    /// # Examples
    ///
    /// ```python
    /// x = Expression.symbol('x')
    /// result = x.add(2)              # Auto-converts 2 to Expression
    /// chain = x.add(2).multiply(3)   # Method chaining
    /// ```
    pub fn add(&self, other: &Bound<'_, PyAny>) -> PyResult<Self> {
        let other_expr = sympify_python(other)?;
        Ok(Self {
            inner: Expression::add(vec![self.inner.clone(), other_expr]),
        })
    }
    /// Multiply two expressions with auto-conversion
    ///
    /// Accepts Expression, int, float, or string. Enables method chaining.
    ///
    /// # Arguments
    ///
    /// * `other` - Expression, int, float, or string to multiply by
    ///
    /// # Examples
    ///
    /// ```python
    /// x = Expression.symbol('x')
    /// result = x.multiply(3)         # Auto-converts 3 to Expression
    /// chain = x.multiply(2).add(1)   # Method chaining
    /// ```
    pub fn multiply(&self, other: &Bound<'_, PyAny>) -> PyResult<Self> {
        let other_expr = sympify_python(other)?;
        Ok(Self {
            inner: Expression::mul(vec![self.inner.clone(), other_expr]),
        })
    }
    /// Raise expression to a power with auto-conversion
    ///
    /// Accepts Expression, int, float, or string. Enables method chaining.
    ///
    /// # Arguments
    ///
    /// * `exponent` - Expression, int, float, or string exponent
    ///
    /// # Examples
    ///
    /// ```python
    /// x = Expression.symbol('x')
    /// result = x.pow(2)           # Auto-converts 2 to Expression
    /// chain = x.pow(2).add(1)     # Method chaining
    /// ```
    pub fn pow(&self, exponent: &Bound<'_, PyAny>) -> PyResult<Self> {
        let exp_expr = sympify_python(exponent)?;
        Ok(Self {
            inner: Expression::pow(self.inner.clone(), exp_expr),
        })
    }
    /// Simplify the expression
    ///
    /// # Examples
    ///
    /// ```python
    /// expr = PyExpression.integer(2).add(PyExpression.integer(3))
    /// simplified = expr.simplify()
    /// ```
    pub fn simplify(&self) -> Self {
        Self {
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
    /// ```python
    /// expr1 = PyExpression.parse("x^2 + 2*x + 1")        # Simple notation
    /// expr2 = PyExpression.parse("\\frac{x^2}{2}")       # LaTeX auto-detected
    /// expr3 = PyExpression.parse("Sin[x] + Cos[y]")       # Wolfram auto-detected
    /// ```
    #[staticmethod]
    pub fn parse(input: &str) -> PyResult<Self> {
        let parser = Parser::new(&ParserConfig::default());
        match parser.parse(input) {
            Ok(expr) => Ok(Self { inner: expr }),
            Err(e) => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                "Parse error: {}",
                e
            ))),
        }
    }
    /// Parse with explicit parser configuration
    ///
    /// Allows control over parser settings like implicit multiplication.
    ///
    /// # Arguments
    ///
    /// * `input` - The mathematical expression string
    /// * `enable_implicit_multiplication` - Whether to enable implicit multiplication (default: True)
    ///
    /// # Examples
    ///
    /// ```python
    /// # With implicit multiplication (2x becomes 2*x)
    /// expr1 = PyExpression.parse_with_config("2x", True)
    ///
    /// # Without implicit multiplication (2x would be an error)
    /// expr2 = PyExpression.parse_with_config("2*x", False)
    /// ```
    #[staticmethod]
    pub fn parse_with_config(input: &str, enable_implicit_multiplication: bool) -> PyResult<Self> {
        let config = ParserConfig {
            enable_implicit_multiplication,
        };
        let parser = Parser::new(&config);
        match parser.parse(input) {
            Ok(expr) => Ok(Self { inner: expr }),
            Err(e) => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                "Parse error: {}",
                e
            ))),
        }
    }
    /// Convert expression to LaTeX format
    ///
    /// # Examples
    ///
    /// ```python
    /// expr = PyExpression.symbol("x").pow(PyExpression.integer(2))
    /// latex = expr.to_latex()  # Returns "x^{2}"
    /// ```
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
    /// ```python
    /// expr = PyExpression.symbol("x").pow(PyExpression.integer(2))
    /// simple = expr.to_simple()  # Returns "x^2"
    /// ```
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
    /// ```python
    /// expr = PyExpression.function("sin", [PyExpression.symbol("x")])
    /// wolfram = expr.to_wolfram()  # Returns "Sin[x]"
    /// ```
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
    /// ```python
    /// x = PyExpression.symbol("x")
    /// sin_x = PyExpression.function("sin", [x])
    /// ```
    #[staticmethod]
    pub fn function(name: &str, args: Vec<PyExpression>) -> Self {
        let inner_args: Vec<Expression> = args.into_iter().map(|arg| arg.inner).collect();
        Self {
            inner: Expression::function(name, inner_args),
        }
    }
    /// Create an equation (equality relation)
    ///
    /// # Examples
    ///
    /// ```python
    /// x = PyExpression.symbol("x")
    /// five = PyExpression.integer(5)
    /// equation = PyExpression.equation(x, five)
    /// ```
    #[staticmethod]
    pub fn equation(left: &PyExpression, right: &PyExpression) -> Self {
        Self {
            inner: Expression::equation(left.inner.clone(), right.inner.clone()),
        }
    }
    /// String representation
    ///
    /// # Examples
    ///
    /// ```python
    /// x = PyExpression.symbol("x")
    /// print(str(x))
    /// ```
    pub fn __str__(&self) -> String {
        format!("{}", self.inner)
    }
    /// Compute the derivative with respect to a variable
    ///
    /// # Examples
    ///
    /// ```python
    /// x = Expression.symbol('x')
    /// expr = x.pow(Expression.integer(2))
    /// derivative = expr.derivative('x')
    /// ```
    pub fn derivative(&self, variable: String) -> PyResult<PyExpression> {
        let symbol = Symbol::new(&variable);
        Ok(PyExpression {
            inner: self.inner.derivative(symbol),
        })
    }
    /// Compute the nth derivative with respect to a variable
    ///
    /// # Examples
    ///
    /// ```python
    /// x = Expression.symbol('x')
    /// expr = x.pow(Expression.integer(3))
    /// second_derivative = expr.nth_derivative('x', 2)
    /// ```
    pub fn nth_derivative(&self, variable: String, order: u32) -> PyResult<PyExpression> {
        let symbol = Symbol::new(&variable);
        Ok(PyExpression {
            inner: self.inner.nth_derivative(symbol, order),
        })
    }
    /// Check if the expression is differentiable
    ///
    /// # Examples
    ///
    /// ```python
    /// x = Expression.symbol('x')
    /// expr = x.pow(Expression.integer(2))
    /// if expr.is_differentiable():
    ///     print("Expression is differentiable")
    /// ```
    pub fn is_differentiable(&self, variable: String) -> bool {
        let symbol = Symbol::new(&variable);
        self.inner.is_differentiable(symbol)
    }
    /// Compute the indefinite integral with respect to a variable
    ///
    /// # Examples
    ///
    /// ```python
    /// x = Expression.symbol('x')
    /// expr = x.pow(Expression.integer(2))
    /// integral = expr.integrate('x')
    /// ```
    pub fn integrate(&self, variable: String) -> PyResult<PyExpression> {
        let symbol = Symbol::new(&variable);
        Ok(PyExpression {
            inner: self.inner.integrate(symbol, 0),
        })
    }
    /// Compute the definite integral with respect to a variable
    ///
    /// # Examples
    ///
    /// ```python
    /// x = Expression.symbol('x')
    /// expr = x.pow(Expression.integer(2))
    /// lower = Expression.integer(0)
    /// upper = Expression.integer(1)
    /// result = expr.integrate_definite('x', lower, upper)
    /// ```
    pub fn integrate_definite(
        &self,
        variable: String,
        lower: &PyExpression,
        upper: &PyExpression,
    ) -> PyResult<PyExpression> {
        let symbol = Symbol::new(&variable);
        Ok(PyExpression {
            inner: self
                .inner
                .definite_integrate(symbol, lower.inner.clone(), upper.inner.clone()),
        })
    }
    /// Numerical integration using adaptive Simpson's rule
    ///
    /// # Arguments
    ///
    /// * `variable` - The variable to integrate with respect to
    /// * `lower` - Lower bound of integration (as float)
    /// * `upper` - Upper bound of integration (as float)
    /// * `tolerance` - Error tolerance (optional, default: 1e-10)
    ///
    /// # Examples
    ///
    /// ```python
    /// x = Expression.symbol('x')
    /// expr = Expression.function("sin", [x])
    /// result = expr.integrate_simpson('x', 0.0, 3.14159, 1e-10)
    /// ```
    pub fn integrate_simpson(
        &self,
        variable: String,
        lower: f64,
        upper: f64,
        tolerance: Option<f64>,
    ) -> PyResult<f64> {
        use mathhook_core::calculus::integrals::numerical::{
            AdaptiveSimpson, IntegrationConfig, NumericalIntegrator,
        };
        use std::collections::HashMap;
        let integrator = AdaptiveSimpson::new();
        let config = IntegrationConfig {
            tolerance: tolerance.unwrap_or(1e-10),
            max_iterations: 1000,
            min_subdivisions: 1,
        };
        let f = |x: f64| -> f64 {
            let mut vars = HashMap::new();
            vars.insert(variable.clone(), Expression::float(x));
            let substituted = self.inner.substitute(&vars);
            match substituted.evaluate_to_f64() {
                Ok(val) => val,
                Err(_) => f64::NAN,
            }
        };
        match integrator.integrate(f, lower, upper, &config) {
            Ok(result) => Ok(result.value),
            Err(e) => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                "Integration failed: {}",
                e
            ))),
        }
    }
    /// Numerical integration using Gaussian quadrature
    ///
    /// # Arguments
    ///
    /// * `variable` - The variable to integrate with respect to
    /// * `lower` - Lower bound of integration (as float)
    /// * `upper` - Upper bound of integration (as float)
    /// * `n_points` - Number of quadrature points (optional, default: 5)
    ///
    /// # Examples
    ///
    /// ```python
    /// x = Expression.symbol('x')
    /// expr = x.pow(Expression.integer(2))
    /// result = expr.integrate_gaussian('x', 0.0, 1.0, 5)
    /// ```
    pub fn integrate_gaussian(
        &self,
        variable: String,
        lower: f64,
        upper: f64,
        n_points: Option<usize>,
    ) -> PyResult<f64> {
        use mathhook_core::calculus::integrals::numerical::{
            GaussianQuadrature, IntegrationConfig, NumericalIntegrator,
        };
        use std::collections::HashMap;
        let integrator = GaussianQuadrature::new(n_points.unwrap_or(5));
        let config = IntegrationConfig::default();
        let f = |x: f64| -> f64 {
            let mut vars = HashMap::new();
            vars.insert(variable.clone(), Expression::float(x));
            let substituted = self.inner.substitute(&vars);
            match substituted.evaluate_to_f64() {
                Ok(val) => val,
                Err(_) => f64::NAN,
            }
        };
        match integrator.integrate(f, lower, upper, &config) {
            Ok(result) => Ok(result.value),
            Err(e) => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                "Integration failed: {}",
                e
            ))),
        }
    }
    /// Numerical integration using Romberg integration
    ///
    /// # Arguments
    ///
    /// * `variable` - The variable to integrate with respect to
    /// * `lower` - Lower bound of integration (as float)
    /// * `upper` - Upper bound of integration (as float)
    /// * `max_iterations` - Maximum iterations (optional, default: 10)
    ///
    /// # Examples
    ///
    /// ```python
    /// x = Expression.symbol('x')
    /// expr = Expression.function("exp", [x])
    /// result = expr.integrate_romberg('x', 0.0, 1.0, 10)
    /// ```
    pub fn integrate_romberg(
        &self,
        variable: String,
        lower: f64,
        upper: f64,
        max_iterations: Option<usize>,
    ) -> PyResult<f64> {
        use mathhook_core::calculus::integrals::numerical::{
            IntegrationConfig, NumericalIntegrator, RombergIntegration,
        };
        use std::collections::HashMap;
        let integrator = RombergIntegration::new(max_iterations.unwrap_or(10));
        let config = IntegrationConfig {
            tolerance: 1e-10,
            max_iterations: max_iterations.unwrap_or(10),
            min_subdivisions: 1,
        };
        let f = |x: f64| -> f64 {
            let mut vars = HashMap::new();
            vars.insert(variable.clone(), Expression::float(x));
            let substituted = self.inner.substitute(&vars);
            match substituted.evaluate_to_f64() {
                Ok(val) => val,
                Err(_) => f64::NAN,
            }
        };
        match integrator.integrate(f, lower, upper, &config) {
            Ok(result) => Ok(result.value),
            Err(e) => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                "Integration failed: {}",
                e
            ))),
        }
    }
    /// Expand the expression algebraically
    ///
    /// # Examples
    ///
    /// ```python
    /// x = Expression.symbol('x')
    /// y = Expression.symbol('y')
    /// expr = (x.add(y)).pow(Expression.integer(2))
    /// expanded = expr.expand()
    /// ```
    pub fn expand(&self) -> PyExpression {
        PyExpression {
            inner: self.inner.expand(),
        }
    }
    /// Factor the expression
    ///
    /// # Examples
    ///
    /// ```python
    /// x = Expression.symbol('x')
    /// expr = x.pow(Expression.integer(2)).subtract(Expression.integer(1))
    /// factored = expr.factor()
    /// ```
    pub fn factor(&self) -> PyExpression {
        PyExpression {
            inner: self.inner.factor(),
        }
    }
    /// Collect terms with respect to a variable
    ///
    /// # Examples
    ///
    /// ```python
    /// x = Expression.symbol('x')
    /// expr = x.add(x).add(x)
    /// collected = expr.collect('x')
    /// ```
    pub fn collect(&self, variable: String) -> PyExpression {
        let symbol = Symbol::new(&variable);
        PyExpression {
            inner: self.inner.collect(&symbol),
        }
    }
    /// Substitute variables with expressions
    ///
    /// # Examples
    ///
    /// ```python
    /// x = Expression.symbol('x')
    /// expr = x.pow(Expression.integer(2))
    /// result = expr.substitute({'x': Expression.integer(3)})
    /// ```
    pub fn substitute(&self, substitutions: HashMap<String, PyExpression>) -> PyExpression {
        let mut subs: HashMap<String, Expression> = HashMap::new();
        for (key, value) in substitutions {
            subs.insert(key, value.inner.clone());
        }
        PyExpression {
            inner: self.inner.substitute(&subs),
        }
    }
    /// Evaluate the expression (numerical evaluation)
    ///
    /// # Examples
    ///
    /// ```python
    /// expr = Expression.integer(2).add(Expression.integer(3))
    /// result = expr.evaluate()
    /// ```
    pub fn evaluate(&self) -> PyResult<PyExpression> {
        match self.inner.evaluate() {
            Ok(result) => Ok(PyExpression { inner: result }),
            Err(e) => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                "{}",
                e
            ))),
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
    /// # Raises
    ///
    /// ValueError: If evaluation encounters domain errors (sqrt of negative, etc.)
    ///
    /// # Examples
    ///
    /// ```python
    /// from mathhook import Expression, Symbol, EvalContext
    ///
    /// x = Symbol('x')
    /// expr = x**2 + 2*x + 1
    ///
    /// # Evaluate at x = 3 (numerical)
    /// ctx = EvalContext.numeric({"x": Expression.integer(3)})
    /// result = expr.evaluate_with_context(ctx)
    /// assert result == Expression.integer(16)
    ///
    /// # Symbolic evaluation (no numerical conversion)
    /// ctx = EvalContext.symbolic()
    /// result = expr.evaluate_with_context(ctx)
    /// # Result stays symbolic: x^2 + 2*x + 1
    ///
    /// # Custom precision
    /// ctx = EvalContext.numeric({"x": Expression.float(3.14159)})
    /// ctx = ctx.with_precision(128)
    /// result = expr.evaluate_with_context(ctx)
    /// ```
    pub fn evaluate_with_context(
        &self,
        context: &crate::eval_context::EvalContext,
    ) -> PyResult<PyExpression> {
        match self.inner.evaluate_with_context(&context.inner) {
            Ok(result) => Ok(PyExpression { inner: result }),
            Err(e) => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                "Evaluation error: {}",
                e
            ))),
        }
    }
    /// Create a rational number
    ///
    /// # Examples
    ///
    /// ```python
    /// half = Expression.rational(1, 2)
    /// ```
    #[staticmethod]
    pub fn rational(numerator: i64, denominator: i64) -> PyExpression {
        PyExpression {
            inner: Expression::rational(numerator, denominator),
        }
    }
    /// Create a floating point number
    ///
    /// # Examples
    ///
    /// ```python
    /// pi_approx = Expression.float(3.14159)
    /// ```
    #[staticmethod]
    pub fn float(value: f64) -> PyExpression {
        PyExpression {
            inner: Expression::float(value),
        }
    }
    /// Create a complex number
    ///
    /// # Examples
    ///
    /// ```python
    /// real = Expression.integer(3)
    /// imag = Expression.integer(4)
    /// complex_num = Expression.complex(real, imag)
    /// ```
    #[staticmethod]
    pub fn complex(real: &PyExpression, imag: &PyExpression) -> PyExpression {
        PyExpression {
            inner: Expression::complex(real.inner.clone(), imag.inner.clone()),
        }
    }
    /// Pi constant (π)
    ///
    /// # Examples
    ///
    /// ```python
    /// pi = Expression.pi()
    /// ```
    #[staticmethod]
    pub fn pi() -> PyExpression {
        PyExpression {
            inner: Expression::pi(),
        }
    }
    /// Euler's number (e)
    ///
    /// # Examples
    ///
    /// ```python
    /// e = Expression.e()
    /// ```
    #[staticmethod]
    pub fn e() -> PyExpression {
        PyExpression {
            inner: Expression::e(),
        }
    }
    /// Imaginary unit (i)
    ///
    /// # Examples
    ///
    /// ```python
    /// i = Expression.i()
    /// ```
    #[staticmethod]
    pub fn i() -> PyExpression {
        PyExpression {
            inner: Expression::i(),
        }
    }
    /// Golden ratio (φ)
    ///
    /// # Examples
    ///
    /// ```python
    /// phi = Expression.golden_ratio()
    /// ```
    #[staticmethod]
    pub fn golden_ratio() -> PyExpression {
        PyExpression {
            inner: Expression::golden_ratio(),
        }
    }
    /// Euler-Mascheroni constant (γ)
    ///
    /// # Examples
    ///
    /// ```python
    /// gamma = Expression.euler_gamma()
    /// ```
    #[staticmethod]
    pub fn euler_gamma() -> PyExpression {
        PyExpression {
            inner: Expression::euler_gamma(),
        }
    }
    /// Subtract two expressions with auto-conversion
    ///
    /// Accepts Expression, int, float, or string. Enables method chaining.
    ///
    /// # Arguments
    ///
    /// * `other` - Expression, int, float, or string to subtract
    ///
    /// # Examples
    ///
    /// ```python
    /// x = Expression.symbol('x')
    /// result = x.subtract(5)         # Auto-converts 5 to Expression
    /// chain = x.subtract(2).add(1)   # Method chaining
    /// ```
    pub fn subtract(&self, other: &Bound<'_, PyAny>) -> PyResult<PyExpression> {
        let other_expr = sympify_python(other)?;
        Ok(PyExpression {
            inner: Expression::add(vec![
                self.inner.clone(),
                Expression::mul(vec![Expression::integer(-1), other_expr]),
            ]),
        })
    }
    /// Divide two expressions with auto-conversion
    ///
    /// Accepts Expression, int, float, or string. Enables method chaining.
    ///
    /// # Arguments
    ///
    /// * `other` - Expression, int, float, or string to divide by
    ///
    /// # Examples
    ///
    /// ```python
    /// x = Expression.symbol('x')
    /// result = x.divide(2)           # Auto-converts 2 to Expression
    /// chain = x.divide(2).add(1)     # Method chaining
    /// ```
    pub fn divide(&self, other: &Bound<'_, PyAny>) -> PyResult<PyExpression> {
        let other_expr = sympify_python(other)?;
        Ok(PyExpression {
            inner: Expression::mul(vec![
                self.inner.clone(),
                Expression::pow(other_expr, Expression::integer(-1)),
            ]),
        })
    }
    /// Negate the expression
    ///
    /// # Examples
    ///
    /// ```python
    /// x = Expression.symbol('x')
    /// result = x.negate()
    /// ```
    pub fn negate(&self) -> PyExpression {
        PyExpression {
            inner: Expression::mul(vec![Expression::integer(-1), self.inner.clone()]),
        }
    }
    /// Get step-by-step simplification explanation
    ///
    /// # Examples
    ///
    /// ```python
    /// expr = Expression.integer(2).add(Expression.integer(3))
    /// explanation = expr.explain_simplification()
    /// for step in explanation.steps:
    ///     print(f"{step.title}: {step.description}")
    /// ```
    pub fn explain_simplification(&self) -> PyStepByStepExplanation {
        let explanation = self.inner.explain_simplification();
        PyStepByStepExplanation {
            steps: explanation
                .steps
                .iter()
                .map(|step| PyStep {
                    title: step.title.clone(),
                    description: step.description.clone(),
                    before: format!("{}", step.expression),
                    after: String::new(),
                    expression: Some(PyExpression {
                        inner: step.expression.clone(),
                    }),
                })
                .collect(),
        }
    }
    /// Derivative with step-by-step explanation
    ///
    /// # Examples
    ///
    /// ```python
    /// x = Expression.symbol('x')
    /// expr = x.pow(Expression.integer(2))
    /// explanation = expr.derivative_with_steps('x')
    /// for step in explanation.steps:
    ///     print(f"{step.title}: {step.description}")
    /// ```
    pub fn derivative_with_steps(&self, variable: String) -> PyStepByStepExplanation {
        let symbol = Symbol::new(&variable);
        let explanation = self.inner.derivative_with_steps(&symbol, 1);
        PyStepByStepExplanation {
            steps: explanation
                .steps
                .iter()
                .map(|step| PyStep {
                    title: step.title.clone(),
                    description: step.description.clone(),
                    before: format!("{}", step.expression),
                    after: String::new(),
                    expression: Some(PyExpression {
                        inner: step.expression.clone(),
                    }),
                })
                .collect(),
        }
    }
    /// Compute limit of expression as variable approaches a value
    ///
    /// # Examples
    ///
    /// ```python
    /// x = Expression.symbol('x')
    /// expr = x.pow(Expression.integer(2))
    /// limit = expr.limit('x', Expression.integer(0))
    /// ```
    pub fn limit(&self, variable: String, value: &PyExpression) -> PyResult<PyExpression> {
        let symbol = Symbol::new(&variable);
        Ok(PyExpression {
            inner: self.inner.limit(&symbol, &value.inner),
        })
    }
    /// Compute limit as variable approaches infinity
    ///
    /// # Examples
    ///
    /// ```python
    /// x = Expression.symbol('x')
    /// expr = Expression.integer(1).divide(x)
    /// limit = expr.limit_at_infinity('x')
    /// ```
    pub fn limit_at_infinity(&self, variable: String) -> PyResult<PyExpression> {
        let symbol = Symbol::new(&variable);
        Ok(PyExpression {
            inner: self.inner.limit_at_infinity(&symbol),
        })
    }
    /// Taylor series expansion around a point
    ///
    /// # Examples
    ///
    /// ```python
    /// x = Expression.symbol('x')
    /// expr = Expression.function('sin', [x])
    /// series = expr.taylor_series('x', Expression.integer(0), 5)
    /// ```
    pub fn taylor_series(
        &self,
        variable: String,
        point: &PyExpression,
        order: u32,
    ) -> PyResult<PyExpression> {
        let symbol = Symbol::new(&variable);
        Ok(PyExpression {
            inner: self.inner.taylor_series(&symbol, &point.inner, order),
        })
    }
    /// Mixed partial derivative (differentiate with respect to variables in sequence)
    ///
    /// # Examples
    ///
    /// ```python
    /// x = Expression.symbol('x')
    /// y = Expression.symbol('y')
    /// expr = x.multiply(x).multiply(y)
    /// # Compute d²f/dxdy
    /// partial = expr.partial_derivative(['x', 'y'])
    /// ```
    pub fn partial_derivative(&self, variables: Vec<String>) -> PyResult<PyExpression> {
        let mut result = self.inner.clone();
        for var_name in variables {
            let symbol = Symbol::new(&var_name);
            result = result.derivative(symbol);
        }
        Ok(PyExpression { inner: result })
    }
    /// Create a matrix from rows
    ///
    /// # Examples
    ///
    /// ```python
    /// row1 = [Expression.integer(1), Expression.integer(2)]
    /// row2 = [Expression.integer(3), Expression.integer(4)]
    /// matrix = Expression.matrix([row1, row2])
    /// ```
    #[staticmethod]
    pub fn matrix(rows: Vec<Vec<PyExpression>>) -> PyExpression {
        let inner_rows: Vec<Vec<Expression>> = rows
            .into_iter()
            .map(|row| row.into_iter().map(|expr| expr.inner.clone()).collect())
            .collect();
        PyExpression {
            inner: Expression::matrix(inner_rows),
        }
    }
    /// Create identity matrix
    ///
    /// # Examples
    ///
    /// ```python
    /// identity = Expression.identity_matrix(3)
    /// ```
    #[staticmethod]
    pub fn identity_matrix(size: usize) -> PyExpression {
        PyExpression {
            inner: Expression::identity_matrix(size),
        }
    }
    /// Create zero matrix
    ///
    /// # Examples
    ///
    /// ```python
    /// zeros = Expression.zero_matrix(2, 3)
    /// ```
    #[staticmethod]
    pub fn zero_matrix(rows: usize, cols: usize) -> PyExpression {
        PyExpression {
            inner: Expression::zero_matrix(rows, cols),
        }
    }
    /// Matrix determinant
    ///
    /// # Examples
    ///
    /// ```python
    /// matrix = Expression.matrix([[Expression.integer(1), Expression.integer(2)],
    ///                             [Expression.integer(3), Expression.integer(4)]])
    /// det = matrix.determinant()
    /// ```
    pub fn determinant(&self) -> PyExpression {
        PyExpression {
            inner: self.inner.matrix_determinant(),
        }
    }
    /// Matrix inverse
    ///
    /// # Examples
    ///
    /// ```python
    /// matrix = Expression.identity_matrix(2)
    /// inv = matrix.inverse()
    /// ```
    pub fn inverse(&self) -> PyExpression {
        PyExpression {
            inner: self.inner.matrix_inverse(),
        }
    }
    /// Matrix transpose
    ///
    /// # Examples
    ///
    /// ```python
    /// matrix = Expression.matrix([[Expression.integer(1), Expression.integer(2)],
    ///                             [Expression.integer(3), Expression.integer(4)]])
    /// transposed = matrix.transpose()
    /// ```
    pub fn transpose(&self) -> PyExpression {
        PyExpression {
            inner: self.inner.matrix_transpose(),
        }
    }
    /// Complex conjugate
    ///
    /// # Examples
    ///
    /// ```python
    /// z = Expression.complex(Expression.integer(3), Expression.integer(4))
    /// conjugate = z.complex_conjugate()
    /// ```
    pub fn complex_conjugate(&self) -> PyExpression {
        PyExpression {
            inner: self.inner.complex_conjugate(),
        }
    }
    /// Complex modulus (absolute value)
    ///
    /// # Examples
    ///
    /// ```python
    /// z = Expression.complex(Expression.integer(3), Expression.integer(4))
    /// modulus = z.complex_modulus()
    /// ```
    pub fn complex_modulus(&self) -> PyExpression {
        PyExpression {
            inner: self.inner.complex_modulus(),
        }
    }
    /// Complex argument (phase angle)
    ///
    /// # Examples
    ///
    /// ```python
    /// z = Expression.complex(Expression.integer(3), Expression.integer(4))
    /// argument = z.complex_argument()
    /// ```
    pub fn complex_argument(&self) -> PyExpression {
        PyExpression {
            inner: self.inner.complex_argument(),
        }
    }
    /// Convert to polar form (r, θ)
    ///
    /// # Examples
    ///
    /// ```python
    /// z = Expression.complex(Expression.integer(3), Expression.integer(4))
    /// r, theta = z.to_polar_form()
    /// ```
    pub fn to_polar_form(&self) -> (PyExpression, PyExpression) {
        let (r, theta) = self.inner.to_polar_form();
        (PyExpression { inner: r }, PyExpression { inner: theta })
    }
    /// Complex addition
    ///
    /// # Examples
    ///
    /// ```python
    /// z1 = Expression.complex(Expression.integer(1), Expression.integer(2))
    /// z2 = Expression.complex(Expression.integer(3), Expression.integer(4))
    /// result = z1.complex_add(z2)
    /// ```
    pub fn complex_add(&self, other: &PyExpression) -> PyExpression {
        PyExpression {
            inner: self.inner.complex_add(&other.inner),
        }
    }
    /// Complex subtraction
    ///
    /// # Examples
    ///
    /// ```python
    /// z1 = Expression.complex(Expression.integer(3), Expression.integer(4))
    /// z2 = Expression.complex(Expression.integer(1), Expression.integer(2))
    /// result = z1.complex_subtract(z2)
    /// ```
    pub fn complex_subtract(&self, other: &PyExpression) -> PyExpression {
        PyExpression {
            inner: self.inner.complex_subtract(&other.inner),
        }
    }
    /// Complex multiplication
    ///
    /// # Examples
    ///
    /// ```python
    /// z1 = Expression.complex(Expression.integer(1), Expression.integer(2))
    /// z2 = Expression.complex(Expression.integer(3), Expression.integer(4))
    /// result = z1.complex_multiply(z2)
    /// ```
    pub fn complex_multiply(&self, other: &PyExpression) -> PyExpression {
        PyExpression {
            inner: self.inner.complex_multiply(&other.inner),
        }
    }
    /// Complex division
    ///
    /// # Examples
    ///
    /// ```python
    /// z1 = Expression.complex(Expression.integer(1), Expression.integer(2))
    /// z2 = Expression.complex(Expression.integer(3), Expression.integer(4))
    /// result = z1.complex_divide(z2)
    /// ```
    pub fn complex_divide(&self, other: &PyExpression) -> PyExpression {
        PyExpression {
            inner: self.inner.complex_divide(&other.inner),
        }
    }
    /// Check if expression is real
    ///
    /// # Examples
    ///
    /// ```python
    /// x = Expression.integer(5)
    /// if x.is_real():
    ///     print("Real number")
    /// ```
    pub fn is_real(&self) -> bool {
        self.inner.is_real()
    }
    /// Check if expression is imaginary
    ///
    /// # Examples
    ///
    /// ```python
    /// z = Expression.complex(Expression.integer(3), Expression.integer(4))
    /// if z.is_imaginary():
    ///     print("Has imaginary component")
    /// ```
    pub fn is_imaginary(&self) -> bool {
        self.inner.is_imaginary()
    }
    /// Check if expression is purely imaginary
    ///
    /// # Examples
    ///
    /// ```python
    /// z = Expression.complex(Expression.integer(0), Expression.integer(5))
    /// if z.is_pure_imaginary():
    ///     print("Pure imaginary")
    /// ```
    pub fn is_pure_imaginary(&self) -> bool {
        self.inner.is_pure_imaginary()
    }
    /// Finite summation
    ///
    /// # Examples
    ///
    /// ```python
    /// x = Expression.symbol('x')
    /// result = x.finite_sum('i', Expression.integer(1), Expression.integer(10))
    /// ```
    pub fn finite_sum(
        &self,
        variable: String,
        start: &PyExpression,
        end: &PyExpression,
    ) -> PyExpression {
        let symbol = Symbol::new(&variable);
        PyExpression {
            inner: self.inner.finite_sum(&symbol, &start.inner, &end.inner),
        }
    }
    /// Infinite summation
    ///
    /// # Examples
    ///
    /// ```python
    /// x = Expression.symbol('x')
    /// result = x.infinite_sum('n', Expression.integer(0))
    /// ```
    pub fn infinite_sum(&self, variable: String, start: &PyExpression) -> PyExpression {
        let symbol = Symbol::new(&variable);
        PyExpression {
            inner: self.inner.infinite_sum(&symbol, &start.inner),
        }
    }
    /// Finite product
    ///
    /// # Examples
    ///
    /// ```python
    /// x = Expression.symbol('x')
    /// result = x.finite_product('i', Expression.integer(1), Expression.integer(5))
    /// ```
    pub fn finite_product(
        &self,
        variable: String,
        start: &PyExpression,
        end: &PyExpression,
    ) -> PyExpression {
        let symbol = Symbol::new(&variable);
        PyExpression {
            inner: self.inner.finite_product(&symbol, &start.inner, &end.inner),
        }
    }
    /// Infinite product
    ///
    /// # Examples
    ///
    /// ```python
    /// x = Expression.symbol('x')
    /// result = x.infinite_product('n', Expression.integer(1))
    /// ```
    pub fn infinite_product(&self, variable: String, start: &PyExpression) -> PyExpression {
        let symbol = Symbol::new(&variable);
        PyExpression {
            inner: self.inner.infinite_product(&symbol, &start.inner),
        }
    }
    /// Polynomial division (returns quotient and remainder as tuple)
    ///
    /// # Examples
    ///
    /// ```python
    /// p1 = Expression.parse("x^2 + 2*x + 1")
    /// p2 = Expression.parse("x + 1")
    /// quo, rem = Expression.polynomial_div(p1, p2, 'x')
    /// ```
    #[staticmethod]
    pub fn polynomial_div(
        p1: &PyExpression,
        p2: &PyExpression,
        variable: String,
    ) -> (PyExpression, PyExpression) {
        let symbol = Symbol::new(&variable);
        let (quo, rem) = polynomial_div(&p1.inner, &p2.inner, &symbol);
        (PyExpression { inner: quo }, PyExpression { inner: rem })
    }
    /// Polynomial quotient
    ///
    /// # Examples
    ///
    /// ```python
    /// p1 = Expression.parse("x^2 + 2*x + 1")
    /// p2 = Expression.parse("x + 1")
    /// result = Expression.polynomial_quo(p1, p2, 'x')
    /// ```
    #[staticmethod]
    pub fn polynomial_quo(p1: &PyExpression, p2: &PyExpression, variable: String) -> PyExpression {
        let symbol = Symbol::new(&variable);
        PyExpression {
            inner: polynomial_quo(&p1.inner, &p2.inner, &symbol),
        }
    }
    /// Polynomial remainder
    ///
    /// # Examples
    ///
    /// ```python
    /// p1 = Expression.parse("x^2 + 2*x + 1")
    /// p2 = Expression.parse("x + 1")
    /// result = Expression.polynomial_rem(p1, p2, 'x')
    /// ```
    #[staticmethod]
    pub fn polynomial_rem(p1: &PyExpression, p2: &PyExpression, variable: String) -> PyExpression {
        let symbol = Symbol::new(&variable);
        PyExpression {
            inner: polynomial_rem(&p1.inner, &p2.inner, &symbol),
        }
    }
    /// Multivariate GCD
    ///
    /// # Examples
    ///
    /// ```python
    /// p1 = Expression.parse("x^2*y + x*y^2")
    /// p2 = Expression.parse("x*y^2 + y^3")
    /// result = Expression.multivariate_gcd(p1, p2, ['x', 'y'])
    /// ```
    #[staticmethod]
    pub fn multivariate_gcd(
        p1: &PyExpression,
        p2: &PyExpression,
        variables: Vec<String>,
    ) -> PyExpression {
        let symbols: Vec<Symbol> = variables.iter().map(Symbol::new).collect();
        PyExpression {
            inner: multivariate_gcd(&p1.inner, &p2.inner, &symbols),
        }
    }
    /// Compute polynomial resultant for elimination
    ///
    /// The resultant of two polynomials p and q with respect to variable var is a polynomial
    /// expression that is zero if and only if p and q have a common root. Used for polynomial
    /// elimination and solving systems of polynomial equations.
    ///
    /// # Examples
    ///
    /// ```python
    /// from mathhook_python import PyExpression
    ///
    /// p1 = PyExpression.parse("x^2 + y")
    /// p2 = PyExpression.parse("x + y^2")
    /// result = PyExpression.resultant(p1, p2, 'x')
    /// ```
    #[staticmethod]
    pub fn resultant(p1: &PyExpression, p2: &PyExpression, variable: String) -> PyExpression {
        let symbol = Symbol::new(&variable);
        PyExpression {
            inner: p1.inner.polynomial_resultant(&p2.inner, &symbol),
        }
    }
    /// Compute polynomial discriminant
    ///
    /// The discriminant of a polynomial is a polynomial expression that characterizes the roots.
    /// For a quadratic ax^2 + bx + c, the discriminant is b^2 - 4ac, which determines if roots
    /// are real (positive), repeated (zero), or complex (negative).
    ///
    /// # Examples
    ///
    /// ```python
    /// from mathhook_python import PyExpression
    ///
    /// poly = PyExpression.parse("x^2 + 2*x + 1")
    /// disc = PyExpression.discriminant(poly, 'x')
    /// ```
    #[staticmethod]
    pub fn discriminant(poly: &PyExpression, variable: String) -> PyExpression {
        let symbol = Symbol::new(&variable);
        PyExpression {
            inner: poly.inner.polynomial_discriminant(&symbol),
        }
    }
    /// Compute eigenvalues
    ///
    /// # Examples
    ///
    /// ```python
    /// matrix = Expression.matrix([[Expression.integer(2), Expression.integer(0)],
    ///                             [Expression.integer(0), Expression.integer(3)]])
    /// eigenvals = matrix.eigenvalues()
    /// ```
    pub fn eigenvalues(&self) -> PyResult<Vec<PyExpression>> {
        match &self.inner {
            Expression::Matrix(matrix) => {
                let eigenvals = matrix.eigenvalues();
                Ok(eigenvals
                    .into_iter()
                    .map(|e| PyExpression { inner: e })
                    .collect())
            }
            _ => Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                "eigenvalues() requires a matrix expression",
            )),
        }
    }
    /// Compute characteristic polynomial
    ///
    /// # Examples
    ///
    /// ```python
    /// matrix = Expression.matrix([[Expression.integer(2), Expression.integer(1)],
    ///                             [Expression.integer(1), Expression.integer(2)]])
    /// poly = matrix.characteristic_polynomial()
    /// ```
    pub fn characteristic_polynomial(&self) -> PyResult<PyExpression> {
        match &self.inner {
            Expression::Matrix(matrix) => {
                let char_poly = matrix.characteristic_polynomial();
                Ok(PyExpression {
                    inner: char_poly.to_expression(),
                })
            }
            _ => Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                "characteristic_polynomial() requires a matrix expression",
            )),
        }
    }
    /// Compute trace (sum of diagonal elements)
    ///
    /// # Examples
    ///
    /// ```python
    /// matrix = Expression.matrix([[Expression.integer(1), Expression.integer(2)],
    ///                             [Expression.integer(3), Expression.integer(4)]])
    /// tr = matrix.trace()
    /// ```
    pub fn trace(&self) -> PyResult<PyExpression> {
        match &self.inner {
            Expression::Matrix(matrix) => Ok(PyExpression {
                inner: matrix.trace(),
            }),
            _ => Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                "trace() requires a matrix expression",
            )),
        }
    }
    /// Compute determinant via eigenvalues
    ///
    /// # Examples
    ///
    /// ```python
    /// matrix = Expression.matrix([[Expression.integer(2), Expression.integer(0)],
    ///                             [Expression.integer(0), Expression.integer(3)]])
    /// det = matrix.determinant_via_eigenvalues()
    /// ```
    pub fn determinant_via_eigenvalues(&self) -> PyResult<PyExpression> {
        match &self.inner {
            Expression::Matrix(matrix) => Ok(PyExpression {
                inner: matrix.determinant_via_eigenvalues(),
            }),
            _ => Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                "determinant_via_eigenvalues() requires a matrix expression",
            )),
        }
    }
    /// Check if matrix is diagonalizable
    ///
    /// # Examples
    ///
    /// ```python
    /// matrix = Expression.identity_matrix(3)
    /// if matrix.is_diagonalizable():
    ///     print("Diagonalizable")
    /// ```
    pub fn is_diagonalizable(&self) -> PyResult<bool> {
        match &self.inner {
            Expression::Matrix(matrix) => Ok(matrix.is_diagonalizable()),
            _ => Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                "is_diagonalizable() requires a matrix expression",
            )),
        }
    }
    /// Compute matrix power
    ///
    /// # Examples
    ///
    /// ```python
    /// matrix = Expression.identity_matrix(2)
    /// result = matrix.matrix_power(3)
    /// ```
    pub fn matrix_power(&self, n: i64) -> PyResult<PyExpression> {
        match &self.inner {
            Expression::Matrix(matrix) => match matrix.matrix_power_eigen(n) {
                Some(result) => Ok(PyExpression {
                    inner: Expression::Matrix(Box::new(result)),
                }),
                None => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                    "Matrix power computation failed",
                )),
            },
            _ => Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                "matrix_power() requires a matrix expression",
            )),
        }
    }
    /// Compute matrix exponential
    ///
    /// # Examples
    ///
    /// ```python
    /// matrix = Expression.zero_matrix(2, 2)
    /// result = matrix.matrix_exponential()
    /// ```
    pub fn matrix_exponential(&self) -> PyResult<PyExpression> {
        match &self.inner {
            Expression::Matrix(matrix) => match matrix.matrix_exponential() {
                Some(result) => Ok(PyExpression {
                    inner: Expression::Matrix(Box::new(result)),
                }),
                None => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                    "Matrix exponential computation failed",
                )),
            },
            _ => Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                "matrix_exponential() requires a matrix expression",
            )),
        }
    }
    /// Perform LU decomposition with partial pivoting
    ///
    /// Decomposes matrix A into PA = LU where:
    /// - P is a permutation matrix
    /// - L is lower triangular with 1s on diagonal
    /// - U is upper triangular
    ///
    /// # Examples
    ///
    /// ```python
    /// matrix = Expression.matrix([[Expression.integer(2), Expression.integer(1), Expression.integer(1)],
    ///                             [Expression.integer(4), Expression.integer(3), Expression.integer(3)],
    ///                             [Expression.integer(8), Expression.integer(7), Expression.integer(9)]])
    /// l, u, p = matrix.lu_decomposition()
    /// ```
    pub fn lu_decomposition(&self) -> PyResult<(PyExpression, PyExpression, PyExpression)> {
        match &self.inner {
            Expression::Matrix(matrix) => match matrix.lu_decomposition() {
                Some(lu) => {
                    let p_matrix = match lu.p {
                        Some(p) => PyExpression {
                            inner: Expression::Matrix(Box::new(p)),
                        },
                        None => PyExpression {
                            inner: Expression::identity_matrix(lu.l.dimensions().0),
                        },
                    };
                    Ok((
                        PyExpression {
                            inner: Expression::Matrix(Box::new(lu.l)),
                        },
                        PyExpression {
                            inner: Expression::Matrix(Box::new(lu.u)),
                        },
                        p_matrix,
                    ))
                }
                None => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                    "LU decomposition failed",
                )),
            },
            _ => Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                "lu_decomposition() requires a matrix expression",
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
    /// ```python
    /// matrix = Expression.matrix([[Expression.integer(1), Expression.integer(1), Expression.integer(0)],
    ///                             [Expression.integer(1), Expression.integer(0), Expression.integer(1)],
    ///                             [Expression.integer(0), Expression.integer(1), Expression.integer(1)]])
    /// q, r = matrix.qr_decomposition()
    /// ```
    pub fn qr_decomposition(&self) -> PyResult<(PyExpression, PyExpression)> {
        match &self.inner {
            Expression::Matrix(matrix) => match matrix.qr_decomposition() {
                Some(qr) => Ok((
                    PyExpression {
                        inner: Expression::Matrix(Box::new(qr.q)),
                    },
                    PyExpression {
                        inner: Expression::Matrix(Box::new(qr.r)),
                    },
                )),
                None => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                    "QR decomposition failed",
                )),
            },
            _ => Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                "qr_decomposition() requires a matrix expression",
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
    /// ```python
    /// matrix = Expression.matrix([[Expression.integer(4), Expression.integer(2), Expression.integer(1)],
    ///                             [Expression.integer(2), Expression.integer(3), Expression.integer(0)],
    ///                             [Expression.integer(1), Expression.integer(0), Expression.integer(2)]])
    /// l = matrix.cholesky_decomposition()
    /// ```
    pub fn cholesky_decomposition(&self) -> PyResult<PyExpression> {
        match &self.inner {
            Expression::Matrix(matrix) => match matrix.cholesky_decomposition() {
                Some(chol) => Ok(PyExpression {
                    inner: Expression::Matrix(Box::new(chol.l)),
                }),
                None => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                    "Cholesky decomposition failed (matrix may not be positive definite)",
                )),
            },
            _ => Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                "cholesky_decomposition() requires a matrix expression",
            )),
        }
    }
    /// Perform Singular Value Decomposition
    ///
    /// Decomposes matrix A into A = UΣV^T where:
    /// - U contains left singular vectors (orthogonal)
    /// - Σ contains singular values (diagonal, non-negative)
    /// - V^T contains right singular vectors (orthogonal)
    ///
    /// # Examples
    ///
    /// ```python
    /// matrix = Expression.matrix([[Expression.integer(1), Expression.integer(2)],
    ///                             [Expression.integer(3), Expression.integer(4)],
    ///                             [Expression.integer(5), Expression.integer(6)]])
    /// u, s, vt = matrix.svd()
    /// ```
    pub fn svd(&self) -> PyResult<(PyExpression, PyExpression, PyExpression)> {
        match &self.inner {
            Expression::Matrix(matrix) => match matrix.svd_decomposition() {
                Some(svd) => Ok((
                    PyExpression {
                        inner: Expression::Matrix(Box::new(svd.u)),
                    },
                    PyExpression {
                        inner: Expression::Matrix(Box::new(svd.sigma)),
                    },
                    PyExpression {
                        inner: Expression::Matrix(Box::new(svd.vt)),
                    },
                )),
                None => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                    "SVD decomposition failed",
                )),
            },
            _ => Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                "svd() requires a matrix expression",
            )),
        }
    }
    /// Get matrix rank using SVD
    ///
    /// # Examples
    ///
    /// ```python
    /// matrix = Expression.matrix([[Expression.integer(1), Expression.integer(2)],
    ///                             [Expression.integer(2), Expression.integer(4)]])
    /// rank = matrix.rank()  # Returns 1
    /// ```
    pub fn rank(&self) -> PyResult<usize> {
        match &self.inner {
            Expression::Matrix(matrix) => Ok(matrix.rank()),
            _ => Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                "rank() requires a matrix expression",
            )),
        }
    }
    /// Check if matrix is positive definite
    ///
    /// # Examples
    ///
    /// ```python
    /// matrix = Expression.identity_matrix(3)
    /// if matrix.is_positive_definite():
    ///     print("Positive definite")
    /// ```
    pub fn is_positive_definite(&self) -> PyResult<bool> {
        match &self.inner {
            Expression::Matrix(matrix) => Ok(matrix.is_positive_definite()),
            _ => Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                "is_positive_definite() requires a matrix expression",
            )),
        }
    }
    /// Get condition number (ratio of largest to smallest singular value)
    ///
    /// # Examples
    ///
    /// ```python
    /// matrix = Expression.identity_matrix(3)
    /// cond = matrix.condition_number()  # Returns 1
    /// ```
    pub fn condition_number(&self) -> PyResult<PyExpression> {
        match &self.inner {
            Expression::Matrix(matrix) => Ok(PyExpression {
                inner: matrix.condition_number(),
            }),
            _ => Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                "condition_number() requires a matrix expression",
            )),
        }
    }
    /// Pattern matching
    ///
    /// # Examples
    ///
    /// ```python
    /// expr = Expression.parse("x + 2")
    /// pattern = PyPattern.wildcard("a")
    /// matches = expr.matches(pattern)
    /// ```
    pub fn matches(&self, pattern: &PyPattern) -> Option<HashMap<String, PyExpression>> {
        self.inner.matches(&pattern.inner).map(|matches| {
            matches
                .into_iter()
                .map(|(k, v)| (k, PyExpression { inner: v }))
                .collect()
        })
    }
    /// Pattern replacement
    ///
    /// # Examples
    ///
    /// ```python
    /// expr = Expression.parse("x + 2")
    /// pattern = PyPattern.wildcard("x")
    /// replacement_pattern = PyPattern.exact(Expression.integer(5))
    /// result = expr.replace(pattern, replacement_pattern)
    /// ```
    pub fn replace(&self, pattern: &PyPattern, replacement: &PyPattern) -> PyExpression {
        PyExpression {
            inner: self.inner.replace(&pattern.inner, &replacement.inner),
        }
    }
    /// Check if expression is zero
    ///
    /// # Examples
    ///
    /// ```python
    /// expr = Expression.integer(0)
    /// if expr.is_zero():
    ///     print("Zero")
    /// ```
    pub fn is_zero(&self) -> bool {
        self.inner.is_zero()
    }
    /// Check if expression is non-zero
    ///
    /// # Examples
    ///
    /// ```python
    /// expr = Expression.integer(5)
    /// if expr.is_nonzero():
    ///     print("Non-zero")
    /// ```
    pub fn is_nonzero(&self) -> bool {
        !self.inner.is_zero()
    }
    /// Rational simplification
    ///
    /// # Examples
    ///
    /// ```python
    /// expr = Expression.parse("(x^2 - 1)/(x - 1)")
    /// simplified = expr.rational_simplify()
    /// ```
    pub fn rational_simplify(&self) -> PyExpression {
        PyExpression {
            inner: self.inner.rationalize(),
        }
    }
    /// Trigonometric simplification (alias for simplify)
    ///
    /// # Examples
    ///
    /// ```python
    /// expr = Expression.parse("sin(x)^2 + cos(x)^2")
    /// simplified = expr.trigsimp()
    /// ```
    pub fn trigsimp(&self) -> PyExpression {
        PyExpression {
            inner: self.inner.simplify(),
        }
    }
    /// Expand trigonometric expressions
    ///
    /// # Examples
    ///
    /// ```python
    /// expr = Expression.parse("sin(2*x)")
    /// expanded = expr.expand_trig()
    /// ```
    pub fn expand_trig(&self) -> PyExpression {
        PyExpression {
            inner: self.inner.expand(),
        }
    }
    /// Addition operator with auto-sympification (enables x + y and x + 2)
    ///
    /// # Examples
    ///
    /// ```python
    /// x = Expression.symbol('x')
    /// y = Expression.symbol('y')
    /// result = x + y       # PyExpression + PyExpression
    /// result = x + 2       # PyExpression + Python int
    /// result = x + 3.14    # PyExpression + Python float
    /// ```
    pub fn __add__(&self, other: &Bound<'_, PyAny>) -> PyResult<PyExpression> {
        let other_expr = sympify_python(other)?;
        Ok(PyExpression {
            inner: Expression::add(vec![self.inner.clone(), other_expr]),
        })
    }
    /// Right addition operator (enables 2 + x)
    ///
    /// # Examples
    ///
    /// ```python
    /// x = Expression.symbol('x')
    /// result = 2 + x       # Python int + PyExpression
    /// result = 3.14 + x    # Python float + PyExpression
    /// ```
    pub fn __radd__(&self, other: &Bound<'_, PyAny>) -> PyResult<PyExpression> {
        self.__add__(other)
    }
    /// Subtraction operator with auto-sympification (enables x - y and x - 2)
    ///
    /// # Examples
    ///
    /// ```python
    /// x = Expression.symbol('x')
    /// y = Expression.symbol('y')
    /// result = x - y
    /// result2 = x - 5  # Auto-sympifies 5 to Expression
    /// ```
    pub fn __sub__(&self, other: &Bound<'_, PyAny>) -> PyResult<PyExpression> {
        let other_expr = sympify_python(other)?;
        Ok(PyExpression {
            inner: Expression::add(vec![
                self.inner.clone(),
                Expression::mul(vec![Expression::integer(-1), other_expr]),
            ]),
        })
    }
    /// Right subtraction operator (enables 2 - x)
    ///
    /// # Examples
    ///
    /// ```python
    /// x = Expression.symbol('x')
    /// result = 5 - x  # Auto-sympifies 5
    /// ```
    pub fn __rsub__(&self, other: &Bound<'_, PyAny>) -> PyResult<PyExpression> {
        let other_expr = sympify_python(other)?;
        Ok(PyExpression {
            inner: Expression::add(vec![
                other_expr,
                Expression::mul(vec![Expression::integer(-1), self.inner.clone()]),
            ]),
        })
    }
    /// Multiplication operator with auto-sympification (enables x * y and x * 2)
    ///
    /// # Examples
    ///
    /// ```python
    /// x = Expression.symbol('x')
    /// y = Expression.symbol('y')
    /// result = x * y
    /// result2 = x * 3  # Auto-sympifies 3 to Expression
    /// ```
    pub fn __mul__(&self, other: &Bound<'_, PyAny>) -> PyResult<PyExpression> {
        let other_expr = sympify_python(other)?;
        Ok(PyExpression {
            inner: Expression::mul(vec![self.inner.clone(), other_expr]),
        })
    }
    /// Right multiplication operator (enables 2 * x)
    ///
    /// # Examples
    ///
    /// ```python
    /// x = Expression.symbol('x')
    /// result = 3 * x  # Auto-sympifies 3
    /// ```
    pub fn __rmul__(&self, other: &Bound<'_, PyAny>) -> PyResult<PyExpression> {
        self.__mul__(other)
    }
    /// Division operator with auto-sympification (enables x / y and x / 2)
    ///
    /// # Arguments
    ///
    /// * `other` - Divisor (Expression, int, float, or string)
    ///
    /// # Examples
    ///
    /// ```python
    /// x = Expression.symbol('x')
    /// result = x / 2  # Auto-sympifies 2 to Expression
    /// result2 = x / y  # Works with other expressions
    /// ```
    pub fn __truediv__(&self, other: &Bound<'_, PyAny>) -> PyResult<PyExpression> {
        let other_expr = sympify_python(other)?;
        Ok(PyExpression {
            inner: Expression::mul(vec![
                self.inner.clone(),
                Expression::pow(other_expr, Expression::integer(-1)),
            ]),
        })
    }
    /// Right division operator (enables 2 / x)
    ///
    /// # Arguments
    ///
    /// * `other` - Dividend (Expression, int, float, or string)
    ///
    /// # Examples
    ///
    /// ```python
    /// x = Expression.symbol('x')
    /// result = 2 / x  # Auto-sympifies 2
    /// ```
    pub fn __rtruediv__(&self, other: &Bound<'_, PyAny>) -> PyResult<PyExpression> {
        let other_expr = sympify_python(other)?;
        Ok(PyExpression {
            inner: Expression::mul(vec![
                other_expr,
                Expression::pow(self.inner.clone(), Expression::integer(-1)),
            ]),
        })
    }
    /// Power operator with auto-sympification (enables x ** y and x ** 2)
    ///
    /// # Arguments
    ///
    /// * `exponent` - Exponent (Expression, int, float, or string)
    /// * `_modulo` - Optional modulo (not used, for Python compatibility)
    ///
    /// # Examples
    ///
    /// ```python
    /// x = Expression.symbol('x')
    /// result = x ** 2  # Auto-sympifies 2
    /// result2 = x ** y  # Works with other expressions
    /// ```
    pub fn __pow__(
        &self,
        exponent: &Bound<'_, PyAny>,
        _modulo: Option<&Bound<'_, PyAny>>,
    ) -> PyResult<PyExpression> {
        let exp_expr = sympify_python(exponent)?;
        Ok(PyExpression {
            inner: Expression::pow(self.inner.clone(), exp_expr),
        })
    }
    /// Right power operator (enables 2 ** x)
    ///
    /// # Arguments
    ///
    /// * `base` - Base (Expression, int, float, or string)
    /// * `_modulo` - Optional modulo (not used, for Python compatibility)
    ///
    /// # Examples
    ///
    /// ```python
    /// x = Expression.symbol('x')
    /// result = 2 ** x  # Auto-sympifies 2
    /// ```
    pub fn __rpow__(
        &self,
        base: &Bound<'_, PyAny>,
        _modulo: Option<&Bound<'_, PyAny>>,
    ) -> PyResult<PyExpression> {
        let base_expr = sympify_python(base)?;
        Ok(PyExpression {
            inner: Expression::pow(base_expr, self.inner.clone()),
        })
    }
    /// Negation operator (enables -x syntax)
    ///
    /// # Examples
    ///
    /// ```python
    /// x = Expression.symbol('x')
    /// result = -x
    /// ```
    pub fn __neg__(&self) -> PyExpression {
        self.negate()
    }
    /// Better string representation for debugging
    ///
    /// # Examples
    ///
    /// ```python
    /// x = Expression.symbol('x')
    /// print(repr(x))
    /// ```
    pub fn __repr__(&self) -> String {
        format!("PyExpression({})", self.to_simple())
    }
    /// Equality comparison
    ///
    /// # Examples
    ///
    /// ```python
    /// x = Expression.symbol('x')
    /// y = Expression.symbol('x')
    /// assert x == y
    /// ```
    pub fn __eq__(&self, other: &PyExpression) -> bool {
        format!("{:?}", self.inner) == format!("{:?}", other.inner)
    }
    /// Hash function (enables use in sets and dicts)
    ///
    /// # Examples
    ///
    /// ```python
    /// x = Expression.symbol('x')
    /// expr_set = {x}
    /// ```
    pub fn __hash__(&self) -> u64 {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        let mut hasher = DefaultHasher::new();
        format!("{:?}", self.inner).hash(&mut hasher);
        hasher.finish()
    }
    /// LaTeX representation for Jupyter notebook rendering
    ///
    /// Returns LaTeX string wrapped in $ for display math mode.
    /// Jupyter notebooks automatically call this method to render
    /// mathematical expressions.
    ///
    /// # Examples
    ///
    /// In Jupyter:
    /// ```python
    /// from mathhook import symbols
    /// x, y = symbols('x y')
    /// expr = x**2 + 2*x*y + y**2
    /// # Jupyter automatically calls _repr_latex_() and renders as LaTeX
    /// expr  # Will display as nicely formatted mathematical equation
    /// ```
    pub fn _repr_latex_(&self) -> Option<String> {
        let config = PRINT_CONFIG.read().unwrap();
        if !config.use_latex {
            return None;
        }
        use mathhook_core::formatter::latex::{LaTeXContext, LaTeXFormatter};
        let context = LaTeXContext::default();
        match self.inner.to_latex(Some(context)) {
            Ok(latex) => Some(format!("${}$", latex)),
            Err(_) => Some(format!("${{{:?}}}$", self.inner)),
        }
    }
}
