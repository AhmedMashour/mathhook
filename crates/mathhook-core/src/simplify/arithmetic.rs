//! Arithmetic operation simplification
//!
//! Handles simplification of basic arithmetic operations: addition, multiplication, and powers.
//! Implements ultra-fast paths for common cases while maintaining mathematical correctness.

mod addition;
mod helpers;
mod multiplication;
mod power;

pub use addition::simplify_addition;
pub use multiplication::simplify_multiplication;
pub use power::simplify_power;

use super::Simplify;
