use super::{FormattingContext, FormattingError};
use crate::core::Expression;

mod expressions;
mod functions;

const MAX_RECURSION_DEPTH: usize = 1000;
const MAX_TERMS_PER_OPERATION: usize = 10000;

/// LaTeX formatting context
#[derive(Debug, Default, Clone)]
pub struct LaTeXContext {
    pub needs_parentheses: bool,
}

impl FormattingContext for LaTeXContext {}

/// Format the expression to LaTeX
pub trait LaTeXFormatter {
    /// Format an Expression as LaTeX mathematical notation
    ///
    /// Converts mathematical expressions into LaTeX format suitable for
    /// rendering in mathematical documents and publications.
    ///
    /// # Arguments
    /// * `context` - LaTeX formatting configuration
    ///
    /// # Context Options
    /// * `needs_parentheses` - Whether to wrap the entire expression in parentheses
    ///
    /// # Examples
    /// ```
    /// use mathhook_core::{Expression, expr};
    /// use mathhook_core::formatter::latex::{LaTeXFormatter, LaTeXContext};
    ///
    /// let expression = expr!(x ^ 2);
    /// let context = LaTeXContext::default();
    /// let result = expression.to_latex(context).unwrap();
    /// assert!(result.contains("x"));
    /// assert!(result.contains("2"));
    /// ```
    ///
    /// # Error Handling
    /// Returns error messages for expressions that exceed safety limits:
    /// - Maximum recursion depth (1000 levels)
    /// - Maximum terms per operation (10000 terms)
    fn to_latex<C>(&self, context: C) -> Result<String, FormattingError>
    where
        C: Into<Option<LaTeXContext>>,
    {
        let context = context.into().unwrap_or_default();
        self.to_latex_with_depth(&context, 0)
    }

    /// Format with explicit recursion depth tracking
    ///
    /// Internal method that provides stack overflow protection by tracking
    /// recursion depth. This method returns a Result to allow proper error
    /// propagation during recursive formatting.
    ///
    /// # Arguments
    /// * `context` - LaTeX formatting configuration
    /// * `depth` - Current recursion depth (starts at 0)
    ///
    /// # Returns
    /// * `Ok(String)` - Successfully formatted LaTeX expression
    /// * `Err(String)` - Error message if limits exceeded
    ///
    /// # Safety Limits
    /// * Maximum recursion depth: 1000 levels
    /// * Maximum terms per operation: 10000 terms/factors/arguments
    fn to_latex_with_depth(
        &self,
        context: &LaTeXContext,
        depth: usize,
    ) -> Result<String, FormattingError>;

    /// Convert function to LaTeX with context and depth tracking
    fn function_to_latex_with_depth(
        &self,
        name: &str,
        args: &[Expression],
        context: &LaTeXContext,
        depth: usize,
    ) -> Result<String, FormattingError>;

    /// Convert function to LaTeX (convenience method)
    fn function_to_latex(
        &self,
        name: &str,
        args: &[Expression],
        context: &LaTeXContext,
    ) -> Result<String, FormattingError> {
        match self.function_to_latex_with_depth(name, args, context, 0) {
            Ok(result) => Ok(result),
            Err(error) => Err(FormattingError::InvalidMathConstruct {
                reason: error.to_string(),
            }),
        }
    }
}

impl LaTeXFormatter for Expression {
    fn to_latex_with_depth(
        &self,
        context: &LaTeXContext,
        depth: usize,
    ) -> Result<String, FormattingError> {
        expressions::to_latex_with_depth_impl(self, context, depth)
    }

    fn function_to_latex_with_depth(
        &self,
        name: &str,
        args: &[Expression],
        context: &LaTeXContext,
        depth: usize,
    ) -> Result<String, FormattingError> {
        functions::function_to_latex_with_depth_impl(self, name, args, context, depth)
    }
}
