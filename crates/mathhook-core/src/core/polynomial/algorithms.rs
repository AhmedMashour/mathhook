//! Polynomial Algorithm Implementations
//!
//! This module contains the core polynomial algorithms:
//! - GCD (pure i64 only)
//! - Factorization (square-free, pure `Poly<T>`)
//! - Zippel modular GCD (industrial-strength, pure numeric)
//!
//! Expression-based GCD operations MOVED TO ALGEBRA LAYER:
//! - polynomial_gcd, univariate_gcd → `algebra::gcd`
//! - resultant, discriminant → `algebra::polynomial_advanced::AdvancedPolynomial`
//! - content extraction, factor_numeric → `algebra::polynomial_advanced::AdvancedPolynomial`

mod division;
mod factorization;
mod gcd;
mod resultant;
pub mod zippel_gcd;

// Re-export i64 GCD only
pub use gcd::integer_gcd;

// Expression-based GCD re-exported from algebra for backward compatibility
pub use crate::algebra::gcd::{polynomial_gcd, univariate_gcd, univariate_gcd_modular};

// Re-export Zippel modular GCD (low-level access, pure numeric)
pub use zippel_gcd::modular_gcd_univariate;

// Re-export content extraction functions (pure numeric)
pub use zippel_gcd::primitive_part;

// Re-export trial division verification
pub use zippel_gcd::{trial_divide, verify_gcd_candidate, TrialDivisionResult};

// Re-export sparse GCD optimization
pub use zippel_gcd::{analyze_sparsity, is_sparse, sparse_multiply, SparsityInfo};

// Re-export factorization (pure Poly<T>)
pub use factorization::square_free_factorization_poly;

// Re-export resultant (backward compatibility - moved to algebra)
pub use resultant::AdvancedPolynomial;
