//! Mathematical Function Properties
//!
//! Defines the mathematical properties and capabilities for all function types.
//! Inspired by SymPy's comprehensive function system but optimized for performance.

pub mod elementary;
pub mod rules;
pub mod special;

pub use rules::{
    AntiderivativeRule, AntiderivativeRuleType, ConstantOfIntegration, DerivativeRule,
    DerivativeRuleType, Domain, DomainRangeData, EvaluationMethod, MathIdentity, Range,
    RecurrenceRule, SpecialValue, ThreeTermRecurrence,
};

pub use elementary::{ElementaryProperties, UserProperties, UserProperty};
pub use special::{
    AsymptoticData, DifferentialEquation, GeneratingFunction, GeneratingFunctionType,
    OrthogonalityData, PolynomialFamily, PolynomialProperties, RodriguesFormula, SpecialProperties,
};

use crate::core::Expression;

/// Mathematical properties for all function types
///
/// Boxed variants to minimize memory footprint
/// while providing comprehensive mathematical intelligence.
///
/// ## Memory Layout Optimization
/// - Uses `Box<T>` to keep enum size small (8 bytes per variant)
/// - Reduces memory fragmentation through consistent allocation patterns
/// - Enables efficient cache utilization for hot path operations
#[derive(Debug, Clone)]
pub enum FunctionProperties {
    /// Elementary functions: sin, cos, exp, log, etc.
    Elementary(Box<ElementaryProperties>),

    /// Special functions: gamma, bessel, zeta, etc.
    Special(Box<SpecialProperties>),

    /// Polynomial families: legendre, hermite, laguerre, etc.
    Polynomial(Box<PolynomialProperties>),

    /// User-defined functions: f, g, h, etc.
    UserDefined(Box<UserProperties>),
}

impl FunctionProperties {
    /// Check if function has derivative rule
    ///
    /// Hot path method for performance-critical operations
    #[inline(always)]
    pub fn has_derivative(&self) -> bool {
        match self {
            FunctionProperties::Elementary(props) => props.derivative_rule.is_some(),
            FunctionProperties::Special(props) => props.has_derivative,
            FunctionProperties::Polynomial(_props) => true,
            FunctionProperties::UserDefined(_) => false,
        }
    }

    /// Check if function has antiderivative rule
    ///
    /// Hot path method for performance-critical operations
    #[inline(always)]
    pub fn has_antiderivative(&self) -> bool {
        match self {
            FunctionProperties::Elementary(props) => props.antiderivative_rule.is_some(),
            FunctionProperties::Special(props) => props.has_antiderivative,
            FunctionProperties::Polynomial(_props) => true,
            FunctionProperties::UserDefined(_) => false,
        }
    }

    /// Get derivative rule if available
    ///
    /// Returns a reference to the derivative rule for registry-based differentiation
    #[inline(always)]
    pub fn get_derivative_rule(&self) -> Option<&DerivativeRule> {
        match self {
            FunctionProperties::Elementary(props) => props.derivative_rule.as_ref(),
            FunctionProperties::Special(_props) => None,
            FunctionProperties::Polynomial(_props) => None,
            FunctionProperties::UserDefined(_) => None,
        }
    }

    /// Get derivative expression using registry rules
    ///
    /// Computes the derivative of f(arg) with respect to arg using the
    /// registered derivative rule. The chain rule (multiplying by d(arg)/dx)
    /// must be applied separately.
    ///
    /// # Arguments
    ///
    /// * `arg` - The argument expression to the function
    ///
    /// # Returns
    ///
    /// Returns the derivative expression, or None if no derivative rule exists
    ///
    /// # Examples
    ///
    /// ```
    /// use mathhook_core::functions::intelligence::get_universal_registry;
    /// use mathhook_core::{expr, symbol};
    ///
    /// let registry = get_universal_registry();
    /// let x = symbol!(x);
    ///
    /// if let Some(props) = registry.get_properties("sin") {
    ///     let derivative = props.get_derivative_expression(&x.into());
    /// }
    /// ```
    pub fn get_derivative_expression(&self, arg: &Expression) -> Option<Expression> {
        let rule = self.get_derivative_rule()?;

        match &rule.rule_type {
            DerivativeRuleType::SimpleFunctionSubstitution(func_name) => {
                Some(Expression::function(func_name, vec![arg.clone()]))
            }
            DerivativeRuleType::Custom { builder } => Some(builder(arg)),
            DerivativeRuleType::ChainRule(_) => None,
            DerivativeRuleType::ProductRule => None,
            DerivativeRuleType::QuotientRule => None,
        }
    }

    /// Get antiderivative rule if available
    ///
    /// Returns a reference to the antiderivative rule for registry-based integration
    #[inline(always)]
    pub fn get_antiderivative_rule(&self) -> Option<&AntiderivativeRule> {
        match self {
            FunctionProperties::Elementary(props) => props.antiderivative_rule.as_ref(),
            FunctionProperties::Special(props) => props.antiderivative_rule.as_ref(),
            FunctionProperties::Polynomial(props) => Some(&props.antiderivative_rule),
            FunctionProperties::UserDefined(_) => None,
        }
    }

    /// Get special value count for caching optimization
    #[inline(always)]
    pub fn special_value_count(&self) -> usize {
        match self {
            FunctionProperties::Elementary(props) => props.special_values.len(),
            FunctionProperties::Special(props) => props.special_values.len(),
            FunctionProperties::Polynomial(props) => props.special_values.len(),
            FunctionProperties::UserDefined(_) => 0,
        }
    }

    /// Get function family for quick classification
    #[inline(always)]
    pub fn family(&self) -> super::intelligence::FunctionFamily {
        match self {
            FunctionProperties::Elementary(_) => super::intelligence::FunctionFamily::Elementary,
            FunctionProperties::Special(_) => super::intelligence::FunctionFamily::Special,
            FunctionProperties::Polynomial(_) => super::intelligence::FunctionFamily::Polynomial,
            FunctionProperties::UserDefined(_) => super::intelligence::FunctionFamily::UserDefined,
        }
    }

    /// Get Wolfram Language function name
    ///
    /// Used for Wolfram formatting without hardcoded function name matching.
    /// Returns the Wolfram name if registered, otherwise None.
    ///
    /// # Examples
    ///
    /// ```
    /// use mathhook_core::functions::intelligence::get_universal_registry;
    ///
    /// let registry = get_universal_registry();
    /// if let Some(props) = registry.get_properties("sin") {
    ///     assert_eq!(props.wolfram_name(), Some("Sin"));
    /// }
    /// if let Some(props) = registry.get_properties("ln") {
    ///     assert_eq!(props.wolfram_name(), Some("Log"));
    /// }
    /// ```
    #[inline(always)]
    pub fn wolfram_name(&self) -> Option<&'static str> {
        match self {
            FunctionProperties::Elementary(props) => props.wolfram_name,
            FunctionProperties::Special(props) => props.wolfram_name,
            FunctionProperties::Polynomial(props) => props.wolfram_name,
            FunctionProperties::UserDefined(props) => props.wolfram_name,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::expr;
    use crate::symbol;

    #[test]
    fn test_function_properties_size() {
        use std::mem::size_of;

        assert!(
            size_of::<FunctionProperties>() <= 32,
            "FunctionProperties size: {} bytes (expected <= 32)",
            size_of::<FunctionProperties>()
        );

        assert!(
            size_of::<ElementaryProperties>() <= 256,
            "ElementaryProperties size: {} bytes (expected <= 256)",
            size_of::<ElementaryProperties>()
        );
    }

    #[test]
    fn test_hot_path_methods() {
        let props: FunctionProperties =
            FunctionProperties::Elementary(Box::new(ElementaryProperties {
                derivative_rule: Some(DerivativeRule {
                    rule_type: DerivativeRuleType::SimpleFunctionSubstitution("cos".to_string()),
                    result_template: "cos(x)".to_string(),
                }),
                antiderivative_rule: Some(AntiderivativeRule {
                    rule_type: AntiderivativeRuleType::Simple {
                        antiderivative_fn: "cos".to_string(),
                        coefficient: Expression::integer(-1),
                    },
                    result_template: "-cos(x) + C".to_string(),
                    constant_handling: ConstantOfIntegration::AddConstant,
                }),
                special_values: vec![],
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
                wolfram_name: Some("Sin"),
            }));

        assert!(props.has_derivative());
        assert!(props.has_antiderivative());
        assert_eq!(props.special_value_count(), 0);
        assert_eq!(props.wolfram_name(), Some("Sin"));

        let rule = props.get_antiderivative_rule();
        assert!(rule.is_some());
        if let Some(r) = rule {
            assert_eq!(r.result_template, "-cos(x) + C");
            assert_eq!(r.constant_handling, ConstantOfIntegration::AddConstant);
        }
    }

    #[test]
    fn test_derivative_expression_simple() {
        let x = symbol!(x);
        let props: FunctionProperties =
            FunctionProperties::Elementary(Box::new(ElementaryProperties {
                derivative_rule: Some(DerivativeRule {
                    rule_type: DerivativeRuleType::SimpleFunctionSubstitution("cos".to_string()),
                    result_template: "cos(x)".to_string(),
                }),
                antiderivative_rule: None,
                special_values: vec![],
                identities: Box::new(vec![]),
                domain_range: Box::new(DomainRangeData {
                    domain: Domain::Real,
                    range: Range::Bounded(expr!(-1), expr!(1)),
                    singularities: vec![],
                }),
                periodicity: None,
                wolfram_name: Some("Sin"),
            }));

        let derivative = props.get_derivative_expression(&x.into());
        assert!(derivative.is_some());

        if let Some(d) = derivative {
            assert_eq!(d.to_string(), "cos(x)");
        }
    }

    #[test]
    fn test_wolfram_name_getter() {
        let props_sin: FunctionProperties =
            FunctionProperties::Elementary(Box::new(ElementaryProperties {
                derivative_rule: None,
                antiderivative_rule: None,
                special_values: vec![],
                identities: Box::new(vec![]),
                domain_range: Box::new(DomainRangeData {
                    domain: Domain::Real,
                    range: Range::Bounded(expr!(-1), expr!(1)),
                    singularities: vec![],
                }),
                periodicity: None,
                wolfram_name: Some("Sin"),
            }));

        assert_eq!(props_sin.wolfram_name(), Some("Sin"));

        let props_user: FunctionProperties =
            FunctionProperties::UserDefined(Box::new(UserProperties {
                definition: None,
                properties: vec![],
                derivatives: std::collections::HashMap::new(),
                domain: None,
                wolfram_name: None,
            }));

        assert_eq!(props_user.wolfram_name(), None);
    }
}
