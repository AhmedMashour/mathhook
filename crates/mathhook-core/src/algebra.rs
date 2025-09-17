//! Algebraic operations and traits for the hybrid API

pub mod advanced_simplify;
pub mod collect;
pub mod complex;
pub mod diagonal_matrix_tests;
pub mod equation_analyzer;
pub mod expand;
pub mod factor;
pub mod gcd;
pub mod groebner;
pub mod matrix_correctness_tests;
pub mod multivariate_gcd;
pub mod polynomial_advanced;
pub mod polynomial_division;
pub mod rational;
pub mod root_finding;
pub mod simplification;
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

// Re-export Expression-based GCD functions (primary location)
pub use gcd::{polynomial_gcd, univariate_gcd, univariate_gcd_modular};

// Re-export solver types
pub use equation_analyzer::{EquationAnalyzer, EquationType, SmartEquationSolver};
pub use solvers::{EquationSolver, SolverResult as AlgebraSolverResult};

// Re-export Gröbner basis types and functions
pub use groebner::{
    buchberger_algorithm, poly_reduce, poly_reduce_completely, s_polynomial, GroebnerBasis,
    MonomialOrder, MonomialOrdering,
};

// Re-export simplification system
pub use simplification::{
    get_simplification_registry, SimplificationRegistry, SimplificationStrategy,
    SIMPLIFICATION_REGISTRY,
};

// Re-export polynomial traits and ops
pub use crate::core::polynomial::finite_field::{
    is_prime, FiniteFieldError, FiniteFieldResult, PolyZp, Zp,
};
pub use crate::core::polynomial::poly::IntPoly;
pub use multivariate_gcd::multivariate_gcd;
