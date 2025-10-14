//! Hyperbolic Function Intelligence
//!
//! Mathematically accurate implementation of hyperbolic functions
//! with verified derivatives, special values, and hyperbolic identities.

use crate::core::{Expression, Symbol};
use crate::functions::properties::*;
use std::collections::HashMap;
use std::sync::Arc;

/// Hyperbolic Function Intelligence
///
/// Complete mathematical intelligence for hyperbolic functions
/// with ABSOLUTE MATHEMATICAL ACCURACY.
pub struct HyperbolicIntelligence {
    /// Function properties for hyperbolic functions
    properties: HashMap<String, FunctionProperties>,
}

impl HyperbolicIntelligence {
    /// Create new hyperbolic intelligence system
    pub fn new() -> Self {
        let mut intelligence = Self {
            properties: HashMap::with_capacity(8),
        };

        intelligence.initialize_sinh_cosh();
        intelligence.initialize_tanh();

        intelligence
    }

    /// Get all hyperbolic function properties
    pub fn get_properties(&self) -> HashMap<String, FunctionProperties> {
        self.properties.clone()
    }

    /// Check if function is hyperbolic
    pub fn has_function(&self, name: &str) -> bool {
        self.properties.contains_key(name)
    }

    /// Initialize sinh and cosh with MATHEMATICAL ACCURACY
    fn initialize_sinh_cosh(&mut self) {
        // Hyperbolic Sine sinh(x) - MATHEMATICALLY VERIFIED
        self.properties.insert(
            "sinh".to_string(),
            FunctionProperties::Elementary(Box::new(ElementaryProperties {
                // DERIVATIVE: d/dx sinh(x) = cosh(x)
                derivative_rule: Some(DerivativeRule {
                    rule_type: DerivativeRuleType::Simple("cosh".to_string()),
                    result_template: "cosh(x)".to_string(),
                }),
                antiderivative_rule: Some(AntiderivativeRule {
                    rule_type: AntiderivativeRuleType::Simple {
                        antiderivative_fn: "cosh".to_string(),
                        coefficient: Expression::integer(1),
                    },
                    result_template: "∫sinh(x)dx = cosh(x) + C".to_string(),
                    constant_handling: ConstantOfIntegration::AddConstant,
                }),

                // Special values (mathematically verified)
                special_values: vec![
                    // sinh(0) = 0
                    SpecialValue {
                        input: "0".to_string(),
                        output: Expression::integer(0),
                        latex_explanation: "\\sinh(0) = 0".to_string(),
                    },
                ],

                // Hyperbolic identities (mathematically verified)
                identities: Box::new(vec![
                    // cosh²(x) - sinh²(x) = 1
                    MathIdentity {
                        name: "Hyperbolic Identity".to_string(),
                        lhs: Expression::function(
                            "cosh_squared_minus_sinh_squared",
                            vec![Expression::symbol("x")],
                        ),
                        rhs: Expression::integer(1),
                        conditions: vec!["x ∈ ℝ".to_string()],
                    },
                ]),

                domain_range: Box::new(DomainRangeData {
                    domain: Domain::Real, // (-∞, ∞)
                    range: Range::Real,   // (-∞, ∞)
                    singularities: vec![],
                }),

                // No periodicity (unlike trigonometric functions)
                periodicity: None,
                numerical_evaluator: Some(
                    crate::functions::properties::NumericalEvaluator::StandardLib(f64::sinh),
                ),
            })),
        );

        // Hyperbolic Cosine cosh(x) - MATHEMATICALLY VERIFIED
        self.properties.insert(
            "cosh".to_string(),
            FunctionProperties::Elementary(Box::new(ElementaryProperties {
                // DERIVATIVE: d/dx cosh(x) = sinh(x)
                derivative_rule: Some(DerivativeRule {
                    rule_type: DerivativeRuleType::Simple("sinh".to_string()),
                    result_template: "sinh(x)".to_string(),
                }),
                antiderivative_rule: Some(AntiderivativeRule {
                    rule_type: AntiderivativeRuleType::Simple {
                        antiderivative_fn: "sinh".to_string(),
                        coefficient: Expression::integer(1),
                    },
                    result_template: "∫cosh(x)dx = sinh(x) + C".to_string(),
                    constant_handling: ConstantOfIntegration::AddConstant,
                }),

                // Special values (mathematically verified)
                special_values: vec![
                    // cosh(0) = 1
                    SpecialValue {
                        input: "0".to_string(),
                        output: Expression::integer(1),
                        latex_explanation: "\\cosh(0) = 1".to_string(),
                    },
                ],

                // Hyperbolic identities (mathematically verified)
                identities: Box::new(vec![
                    // cosh²(x) - sinh²(x) = 1
                    MathIdentity {
                        name: "Hyperbolic Identity".to_string(),
                        lhs: Expression::function(
                            "cosh_squared_minus_sinh_squared",
                            vec![Expression::symbol("x")],
                        ),
                        rhs: Expression::integer(1),
                        conditions: vec!["x ∈ ℝ".to_string()],
                    },
                ]),

                domain_range: Box::new(DomainRangeData {
                    domain: Domain::Real,    // (-∞, ∞)
                    range: Range::Unbounded, // [1, ∞)
                    singularities: vec![],
                }),

                // No periodicity (unlike trigonometric functions)
                periodicity: None,
                numerical_evaluator: Some(
                    crate::functions::properties::NumericalEvaluator::StandardLib(f64::cosh),
                ),
            })),
        );
    }

    /// Initialize tanh with MATHEMATICAL ACCURACY
    fn initialize_tanh(&mut self) {
        // Hyperbolic Tangent tanh(x) - MATHEMATICALLY VERIFIED
        self.properties.insert(
            "tanh".to_string(),
            FunctionProperties::Elementary(Box::new(ElementaryProperties {
                // DERIVATIVE: d/dx tanh(x) = sech²(x) = 1 - tanh²(x)
                derivative_rule: Some(DerivativeRule {
                    rule_type: DerivativeRuleType::Simple("sech²".to_string()),
                    result_template: "sech²(x)".to_string(),
                }),
                antiderivative_rule: Some(AntiderivativeRule {
                    rule_type: AntiderivativeRuleType::Custom {
                        builder: Arc::new(|var: Symbol| {
                            Expression::function(
                                "ln",
                                vec![Expression::function("cosh", vec![Expression::symbol(var)])],
                            )
                        }),
                    },
                    result_template: "∫tanh(x)dx = ln(cosh(x)) + C".to_string(),
                    constant_handling: ConstantOfIntegration::AddConstant,
                }),

                // Special values (mathematically verified)
                special_values: vec![
                    // tanh(0) = 0
                    SpecialValue {
                        input: "0".to_string(),
                        output: Expression::integer(0),
                        latex_explanation: "\\tanh(0) = 0".to_string(),
                    },
                ],

                // Hyperbolic identities
                identities: Box::new(vec![MathIdentity {
                    name: "Hyperbolic Tangent Identity".to_string(),
                    lhs: Expression::function("tanh", vec![Expression::symbol("x")]),
                    rhs: Expression::function("sinh_over_cosh", vec![Expression::symbol("x")]),
                    conditions: vec!["x ∈ ℝ".to_string()],
                }]),

                domain_range: Box::new(DomainRangeData {
                    domain: Domain::Real, // (-∞, ∞)
                    range: Range::Bounded(Expression::integer(-1), Expression::integer(1)), // (-1, 1)
                    singularities: vec![],
                }),

                // No periodicity (unlike trigonometric functions)
                periodicity: None,
                numerical_evaluator: Some(
                    crate::functions::properties::NumericalEvaluator::StandardLib(f64::tanh),
                ),
            })),
        );
    }
}
