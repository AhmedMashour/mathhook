//! Exponential function intelligence
//!
//! Complete mathematical intelligence for exponential functions
//! with derivatives, special values, and mathematical properties.

use crate::core::{Expression, Symbol};
use crate::functions::properties::NumericalEvaluator;
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
            "exp".to_string(),
            FunctionProperties::Elementary(Box::new(ElementaryProperties {
                evaluator: |args| {
                if args.len() == 1 {
                    args[0].clone()
                } else {
                    Expression::function("exp", args.to_vec())
                }
            },
            
                derivative_rule: Some(DerivativeRule {
                    rule_type: DerivativeRuleType::Simple("exp".to_string()),
                    result_template: "exp(x)".to_string(),
                }),
                antiderivative_rule: Some(AntiderivativeRule {
                    rule_type: AntiderivativeRuleType::Simple {
                        antiderivative_fn: "exp".to_string(),
                        coefficient: Expression::integer(1),
                    },
                    result_template: "∫exp(x)dx = exp(x) + C".to_string(),
                    constant_handling: ConstantOfIntegration::AddConstant,
                }),
                special_values: vec![
                    SpecialValue {
                        input: "0".to_string(),
                        output: Expression::integer(1),
                        latex_explanation: "e^0 = 1".to_string(),
                    },
                    SpecialValue {
                        input: "1".to_string(),
                        output: Expression::e(),
                        latex_explanation: "e^1 = e".to_string(),
                    },
                ],
                identities: Box::new(vec![MathIdentity {
                    name: "Exponential Law".to_string(),
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
                    conditions: vec!["x, y ∈ ℝ".to_string()],
                }]),
                domain_range: Box::new(DomainRangeData {
                    domain: Domain::Real,
                    range: Range::Unbounded,
                    singularities: vec![],
                }),
                periodicity: None,
                numerical_evaluator: Some(NumericalEvaluator::StandardLib(f64::exp)),
            })),
        );
    }

    /// Initialize sqrt function
    fn initialize_sqrt(&mut self) {
        self.properties.insert(
            "sqrt".to_string(),
            FunctionProperties::Elementary(Box::new(ElementaryProperties {
                derivative_rule: Some(DerivativeRule {
                    rule_type: DerivativeRuleType::Simple("1/(2√x)".to_string()),
                    result_template: "1/(2√x)".to_string(),
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
                    result_template: "∫√x dx = (2/3)x^(3/2) + C".to_string(),
                    constant_handling: ConstantOfIntegration::AddConstant,
                }),
                special_values: vec![
                    SpecialValue {
                        input: "0".to_string(),
                        output: Expression::integer(0),
                        latex_explanation: "\\sqrt{0} = 0".to_string(),
                    },
                    SpecialValue {
                        input: "1".to_string(),
                        output: Expression::integer(1),
                        latex_explanation: "\\sqrt{1} = 1".to_string(),
                    },
                ],
                identities: Box::new(vec![MathIdentity {
                    name: "Power Rule".to_string(),
                    lhs: Expression::function("sqrt", vec![Expression::symbol("x")]),
                    rhs: Expression::pow(
                        Expression::symbol("x"),
                        Expression::mul(vec![
                            Expression::integer(1),
                            Expression::pow(Expression::integer(2), Expression::integer(-1)),
                        ]),
                    ),
                    conditions: vec!["x ≥ 0".to_string()],
                }]),
                domain_range: Box::new(DomainRangeData {
                    domain: Domain::Real,
                    range: Range::Unbounded,
                    singularities: vec![],
                }),
                periodicity: None,
                numerical_evaluator: Some(NumericalEvaluator::StandardLib(f64::sqrt)),
            })),
        );
    }
}
