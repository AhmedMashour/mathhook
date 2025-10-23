//! Algebraic operations and traits for the hybrid API

pub mod advanced_simplify;
pub mod collect;
pub mod complex;
pub mod diagonal_matrix_tests;
pub mod equation_analyzer;
pub mod expand;
pub mod factor;
pub mod gcd;
pub mod matrix_correctness_tests;
pub mod polynomial_advanced;
pub mod polynomial_division;
pub mod rational;
pub mod solvers;
pub mod zero_detection;

// Re-export the actual traits from their modules
pub use advanced_simplify::AdvancedSimplify;
pub use collect::Collect;
pub use complex::ComplexOperations;
pub use expand::Expand;
pub use factor::Factor;
pub use gcd::PolynomialGcd;
pub use polynomial_advanced::AdvancedPolynomial;
pub use rational::RationalSimplify;
pub use zero_detection::ZeroDetection;

// Re-export polynomial division functions
pub use polynomial_division::{polynomial_div, polynomial_quo, polynomial_rem};

// Re-export solver types
pub use equation_analyzer::{EquationAnalyzer, EquationType, SmartEquationSolver};
pub use solvers::{EquationSolver, SolverResult as AlgebraSolverResult};
