//! Universal mathematical expression parser
//!
//! Provides format-aware parsing with automatic language detection for LaTeX,
//! Wolfram Language, and simple mathematical notation.

use crate::core::Expression;
use crate::parser::latex::LaTeXContext;
use crate::parser::simple::SimpleParser;
use crate::parser::wolfram::WolframContext;
use crate::parser::wolfram::WolframParser;
use crate::parser::ParseError;
use serde::{Deserialize, Serialize};

/// Supported mathematical notation languages
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum MathLanguage {
    Simple,  // Basic mathematical notation
    LaTeX,   // LaTeX mathematical notation
    Wolfram, // Wolfram Language (Mathematica)
    Auto,    // Auto-detect
}

/// Universal parser that handles multiple mathematical notation languages
pub struct UniversalParser {}

impl UniversalParser {
    /// Create new universal parser
    pub fn new() -> Self {
        Self {}
    }

    /// Main parse method with automatic language detection
    /// # Examples
    /// ```rust
    /// use mathhook_core::{Expression, parser::universal::MathLanguage};
    ///
    /// let expr = Expression::parse("x + 1").unwrap();
    /// assert_eq!(expr, Expression::add(vec![Expression::symbol("x"), Expression::integer(1)]));
    /// ```
    pub fn parse(&mut self, input: &str) -> Result<Expression, ParseError> {
        let language = self.detect_language(input);
        self.parse_with_language(input, language)
    }

    /// Parse with explicit language specification
    /// # Examples
    /// ```rust
    /// use mathhook_core::{Expression, parser::universal::MathLanguage};
    ///
    /// let expr = Expression::parse("x + 1").unwrap();
    /// let language = MathLanguage::Simple;
    /// let parsed = parser.parse_with_language(&expr.to_string(), language);
    /// assert_eq!(parsed, Ok(expr));
    /// ```
    pub fn parse_with_language(
        &mut self,
        input: &str,
        language: MathLanguage,
    ) -> Result<Expression, ParseError> {
        match language {
            MathLanguage::Simple => self.parse_simple(input),
            MathLanguage::LaTeX => self.parse_latex(input),
            MathLanguage::Wolfram => self.parse_wolfram(input),
            MathLanguage::Auto => {
                let detected = self.detect_language(input);
                self.parse_with_language(input, detected)
            }
        }
    }

    /// Detect mathematical notation language from input
    /// # Examples
    /// ```rust
    /// use mathhook_core::{Expression, parser::universal::MathLanguage};
    ///
    /// let expr = Expression::parse("x + 1").unwrap();
    /// let language = parser.detect_language(&expr.to_string());
    /// assert_eq!(language, MathLanguage::Simple);
    /// ```
    pub fn detect_language(&self, input: &str) -> MathLanguage {
        use crate::parser::constants::*;

        let latex_score = LATEX_DETECTION_PATTERNS
            .iter()
            .map(|&indicator| input.matches(indicator).count())
            .sum::<usize>();

        let wolfram_score = WOLFRAM_DETECTION_PATTERNS
            .iter()
            .map(|&indicator| input.matches(indicator).count())
            .sum::<usize>();

        // Special case: Wolfram set vs LaTeX set detection
        if input.trim().starts_with('{') && !input.contains('\\') {
            return MathLanguage::Wolfram; // Pure {1,2,3} is Wolfram
        }

        if latex_score > 0 && latex_score > wolfram_score {
            MathLanguage::LaTeX
        } else if wolfram_score > 0 && wolfram_score > latex_score {
            MathLanguage::Wolfram
        } else {
            // No special patterns detected - simple mathematical notation
            MathLanguage::Simple
        }
    }

    /// Parse simple mathematical expression - delegate to SimpleParser
    fn parse_simple(&mut self, input: &str) -> Result<Expression, ParseError> {
        let mut simple_parser = SimpleParser::new();
        simple_parser.parse(input)
    }

    /// Parse LaTeX mathematical expression - delegate to LaTeXParser
    fn parse_latex(&mut self, latex: &str) -> Result<Expression, ParseError> {
        let mut latex_parser = crate::parser::latex::LaTeXParser::new();
        latex_parser.parse(latex)
    }

    /// Parse Wolfram Language mathematical expression - delegate to WolframParser
    fn parse_wolfram(&mut self, wolfram: &str) -> Result<Expression, ParseError> {
        let mut wolfram_parser = WolframParser::new();
        wolfram_parser.parse(wolfram)
    }

    /// Generate simple mathematical notation (no LaTeX commands)
    /// # Examples
    /// ```rust
    /// use mathhook_core::{Expression, parser::universal::MathLanguage};
    ///
    /// let expr = Expression::parse("x + 1").unwrap();
    /// let simple = parser.to_format(&expr, MathLanguage::Simple);
    /// assert_eq!(simple, "x + 1");
    /// ```
    pub fn to_simple(&self, expr: &Expression) -> String {
        let simple_parser = SimpleParser::new();
        simple_parser.format(expr)
    }

    /// Generate LaTeX notation with commands
    /// # Examples
    /// ```rust
    /// use mathhook_core::{Expression, parser::universal::MathLanguage};
    ///
    /// let expr = Expression::parse("x + 1").unwrap();
    /// let latex = parser.to_format(&expr, MathLanguage::LaTeX);
    /// assert_eq!(latex, "\\frac{x + 1}{2}");
    /// ```
    pub fn to_latex(&self, expr: &Expression) -> String {
        let latex_parser = crate::parser::latex::LaTeXParser::new();
        let context = LaTeXContext::default();
        latex_parser.format(expr, &context)
    }

    /// Generate Wolfram Language notation
    /// # Examples
    /// ```rust
    /// use mathhook_core::{Expression, parser::universal::MathLanguage};
    ///
    /// let expr = Expression::parse("x + 1").unwrap();
    /// let wolfram = parser.to_format(&expr, MathLanguage::Wolfram);
    /// assert_eq!(wolfram, "Plus[x, 1]");
    /// ```
    pub fn to_wolfram(&self, expr: &Expression) -> String {
        let wolfram_parser = WolframParser::new();
        let context = WolframContext::default();
        wolfram_parser.format(expr, &context)
    }

    /// Generate output in the same format as the detected input language
    /// # Examples
    /// ```rust
    /// use mathhook_core::{Expression, parser::universal::MathLanguage};
    ///
    /// let expr = Expression::parse("x + 1").unwrap();
    /// let simple = parser.to_format(&expr, MathLanguage::Simple);
    /// assert_eq!(simple, "x + 1");
    /// ```
    pub fn to_format(&self, expr: &Expression, format: MathLanguage) -> String {
        match format {
            MathLanguage::Simple => self.to_simple(expr),
            MathLanguage::LaTeX => self.to_latex(expr),
            MathLanguage::Wolfram => self.to_wolfram(expr),
            MathLanguage::Auto => self.to_simple(expr), // Default to simple for auto
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::Symbol;

    #[test]
    fn test_language_detection() {
        let parser = UniversalParser::new();

        // LaTeX detection
        assert_eq!(parser.detect_language("\\frac{1}{2}"), MathLanguage::LaTeX);
        assert_eq!(
            parser.detect_language("\\sin(x) + \\cos(y)"),
            MathLanguage::LaTeX
        );
        assert_eq!(parser.detect_language("\\int x dx"), MathLanguage::LaTeX);

        // Wolfram detection
        assert_eq!(
            parser.detect_language("Sin[x] + Cos[y]"),
            MathLanguage::Wolfram
        );
        assert_eq!(
            parser.detect_language("Integrate[x, x]"),
            MathLanguage::Wolfram
        );
        assert_eq!(parser.detect_language("Power[x, 2]"), MathLanguage::Wolfram);

        // Ambiguous cases default to Simple (no special patterns detected)
        assert_eq!(parser.detect_language("x + y"), MathLanguage::Simple);
    }

    #[test]
    fn test_basic_latex_parsing() {
        let mut parser = UniversalParser::new();

        let result = parser.parse_latex("x + 1");
        assert!(result.is_ok());

        let result = parser.parse_latex("\\frac{1}{2}");
        assert!(result.is_ok());
    }

    #[test]
    fn test_basic_wolfram_parsing() {
        let mut parser = UniversalParser::new();

        let result = parser.parse_wolfram("x + 1");
        assert!(result.is_ok());

        let result = parser.parse_wolfram("Sin[x]");
        assert!(result.is_ok());
    }

    #[test]
    fn test_latex_output() {
        let parser = UniversalParser::new();

        let expr = Expression::add(vec![
            Expression::symbol(Symbol::new("x")),
            Expression::integer(1),
        ]);

        let latex = parser.to_latex(&expr);
        assert!(latex.contains("x"));
        assert!(latex.contains("1"));
    }

    #[test]
    fn test_wolfram_output() {
        let parser = UniversalParser::new();

        let expr = Expression::add(vec![
            Expression::symbol(Symbol::new("x")),
            Expression::integer(1),
        ]);

        let wolfram = parser.to_wolfram(&expr);
        assert!(wolfram.contains("Plus"));
        assert!(wolfram.contains("x"));
        assert!(wolfram.contains("1"));
    }
}
