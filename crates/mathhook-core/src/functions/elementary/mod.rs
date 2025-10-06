//! 📐 Elementary Function Intelligence
//!
//! Dedicated module for elementary mathematical functions (sin, cos, exp, ln, etc.)
//! with complete mathematical properties, derivatives, and educational explanations.

pub mod exponential;
pub mod hyperbolic;
pub mod logarithmic;
pub mod trigonometric;

use crate::functions::properties::FunctionProperties;
use std::collections::HashMap;

/// Elementary Function Intelligence Registry
///
/// Manages mathematical intelligence for all elementary functions
/// with proper modular separation by function family.
pub struct ElementaryIntelligence {
    /// Trigonometric functions (sin, cos, tan, etc.)
    trigonometric: trigonometric::TrigonometricIntelligence,

    /// Exponential functions (exp, etc.)
    exponential: exponential::ExponentialIntelligence,

    /// Logarithmic functions (ln, log, etc.)
    logarithmic: logarithmic::LogarithmicIntelligence,

    /// Hyperbolic functions (sinh, cosh, tanh, etc.)
    hyperbolic: hyperbolic::HyperbolicIntelligence,
}

impl ElementaryIntelligence {
    /// Create new elementary function intelligence system
    pub fn new() -> Self {
        Self {
            trigonometric: trigonometric::TrigonometricIntelligence::new(),
            exponential: exponential::ExponentialIntelligence::new(),
            logarithmic: logarithmic::LogarithmicIntelligence::new(),
            hyperbolic: hyperbolic::HyperbolicIntelligence::new(),
        }
    }

    /// Get all elementary function properties
    ///
    /// Returns a HashMap of all elementary functions and their properties
    /// for integration with the universal registry.
    pub fn get_all_properties(&self) -> HashMap<String, FunctionProperties> {
        let mut properties = HashMap::with_capacity(32);

        // Add trigonometric function properties
        properties.extend(self.trigonometric.get_properties());

        // Add exponential function properties
        properties.extend(self.exponential.get_properties());

        // Add logarithmic function properties
        properties.extend(self.logarithmic.get_properties());

        // Add hyperbolic function properties
        properties.extend(self.hyperbolic.get_properties());

        properties
    }

    /// Check if function is elementary
    pub fn is_elementary_function(&self, name: &str) -> bool {
        self.trigonometric.has_function(name)
            || self.exponential.has_function(name)
            || self.logarithmic.has_function(name)
            || self.hyperbolic.has_function(name)
    }
}
