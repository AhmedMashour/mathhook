//! Functions module for MathHook Python bindings
//!
//! This module was automatically extracted from lib.rs using syn-based refactoring.

use crate::PyExpression;

use crate::helpers::{sympify_python, LatexMode, PRINT_CONFIG};
use mathhook_core::{Expression, Symbol};
use pyo3::prelude::*;

#[doc = " Create multiple symbols at once"]
#[doc = ""]
#[doc = " Supports three syntaxes:"]
#[doc = " - Space-separated: \"x y z\" → [x, y, z]"]
#[doc = " - Comma-separated: \"a,b,c\" → [a, b, c]"]
#[doc = " - Range syntax: \"x1:4\" → [x1, x2, x3]"]
#[doc = ""]
#[doc = " # Arguments"]
#[doc = ""]
#[doc = " * `names` - String with symbol names"]
#[doc = ""]
#[doc = " # Examples"]
#[doc = ""]
#[doc = " ```python"]
#[doc = " x, y, z = symbols('x y z')"]
#[doc = " a, b = symbols('a,b')"]
#[doc = " x0, x1, x2 = symbols('x0:3')"]
#[doc = " ```"]
#[pyfunction]
pub fn symbols(names: &str) -> PyResult<Vec<PyExpression>> {
    if names.contains(':') {
        let parts: Vec<&str> = names.split(':').collect();
        if parts.len() != 2 {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                "Range syntax must be 'prefix:count' (e.g., 'x1:4')",
            ));
        }
        let prefix = parts[0];
        let end: usize = parts[1].parse().map_err(|_| {
            PyErr::new::<pyo3::exceptions::PyValueError, _>("Range end must be an integer")
        })?;
        let num_start = prefix
            .chars()
            .position(|c| c.is_numeric())
            .unwrap_or(prefix.len());
        let base = &prefix[..num_start];
        let start: usize = if num_start < prefix.len() {
            prefix[num_start..].parse().unwrap_or(0)
        } else {
            0
        };
        let mut result = Vec::new();
        for i in start..end {
            let name = format!("{}{}", base, i);
            result.push(PyExpression {
                inner: Expression::symbol(Symbol::new(&name)),
            });
        }
        return Ok(result);
    }
    let separator = if names.contains(',') { ',' } else { ' ' };
    let symbol_names: Vec<&str> = names
        .split(separator)
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .collect();
    if symbol_names.is_empty() {
        return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
            "No symbol names provided",
        ));
    }
    Ok(symbol_names
        .iter()
        .map(|name| PyExpression {
            inner: Expression::symbol(Symbol::new(name)),
        })
        .collect())
}

#[doc = " Parse a mathematical expression from a string"]
#[doc = ""]
#[doc = " Parses mathematical expressions in standard notation, LaTeX, or Wolfram format."]
#[doc = " Supports implicit multiplication (2x, sin(x)cos(y)), operator precedence,"]
#[doc = " and function calls."]
#[doc = ""]
#[doc = " # Arguments"]
#[doc = ""]
#[doc = " * `input` - The mathematical expression string to parse"]
#[doc = ""]
#[doc = " # Examples"]
#[doc = ""]
#[doc = " ```python"]
#[doc = " from mathhook import parse"]
#[doc = ""]
#[doc = " # Standard notation"]
#[doc = " expr = parse('x^2 + 2*x + 1')"]
#[doc = ""]
#[doc = " # Implicit multiplication"]
#[doc = " expr = parse('2x')  # Parsed as 2*x"]
#[doc = ""]
#[doc = " # Functions"]
#[doc = " expr = parse('sin(x) + cos(x)')"]
#[doc = ""]
#[doc = " # LaTeX"]
#[doc = " expr = parse(r'\\frac{x^2}{2}')"]
#[doc = " ```"]
#[pyfunction]
pub fn parse(input: &str) -> PyResult<PyExpression> {
    PyExpression::parse(input)
}

mathhook_macros::generate_python_binding!(sin);

mathhook_macros::generate_python_binding!(cos);

mathhook_macros::generate_python_binding!(tan);

mathhook_macros::generate_python_binding!(exp);

#[doc = " Natural logarithm"]
#[pyfunction]
# [pyo3 (signature = (x , base = None))]
pub fn log(x: &Bound<'_, PyAny>, base: Option<&Bound<'_, PyAny>>) -> PyResult<PyExpression> {
    let expr = sympify_python(x)?;
    let result = if let Some(b) = base {
        let base_expr = sympify_python(b)?;
        Expression::function("log", vec![expr, base_expr])
    } else {
        Expression::function("ln", vec![expr])
    };
    Ok(PyExpression { inner: result })
}

#[doc = " Square root function"]
#[pyfunction]
pub fn sqrt(x: &Bound<'_, PyAny>) -> PyResult<PyExpression> {
    let expr = sympify_python(x)?;
    Ok(PyExpression {
        inner: Expression::pow(expr, Expression::rational(1, 2)),
    })
}

/// Absolute value
///
/// Note: Python API name is 'abs_expr' (to avoid conflict with built-in abs)
/// but internal function name is 'abs'
#[pyfunction]
pub fn abs_expr(x: &Bound<'_, PyAny>) -> PyResult<PyExpression> {
    let expr = sympify_python(x)?;
    Ok(PyExpression {
        inner: Expression::function("abs", vec![expr]),
    })
}

mathhook_macros::generate_python_binding!(asin);

mathhook_macros::generate_python_binding!(acos);

mathhook_macros::generate_python_binding!(atan);

mathhook_macros::generate_python_binding!(sinh);

mathhook_macros::generate_python_binding!(cosh);

mathhook_macros::generate_python_binding!(tanh);

mathhook_macros::generate_python_binding!(ln);

mathhook_macros::generate_python_binding!(sign);

mathhook_macros::generate_python_binding!(floor);

mathhook_macros::generate_python_binding!(ceil);

mathhook_macros::generate_python_binding!(round);

mathhook_macros::generate_python_binding!(gamma);

mathhook_macros::generate_python_binding!(factorial);

mathhook_macros::generate_python_binding!(digamma);

mathhook_macros::generate_python_binding!(zeta);

mathhook_macros::generate_python_binding!(erf);

mathhook_macros::generate_python_binding!(erfc);

mathhook_macros::generate_python_binding!(isprime);

mathhook_macros::generate_python_binary_binding!(gcd);

mathhook_macros::generate_python_binary_binding!(lcm);

mathhook_macros::generate_python_binary_binding!(modulo);

mathhook_macros::generate_python_binary_binding!(polygamma);

mathhook_macros::generate_python_binary_binding!(bessel_j);

mathhook_macros::generate_python_binary_binding!(bessel_y);

mathhook_macros::generate_python_binary_binding!(beta);

/// Get polynomial degree with respect to a variable
///
/// # Arguments
///
/// * `poly` - Polynomial expression
/// * `variable` - Variable name to check degree for
///
/// # Examples
///
/// ```python
/// from mathhook import degree, symbols, parse
///
/// x, y = symbols('x y')
/// poly = parse('x^3 + 2*x^2 + x + 1')
/// deg = degree(poly, 'x')  # Returns 3
/// ```
#[pyfunction]
pub fn degree(poly: &PyExpression, variable: String) -> PyExpression {
    use mathhook_core::functions::polynomials::polynomial_eval;
    use mathhook_core::Symbol;

    let var_symbol = Symbol::new(&variable);
    PyExpression {
        inner: polynomial_eval::degree(&poly.inner, &var_symbol),
    }
}

/// Find polynomial roots with respect to a variable
///
/// # Arguments
///
/// * `poly` - Polynomial expression
/// * `variable` - Variable name to solve for
///
/// # Examples
///
/// ```python
/// from mathhook import roots, symbols, parse
///
/// x = symbols('x')[0]
/// poly = parse('x^2 - 1')
/// r = roots(poly, 'x')  # Returns roots of quadratic
/// ```
#[pyfunction]
pub fn roots(poly: &PyExpression, variable: String) -> PyExpression {
    use mathhook_core::functions::polynomials::polynomial_eval;
    use mathhook_core::Symbol;

    let var_symbol = Symbol::new(&variable);
    PyExpression {
        inner: polynomial_eval::roots(&poly.inner, &var_symbol),
    }
}

#[doc = " Initialize printing for Jupyter/IPython"]
#[doc = ""]
#[doc = " Configures how MathHook expressions are displayed in Jupyter notebooks,"]
#[doc = " IPython shells, and terminals."]
#[doc = ""]
#[doc = " # Arguments"]
#[doc = ""]
#[doc = " * `use_latex` - Enable LaTeX rendering (default: True)"]
#[doc = " * `latex_mode` - LaTeX rendering mode: 'mathjax', 'png', 'svg' (default: 'mathjax')"]
#[doc = " * `unicode` - Use Unicode characters in text output (default: True)"]
#[doc = ""]
#[doc = " # Examples"]
#[doc = ""]
#[doc = " ```python"]
#[doc = " from mathhook import init_printing, symbols"]
#[doc = ""]
#[doc = " # Enable LaTeX with MathJax (default)"]
#[doc = " init_printing()"]
#[doc = ""]
#[doc = " # Disable LaTeX rendering"]
#[doc = " init_printing(use_latex=False)"]
#[doc = ""]
#[doc = " # Use PNG rendering instead of MathJax"]
#[doc = " init_printing(latex_mode='png')"]
#[doc = " ```"]
#[pyfunction]
# [pyo3 (signature = (use_latex = true , latex_mode = "mathjax" , unicode = true))]
pub fn init_printing(use_latex: bool, latex_mode: &str, unicode: bool) -> PyResult<()> {
    let mode = match latex_mode {
        "mathjax" => LatexMode::MathJax,
        "png" => LatexMode::Png,
        "svg" => LatexMode::Svg,
        _ => {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                "latex_mode must be 'mathjax', 'png', or 'svg'",
            ))
        }
    };
    let mut config = PRINT_CONFIG.write().unwrap();
    config.use_latex = use_latex;
    config.latex_mode = mode;
    config.unicode = unicode;
    Ok(())
}

#[doc = " Pretty print expression with Unicode"]
#[doc = ""]
#[doc = " Prints a formatted representation of the expression to stdout, optionally"]
#[doc = " using Unicode characters for mathematical symbols. This is useful for"]
#[doc = " terminal display and debugging."]
#[doc = ""]
#[doc = " # Arguments"]
#[doc = ""]
#[doc = " * `expr` - The expression to print"]
#[doc = " * `use_unicode` - Use Unicode characters (superscripts, Greek letters, etc.)"]
#[doc = "                   Default: True"]
#[doc = ""]
#[doc = " # Examples"]
#[doc = ""]
#[doc = " ```python"]
#[doc = " from mathhook import pprint, symbols"]
#[doc = ""]
#[doc = " x, y = symbols('x y')"]
#[doc = ""]
#[doc = " # Print with Unicode (default)"]
#[doc = " pprint(x**2)  # Shows: x²"]
#[doc = ""]
#[doc = " # Print without Unicode"]
#[doc = " pprint(x**2, use_unicode=False)  # Shows: x^2"]
#[doc = ""]
#[doc = " # Complex expression"]
#[doc = " pprint((x + y)**2)"]
#[doc = " ```"]
#[pyfunction]
# [pyo3 (signature = (expr , use_unicode = true))]
pub fn pprint(py: Python, expr: &PyExpression, use_unicode: bool) -> PyResult<()> {
    use mathhook_core::formatter::simple::{SimpleContext, SimpleFormatter};
    let context = SimpleContext {
        use_unicode,
        implicit_multiplication: true,
        ..Default::default()
    };
    match expr.inner.to_simple(&context) {
        Ok(output) => {
            let sys = py.import("sys")?;
            let stdout = sys.getattr("stdout")?;
            stdout.call_method1("write", (format!("{}\n", output),))?;
            stdout.call_method0("flush")?;
            Ok(())
        }
        Err(e) => Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
            "Failed to format expression: {:?}",
            e
        ))),
    }
}
