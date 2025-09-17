//! Gaussian quadrature integration
//!
//! Implements Gauss-Legendre quadrature for numerical integration.
//! Uses precomputed nodes and weights for high accuracy.
use super::{IntegrationConfig, IntegrationResult, NumericalIntegrator};
use crate::error::MathError;
/// Gaussian quadrature integrator using Gauss-Legendre nodes
pub struct GaussianQuadrature {
    order: usize,
}
impl GaussianQuadrature {
    /// Create a new Gaussian quadrature integrator
    ///
    /// # Arguments
    ///
    /// * `order` - Number of quadrature points (2, 3, 4, or 5)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::calculus::integrals::numerical::GaussianQuadrature;
    ///
    /// let integrator = GaussianQuadrature::new(5);
    /// ```
    pub fn new(order: usize) -> Self {
        assert!(
            (2..=5).contains(&order),
            "Gaussian quadrature order must be between 2 and 5"
        );
        Self { order }
    }
    /// Get Gauss-Legendre nodes and weights for the specified order
    fn get_nodes_and_weights(&self) -> (&[f64], &[f64]) {
        match self.order {
            2 => (&GAUSS_NODES_2, &GAUSS_WEIGHTS_2),
            3 => (&GAUSS_NODES_3, &GAUSS_WEIGHTS_3),
            4 => (&GAUSS_NODES_4, &GAUSS_WEIGHTS_4),
            5 => (&GAUSS_NODES_5, &GAUSS_WEIGHTS_5),
            _ => panic!("Invalid Gaussian quadrature order"),
        }
    }
    /// Transform integration from [a, b] to [-1, 1]
    fn transform_point(&self, x: f64, a: f64, b: f64) -> f64 {
        0.5 * ((b - a) * x + (b + a))
    }
    /// Compute scaled Jacobian for interval transformation
    fn jacobian(&self, a: f64, b: f64) -> f64 {
        0.5 * (b - a)
    }
}
impl NumericalIntegrator for GaussianQuadrature {
    fn integrate<F>(
        &self,
        f: F,
        a: f64,
        b: f64,
        _config: &IntegrationConfig,
    ) -> Result<IntegrationResult, MathError>
    where
        F: Fn(f64) -> f64,
    {
        if a >= b {
            return Err(MathError::InvalidInterval { lower: a, upper: b });
        }
        let (nodes, weights) = self.get_nodes_and_weights();
        let jac = self.jacobian(a, b);
        let mut sum = 0.0;
        for (node, weight) in nodes.iter().zip(weights.iter()) {
            let x = self.transform_point(*node, a, b);
            sum += weight * f(x);
        }
        let value = jac * sum;
        let error_estimate = self.estimate_error(&f, a, b, value);
        Ok(IntegrationResult {
            value,
            error_estimate,
            iterations: 1,
            subdivisions: 1,
        })
    }
}
impl GaussianQuadrature {
    /// Estimate error using Richardson extrapolation
    fn estimate_error<F>(&self, f: &F, a: f64, b: f64, full_value: f64) -> f64
    where
        F: Fn(f64) -> f64,
    {
        let mid = (a + b) / 2.0;
        let (nodes, weights) = self.get_nodes_and_weights();
        let jac_left = self.jacobian(a, mid);
        let mut sum_left = 0.0;
        for (node, weight) in nodes.iter().zip(weights.iter()) {
            let x = self.transform_point(*node, a, mid);
            sum_left += weight * f(x);
        }
        let value_left = jac_left * sum_left;
        let jac_right = self.jacobian(mid, b);
        let mut sum_right = 0.0;
        for (node, weight) in nodes.iter().zip(weights.iter()) {
            let x = self.transform_point(*node, mid, b);
            sum_right += weight * f(x);
        }
        let value_right = jac_right * sum_right;
        let split_value = value_left + value_right;
        (split_value - full_value).abs() / 15.0
    }
}
const GAUSS_NODES_2: [f64; 2] = [-0.5773502691896257, 0.5773502691896257];
const GAUSS_WEIGHTS_2: [f64; 2] = [1.0, 1.0];
const GAUSS_NODES_3: [f64; 3] = [-0.7745966692414834, 0.0, 0.7745966692414834];
const GAUSS_WEIGHTS_3: [f64; 3] = [0.5555555555555556, 0.8888888888888888, 0.5555555555555556];
const GAUSS_NODES_4: [f64; 4] = [
    -0.8611363115940526,
    -0.3399810435848563,
    0.3399810435848563,
    0.8611363115940526,
];
const GAUSS_WEIGHTS_4: [f64; 4] = [
    0.3478548451374538,
    0.6521451548625461,
    0.6521451548625461,
    0.3478548451374538,
];
const GAUSS_NODES_5: [f64; 5] = [
    -0.906_179_845_938_664,
    -0.538_469_310_105_683,
    0.0,
    0.538_469_310_105_683,
    0.906_179_845_938_664,
];
const GAUSS_WEIGHTS_5: [f64; 5] = [
    0.2369268850561891,
    0.4786286704993665,
    0.5688888888888889,
    0.4786286704993665,
    0.2369268850561891,
];
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_gaussian_quadrature_polynomial() {
        let integrator = GaussianQuadrature::new(3);
        let config = IntegrationConfig::default();
        let result = integrator.integrate(|x| x * x, 0.0, 1.0, &config).unwrap();
        assert!((result.value - 1.0 / 3.0).abs() < 1e-10);
    }
    #[test]
    fn test_gaussian_quadrature_sine() {
        let integrator = GaussianQuadrature::new(5);
        let config = IntegrationConfig::default();
        let result = integrator
            .integrate(|x| x.sin(), 0.0, std::f64::consts::PI, &config)
            .unwrap();
        assert!((result.value - 2.0).abs() < 1e-6);
    }
    #[test]
    fn test_gaussian_quadrature_exponential() {
        let integrator = GaussianQuadrature::new(5);
        let config = IntegrationConfig::default();
        let result = integrator
            .integrate(|x| x.exp(), 0.0, 1.0, &config)
            .unwrap();
        let expected = std::f64::consts::E - 1.0;
        assert!((result.value - expected).abs() < 1e-10);
    }
    #[test]
    fn test_gaussian_error_estimate() {
        let integrator = GaussianQuadrature::new(3);
        let config = IntegrationConfig::default();
        let result = integrator
            .integrate(|x| x * x * x, 0.0, 1.0, &config)
            .unwrap();
        assert!(result.error_estimate < 1e-10);
    }
    #[test]
    fn test_gaussian_invalid_interval() {
        let integrator = GaussianQuadrature::new(3);
        let config = IntegrationConfig::default();
        let result = integrator.integrate(|x| x, 1.0, 0.0, &config);
        assert!(result.is_err());
    }
}
