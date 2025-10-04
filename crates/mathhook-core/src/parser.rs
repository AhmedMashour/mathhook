//! Multi-format mathematical expression parser for MathHook
//!
//! This crate provides parsing capabilities for various mathematical formats
//! including LaTeX, Wolfram Language, and standard mathematical notation.

use serde::{Deserialize, Serialize};

pub mod constants;
pub mod latex;
pub mod serialize;
pub mod simple;
pub mod universal;
pub mod utils;
pub mod wolfram;

pub use constants::*;
pub use serialize::*;
pub use universal::*;
pub use utils::*;
pub use wolfram::*;

pub mod lalrpop;
pub use lalrpop::*;

/// Parsing error types
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ParseError {
    InvalidSyntax(String),
    UnknownFunction(String),
    UnbalancedParentheses,
    InvalidNumber(String),
    EmptyInput,
    UnsupportedOperation(String),
    SyntaxError(String),
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::InvalidSyntax(msg) => write!(f, "Invalid syntax: {}", msg),
            ParseError::UnknownFunction(name) => write!(f, "Unknown function: {}", name),
            ParseError::UnbalancedParentheses => write!(f, "Unbalanced parentheses"),
            ParseError::InvalidNumber(num) => write!(f, "Invalid number: {}", num),
            ParseError::EmptyInput => write!(f, "Empty input"),
            ParseError::UnsupportedOperation(op) => write!(f, "Unsupported operation: {}", op),
            ParseError::SyntaxError(msg) => write!(f, "Syntax error: {}", msg),
        }
    }
}

impl std::error::Error for ParseError {}
