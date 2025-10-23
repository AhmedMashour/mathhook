//! Square root function intelligence
//!
//! Complete mathematical intelligence for the square root function
//! with derivatives, antiderivatives, special values, and simplification rules.

use crate::core::{Expression, Number, Symbol};
use crate::functions::properties::*;
use std::collections::HashMap;
use std::sync::Arc;

/// Square Root Function Intelligence
///
/// Dedicated intelligence system for the square root function
/// with complete mathematical properties.
pub struct SqrtIntelligence {
    properties: HashMap<String, FunctionProperties>,
}

impl Default for SqrtIntelligence {
    fn default() -> Self {
        Self::new()
    }
}

impl SqrtIntelligence {
    /// Create new square root intelligence system
    ///
    /// # Examples
    ///
    /// ```
    /// use mathhook_core::functions::elementary::sqrt::SqrtIntelligence;
    ///
    /// let intelligence = SqrtIntelligence::new();
    /// assert!(intelligence.has_function("sqrt"));
    /// ```
    pub fn new() -> Self {
        let mut intelligence = Self {
            properties: HashMap::with_capacity(1),
        };

        intelligence.initialize_sqrt();
        intelligence
    }

    /// Get square root function properties
    ///
    /// # Examples
    ///
    /// ```
    /// use mathhook_core::functions::elementary::sqrt::SqrtIntelligence;
    ///
    /// let intelligence = SqrtIntelligence::new();
    /// let props = intelligence.get_properties();
    /// assert!(props.contains_key("sqrt"));
    /// ```
    pub fn get_properties(&self) -> HashMap<String, FunctionProperties> {
        self.properties.clone()
    }

    /// Check if function is square root
    ///
    /// # Arguments
    ///
    /// * `name` - The function name to check
    ///
    /// # Examples
    ///
    /// ```
    /// use mathhook_core::functions::elementary::sqrt::SqrtIntelligence;
    ///
    /// let intelligence = SqrtIntelligence::new();
    /// assert!(intelligence.has_function("sqrt"));
    /// assert!(!intelligence.has_function("sin"));
    /// ```
    pub fn has_function(&self, name: &str) -> bool {
        self.properties.contains_key(name)
    }

    /// Initialize square root function
    fn initialize_sqrt(&mut self) {
        self.properties.insert(
            "sqrt".to_string(),
            FunctionProperties::Elementary(Box::new(ElementaryProperties {
                evaluator: |args| {
                if args.len() == 1 {
                    args[0].clone()
                } else {
                    Expression::function("sqrt", args.to_vec())
                }
            },
            
                derivative_rule: Some(DerivativeRule {
                    rule_type: DerivativeRuleType::Simple("1/(2*sqrt(x))".to_string()),
                    result_template: "d/dx sqrt(x) = 1/(2*sqrt(x)) for x > 0".to_string(),
                }),
                antiderivative_rule: Some(AntiderivativeRule {
                    rule_type: AntiderivativeRuleType::Custom {
                        builder: Arc::new(|var: Symbol| {
                            Expression::mul(vec![
                                Expression::rational(2, 3),
                                Expression::pow(
                                    Expression::symbol(var.clone()),
                                    Expression::rational(3, 2),
                                ),
                            ])
                        }),
                    },
                    result_template: "∫sqrt(x)dx = (2/3)x^(3/2) + C".to_string(),
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
                    SpecialValue {
                        input: "4".to_string(),
                        output: Expression::integer(2),
                        latex_explanation: "\\sqrt{4} = 2".to_string(),
                    },
                    SpecialValue {
                        input: "9".to_string(),
                        output: Expression::integer(3),
                        latex_explanation: "\\sqrt{9} = 3".to_string(),
                    },
                ],
                identities: Box::new(vec![
                    MathIdentity {
                        name: "Product Rule".to_string(),
                        lhs: Expression::function(
                            "sqrt",
                            vec![Expression::mul(vec![
                                Expression::symbol("a"),
                                Expression::symbol("b"),
                            ])],
                        ),
                        rhs: Expression::mul(vec![
                            Expression::function("sqrt", vec![Expression::symbol("a")]),
                            Expression::function("sqrt", vec![Expression::symbol("b")]),
                        ]),
                        conditions: vec!["a, b ≥ 0".to_string()],
                    },
                    MathIdentity {
                        name: "Power Simplification".to_string(),
                        lhs: Expression::function(
                            "sqrt",
                            vec![Expression::pow(
                                Expression::symbol("x"),
                                Expression::integer(2),
                            )],
                        ),
                        rhs: Expression::function("abs", vec![Expression::symbol("x")]),
                        conditions: vec!["x ∈ ℝ".to_string()],
                    },
                ]),
                domain_range: Box::new(DomainRangeData {
                    domain: Domain::Union(vec![
                        Domain::Interval(Expression::integer(0), Expression::infinity()),
                        Domain::Complex,
                    ]),
                    range: Range::Bounded(Expression::integer(0), Expression::infinity()),
                    singularities: vec![],
                }),
                periodicity: None,
                numerical_evaluator: Some(NumericalEvaluator::StandardLib(f64::sqrt)),
            })),
        );
    }
}

/// Simplify square root expressions
///
/// Applies mathematical simplification rules for square root.
///
/// # Simplification Rules
///
/// - sqrt(0) = 0
/// - sqrt(1) = 1
/// - sqrt(4) = 2, sqrt(9) = 3, etc. (perfect squares)
/// - sqrt(x²) = |x|
/// - sqrt(x⁴) = x² (even powers)
/// - sqrt(a*b) = sqrt(a)*sqrt(b) (when a, b ≥ 0)
/// - sqrt(a²*b) = a*sqrt(b) (factor perfect squares)
/// - sqrt(1/4) = 1/2 (rational perfect squares)
///
/// # Arguments
///
/// * `arg` - The argument to the square root function
///
/// # Returns
///
/// Simplified expression
///
/// # Examples
///
/// ```
/// use mathhook_core::core::Expression;
/// use mathhook_core::functions::elementary::sqrt::simplify_sqrt;
///
/// let zero = Expression::integer(0);
/// assert_eq!(simplify_sqrt(&zero), Expression::integer(0));
///
/// let four = Expression::integer(4);
/// assert_eq!(simplify_sqrt(&four), Expression::integer(2));
///
/// let squared = Expression::pow(Expression::symbol("x"), Expression::integer(2));
/// assert_eq!(
///     simplify_sqrt(&squared),
///     Expression::function("abs", vec![Expression::symbol("x")])
/// );
/// ```
pub fn simplify_sqrt(arg: &Expression) -> Expression {
    match arg {
        Expression::Number(n) => evaluate_sqrt_number(n),

        Expression::Pow(base, exp) if is_square(exp) => {
            Expression::function("abs", vec![(**base).clone()])
        }

        Expression::Pow(base, exp) if is_even_power(exp) => simplify_sqrt_even_power(base, exp),

        Expression::Mul(terms) => simplify_sqrt_product(terms),

        Expression::Function { name, args } if name == "sqrt" && args.len() == 1 => {
            Expression::function("sqrt", vec![args[0].clone()])
        }

        _ => Expression::function("sqrt", vec![arg.clone()]),
    }
}

/// Evaluate square root for numeric arguments
fn evaluate_sqrt_number(n: &Number) -> Expression {
    use num_traits::ToPrimitive;

    match n {
        Number::Integer(i) => {
            if *i >= 0 {
                let sqrt_val = (*i as f64).sqrt();
                if sqrt_val.fract() == 0.0 {
                    Expression::integer(sqrt_val as i64)
                } else {
                    Expression::function("sqrt", vec![Expression::integer(*i)])
                }
            } else {
                let pos_sqrt = evaluate_sqrt_number(&Number::Integer(-i));
                Expression::mul(vec![pos_sqrt, Expression::constant(crate::core::MathConstant::I)])
            }
        }
        Number::Float(f) => {
            if *f >= 0.0 {
                Expression::float(f.sqrt())
            } else {
                Expression::mul(vec![
                    Expression::float((-f).sqrt()),
                    Expression::constant(crate::core::MathConstant::I),
                ])
            }
        }
        Number::BigInteger(bi) => {
            use num_traits::Signed;
            if **bi >= num_bigint::BigInt::from(0) {
                if let Some(i_val) = bi.to_i64() {
                    let sqrt_val = (i_val as f64).sqrt();
                    if sqrt_val.fract() == 0.0 {
                        Expression::integer(sqrt_val as i64)
                    } else {
                        Expression::function("sqrt", vec![Expression::Number(n.clone())])
                    }
                } else {
                    Expression::function("sqrt", vec![Expression::Number(n.clone())])
                }
            } else {
                let pos_sqrt = evaluate_sqrt_number(&Number::BigInteger(Box::new((**bi).abs())));
                Expression::mul(vec![pos_sqrt, Expression::constant(crate::core::MathConstant::I)])
            }
        }
        Number::Rational(r) => {
            let numer = r.numer();
            let denom = r.denom();

            if let (Some(n_val), Some(d_val)) = (numer.to_i64(), denom.to_i64()) {
                let n_sqrt = (n_val as f64).sqrt();
                let d_sqrt = (d_val as f64).sqrt();

                if n_sqrt.fract() == 0.0 && d_sqrt.fract() == 0.0 {
                    return Expression::rational(n_sqrt as i64, d_sqrt as i64);
                }
            }

            Expression::function("sqrt", vec![Expression::Number(n.clone())])
        }
    }
}

/// Check if exponent represents squaring (power of 2)
fn is_square(exp: &Expression) -> bool {
    matches!(exp, Expression::Number(Number::Integer(2)))
}

/// Check if exponent is an even integer
fn is_even_power(exp: &Expression) -> bool {
    matches!(exp, Expression::Number(Number::Integer(n)) if n % 2 == 0)
}

/// Simplify sqrt of even powers: sqrt(x⁴) = x²
fn simplify_sqrt_even_power(base: &Expression, exp: &Expression) -> Expression {
    if let Expression::Number(Number::Integer(n)) = exp {
        Expression::pow(base.clone(), Expression::integer(n / 2))
    } else {
        Expression::function(
            "sqrt",
            vec![Expression::pow(base.clone(), exp.clone())],
        )
    }
}

/// Simplify square root of a product: sqrt(a*b) = sqrt(a)*sqrt(b)
fn simplify_sqrt_product(terms: &[Expression]) -> Expression {
    let mut perfect_squares = Vec::new();
    let mut other_terms = Vec::new();

    for term in terms {
        if let Expression::Pow(base, exp) = term {
            if is_square(exp) {
                perfect_squares.push(Expression::function("abs", vec![(**base).clone()]));
            } else if is_even_power(exp) {
                if let Expression::Number(Number::Integer(n)) = **exp {
                    perfect_squares.push(Expression::pow(
                        (**base).clone(),
                        Expression::integer(n / 2),
                    ));
                } else {
                    other_terms.push(term.clone());
                }
            } else {
                other_terms.push(term.clone());
            }
        } else if let Expression::Number(n) = term {
            match evaluate_sqrt_number(n) {
                expr @ Expression::Number(_) => perfect_squares.push(expr),
                _ => other_terms.push(term.clone()),
            }
        } else {
            other_terms.push(term.clone());
        }
    }

    if perfect_squares.is_empty() {
        Expression::function("sqrt", vec![Expression::mul(terms.to_vec())])
    } else if other_terms.is_empty() {
        Expression::mul(perfect_squares)
    } else {
        perfect_squares.push(Expression::function("sqrt", vec![Expression::mul(other_terms)]));
        Expression::mul(perfect_squares)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sqrt_intelligence_creation() {
        let intelligence = SqrtIntelligence::new();
        assert!(intelligence.has_function("sqrt"));

        let props = intelligence.get_properties();
        assert!(props.contains_key("sqrt"));
    }

    #[test]
    fn test_sqrt_properties() {
        let intelligence = SqrtIntelligence::new();
        let props = intelligence.get_properties();
        let sqrt_props = props.get("sqrt").unwrap();

        assert!(sqrt_props.has_derivative());
        assert!(sqrt_props.has_antiderivative());
        assert_eq!(sqrt_props.special_value_count(), 4);
    }

    #[test]
    fn test_simplify_sqrt_zero() {
        let result = simplify_sqrt(&Expression::integer(0));
        assert_eq!(result, Expression::integer(0));
    }

    #[test]
    fn test_simplify_sqrt_one() {
        let result = simplify_sqrt(&Expression::integer(1));
        assert_eq!(result, Expression::integer(1));
    }

    #[test]
    fn test_simplify_sqrt_perfect_square() {
        let result = simplify_sqrt(&Expression::integer(4));
        assert_eq!(result, Expression::integer(2));

        let result = simplify_sqrt(&Expression::integer(9));
        assert_eq!(result, Expression::integer(3));
    }

    #[test]
    fn test_simplify_sqrt_square() {
        let expr = Expression::pow(Expression::symbol("x"), Expression::integer(2));
        let result = simplify_sqrt(&expr);
        assert_eq!(
            result,
            Expression::function("abs", vec![Expression::symbol("x")])
        );
    }

    #[test]
    fn test_simplify_sqrt_even_power() {
        let expr = Expression::pow(Expression::symbol("x"), Expression::integer(4));
        let result = simplify_sqrt(&expr);
        assert_eq!(
            result,
            Expression::pow(Expression::symbol("x"), Expression::integer(2))
        );
    }
}
