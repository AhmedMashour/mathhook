//! Partial Differential Equation (PDE) Solvers
//!
//! This module provides a comprehensive framework for solving partial differential equations.
//! It includes classification, various solution methods, and support for standard PDEs like
//! heat, wave, and Laplace equations.
//!
//! # Organization
//!
//! - `types`: Core PDE types and boundary/initial conditions
//! - `classification`: PDE classification algorithms (order, linearity, type)
//! - `methods`: Solution methods (separation of variables, method of characteristics)
//! - `standard`: Standard PDE solvers (heat, wave, Laplace)
//!
//! # Examples
//!
//! ```rust
//! use mathhook_core::pde::types::Pde;
//! use mathhook_core::{symbol, expr};
//!
//! let u = symbol!(u);
//! let x = symbol!(x);
//! let t = symbol!(t);
//! let equation = expr!(u);
//! let pde = Pde::new(equation, u, vec![x, t]);
//! ```

pub mod classification;
pub mod types;

// Solution methods
pub mod method_of_characteristics;
pub mod separation_of_variables;

// Standard PDEs
pub mod standard;

// Educational wrapper
pub mod educational;

// Re-exports
pub use educational::*;
pub use method_of_characteristics::*;
pub use separation_of_variables::*;
pub use types::*;
