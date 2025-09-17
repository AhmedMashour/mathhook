//! Substitution system for replacing expressions
//!
//! Provides recursive tree-walking substitution for Expression types.

mod core;
mod rewrite;

// Re-export the trait
pub use self::core::Substitutable;
