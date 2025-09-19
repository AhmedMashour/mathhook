//! Core module containing fundamental data structures and operations

// Core data structures
pub mod constants;
pub mod expression;
pub mod operators;
pub mod relations;
pub mod symbol;

// Performance optimization modules (essential only)
pub mod arena;
pub mod number;
pub mod simd_ops;

// Re-exports for easy access
pub use arena::ExpressionArena;
pub use constants::MathConstant;
pub use expression::Expression;
pub use number::Number;
pub use relations::{LimitDirection, RelationType};
pub use simd_ops::SimdOptimized;
pub use symbol::Symbol;
