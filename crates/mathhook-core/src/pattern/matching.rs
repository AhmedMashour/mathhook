//! Pattern matching infrastructure for structural matching
//!
//! Provides pattern matching with wildcards and constraints for
//! transformation rules and algebraic manipulation.
//!
//! This module is organized into:
//! - `patterns`: Pattern types, wildcards, and constraints
//! - `engine`: Matching algorithms and replacement logic

mod patterns;
mod engine;

pub use patterns::{Pattern, WildcardConstraints};
pub use engine::{Matchable, PatternMatches};
