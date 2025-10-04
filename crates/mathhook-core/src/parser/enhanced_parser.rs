//! Enhanced parser with implicit multiplication support
//!
//! This module provides a wrapper around your LALRPOP parser that
//! adds implicit multiplication post-processing.

use crate::core::Expression;
use crate::parser::{
    implicit_multiplication::{ImplicitMultiplicationExt, ImplicitMultiplicationProcessor},
    sequence_detector::TermSequence,
    ParseError,
};

/// Enhanced parser that handles implicit multiplication
pub struct EnhancedMathParser {
    processor: ImplicitMultiplicationProcessor,
}

impl EnhancedMathParser {
    /// Create a new enhanced parser with default implicit multiplication rules
    pub fn new() -> Self {
        Self {
            processor: ImplicitMultiplicationProcessor::new(),
        }
    }

    /// Create a conservative parser (fewer implicit multiplication cases)
    pub fn conservative() -> Self {
        Self {
            processor: ImplicitMultiplicationProcessor::conservative(),
        }
    }

    /// Create with custom processor
    pub fn with_processor(processor: ImplicitMultiplicationProcessor) -> Self {
        Self { processor }
    }

    /// Parse expression with implicit multiplication support
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::parser::enhanced_parser::EnhancedMathParser;
    ///
    /// let parser = EnhancedMathParser::new();
    ///
    /// // These would work after you implement sequence detection in your grammar:
    /// // let result = parser.parse("2x + 3y").unwrap();        // -> 2*x + 3*y
    /// // let result = parser.parse("sin(x)cos(y)").unwrap();   // -> sin(x)*cos(y)
    /// // let result = parser.parse("2(x+1)").unwrap();         // -> 2*(x+1)
    /// ```
    pub fn parse(&self, input: &str) -> Result<Expression, ParseError> {
        // Step 1: Parse with your existing LALRPOP grammar
        let ast = self.parse_with_lalrpop(input)?;

        // Step 2: Post-process to add implicit multiplication
        let result = self.processor.process(ast);

        Ok(result)
    }

    /// Internal method to parse with LALRPOP (you'll implement this)
    fn parse_with_lalrpop(&self, input: &str) -> Result<Expression, ParseError> {
        // This is where you'd call your LALRPOP parser
        // For now, let's use a placeholder

        // You would do something like:
        // use crate::parser::lalrpop::grammar::mathematical;
        // let parser = mathematical::ExpressionParser::new();
        // parser.parse(input).map_err(|e| ParseError::SyntaxError(format!("{:?}", e)))

        // Placeholder implementation
        Err(ParseError::SyntaxError(
            "LALRPOP integration not implemented yet".to_string(),
        ))
    }
}

impl Default for EnhancedMathParser {
    fn default() -> Self {
        Self::new()
    }
}

/// Integration with Expression for convenient parsing
impl Expression {
    /// Parse with implicit multiplication support
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::Expression;
    ///
    /// // After you implement sequence detection:
    /// // let expr = Expression::parse_with_implicit_mul("2x + 3y").unwrap();
    /// // assert_eq!(expr.to_string(), "2 * x + 3 * y");
    /// ```
    pub fn parse_with_enhanced_implicit_mul(input: &str) -> Result<Expression, ParseError> {
        let parser = EnhancedMathParser::new();
        parser.parse(input)
    }

    /// Parse with conservative implicit multiplication
    pub fn parse_with_conservative_implicit_mul(input: &str) -> Result<Expression, ParseError> {
        let parser = EnhancedMathParser::conservative();
        parser.parse(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enhanced_parser_creation() {
        let parser = EnhancedMathParser::new();
        assert!(parser.processor.enable_number_variable);

        let conservative = EnhancedMathParser::conservative();
        assert!(!conservative.processor.enable_variable_variable);
    }

    #[test]
    fn test_implicit_multiplication_patterns() {
        let processor = ImplicitMultiplicationProcessor::new();

        // Test various patterns
        let two = Expression::integer(2);
        let x = Expression::Symbol(crate::core::Symbol::new("x"));
        let y = Expression::Symbol(crate::core::Symbol::new("y"));

        // 2x should be implicit multiplication
        assert!(processor.should_insert_implicit_multiplication(&two, &x));

        // xy should be implicit multiplication
        assert!(processor.should_insert_implicit_multiplication(&x, &y));

        // 2 + x should NOT be implicit multiplication (different operator)
        // This would be handled by the grammar structure
    }
}
