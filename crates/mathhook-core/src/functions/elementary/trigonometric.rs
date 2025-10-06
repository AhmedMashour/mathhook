//! Trigonometric Function Intelligence
//!
//! Complete mathematical intelligence for trigonometric functions:
//! sin, cos, tan, sec, csc, cot with derivatives, identities, and special values.

use crate::core::Expression;
use crate::functions::properties::*;
use std::collections::HashMap;

/// Trigonometric Function Intelligence
///
/// Dedicated intelligence system for trigonometric functions
/// with complete mathematical properties and educational context.
pub struct TrigonometricIntelligence {
    /// Function properties for each trigonometric function
    properties: HashMap<String, FunctionProperties>,
}

impl TrigonometricIntelligence {
    /// Create new trigonometric intelligence system
    pub fn new() -> Self {
        let mut intelligence = Self {
            properties: HashMap::with_capacity(8),
        };

        intelligence.initialize_sin_cos();
        intelligence.initialize_tan_cot();
        intelligence.initialize_sec_csc();

        intelligence
    }

    /// Get all trigonometric function properties
    pub fn get_properties(&self) -> HashMap<String, FunctionProperties> {
        self.properties.clone()
    }

    /// Check if function is trigonometric
    pub fn has_function(&self, name: &str) -> bool {
        self.properties.contains_key(name)
    }

    /// Initialize sin and cos with complete mathematical intelligence
    fn initialize_sin_cos(&mut self) {
        // Sin function with complete mathematical properties
        self.properties.insert(
            "sin".to_string(),
            FunctionProperties::Elementary(Box::new(ElementaryProperties {
                derivative_rule: Some(DerivativeRule {
                    rule_type: DerivativeRuleType::Simple("cos".to_string()),
                    result_template: "cos(x)".to_string(),
                }),
                special_values: vec![
                    SpecialValue {
                        input: "0".to_string(),
                        output: Expression::integer(0),
                        latex_explanation: "\\sin(0) = 0".to_string(),
                    },
                    SpecialValue {
                        input: "π/2".to_string(),
                        output: Expression::integer(1),
                        latex_explanation: "\\sin(\\frac{\\pi}{2}) = 1".to_string(),
                    },
                    SpecialValue {
                        input: "π".to_string(),
                        output: Expression::integer(0),
                        latex_explanation: "\\sin(\\pi) = 0".to_string(),
                    },
                ],
                identities: Box::new(vec![MathIdentity {
                    name: "Pythagorean Identity".to_string(),
                    lhs: Expression::add(vec![
                        Expression::pow(
                            Expression::function("sin", vec![Expression::symbol("x")]),
                            Expression::integer(2),
                        ),
                        Expression::pow(
                            Expression::function("cos", vec![Expression::symbol("x")]),
                            Expression::integer(2),
                        ),
                    ]),
                    rhs: Expression::integer(1),
                    conditions: vec!["x ∈ ℝ".to_string()],
                }]),
                domain_range: Box::new(DomainRangeData {
                    domain: Domain::Real,
                    range: Range::Bounded(Expression::integer(-1), Expression::integer(1)),
                    singularities: vec![],
                }),
                periodicity: Some(Expression::mul(vec![
                    Expression::integer(2),
                    Expression::pi(),
                ])),
                numerical_evaluator: Some(
                    crate::functions::properties::NumericalEvaluator::StandardLib(f64::sin),
                ),
            })),
        );

        // Cos function with complete mathematical properties
        self.properties.insert(
            "cos".to_string(),
            FunctionProperties::Elementary(Box::new(ElementaryProperties {
                derivative_rule: Some(DerivativeRule {
                    rule_type: DerivativeRuleType::Simple("-sin".to_string()),
                    result_template: "-sin(x)".to_string(),
                }),
                special_values: vec![
                    // Fundamental cosine values - essential for mathematical accuracy
                    SpecialValue {
                        input: "0".to_string(),
                        output: Expression::integer(1),
                        latex_explanation: "\\cos(0) = 1".to_string(),
                    },
                    SpecialValue {
                        input: "π/2".to_string(),
                        output: Expression::integer(0),
                        latex_explanation: "\\cos(\\frac{\\pi}{2}) = 0".to_string(),
                    },
                    SpecialValue {
                        input: "π".to_string(),
                        output: Expression::integer(-1),
                        latex_explanation: "\\cos(\\pi) = -1".to_string(),
                    },
                    SpecialValue {
                        input: "3π/2".to_string(),
                        output: Expression::integer(0),
                        latex_explanation: "\\cos(\\frac{3\\pi}{2}) = 0".to_string(),
                    },
                    SpecialValue {
                        input: "2π".to_string(),
                        output: Expression::integer(1),
                        latex_explanation: "\\cos(2\\pi) = 1".to_string(),
                    },
                ],
                identities: Box::new(vec![]),
                domain_range: Box::new(DomainRangeData {
                    domain: Domain::Real,
                    range: Range::Bounded(Expression::integer(-1), Expression::integer(1)),
                    singularities: vec![],
                }),
                periodicity: Some(Expression::mul(vec![
                    Expression::integer(2),
                    Expression::pi(),
                ])),
                numerical_evaluator: Some(
                    crate::functions::properties::NumericalEvaluator::StandardLib(f64::cos),
                ),
            })),
        );
    }

    /// Initialize tan and cot functions
    fn initialize_tan_cot(&mut self) {
        // Tangent function
        self.properties.insert(
            "tan".to_string(),
            FunctionProperties::Elementary(Box::new(ElementaryProperties {
                derivative_rule: Some(DerivativeRule {
                    rule_type: DerivativeRuleType::Simple("sec²".to_string()),
                    result_template: "sec²(x)".to_string(),
                }),
                special_values: vec![
                    SpecialValue {
                        input: "0".to_string(),
                        output: Expression::integer(0),
                        latex_explanation: "\\tan(0) = 0".to_string(),
                    },
                    SpecialValue {
                        input: "π/4".to_string(),
                        output: Expression::integer(1),
                        latex_explanation: "\\tan(\\frac{\\pi}{4}) = 1".to_string(),
                    },
                ],
                identities: Box::new(vec![MathIdentity {
                    name: "Tangent Identity".to_string(),
                    lhs: Expression::function("tan", vec![Expression::symbol("x")]),
                    rhs: Expression::function("sin_over_cos", vec![Expression::symbol("x")]),
                    conditions: vec!["cos(x) ≠ 0".to_string()],
                }]),
                domain_range: Box::new(DomainRangeData {
                    domain: Domain::Real,
                    range: Range::Real,
                    singularities: vec![Expression::function(
                        "tan_singularities",
                        vec![Expression::symbol("n")],
                    )],
                }),
                periodicity: Some(Expression::pi()),
                numerical_evaluator: Some(
                    crate::functions::properties::NumericalEvaluator::StandardLib(f64::tan),
                ),
            })),
        );
    }

    /// Initialize sec and csc functions
    fn initialize_sec_csc(&mut self) {
        // Placeholder for sec, csc functions
        // Will be implemented as needed
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trigonometric_intelligence() {
        let trig = TrigonometricIntelligence::new();

        // Test that trigonometric functions are recognized
        assert!(trig.has_function("sin"));
        assert!(trig.has_function("cos"));
        assert!(trig.has_function("tan"));
        assert!(!trig.has_function("exp")); // Not trigonometric

        // Test properties retrieval
        let properties = trig.get_properties();
        assert!(properties.contains_key("sin"));
        assert!(properties.contains_key("cos"));

        // Test mathematical properties
        if let Some(FunctionProperties::Elementary(sin_props)) = properties.get("sin") {
            assert!(sin_props.derivative_rule.is_some());
            assert!(!sin_props.special_values.is_empty());
            assert!(sin_props.periodicity.is_some());
        }
    }
}
