//! Display formatting for expressions
//!
//! Implements fmt::Display for natural mathematical notation output.

use super::Expression;
use crate::core::{LimitDirection, MathConstant, RelationType};
use std::fmt;

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expression::Number(n) => write!(f, "{}", n),
            Expression::Symbol(s) => write!(f, "{}", s),
            Expression::Add(terms) => {
                if terms.is_empty() {
                    write!(f, "0")
                } else {
                    write!(f, "(")?;
                    for (i, term) in terms.iter().enumerate() {
                        if i > 0 {
                            write!(f, " + ")?;
                        }
                        write!(f, "{}", term)?;
                    }
                    write!(f, ")")
                }
            }
            Expression::Mul(factors) => {
                if factors.is_empty() {
                    write!(f, "1")
                } else {
                    write!(f, "(")?;
                    for (i, factor) in factors.iter().enumerate() {
                        if i > 0 {
                            write!(f, " * ")?;
                        }
                        write!(f, "{}", factor)?;
                    }
                    write!(f, ")")
                }
            }
            Expression::Pow(base, exp) => {
                write!(f, "({})^({})", base, exp)
            }
            Expression::Function { name, args } => {
                write!(f, "{}(", name)?;
                for (i, arg) in args.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", arg)?;
                }
                write!(f, ")")
            }
            Expression::Complex(complex_data) => {
                write!(f, "({} + {}i)", complex_data.real, complex_data.imag)
            }
            Expression::Matrix(matrix_data) => {
                write!(f, "[")?;
                for (i, row) in matrix_data.rows.iter().enumerate() {
                    if i > 0 {
                        write!(f, "; ")?;
                    }
                    write!(f, "[")?;
                    for (j, elem) in row.iter().enumerate() {
                        if j > 0 {
                            write!(f, ", ")?;
                        }
                        write!(f, "{}", elem)?;
                    }
                    write!(f, "]")?;
                }
                write!(f, "]")
            }
            Expression::Constant(c) => match c {
                MathConstant::Pi => write!(f, "π"),
                MathConstant::E => write!(f, "e"),
                MathConstant::I => write!(f, "i"),
                MathConstant::Infinity => write!(f, "∞"),
                MathConstant::NegInfinity => write!(f, "-∞"),
                MathConstant::Undefined => write!(f, "undefined"),
                MathConstant::GoldenRatio => write!(f, "φ"),
                MathConstant::EulerGamma => write!(f, "γ"),
            },
            Expression::Relation(relation_data) => {
                let symbol = match relation_data.relation_type {
                    RelationType::Equal => "=",
                    RelationType::NotEqual => "≠",
                    RelationType::Less => "<",
                    RelationType::Greater => ">",
                    RelationType::LessEqual => "≤",
                    RelationType::GreaterEqual => "≥",
                    RelationType::Approximately => "≈",
                };
                write!(
                    f,
                    "{} {} {}",
                    relation_data.left, symbol, relation_data.right
                )
            }
            Expression::Set(elements) => {
                write!(f, "{{")?;
                for (i, elem) in elements.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", elem)?;
                }
                write!(f, "}}")
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
                write!(
                    f,
                    "{}{}, {}{}",
                    start_bracket, interval_data.start, interval_data.end, end_bracket
                )
            }
            Expression::Piecewise(piecewise_data) => {
                write!(f, "piecewise(")?;
                for (i, (condition, value)) in piecewise_data.cases.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{} if {}", value, condition)?;
                }
                if let Some(def) = &piecewise_data.default {
                    write!(f, ", {} otherwise", def)?;
                }
                write!(f, ")")
            }
            Expression::Calculus(calculus_data) => match calculus_data.as_ref() {
                super::CalculusData::Derivative {
                    expression,
                    variable,
                    order,
                } => {
                    if *order == 1 {
                        write!(f, "d/d{} ({})", variable.name(), expression)
                    } else {
                        write!(
                            f,
                            "d^{}/d{}^{} ({})",
                            order,
                            variable.name(),
                            order,
                            expression
                        )
                    }
                }
                super::CalculusData::Integral {
                    integrand,
                    variable,
                    bounds,
                } => match bounds {
                    None => write!(f, "∫ {} d{}", integrand, variable.name()),
                    Some((start, end)) => write!(
                        f,
                        "∫[{} to {}] {} d{}",
                        start,
                        end,
                        integrand,
                        variable.name()
                    ),
                },
                super::CalculusData::Limit {
                    expression,
                    variable,
                    approach,
                    direction,
                } => {
                    let dir_str = match direction {
                        LimitDirection::Both => "",
                        LimitDirection::Left => "⁻",
                        LimitDirection::Right => "⁺",
                    };
                    write!(
                        f,
                        "lim({} → {}{}) {}",
                        variable.name(),
                        approach,
                        dir_str,
                        expression
                    )
                }
                super::CalculusData::Sum {
                    expression,
                    variable,
                    start,
                    end,
                } => {
                    write!(
                        f,
                        "Σ({}={} to {}) {}",
                        variable.name(),
                        start,
                        end,
                        expression
                    )
                }
                super::CalculusData::Product {
                    expression,
                    variable,
                    start,
                    end,
                } => {
                    write!(
                        f,
                        "Π({}={} to {}) {}",
                        variable.name(),
                        start,
                        end,
                        expression
                    )
                }
            },
        }
    }
}
