//! System of ODEs solvers
//!
//! Implements methods for solving systems of first-order linear ODEs with constant coefficients.
//! Uses eigenvalue-eigenvector methods for diagonalizable systems.

pub mod linear;

pub use linear::LinearSystemSolver;
