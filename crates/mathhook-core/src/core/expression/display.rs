//! Display implementation for Expression

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
                    let term_strs: Vec<String> = terms.iter().map(|t| format!("{}", t)).collect();
                    write!(f, "{}", term_strs.join(" + "))
                }
            }
            Expression::Mul(factors) => {
                if factors.is_empty() {
                    write!(f, "1")
                } else {
                    let factor_strs: Vec<String> =
                        factors.iter().map(|f| format!("{}", f)).collect();
                    write!(f, "{}", factor_strs.join(" * "))
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
                };
                write!(f, "{} {} {}", data.left, op, data.right)
            }
            _ => write!(f, "{:?}", self),
        }
    }
}
