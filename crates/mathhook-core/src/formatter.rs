//! Formatting traits for mathematical expressions

pub mod latex;
pub mod simple;
pub mod wolfram;

pub use latex::LaTeXFormatter;
pub use simple::SimpleFormatter;
pub use wolfram::WolframFormatter;

/// Context for formatting operations
pub trait FormattingContext: Default + Clone {}

/// Base trait for all formatters
pub trait ExpressionFormatter<C: FormattingContext> {
    fn format(&self, context: &C) -> String;
}
