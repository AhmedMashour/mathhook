//! MathHook Python bindings
//!
//! High-performance symbolic mathematics for Python

#![deny(clippy::all)]

// Module declarations
mod expression;
mod functions;
mod helpers;
mod solver;
mod types;

// Public API re-exports
pub use expression::PyExpression;
pub use functions::*;
pub use solver::PyMathSolver;
pub use types::*;

use pyo3::prelude::*;

/// Python module definition
#[pymodule]
fn mathhook(m: &Bound<'_, PyModule>) -> PyResult<()> {
    // Register classes
    m.add_class::<PyExpression>()?;
    m.add_class::<PyMathSolver>()?;
    m.add_class::<PySolverResult>()?;
    m.add_class::<PyStep>()?;
    m.add_class::<PyStepByStepExplanation>()?;
    m.add_class::<PyPattern>()?;
    m.add_class::<PyODESolver>()?;
    m.add_class::<PyPDESolver>()?;
    m.add_class::<PyGroebnerBasis>()?;

    // Register functions from functions module
    m.add_function(wrap_pyfunction!(functions::parse, m)?)?;
    m.add_function(wrap_pyfunction!(functions::symbols, m)?)?;
    m.add_function(wrap_pyfunction!(functions::init_printing, m)?)?;
    m.add_function(wrap_pyfunction!(functions::pprint, m)?)?;
    m.add_function(wrap_pyfunction!(functions::sin, m)?)?;
    m.add_function(wrap_pyfunction!(functions::cos, m)?)?;
    m.add_function(wrap_pyfunction!(functions::tan, m)?)?;
    m.add_function(wrap_pyfunction!(functions::asin, m)?)?;
    m.add_function(wrap_pyfunction!(functions::acos, m)?)?;
    m.add_function(wrap_pyfunction!(functions::atan, m)?)?;
    m.add_function(wrap_pyfunction!(functions::sinh, m)?)?;
    m.add_function(wrap_pyfunction!(functions::cosh, m)?)?;
    m.add_function(wrap_pyfunction!(functions::tanh, m)?)?;
    m.add_function(wrap_pyfunction!(functions::exp, m)?)?;
    m.add_function(wrap_pyfunction!(functions::log, m)?)?;
    m.add_function(wrap_pyfunction!(functions::sqrt, m)?)?;
    m.add_function(wrap_pyfunction!(functions::abs_expr, m)?)?;

    Ok(())
}
