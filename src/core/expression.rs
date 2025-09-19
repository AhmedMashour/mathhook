//! Expression representation - the heart of the algebra system
//!
//! Modular organization following modern Rust practices.

// Import submodules
pub mod constructors;
pub mod conversion;
pub mod display;
pub mod operations;
pub mod tests;

// Re-exports will be added here when external modules need them

use crate::core::constants::MathConstant;
use crate::core::relations::{LimitDirection, RelationType};
use crate::core::{Number, Symbol};
use serde::{Deserialize, Serialize};

/// Expression with 32-byte optimization
/// Memory-optimized with boxed vectors for cache
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Expression {
    /// Optimized number representation
    Number(Number),
    /// Symbol (variable)
    Symbol(Symbol),
    /// Addition with boxed vector for memory
    Add(Box<Vec<Expression>>),
    /// Multiplication with boxed vector for memory
    Mul(Box<Vec<Expression>>),
    /// Power operation with boxed expressions
    Pow(Box<Expression>, Box<Expression>),
    /// Function call with boxed arguments
    Function {
        name: String,
        args: Box<Vec<Expression>>,
    },
    /// Complex number representation
    Complex {
        real: Box<Expression>,
        imag: Box<Expression>,
    },
    /// Matrix representation
    Matrix(Box<Vec<Vec<Expression>>>),
    /// Mathematical constants
    Constant(MathConstant),
    /// Equations and relations
    Relation {
        left: Box<Expression>,
        right: Box<Expression>,
        relation_type: RelationType,
    },
    /// Piecewise functions
    Piecewise {
        cases: Box<Vec<(Expression, Expression)>>, // (condition, value)
        default: Option<Box<Expression>>,
    },
    /// Set representation
    Set(Box<Vec<Expression>>),
    /// Interval notation
    Interval {
        start: Box<Expression>,
        end: Box<Expression>,
        start_inclusive: bool,
        end_inclusive: bool,
    },
    /// Derivative
    Derivative {
        expression: Box<Expression>,
        variable: Symbol,
        order: u32,
    },
    /// Integral
    Integral {
        integrand: Box<Expression>,
        variable: Symbol,
        bounds: Option<(Box<Expression>, Box<Expression>)>, // None = indefinite, Some = definite
    },
    /// Limit
    Limit {
        expression: Box<Expression>,
        variable: Symbol,
        approach: Box<Expression>,
        direction: LimitDirection,
    },
    /// Summation
    Sum {
        expression: Box<Expression>,
        variable: Symbol,
        start: Box<Expression>,
        end: Box<Expression>,
    },
    /// Product
    Product {
        expression: Box<Expression>,
        variable: Symbol,
        start: Box<Expression>,
        end: Box<Expression>,
    },
}
