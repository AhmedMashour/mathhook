//! Unified Polynomial Module
//!
//! This module provides comprehensive polynomial operations with automatic classification,
//! smart dispatch, and optional educational explanations.
//!
//! # Architecture
//!
//! The polynomial module is designed around these principles:
//!
//! 1. **Automatic Classification**: Users don't need to manually wrap expressions.
//!    The system automatically detects polynomial structure and routes to optimized algorithms.
//!
//! 2. **Decomposed Traits**: Instead of a monolithic trait, functionality is split into:
//!    - `PolynomialClassification` - Type detection and variable extraction
//!    - `PolynomialProperties` - Degree, leading coefficient, content, primitive part
//!    - `PolynomialArithmetic` - Division operations
//!    - `PolynomialGcdOps` - GCD, LCM, cofactors
//!    - `PolynomialEducational` - Step-by-step explanations (opt-in)
//!
//! 3. **Side-Table Caching**: Expensive computations (degree, classification) are cached
//!    using thread-local LRU cache, preserving the 32-byte Expression size constraint.
//!
//! 4. **Smart Dispatch**: GCD and other functions check cheap cases
//!    (integers, zero, one, symbols) BEFORE expensive classification.
//!
//! 5. **Unified Dispatch**: The `dispatch` module automatically routes operations to
//!    the optimal `Poly<T>` implementation based on coefficient type analysis:
//!    - All integers → `IntPoly` (fastest)
//!    - Any rationals → `RationalPoly` (field operations)
//!    - Multivariate → symbolic fallback
//!
//! # Example
//!
//! ```rust,ignore
//! use mathhook_core::{expr, symbol};
//!
//! let x = symbol!(x);
//! let p1 = expr!(x^2 - 1);
//! let p2 = expr!(x - 1);
//!
//! // Automatic polynomial GCD - no wrapping needed!
//! let gcd = p1.gcd(&p2);  // Returns x - 1
//! ```
//!
//! # Submodules
//!
//! - `algorithms` - Core polynomial algorithms (division, GCD, factorization)
//! - `dispatch` - Unified dispatch to optimal `Poly<T>` implementations
//! - `educational` - Step-by-step explanation generation
//! - `groebner` - Groebner basis computation (migrated from algebra/groebner)
//! - `sparse_polynomial` - Efficient sparse polynomial representation
//! - `special_families` - Orthogonal polynomials (Legendre, Hermite, etc.)

pub mod algorithms;
mod arithmetic;
mod cache;
mod classification;
mod coefficients;
pub mod dispatch;
pub mod educational;
mod error;
pub mod finite_field;
mod gcd_ops;
pub mod groebner;
pub mod poly;
mod properties;
pub mod sparse_polynomial;
pub mod special_families;
pub mod traits;

pub use cache::{
    cache_stats, clear_cache, get_or_compute_intpoly, with_cache, CacheStats, CachedClassification,
    PolynomialCache,
};
pub use classification::PolynomialClassification;
pub use error::PolynomialError;
pub use properties::PolynomialProperties;

pub use arithmetic::PolynomialArithmetic;
pub use gcd_ops::PolynomialGcdOps;

pub use educational::PolynomialEducational;

pub use coefficients::{
    coefficient_at, coefficients_list, constant_term, extract_coefficient_map, is_monic,
};

pub use poly::{IntPoly, Poly, RationalPoly};

pub use traits::{EuclideanDomain, Field, Ring};

// Expression-based GCD operations imported from algebra
pub use crate::algebra::gcd::{polynomial_gcd, univariate_gcd};

// Pure i64 integer GCD
pub use algorithms::integer_gcd;

// Pure Poly<T> factorization
pub use algorithms::square_free_factorization_poly;

pub use sparse_polynomial::{
    expression_to_sparse_polynomial, sparse_polynomial_to_expression, Monomial, SparsePolynomial,
};
