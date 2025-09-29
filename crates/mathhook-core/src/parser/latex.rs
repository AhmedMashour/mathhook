//! LaTeX mathematical expression parser
//!
//! Handles LaTeX-specific syntax including fractions, functions, and commands.

pub mod formatter;
pub mod parser;

/// LaTeX-specific parser
pub struct LaTeXParser {}

impl LaTeXParser {
    pub fn new() -> Self {
        Self {}
    }
}

/// LaTeX formatting context
#[derive(Debug, Default)]
pub struct LaTeXContext {
    needs_parentheses: bool,
}
