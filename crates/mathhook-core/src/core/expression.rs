//! Expression type and core functionality

pub mod classification;
pub mod constructors;
pub mod conversion;
pub mod data_types;
pub mod display;
pub mod eval_numeric;
pub mod evaluation;
pub mod matrix_methods;
pub mod methods;
pub mod operations;
pub mod operators;
pub mod smart_display;

pub use classification::ExpressionClass;

pub use crate::matrices::unified::Matrix;
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
    Matrix(Box<Matrix>),
    Relation(Box<RelationData>),
    Piecewise(Box<PiecewiseData>),
    Interval(Box<IntervalData>),
    Calculus(Box<CalculusData>),
    MethodCall(Box<MethodCallData>),
}

#[cfg(test)]
mod size_tests {
    use super::*;

    #[test]
    fn test_expression_size_constraint() {
        assert_eq!(
            std::mem::size_of::<Expression>(),
            32,
            "Expression size constraint violated! Expected 32 bytes, got {} bytes. \
             This breaks cache-line optimization.",
            std::mem::size_of::<Expression>()
        );
    }
}
