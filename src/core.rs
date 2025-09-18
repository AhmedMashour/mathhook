//! Core module containing fundamental data structures and operations
//! Modern Rust structure - educational features moved to dedicated module

// Core data structures
pub mod symbol;
pub mod number;
pub mod expression;
pub mod operators;

// Performance optimization modules (essential only)
pub mod compact_number;
// 🚀 MAGIC BULLET #2: CompactExpression functionality merged into Expression
pub mod simd_ops;
pub mod arena;

// Re-exports for easy access
pub use symbol::Symbol;
pub use number::Number;
pub use expression::Expression;
pub use compact_number::CompactNumber;
// 🚀 MAGIC BULLET #2: Expression IS CompactExpression (32-byte optimized)
pub use simd_ops::{SimdOps, SimdOptimized};
pub use arena::ExpressionArena;
// Educational features now in dedicated educational module
