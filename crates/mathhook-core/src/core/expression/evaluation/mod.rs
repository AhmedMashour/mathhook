//! Expression evaluation methods
//!
//! This module contains the evaluation logic for expressions, including:
//! - `evaluate()` method with domain checking
//! - `evaluate_with_context()` method for variable substitution
//! - `evaluate_to_f64()` method for numerical conversion
//! - `substitute()` method for variable replacement
//! - Hardcoded function dispatch for performance
//!
//! ## Module Organization
//!
//! - `core`: Main evaluation methods (evaluate, evaluate_with_context, evaluate_to_f64)
//! - `substitution`: Variable substitution (substitute)
//! - `dispatch`: Performance-critical function dispatch table

mod core;
pub mod dispatch;
mod substitution;

// Re-export the dispatch function for use by core.rs
pub use dispatch::evaluate_function_dispatch;
