//! Bisection method for root finding
//!
//! Implements the bisection method which guarantees convergence
//! for continuous functions with a sign change in the interval.
//! Uses interval halving to iteratively narrow down the root location.
//!
//! # Algorithm
//!
//! Given f(a) and f(b) with opposite signs:
//! 1. Compute midpoint: c = (a + b) / 2
//! 2. If f(c) has same sign as f(a), replace a with c
//! 3. Otherwise, replace b with c
//! 4. Repeat until |b - a| < tolerance
//!
//! # Convergence
//!
//! - Guaranteed convergence if f is continuous and f(a)*f(b) < 0
//! - Linear convergence rate: error halves each iteration
//! - Requires O(log2((b-a)/tolerance)) iterations
//!
//! # Tolerance Semantics
//!
//! The algorithm stops when EITHER:
//! - |f(c)| < tolerance (function value criterion)
//! - |b - a| / 2 < tolerance (bracket width criterion)
//!
//! The bracket width criterion guarantees the root is within
//! tolerance distance of the returned value.

use super::{RootFinder, RootFindingConfig, RootResult};
use crate::error::MathError;
use crate::expr;

/// Bisection method root finder
///
/// Guaranteed convergence method that requires an initial bracket [a, b]
/// where f(a) and f(b) have opposite signs.
pub struct BisectionMethod {
    /// Lower bound of initial bracket
    pub a: f64,
    /// Upper bound of initial bracket
    pub b: f64,
}

impl BisectionMethod {
    /// Create a new bisection method with initial bracket
    ///
    /// # Arguments
    ///
    /// * `a` - Lower bound of bracket
    /// * `b` - Upper bound of bracket
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::algebra::root_finding::BisectionMethod;
    ///
    /// let method = BisectionMethod::new(0.0, 2.0);
    /// ```
    pub fn new(a: f64, b: f64) -> Self {
        Self { a, b }
    }

    /// Check if the bracket is valid (function values have opposite signs)
    fn validate_bracket<F>(&self, f: &F) -> Result<(), MathError>
    where
        F: Fn(f64) -> f64,
    {
        let fa = f(self.a);
        let fb = f(self.b);

        if fa.is_nan() || fb.is_nan() {
            return Err(MathError::DomainError {
                operation: "bisection".to_owned(),
                value: expr!(x),
                reason: "Function evaluates to NaN at bracket endpoints".to_owned(),
            });
        }

        if fa * fb > 0.0 {
            return Err(MathError::ConvergenceFailed {
                reason: format!(
                    "Function values at bracket endpoints must have opposite signs: f({}) = {}, f({}) = {}",
                    self.a, fa, self.b, fb
                ),
            });
        }

        Ok(())
    }
}

impl RootFinder for BisectionMethod {
    fn find_root<F>(&self, f: F, config: &RootFindingConfig) -> Result<RootResult, MathError>
    where
        F: Fn(f64) -> f64,
    {
        self.validate_bracket(&f)?;

        let mut a = self.a;
        let mut b = self.b;
        let mut fa = f(a);

        for iteration in 0..config.max_iterations {
            let c = (a + b) / 2.0;
            let fc = f(c);

            // Check convergence: function value OR bracket width
            if fc.abs() < config.tolerance || (b - a).abs() / 2.0 < config.tolerance {
                return Ok(RootResult {
                    root: c,
                    iterations: iteration + 1,
                    function_value: fc,
                    converged: true,
                });
            }

            // Update bracket based on sign of f(c)
            if fa * fc < 0.0 {
                b = c;
            } else {
                a = c;
                fa = fc;
            }
        }

        // Max iterations reached - return best approximation with converged=false
        let final_c = (a + b) / 2.0;
        let final_fc = f(final_c);

        Ok(RootResult {
            root: final_c,
            iterations: config.max_iterations,
            function_value: final_fc,
            converged: false,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bisection_simple_linear() {
        let method = BisectionMethod::new(-1.0, 2.0);
        let config = RootFindingConfig::default();

        let result = method.find_root(|x| x - 1.0, &config).unwrap();

        // Primary: verify equation is satisfied
        assert!(result.function_value.abs() < config.tolerance);
        // Secondary: check expected value
        assert!((result.root - 1.0).abs() < 1e-9);
        assert!(result.converged);
    }

    #[test]
    fn test_bisection_quadratic() {
        let method = BisectionMethod::new(0.0, 3.0);
        let config = RootFindingConfig {
            tolerance: 1e-10,
            ..Default::default()
        };

        let result = method.find_root(|x| x * x - 2.0, &config).unwrap();

        // Primary: verify x² = 2
        let residual = (result.root * result.root - 2.0).abs();
        assert!(
            residual < 1e-9,
            "Solution doesn't satisfy x² = 2: residual = {}",
            residual
        );

        // Secondary: compare to sqrt(2)
        assert!((result.root - 2.0_f64.sqrt()).abs() < 1e-9);
        assert!(result.converged);
    }

    #[test]
    fn test_bisection_transcendental() {
        let method = BisectionMethod::new(0.0, 2.0);
        let config = RootFindingConfig {
            tolerance: 1e-10,
            ..Default::default()
        };

        let result = method.find_root(|x| x.cos() - x, &config).unwrap();

        // Primary: verify cos(x) = x
        let residual = (result.root.cos() - result.root).abs();
        assert!(
            residual < 1e-9,
            "Solution doesn't satisfy cos(x) = x: residual = {}",
            residual
        );

        // Secondary: verify it's in expected range
        // Reference: Dottie number ≈ 0.739085133215161
        assert!(result.root > 0.73_f64 && result.root < 0.75_f64);
        assert!(result.converged);
    }

    #[test]
    fn test_bisection_invalid_bracket() {
        let method = BisectionMethod::new(0.0, 1.0);
        let config = RootFindingConfig::default();

        // x² + 1 has no real roots
        let result = method.find_root(|x| x * x + 1.0, &config);
        assert!(result.is_err());
    }

    #[test]
    fn test_bisection_exact_root() {
        let method = BisectionMethod::new(-1.0, 1.0);
        let config = RootFindingConfig {
            tolerance: 1e-15,
            ..Default::default()
        };

        let result = method.find_root(|x| x, &config).unwrap();

        // Verify f(x) = x has root at 0
        assert!(result.root.abs() < 1e-14);
        assert!(result.function_value.abs() < 1e-14);
        assert!(result.converged);
    }

    #[test]
    fn test_bisection_cubic() {
        let method = BisectionMethod::new(0.0, 1.0);
        let config = RootFindingConfig::default();

        let result = method
            .find_root(|x| x * x * x + x * x - 1.0, &config)
            .unwrap();

        // Primary: verify x³ + x² - 1 = 0
        let residual = (result.root.powi(3) + result.root.powi(2) - 1.0).abs();
        assert!(
            residual < 1e-9,
            "Solution doesn't satisfy x³ + x² = 1: residual = {}",
            residual
        );

        // Secondary: verify it's in expected range
        assert!(result.root > 0.75_f64 && result.root < 0.76_f64);
        assert!(result.converged);
    }

    #[test]
    fn test_bisection_sine() {
        let method = BisectionMethod::new(3.0, 4.0);
        let config = RootFindingConfig::default();

        let result = method.find_root(|x| x.sin(), &config).unwrap();

        // Primary: verify sin(x) = 0
        let residual = result.root.sin().abs();
        assert!(
            residual < 1e-9,
            "Solution doesn't satisfy sin(x) = 0: residual = {}",
            residual
        );

        // Secondary: compare to π
        assert!((result.root - std::f64::consts::PI).abs() < 1e-9);
        assert!(result.converged);
    }

    #[test]
    fn test_bisection_exponential() {
        let method = BisectionMethod::new(-1.0, 1.0);
        let config = RootFindingConfig::default();

        let result = method.find_root(|x| x.exp() - 2.0, &config).unwrap();

        // Primary: verify e^x = 2
        let residual = (result.root.exp() - 2.0).abs();
        assert!(
            residual < 1e-9,
            "Solution doesn't satisfy e^x = 2: residual = {}",
            residual
        );

        // Secondary: compare to ln(2)
        assert!((result.root - 2.0_f64.ln()).abs() < 1e-9);
        assert!(result.converged);
    }

    #[test]
    fn test_bisection_multiple_roots_finds_one() {
        let method = BisectionMethod::new(-2.0, 2.0);
        let config = RootFindingConfig::default();

        // f(x) = x(x-1)(x+1) has roots at -1, 0, 1
        let result = method
            .find_root(|x| x * (x - 1.0) * (x + 1.0), &config)
            .unwrap();

        assert!(result.converged);

        // Primary: verify it's actually a root
        let residual = result.function_value.abs();
        assert!(residual < 1e-9, "Not a valid root: f(x) = {}", residual);

        // Secondary: verify it's one of the three roots
        let is_root = (result.root.abs() < 1e-9)
            || ((result.root - 1.0).abs() < 1e-9)
            || ((result.root + 1.0).abs() < 1e-9);
        assert!(is_root, "Root {} is not one of -1, 0, or 1", result.root);
    }

    #[test]
    fn test_bisection_convergence_rate() {
        let method = BisectionMethod::new(0.0, 2.0);
        let config = RootFindingConfig {
            tolerance: 1e-12,
            ..Default::default()
        };

        let result = method.find_root(|x| x * x - 2.0, &config).unwrap();

        // Bisection requires approximately log2((b-a)/tol) iterations
        // For [0, 2] with tol=1e-12: log2(2/1e-12) ≈ 41 iterations
        assert!(result.iterations > 0);
        assert!(
            result.iterations < 50,
            "Too many iterations: {}",
            result.iterations
        );
        assert!(result.converged);
    }

    #[test]
    fn test_bisection_near_discontinuity() {
        let method = BisectionMethod::new(-1.0, 1.0);
        let config = RootFindingConfig {
            tolerance: 1e-8,
            ..Default::default()
        };

        // Step function: -1 for x < 0, +1 for x ≥ 0
        let result = method
            .find_root(|x| if x < 0.0 { -1.0 } else { 1.0 }, &config)
            .unwrap();

        // The "root" is at the discontinuity
        assert!(result.root.abs() < 1e-7);
    }

    #[test]
    fn test_bisection_polynomial_with_close_roots() {
        let method = BisectionMethod::new(0.5, 1.5);
        let config = RootFindingConfig::default();

        // f(x) = (x-1)(x-2) has roots at 1 and 2
        let result = method
            .find_root(|x| (x - 1.0) * (x - 2.0), &config)
            .unwrap();

        assert!(result.converged);

        // Primary: verify it's a root
        let residual = result.function_value.abs();
        assert!(residual < 1e-9, "Not a valid root: f(x) = {}", residual);

        // Bracket [0.5, 1.5] should find root at x=1
        assert!((result.root - 1.0).abs() < 1e-9);
    }

    #[test]
    fn test_bisection_oscillatory_function() {
        let method = BisectionMethod::new(0.1, 0.5);
        let config = RootFindingConfig::default();

        let result = method.find_root(|x| (10.0 * x).sin(), &config).unwrap();

        assert!(result.converged);

        // Primary: verify sin(10x) = 0
        let residual = (10.0 * result.root).sin().abs();
        assert!(
            residual < 1e-9,
            "Solution doesn't satisfy sin(10x) = 0: residual = {}",
            residual
        );

        // Secondary: compare to π/10
        assert!((result.root - std::f64::consts::PI / 10.0).abs() < 1e-9);
    }

    #[test]
    fn test_bisection_tolerance_control() {
        let method = BisectionMethod::new(0.0, 2.0);

        let config_loose = RootFindingConfig {
            tolerance: 1e-4,
            ..Default::default()
        };
        let result_loose = method.find_root(|x| x * x - 2.0, &config_loose).unwrap();

        let config_tight = RootFindingConfig {
            tolerance: 1e-12,
            ..Default::default()
        };
        let result_tight = method.find_root(|x| x * x - 2.0, &config_tight).unwrap();

        // Tighter tolerance requires more iterations
        assert!(result_loose.iterations < result_tight.iterations);

        // Tighter tolerance produces more accurate result
        assert!(result_tight.function_value.abs() < result_loose.function_value.abs());
    }

    #[test]
    fn test_bisection_negative_interval() {
        let method = BisectionMethod::new(-3.0, -1.0);
        let config = RootFindingConfig::default();

        let result = method.find_root(|x| x + 2.0, &config).unwrap();

        // Primary: verify x + 2 = 0
        let residual = (result.root + 2.0).abs();
        assert!(
            residual < 1e-9,
            "Solution doesn't satisfy x = -2: residual = {}",
            residual
        );

        assert!(result.converged);
    }

    #[test]
    fn test_bisection_max_iterations_reached() {
        let method = BisectionMethod::new(0.0, 2.0);
        let config = RootFindingConfig {
            tolerance: 1e-15,
            max_iterations: 10, // Deliberately too few
            ..Default::default()
        };

        let result = method.find_root(|x| x * x - 2.0, &config).unwrap();

        // Should return non-converged result
        assert!(
            !result.converged,
            "Should not have converged with only 10 iterations"
        );
        assert_eq!(result.iterations, 10);

        // But should still be getting closer to the root
        assert!(result.root > 1.0 && result.root < 2.0);
        assert!(result.function_value.abs() < 1.0); // Better than initial bracket
    }

    #[test]
    fn test_bisection_function_value_convergence() {
        let method = BisectionMethod::new(0.0, 2.0);
        let config = RootFindingConfig {
            tolerance: 1e-10,
            ..Default::default()
        };

        let result = method.find_root(|x| x * x - 2.0, &config).unwrap();

        // When converged, function_value should be near zero
        assert!(result.converged);
        assert!(result.function_value.abs() < 1e-9);
    }

    #[test]
    fn test_bisection_bracket_width_convergence() {
        let method = BisectionMethod::new(1.0, 2.0);
        let config = RootFindingConfig {
            tolerance: 1e-6,
            ..Default::default()
        };

        // Use a function where f(c) might not get small, but bracket does
        let result = method.find_root(|x| x * x - 2.0, &config).unwrap();

        assert!(result.converged);
        // The bracket width criterion ensures root is within tolerance
        let sqrt2 = 2.0_f64.sqrt();
        assert!((result.root - sqrt2).abs() < config.tolerance);
    }
}
