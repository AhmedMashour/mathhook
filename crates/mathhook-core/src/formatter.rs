//! Formatting traits for mathematical expressions

pub mod latex;
pub mod simple;
pub mod wolfram;

pub use latex::LaTeXFormatter;
pub use simple::SimpleFormatter;
pub use wolfram::WolframFormatter;

use crate::core::Expression;
use std::fmt;
/// Mathematical language/format for expressions
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MathLanguage {
    #[default]
    LaTeX,
    Wolfram,
    Simple,
    Human,
    Json,
    Markdown,
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

impl<C: FormattingContext> ExpressionFormatter<C> for Expression {
    fn format(&self, context: &C) -> Result<String, FormattingError> {
        match context.target_format() {
            MathLanguage::Simple | MathLanguage::Human => {
                let simple_context = simple::SimpleContext::default();
                self.to_simple(&simple_context)
            }
            MathLanguage::Wolfram => {
                let wolfram_context = wolfram::WolframContext::default();
                self.to_wolfram(&wolfram_context)
            }
            MathLanguage::Json => {
                // Use serde_json for JSON formatting
                serde_json::to_string_pretty(self).map_err(|e| {
                    FormattingError::SerializationError {
                        message: e.to_string(),
                    }
                })
            }
            MathLanguage::Markdown => {
                // Use LaTeX formatting wrapped in markdown math blocks
                let latex_context = latex::LaTeXContext::default();
                let latex_result = self.to_latex(latex_context)?;
                Ok(format!("$${}$$", latex_result))
            }
            // Default to LaTeX for all other cases including MathLanguage::LaTeX
            _ => {
                let latex_context = latex::LaTeXContext::default();
                self.to_latex(latex_context)
            }
        }
    }
}

/// Convenient formatting methods for Expression without requiring context
impl Expression {
    /// Format expression using default LaTeX formatting
    ///
    /// This is the most convenient way to format expressions when you don't need
    /// specific formatting options. Always uses LaTeX format.
    ///
    /// # Examples
    /// ```rust
    /// use mathhook_core::core::Expression;
    /// use mathhook_core::{expr};
    ///
    /// let x_expr = expr!(x);
    /// let formatted = x_expr.format().unwrap();
    /// // Returns LaTeX formatted string
    /// ```
    pub fn format(&self) -> Result<String, FormattingError> {
        let latex_context = latex::LaTeXContext::default();
        self.to_latex(latex_context)
    }

    /// Format expression with specific language/format
    ///
    /// # Examples
    /// ```rust
    /// use mathhook_core::core::Expression;
    /// use mathhook_core::formatter::MathLanguage;
    /// use mathhook_core::expr;
    ///
    /// let x_expr = expr!(x);
    /// let latex = x_expr.format_as(MathLanguage::LaTeX).unwrap();
    /// let simple = x_expr.format_as(MathLanguage::Simple).unwrap();
    /// let wolfram = x_expr.format_as(MathLanguage::Wolfram).unwrap();
    /// ```
    pub fn format_as(&self, language: MathLanguage) -> Result<String, FormattingError> {
        match language {
            MathLanguage::Simple | MathLanguage::Human => {
                let simple_context = simple::SimpleContext::default();
                self.to_simple(&simple_context)
            }
            MathLanguage::Wolfram => {
                let wolfram_context = wolfram::WolframContext::default();
                self.to_wolfram(&wolfram_context)
            }
            MathLanguage::Json => serde_json::to_string_pretty(self).map_err(|e| {
                FormattingError::SerializationError {
                    message: e.to_string(),
                }
            }),
            MathLanguage::Markdown => {
                let latex_context = latex::LaTeXContext::default();
                let latex_result = self.to_latex(latex_context)?;
                Ok(format!("$${}$$", latex_result))
            }
            // Default to LaTeX
            _ => {
                let latex_context = latex::LaTeXContext::default();
                self.to_latex(latex_context)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::Expression;
    use crate::expr;

    /// Test context that defaults to LaTeX
    #[derive(Debug, Default, Clone)]
    struct TestContext;

    impl FormattingContext for TestContext {}

    #[test]
    fn test_format_defaults_to_latex() {
        let x_expr = expr!(x);
        // Should use LaTeX formatting by default
        let result = ExpressionFormatter::format(&x_expr, &TestContext);
        assert!(result.is_ok());
    }

    #[test]
    fn test_format_without_context() {
        let x_expr = expr!(x);

        // Should work without providing context (defaults to LaTeX)
        let result = x_expr.format();
        assert!(result.is_ok());

        // Test format_as method
        let latex_result = x_expr.format_as(MathLanguage::LaTeX);
        assert!(latex_result.is_ok());

        let simple_result = x_expr.format_as(MathLanguage::Simple);
        assert!(simple_result.is_ok());
    }

    #[test]
    fn test_comprehensive_formatting() {
        use crate::core::expression::RelationType;

        // Test interval formatting
        let interval = Expression::interval(expr!(0), expr!(10), true, false);

        let latex_interval = interval.format().unwrap();
        assert!(latex_interval.contains("[0"));
        assert!(latex_interval.contains("10)"));

        let simple_interval = interval.format_as(MathLanguage::Simple).unwrap();
        assert!(simple_interval.contains("[0"));
        assert!(simple_interval.contains("10)"));

        // Test relation formatting
        let relation = Expression::relation(expr!(x), expr!(5), RelationType::Greater);

        let latex_relation = relation.format().unwrap();
        assert!(latex_relation.contains("x"));
        assert!(latex_relation.contains("5"));

        let simple_relation = relation.format_as(MathLanguage::Simple).unwrap();
        assert!(simple_relation.contains("x > 5"));

        // Test piecewise formatting
        let piecewise = Expression::piecewise(
            vec![(expr!(x), expr!(1)), (expr!(y), expr!(2))],
            Some(expr!(0)),
        );

        let latex_piecewise = piecewise.format().unwrap();
        assert!(latex_piecewise.contains("\\begin{cases}"));

        let simple_piecewise = piecewise.format_as(MathLanguage::Simple).unwrap();
        assert!(simple_piecewise.contains("if"));
        assert!(simple_piecewise.contains("otherwise"));
    }
}
