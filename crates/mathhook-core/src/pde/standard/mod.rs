//! Standard PDE Solvers
//!
//! This module contains solvers for commonly-encountered PDEs in mathematics and physics.

pub mod heat;
pub mod laplace;
pub mod wave;

// Re-exports
pub use heat::*;
pub use laplace::*;
pub use wave::*;
