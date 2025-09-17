//! Adaptive Simpson's rule integration
//!
//! Implements adaptive Simpson's rule with recursive subdivision
//! for automatic error control.

use super::{IntegrationConfig, IntegrationResult, NumericalIntegrator};
use crate::error::MathError;

/// Adaptive Simpson's rule integrator
pub struct AdaptiveSimpson;

/// Parameters for adaptive Simpson recursion
struct SimpsonRecursionParams<'a> {
    tolerance: f64,
    s_whole: f64,
    fa: f64,
    fb: f64,
    depth: usize,
    max_depth: usize,
    subdivisions: &'a mut usize,
}

impl AdaptiveSimpson {
    /// Create a new adaptive Simpson integrator
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::calculus::integrals::numerical::AdaptiveSimpson;
    ///
    /// let integrator = AdaptiveSimpson::new();
    /// ```
    pub fn new() -> Self {
        Self
    }

    /// Compute Simpson's rule estimate over [a, b]
    fn simpson_step<F>(&self, f: &F, a: f64, b: f64, fa: f64, fb: f64) -> f64
    where
        F: Fn(f64) -> f64,
    {
        let mid = (a + b) / 2.0;
        let fmid = f(mid);
        let h = (b - a) / 6.0;
        h * (fa + 4.0 * fmid + fb)
    }

    /// Recursive adaptive Simpson integration
    fn adaptive_simpson_recursive<F>(
        &self,
        f: &F,
        a: f64,
        b: f64,
        params: &mut SimpsonRecursionParams,
    ) -> Result<f64, MathError>
    where
        F: Fn(f64) -> f64,
    {
        if params.depth > params.max_depth {
            return Err(MathError::MaxIterationsReached {
                max_iterations: params.max_depth,
            });
        }

        let mid = (a + b) / 2.0;
        let fmid = f(mid);

        let s_left = self.simpson_step(f, a, mid, params.fa, fmid);
        let s_right = self.simpson_step(f, mid, b, fmid, params.fb);
        let s_split = s_left + s_right;

        let error = (s_split - params.s_whole).abs();

        if error < 15.0 * params.tolerance {
            *params.subdivisions += 1;
            Ok(s_split + error / 15.0)
        } else {
            *params.subdivisions += 1;

            let mut left_params = SimpsonRecursionParams {
                tolerance: params.tolerance / 2.0,
                s_whole: s_left,
                fa: params.fa,
                fb: fmid,
                depth: params.depth + 1,
                max_depth: params.max_depth,
                subdivisions: params.subdivisions,
            };

            let left_result = self.adaptive_simpson_recursive(f, a, mid, &mut left_params)?;

            let mut right_params = SimpsonRecursionParams {
                tolerance: params.tolerance / 2.0,
                s_whole: s_right,
                fa: fmid,
                fb: params.fb,
                depth: params.depth + 1,
                max_depth: params.max_depth,
                subdivisions: params.subdivisions,
            };

            let right_result = self.adaptive_simpson_recursive(f, mid, b, &mut right_params)?;

            Ok(left_result + right_result)
        }
    }
}

impl Default for AdaptiveSimpson {
    fn default() -> Self {
        Self::new()
    }
}

impl NumericalIntegrator for AdaptiveSimpson {
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
            return Err(MathError::InvalidInterval { lower: a, upper: b });
        }

        let fa = f(a);
        let fb = f(b);
        let s_whole = self.simpson_step(&f, a, b, fa, fb);

        let max_depth = (config.max_iterations as f64).log2().ceil() as usize;

        let mut subdivisions = 0;

        let mut params = SimpsonRecursionParams {
            tolerance: config.tolerance,
            s_whole,
            fa,
            fb,
            depth: 0,
            max_depth,
            subdivisions: &mut subdivisions,
        };

        let value = self.adaptive_simpson_recursive(&f, a, b, &mut params)?;

        let error_estimate = config.tolerance;

        Ok(IntegrationResult {
            value,
            error_estimate,
            iterations: subdivisions,
            subdivisions,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_adaptive_simpson_polynomial() {
        let integrator = AdaptiveSimpson::new();
        let config = IntegrationConfig {
            tolerance: 1e-10,
            ..Default::default()
        };

        let result = integrator.integrate(|x| x * x, 0.0, 1.0, &config).unwrap();

        assert!((result.value - 1.0 / 3.0).abs() < 1e-9);
    }

    #[test]
    fn test_adaptive_simpson_sine() {
        let integrator = AdaptiveSimpson::new();
        let config = IntegrationConfig {
            tolerance: 1e-8,
            ..Default::default()
        };

        let result = integrator
            .integrate(|x| x.sin(), 0.0, std::f64::consts::PI, &config)
            .unwrap();

        assert!((result.value - 2.0).abs() < 1e-7);
    }

    #[test]
    fn test_adaptive_simpson_exponential() {
        let integrator = AdaptiveSimpson::new();
        let config = IntegrationConfig {
            tolerance: 1e-10,
            ..Default::default()
        };

        let result = integrator
            .integrate(|x| x.exp(), 0.0, 1.0, &config)
            .unwrap();

        let expected = std::f64::consts::E - 1.0;
        assert!((result.value - expected).abs() < 1e-9);
    }

    #[test]
    fn test_adaptive_simpson_oscillatory() {
        let integrator = AdaptiveSimpson::new();
        let config = IntegrationConfig {
            tolerance: 1e-6,
            ..Default::default()
        };

        let result = integrator
            .integrate(|x| (10.0 * x).sin(), 0.0, 1.0, &config)
            .unwrap();

        let expected = (1.0 - (10.0_f64).cos()) / 10.0;
        assert!((result.value - expected).abs() < 1e-5);
    }

    #[test]
    fn test_adaptive_simpson_cubic() {
        let integrator = AdaptiveSimpson::new();
        let config = IntegrationConfig {
            tolerance: 1e-10,
            ..Default::default()
        };

        let result = integrator
            .integrate(|x| x * x * x, 0.0, 2.0, &config)
            .unwrap();

        assert!((result.value - 4.0).abs() < 1e-9);
    }

    #[test]
    fn test_adaptive_simpson_invalid_interval() {
        let integrator = AdaptiveSimpson::new();
        let config = IntegrationConfig::default();

        let result = integrator.integrate(|x| x, 1.0, 0.0, &config);
        assert!(result.is_err());
    }
}
