//! Core module containing fundamental data structures and operations

// Core data structures
pub mod symbol;
pub mod number;
pub mod expression;
pub mod operators;

// Performance optimization modules (implemented)
pub mod compact_number;
pub mod compact_expression;
pub mod arena;
pub mod simd_ops;
pub mod hot_path_optimization;

// Re-exports for easy access
pub use symbol::Symbol;
pub use number::Number;
pub use expression::Expression;
pub use compact_number::CompactNumber;
pub use compact_expression::CompactExpression;
pub use arena::{ExpressionArena, ArenaExpression, ArenaOptimized};
pub use simd_ops::{SimdOps, SimdOptimized};
pub use hot_path_optimization::{HotPathOptimizer, HotPathOptimized};
