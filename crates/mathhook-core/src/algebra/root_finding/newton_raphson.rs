//! Newton-Raphson method for root finding
//!
//! Implements Newton-Raphson method using numerical differentiation.
//! Provides quadratic convergence near simple roots.
//!
//! # Algorithm
//!
//! Given initial guess x0:
//! x(n+1) = x(n) - f(x(n)) / f'(x(n))
//!
//! # Convergence
//!
//! - Quadratic convergence near simple roots (error squares each iteration)
//! - Requires good initial guess
//! - May fail for multiple roots or if derivative is zero
//! - Uses finite differences for derivative: f'(x) ≈ (f(x+h) - f(x))/h

use super::{RootFinder, RootFindingConfig, RootResult};
use crate::error::MathError;

/// Newton-Raphson method with numerical differentiation
pub struct NewtonRaphson {
    /// Initial guess for the root
    pub initial_guess: f64,
}

impl NewtonRaphson {
    /// Create a new Newton-Raphson method with initial guess
    ///
    /// # Arguments
    ///
    /// * `initial_guess` - Starting point for iteration
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::algebra::root_finding::NewtonRaphson;
    ///
    /// let method = NewtonRaphson::new(1.5);
    /// ```
    #[inline]
    pub fn new(initial_guess: f64) -> Self {
        Self { initial_guess }
    }

    /// Compute numerical derivative using central difference
    #[inline]
    fn numerical_derivative<F>(&self, f: &F, x: f64, h: f64) -> f64
    where
        F: Fn(f64) -> f64,
    {
        (f(x + h) - f(x - h)) / (2.0 * h)
    }

    /// Check for convergence stagnation
    #[inline]
    fn is_stagnant(&self, x_new: f64, x_old: f64, tolerance: f64) -> bool {
        (x_new - x_old).abs() < tolerance * x_old.abs().max(1.0)
    }
}

impl RootFinder for NewtonRaphson {
    fn find_root<F>(&self, f: F, config: &RootFindingConfig) -> Result<RootResult, MathError>
    where
        F: Fn(f64) -> f64,
    {
        let mut x = self.initial_guess;

        for iteration in 0..config.max_iterations {
            let fx = f(x);

            if fx.abs() < config.tolerance {
                return Ok(RootResult {
                    root: x,
                    iterations: iteration + 1,
                    function_value: fx,
                    converged: true,
                });
            }

            let fpx = self.numerical_derivative(&f, x, config.derivative_h);

            if fpx.abs() < 1e-14 {
                return Err(MathError::ConvergenceFailed {
                    reason: format!("Derivative too small at x = {}: f'(x) = {}", x, fpx),
                });
            }

            let x_new = x - fx / fpx;

            if x_new.is_nan() || x_new.is_infinite() {
                return Err(MathError::ConvergenceFailed {
                    reason: format!("Iteration produced invalid value: {}", x_new),
                });
            }

            if self.is_stagnant(x_new, x, config.tolerance) {
                let fx_new = f(x_new);
                if fx_new.abs() < config.tolerance {
                    return Ok(RootResult {
                        root: x_new,
                        iterations: iteration + 1,
                        function_value: fx_new,
                        converged: true,
                    });
                } else {
                    return Err(MathError::ConvergenceFailed {
                        reason: "Iteration stagnated without converging".to_string(),
                    });
                }
            }

            x = x_new;
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
    fn test_newton_simple_linear() {
        let method = NewtonRaphson::new(0.5);
        let config = RootFindingConfig::default();

        let result = method.find_root(|x| x - 1.0, &config).unwrap();

        assert!((result.root - 1.0).abs() < 1e-9);
        assert!(result.converged);
    }

    #[test]
    fn test_newton_quadratic() {
        let method = NewtonRaphson::new(1.5);
        let config = RootFindingConfig::default();

        let result = method.find_root(|x| x * x - 2.0, &config).unwrap();

        assert!((result.root - 2.0_f64.sqrt()).abs() < 1e-12);
        assert!(result.converged);
        assert!(result.iterations < 10);
    }

    #[test]
    fn test_newton_cubic() {
        let method = NewtonRaphson::new(-1.0);
        let config = RootFindingConfig::default();

        let result = method
            .find_root(|x| x * x * x + x * x - 1.0, &config)
            .unwrap();

        let f_at_root = result.root.powi(3) + result.root.powi(2) - 1.0;
        assert!(f_at_root.abs() < 1e-9);
        assert!(result.converged);
    }

    #[test]
    fn test_newton_transcendental() {
        let method = NewtonRaphson::new(0.5);
        let config = RootFindingConfig::default();

        let result = method.find_root(|x| x.cos() - x, &config).unwrap();

        let expected = 0.739085133215160641655312087673873;
        assert!((result.root - expected).abs() < 1e-10);
        assert!(result.converged);
    }

    #[test]
    fn test_newton_exponential() {
        let method = NewtonRaphson::new(0.5);
        let config = RootFindingConfig::default();

        let result = method.find_root(|x| x.exp() - 2.0, &config).unwrap();

        assert!((result.root - 2.0_f64.ln()).abs() < 1e-12);
        assert!(result.converged);
    }

    #[test]
    fn test_newton_sine() {
        let method = NewtonRaphson::new(3.0);
        let config = RootFindingConfig::default();

        let result = method.find_root(|x| x.sin(), &config).unwrap();

        assert!((result.root - std::f64::consts::PI).abs() < 1e-12);
        assert!(result.converged);
    }

    #[test]
    fn test_newton_polynomial() {
        let method = NewtonRaphson::new(1.5);
        let config = RootFindingConfig::default();

        let result = method
            .find_root(|x| x * x * x - 3.0 * x + 1.0, &config)
            .unwrap();

        let f_at_root = result.root.powi(3) - 3.0 * result.root + 1.0;
        assert!(f_at_root.abs() < 1e-9);
        assert!(result.converged);
    }

    #[test]
    fn test_newton_high_degree_polynomial() {
        let method = NewtonRaphson::new(1.2);
        let config = RootFindingConfig::default();

        let result = method.find_root(|x| x.powi(5) - x - 1.0, &config).unwrap();

        let f_at_root = result.root.powi(5) - result.root - 1.0;
        assert!(f_at_root.abs() < 1e-9);
        assert!(result.converged);
    }

    #[test]
    fn test_newton_zero_derivative_fails() {
        let method = NewtonRaphson::new(0.0);
        let config = RootFindingConfig::default();

        let result = method.find_root(|x| x * x, &config);

        assert!(result.is_err());
    }

    #[test]
    fn test_newton_fast_convergence() {
        let method = NewtonRaphson::new(1.5);
        let config = RootFindingConfig {
            tolerance: 1e-14,
            ..Default::default()
        };

        let result = method.find_root(|x| x * x - 2.0, &config).unwrap();

        assert!(result.iterations < 6);
        assert!((result.root - 2.0_f64.sqrt()).abs() < 1e-14);
    }

    #[test]
    fn test_newton_different_initial_guesses() {
        let config = RootFindingConfig::default();

        let guesses = vec![0.5, 1.0, 1.5, 2.0];
        for guess in guesses {
            let method = NewtonRaphson::new(guess);
            let result = method.find_root(|x| x * x - 2.0, &config).unwrap();

            assert!((result.root - 2.0_f64.sqrt()).abs() < 1e-10);
        }
    }

    #[test]
    fn test_newton_negative_root() {
        let method = NewtonRaphson::new(-1.5);
        let config = RootFindingConfig::default();

        let result = method.find_root(|x| x * x - 2.0, &config).unwrap();

        assert!((result.root + 2.0_f64.sqrt()).abs() < 1e-10);
        assert!(result.converged);
    }

    #[test]
    fn test_newton_logarithmic() {
        let method = NewtonRaphson::new(2.0);
        let config = RootFindingConfig::default();

        let result = method.find_root(|x| x.ln() - 1.0, &config).unwrap();

        assert!((result.root - std::f64::consts::E).abs() < 1e-10);
        assert!(result.converged);
    }

    #[test]
    fn test_newton_trigonometric_combination() {
        let method = NewtonRaphson::new(1.0);
        let config = RootFindingConfig::default();

        let result = method
            .find_root(|x| x.sin() + x.cos() - 1.0, &config)
            .unwrap();

        let f_at_root = result.root.sin() + result.root.cos() - 1.0;
        assert!(f_at_root.abs() < 1e-9);
        assert!(result.converged);
    }

    #[test]
    fn test_newton_rational_function() {
        let method = NewtonRaphson::new(2.0);
        let config = RootFindingConfig::default();

        let result = method
            .find_root(|x| (x * x - 4.0) / (x + 1.0), &config)
            .unwrap();

        assert!((result.root - 2.0).abs() < 1e-10 || (result.root + 2.0).abs() < 1e-10);
        assert!(result.converged);
    }

    #[test]
    fn test_newton_composite_function() {
        let method = NewtonRaphson::new(1.5);
        let config = RootFindingConfig::default();

        let result = method.find_root(|x| (x * x).sin() - 0.5, &config).unwrap();

        let f_at_root = (result.root * result.root).sin() - 0.5;
        assert!(f_at_root.abs() < 1e-9);
        assert!(result.converged);
    }

    #[test]
    fn test_newton_near_zero() {
        let method = NewtonRaphson::new(0.1);
        let config = RootFindingConfig::default();

        let result = method.find_root(|x| x * x * x, &config).unwrap();

        assert!(result.root.abs() < 1e-9);
        assert!(result.converged);
    }

    #[test]
    fn test_newton_hyperbolic() {
        let method = NewtonRaphson::new(1.0);
        let config = RootFindingConfig::default();

        let result = method.find_root(|x| x.sinh() - 1.0, &config).unwrap();

        assert!((result.root - 1.0_f64.asinh()).abs() < 1e-10);
        assert!(result.converged);
    }

    #[test]
    fn test_newton_inverse_function() {
        let method = NewtonRaphson::new(0.5);
        let config = RootFindingConfig::default();

        let result = method.find_root(|x| 1.0 / x - 2.0, &config).unwrap();

        assert!((result.root - 0.5).abs() < 1e-10);
        assert!(result.converged);
    }

    #[test]
    fn test_newton_tolerance_control() {
        let method = NewtonRaphson::new(1.5);

        let config_loose = RootFindingConfig {
            tolerance: 1e-6,
            ..Default::default()
        };
        let result_loose = method.find_root(|x| x * x - 2.0, &config_loose).unwrap();

        let config_tight = RootFindingConfig {
            tolerance: 1e-14,
            ..Default::default()
        };
        let result_tight = method.find_root(|x| x * x - 2.0, &config_tight).unwrap();

        assert!(result_tight.function_value.abs() < result_loose.function_value.abs());
    }
}
