//! Expression constructor methods
//!
//! This module provides constructor methods for creating Expression instances.
//! The constructors are organized into three logical categories:
//!
//! - `basic`: Core constructors (numbers, symbols, add, mul, pow, constants)
//! - `functions`: Function and calculus constructors (derivatives, integrals, limits)
//! - `specialized`: Advanced constructors (complex numbers, matrices, sets, intervals)
//!
//! All constructors produce expressions in canonical form, ensuring mathematical correctness
//! and consistency across the system.

mod basic;
mod functions;
mod specialized;

#[cfg(test)]
mod tests;

pub use basic::*;
pub use functions::*;
pub use specialized::*;
