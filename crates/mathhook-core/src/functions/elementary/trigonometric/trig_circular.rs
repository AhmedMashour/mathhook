//! Circular Trigonometric Functions (sin, cos, tan, cot, sec, csc)
//!
//! Complete mathematical intelligence for circular trigonometric functions
//! with derivatives, identities, and special values.

use crate::core::{Expression, Symbol};
use crate::functions::properties::*;
use std::collections::HashMap;
use std::sync::Arc;

/// Circular Trigonometric Function Intelligence
///
/// Dedicated intelligence for sin, cos, tan, cot, sec, csc
pub struct CircularTrigIntelligence {
    properties: HashMap<String, FunctionProperties>,
}

impl CircularTrigIntelligence {
    /// Create new circular trigonometric intelligence system
    pub fn new() -> Self {
        let mut intelligence = Self {
            properties: HashMap::with_capacity(6),
        };

        intelligence.initialize_sin_cos();
        intelligence.initialize_tan_cot();
        intelligence.initialize_sec_csc();

        intelligence
    }

    /// Get all circular trigonometric function properties
    pub fn get_properties(&self) -> HashMap<String, FunctionProperties> {
        self.properties.clone()
    }

    /// Check if function is circular trigonometric
    pub fn has_function(&self, name: &str) -> bool {
        self.properties.contains_key(name)
    }

    /// Initialize sin and cos with complete mathematical intelligence
    fn initialize_sin_cos(&mut self) {
        self.properties.insert(
            "sin".to_string(),
            FunctionProperties::Elementary(Box::new(ElementaryProperties {
                evaluator: |args| {
                    if args.len() == 1 {
                        args[0].clone()
                    } else {
                        Expression::function("sin", args.to_vec())
                    }
                },

                derivative_rule: Some(DerivativeRule {
                    rule_type: DerivativeRuleType::Simple("cos".to_string()),
                    result_template: "cos(x)".to_string(),
                }),
                antiderivative_rule: Some(AntiderivativeRule {
                    rule_type: AntiderivativeRuleType::Simple {
                        antiderivative_fn: "cos".to_string(),
                        coefficient: Expression::integer(-1),
                    },
                    result_template: "∫sin(x)dx = -cos(x) + C".to_string(),
                    constant_handling: ConstantOfIntegration::AddConstant,
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

        self.properties.insert(
            "cos".to_string(),
            FunctionProperties::Elementary(Box::new(ElementaryProperties {
                evaluator: |args| {
                    if args.len() == 1 {
                        super::args[0].clone()
                    } else {
                        Expression::function("cos", args.to_vec())
                    }
                },

                derivative_rule: Some(DerivativeRule {
                    rule_type: DerivativeRuleType::Simple("-sin".to_string()),
                    result_template: "-sin(x)".to_string(),
                }),
                antiderivative_rule: Some(AntiderivativeRule {
                    rule_type: AntiderivativeRuleType::Simple {
                        antiderivative_fn: "sin".to_string(),
                        coefficient: Expression::integer(1),
                    },
                    result_template: "∫cos(x)dx = sin(x) + C".to_string(),
                    constant_handling: ConstantOfIntegration::AddConstant,
                }),
                special_values: vec![
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
        self.properties.insert(
            "tan".to_string(),
            FunctionProperties::Elementary(Box::new(ElementaryProperties {
                evaluator: |args| {
                    if args.len() == 1 {
                        super::args[0].clone()
                    } else {
                        Expression::function("tan", args.to_vec())
                    }
                },

                derivative_rule: Some(DerivativeRule {
                    rule_type: DerivativeRuleType::Simple("sec²".to_string()),
                    result_template: "sec²(x)".to_string(),
                }),
                antiderivative_rule: Some(AntiderivativeRule {
                    rule_type: AntiderivativeRuleType::Custom {
                        builder: Arc::new(|var: Symbol| {
                            Expression::mul(vec![
                                Expression::integer(-1),
                                Expression::function(
                                    "ln",
                                    vec![Expression::function(
                                        "abs",
                                        vec![Expression::function(
                                            "cos",
                                            vec![Expression::symbol(var)],
                                        )],
                                    )],
                                ),
                            ])
                        }),
                    },
                    result_template: "∫tan(x)dx = -ln|cos(x)| + C".to_string(),
                    constant_handling: ConstantOfIntegration::AddConstant,
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

        self.properties.insert(
            "cot".to_string(),
            FunctionProperties::Elementary(Box::new(ElementaryProperties {
                evaluator: |args| {
                    if args.len() == 1 {
                        super::args[0].clone()
                    } else {
                        Expression::function("cot", args.to_vec())
                    }
                },

                derivative_rule: Some(DerivativeRule {
                    rule_type: DerivativeRuleType::Simple("-csc²".to_string()),
                    result_template: "-csc²(x)".to_string(),
                }),
                antiderivative_rule: Some(AntiderivativeRule {
                    rule_type: AntiderivativeRuleType::Custom {
                        builder: Arc::new(|var: Symbol| {
                            Expression::function(
                                "ln",
                                vec![Expression::function(
                                    "abs",
                                    vec![Expression::function(
                                        "sin",
                                        vec![Expression::symbol(var)],
                                    )],
                                )],
                            )
                        }),
                    },
                    result_template: "∫cot(x)dx = ln|sin(x)| + C".to_string(),
                    constant_handling: ConstantOfIntegration::AddConstant,
                }),
                special_values: vec![SpecialValue {
                    input: "π/4".to_string(),
                    output: Expression::integer(1),
                    latex_explanation: "\\cot(\\frac{\\pi}{4}) = 1".to_string(),
                }],
                identities: Box::new(vec![MathIdentity {
                    name: "Cotangent Identity".to_string(),
                    lhs: Expression::function("cot", vec![Expression::symbol("x")]),
                    rhs: Expression::function("cos_over_sin", vec![Expression::symbol("x")]),
                    conditions: vec!["sin(x) ≠ 0".to_string()],
                }]),
                domain_range: Box::new(DomainRangeData {
                    domain: Domain::Real,
                    range: Range::Real,
                    singularities: vec![Expression::function(
                        "cot_singularities",
                        vec![Expression::symbol("n")],
                    )],
                }),
                periodicity: Some(Expression::pi()),
                numerical_evaluator: None,
            })),
        );
    }

    /// Initialize sec and csc functions
    fn initialize_sec_csc(&mut self) {
        self.properties.insert(
            "sec".to_string(),
            FunctionProperties::Elementary(Box::new(ElementaryProperties {
                evaluator: |args| {
                    if args.len() == 1 {
                        super::args[0].clone()
                    } else {
                        Expression::function("sec", args.to_vec())
                    }
                },

                derivative_rule: Some(DerivativeRule {
                    rule_type: DerivativeRuleType::Simple("sec·tan".to_string()),
                    result_template: "sec(x)·tan(x)".to_string(),
                }),
                antiderivative_rule: Some(AntiderivativeRule {
                    rule_type: AntiderivativeRuleType::Custom {
                        builder: Arc::new(|var: Symbol| {
                            Expression::function(
                                "ln",
                                vec![Expression::function(
                                    "abs",
                                    vec![Expression::add(vec![
                                        Expression::function(
                                            "sec",
                                            vec![Expression::symbol(var.clone())],
                                        ),
                                        Expression::function("tan", vec![Expression::symbol(var)]),
                                    ])],
                                )],
                            )
                        }),
                    },
                    result_template: "∫sec(x)dx = ln|sec(x)+tan(x)| + C".to_string(),
                    constant_handling: ConstantOfIntegration::AddConstant,
                }),
                special_values: vec![],
                identities: Box::new(vec![]),
                domain_range: Box::new(DomainRangeData {
                    domain: Domain::Real,
                    range: Range::Real,
                    singularities: vec![],
                }),
                periodicity: Some(Expression::mul(vec![
                    Expression::integer(2),
                    Expression::pi(),
                ])),
                numerical_evaluator: None,
            })),
        );

        self.properties.insert(
            "csc".to_string(),
            FunctionProperties::Elementary(Box::new(ElementaryProperties {
                evaluator: |args| {
                    if args.len() == 1 {
                        super::args[0].clone()
                    } else {
                        Expression::function("csc", args.to_vec())
                    }
                },

                derivative_rule: Some(DerivativeRule {
                    rule_type: DerivativeRuleType::Simple("-csc·cot".to_string()),
                    result_template: "-csc(x)·cot(x)".to_string(),
                }),
                antiderivative_rule: Some(AntiderivativeRule {
                    rule_type: AntiderivativeRuleType::Custom {
                        builder: Arc::new(|var: Symbol| {
                            Expression::mul(vec![
                                Expression::integer(-1),
                                Expression::function(
                                    "ln",
                                    vec![Expression::function(
                                        "abs",
                                        vec![Expression::add(vec![
                                            Expression::function(
                                                "csc",
                                                vec![Expression::symbol(var.clone())],
                                            ),
                                            Expression::function(
                                                "cot",
                                                vec![Expression::symbol(var)],
                                            ),
                                        ])],
                                    )],
                                ),
                            ])
                        }),
                    },
                    result_template: "∫csc(x)dx = -ln|csc(x)+cot(x)| + C".to_string(),
                    constant_handling: ConstantOfIntegration::AddConstant,
                }),
                special_values: vec![],
                identities: Box::new(vec![]),
                domain_range: Box::new(DomainRangeData {
                    domain: Domain::Real,
                    range: Range::Real,
                    singularities: vec![],
                }),
                periodicity: Some(Expression::mul(vec![
                    Expression::integer(2),
                    Expression::pi(),
                ])),
                numerical_evaluator: None,
            })),
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_circular_trig_intelligence() {
        let trig = CircularTrigIntelligence::new();

        assert!(trig.has_function("sin"));
        assert!(trig.has_function("cos"));
        assert!(trig.has_function("tan"));
        assert!(trig.has_function("cot"));
        assert!(trig.has_function("sec"));
        assert!(trig.has_function("csc"));
        assert!(!trig.has_function("arcsin"));

        let properties = trig.get_properties();
        assert_eq!(properties.len(), 6);
    }
}
