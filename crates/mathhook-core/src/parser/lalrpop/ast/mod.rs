/// AST utilities for LALRPOP mathematical expression parser
///
/// Since we use Expression directly in the grammar, this module provides
/// utility functions and helper types for grammar construction.
pub mod visitors;

// Re-export Expression for grammar convenience
pub use crate::core::Expression;
