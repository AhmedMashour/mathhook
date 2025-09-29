//! Declarative macros for ergonomic mathematical expression creation
//!
//! This module provides a comprehensive set of declarative macros that make
//! mathematical programming natural and efficient. All macros are designed
//! for both internal use within MathHook and external use by library users.

// Core macros must be declared first for use by other macros
pub mod expressions;

// Mathematical operation macros
pub mod calculus;
pub mod matrices;
pub mod solvers;

// Utility macros
pub mod educational;
pub mod formatting;
pub mod parsing;
pub mod performance;
pub mod simplification;
pub mod testing;
pub mod validation;

// Future domain-specific macros
pub mod number_theory;
pub mod special_functions;

// Re-export all macros for convenient access (expressions first for macro dependencies)
pub use expressions::*;

// Mathematical operation macros
pub use calculus::*;
pub use matrices::*;
pub use solvers::*;

// Utility macros
pub use educational::*;
pub use formatting::*;
pub use parsing::*;
pub use performance::*;
pub use simplification::*;
pub use testing::*;
pub use validation::*;

// Future domain-specific macros
pub use number_theory::*;
pub use special_functions::*;
