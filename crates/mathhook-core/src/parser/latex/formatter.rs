use super::LaTeXContext;
use super::LaTeXParser;
use crate::core::{Expression, Number};

impl LaTeXParser {
    /// Convert function to LaTeX
    /// # Examples
    /// ```rust
    /// use mathhook_core::parser::latex::LaTeXParser;
    /// let mut parser = LaTeXParser::new();
    /// let expr = parser.function_to_latex("sin", vec![Expression::symbol("x")], &LaTeXContext::default());
    /// assert_eq!(expr, "\\sin(x)");
    /// ```
    fn function_to_latex(&self, name: &str, args: &[Expression], context: &LaTeXContext) -> String {
        match name {
            // Trigonometric functions
            "sin" | "cos" | "tan" | "sec" | "csc" | "cot" => {
                if args.len() == 1 {
                    format!("\\{}({})", name, self.format(&args[0], context))
                } else {
                    format!(
                        "\\{}({})",
                        name,
                        args.iter()
                            .map(|a| self.format(a, context))
                            .collect::<Vec<_>>()
                            .join(", ")
                    )
                }
            }
            // Inverse trigonometric
            "arcsin" | "arccos" | "arctan" => {
                format!("\\{}({})", name, self.format(&args[0], context))
            }
            // Logarithmic functions
            "ln" => format!("\\ln({})", self.format(&args[0], context)),
            "log" => {
                if args.len() == 1 {
                    format!("\\log({})", self.format(&args[0], context))
                } else if args.len() == 2 {
                    format!(
                        "\\log_{{{}}}({})",
                        self.format(&args[1], context),
                        self.format(&args[0], context)
                    )
                } else {
                    format!(
                        "\\log({})",
                        args.iter()
                            .map(|a| self.format(a, context))
                            .collect::<Vec<_>>()
                            .join(", ")
                    )
                }
            }
            "exp" => format!("\\exp({})", self.format(&args[0], context)),
            // Special functions
            "sqrt" => {
                if args.len() == 1 {
                    format!("\\sqrt{{{}}}", self.format(&args[0], context))
                } else if args.len() == 2 {
                    format!(
                        "\\sqrt[{}]{{{}}}",
                        self.format(&args[1], context),
                        self.format(&args[0], context)
                    )
                } else {
                    format!("\\sqrt{{{}}}", self.format(&args[0], context))
                }
            }
            "factorial" => {
                if args.len() == 1 {
                    format!("{}!", self.format(&args[0], context))
                } else {
                    format!(
                        "\\text{{factorial}}({})",
                        args.iter()
                            .map(|a| self.format(a, context))
                            .collect::<Vec<_>>()
                            .join(", ")
                    )
                }
            }
            // Calculus
            "integrate" => {
                if args.len() == 2 {
                    format!(
                        "\\int {} \\, d{}",
                        self.format(&args[0], context),
                        self.format(&args[1], context)
                    )
                } else {
                    format!(
                        "\\int({})",
                        args.iter()
                            .map(|a| self.format(a, context))
                            .collect::<Vec<_>>()
                            .join(", ")
                    )
                }
            }
            "derivative" => {
                if args.len() == 2 {
                    format!(
                        "\\frac{{d}}{{d{}}} {}",
                        self.format(&args[1], context),
                        self.format(&args[0], context)
                    )
                } else {
                    format!("\\frac{{d}}{{dx}}({})", self.format(&args[0], context))
                }
            }
            "sum" => {
                if args.len() == 4 {
                    format!(
                        "\\sum_{{{}={}}}^{{{}}} {}",
                        self.format(&args[1], context),
                        self.format(&args[2], context),
                        self.format(&args[3], context),
                        self.format(&args[0], context)
                    )
                } else {
                    format!(
                        "\\sum({})",
                        args.iter()
                            .map(|a| self.format(a, context))
                            .collect::<Vec<_>>()
                            .join(", ")
                    )
                }
            }
            // Default case
            _ => {
                if args.is_empty() {
                    name.to_string()
                } else {
                    format!(
                        "\\text{{{}}}({})",
                        name,
                        args.iter()
                            .map(|a| self.format(a, context))
                            .collect::<Vec<_>>()
                            .join(", ")
                    )
                }
            }
        }
    }

    /// Convert Expression to LaTeX
    /// # Examples
    /// ```rust
    /// use mathhook_core::parser::latex::LaTeXParser;
    /// let mut parser = LaTeXParser::new();
    /// let expr = parser.format(&Expression::symbol("x"), &LaTeXContext::default());
    /// assert_eq!(expr, "x");
    /// ```
    pub fn format(&self, expr: &Expression, context: &LaTeXContext) -> String {
        match expr {
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
                let term_strs: Vec<String> = terms
                    .iter()
                    .enumerate()
                    .map(|(i, term)| {
                        if i == 0 {
                            self.format(term, context)
                        } else {
                            format!(" + {}", self.format(term, context))
                        }
                    })
                    .collect();
                if context.needs_parentheses {
                    format!("\\left({}\\right)", term_strs.join(""))
                } else {
                    term_strs.join("")
                }
            }
            Expression::Mul(factors) => {
                let factor_strs: Vec<String> = factors
                    .iter()
                    .map(|f| {
                        let needs_parens = matches!(f, Expression::Add(_));
                        if needs_parens {
                            format!("\\left({}\\right)", self.format(f, context))
                        } else {
                            self.format(f, context)
                        }
                    })
                    .collect();
                factor_strs.join(" \\cdot ")
            }
            Expression::Pow(base, exp) => {
                // Check if this is a square root: x^(1/2) -> \sqrt{x}
                if let Expression::Number(Number::Rational(r)) = exp.as_ref() {
                    if r.numer() == &num_bigint::BigInt::from(1)
                        && r.denom() == &num_bigint::BigInt::from(2)
                    {
                        return format!("\\sqrt{{{}}}", self.format(base, context));
                    }
                }

                let base_str = match base.as_ref() {
                    Expression::Add(_) | Expression::Mul(_) => {
                        format!("\\left({}\\right)", self.format(base, context))
                    }
                    _ => self.format(base, context),
                };
                format!("{}^{{{}}}", base_str, self.format(exp, context))
            }
            Expression::Function { name, args } => self.function_to_latex(name, args, context),
            // New expression types - implement later
            Expression::Complex(complex_data) => format!(
                "{} + {}i",
                self.format(&complex_data.real, context),
                self.format(&complex_data.imag, context)
            ),
            Expression::Matrix(matrix) => {
                let (rows, cols) = matrix.dimensions();
                let mut row_strs = Vec::with_capacity(rows);

                for i in 0..rows {
                    let mut col_strs = Vec::with_capacity(cols);
                    for j in 0..cols {
                        let elem = matrix.get_element(i, j);
                        col_strs.push(self.format(&elem, context));
                    }
                    row_strs.push(col_strs.join(" & "));
                }

                format!(
                    "\\begin{{pmatrix}} {} \\end{{pmatrix}}",
                    row_strs.join(" \\\\ ")
                )
            }
            Expression::Constant(c) => format!("{:?}", c),
            Expression::Relation { .. } => "\\text{relation}".to_string(),
            Expression::Piecewise { .. } => "\\text{piecewise}".to_string(),
            Expression::Set(elements) => {
                if elements.is_empty() {
                    "\\{\\}".to_string()
                } else {
                    let element_strs: Vec<String> = elements
                        .iter()
                        .map(|elem| self.format(elem, context))
                        .collect();
                    format!("\\{{{}\\}}", element_strs.join(", "))
                }
            }
            Expression::Interval { .. } => "\\text{interval}".to_string(),
            // Calculus expressions with proper LaTeX formatting
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
                                "\\frac{{d}}{{d{}}} {}",
                                variable.name(),
                                self.format(expression, context)
                            )
                        } else {
                            format!(
                                "\\frac{{d^{}}}{{d{}^{}}} {}",
                                order,
                                variable.name(),
                                order,
                                self.format(expression, context)
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
                            self.format(integrand, context),
                            variable.name()
                        ),
                        Some((start, end)) => format!(
                            "\\int_{{{}}}^{{{}}} {} d{}",
                            self.format(start, context),
                            self.format(end, context),
                            self.format(integrand, context),
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
                                crate::core::expression::LimitDirection::Left => "0^-",
                                crate::core::expression::LimitDirection::Right => "0^+",
                                crate::core::expression::LimitDirection::Both => "0",
                            },
                            self.format(expression, context)
                        )
                    }
                    CalculusData::Sum {
                        expression,
                        variable,
                        start,
                        end,
                    } => {
                        format!(
                            "\\sum_{{{}={}}}^{{{}}} {}",
                            variable.name(),
                            self.format(start, context),
                            self.format(end, context),
                            self.format(expression, context)
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
                            self.format(start, context),
                            self.format(end, context),
                            self.format(expression, context)
                        )
                    }
                }
            }
        }
    }
}
