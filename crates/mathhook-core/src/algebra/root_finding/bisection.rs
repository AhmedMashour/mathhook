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

use super::{RootFinder, RootFindingConfig, RootResult};
use crate::error::MathError;

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
                operation: "bisection".to_string(),
                value: crate::core::Expression::symbol(crate::core::Symbol::scalar("x")),
                reason: "Function evaluates to NaN at bracket endpoints".to_string(),
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
    fn find_root<F>(
        &self,
        f: F,
        config: &RootFindingConfig,
    ) -> Result<RootResult, MathError>
    where
        F: Fn(f64) -> f64,
    {
        self.validate_bracket(&f)?;

        let mut a = self.a;
        let mut b = self.b;
        let mut fa = f(a);
        let mut fb = f(b);

        for iteration in 0..config.max_iterations {
            let c = (a + b) / 2.0;
            let fc = f(c);

            if fc.abs() < config.tolerance || (b - a).abs() / 2.0 < config.tolerance {
                return Ok(RootResult {
                    root: c,
                    iterations: iteration + 1,
                    function_value: fc,
                    converged: true,
                });
            }

            if fa * fc < 0.0 {
                b = c;
                fb = fc;
            } else {
                a = c;
                fa = fc;
            }
        }

        let final_c = (a + b) / 2.0;
        Err(MathError::MaxIterationsReached {
            max_iterations: config.max_iterations,
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

        assert!((result.root - 1.0).abs() < 1e-9);
        assert!(result.converged);
        assert!(result.function_value.abs() < 1e-9);
    }

    #[test]
    fn test_bisection_quadratic() {
        let method = BisectionMethod::new(0.0, 3.0);
        let config = RootFindingConfig {
            tolerance: 1e-10,
            ..Default::default()
        };

        let result = method.find_root(|x| x * x - 2.0, &config).unwrap();

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

        let expected = 0.739085133215160641655312087673873;
        assert!((result.root - expected).abs() < 1e-9);
        assert!(result.converged);
    }

    #[test]
    fn test_bisection_invalid_bracket() {
        let method = BisectionMethod::new(0.0, 1.0);
        let config = RootFindingConfig::default();

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

        assert!(result.root.abs() < 1e-14);
        assert!(result.converged);
    }

    #[test]
    fn test_bisection_cubic() {
        let method = BisectionMethod::new(-2.0, 0.0);
        let config = RootFindingConfig::default();

        let result = method.find_root(|x| x * x * x + x * x - 1.0, &config).unwrap();

        let expected = -1.4655712318767680266567312093829;
        assert!((result.root - expected).abs() < 1e-9);
        assert!(result.converged);
    }

    #[test]
    fn test_bisection_sine() {
        let method = BisectionMethod::new(3.0, 4.0);
        let config = RootFindingConfig::default();

        let result = method.find_root(|x| x.sin(), &config).unwrap();

        assert!((result.root - std::f64::consts::PI).abs() < 1e-9);
        assert!(result.converged);
    }

    #[test]
    fn test_bisection_exponential() {
        let method = BisectionMethod::new(-1.0, 1.0);
        let config = RootFindingConfig::default();

        let result = method.find_root(|x| x.exp() - 2.0, &config).unwrap();

        let expected = 2.0_f64.ln();
        assert!((result.root - expected).abs() < 1e-9);
        assert!(result.converged);
    }

    #[test]
    fn test_bisection_multiple_roots_finds_one() {
        let method = BisectionMethod::new(-2.0, 2.0);
        let config = RootFindingConfig::default();

        let result = method.find_root(|x| x * (x - 1.0) * (x + 1.0), &config).unwrap();

        assert!(result.converged);
        assert!(
            (result.root.abs() < 1e-9)
                || ((result.root - 1.0).abs() < 1e-9)
                || ((result.root + 1.0).abs() < 1e-9)
        );
    }

    #[test]
    fn test_bisection_convergence_rate() {
        let method = BisectionMethod::new(0.0, 2.0);
        let config = RootFindingConfig {
            tolerance: 1e-12,
            ..Default::default()
        };

        let result = method.find_root(|x| x * x - 2.0, &config).unwrap();

        assert!(result.iterations > 0);
        assert!(result.iterations < 50);
    }

    #[test]
    fn test_bisection_near_discontinuity() {
        let method = BisectionMethod::new(-1.0, 1.0);
        let config = RootFindingConfig {
            tolerance: 1e-8,
            ..Default::default()
        };

        let result = method
            .find_root(|x| if x < 0.0 { -1.0 } else { 1.0 }, &config)
            .unwrap();

        assert!(result.root.abs() < 1e-7);
    }

    #[test]
    fn test_bisection_polynomial_with_close_roots() {
        let method = BisectionMethod::new(0.5, 2.5);
        let config = RootFindingConfig::default();

        let result = method
            .find_root(|x| (x - 1.0) * (x - 2.0), &config)
            .unwrap();

        assert!(result.converged);
        assert!(
            (result.root - 1.0).abs() < 1e-9 || (result.root - 2.0).abs() < 1e-9
        );
    }

    #[test]
    fn test_bisection_oscillatory_function() {
        let method = BisectionMethod::new(0.0, 1.0);
        let config = RootFindingConfig::default();

        let result = method.find_root(|x| (10.0 * x).sin(), &config).unwrap();

        assert!(result.converged);
        assert!(result.root.abs() < 1e-9 || (result.root - std::f64::consts::PI / 10.0).abs() < 1e-9);
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

        assert!(result_loose.iterations < result_tight.iterations);
        assert!(result_tight.function_value.abs() < result_loose.function_value.abs());
    }

    #[test]
    fn test_bisection_negative_interval() {
        let method = BisectionMethod::new(-3.0, -1.0);
        let config = RootFindingConfig::default();

        let result = method.find_root(|x| x + 2.0, &config).unwrap();

        assert!((result.root + 2.0).abs() < 1e-9);
        assert!(result.converged);
    }
}
