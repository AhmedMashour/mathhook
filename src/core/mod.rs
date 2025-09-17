//! Core module containing fundamental data structures and operations

// Core data structures
pub mod symbol;
pub mod number;
pub mod expression;
pub mod operators;

// Performance optimization modules (implemented)
pub mod compact_number;
pub mod compact_expression;

// Re-exports for easy access
pub use symbol::Symbol;
pub use number::Number;
pub use expression::Expression;
pub use compact_number::CompactNumber;
pub use compact_expression::CompactExpression;
