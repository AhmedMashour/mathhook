//! Pattern matching infrastructure for structural matching
//!
//! Provides pattern matching with wildcards and constraints for
//! transformation rules and algebraic manipulation.
//!
//! This module is organized into:
//! - `patterns`: Pattern types, wildcards, and constraints
//! - `engine`: Matching algorithms and replacement logic

mod engine;
mod patterns;

pub use engine::{Matchable, PatternMatches};
pub use patterns::{Pattern, WildcardConstraints};
