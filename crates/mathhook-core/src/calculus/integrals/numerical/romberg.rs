//! Romberg integration
//!
//! Implements Romberg integration using Richardson extrapolation
//! on trapezoidal rule estimates for high-order accuracy.

use super::{IntegrationConfig, IntegrationResult, NumericalIntegrator};
use crate::error::MathError;

/// Romberg integration using Richardson extrapolation
pub struct RombergIntegration {
    max_order: usize,
}

impl RombergIntegration {
    /// Create a new Romberg integrator
    ///
    /// # Arguments
    ///
    /// * `max_order` - Maximum order of extrapolation (typically 5-10)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::calculus::integrals::numerical::RombergIntegration;
    ///
    /// let integrator = RombergIntegration::new(8);
    /// ```
    pub fn new(max_order: usize) -> Self {
        Self { max_order }
    }

    /// Compute trapezoidal rule with 2^k subdivisions
    fn trapezoidal_refinement<F>(&self, f: &F, a: f64, b: f64, k: usize, prev: f64) -> f64
    where
        F: Fn(f64) -> f64,
    {
        if k == 0 {
            return 0.5 * (b - a) * (f(a) + f(b));
        }

        let n = 1_usize << (k - 1);
        let h = (b - a) / (n as f64);

        let mut sum = 0.0;
        for i in 0..n {
            let x = a + (2 * i + 1) as f64 * h / 2.0;
            sum += f(x);
        }

        0.5 * prev + h * sum / 2.0
    }

    /// Perform Richardson extrapolation on the Romberg tableau
    fn richardson_extrapolation(&self, tableau: &[Vec<f64>], row: usize, col: usize) -> f64 {
        let r1 = tableau[row][col - 1];
        let r2 = tableau[row - 1][col - 1];
        let power = 4_f64.powi(col as i32);

        (power * r1 - r2) / (power - 1.0)
    }
}

impl NumericalIntegrator for RombergIntegration {
    fn integrate<F>(
        &self,
        f: F,
        a: f64,
        b: f64,
        config: &IntegrationConfig,
    ) -> Result<IntegrationResult, MathError>
    where
        F: Fn(f64) -> f64,
    {
        if a >= b {
            return Err(MathError::InvalidInterval {
                lower: a,
                upper: b,
            });
        }

        let max_iterations = self.max_order.min(config.max_iterations);
        let mut tableau = vec![vec![0.0; max_iterations]; max_iterations];

        tableau[0][0] = self.trapezoidal_refinement(&f, a, b, 0, 0.0);

        for i in 1..max_iterations {
            tableau[i][0] = self.trapezoidal_refinement(&f, a, b, i, tableau[i - 1][0]);

            for j in 1..=i {
                tableau[i][j] = self.richardson_extrapolation(&tableau, i, j);
            }

            if i > 1 {
                let error = (tableau[i][i] - tableau[i - 1][i - 1]).abs();
                if error < config.tolerance {
                    return Ok(IntegrationResult {
                        value: tableau[i][i],
                        error_estimate: error,
                        iterations: i + 1,
                        subdivisions: 1 << i,
                    });
                }
            }
        }

        let final_value = tableau[max_iterations - 1][max_iterations - 1];
        let error_estimate = if max_iterations > 1 {
            (final_value - tableau[max_iterations - 2][max_iterations - 2]).abs()
        } else {
            0.0
        };

        Ok(IntegrationResult {
            value: final_value,
            error_estimate,
            iterations: max_iterations,
            subdivisions: 1 << (max_iterations - 1),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_romberg_polynomial() {
        let integrator = RombergIntegration::new(8);
        let config = IntegrationConfig {
            tolerance: 1e-12,
            ..Default::default()
        };

        let result = integrator
            .integrate(|x| x * x, 0.0, 1.0, &config)
            .unwrap();

        assert!((result.value - 1.0 / 3.0).abs() < 1e-11);
    }

    #[test]
    fn test_romberg_sine() {
        let integrator = RombergIntegration::new(10);
        let config = IntegrationConfig {
            tolerance: 1e-10,
            ..Default::default()
        };

        let result = integrator
            .integrate(|x| x.sin(), 0.0, std::f64::consts::PI, &config)
            .unwrap();

        assert!((result.value - 2.0).abs() < 1e-9);
    }

    #[test]
    fn test_romberg_exponential() {
        let integrator = RombergIntegration::new(8);
        let config = IntegrationConfig {
            tolerance: 1e-12,
            ..Default::default()
        };

        let result = integrator
            .integrate(|x| x.exp(), 0.0, 1.0, &config)
            .unwrap();

        let expected = std::f64::consts::E - 1.0;
        assert!((result.value - expected).abs() < 1e-11);
    }

    #[test]
    fn test_romberg_convergence() {
        let integrator = RombergIntegration::new(10);
        let config = IntegrationConfig {
            tolerance: 1e-10,
            ..Default::default()
        };

        let result = integrator
            .integrate(|x| x * x * x * x, 0.0, 1.0, &config)
            .unwrap();

        assert!((result.value - 0.2).abs() < 1e-10);
        assert!(result.error_estimate < 1e-10);
    }

    #[test]
    fn test_romberg_high_accuracy() {
        let integrator = RombergIntegration::new(12);
        let config = IntegrationConfig {
            tolerance: 1e-14,
            ..Default::default()
        };

        let result = integrator
            .integrate(|x| (x * std::f64::consts::PI).cos(), 0.0, 1.0, &config)
            .unwrap();

        let expected = 1.0 / std::f64::consts::PI * (std::f64::consts::PI).sin();
        assert!((result.value - expected).abs() < 1e-12);
    }

    #[test]
    fn test_romberg_invalid_interval() {
        let integrator = RombergIntegration::new(8);
        let config = IntegrationConfig::default();

        let result = integrator.integrate(|x| x, 1.0, 0.0, &config);
        assert!(result.is_err());
    }
}
