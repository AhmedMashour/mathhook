//! Conversion traits for expressions
//!
//! Implements From/Into traits for ergonomic expression creation.

use super::Expression;
use crate::core::{Number, Symbol};

impl From<i32> for Expression {
    fn from(value: i32) -> Self {
        Self::integer(value as i64)
    }
}

impl From<i64> for Expression {
    fn from(value: i64) -> Self {
        Self::integer(value)
    }
}

impl From<f64> for Expression {
    fn from(value: f64) -> Self {
        Self::Number(Number::float(value))
    }
}

impl From<Symbol> for Expression {
    fn from(symbol: Symbol) -> Self {
        Self::Symbol(symbol)
    }
}

impl From<&str> for Expression {
    fn from(name: &str) -> Self {
        Self::Symbol(Symbol::new(name))
    }
}
