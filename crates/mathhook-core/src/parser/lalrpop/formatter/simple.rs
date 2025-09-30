/// Simple mathematical notation formatter
///
/// Outputs clean mathematical notation that can be perfectly re-parsed
/// by the LALRPOP grammar with optimal performance.
use crate::core::{Expression, MathConstant, Number};

/// Format Expression as simple mathematical notation
///
/// Optimized for roundtrip consistency with LALRPOP grammar
pub fn format(expr: &Expression) -> String {
    match expr {
        Expression::Number(Number::Integer(n)) => n.to_string(),
        Expression::Number(Number::BigInteger(n)) => n.to_string(),
        Expression::Number(Number::Float(f)) => f.to_string(),
        Expression::Number(Number::Rational(r)) => {
            if r.denom() == &num_bigint::BigInt::from(1) {
                r.numer().to_string()
            } else {
                format!("({}/{})", r.numer(), r.denom())
            }
        }

        Expression::Symbol(s) => s.name().to_string(),

        Expression::Constant(c) => match c {
            MathConstant::Pi => "pi".to_string(),
            MathConstant::E => "e".to_string(),
            MathConstant::I => "i".to_string(),
            MathConstant::Infinity => "infinity".to_string(),
            MathConstant::NegativeInfinity => "-infinity".to_string(),
            MathConstant::Undefined => "undefined".to_string(),
            MathConstant::GoldenRatio => "phi".to_string(),
            MathConstant::EulerGamma => "gamma".to_string(),
            MathConstant::TribonacciConstant => "tribonacci_constant".to_string(),
        },

        Expression::Add(terms) => {
            if terms.is_empty() {
                "0".to_string()
            } else if terms.len() == 1 {
                format(&terms[0])
            } else {
                let mut result = format(&terms[0]);

                for term in &terms[1..] {
                    // Check if this is a negative multiplication like -1 * y
                    if let Expression::Mul(factors) = term {
                        if factors.len() == 2 {
                            if let Expression::Number(Number::Integer(n)) = &factors[0] {
                                if *n == -1 {
                                    result.push_str(" - ");
                                    result.push_str(&format(&factors[1]));
                                    continue;
                                }
                            }
                        }
                    }

                    // Handle regular negative terms
                    let term_str = format(term);
                    if term_str.starts_with('-') {
                        result.push_str(" - ");
                        result.push_str(&term_str[1..]);
                    } else {
                        result.push_str(" + ");
                        result.push_str(&term_str);
                    }
                }

                result
            }
        }

        Expression::Mul(factors) => {
            if factors.is_empty() {
                "1".to_string()
            } else if factors.len() == 1 {
                format(&factors[0])
            } else {
                let factor_strs: Vec<String> = factors
                    .iter()
                    .map(|f| {
                        let needs_parens = matches!(f, Expression::Add(_));
                        if needs_parens {
                            format!("({})", format(f))
                        } else {
                            format(f)
                        }
                    })
                    .collect();
                factor_strs.join(" * ")
            }
        }

        Expression::Pow(base, exp) => {
            let base_str = format(base);
            let exp_str = format(exp);

            // Add parentheses around base if it's complex
            let base_formatted = if matches!(**base, Expression::Add(_) | Expression::Mul(_)) {
                format!("({})", base_str)
            } else {
                base_str
            };

            // Add parentheses around exponent if it's complex
            if exp_str.contains(' ') || exp_str.starts_with('-') {
                format!("{}^({})", base_formatted, exp_str)
            } else {
                format!("{}^{}", base_formatted, exp_str)
            }
        }

        Expression::Function { name, args } => {
            if args.is_empty() {
                name.clone()
            } else {
                let arg_strs: Vec<String> = args.iter().map(format).collect();
                format!("{}({})", name, arg_strs.join(", "))
            }
        }

        Expression::Set(elements) => {
            let element_strs: Vec<String> = elements.iter().map(format).collect();
            format!("{{{}}}", element_strs.join(", "))
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
            format!(
                "{}{}, {}{}",
                start_bracket,
                format(&interval_data.start),
                format(&interval_data.end),
                end_bracket
            )
        }

        Expression::Relation(relation_data) => {
            let op = match relation_data.relation_type {
                crate::core::expression::RelationType::Equal => "=",
                crate::core::expression::RelationType::NotEqual => "!=",
                crate::core::expression::RelationType::Less => "<",
                crate::core::expression::RelationType::LessEqual => "<=",
                crate::core::expression::RelationType::Greater => ">",
                crate::core::expression::RelationType::GreaterEqual => ">=",
            };
            format!(
                "{} {} {}",
                format(&relation_data.left),
                op,
                format(&relation_data.right)
            )
        }

        Expression::Complex(complex_data) => {
            let real_str = format(&complex_data.real);
            let imag_str = format(&complex_data.imag);

            if imag_str.starts_with('-') {
                format!("{} - {}i", real_str, &imag_str[1..])
            } else {
                format!("{} + {}i", real_str, imag_str)
            }
        }

        Expression::Matrix(matrix) => {
            // Simple matrix representation for basic parsing
            let (rows, cols) = matrix.dimensions();
            format!("matrix({}x{})", rows, cols)
        }

        Expression::Piecewise(_) => {
            // Simplified piecewise representation
            "piecewise(...)".to_string()
        }

        Expression::Calculus(_) => {
            // Simplified calculus representation
            "calculus(...)".to_string()
        }
    }
}
