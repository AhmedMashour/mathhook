//! Internal Helper Functions
//!
//! Low-level arithmetic and utility functions for Zippel GCD computation.

pub mod arithmetic;
mod heuristic;

pub use arithmetic::*;
pub use heuristic::*;

/// Large primes suitable for modular arithmetic
pub const LARGE_PRIMES: [u64; 20] = [
    2147483647, 2147483629, 2147483587, 2147483579, 2147483563, 2147483549, 2147483543, 2147483497,
    2147483489, 2147483477, 2147483423, 2147483399, 2147483353, 2147483323, 2147483269, 2147483249,
    2147483237, 2147483179, 2147483171, 2147483137,
];

/// Maximum iterations for CRT reconstruction before giving up
pub const MAX_CRT_ITERATIONS: usize = 50;

/// Maximum number of evaluation points for multivariate interpolation
pub const MAX_EVALUATION_POINTS: usize = 100;

/// Number of retries for heuristic GCD with different evaluation points
pub const HEURISTIC_GCD_RETRIES: usize = 3;
