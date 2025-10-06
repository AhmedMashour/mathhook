//! Formatting traits for mathematical expressions

pub mod latex;
pub mod simple;
pub mod wolfram;

pub use latex::LaTeXFormatter;
pub use simple::SimpleFormatter;
pub use wolfram::WolframFormatter;

use std::fmt;

/// Mathematical language/format for expressions
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MathLanguage {
    LaTeX,
    Wolfram,
    Simple,
    Human,
    Json,
    Markdown,
}

impl Default for MathLanguage {
    fn default() -> Self {
        Self::LaTeX
    }
}

impl MathLanguage {
    /// Convert to format string for conditional formatting
    pub fn as_str(self) -> &'static str {
        match self {
            Self::LaTeX => "latex",
            Self::Wolfram => "wolfram",
            Self::Simple => "human", // Simple maps to human in the format! macro
            Self::Human => "human",
            Self::Json => "json",
            Self::Markdown => "markdown",
        }
    }
}

/// Structured error type for formatting operations
#[derive(Debug, Clone, PartialEq)]
pub enum FormattingError {
    /// Recursion depth limit exceeded during formatting
    RecursionLimitExceeded { depth: usize, limit: usize },
    /// Expression type not supported for target format
    UnsupportedExpression {
        expr_type: String,
        target_format: MathLanguage,
    },
    /// Too many terms in operation (performance limit)
    TooManyTerms { count: usize, limit: usize },
    /// Invalid mathematical construct
    InvalidMathConstruct { reason: String },
    /// Memory limit exceeded
    MemoryLimitExceeded,
    /// Serialization error (for JSON format)
    SerializationError { message: String },
}

impl fmt::Display for FormattingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::RecursionLimitExceeded { depth, limit } => {
                write!(f, "Recursion limit exceeded: {} > {}", depth, limit)
            }
            Self::UnsupportedExpression {
                expr_type,
                target_format,
            } => {
                write!(
                    f,
                    "Unsupported expression type '{}' for format {:?}",
                    expr_type, target_format
                )
            }
            Self::TooManyTerms { count, limit } => {
                write!(f, "Too many terms: {} > {}", count, limit)
            }
            Self::InvalidMathConstruct { reason } => {
                write!(f, "Invalid mathematical construct: {}", reason)
            }
            Self::MemoryLimitExceeded => {
                write!(f, "Memory limit exceeded during formatting")
            }
            Self::SerializationError { message } => {
                write!(f, "Serialization error: {}", message)
            }
        }
    }
}

impl std::error::Error for FormattingError {}

/// Context for formatting operations
pub trait FormattingContext: Default + Clone {
    fn target_format(&self) -> MathLanguage {
        MathLanguage::default()
    }
}

/// Base trait for all formatters
pub trait ExpressionFormatter<C: FormattingContext> {
    fn format(&self, context: &C) -> Result<String, FormattingError>;
}
