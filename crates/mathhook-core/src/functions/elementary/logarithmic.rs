//! Logarithmic Function Intelligence
//!
//! with verified derivatives, special values, and logarithm laws.

use crate::core::{Expression, Symbol};
use crate::functions::properties::*;
use std::collections::HashMap;
use std::sync::Arc;

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
        intelligence.initialize_log();

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
                evaluator: |args| {
                    if args.len() == 1 {
                        args[0].clone()
                    } else {
                        Expression::function("ln", args.to_vec())
                    }
                },

                // DERIVATIVE: d/dx ln(x) = 1/x (x > 0)
                derivative_rule: Some(DerivativeRule {
                    rule_type: DerivativeRuleType::Simple("1/x".to_string()),
                    result_template: "1/x".to_string(),
                }),
                antiderivative_rule: Some(AntiderivativeRule {
                    rule_type: AntiderivativeRuleType::Custom {
                        builder: Arc::new(|var: Symbol| {
                            Expression::add(vec![
                                Expression::mul(vec![
                                    Expression::symbol(var.clone()),
                                    Expression::function(
                                        "ln",
                                        vec![Expression::symbol(var.clone())],
                                    ),
                                ]),
                                Expression::mul(vec![
                                    Expression::integer(-1),
                                    Expression::symbol(var),
                                ]),
                            ])
                        }),
                    },
                    result_template: "∫ln(x)dx = x·ln(x) - x + C".to_string(),
                    constant_handling: ConstantOfIntegration::AddConstant,
                }),

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

    /// Initialize base-10 logarithm
    fn initialize_log(&mut self) {
        self.properties.insert(
            "log".to_string(),
            FunctionProperties::Elementary(Box::new(ElementaryProperties {
                evaluator: |args| {
                    if args.len() == 2 {
                        args[0].clone()
                    } else if args.len() == 1 {
                        super::super::logarithmic::log(
                            &args[0],
                            &Expression::constant(crate::core::MathConstant::E),
                        )
                    } else {
                        Expression::function("log", args.to_vec())
                    }
                },

                derivative_rule: Some(DerivativeRule {
                    rule_type: DerivativeRuleType::Simple("1/(x·ln(10))".to_string()),
                    result_template: "1/(x·ln(10))".to_string(),
                }),
                antiderivative_rule: Some(AntiderivativeRule {
                    rule_type: AntiderivativeRuleType::Custom {
                        builder: Arc::new(|var: Symbol| {
                            Expression::mul(vec![
                                Expression::pow(
                                    Expression::function("ln", vec![Expression::integer(10)]),
                                    Expression::integer(-1),
                                ),
                                Expression::add(vec![
                                    Expression::mul(vec![
                                        Expression::symbol(var.clone()),
                                        Expression::function(
                                            "ln",
                                            vec![Expression::symbol(var.clone())],
                                        ),
                                    ]),
                                    Expression::mul(vec![
                                        Expression::integer(-1),
                                        Expression::symbol(var),
                                    ]),
                                ]),
                            ])
                        }),
                    },
                    result_template: "∫log(x)dx = (1/ln(10))·(x·ln(x) - x) + C".to_string(),
                    constant_handling: ConstantOfIntegration::AddConstant,
                }),
                special_values: vec![
                    SpecialValue {
                        input: "1".to_string(),
                        output: Expression::integer(0),
                        latex_explanation: "\\log(1) = 0".to_string(),
                    },
                    SpecialValue {
                        input: "10".to_string(),
                        output: Expression::integer(1),
                        latex_explanation: "\\log(10) = 1".to_string(),
                    },
                ],
                identities: Box::new(vec![]),
                domain_range: Box::new(DomainRangeData {
                    domain: Domain::Interval(Expression::integer(0), Expression::symbol("∞")),
                    range: Range::Real,
                    singularities: vec![Expression::integer(0)],
                }),
                periodicity: None,
                numerical_evaluator: Some(
                    crate::functions::properties::NumericalEvaluator::StandardLib(f64::log10),
                ),
            })),
        );
    }
}
