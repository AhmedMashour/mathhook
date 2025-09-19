//! Multi-format mathematical expression parser for MathHook
//!
//! This crate provides parsing capabilities for various mathematical formats
//! including LaTeX, Wolfram Language, and standard mathematical notation.

use mathhook_core::Expression;
use serde::{Deserialize, Serialize};

/// Supported mathematical input formats
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MathLanguage {
    LaTeX,
    Wolfram,
    Standard,
    Infix,
}

/// Parsing error types
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ParseError {
    InvalidSyntax(String),
    UnknownFunction(String),
    UnbalancedParentheses,
    InvalidNumber(String),
    EmptyInput,
    UnsupportedOperation(String),
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
        }
    }
}

impl std::error::Error for ParseError {}

/// Mathematical expression parser
///
/// This is the main parser interface that can handle multiple mathematical formats.
///
/// # Examples
///
/// ```rust
/// use mathhook_parser::{MathParser, MathLanguage};
///
/// let parser = MathParser::new();
///
/// // Parse LaTeX
/// let expr = parser.parse("x^2 + 2x + 1", MathLanguage::LaTeX)?;
///
/// // Parse standard notation
/// let expr = parser.parse("sin(x) + cos(y)", MathLanguage::Standard)?;
/// # Ok::<(), mathhook_parser::ParseError>(())
/// ```
pub struct MathParser {
    // Parser configuration could go here
}

impl MathParser {
    /// Create a new parser
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_parser::MathParser;
    ///
    /// let parser = MathParser::new();
    /// ```
    pub fn new() -> Self {
        Self {}
    }

    /// Parse a mathematical expression from a string
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_parser::{MathParser, MathLanguage};
    ///
    /// let parser = MathParser::new();
    /// let expr = parser.parse("2 + 3", MathLanguage::Standard)?;
    /// # Ok::<(), mathhook_parser::ParseError>(())
    /// ```
    pub fn parse(&self, input: &str, language: MathLanguage) -> Result<Expression, ParseError> {
        if input.trim().is_empty() {
            return Err(ParseError::EmptyInput);
        }

        match language {
            MathLanguage::Standard | MathLanguage::Infix => self.parse_standard(input),
            MathLanguage::LaTeX => self.parse_latex(input),
            MathLanguage::Wolfram => self.parse_wolfram(input),
        }
    }

    /// Parse LaTeX mathematical notation
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_parser::MathParser;
    ///
    /// let parser = MathParser::new();
    /// let expr = parser.parse_latex("\\frac{x^2}{2} + 3")?;
    /// # Ok::<(), mathhook_parser::ParseError>(())
    /// ```
    pub fn parse_latex(&self, input: &str) -> Result<Expression, ParseError> {
        // Basic LaTeX parsing implementation
        self.parse_basic(input)
    }

    /// Parse Wolfram Language notation
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_parser::MathParser;
    ///
    /// let parser = MathParser::new();
    /// let expr = parser.parse_wolfram("Sin[x] + Cos[y]")?;
    /// # Ok::<(), mathhook_parser::ParseError>(())
    /// ```
    pub fn parse_wolfram(&self, input: &str) -> Result<Expression, ParseError> {
        // Basic Wolfram parsing implementation
        self.parse_basic(input)
    }

    /// Parse standard mathematical notation
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_parser::MathParser;
    ///
    /// let parser = MathParser::new();
    /// let expr = parser.parse_standard("sin(x) + cos(y)")?;
    /// # Ok::<(), mathhook_parser::ParseError>(())
    /// ```
    pub fn parse_standard(&self, input: &str) -> Result<Expression, ParseError> {
        self.parse_basic(input)
    }

    // Basic parsing implementation for the minimal viable version
    fn parse_basic(&self, input: &str) -> Result<Expression, ParseError> {
        let input = input.trim();

        // Handle simple numbers
        if let Ok(num) = input.parse::<i64>() {
            return Ok(Expression::integer(num));
        }

        if let Ok(num) = input.parse::<f64>() {
            return Ok(Expression::number(num));
        }

        // Handle simple symbols
        if input.chars().all(|c| c.is_alphabetic() || c == '_') {
            return Ok(Expression::symbol(input));
        }

        // Handle basic addition: "a + b"
        if let Some(plus_pos) = input.find(" + ") {
            let left = &input[..plus_pos];
            let right = &input[plus_pos + 3..];

            let left_expr = self.parse_basic(left)?;
            let right_expr = self.parse_basic(right)?;

            return Ok(Expression::add(vec![left_expr, right_expr]));
        }

        // Handle basic multiplication: "a * b"
        if let Some(mul_pos) = input.find(" * ") {
            let left = &input[..mul_pos];
            let right = &input[mul_pos + 3..];

            let left_expr = self.parse_basic(left)?;
            let right_expr = self.parse_basic(right)?;

            return Ok(Expression::mul(vec![left_expr, right_expr]));
        }

        // Handle basic powers: "a^b"
        if let Some(pow_pos) = input.find('^') {
            let base = &input[..pow_pos];
            let exponent = &input[pow_pos + 1..];

            let base_expr = self.parse_basic(base)?;
            let exp_expr = self.parse_basic(exponent)?;

            return Ok(Expression::pow(base_expr, exp_expr));
        }

        Err(ParseError::InvalidSyntax(format!(
            "Cannot parse: {}",
            input
        )))
    }
}

impl Default for MathParser {
    fn default() -> Self {
        Self::new()
    }
}

/// Format expressions back to strings
pub struct MathFormatter;

impl MathFormatter {
    /// Format an expression as LaTeX
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_parser::MathFormatter;
    /// use mathhook_core::Expression;
    ///
    /// let expr = Expression::add(vec![
    ///     Expression::symbol("x"),
    ///     Expression::integer(1),
    /// ]);
    /// let latex = MathFormatter::to_latex(&expr);
    /// ```
    pub fn to_latex(expr: &Expression) -> String {
        match expr {
            Expression::Number(num) => format!("{:?}", num), // Simplified
            Expression::Symbol(sym) => sym.name.clone(),
            Expression::Add(terms) => {
                let term_strs: Vec<String> = terms.iter().map(|t| Self::to_latex(t)).collect();
                term_strs.join(" + ")
            }
            Expression::Mul(factors) => {
                let factor_strs: Vec<String> = factors.iter().map(|f| Self::to_latex(f)).collect();
                factor_strs.join(" \\cdot ")
            }
            Expression::Pow(base, exp) => {
                format!("{}^{{{}}}", Self::to_latex(base), Self::to_latex(exp))
            }
            Expression::Function { name, args } => {
                let arg_strs: Vec<String> = args.iter().map(|a| Self::to_latex(a)).collect();
                format!("\\{}({})", name, arg_strs.join(", "))
            }
            _ => format!("{:?}", expr), // Fallback for complex types
        }
    }

    /// Format an expression as standard notation
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_parser::MathFormatter;
    /// use mathhook_core::Expression;
    ///
    /// let expr = Expression::pow(Expression::symbol("x"), Expression::integer(2));
    /// let standard = MathFormatter::to_standard(&expr);
    /// ```
    pub fn to_standard(expr: &Expression) -> String {
        match expr {
            Expression::Number(num) => format!("{:?}", num), // Simplified
            Expression::Symbol(sym) => sym.name.clone(),
            Expression::Add(terms) => {
                let term_strs: Vec<String> = terms.iter().map(|t| Self::to_standard(t)).collect();
                term_strs.join(" + ")
            }
            Expression::Mul(factors) => {
                let factor_strs: Vec<String> =
                    factors.iter().map(|f| Self::to_standard(f)).collect();
                factor_strs.join(" * ")
            }
            Expression::Pow(base, exp) => {
                format!("{}^{}", Self::to_standard(base), Self::to_standard(exp))
            }
            Expression::Function { name, args } => {
                let arg_strs: Vec<String> = args.iter().map(|a| Self::to_standard(a)).collect();
                format!("{}({})", name, arg_strs.join(", "))
            }
            _ => format!("{:?}", expr), // Fallback for complex types
        }
    }
}
