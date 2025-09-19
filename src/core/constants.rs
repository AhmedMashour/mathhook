//! Mathematical constants enumeration

use serde::{Deserialize, Serialize};

/// Mathematical constants
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MathConstant {
    /// Pi (π)
    Pi,
    /// Euler's number (e)
    E,
    /// Imaginary unit (i)
    I,
    /// Positive infinity (∞)
    Infinity,
    /// Negative infinity (-∞)
    NegInfinity,
    /// Undefined/NaN
    Undefined,
    /// Golden ratio (φ)
    GoldenRatio,
    /// Euler-Mascheroni constant (γ)
    EulerGamma,
}
