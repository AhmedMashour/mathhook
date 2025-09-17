//! Core mathematical types and operations

pub mod commutativity;
pub mod constants;
pub mod expression;
pub mod matrix;
pub mod number;
pub mod performance;
pub mod polynomial;
pub mod symbol;

pub use commutativity::Commutativity;
pub use constants::MathConstant;
pub use expression::Expression;
pub use expression::ExpressionClass;
pub use matrix::NumericMatrix;
pub use number::Number;
pub use performance::*;
pub use polynomial::{PolynomialClassification, PolynomialError, PolynomialProperties};
pub use symbol::{Symbol, SymbolType};
