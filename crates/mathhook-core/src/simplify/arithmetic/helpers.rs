//! Helper functions for arithmetic simplification

use crate::core::{Expression, Number};
use num_traits::ToPrimitive;
use std::cmp::Ordering;
use std::sync::Arc;

/// Canonical ordering for expressions to ensure consistent output
pub(super) fn expression_order(a: &Expression, b: &Expression) -> Ordering {
    match (a, b) {
        (Expression::Number(n1), Expression::Number(n2)) => {
            let val1 = match n1 {
                Number::Integer(i) => *i as f64,
                Number::Float(f) => *f,
                Number::Rational(r) => r.to_f64().unwrap_or(0.0),
                _ => 0.0,
            };
            let val2 = match n2 {
                Number::Integer(i) => *i as f64,
                Number::Float(f) => *f,
                Number::Rational(r) => r.to_f64().unwrap_or(0.0),
                _ => 0.0,
            };
            val1.partial_cmp(&val2).unwrap_or(Ordering::Equal)
        }
        (Expression::Number(_), _) => Ordering::Less,
        (_, Expression::Number(_)) => Ordering::Greater,

        (Expression::Symbol(s1), Expression::Symbol(s2)) => s1.name().cmp(s2.name()),
        (Expression::Symbol(_), _) => Ordering::Less,
        (_, Expression::Symbol(_)) => Ordering::Greater,

        (Expression::Add(terms1), Expression::Add(terms2)) => {
            if let (Some(first1), Some(first2)) = (terms1.first(), terms2.first()) {
                expression_order(first1, first2)
            } else {
                terms1.len().cmp(&terms2.len())
            }
        }
        (Expression::Add(_), _) => Ordering::Greater,
        (_, Expression::Add(_)) => Ordering::Less,

        (Expression::Mul(factors1), Expression::Mul(factors2)) => {
            if let (Some(first1), Some(first2)) = (factors1.first(), factors2.first()) {
                expression_order(first1, first2)
            } else {
                factors1.len().cmp(&factors2.len())
            }
        }
        (Expression::Mul(_), _) => Ordering::Greater,
        (_, Expression::Mul(_)) => Ordering::Less,

        (
            Expression::Function {
                name: name1,
                args: args1,
            },
            Expression::Function {
                name: name2,
                args: args2,
            },
        ) => {
            let name_cmp = name1.cmp(name2);
            if name_cmp != Ordering::Equal {
                return name_cmp;
            }

            use crate::core::commutativity::Commutativity;

            let args1_commutativity =
                Commutativity::combine(args1.iter().map(|arg| arg.commutativity()));
            let args2_commutativity =
                Commutativity::combine(args2.iter().map(|arg| arg.commutativity()));

            if args1_commutativity.can_sort() && args2_commutativity.can_sort() {
                for (arg1, arg2) in args1.iter().zip(args2.iter()) {
                    let arg_cmp = expression_order(arg1, arg2);
                    if arg_cmp != Ordering::Equal {
                        return arg_cmp;
                    }
                }
                args1.len().cmp(&args2.len())
            } else {
                Ordering::Equal
            }
        }
        (Expression::Function { .. }, _) => Ordering::Greater,
        (_, Expression::Function { .. }) => Ordering::Less,

        _ => format!("{:?}", a).cmp(&format!("{:?}", b)),
    }
}

/// Extract coefficient and base term from an expression for arithmetic operations
///
/// # Examples
///
/// - `3*x` -> `(3, x)`
/// - `-2*y` -> `(-2, y)`
/// - `x` -> `(1, x)`
pub(super) fn extract_arithmetic_coefficient_and_base(
    expr: &Expression,
) -> (Expression, Expression) {
    match expr {
        Expression::Mul(factors) if factors.len() >= 2 => {
            if matches!(factors[0], Expression::Number(_)) {
                let coeff = factors[0].clone();
                let base = if factors.len() == 2 {
                    factors[1].clone()
                } else {
                    Expression::Mul(Arc::new(factors[1..].to_vec()))
                };
                (coeff, base)
            } else {
                (Expression::integer(1), expr.clone())
            }
        }
        _ => (Expression::integer(1), expr.clone()),
    }
}
