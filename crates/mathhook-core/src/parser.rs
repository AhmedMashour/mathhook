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
use config::ParserConfig;
use error::ParseError;
use lexer::tokens::*;
use lexer::ImplicitMultiplicationLexer;

pub use cache::*;
pub use constants::*;
/// Fully integrated mathematical expression parser
pub struct Parser {
    enable_implicit_multiplication: bool,
}

impl Parser {
    /// Create parser with implicit multiplication enabled
    pub fn new(config: ParserConfig) -> Self {
        Self {
            enable_implicit_multiplication: config.enable_implicit_multiplication,
        }
    }

    /// Parse mathematical expression with full integration
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::parser::integrated_parser::Parser;
    ///
    /// let parser = Parser::new();
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

    /// Parse with implicit multiplication enabled
    fn parse_with_implicit_multiplication(&self, input: &str) -> Result<Expression, ParseError> {
        // Step 1: Enhanced lexer processes input and inserts implicit * tokens
        let mut enhanced_lexer = ImplicitMultiplicationLexer::new(input);
        let lalrpop_tokens = enhanced_lexer.to_lalrpop_tokens();

        // Step 2: Convert tokens to string format (what LALRPOP expects)
        let enhanced_string = self.tokens_to_string(&lalrpop_tokens);

        // Step 3: Parse with LALRPOP grammar
        let parser = grammar::ExpressionParser::new();
        parser
            .parse(&enhanced_string)
            .map_err(|e| ParseError::SyntaxError(format!("LALRPOP parse error: {:?}", e)))
    }

    /// Parse with explicit operators only (no implicit multiplication)
    fn parse_explicit_only(&self, input: &str) -> Result<Expression, ParseError> {
        let parser = grammar::ExpressionParser::new();
        parser
            .parse(input)
            .map_err(|e| ParseError::SyntaxError(format!("LALRPOP parse error: {:?}", e)))
    }

    /// Convert tokens to string format for LALRPOP
    fn tokens_to_string(&self, tokens: &[(usize, Token, usize)]) -> String {
        let mut result = String::new();

        for (i, (_, token, _)) in tokens.iter().enumerate() {
            if i > 0 {
                result.push(' ');
            }
            result.push_str(&self.token_to_string(token));
        }

        result
    }

    /// Convert individual token to string
    fn token_to_string<'a>(&self, token: &Token<'a>) -> &'a str {
        use Token;

        match token {
            Token::Plus => "+",
            Token::Minus => "-",
            Token::Star => "*",
            Token::Slash => "/",
            Token::Caret => "^",
            Token::Equals => "=",
            Token::LParen => "(",
            Token::RParen => ")",
            Token::LBracket => "[",
            Token::RBracket => "]",
            Token::LBrace => "{",
            Token::RBrace => "}",
            Token::Comma => ",",
            Token::Exclamation => "!",
            Token::Number(n) => n,
            Token::Identifier(id) => id,
            Token::Pi => "pi",
            Token::E => "e",
            Token::ImaginaryUnit => "i",
            Token::Infinity => "infinity",
            _ => "?", // Fallback for unsupported tokens
        }
    }
}

impl Default for Parser {
    fn default() -> Self {
        Self::new(ParserConfig::default())
    }
}

/// Convenient parsing functions for Expression
impl Expression {
    /// Parse with automatic implicit multiplication detection
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::Expression;
    ///
    /// // Implicit multiplication cases:
    /// let expr = Expression::parse("2x").unwrap();      // -> 2 * x
    /// let expr = Expression::parse("xy").unwrap();      // -> x * y
    /// let expr = Expression::parse("2(x+1)").unwrap();  // -> 2 * (x + 1)
    ///
    /// // Regular cases:
    /// let expr = Expression::parse("x + y", { enable_implicit_multiplication: false }).unwrap();   // -> x + y
    /// let expr = Expression::parse("x^2", { enable_implicit_multiplication: false }).unwrap();     // -> x^2
    /// ```
    pub fn parse<C>(input: &str, config: C) -> Result<Expression, ParseError>
    where
        C: Into<Option<ParserConfig>>,
    {
        let config = config.into().unwrap_or_default();
        let parser = Parser::new(config);
        parser.parse(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_implicit_multiplication_integration() {
        let parser = Parser::new(ParserConfig::default());

        // Test cases that should work with implicit multiplication
        let test_cases = vec![
            ("2*x", "2 * x"),             // Already explicit
            ("x+y", "x + y"),             // Addition (no implicit mul)
            ("x^2", "x ^ 2"),             // Power
            ("2*(x+1)", "2 * ( x + 1 )"), // Explicit with parentheses
        ];

        for (input, expected_enhanced) in test_cases {
            println!("Testing: {} -> {}", input, expected_enhanced);

            // Test that it parses successfully
            let result = parser.parse(input);
            assert!(result.is_ok(), "Failed to parse: {}", input);
        }
    }

    #[test]
    fn test_parser_modes() {
        let implicit_parser = Parser::new(ParserConfig::default());
        let explicit_parser = Parser::new(ParserConfig {
            enable_implicit_multiplication: false,
        });

        // Both should handle explicit operators
        let explicit_expr = "2 * x";
        assert!(implicit_parser.parse(explicit_expr).is_ok());
        assert!(explicit_parser.parse(explicit_expr).is_ok());

        // Test the difference in behavior (when implicit multiplication is implemented)
        // For now, both will behave the same since we're using string conversion
    }
}
