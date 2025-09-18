//! Algebra module containing all mathematical operations
//! Modern Rust module structure

// Individual algebra modules
pub mod advanced_simplify;
pub mod collect;
pub mod expand;
pub mod factor;
pub mod gcd;
pub mod polynomial_advanced;
pub mod rational;
pub mod simplify;
pub mod zero_detection;

// Solvers module (modern structure)
pub mod solvers;

// Smart equation analysis and dispatch
pub mod equation_analyzer;

// Re-exports for easy access
pub use advanced_simplify::AdvancedSimplify;
pub use collect::Collect;
pub use expand::Expand;
pub use factor::Factor;
pub use gcd::PolynomialGcd;
pub use polynomial_advanced::{AdvancedPolynomial, PolynomialArithmetic};
pub use rational::RationalSimplify;
pub use simplify::Simplify;
pub use solvers::{EquationSolver, LinearSolver, SolverResult};
pub use zero_detection::ZeroDetection;

// Smart equation analysis and dispatch
pub use equation_analyzer::{EquationAnalyzer, EquationType, SmartEquationSolver};
