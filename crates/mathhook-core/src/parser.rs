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
pub use config::*;
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
        // Step 1: Simple string preprocessing to insert * where needed
        let enhanced_input = self.insert_implicit_multiplication(input);

        // Step 2: Parse with LALRPOP grammar (let LALRPOP handle all tokenization)
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

    /// Insert implicit multiplication operators in string format
    fn insert_implicit_multiplication(&self, input: &str) -> String {
        let mut result = String::new();
        let chars: Vec<char> = input.chars().collect();
        let mut i = 0;

        while i < chars.len() {
            let current = chars[i];
            result.push(current);

            // Look ahead to see if we should insert implicit multiplication
            if i + 1 < chars.len() {
                let next = chars[i + 1];

                // Insert * between number and letter: 2x -> 2*x
                if current.is_ascii_digit() && (next.is_alphabetic() || next == '\\') {
                    // Check if next char starts a LaTeX command or is a simple variable
                    if !self.is_followed_by_operator(&chars, i + 1) {
                        result.push('*');
                    }
                }
                // Insert * between letter and letter: xy -> x*y (but not in LaTeX commands)
                else if current.is_alphabetic()
                    && next.is_alphabetic()
                    && !self.is_in_latex_command(&chars, i)
                {
                    if !self.is_followed_by_operator(&chars, i + 1) {
                        result.push('*');
                    }
                }
                // Insert * between ) and letter/number: (x)y -> (x)*y
                else if current == ')' && (next.is_alphanumeric() || next == '\\') {
                    if !self.is_followed_by_operator(&chars, i + 1) {
                        result.push('*');
                    }
                }
                // Insert * between letter/number and (: x( -> x*(
                else if (current.is_alphanumeric()) && next == '(' {
                    // Don't insert if it's a function call
                    if !self.is_function_call(&chars, i) {
                        result.push('*');
                    }
                }
            }

            i += 1;
        }

        result
    }

    /// Check if position is followed by an operator
    fn is_followed_by_operator(&self, chars: &[char], pos: usize) -> bool {
        if pos >= chars.len() {
            return false;
        }

        // Skip whitespace
        let mut i = pos;
        while i < chars.len() && chars[i].is_whitespace() {
            i += 1;
        }

        if i >= chars.len() {
            return false;
        }

        matches!(
            chars[i],
            '+' | '-' | '*' | '/' | '^' | '=' | '<' | '>' | '!' | ',' | ';'
        )
    }

    /// Check if position is within a LaTeX command
    fn is_in_latex_command(&self, chars: &[char], pos: usize) -> bool {
        // Look backwards to see if we're after a backslash
        if pos == 0 {
            return false;
        }

        let mut i = pos;
        while i > 0 {
            i -= 1;
            if chars[i] == '\\' {
                return true;
            }
            if !chars[i].is_alphabetic() {
                break;
            }
        }
        false
    }

    /// Check if this looks like a function call
    fn is_function_call(&self, chars: &[char], pos: usize) -> bool {
        // Extract the identifier before the parenthesis
        if pos == 0 {
            return false;
        }

        let mut start = pos;
        while start > 0 && chars[start - 1].is_alphabetic() {
            start -= 1;
        }

        if start == pos {
            return false;
        }

        let identifier: String = chars[start..=pos].iter().collect();

        // Check if it's a known function
        matches!(
            identifier.as_str(),
            "sin"
                | "cos"
                | "tan"
                | "sec"
                | "csc"
                | "cot"
                | "sinh"
                | "cosh"
                | "tanh"
                | "sech"
                | "csch"
                | "coth"
                | "arcsin"
                | "arccos"
                | "arctan"
                | "arcsec"
                | "arccsc"
                | "arccot"
                | "asin"
                | "acos"
                | "atan"
                | "asec"
                | "acsc"
                | "acot"
                | "log"
                | "ln"
                | "exp"
                | "sqrt"
                | "abs"
                | "floor"
                | "ceil"
                | "round"
                | "sign"
                | "max"
                | "min"
                | "gcd"
                | "lcm"
        )
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
