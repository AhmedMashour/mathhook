//! Function Evaluation
//!
//! Clean evaluation using direct dispatch for performance.
//! Properties provide metadata only (domain, range, special values).

use crate::core::Expression;

/// Result of mathematical evaluation
#[derive(Debug, Clone)]
pub enum EvaluationResult {
    /// Exact symbolic result
    Exact(Expression),
    /// Numerical approximation
    Numerical(f64),
    /// Unevaluated (remains symbolic)
    Unevaluated,
    /// Error in evaluation
    Error(String),
}
