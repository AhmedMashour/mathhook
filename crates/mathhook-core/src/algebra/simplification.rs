//! Algebraic Simplification System
//!
//! Registry-based simplification strategies for all mathematical functions.
//! Provides extensible pattern-based algebraic rewrite rules.

pub mod elementary;
pub mod logarithmic;
pub mod registry;
pub mod special;
pub mod strategy;
pub mod trigonometric;

pub use registry::{get_simplification_registry, SimplificationRegistry, SIMPLIFICATION_REGISTRY};
pub use strategy::SimplificationStrategy;
