//! Prelude module for FFI bindings (Python PyO3 and Node.js NAPI)
//!
//! Re-exports core types and provides conversion utilities for bindings.

pub use crate::core::expression::Expression;
pub use crate::core::symbol::Symbol;
pub use crate::error::MathError;
pub use crate::Number;

#[cfg(feature = "nodejs-bindings")]
pub fn to_napi_error(err: MathError) -> napi::Error {
    napi::Error::from_reason(format!("{:?}", err))
}

#[cfg(feature = "python-bindings")]
pub fn to_py_error(err: MathError) -> pyo3::PyErr {
    pyo3::exceptions::PyValueError::new_err(format!("{:?}", err))
}
