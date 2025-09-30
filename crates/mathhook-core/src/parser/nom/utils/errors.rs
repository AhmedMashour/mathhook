/// Error handling utilities for nom parsing
///
/// Provides custom error types and conversion utilities for mathematical expression parsing.

use nom::{error::ParseError as NomParseError, IResult};
use std::fmt;

/// Custom error type for mathematical expression parsing
#[derive(Debug, Clone, PartialEq)]
pub enum MathParseError {
    /// Invalid number format
    InvalidNumber(String),
    /// Invalid identifier
    InvalidIdentifier(String),
    /// Unexpected token
    UnexpectedToken(String),
    /// Missing operand
    MissingOperand,
    /// Unmatched parentheses
    UnmatchedParentheses,
    /// Generic parsing error
    ParseError(String),
}

impl fmt::Display for MathParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MathParseError::InvalidNumber(s) => write!(f, "Invalid number: {}", s),
            MathParseError::InvalidIdentifier(s) => write!(f, "Invalid identifier: {}", s),
            MathParseError::UnexpectedToken(s) => write!(f, "Unexpected token: {}", s),
            MathParseError::MissingOperand => write!(f, "Missing operand"),
            MathParseError::UnmatchedParentheses => write!(f, "Unmatched parentheses"),
            MathParseError::ParseError(s) => write!(f, "Parse error: {}", s),
        }
    }
}

impl std::error::Error for MathParseError {}

/// Result type for mathematical parsing operations
pub type ParseResult<T> = Result<T, MathParseError>;

/// Convert nom error to our custom error type
pub fn convert_error<I>(err: nom::Err<nom::error::Error<I>>) -> MathParseError {
    match err {
        nom::Err::Incomplete(_) => MathParseError::ParseError("Incomplete input".to_string()),
        nom::Err::Error(e) | nom::Err::Failure(e) => {
            MathParseError::ParseError(format!("Parse error: {:?}", e.code))
        }
    }
}

/// Custom nom error type that integrates with our error system
#[derive(Debug, Clone, PartialEq)]
pub struct CustomError<I> {
    pub input: I,
    pub error: MathParseError,
}

impl<I> NomParseError<I> for CustomError<I> {
    fn from_error_kind(input: I, _kind: nom::error::ErrorKind) -> Self {
        CustomError {
            input,
            error: MathParseError::ParseError("Generic parse error".to_string()),
        }
    }

    fn append(_input: I, _kind: nom::error::ErrorKind, other: Self) -> Self {
        other
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = MathParseError::InvalidNumber("abc".to_string());
        assert_eq!(format!("{}", err), "Invalid number: abc");

        let err = MathParseError::MissingOperand;
        assert_eq!(format!("{}", err), "Missing operand");
    }
}
