//! Logarithmic Function Intelligence
//!
//! with verified derivatives, special values, and logarithm laws.

use crate::core::Expression;
use crate::functions::properties::*;
use std::collections::HashMap;

/// Logarithmic Function Intelligence
///
/// Complete mathematical intelligence for logarithmic functions
pub struct LogarithmicIntelligence {
    /// Function properties for logarithmic functions
    properties: HashMap<String, FunctionProperties>,
}

impl LogarithmicIntelligence {
    /// Create new logarithmic intelligence system
    pub fn new() -> Self {
        let mut intelligence = Self {
            properties: HashMap::with_capacity(4),
        };

        intelligence.initialize_ln();

        intelligence
    }

    /// Get all logarithmic function properties
    pub fn get_properties(&self) -> HashMap<String, FunctionProperties> {
        self.properties.clone()
    }

    /// Check if function is logarithmic
    pub fn has_function(&self, name: &str) -> bool {
        self.properties.contains_key(name)
    }

    /// Initialize natural logarithm
    fn initialize_ln(&mut self) {
        // Natural Logarithm ln(x)
        self.properties.insert(
            "ln".to_string(),
            FunctionProperties::Elementary(Box::new(ElementaryProperties {
                // DERIVATIVE: d/dx ln(x) = 1/x (x > 0)
                derivative_rule: Some(DerivativeRule {
                    rule_type: DerivativeRuleType::Simple("1/x".to_string()),
                    result_template: "1/x".to_string(),
                }),
                antiderivative_rule: None,

                // SPECIAL VALUES
                special_values: vec![
                    // ln(1) = 0
                    SpecialValue {
                        input: "1".to_string(),
                        output: Expression::integer(0),
                        latex_explanation: "\\ln(1) = 0".to_string(),
                    },
                    // ln(e) = 1
                    SpecialValue {
                        input: "e".to_string(),
                        output: Expression::integer(1),
                        latex_explanation: "\\ln(e) = 1".to_string(),
                    },
                ],

                // Logarithm Laws
                identities: Box::new(vec![
                    // ln(xy) = ln(x) + ln(y)
                    MathIdentity {
                        name: "Logarithm Product Law".to_string(),
                        lhs: Expression::function(
                            "ln",
                            vec![Expression::mul(vec![
                                Expression::symbol("x"),
                                Expression::symbol("y"),
                            ])],
                        ),
                        rhs: Expression::add(vec![
                            Expression::function("ln", vec![Expression::symbol("x")]),
                            Expression::function("ln", vec![Expression::symbol("y")]),
                        ]),
                        conditions: vec!["x, y > 0".to_string()],
                    },
                ]),

                domain_range: Box::new(DomainRangeData {
                    domain: Domain::Interval(Expression::integer(0), Expression::symbol("∞")), // (0, ∞)
                    range: Range::Real,                          // (-∞, ∞)
                    singularities: vec![Expression::integer(0)], // Singularity at x = 0
                }),

                // No periodicity
                periodicity: None,
                numerical_evaluator: Some(
                    crate::functions::properties::NumericalEvaluator::StandardLib(f64::ln),
                ),
            })),
        );
    }
}
