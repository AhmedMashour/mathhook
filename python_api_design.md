# MathHook Python API Design

## Core PyO3 Wrapper Structure

```rust
// src/python.rs
use pyo3::prelude::*;
use crate::core::Expression;
use crate::parsing::UniversalParser;

#[pyclass]
struct PyExpression {
    inner: Expression,
}

#[pymethods]
impl PyExpression {
    // Arithmetic operators
    fn __add__(&self, other: &PyExpression) -> PyResult<PyExpression> {
        Ok(PyExpression {
            inner: Expression::add(vec![self.inner.clone(), other.inner.clone()])
        })
    }
    
    fn __mul__(&self, other: &PyExpression) -> PyResult<PyExpression> {
        Ok(PyExpression {
            inner: Expression::mul(vec![self.inner.clone(), other.inner.clone()])
        })
    }
    
    fn __pow__(&self, other: &PyExpression, _modulo: Option<&PyExpression>) -> PyResult<PyExpression> {
        Ok(PyExpression {
            inner: Expression::pow(self.inner.clone(), other.inner.clone())
        })
    }
    
    fn __truediv__(&self, other: &PyExpression) -> PyResult<PyExpression> {
        Ok(PyExpression {
            inner: Expression::mul(vec![
                self.inner.clone(),
                Expression::pow(other.inner.clone(), Expression::integer(-1))
            ])
        })
    }
    
    // Format conversion methods
    fn to_latex(&self) -> String {
        let parser = UniversalParser::new();
        parser.to_latex(&self.inner)
    }
    
    fn to_simple(&self) -> String {
        let parser = UniversalParser::new();
        parser.to_simple(&self.inner)
    }
    
    fn to_wolfram(&self) -> String {
        let parser = UniversalParser::new();
        parser.to_wolfram(&self.inner)
    }
    
    // String representation
    fn __str__(&self) -> String {
        self.to_simple()
    }
    
    fn __repr__(&self) -> String {
        format!("Expression({})", self.to_simple())
    }
}

// Module-level functions
#[pyfunction]
fn symbol(name: &str) -> PyExpression {
    PyExpression {
        inner: Expression::symbol(Symbol::new(name))
    }
}

#[pyfunction]
fn symbols(names: &str) -> Vec<PyExpression> {
    names.split_whitespace()
        .map(|name| symbol(name))
        .collect()
}

#[pyfunction]
fn parse(input: &str, format: Option<&str>) -> PyResult<PyExpression> {
    let mut parser = UniversalParser::new();
    
    let result = match format {
        Some("latex") => parser.parse_with_language(input, MathLanguage::LaTeX),
        Some("wolfram") => parser.parse_with_language(input, MathLanguage::Wolfram),
        Some("simple") => parser.parse_with_language(input, MathLanguage::Simple),
        None => parser.parse(input),
        _ => return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>("Invalid format"))
    };
    
    match result {
        Ok(expr) => Ok(PyExpression { inner: expr }),
        Err(e) => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{}", e)))
    }
}

// Mathematical functions
#[pyfunction]
fn sin(expr: &PyExpression) -> PyExpression {
    PyExpression {
        inner: Expression::function("sin", vec![expr.inner.clone()])
    }
}

#[pyfunction]
fn cos(expr: &PyExpression) -> PyExpression {
    PyExpression {
        inner: Expression::function("cos", vec![expr.inner.clone()])
    }
}

// Constants
#[pyfunction]
fn pi() -> PyExpression {
    PyExpression {
        inner: Expression::constant(MathConstant::Pi)
    }
}

#[pyfunction]
fn e() -> PyExpression {
    PyExpression {
        inner: Expression::constant(MathConstant::E)
    }
}

// Module definition
#[pymodule]
fn mathhook(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyExpression>()?;
    m.add_function(wrap_pyfunction!(symbol, m)?)?;
    m.add_function(wrap_pyfunction!(symbols, m)?)?;
    m.add_function(wrap_pyfunction!(parse, m)?)?;
    m.add_function(wrap_pyfunction!(sin, m)?)?;
    m.add_function(wrap_pyfunction!(cos, m)?)?;
    m.add_function(wrap_pyfunction!(pi, m)?)?;
    m.add_function(wrap_pyfunction!(e, m)?)?;
    Ok(())
}
```

## Python Usage Examples

```python
import mathhook as mh

# Method 1: Parse strings (most convenient for complex expressions)
expr = mh.parse("a*x^2 + b*x + c")
latex_expr = mh.parse("\\frac{x}{y}", format="latex")

# Method 2: Build programmatically with operator overloading
x, a, b, c = mh.symbols("x a b c")
quadratic = a*x**2 + b*x + c
fraction = (x + 1) / (x - 1)
trig = mh.sin(x) + mh.cos(x)

# Method 3: Constants
pi_expr = x + mh.pi()
e_expr = mh.e()**x

# Format conversion
print(f"Simple:  {quadratic}")              # "a * x^2 + b * x + c"
print(f"LaTeX:   {quadratic.to_latex()}")   # "a \\cdot x^{2} + b \\cdot x + c"
print(f"Wolfram: {quadratic.to_wolfram()}") # "Plus[Times[a, Power[x, 2]], ...]"

# Calculus (if implemented)
derivative = quadratic.derivative("x")
integral = quadratic.integrate("x")
```

## Advantages of This Approach

1. **Natural Python syntax** - Uses familiar operators (`+`, `*`, `**`, `/`)
2. **Method chaining** - `expr.to_latex().upper()` works naturally
3. **String parsing** - Complex expressions easier via `parse("...")`
4. **Type safety** - PyO3 handles Rust↔Python conversions
5. **Performance** - Core computation in Rust, ergonomic API in Python

## Implementation Priority

1. **Core Expression wrapper** with operator overloading
2. **Parse function** with format detection
3. **Format conversion methods** (to_latex, to_simple, to_wolfram)
4. **Mathematical functions** (sin, cos, etc.)
5. **Constants** (pi, e, i)
6. **Calculus operations** (derivative, integral)

This gives Python users the same ergonomic experience as Rust macros, just using Python's natural syntax!
