//! Mathematical constants

use serde::{Deserialize, Serialize};

/// Common mathematical constants
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum MathConstant {
    Pi,
    E,
    I,
    Infinity,
    NegativeInfinity,
    Undefined,
    GoldenRatio,
    EulerGamma,
}

impl MathConstant {
    /// Get the approximate floating-point value of the constant
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::MathConstant;
    ///
    /// assert!((MathConstant::Pi.to_f64() - std::f64::consts::PI).abs() < 1e-10);
    /// assert!((MathConstant::E.to_f64() - std::f64::consts::E).abs() < 1e-10);
    /// ```
    pub fn to_f64(self) -> f64 {
        match self {
            MathConstant::Pi => std::f64::consts::PI,
            MathConstant::E => std::f64::consts::E,
            MathConstant::I => f64::NAN,
            MathConstant::Infinity => f64::INFINITY,
            MathConstant::NegativeInfinity => f64::NEG_INFINITY,
            MathConstant::Undefined => f64::NAN,
            MathConstant::GoldenRatio => 1.618033988749895,
            MathConstant::EulerGamma => 0.5772156649015329,
        }
    }
}
