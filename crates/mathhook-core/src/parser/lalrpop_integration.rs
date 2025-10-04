//! Integration between enhanced lexer and LALRPOP parser
//!
//! This module provides the bridge between the implicit multiplication
//! lexer and your LALRPOP grammar.
use crate::parser::lalrpop::grammar::mathematical;

use crate::core::Expression;
use crate::parser::{
    lalrpop::lexer::tokens::Token,
    lexer_with_implicit_mul::{EnhancedToken, ImplicitMultiplicationLexer},
    ParseError,
};

/// LALRPOP-compatible parser with implicit multiplication
pub struct LALRPOPWithImplicitMul;

impl LALRPOPWithImplicitMul {
    /// Parse expression using enhanced lexer + LALRPOP grammar
    pub fn parse(input: &str) -> Result<Expression, ParseError> {
        // Step 1: Enhanced tokenization with implicit multiplication detection
        let mut enhanced_lexer = ImplicitMultiplicationLexer::new(input);
        let enhanced_tokens: Vec<_> =
            std::iter::from_fn(|| enhanced_lexer.next_enhanced_token()).collect();

        // Step 2: Convert enhanced tokens to LALRPOP-compatible format
        let lalrpop_tokens = Self::convert_to_lalrpop_tokens(enhanced_tokens);

        // Step 3: Parse with your LALRPOP grammar
        Self::parse_with_lalrpop(lalrpop_tokens)
    }

    /// Convert enhanced tokens to LALRPOP format
    fn convert_to_lalrpop_tokens(
        enhanced_tokens: Vec<(usize, EnhancedToken, usize)>,
    ) -> Vec<(usize, Token, usize)> {
        enhanced_tokens
            .into_iter()
            .map(|(start, enhanced_token, end)| {
                match enhanced_token {
                    EnhancedToken::Regular(token) => (start, token, end),
                    EnhancedToken::ImplicitMultiply => {
                        // Convert implicit multiply to explicit multiply token
                        (start, Token::Star, end)
                    }
                }
            })
            .collect()
    }

    /// Parse tokens with LALRPOP grammar
    fn parse_with_lalrpop(tokens: Vec<(usize, Token, usize)>) -> Result<Expression, ParseError> {
        // Convert tokens back to string format for LALRPOP
        let input_string = Self::tokens_to_input_string(&tokens);

        // Parse with LALRPOP grammar
        use crate::parser::lalrpop::grammar::mathematical;
        let parser = mathematical::ExpressionParser::new();

        parser
            .parse(&input_string)
            .map_err(|e| ParseError::SyntaxError(format!("LALRPOP parse error: {:?}", e)))
    }

    /// Convert tokens back to input string
    fn tokens_to_input_string(tokens: &[(usize, Token, usize)]) -> String {
        let mut result = String::new();

        for (i, (_, token, _)) in tokens.iter().enumerate() {
            if i > 0 {
                result.push(' ');
            }

            let token_str = match token {
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
                _ => "?", // Fallback
            };

            result.push_str(token_str);
        }

        result
    }
}

/// Convenient parsing functions
impl Expression {
    /// Parse with automatic implicit multiplication detection
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::Expression;
    ///
    /// // After full implementation:
    /// // let expr = Expression::parse_smart("2x + 3y").unwrap();
    /// // assert_eq!(expr.to_string(), "2 * x + 3 * y");
    /// //
    /// // let expr = Expression::parse_smart("sin(x)cos(y)").unwrap();  
    /// // assert_eq!(expr.to_string(), "sin(x) * cos(y)");
    /// //
    /// // let expr = Expression::parse_smart("2(x+1)").unwrap();
    /// // assert_eq!(expr.to_string(), "2 * (x + 1)");
    /// ```
    pub fn parse_smart(input: &str) -> Result<Expression, ParseError> {
        LALRPOPWithImplicitMul::parse(input)
    }
}

/// Example of how to integrate with your existing LALRPOP parser
///
/// You would modify your existing parser integration like this:
///
/// ```rust
/// // In your existing parser module
/// use crate::parser::lalrpop::grammar::mathematical;
///
/// impl LALRPOPWithImplicitMul {
///     fn parse_with_lalrpop(tokens: Vec<(usize, Token, usize)>) -> Result<Expression, ParseError> {
///         let parser = mathematical::ExpressionParser::new();
///         
///         // Convert tokens to iterator
///         let token_iter = tokens.into_iter().map(|t| Ok(t));
///         
///         // Parse with LALRPOP
///         parser.parse(token_iter)
///             .map_err(|e| ParseError::SyntaxError(format!("Parse error: {:?}", e)))
///     }
/// }
/// ```

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_conversion() {
        let enhanced_tokens = vec![
            (0, EnhancedToken::Regular(Token::Number("2")), 1),
            (1, EnhancedToken::ImplicitMultiply, 1),
            (1, EnhancedToken::Regular(Token::Identifier("x")), 2),
        ];

        let lalrpop_tokens = LALRPOPWithImplicitMul::convert_to_lalrpop_tokens(enhanced_tokens);

        assert_eq!(lalrpop_tokens.len(), 3);
        assert!(matches!(lalrpop_tokens[0].1, Token::Number(_)));
        assert!(matches!(lalrpop_tokens[1].1, Token::Star)); // Implicit multiply becomes explicit
        assert!(matches!(lalrpop_tokens[2].1, Token::Identifier(_)));
    }

    #[test]
    fn test_integration_placeholder() {
        // This test will work once you implement the LALRPOP integration
        let result = LALRPOPWithImplicitMul::parse("x + y");
        assert!(result.is_err()); // Expected since integration is not complete
    }
}
