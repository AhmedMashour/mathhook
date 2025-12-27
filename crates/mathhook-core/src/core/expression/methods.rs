//! Expression utility methods
//!
//! This module re-exports utility methods from specialized sub-modules:
//! - `arithmetic_utilities`: GCD, LCM, factorization
//! - `equation_solving`: Symbolic equation solving with auto-detection
//! - `fast_path_solvers`: Direct solver access (skip classification)
//! - `analysis`: Commutativity and variable analysis
//!
//! Matrix-specific methods (transpose, inverse) are in the `matrix_methods` module.

pub mod analysis;
pub mod arithmetic_utilities;
pub mod equation_solving;
pub mod fast_path_solvers;
