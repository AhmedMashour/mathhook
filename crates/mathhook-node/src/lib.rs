//! MathHook Node.js bindings
//!
//! High-performance symbolic mathematics for Node.js

#![deny(clippy::all)]

// Module declarations
mod eval_context;
mod expression;
mod functions;
mod types;

// Public API re-exports
pub use eval_context::EvalContext;
pub use expression::JsExpression;
pub use functions::*;
pub use types::{
    JsMathSolver, JsPDESolver, JsSolveWithStepsResult, JsSolverResult, JsStep,
    JsStepByStepExplanation, LUDecompositionResult, PDESolution, QRDecompositionResult,
    SVDDecompositionResult,
};

// Macro-generated bindings (inline in lib.rs so NAPI can discover them)
mathhook_macros::generate_nodejs_binding!(sin_macro_generated);
mathhook_macros::generate_nodejs_binding!(cos_macro_generated);
mathhook_macros::generate_nodejs_binding!(tan_macro_generated);

#[cfg(test)]
mod export_integration_test;
