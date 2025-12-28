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
use std::sync::Arc;

/// Memory-optimized Expression enum (target: 32 bytes)
///
/// Uses Arc for O(1) clone performance - cloning is just an atomic increment.
/// Hot-path variants (frequently used) are kept inline for performance.
/// Cold-path variants (less common) use Arc to maintain small enum size.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Expression {
    Number(Number),
    Symbol(Symbol),
    Add(Arc<Vec<Expression>>),
    Mul(Arc<Vec<Expression>>),
    Pow(Arc<Expression>, Arc<Expression>),
    Function {
        name: Arc<str>,
        args: Arc<Vec<Expression>>,
    },
    Constant(MathConstant),
    Set(Arc<Vec<Expression>>),
    Complex(Arc<ComplexData>),
    Matrix(Arc<Matrix>),
    Relation(Arc<RelationData>),
    Piecewise(Arc<PiecewiseData>),
    Interval(Arc<IntervalData>),
    Calculus(Arc<CalculusData>),
    MethodCall(Arc<MethodCallData>),
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
