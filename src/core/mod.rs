//! Core module containing fundamental data structures and operations

// Core data structures
pub mod symbol;
pub mod number;
pub mod expression;
pub mod operators;

// Educational features (temporarily disabled for compilation)
// pub mod step_by_step;

// Performance optimization modules (essential only)
pub mod compact_number;
pub mod compact_expression;
pub mod simd_ops;
pub mod arena;

// Re-exports for easy access
pub use symbol::Symbol;
pub use number::Number;
pub use expression::Expression;
pub use compact_number::CompactNumber;
pub use compact_expression::CompactExpression;
pub use simd_ops::{SimdOps, SimdOptimized};
pub use arena::ExpressionArena;
// pub use step_by_step::{StepByStep, StepByStepExplanation, Step};
