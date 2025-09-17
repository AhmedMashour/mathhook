//! Inverse Trigonometric Functions (arcsin, arccos, arctan, arcsec, arccsc, arccot)
//!
//! Complete mathematical intelligence for inverse trigonometric functions
//! with derivatives, domain restrictions, and special values.

use crate::core::Expression;
use crate::core::Symbol;
use crate::functions::properties::*;
use std::collections::HashMap;
use std::sync::Arc;

/// Inverse Trigonometric Function Intelligence
///
/// Dedicated intelligence for arcsin, arccos, arctan, arcsec, arccsc, arccot
pub struct InverseTrigIntelligence {
    properties: HashMap<String, FunctionProperties>,
}

impl Default for InverseTrigIntelligence {
    fn default() -> Self {
        Self::new()
    }
}

impl InverseTrigIntelligence {
    /// Create new inverse trigonometric intelligence system
    pub fn new() -> Self {
        let mut intelligence = Self {
            properties: HashMap::with_capacity(6),
        };

        intelligence.initialize_inverse_trig();

        intelligence
    }

    /// Get all inverse trigonometric function properties
    pub fn get_properties(&self) -> HashMap<String, FunctionProperties> {
        self.properties.clone()
    }

    /// Check if function is inverse trigonometric
    pub fn has_function(&self, name: &str) -> bool {
        self.properties.contains_key(name)
    }

    /// Initialize inverse trigonometric functions
    fn initialize_inverse_trig(&mut self) {
        self.properties.insert(
            "arcsin".to_owned(),
            FunctionProperties::Elementary(Box::new(ElementaryProperties {
                derivative_rule: Some(DerivativeRule {
                    rule_type: DerivativeRuleType::Custom {
                        builder: Arc::new(|arg: &Expression| {
                            Expression::pow(
                                Expression::function(
                                    "sqrt",
                                    vec![Expression::add(vec![
                                        Expression::integer(1),
                                        Expression::mul(vec![
                                            Expression::integer(-1),
                                            Expression::pow(arg.clone(), Expression::integer(2)),
                                        ]),
                                    ])],
                                ),
                                Expression::integer(-1),
                            )
                        }),
                    },
                    result_template: "1/√(1-x²)".to_owned(),
                }),
                antiderivative_rule: Some(AntiderivativeRule {
                    rule_type: AntiderivativeRuleType::Custom {
                        builder: Arc::new(|var: Symbol| {
                            Expression::add(vec![
                                Expression::mul(vec![
                                    Expression::symbol(var.clone()),
                                    Expression::function(
                                        "arcsin",
                                        vec![Expression::symbol(var.clone())],
                                    ),
                                ]),
                                Expression::function(
                                    "sqrt",
                                    vec![Expression::add(vec![
                                        Expression::integer(1),
                                        Expression::mul(vec![
                                            Expression::integer(-1),
                                            Expression::pow(
                                                Expression::symbol(var),
                                                Expression::integer(2),
                                            ),
                                        ]),
                                    ])],
                                ),
                            ])
                        }),
                    },
                    result_template: "∫arcsin(x)dx = x·arcsin(x) + √(1-x²) + C".to_owned(),
                    constant_handling: ConstantOfIntegration::AddConstant,
                }),
                special_values: vec![],
                identities: Box::new(vec![]),
                domain_range: Box::new(DomainRangeData {
                    domain: Domain::Interval(Expression::integer(-1), Expression::integer(1)),
                    range: Range::Bounded(
                        Expression::mul(vec![Expression::rational(-1, 2), Expression::pi()]),
                        Expression::mul(vec![Expression::rational(1, 2), Expression::pi()]),
                    ),
                    singularities: vec![],
                }),
                periodicity: None,
                wolfram_name: Some("ArcSin"),
            })),
        );

        self.properties.insert(
            "arccos".to_owned(),
            FunctionProperties::Elementary(Box::new(ElementaryProperties {
                derivative_rule: Some(DerivativeRule {
                    rule_type: DerivativeRuleType::Custom {
                        builder: Arc::new(|arg: &Expression| {
                            Expression::mul(vec![
                                Expression::integer(-1),
                                Expression::pow(
                                    Expression::function(
                                        "sqrt",
                                        vec![Expression::add(vec![
                                            Expression::integer(1),
                                            Expression::mul(vec![
                                                Expression::integer(-1),
                                                Expression::pow(
                                                    arg.clone(),
                                                    Expression::integer(2),
                                                ),
                                            ]),
                                        ])],
                                    ),
                                    Expression::integer(-1),
                                ),
                            ])
                        }),
                    },
                    result_template: "-1/√(1-x²)".to_owned(),
                }),
                antiderivative_rule: Some(AntiderivativeRule {
                    rule_type: AntiderivativeRuleType::Custom {
                        builder: Arc::new(|var: Symbol| {
                            Expression::add(vec![
                                Expression::mul(vec![
                                    Expression::symbol(var.clone()),
                                    Expression::function(
                                        "arccos",
                                        vec![Expression::symbol(var.clone())],
                                    ),
                                ]),
                                Expression::mul(vec![
                                    Expression::integer(-1),
                                    Expression::function(
                                        "sqrt",
                                        vec![Expression::add(vec![
                                            Expression::integer(1),
                                            Expression::mul(vec![
                                                Expression::integer(-1),
                                                Expression::pow(
                                                    Expression::symbol(var),
                                                    Expression::integer(2),
                                                ),
                                            ]),
                                        ])],
                                    ),
                                ]),
                            ])
                        }),
                    },
                    result_template: "∫arccos(x)dx = x·arccos(x) - √(1-x²) + C".to_owned(),
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
                wolfram_name: Some("ArcCos"),
            })),
        );

        self.properties.insert(
            "arctan".to_owned(),
            FunctionProperties::Elementary(Box::new(ElementaryProperties {
                derivative_rule: Some(DerivativeRule {
                    rule_type: DerivativeRuleType::Custom {
                        builder: Arc::new(|arg: &Expression| {
                            Expression::pow(
                                Expression::add(vec![
                                    Expression::integer(1),
                                    Expression::pow(arg.clone(), Expression::integer(2)),
                                ]),
                                Expression::integer(-1),
                            )
                        }),
                    },
                    result_template: "1/(1+x²)".to_owned(),
                }),
                antiderivative_rule: Some(AntiderivativeRule {
                    rule_type: AntiderivativeRuleType::Custom {
                        builder: Arc::new(|var: Symbol| {
                            Expression::add(vec![
                                Expression::mul(vec![
                                    Expression::symbol(var.clone()),
                                    Expression::function(
                                        "arctan",
                                        vec![Expression::symbol(var.clone())],
                                    ),
                                ]),
                                Expression::mul(vec![
                                    Expression::rational(-1, 2),
                                    Expression::function(
                                        "ln",
                                        vec![Expression::add(vec![
                                            Expression::integer(1),
                                            Expression::pow(
                                                Expression::symbol(var),
                                                Expression::integer(2),
                                            ),
                                        ])],
                                    ),
                                ]),
                            ])
                        }),
                    },
                    result_template: "∫arctan(x)dx = x·arctan(x) - ½ln(1+x²) + C".to_owned(),
                    constant_handling: ConstantOfIntegration::AddConstant,
                }),
                special_values: vec![],
                identities: Box::new(vec![]),
                domain_range: Box::new(DomainRangeData {
                    domain: Domain::Real,
                    range: Range::Bounded(
                        Expression::mul(vec![Expression::rational(-1, 2), Expression::pi()]),
                        Expression::mul(vec![Expression::rational(1, 2), Expression::pi()]),
                    ),
                    singularities: vec![],
                }),
                periodicity: None,
                wolfram_name: Some("ArcTan"),
            })),
        );

        self.properties.insert(
            "arcsec".to_owned(),
            FunctionProperties::Elementary(Box::new(ElementaryProperties {
                derivative_rule: Some(DerivativeRule {
                    rule_type: DerivativeRuleType::Custom {
                        builder: Arc::new(|arg: &Expression| {
                            let abs_arg = Expression::function("abs", vec![arg.clone()]);
                            let sqrt_term = Expression::function(
                                "sqrt",
                                vec![Expression::add(vec![
                                    Expression::pow(arg.clone(), Expression::integer(2)),
                                    Expression::integer(-1),
                                ])],
                            );
                            Expression::pow(
                                Expression::mul(vec![abs_arg, sqrt_term]),
                                Expression::integer(-1),
                            )
                        }),
                    },
                    result_template: "1/(|x|·√(x²-1))".to_owned(),
                }),
                antiderivative_rule: None,
                special_values: vec![],
                identities: Box::new(vec![]),
                domain_range: Box::new(DomainRangeData {
                    domain: Domain::Real,
                    range: Range::Bounded(Expression::integer(0), Expression::pi()),
                    singularities: vec![],
                }),
                periodicity: None,
                wolfram_name: Some("ArcSec"),
            })),
        );

        self.properties.insert(
            "arccsc".to_owned(),
            FunctionProperties::Elementary(Box::new(ElementaryProperties {
                derivative_rule: Some(DerivativeRule {
                    rule_type: DerivativeRuleType::Custom {
                        builder: Arc::new(|arg: &Expression| {
                            let abs_arg = Expression::function("abs", vec![arg.clone()]);
                            let sqrt_term = Expression::function(
                                "sqrt",
                                vec![Expression::add(vec![
                                    Expression::pow(arg.clone(), Expression::integer(2)),
                                    Expression::integer(-1),
                                ])],
                            );
                            Expression::mul(vec![
                                Expression::integer(-1),
                                Expression::pow(
                                    Expression::mul(vec![abs_arg, sqrt_term]),
                                    Expression::integer(-1),
                                ),
                            ])
                        }),
                    },
                    result_template: "-1/(|x|·√(x²-1))".to_owned(),
                }),
                antiderivative_rule: None,
                special_values: vec![],
                identities: Box::new(vec![]),
                domain_range: Box::new(DomainRangeData {
                    domain: Domain::Real,
                    range: Range::Bounded(
                        Expression::mul(vec![Expression::rational(-1, 2), Expression::pi()]),
                        Expression::mul(vec![Expression::rational(1, 2), Expression::pi()]),
                    ),
                    singularities: vec![],
                }),
                periodicity: None,
                wolfram_name: Some("ArcCsc"),
            })),
        );

        self.properties.insert(
            "arccot".to_owned(),
            FunctionProperties::Elementary(Box::new(ElementaryProperties {
                derivative_rule: Some(DerivativeRule {
                    rule_type: DerivativeRuleType::Custom {
                        builder: Arc::new(|arg: &Expression| {
                            Expression::mul(vec![
                                Expression::integer(-1),
                                Expression::pow(
                                    Expression::add(vec![
                                        Expression::integer(1),
                                        Expression::pow(arg.clone(), Expression::integer(2)),
                                    ]),
                                    Expression::integer(-1),
                                ),
                            ])
                        }),
                    },
                    result_template: "-1/(1+x²)".to_owned(),
                }),
                antiderivative_rule: None,
                special_values: vec![],
                identities: Box::new(vec![]),
                domain_range: Box::new(DomainRangeData {
                    domain: Domain::Real,
                    range: Range::Bounded(Expression::integer(0), Expression::pi()),
                    singularities: vec![],
                }),
                periodicity: None,
                wolfram_name: Some("ArcCot"),
            })),
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inverse_trig_intelligence() {
        let inv_trig = InverseTrigIntelligence::new();

        assert!(inv_trig.has_function("arcsin"));
        assert!(inv_trig.has_function("arccos"));
        assert!(inv_trig.has_function("arctan"));
        assert!(inv_trig.has_function("arcsec"));
        assert!(inv_trig.has_function("arccsc"));
        assert!(inv_trig.has_function("arccot"));
        assert!(!inv_trig.has_function("sin"));

        let properties = inv_trig.get_properties();
        assert_eq!(properties.len(), 6);
    }

    #[test]
    fn test_inverse_trig_derivative_rules() {
        let inv_trig = InverseTrigIntelligence::new();
        let properties = inv_trig.get_properties();

        if let Some(FunctionProperties::Elementary(arcsin_props)) = properties.get("arcsin") {
            assert!(arcsin_props.derivative_rule.is_some());
            let deriv = arcsin_props.derivative_rule.as_ref().unwrap();
            assert!(matches!(deriv.rule_type, DerivativeRuleType::Custom { .. }));
        } else {
            panic!("arcsin properties not found");
        }

        if let Some(FunctionProperties::Elementary(arctan_props)) = properties.get("arctan") {
            assert!(arctan_props.derivative_rule.is_some());
            let deriv = arctan_props.derivative_rule.as_ref().unwrap();
            assert!(matches!(deriv.rule_type, DerivativeRuleType::Custom { .. }));
        } else {
            panic!("arctan properties not found");
        }
    }
}
