//! Core module containing fundamental data structures and operations

// Core data structures
pub mod expression;
pub mod operators;
pub mod symbol;

// Performance optimization modules (essential only)
pub mod arena;
pub mod number;
pub mod simd_ops;

// Re-exports for easy access
pub use arena::ExpressionArena;
pub use expression::{Expression, LimitDirection, MathConstant, RelationType};
pub use number::Number;
pub use simd_ops::SimdOptimized;
pub use symbol::Symbol;
