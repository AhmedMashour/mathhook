//! Number Theoretic Transform (NTT) for Fast Polynomial Multiplication
//!
//! This module implements the Number Theoretic Transform (NTT), which is the finite field
//! analog of the Fast Fourier Transform (FFT). NTT enables O(n log n) polynomial multiplication
//! instead of naive O(n²) convolution.
//!
//! # Mathematical Background
//!
//! The NTT works in finite field Z_p where p is a prime of the form p = k * 2^n + 1.
//! This ensures existence of primitive 2^n-th roots of unity modulo p.
//!
//! **Primitive Root of Unity**: An element ω such that ω^(2^n) ≡ 1 (mod p) but ω^(2^k) ≢ 1 for k < 2^n
//!
//! **Forward NTT**: Transforms polynomial coefficients to point-value representation
//! **Inverse NTT**: Transforms point-values back to coefficients
//! **Pointwise Multiplication**: Multiply polynomials in point-value form in O(n)
//!
//! # NTT-Friendly Primes
//!
//! Common primes with good 2-adic properties:
//! - 2013265921 = 15 * 2^27 + 1 (supports NTT up to degree 2^27 - 1)
//! - 469762049  = 7 * 2^26 + 1  (supports NTT up to degree 2^26 - 1)
//! - 1004535809 = 479 * 2^21 + 1 (supports NTT up to degree 2^21 - 1)
//!
//! # Algorithm: Cooley-Tukey Radix-2 NTT
//!
//! ```text
//! Forward NTT (Decimation-in-time):
//! 1. Bit-reverse permutation of input
//! 2. Butterfly operations with powers of ω
//! 3. Output is point-value representation
//!
//! Inverse NTT (Decimation-in-frequency):
//! 1. Butterfly operations with powers of ω^(-1)
//! 2. Scale by n^(-1) (mod p)
//! 3. Bit-reverse permutation
//! 4. Output is coefficient representation
//! ```
//!
//! # Performance
//!
//! - **Complexity**: O(n log n) for degree n polynomials
//! - **Crossover Point**: Faster than naive multiplication for degree > ~64
//! - **Memory**: O(n) auxiliary space for bit-reversal and twiddle factors
//!
//! # References
//!
//! - [CT65] Cooley, Tukey. "An algorithm for the machine calculation of complex Fourier series" (1965)
//! - [Sei98] Seifert. "Using primitive roots for efficient computation of integer DFT" (1998)
//! - [GG13] Gathen, Gerhard. "Modern Computer Algebra" §8.2 (2013)

use super::{FiniteFieldError, FiniteFieldResult};

mod multiply;
mod transform;

pub use multiply::{multiply_auto, ntt_multiply};

/// Threshold for switching from naive to NTT multiplication
/// Empirically determined crossover point where NTT becomes faster
/// Lowered to 32 for better performance on medium-sized polynomials
pub const NTT_THRESHOLD: usize = 32;

/// Common NTT-friendly prime: 2013265921 = 15 * 2^27 + 1
/// Supports NTT up to degree 2^27 - 1
pub const NTT_PRIME_1: u64 = 2013265921;

/// Common NTT-friendly prime: 469762049 = 7 * 2^26 + 1
/// Supports NTT up to degree 2^26 - 1
pub const NTT_PRIME_2: u64 = 469762049;

/// Common NTT-friendly prime: 1004535809 = 479 * 2^21 + 1
/// Supports NTT up to degree 2^21 - 1
pub const NTT_PRIME_3: u64 = 1004535809;

/// Precomputed primitive roots for NTT-friendly primes
///
/// These are primitive 2^n-th roots of unity for each prime
pub(super) fn get_primitive_root(p: u64) -> FiniteFieldResult<u64> {
    match p {
        NTT_PRIME_1 => Ok(31),
        NTT_PRIME_2 => Ok(3),
        NTT_PRIME_3 => Ok(3),
        _ => Err(FiniteFieldError::InvalidEvaluationPoint {
            reason: format!(
                "Prime {} is not a known NTT-friendly prime. Use {}, {}, or {}",
                p, NTT_PRIME_1, NTT_PRIME_2, NTT_PRIME_3
            ),
        }),
    }
}

/// Compute next power of 2 greater than or equal to n
#[inline]
pub(super) fn next_power_of_2(n: usize) -> usize {
    if n == 0 {
        return 1;
    }
    let mut p = 1;
    while p < n {
        p <<= 1;
    }
    p
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_power_of_2() {
        assert_eq!(next_power_of_2(0), 1);
        assert_eq!(next_power_of_2(1), 1);
        assert_eq!(next_power_of_2(2), 2);
        assert_eq!(next_power_of_2(3), 4);
        assert_eq!(next_power_of_2(5), 8);
        assert_eq!(next_power_of_2(7), 8);
        assert_eq!(next_power_of_2(8), 8);
        assert_eq!(next_power_of_2(9), 16);
    }
}
