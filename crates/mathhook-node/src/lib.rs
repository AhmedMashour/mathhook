//! MathHook Node.js bindings
//!
//! High-performance symbolic mathematics for Node.js

#![deny(clippy::all)]

mod generated;

// Hand-written wrappers and API convenience functions
mod functions;
mod types;

// Public API re-exports
pub use functions::*;
pub use generated::JsExpression;
