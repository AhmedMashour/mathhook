//! Finite Field Arithmetic for Modular GCD Algorithms
//!
//! This module provides efficient arithmetic in finite fields Z_p (integers mod prime p)
//! and polynomials over finite fields `Z_p[x]`. These are foundational for:
//!
//! - **Modular GCD algorithms** (Zippel, Brown)
//! - **Chinese Remainder Theorem reconstruction**
//! - **Sparse interpolation**
//! - **Industrial-strength polynomial computation**
//! - **Fast polynomial multiplication via NTT**
//! - **Polynomial factorization** (Berlekamp, Hensel lifting)
//!
//! # Mathematical Background
//!
//! A finite field Z_p contains integers {0, 1, 2, ..., p-1} with arithmetic modulo prime p.
//! Every non-zero element has a multiplicative inverse (Fermat's little theorem: a^(p-1) = 1 mod p).
//!
//! Key properties exploited:
//! - Division is always exact (multiplicative inverses exist)
//! - GCD algorithms terminate in polynomial time
//! - Evaluation at random points allows sparse reconstruction
//! - Number Theoretic Transform enables O(n log n) polynomial multiplication
//! - Factorization over Z_p enables factorization over Z via Hensel lifting
//!
//! # Module Organization
//!
//! - `element`: `Zp` type (field elements)
//! - `poly`: `PolyZp` type (polynomials over Z_p)
//! - `gcd`: GCD algorithms for PolyZp
//! - `ntt`: Fast polynomial multiplication via Number Theoretic Transform
//! - `berlekamp`: Polynomial factorization (Berlekamp's algorithm, Hensel lifting)
//! - `bridge`: Conversion to/from Expression
//!
//! # Performance Considerations
//!
//! Following the Rust Performance Book guidelines:
//! - Use `u64` for field elements to leverage native CPU operations
//! - Avoid branches in hot paths using conditional moves
//! - Preallocate vectors with known capacity
//! - Use `#[inline]` for small, frequently-called functions
//! - Use NTT for large polynomial multiplication (O(n log n) vs O(n²))
//!
//! # References
//!
//! - `[Zippel79]` Zippel, R. "Probabilistic algorithms for sparse polynomials"
//! - `[GCL92]` Geddes, Czapor, Labahn. "Algorithms for Computer Algebra"
//! - `[CT65]` Cooley, Tukey. "An algorithm for the machine calculation of complex Fourier series"

pub mod berlekamp;
mod bridge;
mod element;
mod gcd;
mod ntt;
mod poly;

use std::fmt;

/// Error types for finite field operations
#[derive(Debug, Clone, PartialEq)]
pub enum FiniteFieldError {
    /// Modulus must be a prime number
    NonPrimeModulus { modulus: u64 },

    /// Division by zero in finite field
    DivisionByZero,

    /// Modular inverse does not exist (gcd(a, p) != 1)
    NoInverse { element: u64, modulus: u64 },

    /// Polynomial is empty (no coefficients)
    EmptyPolynomial,

    /// Degree mismatch in polynomial operation
    DegreeMismatch {
        expected: usize,
        got: usize,
        operation: &'static str,
    },

    /// Overflow during computation
    Overflow { operation: &'static str },

    /// Invalid evaluation point
    InvalidEvaluationPoint { reason: String },
}

impl fmt::Display for FiniteFieldError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FiniteFieldError::NonPrimeModulus { modulus } => {
                write!(f, "modulus {} is not prime", modulus)
            }
            FiniteFieldError::DivisionByZero => {
                write!(f, "division by zero in finite field")
            }
            FiniteFieldError::NoInverse { element, modulus } => {
                write!(
                    f,
                    "no multiplicative inverse for {} mod {} (gcd != 1)",
                    element, modulus
                )
            }
            FiniteFieldError::EmptyPolynomial => {
                write!(f, "polynomial has no coefficients")
            }
            FiniteFieldError::DegreeMismatch {
                expected,
                got,
                operation,
            } => {
                write!(f, "{} expected degree {}, got {}", operation, expected, got)
            }
            FiniteFieldError::Overflow { operation } => {
                write!(f, "overflow during {}", operation)
            }
            FiniteFieldError::InvalidEvaluationPoint { reason } => {
                write!(f, "invalid evaluation point: {}", reason)
            }
        }
    }
}

impl std::error::Error for FiniteFieldError {}

/// Result type for finite field operations
pub type FiniteFieldResult<T> = Result<T, FiniteFieldError>;

// Re-export from submodules
pub use bridge::educational;
pub use element::{extended_gcd, is_prime, Zp};
pub use gcd::content;
pub use ntt::{multiply_auto, ntt_multiply, NTT_PRIME_1, NTT_PRIME_2, NTT_PRIME_3, NTT_THRESHOLD};
pub use poly::PolyZp;
