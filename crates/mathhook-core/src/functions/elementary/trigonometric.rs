//! Trigonometric Function Intelligence
//!
//! Complete mathematical intelligence for trigonometric functions:
//! sin, cos, tan, sec, csc, cot with derivatives, identities, and special values.

use crate::core::{Expression, Symbol};
use crate::functions::properties::*;
use std::collections::HashMap;
use std::sync::Arc;

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
        intelligence.initialize_inverse_trig();

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

        // Cos function with complete mathematical properties
        self.properties.insert(
            "cos".to_string(),
            FunctionProperties::Elementary(Box::new(ElementaryProperties {
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
                antiderivative_rule: Some(AntiderivativeRule {
                    rule_type: AntiderivativeRuleType::Custom {
                        builder: Arc::new(|var: Symbol| {
                            Expression::mul(vec![
                                Expression::integer(-1),
                                Expression::function("ln", vec![
                                    Expression::function("abs", vec![
                                        Expression::function("cos", vec![Expression::symbol(var)])
                                    ])
                                ]),
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

        // Cotangent function
        self.properties.insert(
            "cot".to_string(),
            FunctionProperties::Elementary(Box::new(ElementaryProperties {
                derivative_rule: Some(DerivativeRule {
                    rule_type: DerivativeRuleType::Simple("-csc²".to_string()),
                    result_template: "-csc²(x)".to_string(),
                }),
                antiderivative_rule: Some(AntiderivativeRule {
                    rule_type: AntiderivativeRuleType::Custom {
                        builder: Arc::new(|var: Symbol| {
                            Expression::function("ln", vec![
                                Expression::function("abs", vec![
                                    Expression::function("sin", vec![Expression::symbol(var)])
                                ])
                            ])
                        }),
                    },
                    result_template: "∫cot(x)dx = ln|sin(x)| + C".to_string(),
                    constant_handling: ConstantOfIntegration::AddConstant,
                }),
                special_values: vec![
                    SpecialValue {
                        input: "π/4".to_string(),
                        output: Expression::integer(1),
                        latex_explanation: "\\cot(\\frac{\\pi}{4}) = 1".to_string(),
                    },
                ],
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
                derivative_rule: Some(DerivativeRule {
                    rule_type: DerivativeRuleType::Simple("sec·tan".to_string()),
                    result_template: "sec(x)·tan(x)".to_string(),
                }),
                antiderivative_rule: Some(AntiderivativeRule {
                    rule_type: AntiderivativeRuleType::Custom {
                        builder: Arc::new(|var: Symbol| {
                            Expression::function("ln", vec![
                                Expression::function("abs", vec![
                                    Expression::add(vec![
                                        Expression::function("sec", vec![Expression::symbol(var.clone())]),
                                        Expression::function("tan", vec![Expression::symbol(var)]),
                                    ])
                                ])
                            ])
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
                derivative_rule: Some(DerivativeRule {
                    rule_type: DerivativeRuleType::Simple("-csc·cot".to_string()),
                    result_template: "-csc(x)·cot(x)".to_string(),
                }),
                antiderivative_rule: Some(AntiderivativeRule {
                    rule_type: AntiderivativeRuleType::Custom {
                        builder: Arc::new(|var: Symbol| {
                            Expression::mul(vec![
                                Expression::integer(-1),
                                Expression::function("ln", vec![
                                    Expression::function("abs", vec![
                                        Expression::add(vec![
                                            Expression::function("csc", vec![Expression::symbol(var.clone())]),
                                            Expression::function("cot", vec![Expression::symbol(var)]),
                                        ])
                                    ])
                                ]),
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

    /// Initialize inverse trigonometric functions
    fn initialize_inverse_trig(&mut self) {
        self.properties.insert(
            "arcsin".to_string(),
            FunctionProperties::Elementary(Box::new(ElementaryProperties {
                derivative_rule: Some(DerivativeRule {
                    rule_type: DerivativeRuleType::Simple("1/√(1-x²)".to_string()),
                    result_template: "1/√(1-x²)".to_string(),
                }),
                antiderivative_rule: Some(AntiderivativeRule {
                    rule_type: AntiderivativeRuleType::Custom {
                        builder: Arc::new(|var: Symbol| {
                            Expression::add(vec![
                                Expression::mul(vec![
                                    Expression::symbol(var.clone()),
                                    Expression::function("arcsin", vec![Expression::symbol(var.clone())]),
                                ]),
                                Expression::function("sqrt", vec![
                                    Expression::add(vec![
                                        Expression::integer(1),
                                        Expression::mul(vec![
                                            Expression::integer(-1),
                                            Expression::pow(Expression::symbol(var), Expression::integer(2)),
                                        ]),
                                    ])
                                ]),
                            ])
                        }),
                    },
                    result_template: "∫arcsin(x)dx = x·arcsin(x) + √(1-x²) + C".to_string(),
                    constant_handling: ConstantOfIntegration::AddConstant,
                }),
                special_values: vec![],
                identities: Box::new(vec![]),
                domain_range: Box::new(DomainRangeData {
                    domain: Domain::Interval(Expression::integer(-1), Expression::integer(1)),
                    range: Range::Bounded(
                        Expression::mul(vec![
                            Expression::rational(-1, 2),
                            Expression::pi(),
                        ]),
                        Expression::mul(vec![
                            Expression::rational(1, 2),
                            Expression::pi(),
                        ]),
                    ),
                    singularities: vec![],
                }),
                periodicity: None,
                numerical_evaluator: Some(
                    crate::functions::properties::NumericalEvaluator::StandardLib(f64::asin),
                ),
            })),
        );

        self.properties.insert(
            "arccos".to_string(),
            FunctionProperties::Elementary(Box::new(ElementaryProperties {
                derivative_rule: Some(DerivativeRule {
                    rule_type: DerivativeRuleType::Simple("-1/√(1-x²)".to_string()),
                    result_template: "-1/√(1-x²)".to_string(),
                }),
                antiderivative_rule: Some(AntiderivativeRule {
                    rule_type: AntiderivativeRuleType::Custom {
                        builder: Arc::new(|var: Symbol| {
                            Expression::add(vec![
                                Expression::mul(vec![
                                    Expression::symbol(var.clone()),
                                    Expression::function("arccos", vec![Expression::symbol(var.clone())]),
                                ]),
                                Expression::mul(vec![
                                    Expression::integer(-1),
                                    Expression::function("sqrt", vec![
                                        Expression::add(vec![
                                            Expression::integer(1),
                                            Expression::mul(vec![
                                                Expression::integer(-1),
                                                Expression::pow(Expression::symbol(var), Expression::integer(2)),
                                            ]),
                                        ])
                                    ]),
                                ]),
                            ])
                        }),
                    },
                    result_template: "∫arccos(x)dx = x·arccos(x) - √(1-x²) + C".to_string(),
                    constant_handling: ConstantOfIntegration::AddConstant,
                }),
                special_values: vec![],
                identities: Box::new(vec![]),
                domain_range: Box::new(DomainRangeData {
                    domain: Domain::Interval(Expression::integer(-1), Expression::integer(1)),
                    range: Range::Bounded(Expression::integer(0), Expression::pi()),
                    singularities: vec![],
                }),
                periodicity: None,
                numerical_evaluator: Some(
                    crate::functions::properties::NumericalEvaluator::StandardLib(f64::acos),
                ),
            })),
        );

        self.properties.insert(
            "arctan".to_string(),
            FunctionProperties::Elementary(Box::new(ElementaryProperties {
                derivative_rule: Some(DerivativeRule {
                    rule_type: DerivativeRuleType::Simple("1/(1+x²)".to_string()),
                    result_template: "1/(1+x²)".to_string(),
                }),
                antiderivative_rule: Some(AntiderivativeRule {
                    rule_type: AntiderivativeRuleType::Custom {
                        builder: Arc::new(|var: Symbol| {
                            Expression::add(vec![
                                Expression::mul(vec![
                                    Expression::symbol(var.clone()),
                                    Expression::function("arctan", vec![Expression::symbol(var.clone())]),
                                ]),
                                Expression::mul(vec![
                                    Expression::rational(-1, 2),
                                    Expression::function("ln", vec![
                                        Expression::add(vec![
                                            Expression::integer(1),
                                            Expression::pow(Expression::symbol(var), Expression::integer(2)),
                                        ])
                                    ]),
                                ]),
                            ])
                        }),
                    },
                    result_template: "∫arctan(x)dx = x·arctan(x) - ½ln(1+x²) + C".to_string(),
                    constant_handling: ConstantOfIntegration::AddConstant,
                }),
                special_values: vec![],
                identities: Box::new(vec![]),
                domain_range: Box::new(DomainRangeData {
                    domain: Domain::Real,
                    range: Range::Bounded(
                        Expression::mul(vec![
                            Expression::rational(-1, 2),
                            Expression::pi(),
                        ]),
                        Expression::mul(vec![
                            Expression::rational(1, 2),
                            Expression::pi(),
                        ]),
                    ),
                    singularities: vec![],
                }),
                periodicity: None,
                numerical_evaluator: Some(
                    crate::functions::properties::NumericalEvaluator::StandardLib(f64::atan),
                ),
            })),
        );
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
