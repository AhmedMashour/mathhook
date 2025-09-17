//! Helpers module for MathHook Python bindings
//!
//! This module was automatically extracted from lib.rs using syn-based refactoring.

use crate::expression::PyExpression;
use mathhook_core::{Expression, Symbol};
use pyo3::prelude::*;
use std::sync::RwLock;

#[derive(Clone, Copy)]
pub(crate) enum LatexMode {
    MathJax,
    Png,
    Svg,
}

pub(crate) struct PrintConfig {
    pub(crate) use_latex: bool,
    pub(crate) latex_mode: LatexMode,
    pub(crate) unicode: bool,
}

impl PrintConfig {
    pub(crate) const DEFAULT: Self = Self {
        use_latex: true,
        latex_mode: LatexMode::MathJax,
        unicode: true,
    };
}

pub(crate) static PRINT_CONFIG: RwLock<PrintConfig> = RwLock::new(PrintConfig::DEFAULT);

/// Helper function to convert Python objects to Expression
/// This enables auto-sympification: Python int/float → PyExpression
pub(crate) fn sympify_python(obj: &Bound<'_, PyAny>) -> PyResult<Expression> {
    if let Ok(expr) = obj.extract::<PyExpression>() {
        return Ok(expr.inner);
    }
    if let Ok(value) = obj.extract::<i64>() {
        return Ok(Expression::integer(value));
    }
    if let Ok(value) = obj.extract::<f64>() {
        return Ok(Expression::float(value));
    }
    if let Ok(value) = obj.extract::<String>() {
        return Ok(Expression::symbol(Symbol::new(&value)));
    }
    Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(format!(
        "Cannot convert {} to Expression",
        obj.get_type().name()?
    )))
}
