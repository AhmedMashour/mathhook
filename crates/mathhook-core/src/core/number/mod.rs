//! Number type for exact arithmetic
//!
//! Supports three representations:
//! - Integer: Arbitrary precision integers (i64 with promotion to BigInt)
//! - Rational: Exact fractions (numerator/denominator as BigInt)
//! - Float: Floating-point approximations (f64)
//!
//! All arithmetic operations use checked arithmetic to detect overflow and
//! automatically promote to BigInt or Rational types when needed. Float operations
//! check for infinity and NaN to prevent silent error propagation.

mod arithmetic;
mod integer_ops;
mod types;

pub use types::Number;
