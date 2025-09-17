//! Algebra module containing all mathematical operations

pub mod advanced_simplify;
pub mod collect;
pub mod expand;
pub mod factor;
pub mod gcd;
pub mod rational;
pub mod simplify;
pub mod zero_detection;
pub mod polynomial_advanced;

// Re-exports for easy access
pub use advanced_simplify::AdvancedSimplify;
pub use collect::Collect;
pub use expand::Expand;
pub use factor::Factor;
pub use gcd::PolynomialGcd;
pub use rational::RationalSimplify;
pub use simplify::Simplify;
pub use zero_detection::ZeroDetection;
pub use polynomial_advanced::{AdvancedPolynomial, PolynomialArithmetic};
