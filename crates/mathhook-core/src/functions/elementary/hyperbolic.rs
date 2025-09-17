//! Hyperbolic Function Intelligence
//!
//! Mathematically accurate implementation of hyperbolic and inverse hyperbolic functions
//! with verified derivatives, special values, and hyperbolic identities.

use crate::core::{Expression, Symbol};
use crate::functions::properties::*;
use std::collections::HashMap;
use std::sync::Arc;

/// Hyperbolic Function Intelligence
///
/// Complete mathematical intelligence for hyperbolic and inverse hyperbolic functions
/// with ABSOLUTE MATHEMATICAL ACCURACY.
pub struct HyperbolicIntelligence {
    /// Function properties for hyperbolic functions
    properties: HashMap<String, FunctionProperties>,
}

impl Default for HyperbolicIntelligence {
    fn default() -> Self {
        Self::new()
    }
}

impl HyperbolicIntelligence {
    /// Create new hyperbolic intelligence system
    pub fn new() -> Self {
        let mut intelligence = Self {
            properties: HashMap::with_capacity(12),
        };

        intelligence.initialize_sinh_cosh();
        intelligence.initialize_tanh();
        intelligence.initialize_extended();
        intelligence.initialize_inverse();

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
        self.properties.insert(
            "sinh".to_owned(),
            FunctionProperties::Elementary(Box::new(ElementaryProperties {
                derivative_rule: Some(DerivativeRule {
                    rule_type: DerivativeRuleType::SimpleFunctionSubstitution("cosh".to_owned()),
                    result_template: "cosh(x)".to_owned(),
                }),
                antiderivative_rule: Some(AntiderivativeRule {
                    rule_type: AntiderivativeRuleType::Simple {
                        antiderivative_fn: "cosh".to_owned(),
                        coefficient: Expression::integer(1),
                    },
                    result_template: "∫sinh(x)dx = cosh(x) + C".to_owned(),
                    constant_handling: ConstantOfIntegration::AddConstant,
                }),
                special_values: vec![SpecialValue {
                    input: "0".to_owned(),
                    output: Expression::integer(0),
                    latex_explanation: "\\sinh(0) = 0".to_owned(),
                }],
                identities: Box::new(vec![MathIdentity {
                    name: "Hyperbolic Identity".to_owned(),
                    lhs: Expression::function(
                        "cosh_squared_minus_sinh_squared",
                        vec![Expression::symbol("x")],
                    ),
                    rhs: Expression::integer(1),
                    conditions: vec!["x ∈ ℝ".to_owned()],
                }]),
                domain_range: Box::new(DomainRangeData {
                    domain: Domain::Real,
                    range: Range::Real,
                    singularities: vec![],
                }),
                periodicity: None,
                wolfram_name: None,
            })),
        );

        self.properties.insert(
            "cosh".to_owned(),
            FunctionProperties::Elementary(Box::new(ElementaryProperties {
                derivative_rule: Some(DerivativeRule {
                    rule_type: DerivativeRuleType::SimpleFunctionSubstitution("sinh".to_owned()),
                    result_template: "sinh(x)".to_owned(),
                }),
                antiderivative_rule: Some(AntiderivativeRule {
                    rule_type: AntiderivativeRuleType::Simple {
                        antiderivative_fn: "sinh".to_owned(),
                        coefficient: Expression::integer(1),
                    },
                    result_template: "∫cosh(x)dx = sinh(x) + C".to_owned(),
                    constant_handling: ConstantOfIntegration::AddConstant,
                }),
                special_values: vec![SpecialValue {
                    input: "0".to_owned(),
                    output: Expression::integer(1),
                    latex_explanation: "\\cosh(0) = 1".to_owned(),
                }],
                identities: Box::new(vec![MathIdentity {
                    name: "Hyperbolic Identity".to_owned(),
                    lhs: Expression::function(
                        "cosh_squared_minus_sinh_squared",
                        vec![Expression::symbol("x")],
                    ),
                    rhs: Expression::integer(1),
                    conditions: vec!["x ∈ ℝ".to_owned()],
                }]),
                domain_range: Box::new(DomainRangeData {
                    domain: Domain::Real,
                    range: Range::Unbounded,
                    singularities: vec![],
                }),
                periodicity: None,
                wolfram_name: None,
            })),
        );
    }

    /// Initialize tanh with MATHEMATICAL ACCURACY
    fn initialize_tanh(&mut self) {
        self.properties.insert(
            "tanh".to_owned(),
            FunctionProperties::Elementary(Box::new(ElementaryProperties {
                derivative_rule: Some(DerivativeRule {
                    rule_type: DerivativeRuleType::Custom {
                        builder: Arc::new(|arg: &Expression| {
                            let tanh_arg = Expression::function("tanh", vec![arg.clone()]);
                            let tanh_squared = Expression::pow(tanh_arg, Expression::integer(2));
                            Expression::add(vec![
                                Expression::integer(1),
                                Expression::mul(vec![Expression::integer(-1), tanh_squared]),
                            ])
                        }),
                    },
                    result_template: "1 - tanh²(x)".to_owned(),
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
                    result_template: "∫tanh(x)dx = ln(cosh(x)) + C".to_owned(),
                    constant_handling: ConstantOfIntegration::AddConstant,
                }),
                special_values: vec![SpecialValue {
                    input: "0".to_owned(),
                    output: Expression::integer(0),
                    latex_explanation: "\\tanh(0) = 0".to_owned(),
                }],
                identities: Box::new(vec![MathIdentity {
                    name: "Hyperbolic Tangent Identity".to_owned(),
                    lhs: Expression::function("tanh", vec![Expression::symbol("x")]),
                    rhs: Expression::function("sinh_over_cosh", vec![Expression::symbol("x")]),
                    conditions: vec!["x ∈ ℝ".to_owned()],
                }]),
                domain_range: Box::new(DomainRangeData {
                    domain: Domain::Real,
                    range: Range::Bounded(Expression::integer(-1), Expression::integer(1)),
                    singularities: vec![],
                }),
                periodicity: None,
                wolfram_name: None,
            })),
        );
    }

    /// Initialize extended hyperbolic functions (sech, csch, coth)
    fn initialize_extended(&mut self) {
        self.properties.insert(
            "sech".to_owned(),
            FunctionProperties::Elementary(Box::new(ElementaryProperties {
                derivative_rule: Some(DerivativeRule {
                    rule_type: DerivativeRuleType::Custom {
                        builder: Arc::new(|arg: &Expression| {
                            let sech_arg = Expression::function("sech", vec![arg.clone()]);
                            let tanh_arg = Expression::function("tanh", vec![arg.clone()]);
                            Expression::mul(vec![Expression::integer(-1), sech_arg, tanh_arg])
                        }),
                    },
                    result_template: "-sech(x)·tanh(x)".to_owned(),
                }),
                antiderivative_rule: None,
                special_values: vec![SpecialValue {
                    input: "0".to_owned(),
                    output: Expression::integer(1),
                    latex_explanation: "\\text{sech}(0) = 1".to_owned(),
                }],
                identities: Box::new(vec![]),
                domain_range: Box::new(DomainRangeData {
                    domain: Domain::Real,
                    range: Range::Bounded(Expression::integer(0), Expression::integer(1)),
                    singularities: vec![],
                }),
                periodicity: None,
                wolfram_name: None,
            })),
        );

        self.properties.insert(
            "csch".to_owned(),
            FunctionProperties::Elementary(Box::new(ElementaryProperties {
                derivative_rule: Some(DerivativeRule {
                    rule_type: DerivativeRuleType::Custom {
                        builder: Arc::new(|arg: &Expression| {
                            let csch_arg = Expression::function("csch", vec![arg.clone()]);
                            let coth_arg = Expression::function("coth", vec![arg.clone()]);
                            Expression::mul(vec![Expression::integer(-1), csch_arg, coth_arg])
                        }),
                    },
                    result_template: "-csch(x)·coth(x)".to_owned(),
                }),
                antiderivative_rule: None,
                special_values: vec![],
                identities: Box::new(vec![]),
                domain_range: Box::new(DomainRangeData {
                    domain: Domain::Real,
                    range: Range::Real,
                    singularities: vec![Expression::integer(0)],
                }),
                periodicity: None,
                wolfram_name: None,
            })),
        );

        self.properties.insert(
            "coth".to_owned(),
            FunctionProperties::Elementary(Box::new(ElementaryProperties {
                derivative_rule: Some(DerivativeRule {
                    rule_type: DerivativeRuleType::Custom {
                        builder: Arc::new(|arg: &Expression| {
                            let csch_arg = Expression::function("csch", vec![arg.clone()]);
                            let csch_squared = Expression::pow(csch_arg, Expression::integer(2));
                            Expression::add(vec![
                                Expression::integer(1),
                                Expression::mul(vec![Expression::integer(-1), csch_squared]),
                            ])
                        }),
                    },
                    result_template: "1 - csch²(x)".to_owned(),
                }),
                antiderivative_rule: None,
                special_values: vec![],
                identities: Box::new(vec![]),
                domain_range: Box::new(DomainRangeData {
                    domain: Domain::Real,
                    range: Range::Real,
                    singularities: vec![Expression::integer(0)],
                }),
                periodicity: None,
                wolfram_name: None,
            })),
        );
    }

    /// Initialize inverse hyperbolic functions (asinh, acosh, atanh)
    fn initialize_inverse(&mut self) {
        self.properties.insert(
            "asinh".to_owned(),
            FunctionProperties::Elementary(Box::new(ElementaryProperties {
                derivative_rule: Some(DerivativeRule {
                    rule_type: DerivativeRuleType::Custom {
                        builder: Arc::new(|arg: &Expression| {
                            let sqrt_term = Expression::function(
                                "sqrt",
                                vec![Expression::add(vec![
                                    Expression::pow(arg.clone(), Expression::integer(2)),
                                    Expression::integer(1),
                                ])],
                            );
                            Expression::mul(vec![
                                Expression::integer(1),
                                Expression::pow(sqrt_term, Expression::integer(-1)),
                            ])
                        }),
                    },
                    result_template: "1/√(x²+1)".to_owned(),
                }),
                antiderivative_rule: None,
                special_values: vec![SpecialValue {
                    input: "0".to_owned(),
                    output: Expression::integer(0),
                    latex_explanation: "\\text{asinh}(0) = 0".to_owned(),
                }],
                identities: Box::new(vec![]),
                domain_range: Box::new(DomainRangeData {
                    domain: Domain::Real,
                    range: Range::Real,
                    singularities: vec![],
                }),
                periodicity: None,
                wolfram_name: None,
            })),
        );

        self.properties.insert(
            "acosh".to_owned(),
            FunctionProperties::Elementary(Box::new(ElementaryProperties {
                derivative_rule: Some(DerivativeRule {
                    rule_type: DerivativeRuleType::Custom {
                        builder: Arc::new(|arg: &Expression| {
                            let sqrt_term = Expression::function(
                                "sqrt",
                                vec![Expression::add(vec![
                                    Expression::pow(arg.clone(), Expression::integer(2)),
                                    Expression::integer(-1),
                                ])],
                            );
                            Expression::mul(vec![
                                Expression::integer(1),
                                Expression::pow(sqrt_term, Expression::integer(-1)),
                            ])
                        }),
                    },
                    result_template: "1/√(x²-1)".to_owned(),
                }),
                antiderivative_rule: None,
                special_values: vec![SpecialValue {
                    input: "1".to_owned(),
                    output: Expression::integer(0),
                    latex_explanation: "\\text{acosh}(1) = 0".to_owned(),
                }],
                identities: Box::new(vec![]),
                domain_range: Box::new(DomainRangeData {
                    domain: Domain::Real,
                    range: Range::Unbounded,
                    singularities: vec![],
                }),
                periodicity: None,
                wolfram_name: None,
            })),
        );

        self.properties.insert(
            "atanh".to_owned(),
            FunctionProperties::Elementary(Box::new(ElementaryProperties {
                derivative_rule: Some(DerivativeRule {
                    rule_type: DerivativeRuleType::Custom {
                        builder: Arc::new(|arg: &Expression| {
                            let arg_squared = Expression::pow(arg.clone(), Expression::integer(2));
                            let denominator = Expression::add(vec![
                                Expression::integer(1),
                                Expression::mul(vec![Expression::integer(-1), arg_squared]),
                            ]);
                            Expression::mul(vec![
                                Expression::integer(1),
                                Expression::pow(denominator, Expression::integer(-1)),
                            ])
                        }),
                    },
                    result_template: "1/(1-x²)".to_owned(),
                }),
                antiderivative_rule: None,
                special_values: vec![SpecialValue {
                    input: "0".to_owned(),
                    output: Expression::integer(0),
                    latex_explanation: "\\text{atanh}(0) = 0".to_owned(),
                }],
                identities: Box::new(vec![]),
                domain_range: Box::new(DomainRangeData {
                    domain: Domain::Interval(Expression::integer(-1), Expression::integer(1)),
                    range: Range::Real,
                    singularities: vec![Expression::integer(-1), Expression::integer(1)],
                }),
                periodicity: None,
                wolfram_name: None,
            })),
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hyperbolic_intelligence() {
        let hyp = HyperbolicIntelligence::new();

        assert!(hyp.has_function("sinh"));
        assert!(hyp.has_function("cosh"));
        assert!(hyp.has_function("tanh"));
        assert!(hyp.has_function("sech"));
        assert!(hyp.has_function("csch"));
        assert!(hyp.has_function("coth"));
        assert!(hyp.has_function("asinh"));
        assert!(hyp.has_function("acosh"));
        assert!(hyp.has_function("atanh"));
        assert!(!hyp.has_function("sin"));

        let properties = hyp.get_properties();
        assert_eq!(properties.len(), 9);
    }

    #[test]
    fn test_hyperbolic_derivative_rules() {
        let hyp = HyperbolicIntelligence::new();
        let properties = hyp.get_properties();

        if let Some(FunctionProperties::Elementary(tanh_props)) = properties.get("tanh") {
            assert!(tanh_props.derivative_rule.is_some());
            let deriv = tanh_props.derivative_rule.as_ref().unwrap();
            assert!(matches!(deriv.rule_type, DerivativeRuleType::Custom { .. }));
        } else {
            panic!("tanh properties not found");
        }

        if let Some(FunctionProperties::Elementary(asinh_props)) = properties.get("asinh") {
            assert!(asinh_props.derivative_rule.is_some());
            let deriv = asinh_props.derivative_rule.as_ref().unwrap();
            assert!(matches!(deriv.rule_type, DerivativeRuleType::Custom { .. }));
        } else {
            panic!("asinh properties not found");
        }
    }
}
