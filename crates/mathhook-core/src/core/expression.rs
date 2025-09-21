//! Expression type and core functionality

pub mod constructors;
pub mod conversion;
pub mod data_types;
pub mod display;
pub mod methods;
pub mod operations;
pub mod operators;

pub use constructors::*;
pub use data_types::*;

use crate::core::{MathConstant, Number, Symbol};
use serde::{Deserialize, Serialize};

/// Memory-optimized Expression enum (target: 32 bytes)
///
/// Hot-path variants (frequently used) are kept inline for performance.
/// Cold-path variants (less common) are boxed to maintain small enum size.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Expression {
    Number(Number),
    Symbol(Symbol),
    Add(Box<Vec<Expression>>),
    Mul(Box<Vec<Expression>>),
    Pow(Box<Expression>, Box<Expression>),
    Function {
        name: String,
        args: Box<Vec<Expression>>,
    },
    Constant(MathConstant),
    Set(Box<Vec<Expression>>),
    Complex(Box<ComplexData>),
    Matrix(Box<MatrixData>),
    Relation(Box<RelationData>),
    Piecewise(Box<PiecewiseData>),
    Interval(Box<IntervalData>),
    Calculus(Box<CalculusData>),
}
