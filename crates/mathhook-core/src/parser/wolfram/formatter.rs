use super::WolframParser;
use crate::core::{Expression, Number};
use crate::parser::wolfram::WolframContext;

impl WolframParser {
    /// Convert Expression to Wolfram Language
    /// # Examples
    /// ```
    /// use mathhook::core::Expression;
    /// use mathhook::parser::wolfram::WolframParser;
    /// use mathhook::parser::wolfram::WolframContext;
    ///
    /// let expr = Expression::from("1 + 2 * 3");
    /// let context = WolframContext::default();
    /// let result = WolframParser::format(&expr, &context);
    /// assert_eq!(result, "Plus[1, Times[2, 3]]");
    /// ```
    pub fn format(&self, expr: &Expression, context: &WolframContext) -> String {
        match expr {
            Expression::Number(Number::Integer(n)) => n.to_string(),
            Expression::Number(Number::BigInteger(n)) => n.to_string(),
            Expression::Number(Number::Rational(r)) => {
                if r.denom() == &num_bigint::BigInt::from(1) {
                    r.numer().to_string()
                } else {
                    // Use Power[denominator, -1] for proper Wolfram syntax
                    format!("Times[{}, Power[{}, -1]]", r.numer(), r.denom())
                }
            }
            Expression::Number(Number::Float(f)) => f.to_string(),
            Expression::Symbol(s) => s.name().to_string(),
            Expression::Add(terms) => {
                if terms.len() == 1 {
                    self.format(&terms[0], context)
                } else {
                    let term_strs: Vec<String> =
                        terms.iter().map(|t| self.format(t, context)).collect();
                    format!("Plus[{}]", term_strs.join(", "))
                }
            }
            Expression::Mul(factors) => {
                if factors.len() == 1 {
                    self.format(&factors[0], context)
                } else {
                    let factor_strs: Vec<String> =
                        factors.iter().map(|f| self.format(f, context)).collect();
                    format!("Times[{}]", factor_strs.join(", "))
                }
            }
            Expression::Pow(base, exp) => {
                format!(
                    "Power[{}, {}]",
                    self.format(base, context),
                    self.format(exp, context)
                )
            }
            Expression::Function { name, args } => self.format_function(name, args, context),
            // New expression types - implement later
            Expression::Complex(complex_data) => format!(
                "Complex[{}, {}]",
                self.format(&complex_data.real, context),
                self.format(&complex_data.imag, context)
            ),
            Expression::Matrix(_) => "matrix".to_string(),
            Expression::Constant(c) => format!("{:?}", c),
            Expression::Relation(_) => "relation".to_string(),
            Expression::Piecewise(_) => "piecewise".to_string(),
            Expression::Set(elements) => {
                if elements.is_empty() {
                    "{}".to_string()
                } else {
                    let element_strs: Vec<String> = elements
                        .iter()
                        .map(|elem| self.format(elem, context))
                        .collect();
                    format!("{{{}}}", element_strs.join(", "))
                }
            }
            Expression::Interval(_) => "interval".to_string(),
            // Calculus expressions with proper Wolfram formatting
            Expression::Calculus(calculus_data) => {
                use crate::core::expression::CalculusData;
                match calculus_data.as_ref() {
                    CalculusData::Derivative {
                        expression,
                        variable,
                        order,
                    } => {
                        if *order == 1 {
                            format!(
                                "D[{}, {}]",
                                self.format(expression, context),
                                variable.name()
                            )
                        } else {
                            format!(
                                "D[{}, {{{}, {}}}]",
                                self.format(expression, context),
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
                            self.format(integrand, context),
                            variable.name()
                        ),
                        Some((start, end)) => format!(
                            "Integrate[{}, {{{}, {}, {}}}]",
                            self.format(integrand, context),
                            variable.name(),
                            self.format(start, context),
                            self.format(end, context)
                        ),
                    },
                    CalculusData::Limit {
                        expression,
                        variable,
                        point,
                        direction,
                    } => {
                        // Simplified Wolfram limit format for roundtrip consistency
                        format!(
                            "Limit[{}, {} -> {}]",
                            self.format(expression, context),
                            variable.name(),
                            self.format(point, context)
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
                            self.format(expression, context),
                            variable.name(),
                            self.format(start, context),
                            self.format(end, context)
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
                            self.format(expression, context),
                            variable.name(),
                            self.format(start, context),
                            self.format(end, context)
                        )
                    }
                }
            }
        }
    }

    /// Convert function to Wolfram Language
    fn format_function(&self, name: &str, args: &[Expression], context: &WolframContext) -> String {
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
            wolfram_name.to_string()
        } else {
            let arg_strs: Vec<String> = args.iter().map(|a| self.format(a, context)).collect();
            format!("{}[{}]", wolfram_name, arg_strs.join(", "))
        }
    }
}
