//! Parser integration for Expression - thread-local cached parsing
//!
//! This module provides convenient parser methods directly on Expression
//! using thread-local caching for optimal performance.

use crate::core::Expression;
use crate::parser::{
    universal::{MathLanguage, UniversalParser},
    ParseError,
};
use std::cell::RefCell;

thread_local! {
    /// Thread-local parser instance for optimal performance
    ///
    /// Each thread gets its own parser with persistent variable cache,
    /// avoiding allocation overhead while maintaining thread safety.
    static PARSER: RefCell<UniversalParser> = RefCell::new(UniversalParser::new());
}

impl Expression {
    /// Parse a mathematical expression from string with automatic language detection
    ///
    /// Uses thread-local cached parser for optimal performance.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::Expression;
    ///
    /// // Automatic language detection
    /// let expr1 = Expression::parse("x^2 + 2*x + 1").unwrap();
    /// let expr2 = Expression::parse("\\frac{x^2}{2}").unwrap();      // LaTeX
    /// let expr3 = Expression::parse("Sin[x] + Cos[y]").unwrap();     // Wolfram
    ///
    /// // Chaining
    /// let result = Expression::parse("x^2 + 1").unwrap()
    ///     .simplify()
    ///     .to_latex();
    /// ```
    pub fn parse(input: &str) -> Result<Expression, ParseError> {
        PARSER.with(|parser| parser.borrow_mut().parse(input))
    }

    /// Parse with explicit language specification
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{Expression, parser::universal::MathLanguage};
    ///
    /// let latex = Expression::parse_with_language("\\sin(x)", MathLanguage::LaTeX).unwrap();
    /// let wolfram = Expression::parse_with_language("Sin[x]", MathLanguage::Wolfram).unwrap();
    /// let simple = Expression::parse_with_language("x + 1", MathLanguage::Simple).unwrap();
    /// ```
    pub fn parse_with_language(
        input: &str,
        language: MathLanguage,
    ) -> Result<Expression, ParseError> {
        PARSER.with(|parser| parser.borrow_mut().parse_with_language(input, language))
    }

    /// Convert expression to LaTeX format using the comprehensive parser
    ///
    /// Uses thread-local cached parser for optimal performance.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::Expression;
    ///
    /// let expr = Expression::pow(Expression::symbol("x"), Expression::integer(2));
    /// let latex = expr.to_latex();
    /// assert_eq!(latex, "x^{2}");
    ///
    /// let fraction = Expression::rational(1, 2);
    /// let latex_frac = fraction.to_latex();
    /// assert_eq!(latex_frac, "\\frac{1}{2}");
    /// ```
    pub fn to_latex(&self) -> String {
        PARSER.with(|parser| parser.borrow().to_latex(self))
    }

    /// Convert expression to simple mathematical notation
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::Expression;
    ///
    /// let expr = Expression::pow(Expression::symbol("x"), Expression::integer(2));
    /// let simple = expr.to_simple();
    /// assert_eq!(simple, "x^2");
    /// ```
    pub fn to_simple(&self) -> String {
        PARSER.with(|parser| parser.borrow().to_simple(self))
    }

    /// Convert expression to Wolfram Language format
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::Expression;
    ///
    /// let expr = Expression::function("sin", vec![Expression::symbol("x")]);
    /// let wolfram = expr.to_wolfram();
    /// assert_eq!(wolfram, "Sin[x]");
    /// ```
    pub fn to_wolfram(&self) -> String {
        PARSER.with(|parser| parser.borrow().to_wolfram(self))
    }

    /// Detect the mathematical language of input string
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{Expression, parser::universal::MathLanguage};
    ///
    /// assert_eq!(Expression::detect_language("\\frac{1}{2}"), MathLanguage::LaTeX);
    /// assert_eq!(Expression::detect_language("Sin[x]"), MathLanguage::Wolfram);
    /// assert_eq!(Expression::detect_language("x + 1"), MathLanguage::Simple);
    /// ```
    pub fn detect_language(input: &str) -> MathLanguage {
        PARSER.with(|parser| parser.borrow().detect_language(input))
    }
}
