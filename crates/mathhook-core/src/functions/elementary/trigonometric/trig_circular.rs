//! Circular Trigonometric Functions (sin, cos, tan, cot, sec, csc)
//!
//! Complete mathematical intelligence for circular trigonometric functions
//! with derivatives, identities, and special values.

use crate::core::{Expression, Symbol};
use crate::expr;
use crate::symbol;

use crate::functions::properties::*;
use std::collections::HashMap;
use std::sync::Arc;

/// Circular Trigonometric Function Intelligence
///
/// Dedicated intelligence for sin, cos, tan, cot, sec, csc
pub struct CircularTrigIntelligence {
    properties: HashMap<String, FunctionProperties>,
}

impl Default for CircularTrigIntelligence {
    fn default() -> Self {
        Self::new()
    }
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

    fn initialize_sin_cos(&mut self) {
        self.properties.insert(
            "sin".to_owned(),
            FunctionProperties::Elementary(Box::new(ElementaryProperties {
                derivative_rule: Some(DerivativeRule {
                    rule_type: DerivativeRuleType::SimpleFunctionSubstitution("cos".to_owned()),
                    result_template: "cos(x)".to_owned(),
                }),
                antiderivative_rule: Some(AntiderivativeRule {
                    rule_type: AntiderivativeRuleType::Simple {
                        antiderivative_fn: "cos".to_owned(),
                        coefficient: expr!(-1),
                    },
                    result_template: "∫sin(x)dx = -cos(x) + C".to_owned(),
                    constant_handling: ConstantOfIntegration::AddConstant,
                }),
                special_values: vec![
                    SpecialValue {
                        input: "0".to_owned(),
                        output: expr!(0),
                        latex_explanation: "\\sin(0) = 0".to_owned(),
                    },
                    SpecialValue {
                        input: "π/2".to_owned(),
                        output: expr!(1),
                        latex_explanation: "\\sin(\\frac{\\pi}{2}) = 1".to_owned(),
                    },
                    SpecialValue {
                        input: "π".to_owned(),
                        output: expr!(0),
                        latex_explanation: "\\sin(\\pi) = 0".to_owned(),
                    },
                ],
                identities: Box::new(vec![MathIdentity {
                    name: "Pythagorean Identity".to_owned(),
                    lhs: Expression::add(vec![
                        Expression::pow(
                            Expression::function("sin", vec![symbol!(x).into()]),
                            expr!(2),
                        ),
                        Expression::pow(
                            Expression::function("cos", vec![symbol!(x).into()]),
                            expr!(2),
                        ),
                    ]),
                    rhs: expr!(1),
                    conditions: vec!["x ∈ ℝ".to_owned()],
                }]),
                domain_range: Box::new(DomainRangeData {
                    domain: Domain::Real,
                    range: Range::Bounded(expr!(-1), expr!(1)),
                    singularities: vec![],
                }),
                wolfram_name: Some("Sin"),
                periodicity: Some(Expression::mul(vec![expr!(2), Expression::pi()])),
            })),
        );

        self.properties.insert(
            "cos".to_owned(),
            FunctionProperties::Elementary(Box::new(ElementaryProperties {
                derivative_rule: Some(DerivativeRule {
                    rule_type: DerivativeRuleType::Custom {
                        builder: Arc::new(|arg: &Expression| {
                            Expression::mul(vec![
                                expr!(-1),
                                Expression::function("sin", vec![arg.clone()]),
                            ])
                        }),
                    },
                    result_template: "-sin(x)".to_owned(),
                }),
                antiderivative_rule: Some(AntiderivativeRule {
                    rule_type: AntiderivativeRuleType::Simple {
                        antiderivative_fn: "sin".to_owned(),
                        coefficient: expr!(1),
                    },
                    result_template: "∫cos(x)dx = sin(x) + C".to_owned(),
                    constant_handling: ConstantOfIntegration::AddConstant,
                }),
                special_values: vec![
                    SpecialValue {
                        input: "0".to_owned(),
                        output: expr!(1),
                        latex_explanation: "\\cos(0) = 1".to_owned(),
                    },
                    SpecialValue {
                        input: "π/2".to_owned(),
                        output: expr!(0),
                        latex_explanation: "\\cos(\\frac{\\pi}{2}) = 0".to_owned(),
                    },
                    SpecialValue {
                        input: "π".to_owned(),
                        output: expr!(-1),
                        latex_explanation: "\\cos(\\pi) = -1".to_owned(),
                    },
                    SpecialValue {
                        input: "3π/2".to_owned(),
                        output: expr!(0),
                        latex_explanation: "\\cos(\\frac{3\\pi}{2}) = 0".to_owned(),
                    },
                    SpecialValue {
                        input: "2π".to_owned(),
                        output: expr!(1),
                        latex_explanation: "\\cos(2\\pi) = 1".to_owned(),
                    },
                ],
                identities: Box::new(vec![]),
                domain_range: Box::new(DomainRangeData {
                    domain: Domain::Real,
                    range: Range::Bounded(expr!(-1), expr!(1)),
                    singularities: vec![],
                }),
                wolfram_name: Some("Cos"),
                periodicity: Some(Expression::mul(vec![expr!(2), Expression::pi()])),
            })),
        );
    }

    fn initialize_tan_cot(&mut self) {
        self.properties.insert(
            "tan".to_owned(),
            FunctionProperties::Elementary(Box::new(ElementaryProperties {
                derivative_rule: Some(DerivativeRule {
                    rule_type: DerivativeRuleType::Custom {
                        builder: Arc::new(|arg: &Expression| {
                            let sec_arg = Expression::function("sec", vec![arg.clone()]);
                            Expression::pow(sec_arg, expr!(2))
                        }),
                    },
                    result_template: "sec²(x)".to_owned(),
                }),
                antiderivative_rule: Some(AntiderivativeRule {
                    rule_type: AntiderivativeRuleType::Custom {
                        builder: Arc::new(|var: Symbol| {
                            Expression::mul(vec![
                                expr!(-1),
                                Expression::function(
                                    "ln",
                                    vec![Expression::function(
                                        "abs",
                                        vec![Expression::function("cos", vec![var.into()])],
                                    )],
                                ),
                            ])
                        }),
                    },
                    result_template: "∫tan(x)dx = -ln|cos(x)| + C".to_owned(),
                    constant_handling: ConstantOfIntegration::AddConstant,
                }),
                special_values: vec![
                    SpecialValue {
                        input: "0".to_owned(),
                        output: expr!(0),
                        latex_explanation: "\\tan(0) = 0".to_owned(),
                    },
                    SpecialValue {
                        input: "π/4".to_owned(),
                        output: expr!(1),
                        latex_explanation: "\\tan(\\frac{\\pi}{4}) = 1".to_owned(),
                    },
                ],
                identities: Box::new(vec![MathIdentity {
                    name: "Tangent Identity".to_owned(),
                    lhs: Expression::function("tan", vec![symbol!(x).into()]),
                    rhs: Expression::function("sin_over_cos", vec![symbol!(x).into()]),
                    conditions: vec!["cos(x) ≠ 0".to_owned()],
                }]),
                domain_range: Box::new(DomainRangeData {
                    domain: Domain::Real,
                    range: Range::Real,
                    singularities: vec![expr!((pi) * (1 / 2))],
                }),
                wolfram_name: Some("Tan"),
                periodicity: Some(Expression::pi()),
            })),
        );

        self.properties.insert(
            "cot".to_owned(),
            FunctionProperties::Elementary(Box::new(ElementaryProperties {
                derivative_rule: Some(DerivativeRule {
                    rule_type: DerivativeRuleType::Custom {
                        builder: Arc::new(|arg: &Expression| {
                            let csc_arg = Expression::function("csc", vec![arg.clone()]);
                            let csc_squared = Expression::pow(csc_arg, expr!(2));
                            Expression::mul(vec![expr!(-1), csc_squared])
                        }),
                    },
                    result_template: "-csc²(x)".to_owned(),
                }),
                antiderivative_rule: Some(AntiderivativeRule {
                    rule_type: AntiderivativeRuleType::Custom {
                        builder: Arc::new(|var: Symbol| {
                            Expression::function(
                                "ln",
                                vec![Expression::function(
                                    "abs",
                                    vec![Expression::function("sin", vec![var.into()])],
                                )],
                            )
                        }),
                    },
                    result_template: "∫cot(x)dx = ln|sin(x)| + C".to_owned(),
                    constant_handling: ConstantOfIntegration::AddConstant,
                }),
                special_values: vec![SpecialValue {
                    input: "π/4".to_owned(),
                    output: expr!(1),
                    latex_explanation: "\\cot(\\frac{\\pi}{4}) = 1".to_owned(),
                }],
                identities: Box::new(vec![MathIdentity {
                    name: "Cotangent Identity".to_owned(),
                    lhs: Expression::function("cot", vec![symbol!(x).into()]),
                    rhs: Expression::function("cos_over_sin", vec![symbol!(x).into()]),
                    conditions: vec!["sin(x) ≠ 0".to_owned()],
                }]),
                domain_range: Box::new(DomainRangeData {
                    domain: Domain::Real,
                    range: Range::Real,
                    singularities: vec![expr!(0)],
                }),
                wolfram_name: Some("Cot"),
                periodicity: Some(Expression::pi()),
            })),
        );
    }

    fn initialize_sec_csc(&mut self) {
        self.properties.insert(
            "sec".to_owned(),
            FunctionProperties::Elementary(Box::new(ElementaryProperties {
                derivative_rule: Some(DerivativeRule {
                    rule_type: DerivativeRuleType::Custom {
                        builder: Arc::new(|arg: &Expression| {
                            let sec_arg = Expression::function("sec", vec![arg.clone()]);
                            let tan_arg = Expression::function("tan", vec![arg.clone()]);
                            Expression::mul(vec![sec_arg, tan_arg])
                        }),
                    },
                    result_template: "sec(x)·tan(x)".to_owned(),
                }),
                antiderivative_rule: Some(AntiderivativeRule {
                    rule_type: AntiderivativeRuleType::Custom {
                        builder: Arc::new(|var: Symbol| {
                            Expression::function(
                                "ln",
                                vec![Expression::function(
                                    "abs",
                                    vec![Expression::add(vec![
                                        Expression::function("sec", vec![var.clone().into()]),
                                        Expression::function("tan", vec![var.into()]),
                                    ])],
                                )],
                            )
                        }),
                    },
                    result_template: "∫sec(x)dx = ln|sec(x)+tan(x)| + C".to_owned(),
                    constant_handling: ConstantOfIntegration::AddConstant,
                }),
                special_values: vec![],
                identities: Box::new(vec![]),
                domain_range: Box::new(DomainRangeData {
                    domain: Domain::Real,
                    range: Range::Real,
                    singularities: vec![expr!((pi) * (1 / 2))],
                }),
                wolfram_name: Some("Sec"),
                periodicity: Some(Expression::mul(vec![expr!(2), Expression::pi()])),
            })),
        );

        self.properties.insert(
            "csc".to_owned(),
            FunctionProperties::Elementary(Box::new(ElementaryProperties {
                derivative_rule: Some(DerivativeRule {
                    rule_type: DerivativeRuleType::Custom {
                        builder: Arc::new(|arg: &Expression| {
                            let csc_arg = Expression::function("csc", vec![arg.clone()]);
                            let cot_arg = Expression::function("cot", vec![arg.clone()]);
                            Expression::mul(vec![expr!(-1), csc_arg, cot_arg])
                        }),
                    },
                    result_template: "-csc(x)·cot(x)".to_owned(),
                }),
                antiderivative_rule: Some(AntiderivativeRule {
                    rule_type: AntiderivativeRuleType::Custom {
                        builder: Arc::new(|var: Symbol| {
                            Expression::mul(vec![
                                expr!(-1),
                                Expression::function(
                                    "ln",
                                    vec![Expression::function(
                                        "abs",
                                        vec![Expression::add(vec![
                                            Expression::function("csc", vec![var.clone().into()]),
                                            Expression::function("cot", vec![var.into()]),
                                        ])],
                                    )],
                                ),
                            ])
                        }),
                    },
                    result_template: "∫csc(x)dx = -ln|csc(x)+cot(x)| + C".to_owned(),
                    constant_handling: ConstantOfIntegration::AddConstant,
                }),
                special_values: vec![],
                identities: Box::new(vec![]),
                domain_range: Box::new(DomainRangeData {
                    domain: Domain::Real,
                    range: Range::Real,
                    singularities: vec![expr!(0)],
                }),
                wolfram_name: Some("Csc"),
                periodicity: Some(Expression::mul(vec![expr!(2), Expression::pi()])),
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

    #[test]
    fn test_circular_trig_derivative_rules() {
        let trig = CircularTrigIntelligence::new();
        let properties = trig.get_properties();

        if let Some(FunctionProperties::Elementary(tan_props)) = properties.get("tan") {
            assert!(tan_props.derivative_rule.is_some());
            let deriv = tan_props.derivative_rule.as_ref().unwrap();
            assert!(matches!(deriv.rule_type, DerivativeRuleType::Custom { .. }));
        } else {
            panic!("tan properties not found");
        }

        if let Some(FunctionProperties::Elementary(sec_props)) = properties.get("sec") {
            assert!(sec_props.derivative_rule.is_some());
            let deriv = sec_props.derivative_rule.as_ref().unwrap();
            assert!(matches!(deriv.rule_type, DerivativeRuleType::Custom { .. }));
        } else {
            panic!("sec properties not found");
        }
    }
}
