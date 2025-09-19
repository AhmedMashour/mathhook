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

/// Matrix data (boxed for memory optimization)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MatrixData {
    pub rows: Vec<Vec<Expression>>,
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
}

/// Direction for limit operations
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum LimitDirection {
    Both,
    Left,
    Right,
}
