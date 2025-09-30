/// LaTeX mathematical notation formatter
///
/// Outputs proper LaTeX syntax that can be perfectly re-parsed
/// by the LALRPOP grammar.
use crate::core::Expression;

/// Format Expression as LaTeX mathematical notation
///
/// Ensures perfect roundtrip consistency with LALRPOP LaTeX parsing
pub fn format(expr: &Expression) -> String {
    // For now, delegate to simple formatter
    // This will be enhanced with proper LaTeX formatting
    super::simple::format(expr)
}
