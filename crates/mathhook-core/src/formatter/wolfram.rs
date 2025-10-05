use super::FormattingContext;
use crate::core::expression::CalculusData;
use crate::core::{Expression, Number};

const MAX_RECURSION_DEPTH: usize = 1000;
const MAX_TERMS_PER_OPERATION: usize = 10000;

/// Wolfram formatting context
#[derive(Debug, Default, Clone)]
pub struct WolframContext {
    pub needs_parentheses: bool,
}

impl FormattingContext for WolframContext {}

/// Format the expression to Wolfram Language
pub trait WolframFormatter {
    /// Format an Expression as Wolfram Language notation
    ///
    /// Converts mathematical expressions into Wolfram Language format suitable for
    /// use in Mathematica and other Wolfram products.
    ///
    /// # Arguments
    /// * `context` - Wolfram formatting configuration
    ///
    /// # Context Options
    /// * `needs_parentheses` - Whether to wrap the entire expression in parentheses
    ///
    /// # Examples
    /// ```
    /// use mathhook_core::core::Expression;
    /// use mathhook_core::formatter::wolfram::{WolframFormatter, WolframContext};
    ///
    /// let expr = Expression::from("x^2 + 1");
    /// let context = WolframContext::default();
    /// let result = expr.to_wolfram(&context);
    /// assert_eq!(result, "Plus[Power[x, 2], 1]");
    /// ```
    ///
    /// # Error Handling
    /// Returns error messages for expressions that exceed safety limits:
    /// - Maximum recursion depth (1000 levels)
    /// - Maximum terms per operation (10000 terms)
    fn to_wolfram(&self, context: &WolframContext) -> String {
        match self.to_wolfram_with_depth(context, 0) {
            Ok(result) => result,
            Err(error) => format!("Error: {}", error),
        }
    }

    /// Format with explicit recursion depth tracking
    ///
    /// Internal method that provides stack overflow protection by tracking
    /// recursion depth. This method returns a Result to allow proper error
    /// propagation during recursive formatting.
    ///
    /// # Arguments
    /// * `context` - Wolfram formatting configuration
    /// * `depth` - Current recursion depth (starts at 0)
    ///
    /// # Returns
    /// * `Ok(String)` - Successfully formatted Wolfram expression
    /// * `Err(String)` - Error message if limits exceeded
    ///
    /// # Safety Limits
    /// * Maximum recursion depth: 1000 levels
    /// * Maximum terms per operation: 10000 terms/factors/arguments
    fn to_wolfram_with_depth(
        &self,
        context: &WolframContext,
        depth: usize,
    ) -> Result<String, String>;

    /// Convert function to Wolfram Language with depth tracking
    fn format_function_with_depth(
        &self,
        name: &str,
        args: &[Expression],
        context: &WolframContext,
        depth: usize,
    ) -> Result<String, String>;
}

impl WolframFormatter for Expression {
    fn to_wolfram_with_depth(
        &self,
        context: &WolframContext,
        depth: usize,
    ) -> Result<String, String> {
        if depth > MAX_RECURSION_DEPTH {
            return Err("Maximum recursion depth exceeded".to_string());
        }

        match self {
            Expression::Number(Number::Integer(n)) => Ok(n.to_string()),
            Expression::Number(Number::BigInteger(n)) => Ok(n.to_string()),
            Expression::Number(Number::Rational(r)) => {
                if r.denom() == &num_bigint::BigInt::from(1) {
                    Ok(r.numer().to_string())
                } else {
                    // Use Power[denominator, -1] for proper Wolfram syntax
                    Ok(format!("Times[{}, Power[{}, -1]]", r.numer(), r.denom()))
                }
            }
            Expression::Number(Number::Float(f)) => Ok(f.to_string()),
            Expression::Symbol(s) => Ok(s.name().to_string()),
            Expression::Add(terms) => {
                if terms.len() > MAX_TERMS_PER_OPERATION {
                    return Err(format!(
                        "Too many terms in addition: {} (max: {})",
                        terms.len(),
                        MAX_TERMS_PER_OPERATION
                    ));
                }

                if terms.len() == 1 {
                    terms[0].to_wolfram_with_depth(context, depth + 1)
                } else {
                    let mut term_strs = Vec::with_capacity(terms.len());
                    for term in terms.iter() {
                        term_strs.push(term.to_wolfram_with_depth(context, depth + 1)?);
                    }
                    Ok(format!("Plus[{}]", term_strs.join(", ")))
                }
            }
            Expression::Mul(factors) => {
                if factors.len() > MAX_TERMS_PER_OPERATION {
                    return Err(format!(
                        "Too many factors in multiplication: {} (max: {})",
                        factors.len(),
                        MAX_TERMS_PER_OPERATION
                    ));
                }

                if factors.len() == 1 {
                    factors[0].to_wolfram_with_depth(context, depth + 1)
                } else {
                    let mut factor_strs = Vec::with_capacity(factors.len());
                    for factor in factors.iter() {
                        factor_strs.push(factor.to_wolfram_with_depth(context, depth + 1)?);
                    }
                    Ok(format!("Times[{}]", factor_strs.join(", ")))
                }
            }
            Expression::Pow(base, exp) => Ok(format!(
                "Power[{}, {}]",
                base.to_wolfram_with_depth(context, depth + 1)?,
                exp.to_wolfram_with_depth(context, depth + 1)?
            )),
            Expression::Function { name, args } => {
                self.format_function_with_depth(name, args, context, depth + 1)
            }
            Expression::Complex(complex_data) => Ok(format!(
                "Complex[{}, {}]",
                complex_data
                    .real
                    .to_wolfram_with_depth(context, depth + 1)?,
                complex_data
                    .imag
                    .to_wolfram_with_depth(context, depth + 1)?
            )),
            Expression::Matrix(_) => Ok("matrix".to_string()),
            Expression::Constant(c) => Ok(format!("{:?}", c)),
            Expression::Relation(_) => Ok("relation".to_string()),
            Expression::Piecewise(_) => Ok("piecewise".to_string()),
            Expression::Set(elements) => {
                if elements.len() > MAX_TERMS_PER_OPERATION {
                    return Err(format!(
                        "Too many set elements: {} (max: {})",
                        elements.len(),
                        MAX_TERMS_PER_OPERATION
                    ));
                }

                if elements.is_empty() {
                    Ok("{}".to_string())
                } else {
                    let mut element_strs = Vec::with_capacity(elements.len());
                    for elem in elements.iter() {
                        element_strs.push(elem.to_wolfram_with_depth(context, depth + 1)?);
                    }
                    Ok(format!("{{{}}}", element_strs.join(", ")))
                }
            }
            Expression::Interval(_) => Ok("interval".to_string()),
            Expression::Calculus(calculus_data) => {
                Ok(match calculus_data.as_ref() {
                    CalculusData::Derivative {
                        expression,
                        variable,
                        order,
                    } => {
                        if *order == 1 {
                            format!(
                                "D[{}, {}]",
                                expression.to_wolfram_with_depth(context, depth + 1)?,
                                variable.name()
                            )
                        } else {
                            format!(
                                "D[{}, {{{}, {}}}]",
                                expression.to_wolfram_with_depth(context, depth + 1)?,
                                variable.name(),
                                order
                            )
                        }
                    }
                    CalculusData::Integral {
                        integrand,
                        variable,
                        bounds,
                    } => match bounds {
                        None => format!(
                            "Integrate[{}, {}]",
                            integrand.to_wolfram_with_depth(context, depth + 1)?,
                            variable.name()
                        ),
                        Some((start, end)) => format!(
                            "Integrate[{}, {{{}, {}, {}}}]",
                            integrand.to_wolfram_with_depth(context, depth + 1)?,
                            variable.name(),
                            start.to_wolfram_with_depth(context, depth + 1)?,
                            end.to_wolfram_with_depth(context, depth + 1)?
                        ),
                    },
                    CalculusData::Limit {
                        expression,
                        variable,
                        point,
                        direction: _,
                    } => {
                        // Simplified Wolfram limit format for roundtrip consistency
                        format!(
                            "Limit[{}, {} -> {}]",
                            expression.to_wolfram_with_depth(context, depth + 1)?,
                            variable.name(),
                            point.to_wolfram_with_depth(context, depth + 1)?
                        )
                    }
                    CalculusData::Sum {
                        expression,
                        variable,
                        start,
                        end,
                    } => {
                        format!(
                            "Sum[{}, {{{}, {}, {}}}]",
                            expression.to_wolfram_with_depth(context, depth + 1)?,
                            variable.name(),
                            start.to_wolfram_with_depth(context, depth + 1)?,
                            end.to_wolfram_with_depth(context, depth + 1)?
                        )
                    }
                    CalculusData::Product {
                        expression,
                        variable,
                        start,
                        end,
                    } => {
                        format!(
                            "Product[{}, {{{}, {}, {}}}]",
                            expression.to_wolfram_with_depth(context, depth + 1)?,
                            variable.name(),
                            start.to_wolfram_with_depth(context, depth + 1)?,
                            end.to_wolfram_with_depth(context, depth + 1)?
                        )
                    }
                })
            }
        }
    }

    /// Convert function to Wolfram Language with depth tracking
    fn format_function_with_depth(
        &self,
        name: &str,
        args: &[Expression],
        context: &WolframContext,
        depth: usize,
    ) -> Result<String, String> {
        if args.len() > MAX_TERMS_PER_OPERATION {
            return Err(format!(
                "Too many function arguments: {} (max: {})",
                args.len(),
                MAX_TERMS_PER_OPERATION
            ));
        }

        let wolfram_name = match name {
            // Trigonometric functions
            "sin" => "Sin",
            "cos" => "Cos",
            "tan" => "Tan",
            "sec" => "Sec",
            "csc" => "Csc",
            "cot" => "Cot",
            "arcsin" => "ArcSin",
            "arccos" => "ArcCos",
            "arctan" => "ArcTan",
            // Logarithmic
            "ln" => "Log",
            "log" => "Log",
            "exp" => "Exp",
            // Special functions
            "sqrt" => "Sqrt",
            "factorial" => "Factorial",
            "gamma" => "Gamma",
            "zeta" => "Zeta",
            "beta" => "Beta",
            // Calculus
            "integrate" => "Integrate",
            "derivative" => "D",
            "sum" => "Sum",
            "product" => "Product",
            "limit" => "Limit",
            // Default
            _ => name,
        };

        if args.is_empty() {
            Ok(wolfram_name.to_string())
        } else {
            let mut arg_strs = Vec::with_capacity(args.len());
            for arg in args.iter() {
                arg_strs.push(arg.to_wolfram_with_depth(context, depth + 1)?);
            }
            Ok(format!("{}[{}]", wolfram_name, arg_strs.join(", ")))
        }
    }
}
