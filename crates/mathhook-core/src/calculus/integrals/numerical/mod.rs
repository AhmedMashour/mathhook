//! Numerical integration methods
//!
//! Provides numerical quadrature algorithms for approximate evaluation
//! of definite integrals when symbolic integration is not feasible.

pub mod gaussian;
pub mod simpson;
pub mod romberg;

pub use gaussian::GaussianQuadrature;
pub use simpson::AdaptiveSimpson;
pub use romberg::RombergIntegration;

use crate::core::Expression;
use crate::error::MathError;

/// Configuration for numerical integrators
#[derive(Debug, Clone)]
pub struct IntegrationConfig {
    pub tolerance: f64,
    pub max_iterations: usize,
    pub min_subdivisions: usize,
}

impl Default for IntegrationConfig {
    fn default() -> Self {
        Self {
            tolerance: 1e-10,
            max_iterations: 1000,
            min_subdivisions: 1,
        }
    }
}

/// Result of numerical integration
#[derive(Debug, Clone)]
pub struct IntegrationResult {
    pub value: f64,
    pub error_estimate: f64,
    pub iterations: usize,
    pub subdivisions: usize,
}

/// Trait for numerical integrators
pub trait NumericalIntegrator {
    /// Integrate a function over an interval
    ///
    /// # Arguments
    ///
    /// * `f` - Function to integrate (as closure)
    /// * `a` - Lower bound of integration
    /// * `b` - Upper bound of integration
    /// * `config` - Integration configuration
    ///
    /// # Returns
    ///
    /// Integration result with value and error estimate
    fn integrate<F>(
        &self,
        f: F,
        a: f64,
        b: f64,
        config: &IntegrationConfig,
    ) -> Result<IntegrationResult, MathError>
    where
        F: Fn(f64) -> f64;
}
