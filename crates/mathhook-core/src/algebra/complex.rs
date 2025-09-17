//! Complex number operations and arithmetic
//!
//! Handles symbolic complex numbers with Expression-based real and imaginary parts.
//! Provides comprehensive complex arithmetic including addition, multiplication,
//! division, conjugation, and polar form conversions.
//!
//! This module is organized into:
//! - `operations`: ComplexOperations trait with arithmetic operations
//! - `arithmetic`: Convenience methods for complex number manipulation

mod arithmetic;
mod operations;

pub use operations::ComplexOperations;
