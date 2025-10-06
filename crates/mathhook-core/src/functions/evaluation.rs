//! 🧮 Function Evaluation
//!
//! Simple, clean evaluation that uses existing implementations
//! and Function Intelligence properties.

use crate::core::{Expression, Number};
use crate::functions::intelligence::UNIVERSAL_REGISTRY;

/// Result of mathematical evaluation
#[derive(Debug, Clone)]
pub enum EvaluationResult {
    /// Exact symbolic result
    Exact(Expression),
    /// Numerical approximation
    Numerical(f64),
    /// Unevaluated (remains symbolic)
    Unevaluated,
    /// Error in evaluation
    Error(String),
}

/// Function evaluator
pub struct FunctionEvaluator;

impl FunctionEvaluator {
    /// Create new evaluator
    pub fn new() -> Self {
        Self
    }

    /// Evaluate function using pure Function Intelligence
    pub fn evaluate(&self, name: &str, args: &[Expression]) -> EvaluationResult {
        // Get function properties and let them drive evaluation
        if let Some(properties) = UNIVERSAL_REGISTRY.get_properties(name) {
            properties.evaluate(name, args)
        } else {
            EvaluationResult::Error(format!(
                "Function '{}' not found in intelligence system",
                name
            ))
        }
    }

    // All evaluation logic is now in Function Intelligence properties!

    /// Bulk evaluation using Function Intelligence
    pub fn evaluate_bulk_f64(&self, name: &str, points: &[f64]) -> Option<Vec<f64>> {
        // Only evaluate if function has intelligence and sufficient points for bulk processing
        if points.len() < 4 || !UNIVERSAL_REGISTRY.has_intelligence(name) {
            return None;
        }

        // Get function properties and let them drive bulk evaluation
        if let Some(properties) = UNIVERSAL_REGISTRY.get_properties(name) {
            // Use Function Intelligence to determine if bulk numerical evaluation is possible
            match properties {
                crate::functions::properties::FunctionProperties::Elementary(_) => {
                    // Elementary functions can potentially support bulk evaluation
                    // This would be extended to use properties to determine evaluation method
                    self.try_bulk_evaluation_from_intelligence(name, points, properties)
                }
                _ => None, // Other function types don't support bulk evaluation yet
            }
        } else {
            None
        }
    }

    /// Try bulk evaluation using Function Intelligence properties
    fn try_bulk_evaluation_from_intelligence(
        &self,
        name: &str,
        points: &[f64],
        properties: &crate::functions::properties::FunctionProperties,
    ) -> Option<Vec<f64>> {
        // Use Function Intelligence to determine if numerical bulk evaluation is supported
        match properties {
            crate::functions::properties::FunctionProperties::Elementary(props) => {
                // Check if this elementary function supports numerical evaluation
                self.evaluate_elementary_bulk(name, points, props)
            }
            _ => None,
        }
    }

    /// Evaluate elementary functions in bulk using their mathematical properties
    ///
    /// Modular bulk evaluation driven by function intelligence properties
    fn evaluate_elementary_bulk(
        &self,
        name: &str,
        points: &[f64],
        props: &crate::functions::properties::ElementaryProperties,
    ) -> Option<Vec<f64>> {
        // Use function properties to determine if bulk numerical evaluation is supported
        if self.supports_bulk_numerical_evaluation(name, props) {
            self.perform_bulk_numerical_evaluation(name, points)
        } else {
            None
        }
    }

    /// Check if function supports bulk numerical evaluation using intelligence
    ///
    /// NO HARDCODED MATCHES - uses function properties to determine capability
    fn supports_bulk_numerical_evaluation(
        &self,
        _name: &str,
        props: &crate::functions::properties::ElementaryProperties,
    ) -> bool {
        // Use function properties to determine bulk evaluation capability
        // Elementary functions with defined domain/range can support bulk evaluation
        props.domain_range.domain != crate::functions::properties::Domain::Complex
    }

    /// Perform bulk numerical evaluation using function intelligence
    ///
    ///Bulk computation using function properties
    fn perform_bulk_numerical_evaluation(&self, name: &str, points: &[f64]) -> Option<Vec<f64>> {
        // Get function properties to determine numerical evaluation method
        if let Some(properties) = super::intelligence::UNIVERSAL_REGISTRY.get_properties(name) {
            // Use function intelligence to perform bulk evaluation
            self.bulk_evaluate_using_properties(properties, points)
        } else {
            None
        }
    }

    /// Bulk evaluate using function properties - pure intelligence approach
    ///
    /// Uses mathematical properties to determine evaluation strategy
    fn bulk_evaluate_using_properties(
        &self,
        properties: &crate::functions::properties::FunctionProperties,
        points: &[f64],
    ) -> Option<Vec<f64>> {
        match properties {
            crate::functions::properties::FunctionProperties::Elementary(props) => {
                // Use elementary function properties for bulk evaluation
                self.bulk_evaluate_elementary_from_properties(props, points)
            }
            _ => None, // Other function types don't support bulk evaluation yet
        }
    }

    /// Bulk evaluate elementary functions using their mathematical properties
    ///
    /// Pure intelligence-driven numerical computation using function properties
    fn bulk_evaluate_elementary_from_properties(
        &self,
        props: &crate::functions::properties::ElementaryProperties,
        points: &[f64],
    ) -> Option<Vec<f64>> {
        // Use the numerical evaluator from function properties
        if let Some(evaluator) = &props.numerical_evaluator {
            match evaluator {
                crate::functions::properties::NumericalEvaluator::StandardLib(func) => {
                    // Use the function pointer stored in properties
                    Some(points.iter().map(|&x| func(x)).collect())
                }
                crate::functions::properties::NumericalEvaluator::Custom(bulk_func) => {
                    // Use custom bulk evaluation function
                    Some(bulk_func(points))
                }
                _ => None,
            }
        } else {
            None
        }
    }
}

impl Default for FunctionEvaluator {
    fn default() -> Self {
        Self::new()
    }
}
