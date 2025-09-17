//! Data structures for complex Expression variants

use super::Expression;
use crate::core::Symbol;
use serde::{Deserialize, Serialize};

/// Complex number data (boxed for memory optimization)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ComplexData {
    pub real: Expression,
    pub imag: Expression,
}

/// Relation data (boxed for memory optimization)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RelationData {
    pub left: Expression,
    pub right: Expression,
    pub relation_type: RelationType,
}

/// Piecewise function data (boxed for memory optimization)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PiecewiseData {
    pub pieces: Vec<(Expression, Expression)>,
    pub default: Option<Expression>,
}

/// Interval data (boxed for memory optimization)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IntervalData {
    pub start: Expression,
    pub end: Expression,
    pub start_inclusive: bool,
    pub end_inclusive: bool,
}

/// Calculus operations (boxed for memory optimization)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CalculusData {
    Derivative {
        expression: Expression,
        variable: Symbol,
        order: u32,
    },
    Integral {
        integrand: Expression,
        variable: Symbol,
        bounds: Option<(Expression, Expression)>,
    },
    Limit {
        expression: Expression,
        variable: Symbol,
        point: Expression,
        direction: LimitDirection,
    },
    Sum {
        expression: Expression,
        variable: Symbol,
        start: Expression,
        end: Expression,
    },
    Product {
        expression: Expression,
        variable: Symbol,
        start: Expression,
        end: Expression,
    },
}

/// Relation types for equations and inequalities
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum RelationType {
    Equal,
    NotEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    Approximate,
    Similar,
    Proportional,
    Congruent,
}

/// Direction for limit operations
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum LimitDirection {
    Both,
    Left,
    Right,
}

/// Method call data for object.method() syntax
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MethodCallData {
    pub object: Expression,
    pub method_name: String,
    pub args: Vec<Expression>,
}

#[cfg(test)]
mod expression_size_tests {
    use super::*;
    use std::mem::size_of;

    #[test]
    fn test_expression_size() {
        let size = size_of::<Expression>();
        assert_eq!(
            size, 32,
            "Expression size is {} bytes, MUST be exactly 32 bytes! \
             Noncommutative algebra support must NOT increase Expression size.",
            size
        );
    }
}
