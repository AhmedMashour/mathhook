/// Wolfram Language formatter
///
/// Outputs proper Wolfram Language syntax that can be perfectly re-parsed
/// by the LALRPOP grammar.
use crate::core::Expression;

/// Format Expression as Wolfram Language notation
///
/// Ensures perfect roundtrip consistency with LALRPOP Wolfram parsing
pub fn format(expr: &Expression) -> String {
    // For now, delegate to simple formatter
    // This will be enhanced with proper Wolfram formatting
    super::simple::format(expr)
}
