//! Absolute value function intelligence
//!
//! Complete mathematical intelligence for the absolute value function
//! with derivatives, antiderivatives, special values, and simplification rules.

use crate::core::{Expression, Number, Symbol};
use crate::functions::properties::*;
use std::collections::HashMap;
use std::sync::Arc;

/// Absolute Value Function Intelligence
///
/// Dedicated intelligence system for the absolute value function
/// with complete mathematical properties.
pub struct AbsoluteValueIntelligence {
    properties: HashMap<String, FunctionProperties>,
}

impl Default for AbsoluteValueIntelligence {
    fn default() -> Self {
        Self::new()
    }
}

impl AbsoluteValueIntelligence {
    /// Create new absolute value intelligence system
    ///
    /// # Examples
    ///
    /// ```
    /// use mathhook_core::functions::elementary::abs::AbsoluteValueIntelligence;
    ///
    /// let intelligence = AbsoluteValueIntelligence::new();
    /// assert!(intelligence.has_function("abs"));
    /// ```
    pub fn new() -> Self {
        let mut intelligence = Self {
            properties: HashMap::with_capacity(1),
        };

        intelligence.initialize_abs();
        intelligence
    }

    /// Get absolute value function properties
    ///
    /// # Examples
    ///
    /// ```
    /// use mathhook_core::functions::elementary::abs::AbsoluteValueIntelligence;
    ///
    /// let intelligence = AbsoluteValueIntelligence::new();
    /// let props = intelligence.get_properties();
    /// assert!(props.contains_key("abs"));
    /// ```
    pub fn get_properties(&self) -> HashMap<String, FunctionProperties> {
        self.properties.clone()
    }

    /// Check if function is absolute value
    ///
    /// # Arguments
    ///
    /// * `name` - The function name to check
    ///
    /// # Examples
    ///
    /// ```
    /// use mathhook_core::functions::elementary::abs::AbsoluteValueIntelligence;
    ///
    /// let intelligence = AbsoluteValueIntelligence::new();
    /// assert!(intelligence.has_function("abs"));
    /// assert!(!intelligence.has_function("sin"));
    /// ```
    pub fn has_function(&self, name: &str) -> bool {
        self.properties.contains_key(name)
    }

    /// Initialize absolute value function
    fn initialize_abs(&mut self) {
        self.properties.insert(
            "abs".to_owned(),
            FunctionProperties::Elementary(Box::new(ElementaryProperties {
                derivative_rule: Some(DerivativeRule {
                    rule_type: DerivativeRuleType::Custom {
                        builder: Arc::new(|arg: &Expression| {
                            let abs_arg = Expression::function("abs", vec![arg.clone()]);
                            Expression::mul(vec![
                                arg.clone(),
                                Expression::pow(abs_arg, Expression::integer(-1)),
                            ])
                        }),
                    },
                    result_template: "x/|x| for x ≠ 0".to_owned(),
                }),
                antiderivative_rule: Some(AntiderivativeRule {
                    rule_type: AntiderivativeRuleType::Custom {
                        builder: Arc::new(|var: Symbol| {
                            Expression::mul(vec![
                                Expression::rational(1, 2),
                                Expression::mul(vec![
                                    Expression::symbol(var.clone()),
                                    Expression::function("abs", vec![Expression::symbol(var)]),
                                ]),
                            ])
                        }),
                    },
                    result_template: "∫|x|dx = x|x|/2 + C".to_owned(),
                    constant_handling: ConstantOfIntegration::AddConstant,
                }),
                special_values: vec![
                    SpecialValue {
                        input: "0".to_owned(),
                        output: Expression::integer(0),
                        latex_explanation: "|0| = 0".to_owned(),
                    },
                    SpecialValue {
                        input: "1".to_owned(),
                        output: Expression::integer(1),
                        latex_explanation: "|1| = 1".to_owned(),
                    },
                    SpecialValue {
                        input: "-1".to_owned(),
                        output: Expression::integer(1),
                        latex_explanation: "|-1| = 1".to_owned(),
                    },
                ],
                identities: Box::new(vec![
                    MathIdentity {
                        name: "Even Function".to_owned(),
                        lhs: Expression::function(
                            "abs",
                            vec![Expression::mul(vec![
                                Expression::integer(-1),
                                Expression::symbol("x"),
                            ])],
                        ),
                        rhs: Expression::function("abs", vec![Expression::symbol("x")]),
                        conditions: vec!["x ∈ ℝ".to_owned()],
                    },
                    MathIdentity {
                        name: "Product Rule".to_owned(),
                        lhs: Expression::function(
                            "abs",
                            vec![Expression::mul(vec![
                                Expression::symbol("a"),
                                Expression::symbol("b"),
                            ])],
                        ),
                        rhs: Expression::mul(vec![
                            Expression::function("abs", vec![Expression::symbol("a")]),
                            Expression::function("abs", vec![Expression::symbol("b")]),
                        ]),
                        conditions: vec!["a, b ∈ ℂ".to_owned()],
                    },
                    MathIdentity {
                        name: "Quotient Rule".to_owned(),
                        lhs: Expression::function(
                            "abs",
                            vec![Expression::mul(vec![
                                Expression::symbol("a"),
                                Expression::pow(Expression::symbol("b"), Expression::integer(-1)),
                            ])],
                        ),
                        rhs: Expression::mul(vec![
                            Expression::function("abs", vec![Expression::symbol("a")]),
                            Expression::pow(
                                Expression::function("abs", vec![Expression::symbol("b")]),
                                Expression::integer(-1),
                            ),
                        ]),
                        conditions: vec!["a, b ∈ ℂ, b ≠ 0".to_owned()],
                    },
                ]),
                domain_range: Box::new(DomainRangeData {
                    domain: Domain::Real,
                    range: Range::Bounded(Expression::integer(0), Expression::infinity()),
                    singularities: vec![],
                }),
                periodicity: None,
                wolfram_name: None,
            })),
        );
    }
}

/// Simplify absolute value expressions
///
/// Applies mathematical simplification rules for absolute value.
///
/// # Simplification Rules
///
/// - |0| = 0
/// - |-x| = |x|
/// - |x²| = x² (squares are always non-negative)
/// - |a*b| = |a|*|b|
/// - |a/b| = |a|/|b|
/// - ||x|| = |x|
///
/// # Arguments
///
/// * `arg` - The argument to the absolute value function
///
/// # Returns
///
/// Simplified expression
///
/// # Examples
///
/// ```
/// use mathhook_core::core::Expression;
/// use mathhook_core::functions::elementary::abs::simplify_abs;
///
/// let zero = Expression::integer(0);
/// assert_eq!(simplify_abs(&zero), Expression::integer(0));
///
/// let neg_five = Expression::integer(-5);
/// assert_eq!(simplify_abs(&neg_five), Expression::integer(5));
///
/// let squared = Expression::pow(Expression::symbol("x"), Expression::integer(2));
/// assert_eq!(simplify_abs(&squared), squared);
/// ```
pub fn simplify_abs(arg: &Expression) -> Expression {
    match arg {
        Expression::Number(n) => evaluate_abs_number(n),

        Expression::Mul(terms) if is_negation(terms) => {
            let inner = extract_negation_argument(terms);
            Expression::function("abs", vec![inner])
        }

        Expression::Pow(base, exp) if is_square(exp) => {
            Expression::pow((**base).clone(), (**exp).clone())
        }

        Expression::Mul(terms) => simplify_abs_product(terms),

        Expression::Function { name, args } if name == "abs" && args.len() == 1 => {
            Expression::function("abs", vec![args[0].clone()])
        }

        _ => Expression::function("abs", vec![arg.clone()]),
    }
}

/// Evaluate absolute value for numeric arguments
fn evaluate_abs_number(n: &Number) -> Expression {
    use num_rational::BigRational;
    use num_traits::Signed;

    match n {
        Number::Integer(i) => Expression::integer(i.abs()),
        Number::Float(f) => Expression::float(f.abs()),
        Number::BigInteger(bi) => Expression::big_integer(bi.abs()),
        Number::Rational(r) => Expression::Number(Number::rational(BigRational::new(
            r.numer().abs(),
            r.denom().clone(),
        ))),
    }
}

/// Check if expression is a negation (-x)
fn is_negation(terms: &[Expression]) -> bool {
    terms.len() == 2 && matches!(terms[0], Expression::Number(Number::Integer(-1)))
}

/// Extract argument from negation expression
fn extract_negation_argument(terms: &[Expression]) -> Expression {
    terms[1].clone()
}

/// Check if exponent represents squaring (power of 2)
fn is_square(exp: &Expression) -> bool {
    matches!(exp, Expression::Number(Number::Integer(2)))
}

/// Simplify absolute value of a product: |a*b| = |a|*|b|
fn simplify_abs_product(terms: &[Expression]) -> Expression {
    Expression::mul(
        terms
            .iter()
            .map(|term| Expression::function("abs", vec![term.clone()]))
            .collect(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_abs_intelligence_creation() {
        let intelligence = AbsoluteValueIntelligence::new();
        assert!(intelligence.has_function("abs"));

        let props = intelligence.get_properties();
        assert!(props.contains_key("abs"));
    }

    #[test]
    fn test_abs_properties() {
        let intelligence = AbsoluteValueIntelligence::new();
        let props = intelligence.get_properties();
        let abs_props = props.get("abs").unwrap();

        assert!(abs_props.has_derivative());
        assert!(abs_props.has_antiderivative());
        assert_eq!(abs_props.special_value_count(), 3);
    }

    #[test]
    fn test_simplify_abs_zero() {
        let result = simplify_abs(&Expression::integer(0));
        assert_eq!(result, Expression::integer(0));
    }

    #[test]
    fn test_simplify_abs_positive() {
        let result = simplify_abs(&Expression::integer(5));
        assert_eq!(result, Expression::integer(5));
    }

    #[test]
    fn test_simplify_abs_negative() {
        let result = simplify_abs(&Expression::integer(-5));
        assert_eq!(result, Expression::integer(5));
    }

    #[test]
    fn test_simplify_abs_negation() {
        let expr = Expression::mul(vec![Expression::integer(-1), Expression::symbol("x")]);
        let result = simplify_abs(&expr);
        assert_eq!(
            result,
            Expression::function("abs", vec![Expression::symbol("x")])
        );
    }

    #[test]
    fn test_simplify_abs_square() {
        let expr = Expression::pow(Expression::symbol("x"), Expression::integer(2));
        let result = simplify_abs(&expr);
        assert_eq!(result, expr);
    }
}
