//! Common utilities for PDE solving
//!
//! This module provides shared functionality used across multiple PDE solvers,
//! including eigenvalue computation, domain extraction, and Fourier coefficient generation.

pub mod domain;
pub mod eigenvalue_problem;
pub mod eigenvalues;
pub mod fourier;
pub mod fourier_coefficients;

pub use domain::*;
pub use eigenvalue_problem::*;
pub use eigenvalues::*;
pub use fourier::*;
pub use fourier_coefficients::*;
