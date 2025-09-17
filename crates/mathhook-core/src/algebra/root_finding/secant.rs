//! Secant method for root finding
//!
//! Implements the secant method which approximates the derivative
//! using two function evaluations. No analytical or numerical derivative required.
//!
//! # Algorithm
//!
//! Given two initial guesses x0 and x1:
//! x(n+1) = x(n) - f(x(n)) * (x(n) - x(n-1)) / (f(x(n)) - f(x(n-1)))
//!
//! # Convergence
//!
//! - Superlinear convergence rate: ≈ 1.618 (golden ratio)
//! - Faster than bisection, slower than Newton-Raphson
//! - No derivative calculation required
//! - Requires two initial guesses (not necessarily a bracket)

use super::{RootFinder, RootFindingConfig, RootResult};
use crate::error::MathError;

/// Secant method for root finding
pub struct SecantMethod {
    /// First initial guess
    pub x0: f64,
    /// Second initial guess
    pub x1: f64,
}

impl SecantMethod {
    /// Create a new secant method with two initial guesses
    ///
    /// # Arguments
    ///
    /// * `x0` - First initial guess
    /// * `x1` - Second initial guess (should be close to but different from x0)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::algebra::root_finding::SecantMethod;
    ///
    /// let method = SecantMethod::new(1.0, 2.0);
    /// ```
    pub fn new(x0: f64, x1: f64) -> Self {
        Self { x0, x1 }
    }
}

impl RootFinder for SecantMethod {
    fn find_root<F>(&self, f: F, config: &RootFindingConfig) -> Result<RootResult, MathError>
    where
        F: Fn(f64) -> f64,
    {
        let mut x_prev = self.x0;
        let mut x_curr = self.x1;
        let mut f_prev = f(x_prev);
        let mut f_curr = f(x_curr);

        for iteration in 0..config.max_iterations {
            if f_curr.abs() < config.tolerance {
                return Ok(RootResult {
                    root: x_curr,
                    iterations: iteration + 1,
                    function_value: f_curr,
                    converged: true,
                });
            }

            let denominator = f_curr - f_prev;
            if denominator.abs() < 1e-14 {
                return Err(MathError::ConvergenceFailed {
                    reason: format!(
                        "Denominator too small: f({}) - f({}) = {}",
                        x_curr, x_prev, denominator
                    ),
                });
            }

            let x_new = x_curr - f_curr * (x_curr - x_prev) / denominator;

            if x_new.is_nan() || x_new.is_infinite() {
                return Err(MathError::ConvergenceFailed {
                    reason: format!("Iteration produced invalid value: {}", x_new),
                });
            }

            let f_new = f(x_new);

            if (x_new - x_curr).abs() < config.tolerance * x_new.abs().max(1.0)
                && f_new.abs() < config.tolerance
            {
                return Ok(RootResult {
                    root: x_new,
                    iterations: iteration + 1,
                    function_value: f_new,
                    converged: true,
                });
            }

            x_prev = x_curr;
            f_prev = f_curr;
            x_curr = x_new;
            f_curr = f_new;
        }

        Err(MathError::MaxIterationsReached {
            max_iterations: config.max_iterations,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_secant_simple_linear() {
        let method = SecantMethod::new(0.0, 2.0);
        let config = RootFindingConfig::default();

        let result = method.find_root(|x| x - 1.0, &config).unwrap();

        assert!((result.root - 1.0).abs() < 1e-9);
        assert!(result.converged);
    }

    #[test]
    fn test_secant_quadratic() {
        let method = SecantMethod::new(1.0, 2.0);
        let config = RootFindingConfig::default();

        let result = method.find_root(|x| x * x - 2.0, &config).unwrap();

        assert!((result.root - 2.0_f64.sqrt()).abs() < 1e-10);
        assert!(result.converged);
    }

    #[test]
    fn test_secant_cubic() {
        let method = SecantMethod::new(-2.0, 0.0);
        let config = RootFindingConfig::default();

        let result = method
            .find_root(|x| x * x * x + x * x - 1.0, &config)
            .unwrap();

        let f_at_root = result.root.powi(3) + result.root.powi(2) - 1.0;
        assert!(f_at_root.abs() < 1e-9);
        assert!(result.converged);
    }

    #[test]
    fn test_secant_transcendental() {
        let method = SecantMethod::new(0.5, 1.0);
        let config = RootFindingConfig::default();

        let result = method.find_root(|x| x.cos() - x, &config).unwrap();

        let expected = 0.7390851332151607;
        assert!((result.root - expected).abs() < 1e-10);
        assert!(result.converged);
    }

    #[test]
    fn test_secant_exponential() {
        let method = SecantMethod::new(0.5, 1.0);
        let config = RootFindingConfig::default();

        let result = method.find_root(|x| x.exp() - 2.0, &config).unwrap();

        assert!((result.root - 2.0_f64.ln()).abs() < 1e-10);
        assert!(result.converged);
    }

    #[test]
    fn test_secant_sine() {
        let method = SecantMethod::new(3.0, 3.5);
        let config = RootFindingConfig::default();

        let result = method.find_root(|x| x.sin(), &config).unwrap();

        assert!((result.root - std::f64::consts::PI).abs() < 1e-10);
        assert!(result.converged);
    }

    #[test]
    fn test_secant_polynomial() {
        let method = SecantMethod::new(0.0, 1.0);
        let config = RootFindingConfig::default();

        let result = method
            .find_root(|x| x * x * x - 3.0 * x + 1.0, &config)
            .unwrap();

        let f_at_root = result.root.powi(3) - 3.0 * result.root + 1.0;
        assert!(f_at_root.abs() < 1e-9);
        assert!(result.converged);
    }

    #[test]
    fn test_secant_logarithmic() {
        let method = SecantMethod::new(2.0, 3.0);
        let config = RootFindingConfig::default();

        let result = method.find_root(|x| x.ln() - 1.0, &config).unwrap();

        assert!((result.root - std::f64::consts::E).abs() < 1e-10);
        assert!(result.converged);
    }

    #[test]
    fn test_secant_convergence_rate() {
        let method = SecantMethod::new(1.0, 2.0);
        let config = RootFindingConfig {
            tolerance: 1e-12,
            ..Default::default()
        };

        let result = method.find_root(|x| x * x - 2.0, &config).unwrap();

        assert!(result.iterations < 15);
        assert!((result.root - 2.0_f64.sqrt()).abs() < 1e-12);
    }

    #[test]
    fn test_secant_negative_root() {
        let method = SecantMethod::new(-2.0, -1.0);
        let config = RootFindingConfig::default();

        let result = method.find_root(|x| x * x - 2.0, &config).unwrap();

        assert!((result.root + 2.0_f64.sqrt()).abs() < 1e-10);
        assert!(result.converged);
    }

    #[test]
    fn test_secant_trigonometric_combination() {
        let method = SecantMethod::new(0.5, 1.5);
        let config = RootFindingConfig::default();

        let result = method
            .find_root(|x| x.sin() + x.cos() - 1.0, &config)
            .unwrap();

        let f_at_root = result.root.sin() + result.root.cos() - 1.0;
        assert!(f_at_root.abs() < 1e-9);
        assert!(result.converged);
    }

    #[test]
    fn test_secant_rational_function() {
        let method = SecantMethod::new(1.5, 2.5);
        let config = RootFindingConfig::default();

        let result = method
            .find_root(|x| (x * x - 4.0) / (x + 1.0), &config)
            .unwrap();

        assert!((result.root - 2.0).abs() < 1e-10 || (result.root + 2.0).abs() < 1e-10);
        assert!(result.converged);
    }

    #[test]
    fn test_secant_hyperbolic() {
        let method = SecantMethod::new(0.5, 1.5);
        let config = RootFindingConfig::default();

        let result = method.find_root(|x| x.sinh() - 1.0, &config).unwrap();

        assert!((result.root - 1.0_f64.asinh()).abs() < 1e-10);
        assert!(result.converged);
    }

    #[test]
    fn test_secant_close_initial_guesses() {
        let method = SecantMethod::new(1.4, 1.5);
        let config = RootFindingConfig::default();

        let result = method.find_root(|x| x * x - 2.0, &config).unwrap();

        assert!((result.root - 2.0_f64.sqrt()).abs() < 1e-10);
        assert!(result.converged);
    }

    #[test]
    fn test_secant_wide_initial_guesses() {
        let method = SecantMethod::new(0.5, 5.0);
        let config = RootFindingConfig::default();

        let result = method.find_root(|x| x * x - 2.0, &config).unwrap();

        assert!((result.root - 2.0_f64.sqrt()).abs() < 1e-10);
        assert!(result.converged);
    }

    #[test]
    fn test_secant_tolerance_control() {
        let method = SecantMethod::new(1.0, 2.0);

        let config_loose = RootFindingConfig {
            tolerance: 1e-6,
            ..Default::default()
        };
        let result_loose = method.find_root(|x| x * x - 2.0, &config_loose).unwrap();

        let config_tight = RootFindingConfig {
            tolerance: 1e-12,
            ..Default::default()
        };
        let result_tight = method.find_root(|x| x * x - 2.0, &config_tight).unwrap();

        assert!(result_tight.function_value.abs() < result_loose.function_value.abs());
    }
}
