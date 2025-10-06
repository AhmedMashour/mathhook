use super::{FormattingContext, FormattingError};
use crate::core::expression::CalculusData;
use crate::core::expression::{LimitDirection, RelationType};
use crate::core::MathConstant;
use crate::core::{Expression, Number};

const MAX_RECURSION_DEPTH: usize = 1000;
const MAX_TERMS_PER_OPERATION: usize = 10000;

/// LaTeX formatting context
#[derive(Debug, Default, Clone)]
pub struct LaTeXContext {
    pub needs_parentheses: bool,
}

impl FormattingContext for LaTeXContext {}

/// Formt the expression to LaTeX
pub trait LaTeXFormatter {
    /// Format an Expression as LaTeX mathematical notation
    ///
    /// Converts mathematical expressions into LaTeX format suitable for
    /// rendering in mathematical documents and publications.
    ///
    /// # Arguments
    /// * `context` - LaTeX formatting configuration
    ///
    /// # Context Options
    /// * `needs_parentheses` - Whether to wrap the entire expression in parentheses
    ///
    /// # Examples
    /// ```
    /// use mathhook_core::core::Expression;
    /// use mathhook_core::formatter::latex::{LaTeXFormatter, LaTeXContext};
    ///
    /// let expr = Expression::from("x^2 + 1");
    /// let context = LaTeXContext::default();
    /// let result = expr.to_latex(&context);
    /// assert_eq!(result, "x^{2} + 1");
    /// ```
    ///
    /// # Error Handling
    /// Returns error messages for expressions that exceed safety limits:
    /// - Maximum recursion depth (1000 levels)
    /// - Maximum terms per operation (10000 terms)
    fn to_latex<C>(&self, context: C) -> Result<String, FormattingError>
    where
        C: Into<Option<LaTeXContext>>,
    {
        let context = context.into().unwrap_or_default();
        self.to_latex_with_depth(&context, 0)
    }

    /// Format with explicit recursion depth tracking
    ///
    /// Internal method that provides stack overflow protection by tracking
    /// recursion depth. This method returns a Result to allow proper error
    /// propagation during recursive formatting.
    ///
    /// # Arguments
    /// * `context` - LaTeX formatting configuration
    /// * `depth` - Current recursion depth (starts at 0)
    ///
    /// # Returns
    /// * `Ok(String)` - Successfully formatted LaTeX expression
    /// * `Err(String)` - Error message if limits exceeded
    ///
    /// # Safety Limits
    /// * Maximum recursion depth: 1000 levels
    /// * Maximum terms per operation: 10000 terms/factors/arguments
    fn to_latex_with_depth(
        &self,
        context: &LaTeXContext,
        depth: usize,
    ) -> Result<String, FormattingError>;

    /// Convert function to LaTeX with context and depth tracking
    fn function_to_latex_with_depth(
        &self,
        name: &str,
        args: &[Expression],
        context: &LaTeXContext,
        depth: usize,
    ) -> Result<String, FormattingError>;

    /// Convert function to LaTeX (convenience method)
    fn function_to_latex(
        &self,
        name: &str,
        args: &[Expression],
        context: &LaTeXContext,
    ) -> Result<String, FormattingError> {
        match self.function_to_latex_with_depth(name, args, context, 0) {
            Ok(result) => Ok(result),
            Err(error) => Err(FormattingError::InvalidMathConstruct {
                reason: error.to_string(),
            }),
        }
    }
}

impl LaTeXFormatter for Expression {
    fn function_to_latex_with_depth(
        &self,
        name: &str,
        args: &[Expression],
        context: &LaTeXContext,
        depth: usize,
    ) -> Result<String, FormattingError> {
        if depth > MAX_RECURSION_DEPTH {
            return Err(FormattingError::RecursionLimitExceeded {
                depth,
                limit: MAX_RECURSION_DEPTH,
            });
        }

        if args.len() > MAX_TERMS_PER_OPERATION {
            return Err(FormattingError::TooManyTerms {
                count: args.len(),
                limit: MAX_TERMS_PER_OPERATION,
            });
        }
        Ok(match name {
            // Trigonometric functions
            "sin" | "cos" | "tan" | "sec" | "csc" | "cot" => {
                if args.len() == 1 {
                    format!(
                        "\\{}({})",
                        name,
                        args[0].to_latex_with_depth(context, depth + 1)?
                    )
                } else {
                    let mut arg_strs = Vec::with_capacity(args.len());
                    for arg in args.iter() {
                        arg_strs.push(arg.to_latex_with_depth(context, depth + 1)?);
                    }
                    format!("\\{}({})", name, arg_strs.join(", "))
                }
            }
            // Inverse trigonometric
            "arcsin" | "arccos" | "arctan" => {
                format!(
                    "\\{}({})",
                    name,
                    args[0].to_latex_with_depth(context, depth + 1)?
                )
            }
            // Logarithmic functions
            "ln" => format!("\\ln({})", args[0].to_latex_with_depth(context, depth + 1)?),
            "log" => {
                if args.len() == 1 {
                    format!(
                        "\\log({})",
                        args[0].to_latex_with_depth(context, depth + 1)?
                    )
                } else if args.len() == 2 {
                    format!(
                        "\\log_{{{}}}({})",
                        args[1].to_latex_with_depth(context, depth + 1)?,
                        args[0].to_latex_with_depth(context, depth + 1)?
                    )
                } else {
                    format!("\\log({})", {
                        let mut arg_strs = Vec::with_capacity(args.len());
                        for arg in args.iter() {
                            arg_strs.push(arg.to_latex_with_depth(context, depth + 1)?);
                        }
                        arg_strs.join(", ")
                    })
                }
            }
            "exp" => format!(
                "\\exp({})",
                args[0].to_latex_with_depth(context, depth + 1)?
            ),
            // Special functions
            "sqrt" => {
                if args.len() == 1 {
                    format!(
                        "\\sqrt{{{}}}",
                        args[0].to_latex_with_depth(context, depth + 1)?
                    )
                } else if args.len() == 2 {
                    format!(
                        "\\sqrt[{}]{{{}}}",
                        args[1].to_latex_with_depth(context, depth + 1)?,
                        args[0].to_latex_with_depth(context, depth + 1)?
                    )
                } else {
                    format!(
                        "\\sqrt{{{}}}",
                        args[0].to_latex_with_depth(context, depth + 1)?
                    )
                }
            }
            "factorial" => {
                if args.len() == 1 {
                    format!("{}!", args[0].to_latex_with_depth(context, depth + 1)?)
                } else {
                    format!("\\text{{factorial}}({})", {
                        let mut arg_strs = Vec::with_capacity(args.len());
                        for arg in args.iter() {
                            arg_strs.push(arg.to_latex_with_depth(context, depth + 1)?);
                        }
                        arg_strs.join(", ")
                    })
                }
            }
            // Calculus
            "integrate" => {
                if args.len() == 2 {
                    format!(
                        "\\int {} \\, d{}",
                        args[0].to_latex_with_depth(context, depth + 1)?,
                        args[1].to_latex_with_depth(context, depth + 1)?
                    )
                } else {
                    format!("\\int({})", {
                        let mut arg_strs = Vec::with_capacity(args.len());
                        for arg in args.iter() {
                            arg_strs.push(arg.to_latex_with_depth(context, depth + 1)?);
                        }
                        arg_strs.join(", ")
                    })
                }
            }
            "derivative" => {
                if args.len() == 2 {
                    format!(
                        "\\frac{{d}}{{d{}}} {}",
                        args[1].to_latex_with_depth(context, depth + 1)?,
                        args[0].to_latex_with_depth(context, depth + 1)?
                    )
                } else {
                    format!(
                        "\\frac{{d}}{{dx}}({})",
                        args[0].to_latex_with_depth(context, depth + 1)?
                    )
                }
            }
            "sum" => {
                if args.len() == 4 {
                    format!(
                        "\\sum_{{{}={}}}^{{{}}} {}",
                        args[1].to_latex_with_depth(context, depth + 1)?,
                        args[2].to_latex_with_depth(context, depth + 1)?,
                        args[3].to_latex_with_depth(context, depth + 1)?,
                        args[0].to_latex_with_depth(context, depth + 1)?
                    )
                } else {
                    format!("\\sum({})", {
                        let mut arg_strs = Vec::with_capacity(args.len());
                        for arg in args.iter() {
                            arg_strs.push(arg.to_latex_with_depth(context, depth + 1)?);
                        }
                        arg_strs.join(", ")
                    })
                }
            }
            // Special functions
            "gamma" => format!(
                "\\Gamma({})",
                args[0].to_latex_with_depth(context, depth + 1)?
            ),
            // Default case
            _ => {
                if args.is_empty() {
                    name.to_string()
                } else {
                    format!("\\text{{{}}}({})", name, {
                        let mut arg_strs = Vec::with_capacity(args.len());
                        for arg in args.iter() {
                            arg_strs.push(arg.to_latex_with_depth(context, depth + 1)?);
                        }
                        arg_strs.join(", ")
                    })
                }
            }
        })
    }

    fn to_latex_with_depth(
        &self,
        context: &LaTeXContext,
        depth: usize,
    ) -> Result<String, FormattingError> {
        if depth > MAX_RECURSION_DEPTH {
            return Err(FormattingError::RecursionLimitExceeded {
                depth,
                limit: MAX_RECURSION_DEPTH,
            });
        }

        Ok(match self {
            Expression::Number(Number::Integer(n)) => n.to_string(),
            Expression::Number(Number::BigInteger(n)) => n.to_string(),
            Expression::Number(Number::Rational(r)) => {
                if r.denom() == &num_bigint::BigInt::from(1) {
                    r.numer().to_string()
                } else {
                    format!("\\frac{{{}}}{{{}}}", r.numer(), r.denom())
                }
            }
            Expression::Number(Number::Float(f)) => f.to_string(),
            Expression::Symbol(s) => s.name().to_string(),
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
                        term_strs.push(term.to_latex_with_depth(context, depth + 1)?);
                    } else {
                        term_strs.push(format!(
                            " + {}",
                            term.to_latex_with_depth(context, depth + 1)?
                        ));
                    }
                }
                if context.needs_parentheses {
                    format!("\\left({}\\right)", term_strs.join(""))
                } else {
                    term_strs.join("")
                }
            }
            Expression::Mul(factors) => {
                if factors.len() > MAX_TERMS_PER_OPERATION {
                    return Err(FormattingError::TooManyTerms {
                        count: factors.len(),
                        limit: MAX_TERMS_PER_OPERATION,
                    });
                }

                let mut factor_strs = Vec::with_capacity(factors.len());
                for f in factors.iter() {
                    let needs_parens = matches!(f, Expression::Add(_));
                    if needs_parens {
                        factor_strs.push(format!(
                            "\\left({}\\right)",
                            f.to_latex_with_depth(context, depth + 1)?
                        ));
                    } else {
                        factor_strs.push(f.to_latex_with_depth(context, depth + 1)?);
                    }
                }
                // Use simpler multiplication format for better roundtrip consistency
                if factors.len() == 2 {
                    // For simple cases like 2π, use implicit multiplication
                    let first = factors[0].to_latex_with_depth(context, depth + 1)?;
                    let second = factors[1].to_latex_with_depth(context, depth + 1)?;

                    // Check if this is number * constant (like 2π)
                    if let (Expression::Number(_), Expression::Constant(_)) =
                        (&factors[0], &factors[1])
                    {
                        format!("{}{}", first, second)
                    } else {
                        format!("{} * {}", first, second)
                    }
                } else {
                    factor_strs.join(" * ")
                }
            }
            Expression::Pow(base, exp) => {
                // Check if this is a square root: x^(1/2) -> \sqrt{x}
                if let Expression::Number(Number::Rational(r)) = exp.as_ref() {
                    if r.numer() == &num_bigint::BigInt::from(1)
                        && r.denom() == &num_bigint::BigInt::from(2)
                    {
                        return Ok(format!(
                            "\\sqrt{{{}}}",
                            base.to_latex_with_depth(context, depth + 1)?
                        ));
                    }
                }

                // Check if this is a function power: sin(x)^2 -> \sin^2(x)
                if let Expression::Function { name, args } = base.as_ref() {
                    if args.len() == 1
                        && matches!(name.as_str(), "sin" | "cos" | "tan" | "ln" | "log")
                    {
                        return Ok(format!(
                            "\\{}^{{{}}}({})",
                            name,
                            exp.to_latex_with_depth(context, depth + 1)?,
                            args[0].to_latex_with_depth(context, depth + 1)?
                        ));
                    }
                }

                let base_str = match base.as_ref() {
                    Expression::Add(_) | Expression::Mul(_) => {
                        format!(
                            "\\left({}\\right)",
                            base.to_latex_with_depth(context, depth + 1)?
                        )
                    }
                    _ => base.to_latex_with_depth(context, depth + 1)?,
                };
                format!(
                    "{}^{{{}}}",
                    base_str,
                    exp.to_latex_with_depth(context, depth + 1)?
                )
            }
            Expression::Function { name, args } => {
                self.function_to_latex_with_depth(name, args, context, depth + 1)?
            }
            // Mathematical constants with consistent formatting
            Expression::Constant(c) => match c {
                MathConstant::Pi => "π".to_string(),
                MathConstant::E => "e".to_string(),
                MathConstant::I => "i".to_string(),
                MathConstant::Infinity => "∞".to_string(),
                MathConstant::NegativeInfinity => "-∞".to_string(),
                MathConstant::Undefined => "\\text{undefined}".to_string(),
                MathConstant::GoldenRatio => "φ".to_string(),
                MathConstant::EulerGamma => "γ".to_string(),
                MathConstant::TribonacciConstant => "α₃".to_string(),
            },
            // New expression types - implement later
            Expression::Complex(complex_data) => format!(
                "{} + {}i",
                complex_data.real.to_latex_with_depth(context, depth + 1)?,
                complex_data.imag.to_latex_with_depth(context, depth + 1)?
            ),
            Expression::Matrix(matrix) => {
                let (rows, cols) = matrix.dimensions();
                let mut row_strs = Vec::with_capacity(rows);

                for i in 0..rows {
                    let mut col_strs = Vec::with_capacity(cols);
                    for j in 0..cols {
                        let elem = matrix.get_element(i, j);
                        col_strs.push(elem.to_latex_with_depth(context, depth + 1)?);
                    }
                    row_strs.push(col_strs.join(" & "));
                }

                format!(
                    "\\begin{{pmatrix}} {} \\end{{pmatrix}}",
                    row_strs.join(" \\\\ ")
                )
            }
            Expression::Relation(relation_data) => {
                let left_latex = relation_data.left.to_latex_with_depth(context, depth + 1)?;
                let right_latex = relation_data
                    .right
                    .to_latex_with_depth(context, depth + 1)?;
                let operator = match relation_data.relation_type {
                    RelationType::Equal => "=",
                    RelationType::NotEqual => "\\neq",
                    RelationType::Less => "<",
                    RelationType::LessEqual => "\\leq",
                    RelationType::Greater => ">",
                    RelationType::GreaterEqual => "\\geq",
                    RelationType::Approximate => "\\approx",
                    RelationType::Similar => "\\sim",
                    RelationType::Proportional => "\\propto",
                };
                format!("{} {} {}", left_latex, operator, right_latex)
            }
            Expression::Piecewise(piecewise_data) => {
                let mut cases = Vec::new();

                for (condition, value) in &piecewise_data.pieces {
                    let condition_latex = condition.to_latex_with_depth(context, depth + 1)?;
                    let value_latex = value.to_latex_with_depth(context, depth + 1)?;
                    cases.push(format!(
                        "{} & \\text{{if }} {}",
                        value_latex, condition_latex
                    ));
                }

                if let Some(default_value) = &piecewise_data.default {
                    let default_latex = default_value.to_latex_with_depth(context, depth + 1)?;
                    cases.push(format!("{} & \\text{{otherwise}}", default_latex));
                }

                format!(
                    "\\begin{{cases}} {} \\end{{cases}}",
                    cases.join(" \\\\\\\\ ")
                )
            }
            Expression::Set(elements) => {
                if elements.is_empty() {
                    "\\{\\}".to_string()
                } else {
                    let mut element_strs = Vec::with_capacity(elements.len());
                    for elem in elements.iter() {
                        element_strs.push(elem.to_latex_with_depth(context, depth + 1)?);
                    }
                    format!("\\{{{}\\}}", element_strs.join(", "))
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
                let start_latex = interval_data
                    .start
                    .to_latex_with_depth(context, depth + 1)?;
                let end_latex = interval_data.end.to_latex_with_depth(context, depth + 1)?;
                format!(
                    "{}{}; {}{}",
                    start_bracket, start_latex, end_latex, end_bracket
                )
            }
            // Calculus expressions with proper LaTeX formatting
            Expression::Calculus(calculus_data) => match calculus_data.as_ref() {
                CalculusData::Derivative {
                    expression,
                    variable,
                    order,
                } => {
                    if *order == 1 {
                        format!(
                            "\\frac{{d}}{{d{}}} {}",
                            variable.name(),
                            expression.to_latex_with_depth(context, depth + 1)?
                        )
                    } else {
                        format!(
                            "\\frac{{d^{}}}{{d{}^{}}} {}",
                            order,
                            variable.name(),
                            order,
                            expression.to_latex_with_depth(context, depth + 1)?
                        )
                    }
                }
                CalculusData::Integral {
                    integrand,
                    variable,
                    bounds,
                } => match bounds {
                    None => format!(
                        "\\int {} d{}",
                        integrand.to_latex_with_depth(context, depth + 1)?,
                        variable.name()
                    ),
                    Some((start, end)) => format!(
                        "\\int_{}^{} {} d{}",
                        start.to_latex_with_depth(context, depth + 1)?,
                        end.to_latex_with_depth(context, depth + 1)?,
                        integrand.to_latex_with_depth(context, depth + 1)?,
                        variable.name()
                    ),
                },
                CalculusData::Limit {
                    expression,
                    variable,
                    direction,
                    ..
                } => {
                    format!(
                        "\\lim_{{{}\\to{}}} {}",
                        variable.name(),
                        match direction {
                            LimitDirection::Left => "0^-",
                            LimitDirection::Right => "0^+",
                            LimitDirection::Both => "0",
                        },
                        expression.to_latex_with_depth(context, depth + 1)?
                    )
                }
                CalculusData::Sum {
                    expression,
                    variable,
                    start,
                    end,
                } => {
                    format!(
                        "\\sum_{{{}={}}}^{} {}",
                        variable.name(),
                        start.to_latex_with_depth(context, depth + 1)?,
                        end.to_latex_with_depth(context, depth + 1)?,
                        expression.to_latex_with_depth(context, depth + 1)?
                    )
                }
                CalculusData::Product {
                    expression,
                    variable,
                    start,
                    end,
                } => {
                    format!(
                        "\\prod_{{{}={}}}^{{{}}} {}",
                        variable.name(),
                        start.to_latex_with_depth(context, depth + 1)?,
                        end.to_latex_with_depth(context, depth + 1)?,
                        expression.to_latex_with_depth(context, depth + 1)?
                    )
                }
            },
        })
    }
}
