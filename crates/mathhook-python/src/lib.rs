//! Python bindings for MathHook
//!
//! This crate provides Python bindings using PyO3, exposing the hybrid API
//! for Python users with both Expression-centric and object-oriented interfaces.

use mathhook_core::{Expression, MathSolver, Simplify, Symbol};
use mathhook_parser::{MathLanguage, MathParser};
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
            inner: Expression::multiply(self.inner.clone(), other.inner.clone()),
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

/// Python wrapper for MathParser
#[pyclass]
pub struct PyMathParser {
    inner: MathParser,
}

#[pymethods]
impl PyMathParser {
    /// Create a new parser
    ///
    /// # Examples
    ///
    /// ```python
    /// parser = PyMathParser()
    /// ```
    #[new]
    pub fn new() -> Self {
        Self {
            inner: MathParser::new(),
        }
    }

    /// Parse a mathematical expression
    ///
    /// # Examples
    ///
    /// ```python
    /// parser = PyMathParser()
    /// expr = parser.parse("x + 2", "standard")
    /// latex_expr = parser.parse("\\frac{x}{2}", "latex")
    /// ```
    pub fn parse(&self, input: &str, language: &str) -> PyResult<PyExpression> {
        let lang = match language {
            "latex" => MathLanguage::LaTeX,
            "wolfram" => MathLanguage::Wolfram,
            "standard" => MathLanguage::Standard,
            _ => MathLanguage::Standard,
        };

        match self.inner.parse(input, lang) {
            Ok(expr) => Ok(PyExpression { inner: expr }),
            Err(e) => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                "Parse error: {}",
                e
            ))),
        }
    }
}

/// Python module
#[pymodule]
fn mathhook_python(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyExpression>()?;
    m.add_class::<PyMathSolver>()?;
    m.add_class::<PyMathParser>()?;
    Ok(())
}
