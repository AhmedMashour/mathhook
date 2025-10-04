use crate::core::{Expression, Number};

impl Formatter {
    /// Format an Expression as simple mathematical notation (no LaTeX commands)
    pub fn format_simple(&self, expr: &Expression) -> String {
        match expr {
            Expression::Number(Number::Integer(n)) => n.to_string(),
            Expression::Number(Number::BigInteger(n)) => n.to_string(),
            Expression::Number(Number::Rational(r)) => {
                if r.denom() == &num_bigint::BigInt::from(1) {
                    r.numer().to_string()
                } else {
                    format!("{}/{}", r.numer(), r.denom())
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
                            self.format(term)
                        } else {
                            // Handle negative terms properly
                            let term_str = self.format(term);
                            if term_str.starts_with('-') {
                                format!(" - {}", &term_str[1..])
                            } else {
                                format!(" + {}", term_str)
                            }
                        }
                    })
                    .collect();
                term_strs.join("")
            }
            Expression::Mul(factors) => {
                let factor_strs: Vec<String> = factors
                    .iter()
                    .map(|f| {
                        let needs_parens = matches!(f, Expression::Add(_));
                        if needs_parens {
                            format!("({})", self.format(f))
                        } else {
                            self.format(f)
                        }
                    })
                    .collect();
                factor_strs.join(" * ")
            }
            Expression::Pow(base, exp) => {
                let base_simple = self.format(base);
                let exp_simple = self.format(exp);
                // Add parentheses around negative or complex exponents for clarity
                if exp_simple.starts_with('-') || exp_simple.contains(' ') {
                    format!("{}^({})", base_simple, exp_simple)
                } else {
                    format!("{}^{}", base_simple, exp_simple)
                }
            }
            Expression::Function { name, args } => {
                if args.is_empty() {
                    name.clone()
                } else {
                    let arg_strs: Vec<String> = args.iter().map(|a| self.format(a)).collect();
                    format!("{}({})", name, arg_strs.join(", "))
                }
            }
            _ => "unknown".to_string(),
        }
    }
}
