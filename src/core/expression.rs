//! Expression representation - the heart of the algebra system
//!
//! Modular organization following modern Rust practices.

// Import submodules
pub mod constructors;
pub mod conversion;
pub mod display;
pub mod operations;
pub mod tests;

// Data structures are defined in this module and don't need re-export

use crate::core::constants::MathConstant;
use crate::core::relations::{LimitDirection, RelationType};
use crate::core::{Number, Symbol};
use serde::{Deserialize, Serialize};

/// Complex number data (boxed for memory efficiency)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ComplexData {
    pub real: Expression,
    pub imag: Expression,
}

/// Matrix data (boxed for memory efficiency)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MatrixData {
    pub rows: Vec<Vec<Expression>>,
}

/// Relation data (boxed for memory efficiency)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RelationData {
    pub left: Expression,
    pub right: Expression,
    pub relation_type: RelationType,
}

/// Piecewise function data (boxed for memory efficiency)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PiecewiseData {
    pub cases: Vec<(Expression, Expression)>,
    pub default: Option<Expression>,
}

/// Interval data (boxed for memory efficiency)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IntervalData {
    pub start: Expression,
    pub end: Expression,
    pub start_inclusive: bool,
    pub end_inclusive: bool,
}

/// Calculus operation data (unified for memory efficiency)
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
        approach: Expression,
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

/// Expression with 32-byte optimization
/// Memory-optimized with boxed complex variants for cache efficiency
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Expression {
    // ========== HOT PATH VARIANTS (Keep inline for performance) ==========
    /// Optimized number representation (16 bytes)
    Number(Number),
    /// Symbol (variable) (24 bytes)
    Symbol(Symbol),
    /// Addition with boxed vector for memory (8 bytes)
    Add(Box<Vec<Expression>>),
    /// Multiplication with boxed vector for memory (8 bytes)
    Mul(Box<Vec<Expression>>),
    /// Power operation with boxed expressions (16 bytes)
    Pow(Box<Expression>, Box<Expression>),
    /// Function call with boxed arguments (32 bytes)
    Function {
        name: String,
        args: Box<Vec<Expression>>,
    },
    /// Mathematical constants (8 bytes)
    Constant(MathConstant),
    /// Set representation (8 bytes)
    Set(Box<Vec<Expression>>),

    // ========== COLD PATH VARIANTS (Boxed for memory efficiency) ==========
    /// Complex number representation (8 bytes - boxed)
    Complex(Box<ComplexData>),
    /// Matrix representation (8 bytes - boxed)
    Matrix(Box<MatrixData>),
    /// Equations and relations (8 bytes - boxed)
    Relation(Box<RelationData>),
    /// Piecewise functions (8 bytes - boxed)
    Piecewise(Box<PiecewiseData>),
    /// Interval notation (8 bytes - boxed)
    Interval(Box<IntervalData>),
    /// Calculus operations (8 bytes - boxed, unified)
    Calculus(Box<CalculusData>),
}
