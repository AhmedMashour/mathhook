//! Mathematical operations and queries for expressions
//!
//! Methods for checking properties and extracting information from expressions.

use super::Expression;
use crate::core::{Number, Symbol};

impl Expression {
    /// Check if the expression is zero (optimized)
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        match self {
            Expression::Number(n) => n.is_zero(),
            _ => false,
        }
    }

    /// Check if the expression is one (optimized)
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        match self {
            Expression::Number(n) => n.is_one(),
            _ => false,
        }
    }

    /// Get the numeric coefficient if this is a simple numeric expression
    #[inline]
    pub fn as_number(&self) -> Option<&Number> {
        match self {
            Expression::Number(n) => Some(n),
            _ => None,
        }
    }

    /// Get the symbol if this is a simple symbol expression
    #[inline]
    pub fn as_symbol(&self) -> Option<&Symbol> {
        match self {
            Expression::Symbol(s) => Some(s),
            _ => None,
        }
    }
}
