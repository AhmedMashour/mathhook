use super::{FormattingContext, FormattingError};
use crate::core::expression::smart_display::SmartDisplayFormatter;
use crate::core::expression::RelationType;
use crate::core::{Expression, Number};

const MAX_RECURSION_DEPTH: usize = 1000;
const MAX_TERMS_PER_OPERATION: usize = 10000;

/// Simple formatting context
#[derive(Debug, Default, Clone)]
pub struct SimpleContext {
    /// Whether to use parentheses around negative numbers
    pub parenthesize_negatives: bool,
    /// Whether to use implicit multiplication (2x vs 2*x)
    pub implicit_multiplication: bool,
    /// Maximum precision for floating point numbers
    pub float_precision: Option<usize>,
    /// Whether to use Unicode symbols (× instead of *)
    pub use_unicode: bool,
}

impl FormattingContext for SimpleContext {}

/// Format the expression to Simple
pub trait SimpleFormatter {
    /// Format an Expression as simple mathematical notation
    ///
    /// Converts mathematical expressions into clean, readable text format
    /// without LaTeX commands or complex markup. The output can be customized
    /// using the provided context.
    ///
    /// # Arguments
    /// * `context` - Formatting configuration controlling output style
    ///
    /// # Context Options
    /// * `float_precision` - Number of decimal places for floating point numbers
    /// * `use_unicode` - Whether to use Unicode symbols (× instead of *)
    /// * `parenthesize_negatives` - Whether to wrap negative numbers in parentheses
    /// * `implicit_multiplication` - Whether to use implicit multiplication (2x vs 2*x)
    ///
    /// # Examples
    /// ```
    /// use mathhook_core::core::Expression;
    /// use mathhook_core::formatter::simple::{SimpleFormatter, SimpleContext};
    ///
    /// let expr = Expression::from("2 * x + 1");
    /// let context = SimpleContext::default();
    /// let result = expr.to_simple(&context);
    /// assert_eq!(result, "2 * x + 1");
    ///
    /// // With Unicode symbols
    /// let context = SimpleContext { use_unicode: true, ..Default::default() };
    /// let result = expr.to_simple(&context);
    /// assert_eq!(result, "2 × x + 1");
    /// ```
    ///
    /// # Error Handling
    /// Returns error messages for expressions that exceed safety limits:
    /// - Maximum recursion depth (1000 levels)
    /// - Maximum terms per operation (10000 terms)
    fn to_simple(&self, context: &SimpleContext) -> Result<String, FormattingError> {
        self.to_simple_with_depth(context, 0)
    }

    /// Format with explicit recursion depth tracking
    ///
    /// Internal method that provides stack overflow protection by tracking
    /// recursion depth. This method returns a Result to allow proper error
    /// propagation during recursive formatting.
    ///
    /// # Arguments
    /// * `context` - Formatting configuration
    /// * `depth` - Current recursion depth (starts at 0)
    ///
    /// # Returns
    /// * `Ok(String)` - Successfully formatted expression
    /// * `Err(String)` - Error message if limits exceeded
    ///
    /// # Safety Limits
    /// * Maximum recursion depth: 1000 levels
    /// * Maximum terms per operation: 10000 terms/factors/arguments
    ///
    /// # Examples
    /// ```
    /// use mathhook_core::core::Expression;
    /// use mathhook_core::formatter::simple::{SimpleFormatter, SimpleContext};
    ///
    /// let expr = Expression::from("x + y");
    /// let context = SimpleContext::default();
    /// let result = expr.to_simple_with_depth(&context, 0);
    /// assert!(result.is_ok());
    /// assert_eq!(result.unwrap(), "x + y");
    /// ```
    fn to_simple_with_depth(
        &self,
        context: &SimpleContext,
        depth: usize,
    ) -> Result<String, FormattingError>;
}

impl SimpleFormatter for Expression {
    fn to_simple_with_depth(
        &self,
        context: &SimpleContext,
        depth: usize,
    ) -> Result<String, FormattingError> {
        if depth > MAX_RECURSION_DEPTH {
            return Err(FormattingError::RecursionLimitExceeded {
                depth,
                limit: MAX_RECURSION_DEPTH,
            });
        }
        match self {
            Expression::Number(Number::Integer(n)) => Ok(n.to_string()),
            Expression::Number(Number::BigInteger(n)) => Ok(n.to_string()),
            Expression::Number(Number::Rational(r)) => {
                if r.denom() == &num_bigint::BigInt::from(1) {
                    Ok(r.numer().to_string())
                } else {
                    Ok(format!("{}/{}", r.numer(), r.denom()))
                }
            }
            Expression::Number(Number::Float(f)) => {
                if let Some(precision) = context.float_precision {
                    Ok(format!("{:.1$}", f, precision))
                } else {
                    Ok(f.to_string())
                }
            }
            Expression::Symbol(s) => Ok(s.name().to_string()),
            Expression::Add(terms) => {
                if terms.len() > MAX_TERMS_PER_OPERATION {
                    return Err(FormattingError::TooManyTerms {
                        count: terms.len(),
                        limit: MAX_TERMS_PER_OPERATION,
                    });
                }

                let mut term_strs = Vec::with_capacity(terms.len());
                for (i, term) in terms.iter().enumerate() {
                    if i == 0 {
                        let term_result = term.to_simple_with_depth(context, depth + 1)?;
                        term_strs.push(term_result);
                    } else {
                        // Smart subtraction detection for Simple format
                        if SmartDisplayFormatter::is_negated_expression(term) {
                            if let Some(positive_part) =
                                SmartDisplayFormatter::extract_negated_expression(term)
                            {
                                let positive_result =
                                    positive_part.to_simple_with_depth(context, depth + 1)?;
                                term_strs.push(format!(" - {}", positive_result));
                            } else {
                                let term_result = term.to_simple_with_depth(context, depth + 1)?;
                                term_strs.push(format!(" + {}", term_result));
                            }
                        } else {
                            let term_result = term.to_simple_with_depth(context, depth + 1)?;
                            term_strs.push(format!(" + {}", term_result));
                        }
                    }
                }
                Ok(term_strs.join(""))
            }
            Expression::Mul(factors) => {
                if factors.len() > MAX_TERMS_PER_OPERATION {
                    return Err(FormattingError::TooManyTerms {
                        count: factors.len(),
                        limit: MAX_TERMS_PER_OPERATION,
                    });
                }

                // Smart division detection for Simple format: x * y^(-1) → x / y
                if let Some((dividend, divisor)) =
                    SmartDisplayFormatter::extract_division_parts(factors)
                {
                    let dividend_str = dividend.to_simple_with_depth(context, depth + 1)?;
                    let divisor_str = divisor.to_simple_with_depth(context, depth + 1)?;
                    return Ok(format!("{} / {}", dividend_str, divisor_str));
                }

                let mut factor_strs = Vec::with_capacity(factors.len());
                for f in factors.iter() {
                    let factor_result = f.to_simple_with_depth(context, depth + 1)?;
                    let needs_parens = matches!(f, Expression::Add(_));
                    if needs_parens {
                        factor_strs.push(format!("({})", factor_result));
                    } else {
                        factor_strs.push(factor_result);
                    }
                }
                let separator = if context.use_unicode { " × " } else { " * " };
                Ok(factor_strs.join(separator))
            }
            Expression::Pow(base, exp) => {
                let base_simple = base.to_simple_with_depth(context, depth + 1)?;
                let exp_simple = exp.to_simple_with_depth(context, depth + 1)?;
                // Add parentheses around negative or complex exponents for clarity
                if exp_simple.starts_with('-') || exp_simple.contains(' ') {
                    Ok(format!("{}^({})", base_simple, exp_simple))
                } else {
                    Ok(format!("{}^{}", base_simple, exp_simple))
                }
            }
            Expression::Function { name, args } => {
                if args.is_empty() {
                    Ok(name.clone())
                } else {
                    if args.len() > MAX_TERMS_PER_OPERATION {
                        return Err(FormattingError::TooManyTerms {
                            count: args.len(),
                            limit: MAX_TERMS_PER_OPERATION,
                        });
                    }

                    let mut arg_strs = Vec::with_capacity(args.len());
                    for arg in args.iter() {
                        arg_strs.push(arg.to_simple_with_depth(context, depth + 1)?);
                    }
                    Ok(format!("{}({})", name, arg_strs.join(", ")))
                }
            }
            Expression::Interval(interval_data) => {
                let start_bracket = if interval_data.start_inclusive {
                    "["
                } else {
                    "("
                };
                let end_bracket = if interval_data.end_inclusive {
                    "]"
                } else {
                    ")"
                };
                let start_simple = interval_data
                    .start
                    .to_simple_with_depth(context, depth + 1)?;
                let end_simple = interval_data.end.to_simple_with_depth(context, depth + 1)?;
                Ok(format!(
                    "{}{}, {}{}",
                    start_bracket, start_simple, end_simple, end_bracket
                ))
            }
            Expression::Relation(relation_data) => {
                let left_simple = relation_data
                    .left
                    .to_simple_with_depth(context, depth + 1)?;
                let right_simple = relation_data
                    .right
                    .to_simple_with_depth(context, depth + 1)?;
                let operator = match relation_data.relation_type {
                    RelationType::Equal => "=",
                    RelationType::NotEqual => {
                        if context.use_unicode {
                            "≠"
                        } else {
                            "!="
                        }
                    }
                    RelationType::Less => "<",
                    RelationType::LessEqual => {
                        if context.use_unicode {
                            "≤"
                        } else {
                            "<="
                        }
                    }
                    RelationType::Greater => ">",
                    RelationType::GreaterEqual => {
                        if context.use_unicode {
                            "≥"
                        } else {
                            ">="
                        }
                    }
                    RelationType::Approximate => {
                        if context.use_unicode {
                            "≈"
                        } else {
                            "~="
                        }
                    }
                    RelationType::Similar => {
                        if context.use_unicode {
                            "∼"
                        } else {
                            "~"
                        }
                    }
                    RelationType::Proportional => {
                        if context.use_unicode {
                            "∝"
                        } else {
                            "prop"
                        }
                    }
                };
                Ok(format!("{} {} {}", left_simple, operator, right_simple))
            }
            Expression::Piecewise(piecewise_data) => {
                let mut result = String::from("{");

                for (i, (condition, value)) in piecewise_data.pieces.iter().enumerate() {
                    if i > 0 {
                        result.push_str(", ");
                    }
                    let condition_simple = condition.to_simple_with_depth(context, depth + 1)?;
                    let value_simple = value.to_simple_with_depth(context, depth + 1)?;
                    result.push_str(&format!("{} if {}", value_simple, condition_simple));
                }

                if let Some(default_value) = &piecewise_data.default {
                    let default_simple = default_value.to_simple_with_depth(context, depth + 1)?;
                    result.push_str(&format!(", {} otherwise", default_simple));
                }

                result.push('}');
                Ok(result)
            }
            Expression::MethodCall(method_data) => {
                let object_str = method_data
                    .object
                    .to_simple_with_depth(context, depth + 1)?;
                if method_data.args.is_empty() {
                    Ok(format!("{}.{}()", object_str, method_data.method_name))
                } else {
                    let args_str = method_data
                        .args
                        .iter()
                        .map(|arg| arg.to_simple_with_depth(context, depth + 1))
                        .collect::<Result<Vec<_>, _>>()?
                        .join(", ");
                    Ok(format!(
                        "{}.{}({})",
                        object_str, method_data.method_name, args_str
                    ))
                }
            }
            _ => Ok("unknown".to_string()),
        }
    }
}
