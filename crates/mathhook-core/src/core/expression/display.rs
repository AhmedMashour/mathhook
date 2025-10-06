//! Display implementation for Expression

use super::smart_display::SmartDisplayFormatter;
use super::{Expression, RelationType};
use std::fmt;

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expression::Number(num) => write!(f, "{:?}", num),
            Expression::Symbol(sym) => write!(f, "{}", sym.name),
            Expression::Add(terms) => {
                if terms.is_empty() {
                    write!(f, "0")
                } else {
                    SmartDisplayFormatter::format_addition_smartly(f, terms)
                }
            }
            Expression::Mul(factors) => {
                if factors.is_empty() {
                    write!(f, "1")
                } else {
                    SmartDisplayFormatter::format_multiplication_smartly(f, factors)
                }
            }
            Expression::Pow(base, exp) => write!(f, "{}^{}", base, exp),
            Expression::Function { name, args } => {
                let arg_strs: Vec<String> = args.iter().map(|a| format!("{}", a)).collect();
                write!(f, "{}({})", name, arg_strs.join(", "))
            }
            Expression::Constant(c) => write!(f, "{:?}", c),
            Expression::Set(elements) => {
                let elem_strs: Vec<String> = elements.iter().map(|e| format!("{}", e)).collect();
                write!(f, "{{{}}}", elem_strs.join(", "))
            }
            Expression::Complex(data) => write!(f, "{} + {}i", data.real, data.imag),
            Expression::Matrix(matrix) => {
                let (rows, cols) = matrix.dimensions();
                let mut row_strs = Vec::with_capacity(rows);

                for i in 0..rows {
                    let mut col_strs = Vec::with_capacity(cols);
                    for j in 0..cols {
                        col_strs.push(format!("{}", matrix.get_element(i, j)));
                    }
                    row_strs.push(format!("[{}]", col_strs.join(", ")));
                }

                write!(f, "[{}]", row_strs.join(", "))
            }
            Expression::Relation(data) => {
                let op = match data.relation_type {
                    RelationType::Equal => "=",
                    RelationType::NotEqual => "≠",
                    RelationType::Less => "<",
                    RelationType::LessEqual => "≤",
                    RelationType::Greater => ">",
                    RelationType::GreaterEqual => "≥",
                    RelationType::Approximate => "≈",
                    RelationType::Similar => "∼",
                    RelationType::Proportional => "∝",
                };
                write!(f, "{} {} {}", data.left, op, data.right)
            }
            Expression::MethodCall(method_data) => {
                if method_data.args.is_empty() {
                    write!(f, "{}.{}()", method_data.object, method_data.method_name)
                } else {
                    let arg_strs: Vec<String> =
                        method_data.args.iter().map(|a| format!("{}", a)).collect();
                    write!(
                        f,
                        "{}.{}({})",
                        method_data.object,
                        method_data.method_name,
                        arg_strs.join(", ")
                    )
                }
            }
            _ => write!(f, "{:?}", self),
        }
    }
}
