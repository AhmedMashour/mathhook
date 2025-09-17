//! Zippel's Modular GCD Algorithm
//!
//! Industrial-strength polynomial GCD using modular arithmetic.
//! This implements Zippel's probabilistic sparse polynomial GCD algorithm
//! with CRT reconstruction and verification.
//!
//! PURE NUMERIC: ALL functions work on `&[i64]`, `Vec<i64>`, and `HashMap<Vec<usize>, i64>`.
//! NO Expression usage in core algorithms.
//!
//! # Module Organization
//!
//! - `content`: Content extraction and primitive part computation (pure numeric)
//! - `trial_division`: Trial division verification (pure numeric)
//! - `sparse`: Sparsity analysis and sparse operations (pure numeric)
//! - `univariate`: Modular GCD for single-variable polynomials (pure numeric)
//! - `multivariate`: Multivariate GCD using Zippel's algorithm (pure numeric)
//! - `interpolation`: Lagrange interpolation for polynomial reconstruction
//! - `degree_bounds`: Degree bound computation
//! - `variable_order`: Variable ordering optimization
//! - `educational`: Educational explanations for algorithm steps
//! - `helpers`: Internal utility functions (constants, arithmetic, heuristic GCD)
//!
//! # References
//!
//! - `[Zippel79]` Zippel, R. "Probabilistic algorithms for sparse polynomials"
//! - `[Brown71]` Brown, W.S. "On Euclid's algorithm and the computation of polynomial GCDs"
//! - `[GCL92]` Geddes, Czapor, Labahn. "Algorithms for Computer Algebra", Ch. 7

mod content;
mod degree_bounds;
pub mod educational;
mod helpers;
mod interpolation;
mod multivariate;
mod sparse;
mod trial_division;
mod univariate;
mod variable_order;

// Re-export public API (pure numeric)
pub use content::primitive_part;
pub use degree_bounds::{
    compute_degree_bounds, content_multivar, extract_lc_gcd_multivar, primitive_part_multivar,
};
pub use helpers::{heuristic_gcd, integer_gcd};
pub use interpolation::{lagrange_interpolation, lagrange_interpolation_multivar};
pub use multivariate::{multivariate_gcd_zippel, MultiPoly, MultivarGcdResult, MultivariateConfig};
pub use sparse::{
    analyze_sparsity, is_sparse, polynomial_multiply_coeffs, sparse_multiply, SparsityInfo,
};
pub use trial_division::{trial_divide, verify_gcd_candidate, TrialDivisionResult};
pub use univariate::modular_gcd_univariate;
pub use variable_order::order_variables_by_degree;

/// Internal helper functions re-exported for backward compatibility.
///
/// **Deprecation Notice**: These functions are considered internal implementation details.
/// They will be made private in the next major version (2.0).
/// Do not rely on these in new code - use the high-level APIs instead.
///
/// - Use `modular_gcd_univariate` for univariate GCD with cofactors
/// - Use `primitive_part` for content operations
/// - Use `verify_gcd_candidate` for trial division verification
/// - Use `heuristic_gcd` for fast path GCD attempts
pub mod helpers_compat {
    pub use super::helpers::arithmetic::{evaluate_polynomial_at, extract_lc_gcd, mod_inverse};
    pub use super::helpers::{
        crt_combine_u128, extended_gcd_i64, heuristic_gcd, mod_positive, symmetric_mod,
        HEURISTIC_GCD_RETRIES, LARGE_PRIMES, MAX_CRT_ITERATIONS, MAX_EVALUATION_POINTS,
    };
    pub use super::sparse::SPARSITY_THRESHOLD;
    pub use super::trial_division::compute_cofactor;
}

// Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_modular_gcd_univariate_basic() {
        // f = x^2 - 1, g = x - 1
        let f = vec![-1, 0, 1];
        let g = vec![-1, 1];
        let result = modular_gcd_univariate(&f, &g);
        assert!(result.is_ok());
        let (gcd, _, _) = result.unwrap();
        // GCD should be x - 1
        assert_eq!(gcd.len(), 2);
    }

    #[test]
    fn test_primitive_part_basic() {
        // 6x + 12 = 6(x + 2)
        let coeffs = vec![12, 6];
        let (content, prim) = primitive_part(&coeffs);
        assert_eq!(content, 6);
        assert_eq!(prim, vec![2, 1]);
    }

    #[test]
    fn test_verify_gcd_basic() {
        // f = x^2 - 1, g = x - 1, h = x - 1
        let f = vec![-1, 0, 1];
        let g = vec![-1, 1];
        let h = vec![-1, 1];
        assert!(verify_gcd_candidate(&f, &g, &h));

        let h_wrong = vec![1, 1];
        assert!(!verify_gcd_candidate(&f, &g, &h_wrong));
    }

    #[test]
    fn test_polynomial_multiply_coeffs() {
        // (x + 1) * (x + 2) = x² + 3x + 2
        let f = vec![1, 1];
        let g = vec![2, 1];
        let product = polynomial_multiply_coeffs(&f, &g);
        assert_eq!(product, vec![2, 3, 1]);
    }

    #[test]
    fn test_multivariate_simple() {
        use std::collections::HashMap;

        // f = xy
        let mut f = HashMap::new();
        f.insert(vec![1, 1], 1);

        // g = xy
        let g = f.clone();

        let config = MultivariateConfig::default();
        let result = multivariate_gcd_zippel(&f, &g, 2, &config);
        assert!(result.is_ok());
    }
}
