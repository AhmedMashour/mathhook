//! Root-finding algorithms
//!
//! Provides numerical methods for finding roots (zeros) of functions.
//! All methods work with closures for maximum flexibility.

pub mod bisection;
pub mod newton_raphson;
pub mod secant;

pub use bisection::BisectionMethod;
pub use newton_raphson::NewtonRaphson;
pub use secant::SecantMethod;

use crate::error::MathError;

/// Configuration for root-finding algorithms
#[derive(Debug, Clone)]
pub struct RootFindingConfig {
    pub tolerance: f64,
    pub max_iterations: usize,
    pub derivative_h: f64,
}

impl Default for RootFindingConfig {
    fn default() -> Self {
        Self {
            tolerance: 1e-10,
            max_iterations: 1000,
            derivative_h: 1e-8,
        }
    }
}

/// Result of root-finding operation
#[derive(Debug, Clone)]
pub struct RootResult {
    pub root: f64,
    pub iterations: usize,
    pub function_value: f64,
    pub converged: bool,
}

/// Trait for root-finding methods
pub trait RootFinder {
    /// Find a root of the given function
    ///
    /// # Arguments
    ///
    /// * `f` - Function to find root of
    /// * `config` - Root-finding configuration
    ///
    /// # Returns
    ///
    /// Root result with convergence information
    fn find_root<F>(&self, f: F, config: &RootFindingConfig) -> Result<RootResult, MathError>
    where
        F: Fn(f64) -> f64;
}
