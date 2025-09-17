//! MathHook Python bindings
//!
//! High-performance symbolic mathematics for Python

#![deny(clippy::all)]

// Module declarations
mod eval_context;
mod expression;
mod functions;
mod functions_generated;
mod helpers;
mod polyzp;
mod solver;
mod types;

// Public API re-exports
pub use eval_context::EvalContext;
pub use expression::PyExpression;
pub use functions::*;
pub use polyzp::{poly_gcd, poly_mul_fast, poly_zp, PyPolyZp};
pub use solver::PyMathSolver;
pub use types::*;

use pyo3::prelude::*;

/// Python module definition
#[pymodule]
fn mathhook(m: &Bound<'_, PyModule>) -> PyResult<()> {
    // Register classes
    m.add_class::<EvalContext>()?;
    m.add_class::<PyExpression>()?;
    m.add_class::<PyMathSolver>()?;
    m.add_class::<PySolverResult>()?;
    m.add_class::<PyStep>()?;
    m.add_class::<PyStepByStepExplanation>()?;
    m.add_class::<PyPattern>()?;
    m.add_class::<PyODESolver>()?;
    m.add_class::<PyPDESolver>()?;
    m.add_class::<PyGroebnerBasis>()?;
    m.add_class::<PyPolyZp>()?;

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
    m.add_function(wrap_pyfunction!(functions::ln, m)?)?;
    m.add_function(wrap_pyfunction!(functions::sign, m)?)?;
    m.add_function(wrap_pyfunction!(functions::floor, m)?)?;
    m.add_function(wrap_pyfunction!(functions::ceil, m)?)?;
    m.add_function(wrap_pyfunction!(functions::round, m)?)?;
    m.add_function(wrap_pyfunction!(functions::gamma, m)?)?;
    m.add_function(wrap_pyfunction!(functions::factorial, m)?)?;
    m.add_function(wrap_pyfunction!(functions::digamma, m)?)?;
    m.add_function(wrap_pyfunction!(functions::zeta, m)?)?;
    m.add_function(wrap_pyfunction!(functions::erf, m)?)?;
    m.add_function(wrap_pyfunction!(functions::erfc, m)?)?;
    m.add_function(wrap_pyfunction!(functions::isprime, m)?)?;
    m.add_function(wrap_pyfunction!(functions::gcd, m)?)?;
    m.add_function(wrap_pyfunction!(functions::lcm, m)?)?;
    m.add_function(wrap_pyfunction!(functions::modulo, m)?)?;
    m.add_function(wrap_pyfunction!(functions::polygamma, m)?)?;
    m.add_function(wrap_pyfunction!(functions::bessel_j, m)?)?;
    m.add_function(wrap_pyfunction!(functions::bessel_y, m)?)?;
    m.add_function(wrap_pyfunction!(functions::beta, m)?)?;
    m.add_function(wrap_pyfunction!(functions::degree, m)?)?;
    m.add_function(wrap_pyfunction!(functions::roots, m)?)?;

    // Register macro-generated functions for benchmarking
    m.add_function(wrap_pyfunction!(
        functions_generated::sin_macro_generated,
        m
    )?)?;
    m.add_function(wrap_pyfunction!(
        functions_generated::cos_macro_generated,
        m
    )?)?;
    m.add_function(wrap_pyfunction!(
        functions_generated::tan_macro_generated,
        m
    )?)?;

    // Register fast polynomial functions (direct PolyZp access)
    m.add_function(wrap_pyfunction!(polyzp::poly_zp, m)?)?;
    m.add_function(wrap_pyfunction!(polyzp::poly_gcd, m)?)?;
    m.add_function(wrap_pyfunction!(polyzp::poly_mul_fast, m)?)?;

    Ok(())
}
