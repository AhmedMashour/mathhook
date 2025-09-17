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
//! - `registry`: Registry-based solver dispatch (O(1) lookup, extensible)
//! - `methods`: Solution methods (separation of variables, method of characteristics)
//! - `standard`: Standard PDE solvers (heat, wave, Laplace)
//! - `common`: Shared utilities for PDE solving
//!
//! # Examples
//!
//! ```rust
//! use mathhook_core::calculus::pde;
//! use mathhook_core::{symbol, expr};
//!
//! let u = symbol!(u);
//! let x = symbol!(x);
//! let t = symbol!(t);
//! let equation = expr!(u);
//! let pde = pde::Pde::new(equation, u, vec![x, t]);
//!
//! // Solve using automatic solver selection
//! let solution = pde::solve(&pde).unwrap();
//! ```

pub mod classification;
pub mod types;

// Registry pattern for solver dispatch
pub mod registry;

// Solution methods
pub mod method_of_characteristics;
pub mod separation_of_variables;

// Standard PDEs
pub mod standard;

// Common utilities
pub mod common;

// Educational wrapper
pub mod educational;

// Re-exports
pub use educational::*;
pub use method_of_characteristics::*;
pub use registry::{PDEError, PDEResult, PDESolver, PDESolverRegistry};
pub use separation_of_variables::*;
pub use types::*;

/// Solves a PDE using automatic solver selection.
///
/// Classifies the PDE and dispatches to the appropriate solver based on PDE type
/// (parabolic, hyperbolic, elliptic) using the registry system.
///
/// # Arguments
///
/// * `pde` - The partial differential equation to solve
///
/// # Returns
///
/// Returns the PDE solution wrapped in `PDESolution` type, or an error if solving fails.
///
/// # Examples
///
/// ```rust
/// use mathhook_core::calculus::pde;
/// use mathhook_core::{symbol, expr};
///
/// let u = symbol!(u);
/// let x = symbol!(x);
/// let t = symbol!(t);
/// let equation = expr!(u);
/// let pde = pde::Pde::new(equation, u, vec![x, t]);
///
/// let solution = pde::solve(&pde).unwrap();
/// ```
///
/// # Errors
///
/// Returns `PDEError` if:
/// - PDE classification fails
/// - No solver available for the PDE type
/// - Solver execution fails
pub fn solve(pde: &Pde) -> PDEResult {
    let registry = PDESolverRegistry::new();
    registry.solve(pde)
}
