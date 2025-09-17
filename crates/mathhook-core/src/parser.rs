//! Multi-format mathematical expression parser for MathHook
//!
//! This crate provides parsing capabilities for various mathematical formats
//! including LaTeX, Wolfram Language, and standard mathematical notation.
pub mod cache;
pub mod config;
pub mod constants;
pub mod error;
pub mod grammar;
pub mod lexer;
use crate::core::Expression;
pub use cache::*;
use config::ParserConfig;
pub use constants::*;
use error::ParseError;
/// Fully integrated mathematical expression parser
pub struct Parser {
    enable_implicit_multiplication: bool,
}
impl Parser {
    /// Create parser with implicit multiplication enabled
    pub fn new(config: &ParserConfig) -> Self {
        Self {
            enable_implicit_multiplication: config.enable_implicit_multiplication,
        }
    }
    /// Parse mathematical expression with full integration
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::parser::Parser;
    /// use mathhook_core::parser::config::ParserConfig;
    ///
    /// let parser = Parser::new(&ParserConfig::default());
    ///
    /// // These will work with implicit multiplication:
    /// let expr = parser.parse("2x").unwrap();           // -> 2 * x
    /// let expr = parser.parse("xy").unwrap();           // -> x * y
    /// let expr = parser.parse("2(x+1)").unwrap();       // -> 2 * (x + 1)
    ///
    /// // These work normally:
    /// let expr = parser.parse("x + y").unwrap();        // -> x + y
    /// let expr = parser.parse("x^2").unwrap();          // -> x^2
    /// ```
    pub fn parse(&self, input: &str) -> Result<Expression, ParseError> {
        if self.enable_implicit_multiplication {
            self.parse_with_implicit_multiplication(input)
        } else {
            self.parse_explicit_only(input)
        }
    }
    /// Parse with implicit multiplication enabled using comprehensive processor
    fn parse_with_implicit_multiplication(&self, input: &str) -> Result<Expression, ParseError> {
        let enhanced_input =
            lexer::ImplicitMultiplicationProcessor::insert_implicit_multiplication(input);
        let parser = grammar::ExpressionParser::new();
        parser
            .parse(&enhanced_input)
            .map_err(|e| ParseError::SyntaxError(format!("LALRPOP parse error: {:?}", e)))
    }
    /// Parse with explicit operators only (no implicit multiplication)
    fn parse_explicit_only(&self, input: &str) -> Result<Expression, ParseError> {
        println!("Parsing with explicit operators only: {}", input);
        let parser = grammar::ExpressionParser::new();
        parser
            .parse(input)
            .map_err(|e| ParseError::SyntaxError(format!("LALRPOP parse error: {:?}", e)))
    }
}
impl Default for Parser {
    fn default() -> Self {
        Self::new(&ParserConfig::default())
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parser_modes() {
        let config_implicit = ParserConfig {
            enable_implicit_multiplication: true,
        };
        let parser_implicit = Parser::new(&config_implicit);
        let config_explicit = ParserConfig {
            enable_implicit_multiplication: false,
        };
        let parser_explicit = Parser::new(&config_explicit);
        assert!(parser_implicit.parse("x + y").is_ok());
        assert!(parser_explicit.parse("x + y").is_ok());
    }
    #[test]
    fn test_implicit_multiplication_integration() {
        let config = ParserConfig {
            enable_implicit_multiplication: true,
        };
        let parser = Parser::new(&config);
        println!("Testing: 2*x -> 2 * x");
        assert!(parser.parse("2x").is_ok());
        assert!(parser.parse("xy").is_ok());
        assert!(parser.parse("2pi").is_ok());
        assert!(parser.parse("2+3").is_ok());
        assert!(parser.parse("x*y").is_ok());
        assert!(parser.parse("a/b").is_ok());
    }
}
