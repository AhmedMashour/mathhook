//! Trigonometric Function Intelligence
//!
//! Complete mathematical intelligence for all trigonometric functions.
//! This module combines circular and inverse trigonometric functions.

mod trig_circular;
mod trig_evaluation;
mod trig_inverse;
mod trig_inverse_eval;

pub use trig_circular::CircularTrigIntelligence;
pub use trig_evaluation::{cos, cos_evaluator, sin, sin_evaluator, tan, tan_evaluator};
pub use trig_inverse::InverseTrigIntelligence;
pub use trig_inverse_eval::{arccos, arcsin, arctan};

use crate::functions::properties::FunctionProperties;
use std::collections::HashMap;

/// Trigonometric Function Intelligence (Combined)
///
/// Provides unified access to all trigonometric functions:
/// circular (sin, cos, tan, cot, sec, csc) and inverse (arcsin, arccos, arctan)
pub struct TrigonometricIntelligence {
    circular: CircularTrigIntelligence,
    inverse: InverseTrigIntelligence,
}

impl TrigonometricIntelligence {
    /// Create new trigonometric intelligence system
    pub fn new() -> Self {
        Self {
            circular: CircularTrigIntelligence::new(),
            inverse: InverseTrigIntelligence::new(),
        }
    }

    /// Get all trigonometric function properties
    pub fn get_properties(&self) -> HashMap<String, FunctionProperties> {
        let mut props = HashMap::with_capacity(9);
        props.extend(self.circular.get_properties());
        props.extend(self.inverse.get_properties());
        props
    }

    /// Check if function is trigonometric
    pub fn has_function(&self, name: &str) -> bool {
        self.circular.has_function(name) || self.inverse.has_function(name)
    }
}

impl Default for TrigonometricIntelligence {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trigonometric_intelligence() {
        let trig = TrigonometricIntelligence::new();

        assert!(trig.has_function("sin"));
        assert!(trig.has_function("cos"));
        assert!(trig.has_function("tan"));
        assert!(trig.has_function("arcsin"));
        assert!(trig.has_function("arccos"));
        assert!(trig.has_function("arctan"));
        assert!(!trig.has_function("exp"));

        let properties = trig.get_properties();
        assert!(properties.contains_key("sin"));
        assert!(properties.contains_key("cos"));
        assert!(properties.contains_key("arcsin"));
        assert!(properties.len() >= 9);

        if let Some(FunctionProperties::Elementary(sin_props)) = properties.get("sin") {
            assert!(sin_props.derivative_rule.is_some());
            assert!(!sin_props.special_values.is_empty());
            assert!(sin_props.periodicity.is_some());
        }
    }
}
