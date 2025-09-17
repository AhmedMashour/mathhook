//! Exponential function intelligence
//!
//! Complete mathematical intelligence for exponential functions
//! with derivatives, special values, and mathematical properties.

use crate::core::{Expression, Symbol};
use crate::functions::properties::*;
use std::collections::HashMap;
use std::sync::Arc;
/// Exponential Function Intelligence
///
/// Dedicated intelligence system for exponential functions
/// with complete mathematical properties.
pub struct ExponentialIntelligence {
    /// Function properties for exponential functions
    properties: HashMap<String, FunctionProperties>,
}

impl Default for ExponentialIntelligence {
    fn default() -> Self {
        Self::new()
    }
}

impl ExponentialIntelligence {
    /// Create new exponential intelligence system
    pub fn new() -> Self {
        let mut intelligence = Self {
            properties: HashMap::with_capacity(4),
        };

        intelligence.initialize_exp();
        intelligence.initialize_sqrt();

        intelligence
    }

    /// Get all exponential function properties
    pub fn get_properties(&self) -> HashMap<String, FunctionProperties> {
        self.properties.clone()
    }

    /// Check if function is exponential
    pub fn has_function(&self, name: &str) -> bool {
        self.properties.contains_key(name)
    }

    /// Initialize exponential function
    fn initialize_exp(&mut self) {
        self.properties.insert(
            "exp".to_owned(),
            FunctionProperties::Elementary(Box::new(ElementaryProperties {
                derivative_rule: Some(DerivativeRule {
                    rule_type: DerivativeRuleType::SimpleFunctionSubstitution("exp".to_owned()),
                    result_template: "exp(x)".to_owned(),
                }),
                antiderivative_rule: Some(AntiderivativeRule {
                    rule_type: AntiderivativeRuleType::Simple {
                        antiderivative_fn: "exp".to_owned(),
                        coefficient: Expression::integer(1),
                    },
                    result_template: "∫exp(x)dx = exp(x) + C".to_owned(),
                    constant_handling: ConstantOfIntegration::AddConstant,
                }),
                special_values: vec![
                    SpecialValue {
                        input: "0".to_owned(),
                        output: Expression::integer(1),
                        latex_explanation: "e^0 = 1".to_owned(),
                    },
                    SpecialValue {
                        input: "1".to_owned(),
                        output: Expression::e(),
                        latex_explanation: "e^1 = e".to_owned(),
                    },
                ],
                identities: Box::new(vec![MathIdentity {
                    name: "Exponential Law".to_owned(),
                    lhs: Expression::function(
                        "exp",
                        vec![Expression::add(vec![
                            Expression::symbol("x"),
                            Expression::symbol("y"),
                        ])],
                    ),
                    rhs: Expression::mul(vec![
                        Expression::function("exp", vec![Expression::symbol("x")]),
                        Expression::function("exp", vec![Expression::symbol("y")]),
                    ]),
                    conditions: vec!["x, y ∈ ℝ".to_owned()],
                }]),
                domain_range: Box::new(DomainRangeData {
                    domain: Domain::Real,
                    range: Range::Unbounded,
                    singularities: vec![],
                }),
                periodicity: None,
                wolfram_name: Some("Exp"),
            })),
        );
    }

    /// Initialize sqrt function
    fn initialize_sqrt(&mut self) {
        self.properties.insert(
            "sqrt".to_owned(),
            FunctionProperties::Elementary(Box::new(ElementaryProperties {
                derivative_rule: Some(DerivativeRule {
                    rule_type: DerivativeRuleType::Custom {
                        builder: Arc::new(|arg: &Expression| {
                            Expression::mul(vec![
                                Expression::rational(1, 2),
                                Expression::pow(arg.clone(), Expression::rational(-1, 2)),
                            ])
                        }),
                    },
                    result_template: "1/(2√x)".to_owned(),
                }),
                antiderivative_rule: Some(AntiderivativeRule {
                    rule_type: AntiderivativeRuleType::Custom {
                        builder: Arc::new(|var: Symbol| {
                            Expression::mul(vec![
                                Expression::rational(2, 3),
                                Expression::pow(
                                    Expression::symbol(var),
                                    Expression::rational(3, 2),
                                ),
                            ])
                        }),
                    },
                    result_template: "∫√x dx = (2/3)x^(3/2) + C".to_owned(),
                    constant_handling: ConstantOfIntegration::AddConstant,
                }),
                special_values: vec![
                    SpecialValue {
                        input: "0".to_owned(),
                        output: Expression::integer(0),
                        latex_explanation: "\\sqrt{0} = 0".to_owned(),
                    },
                    SpecialValue {
                        input: "1".to_owned(),
                        output: Expression::integer(1),
                        latex_explanation: "\\sqrt{1} = 1".to_owned(),
                    },
                ],
                identities: Box::new(vec![MathIdentity {
                    name: "Power Rule".to_owned(),
                    lhs: Expression::function("sqrt", vec![Expression::symbol("x")]),
                    rhs: Expression::pow(
                        Expression::symbol("x"),
                        Expression::mul(vec![
                            Expression::integer(1),
                            Expression::pow(Expression::integer(2), Expression::integer(-1)),
                        ]),
                    ),
                    conditions: vec!["x ≥ 0".to_owned()],
                }]),
                domain_range: Box::new(DomainRangeData {
                    domain: Domain::Real,
                    range: Range::Unbounded,
                    singularities: vec![],
                }),
                periodicity: None,
                wolfram_name: Some("Sqrt"),
            })),
        );
    }
}
