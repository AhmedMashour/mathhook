//! Enhanced lexer with implicit multiplication detection
//!
//! This lexer extends the standard mathematical lexer to detect
//! adjacent terms that should be implicitly multiplied and inserts
//! multiplication tokens automatically.
//!
use super::single_char::SingleCharVariableLexer;
use super::tokens::Token;
use std::collections::VecDeque;

/// Enhanced token that includes implicit multiplication markers
#[derive(Debug, Clone, PartialEq)]
pub enum EnhancedToken<'input> {
    /// Regular token from the base lexer
    Regular(Token<'input>),
    /// Implicit multiplication token (inserted by lexer)
    ImplicitMultiply,
}

/// Enhanced lexer that detects and inserts implicit multiplication
pub struct ImplicitMultiplicationLexer<'input> {
    base_lexer: SingleCharVariableLexer<'input>,
    token_buffer: VecDeque<(usize, EnhancedToken<'input>, usize)>,
    last_token: Option<Token<'input>>,
    enable_implicit_mul: bool,
}

impl<'input> ImplicitMultiplicationLexer<'input> {
    /// Create a new enhanced lexer
    pub fn new(input: &'input str) -> Self {
        Self {
            base_lexer: SingleCharVariableLexer::new(input),
            token_buffer: VecDeque::new(),
            last_token: None,
            enable_implicit_mul: true,
        }
    }

    /// Create lexer with implicit multiplication disabled
    pub fn without_implicit_mul(input: &'input str) -> Self {
        Self {
            base_lexer: SingleCharVariableLexer::new(input),
            token_buffer: VecDeque::new(),
            last_token: None,
            enable_implicit_mul: false,
        }
    }

    /// Get the next enhanced token
    pub fn next_enhanced_token(&mut self) -> Option<(usize, EnhancedToken<'input>, usize)> {
        // Return buffered tokens first
        if let Some(token) = self.token_buffer.pop_front() {
            return Some(token);
        }

        // Get next token from base lexer
        if let Some((start, token, end)) = self.base_lexer.next_token() {
            // Check if we should insert implicit multiplication before this token
            if self.enable_implicit_mul && self.should_insert_implicit_mul_before(&token) {
                // Buffer the implicit multiply token
                self.token_buffer
                    .push_back((start, EnhancedToken::ImplicitMultiply, start));
                // Buffer the current token
                self.token_buffer
                    .push_back((start, EnhancedToken::Regular(token.clone()), end));

                // Return the implicit multiply token first
                self.last_token = Some(token);
                return self.token_buffer.pop_front();
            }

            self.last_token = Some(token.clone());
            Some((start, EnhancedToken::Regular(token), end))
        } else {
            None
        }
    }

    /// Determine if implicit multiplication should be inserted before this token
    fn should_insert_implicit_mul_before(&self, current_token: &Token<'input>) -> bool {
        if let Some(ref last) = self.last_token {
            self.should_insert_between(last, current_token)
        } else {
            false
        }
    }

    /// Check if implicit multiplication should be inserted between two tokens
    fn should_insert_between(&self, left: &Token<'input>, right: &Token<'input>) -> bool {
        use Token::*;

        match (left, right) {
            // Number followed by identifier: 2x -> 2*x
            (Number(_), Identifier(_)) => true,

            // Number followed by function: 2sin -> 2*sin
            (Number(_), Identifier(name)) if self.is_function_name(name) => true,

            // Identifier followed by identifier: xy -> x*y
            (Identifier(_), Identifier(_)) => true,

            // Number followed by parentheses: 2(x) -> 2*(x)
            (Number(_), LParen) => true,

            // Identifier followed by parentheses: x(y) -> x*(y) (unless it's a function call)
            (Identifier(name), LParen) if !self.is_function_name(name) => true,

            // Closing parentheses followed by identifier: (x)y -> (x)*y
            (RParen, Identifier(_)) => true,

            // Closing parentheses followed by opening parentheses: (x)(y) -> (x)*(y)
            (RParen, LParen) => true,

            // Function call followed by identifier: sin(x)y -> sin(x)*y
            (RParen, Identifier(_)) if self.was_function_call() => true,

            // Constants followed by variables: πx -> π*x
            (Pi | E | ImaginaryUnit, Identifier(_)) => true,

            // Variables followed by constants: xπ -> x*π
            (Identifier(_), Pi | E | ImaginaryUnit) => true,

            // Power followed by identifier: x^2 y -> x^2 * y (handled by precedence)
            // Note: This is tricky because x^2y could be x^(2y) or (x^2)*y
            // We'll be conservative and not insert here
            _ => false,
        }
    }

    /// Check if an identifier is a known function name
    fn is_function_name(&self, name: &str) -> bool {
        // Common mathematical functions
        matches!(
            name,
            "sin" | "cos" | "tan" | "sec" | "csc" | "cot" |
            "sinh" | "cosh" | "tanh" | "sech" | "csch" | "coth" |
            "arcsin" | "arccos" | "arctan" | "arcsec" | "arccsc" | "arccot" |
            "asin" | "acos" | "atan" | "asec" | "acsc" | "acot" |
            "log" | "ln" | "exp" | "sqrt" | "abs" |
            "floor" | "ceil" | "round" | "sign" |
            "max" | "min" | "gcd" | "lcm" |
            "gamma" | "beta" | "zeta" | "erf" | "erfc" |
            "J" | "Y" | "I" | "K" | // Bessel functions
            "P" | "Q" | "L" | "H" | // Legendre, Hermite functions
            "F" | "G" | "U" | "M" | "W" // Hypergeometric, Whittaker functions
        )
    }

    /// Check if the last token sequence was a function call
    fn was_function_call(&self) -> bool {
        // This would require more sophisticated state tracking
        // For now, we'll use a simple heuristic
        false // TODO: Implement function call detection
    }

    /// Convert enhanced tokens back to regular tokens for LALRPOP
    pub fn to_lalrpop_tokens(&mut self) -> Vec<(usize, Token<'input>, usize)> {
        let mut tokens = Vec::new();

        while let Some((start, enhanced_token, end)) = self.next_enhanced_token() {
            match enhanced_token {
                EnhancedToken::Regular(token) => {
                    tokens.push((start, token, end));
                }
                EnhancedToken::ImplicitMultiply => {
                    // Convert implicit multiply to explicit multiply token
                    tokens.push((start, Token::Star, end));
                }
            }
        }

        tokens
    }
}

/// Integration with your LALRPOP parser
pub struct ImplicitMultiplicationParser {
    enable_implicit_mul: bool,
}

impl ImplicitMultiplicationParser {
    pub fn new() -> Self {
        Self {
            enable_implicit_mul: true,
        }
    }

    pub fn without_implicit_mul() -> Self {
        Self {
            enable_implicit_mul: false,
        }
    }

    /// Parse with implicit multiplication detection
    pub fn parse(&self, input: &str) -> Result<crate::core::Expression, crate::parser::ParseError> {
        // Step 1: Enhanced tokenization with implicit multiplication detection
        let mut enhanced_lexer = if self.enable_implicit_mul {
            ImplicitMultiplicationLexer::new(input)
        } else {
            ImplicitMultiplicationLexer::without_implicit_mul(input)
        };

        // Step 2: Convert to LALRPOP tokens
        let tokens = enhanced_lexer.to_lalrpop_tokens();

        // Step 3: Parse with your LALRPOP grammar
        self.parse_tokens(tokens)
    }

    /// Parse tokens with LALRPOP grammar (you'll implement this)
    fn parse_tokens<'input>(
        &self,
        _tokens: Vec<(usize, Token<'input>, usize)>,
    ) -> Result<crate::core::Expression, crate::parser::ParseError> {
        // This is where you'd integrate with your LALRPOP parser
        //
        // You would do something like:
        // use crate::parser::lalrpop::grammar::mathematical;
        // let parser = mathematical::ExpressionParser::new();
        //
        // // Convert tokens to LALRPOP format and parse
        // let lalrpop_tokens = self.convert_to_lalrpop_format(tokens);
        // parser.parse(lalrpop_tokens).map_err(|e| ParseError::SyntaxError(format!("{:?}", e)))

        // Placeholder for now
        Err(crate::parser::ParseError::SyntaxError(
            "Token parsing not implemented yet".to_string(),
        ))
    }
}

impl Default for ImplicitMultiplicationParser {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_implicit_multiplication_detection() {
        let mut lexer = ImplicitMultiplicationLexer::new("2x");

        // Should generate: Number(2), ImplicitMultiply, Identifier(x)
        let tokens: Vec<_> = std::iter::from_fn(|| lexer.next_enhanced_token()).collect();

        assert_eq!(tokens.len(), 3);
        assert!(matches!(
            tokens[0].1,
            EnhancedToken::Regular(Token::Number(_))
        ));
        assert!(matches!(tokens[1].1, EnhancedToken::ImplicitMultiply));
        assert!(matches!(
            tokens[2].1,
            EnhancedToken::Regular(Token::Identifier(_))
        ));
    }

    #[test]
    fn test_function_detection() {
        let lexer = ImplicitMultiplicationLexer::new("");

        assert!(lexer.is_function_name("sin"));
        assert!(lexer.is_function_name("cos"));
        assert!(lexer.is_function_name("log"));
        assert!(!lexer.is_function_name("x"));
        assert!(!lexer.is_function_name("variable"));
    }

    #[test]
    fn test_no_implicit_mul_between_operators() {
        let mut lexer = ImplicitMultiplicationLexer::new("x+y");

        let tokens: Vec<_> = std::iter::from_fn(|| lexer.next_enhanced_token()).collect();

        // Should NOT insert implicit multiplication between x and +
        assert_eq!(tokens.len(), 3); // x, +, y (no implicit multiply)
        assert!(matches!(
            tokens[0].1,
            EnhancedToken::Regular(Token::Identifier(_))
        ));
        assert!(matches!(tokens[1].1, EnhancedToken::Regular(Token::Plus)));
        assert!(matches!(
            tokens[2].1,
            EnhancedToken::Regular(Token::Identifier(_))
        ));
    }

    #[test]
    fn test_parentheses_implicit_mul() {
        let mut lexer = ImplicitMultiplicationLexer::new("2(x+1)");

        let tokens: Vec<_> = std::iter::from_fn(|| lexer.next_enhanced_token()).collect();

        // Should generate: Number(2), ImplicitMultiply, LParen, Identifier(x), Plus, Number(1), RParen
        assert!(tokens.len() >= 3);
        assert!(matches!(
            tokens[0].1,
            EnhancedToken::Regular(Token::Number(_))
        ));
        assert!(matches!(tokens[1].1, EnhancedToken::ImplicitMultiply));
        assert!(matches!(tokens[2].1, EnhancedToken::Regular(Token::LParen)));
    }
}
