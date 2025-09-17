/// Parsing error types
use serde::{Deserialize, Serialize};

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
