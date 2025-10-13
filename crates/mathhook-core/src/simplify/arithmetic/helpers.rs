//! Helper functions for arithmetic simplification

use crate::core::{Expression, Number};
use num_traits::ToPrimitive;
use std::cmp::Ordering;

/// Canonical ordering for expressions to ensure consistent output
pub(super) fn expression_order(a: &Expression, b: &Expression) -> Ordering {
    match (a, b) {
        // Numbers come first, ordered by value
        (Expression::Number(n1), Expression::Number(n2)) => {
            // Convert to f64 for comparison (handles integers, floats, rationals)
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

        // Symbols come next, ordered alphabetically
        (Expression::Symbol(s1), Expression::Symbol(s2)) => s1.name().cmp(s2.name()),
        (Expression::Symbol(_), _) => Ordering::Less,
        (_, Expression::Symbol(_)) => Ordering::Greater,

        // Add expressions ordered by their first term
        (Expression::Add(terms1), Expression::Add(terms2)) => {
            if let (Some(first1), Some(first2)) = (terms1.first(), terms2.first()) {
                expression_order(first1, first2)
            } else {
                terms1.len().cmp(&terms2.len())
            }
        }
        (Expression::Add(_), _) => Ordering::Greater,
        (_, Expression::Add(_)) => Ordering::Less,

        // Mul expressions ordered by their first factor
        (Expression::Mul(factors1), Expression::Mul(factors2)) => {
            if let (Some(first1), Some(first2)) = (factors1.first(), factors2.first()) {
                expression_order(first1, first2)
            } else {
                factors1.len().cmp(&factors2.len())
            }
        }
        (Expression::Mul(_), _) => Ordering::Greater,
        (_, Expression::Mul(_)) => Ordering::Less,

        // For other expressions, use debug representation for consistent ordering
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
            // Check if first factor is numeric
            if matches!(factors[0], Expression::Number(_)) {
                let coeff = factors[0].clone();
                let base = if factors.len() == 2 {
                    factors[1].clone()
                } else {
                    Expression::Mul(Box::new(factors[1..].to_vec()))
                };
                (coeff, base)
            } else {
                // No numeric coefficient, coefficient is 1
                (Expression::integer(1), expr.clone())
            }
        }
        _ => {
            // Single term, coefficient is 1
            (Expression::integer(1), expr.clone())
        }
    }
}
