//! Mathematical Function Properties
//!
//! Defines the mathematical properties and capabilities for all function types.
//! Inspired by SymPy's comprehensive function system but optimized for performance.

pub mod elementary;
pub mod rules;
pub mod special;

use crate::core::Expression;
use crate::functions::evaluation::EvaluationResult;

pub use rules::{
    AntiderivativeRule, AntiderivativeRuleType, ConstantOfIntegration, DerivativeRule,
    DerivativeRuleType, Domain, DomainRangeData, EvaluationMethod, MathIdentity,
    NumericalEvaluator, Range, RecurrenceRule, SpecialValue, ThreeTermRecurrence,
};

pub use elementary::{ElementaryProperties, UserProperties, UserProperty};
pub use special::{
    AsymptoticData, DifferentialEquation, GeneratingFunction, GeneratingFunctionType,
    OrthogonalityData, PolynomialFamily, PolynomialProperties, RodriguesFormula, SpecialProperties,
};

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

    /// Evaluate function using its mathematical intelligence
    ///
    /// This is the core of truly intelligent evaluation - each function
    /// evaluates itself based on its mathematical properties
    pub fn evaluate(&self, name: &str, args: &[Expression]) -> EvaluationResult {
        if let Some(result) = self.try_special_values(args) {
            return result;
        }

        if let Some(result) = self.try_existing_operations(name, args) {
            return result;
        }

        EvaluationResult::Unevaluated
    }

    /// Try evaluation using special values from intelligence
    fn try_special_values(&self, args: &[Expression]) -> Option<EvaluationResult> {
        if args.len() != 1 {
            return None;
        }

        let special_values = match self {
            FunctionProperties::Elementary(props) => &props.special_values,
            FunctionProperties::Special(props) => &props.special_values,
            FunctionProperties::Polynomial(props) => &props.special_values,
            _ => return None,
        };

        for special_value in special_values {
            if self.matches_pattern(&args[0], &special_value.input) {
                return Some(EvaluationResult::Exact(special_value.output.clone()));
            }
        }

        None
    }

    /// Try evaluation using existing mathematical operations
    ///
    /// Intelligence-driven evaluation using function properties
    fn try_existing_operations(&self, name: &str, args: &[Expression]) -> Option<EvaluationResult> {
        if args.len() == 2 && self.is_binary_number_theory_function() {
            self.evaluate_binary_number_theory(name, args)
        } else {
            None
        }
    }

    /// Check if this is a binary number theory function using properties
    fn is_binary_number_theory_function(&self) -> bool {
        match self {
            FunctionProperties::Elementary(props) => {
                matches!(
                    props.domain_range.domain,
                    Domain::Integer | Domain::PositiveInteger
                )
            }
            _ => false,
        }
    }

    /// Evaluate binary number theory operations using existing implementations
    ///
    /// Intelligence-driven evaluation - uses function properties to determine operation
    fn evaluate_binary_number_theory(
        &self,
        name: &str,
        args: &[Expression],
    ) -> Option<EvaluationResult> {
        match self {
            FunctionProperties::Elementary(props) => match props.domain_range.domain {
                Domain::Integer | Domain::PositiveInteger => {
                    self.apply_number_theory_algorithm(name, args)
                }
                _ => None,
            },
            _ => None,
        }
    }

    /// Apply number theory algorithm using existing implementations
    ///
    /// Maps function intelligence to existing mathematical algorithms
    fn apply_number_theory_algorithm(
        &self,
        name: &str,
        args: &[Expression],
    ) -> Option<EvaluationResult> {
        match name {
            "gcd" => Some(EvaluationResult::Exact(args[0].gcd(&args[1]))),
            "lcm" => Some(EvaluationResult::Exact(args[0].lcm(&args[1]))),
            _ => None,
        }
    }

    /// Check if expression matches special value
    fn matches_pattern(&self, expr: &Expression, pattern: &str) -> bool {
        if let Some(pattern_expr) = self.parse_pattern_to_expression(pattern) {
            self.expressions_are_equivalent(expr, &pattern_expr)
        } else {
            false
        }
    }

    /// Parse pattern string to Expression - intelligent pattern recognition
    ///
    /// Converts mathematical patterns to actual expressions for comparison
    fn parse_pattern_to_expression(&self, pattern: &str) -> Option<Expression> {
        match pattern {
            "0" => Some(Expression::integer(0)),
            "1" => Some(Expression::integer(1)),
            "-1" => Some(Expression::integer(-1)),
            "π" | "pi" => Some(Expression::constant(crate::core::MathConstant::Pi)),
            "e" => Some(Expression::constant(crate::core::MathConstant::E)),
            _ => None,
        }
    }

    /// Check if two expressions are mathematically equivalent
    ///
    /// Intelligent expression comparison using mathematical equivalence
    fn expressions_are_equivalent(&self, expr1: &Expression, expr2: &Expression) -> bool {
        if expr1 == expr2 {
            return true;
        }

        self.check_mathematical_equivalence(expr1, expr2)
    }

    /// Check mathematical equivalence beyond structural equality
    ///
    /// Intelligence-driven mathematical comparison
    fn check_mathematical_equivalence(&self, expr1: &Expression, expr2: &Expression) -> bool {
        match (expr1, expr2) {
            (Expression::Number(n1), Expression::Number(n2)) => self.numbers_are_equivalent(n1, n2),
            (Expression::Constant(c1), Expression::Constant(c2)) => c1 == c2,
            _ => false,
        }
    }

    /// Check if two numbers are mathematically equivalent
    ///
    /// Handles different number representations
    fn numbers_are_equivalent(&self, n1: &crate::core::Number, n2: &crate::core::Number) -> bool {
        use crate::core::Number;
        match (n1, n2) {
            (Number::Integer(i1), Number::Integer(i2)) => i1 == i2,
            (Number::Rational(r1), Number::Rational(r2)) => r1 == r2,
            (Number::Float(f1), Number::Float(f2)) => (f1 - f2).abs() < 1e-15,
            (Number::Integer(i), Number::Float(f)) => (*i as f64 - f).abs() < 1e-15,
            (Number::Float(f), Number::Integer(i)) => (f - *i as f64).abs() < 1e-15,
            _ => false,
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
}

#[cfg(test)]
mod tests {
    use super::*;

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
                    rule_type: DerivativeRuleType::Simple("cos".to_string()),
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
                numerical_evaluator: Some(NumericalEvaluator::StandardLib(f64::sin)),
            }));

        assert!(props.has_derivative());
        assert!(props.has_antiderivative());
        assert_eq!(props.special_value_count(), 0);

        let rule = props.get_antiderivative_rule();
        assert!(rule.is_some());
        if let Some(r) = rule {
            assert_eq!(r.result_template, "-cos(x) + C");
            assert_eq!(r.constant_handling, ConstantOfIntegration::AddConstant);
        }
    }
}
