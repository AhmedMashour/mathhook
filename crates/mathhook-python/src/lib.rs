//! Python bindings for MathHook
//!
//! This crate provides Python bindings using PyO3, exposing the hybrid API
//! for Python users with both Expression-centric and object-oriented interfaces.

use mathhook_core::parser::config::ParserConfig;
use mathhook_core::{Expression, MathSolver, Parser, Simplify, Symbol};
use pyo3::prelude::*;

/// Python wrapper for Expression
#[pyclass]
#[derive(Clone)]
pub struct PyExpression {
    inner: Expression,
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

    /// Add two expressions
    ///
    /// # Examples
    ///
    /// ```python
    /// x = PyExpression.symbol("x")
    /// two = PyExpression.integer(2)
    /// result = x.add(two)
    /// ```
    pub fn add(&self, other: &PyExpression) -> Self {
        Self {
            inner: Expression::add(vec![self.inner.clone(), other.inner.clone()]),
        }
    }

    /// Multiply two expressions
    ///
    /// # Examples
    ///
    /// ```python
    /// x = PyExpression.symbol("x")
    /// three = PyExpression.integer(3)
    /// result = x.multiply(three)
    /// ```
    pub fn multiply(&self, other: &PyExpression) -> Self {
        Self {
            inner: Expression::mul(vec![self.inner.clone(), other.inner.clone()]),
        }
    }

    /// Raise expression to a power
    ///
    /// # Examples
    ///
    /// ```python
    /// x = PyExpression.symbol("x")
    /// two = PyExpression.integer(2)
    /// x_squared = x.pow(two)
    /// ```
    pub fn pow(&self, exponent: &PyExpression) -> Self {
        Self {
            inner: Expression::pow(self.inner.clone(), exponent.inner.clone()),
        }
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
        let parser = Parser::new(ParserConfig::default());
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
        format!("{:?}", self.inner)
    }
}

/// Python wrapper for MathSolver
#[pyclass]
pub struct PyMathSolver {
    inner: MathSolver,
}

#[pymethods]
impl PyMathSolver {
    /// Create a new solver
    ///
    /// # Examples
    ///
    /// ```python
    /// solver = PyMathSolver()
    /// ```
    #[new]
    pub fn new() -> Self {
        Self {
            inner: MathSolver::new(),
        }
    }

    /// Solve an equation
    ///
    /// # Examples
    ///
    /// ```python
    /// solver = PyMathSolver()
    /// x = PyExpression.symbol("x")
    /// five = PyExpression.integer(5)
    /// equation = PyExpression.equation(x, five)
    /// result = solver.solve(equation, "x")
    /// ```
    pub fn solve(&mut self, equation: &PyExpression, variable: &str) -> String {
        let symbol = Symbol::new(variable);
        let result = self.inner.solve(&equation.inner, &symbol);
        format!("{:?}", result)
    }
}

/// Python module
#[pymodule]
fn mathhook_python(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyExpression>()?;
    m.add_class::<PyMathSolver>()?;
    Ok(())
}
