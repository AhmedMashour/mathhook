//! Second-order ODE solvers
//!
//! Provides solvers for common second-order differential equation types:
//! - Constant coefficients: ay'' + by' + cy = r(x)
//! - Future: Cauchy-Euler, variation of parameters, etc.

pub mod constant_coeff;

// Re-export main types
pub use constant_coeff::{ConstantCoeffSecondOrderSolver, RootType};
