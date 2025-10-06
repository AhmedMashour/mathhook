//! Enhanced lexer with implicit multiplication detection
//!
//! This lexer extends the standard mathematical lexer to detect
//! adjacent terms that should be implicitly multiplied and inserts
//! multiplication tokens automatically.
//!
use super::single_char::SingleCharVariableLexer;
use super::tokens::Token;
use std::collections::{HashSet, VecDeque};
use std::sync::LazyLock;

/// Categories of mathematical tokens for implicit multiplication logic
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum TokenCategory {
    Number,
    Identifier,
    Constant,
    GreekSymbol,
    Function,
    LeftParen,
    RightParen,
    Operator,
    Other,
}

/// Mathematical constants that should trigger implicit multiplication
static CONSTANTS: LazyLock<HashSet<&'static str>> = LazyLock::new(|| {
    HashSet::from([
        // Basic constants (as token variants)
        "PI",
        "E_CONST",
        "I_CONST",
        "INFINITY",
        "PHI",
        "GOLDEN_RATIO",
        "EULER_GAMMA",
        "GAMMA_CONST",
        "UNDEFINED",
        // LaTeX constants
        "LATEX_PI",
        "LATEX_PHI",
        "LATEX_VARPHI",
        "LATEX_INFTY",
        "LATEX_EULER_GAMMA",
        "LATEX_GAMMA", // Gamma function
        // Wolfram constants/functions that act like constants
        "WOLFRAM_GAMMA", // Gamma function
    ])
});

/// Greek symbols that should trigger implicit multiplication
static GREEK_SYMBOLS: LazyLock<HashSet<&'static str>> = LazyLock::new(|| {
    HashSet::from([
        // LaTeX Greek symbols
        "LATEX_ALPHA",
        "LATEX_BETA",
        "LATEX_DELTA",
        "LATEX_EPSILON",
        "LATEX_ZETA",
        "LATEX_ETA",
        "LATEX_THETA",
        "LATEX_IOTA",
        "LATEX_KAPPA",
        "LATEX_LAMBDA",
        "LATEX_MU",
        "LATEX_NU",
        "LATEX_XI",
        "LATEX_OMICRON",
        "LATEX_RHO",
        "LATEX_SIGMA",
        "LATEX_TAU",
        "LATEX_UPSILON",
        "LATEX_CHI",
        "LATEX_PSI",
        "LATEX_OMEGA",
        // Wolfram Greek symbols
        "WOLFRAM_ALPHA",
        "WOLFRAM_BETA",
        "WOLFRAM_DELTA",
        "WOLFRAM_EPSILON",
        "WOLFRAM_ZETA",
        "WOLFRAM_ETA",
        "WOLFRAM_THETA",
        "WOLFRAM_IOTA",
        "WOLFRAM_KAPPA",
        "WOLFRAM_LAMBDA",
        "WOLFRAM_MU",
        "WOLFRAM_NU",
        "WOLFRAM_XI",
        "WOLFRAM_OMICRON",
        "WOLFRAM_RHO",
        "WOLFRAM_SIGMA",
        "WOLFRAM_TAU",
        "WOLFRAM_UPSILON",
        "WOLFRAM_CHI",
        "WOLFRAM_PSI",
        "WOLFRAM_OMEGA",
    ])
});

/// Mathematical functions that should NOT trigger implicit multiplication when followed by parentheses
static FUNCTIONS: LazyLock<HashSet<&'static str>> = LazyLock::new(|| {
    HashSet::from([
        "sin", "cos", "tan", "sec", "csc", "cot", "sinh", "cosh", "tanh", "sech", "csch", "coth",
        "arcsin", "arccos", "arctan", "arcsec", "arccsc", "arccot", "asin", "acos", "atan", "asec",
        "acsc", "acot", "log", "ln", "exp", "sqrt", "abs", "floor", "ceil", "round", "sign", "max",
        "min", "gcd", "lcm", "gamma", "beta", "zeta", "erf", "erfc", "J", "Y", "I",
        "K", // Bessel functions
        "P", "Q", "L", "H", // Legendre, Hermite functions
        "F", "G", "U", "M", "W", // Hypergeometric, Whittaker functions
    ])
});

/// Rules for when implicit multiplication should be inserted
static IMPLICIT_MUL_RULES: LazyLock<HashSet<(TokenCategory, TokenCategory)>> =
    LazyLock::new(|| {
        HashSet::from([
            // Number followed by anything multiplicative
            (TokenCategory::Number, TokenCategory::Identifier),
            (TokenCategory::Number, TokenCategory::Constant),
            (TokenCategory::Number, TokenCategory::GreekSymbol),
            (TokenCategory::Number, TokenCategory::LeftParen),
            // Identifier followed by anything multiplicative
            (TokenCategory::Identifier, TokenCategory::Identifier),
            (TokenCategory::Identifier, TokenCategory::Constant),
            (TokenCategory::Identifier, TokenCategory::GreekSymbol),
            // Constants followed by anything multiplicative
            (TokenCategory::Constant, TokenCategory::Identifier),
            (TokenCategory::Constant, TokenCategory::Constant),
            (TokenCategory::Constant, TokenCategory::GreekSymbol),
            (TokenCategory::Constant, TokenCategory::LeftParen),
            // Greek symbols followed by anything multiplicative
            (TokenCategory::GreekSymbol, TokenCategory::Identifier),
            (TokenCategory::GreekSymbol, TokenCategory::Constant),
            (TokenCategory::GreekSymbol, TokenCategory::GreekSymbol),
            (TokenCategory::GreekSymbol, TokenCategory::LeftParen),
            // Right parentheses followed by anything multiplicative
            (TokenCategory::RightParen, TokenCategory::Identifier),
            (TokenCategory::RightParen, TokenCategory::Constant),
            (TokenCategory::RightParen, TokenCategory::GreekSymbol),
            (TokenCategory::RightParen, TokenCategory::LeftParen),
        ])
    });

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
}

impl<'input> ImplicitMultiplicationLexer<'input> {
    /// Create a new enhanced lexer
    pub fn new(input: &'input str) -> Self {
        Self {
            base_lexer: SingleCharVariableLexer::new(input),
            token_buffer: VecDeque::new(),
            last_token: None,
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
            if self.should_insert_implicit_mul_before(&token) {
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
        let left_category = self.categorize_token(left);
        let right_category = self.categorize_token(right);

        // Special case: Don't insert multiplication between identifier and left paren if it's a function
        if left_category == TokenCategory::Identifier && right_category == TokenCategory::LeftParen
        {
            if let Token::IDENTIFIER(name) = left {
                if self.is_function_name(name) {
                    return false;
                }
            }
        }

        // Check if this combination should trigger implicit multiplication
        IMPLICIT_MUL_RULES.contains(&(left_category, right_category))
    }

    /// Categorize a token for implicit multiplication logic
    fn categorize_token(&self, token: &Token<'input>) -> TokenCategory {
        use Token::*;

        match token {
            INTEGER(_) | FLOAT(_) => TokenCategory::Number,
            IDENTIFIER(_) => TokenCategory::Identifier,
            LPAREN => TokenCategory::LeftParen,
            RPAREN => TokenCategory::RightParen,
            PLUS | MINUS | MULTIPLY | DIVIDE | POWER | EQUALS | LESS | GREATER => {
                TokenCategory::Operator
            }
            token if self.is_constant_token_by_name(token) => TokenCategory::Constant,
            token if self.is_greek_symbol_token_by_name(token) => TokenCategory::GreekSymbol,
            _ => TokenCategory::Other,
        }
    }

    /// Check if an identifier is a known function name
    fn is_function_name(&self, name: &str) -> bool {
        FUNCTIONS.contains(name)
    }

    /// Check if a token is a mathematical constant by checking its debug name
    fn is_constant_token_by_name(&self, token: &Token<'input>) -> bool {
        let token_debug = format!("{:?}", token);
        let token_name = token_debug.split('(').next().unwrap_or("");
        CONSTANTS.contains(token_name)
    }

    /// Check if a token is a Greek symbol by checking its debug name
    fn is_greek_symbol_token_by_name(&self, token: &Token<'input>) -> bool {
        let token_debug = format!("{:?}", token);
        let token_name = token_debug.split('(').next().unwrap_or("");
        GREEK_SYMBOLS.contains(token_name)
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
                    tokens.push((start, Token::MULTIPLY, end));
                }
            }
        }

        tokens
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_implicit_multiplication_detection() {
        let mut lexer = ImplicitMultiplicationLexer::new("2x");

        // Should generate: INTEGER(2), ImplicitMultiply, IDENTIFIER(x)
        let tokens: Vec<_> = std::iter::from_fn(|| lexer.next_enhanced_token()).collect();

        assert_eq!(tokens.len(), 3);
        assert!(matches!(
            tokens[0].1,
            EnhancedToken::Regular(Token::INTEGER(_))
        ));
        assert!(matches!(tokens[1].1, EnhancedToken::ImplicitMultiply));
        assert!(matches!(
            tokens[2].1,
            EnhancedToken::Regular(Token::IDENTIFIER(_))
        ));
    }

    #[test]
    fn test_constants_implicit_multiplication() {
        // Test number * constant: 2π -> 2*π
        let mut lexer = ImplicitMultiplicationLexer::new("2pi");
        let tokens: Vec<_> = std::iter::from_fn(|| lexer.next_enhanced_token()).collect();

        assert!(tokens.len() >= 3);
        assert!(matches!(
            tokens[0].1,
            EnhancedToken::Regular(Token::INTEGER(_))
        ));
        assert!(matches!(tokens[1].1, EnhancedToken::ImplicitMultiply));
        assert!(matches!(tokens[2].1, EnhancedToken::Regular(Token::PI)));
    }

    #[test]
    fn test_token_categorization() {
        let lexer = ImplicitMultiplicationLexer::new("");

        // Test number categorization
        assert_eq!(
            lexer.categorize_token(&Token::INTEGER("2")),
            TokenCategory::Number
        );
        assert_eq!(
            lexer.categorize_token(&Token::FLOAT("2.5")),
            TokenCategory::Number
        );

        // Test identifier categorization
        assert_eq!(
            lexer.categorize_token(&Token::IDENTIFIER("x")),
            TokenCategory::Identifier
        );

        // Test constant categorization
        assert_eq!(lexer.categorize_token(&Token::PI), TokenCategory::Constant);
        assert_eq!(
            lexer.categorize_token(&Token::E_CONST),
            TokenCategory::Constant
        );

        // Test Greek symbol categorization
        assert_eq!(
            lexer.categorize_token(&Token::LATEX_ALPHA),
            TokenCategory::GreekSymbol
        );
        assert_eq!(
            lexer.categorize_token(&Token::WOLFRAM_BETA),
            TokenCategory::GreekSymbol
        );

        // Test parentheses categorization
        assert_eq!(
            lexer.categorize_token(&Token::LPAREN),
            TokenCategory::LeftParen
        );
        assert_eq!(
            lexer.categorize_token(&Token::RPAREN),
            TokenCategory::RightParen
        );
    }

    #[test]
    fn test_function_detection() {
        let lexer = ImplicitMultiplicationLexer::new("");

        // Test function name detection
        assert!(lexer.is_function_name("sin"));
        assert!(lexer.is_function_name("cos"));
        assert!(lexer.is_function_name("log"));
        assert!(lexer.is_function_name("sqrt"));

        // Test non-function names
        assert!(!lexer.is_function_name("x"));
        assert!(!lexer.is_function_name("variable"));
        assert!(!lexer.is_function_name("pi"));
    }

    #[test]
    fn test_no_implicit_mul_between_operators() {
        let mut lexer = ImplicitMultiplicationLexer::new("x+y");

        let tokens: Vec<_> = std::iter::from_fn(|| lexer.next_enhanced_token()).collect();

        // Should NOT insert implicit multiplication between x and +
        assert_eq!(tokens.len(), 3); // x, +, y (no implicit multiply)
        assert!(matches!(
            tokens[0].1,
            EnhancedToken::Regular(Token::IDENTIFIER(_))
        ));
        assert!(matches!(tokens[1].1, EnhancedToken::Regular(Token::PLUS)));
        assert!(matches!(
            tokens[2].1,
            EnhancedToken::Regular(Token::IDENTIFIER(_))
        ));
    }

    #[test]
    fn test_parentheses_implicit_mul() {
        let mut lexer = ImplicitMultiplicationLexer::new("2(x+1)");

        let tokens: Vec<_> = std::iter::from_fn(|| lexer.next_enhanced_token()).collect();

        // Should generate: INTEGER(2), ImplicitMultiply, LPAREN, IDENTIFIER(x), PLUS, INTEGER(1), RPAREN
        assert!(tokens.len() >= 3);
        assert!(matches!(
            tokens[0].1,
            EnhancedToken::Regular(Token::INTEGER(_))
        ));
        assert!(matches!(tokens[1].1, EnhancedToken::ImplicitMultiply));
        assert!(matches!(tokens[2].1, EnhancedToken::Regular(Token::LPAREN)));
    }

    #[test]
    fn test_implicit_multiplication_rules() {
        let lexer = ImplicitMultiplicationLexer::new("");

        // Test various combinations that should trigger implicit multiplication
        assert!(lexer.should_insert_between(&Token::INTEGER("2"), &Token::PI));
        assert!(lexer.should_insert_between(&Token::FLOAT("2.5"), &Token::E_CONST));
        assert!(lexer.should_insert_between(&Token::IDENTIFIER("x"), &Token::LATEX_ALPHA));
        assert!(lexer.should_insert_between(&Token::PI, &Token::IDENTIFIER("x")));
        assert!(lexer.should_insert_between(&Token::LATEX_ALPHA, &Token::LATEX_BETA));
        assert!(lexer.should_insert_between(&Token::PI, &Token::E_CONST));
        assert!(lexer.should_insert_between(&Token::RPAREN, &Token::LATEX_PI));
        assert!(lexer.should_insert_between(&Token::INTEGER("2"), &Token::LPAREN));

        // Test combinations that should NOT trigger implicit multiplication
        assert!(!lexer.should_insert_between(&Token::PLUS, &Token::IDENTIFIER("x")));
        assert!(!lexer.should_insert_between(&Token::IDENTIFIER("sin"), &Token::LPAREN));
        assert!(!lexer.should_insert_between(&Token::MULTIPLY, &Token::PI));
    }

    #[test]
    fn test_comprehensive_implicit_multiplication() {
        let lexer = ImplicitMultiplicationLexer::new("");

        // Numbers with constants: 2π, 3.14e, 5φ
        assert!(lexer.should_insert_between(&Token::INTEGER("2"), &Token::PI));
        assert!(lexer.should_insert_between(&Token::FLOAT("3.14"), &Token::E_CONST));
        assert!(lexer.should_insert_between(&Token::INTEGER("5"), &Token::PHI));
        assert!(lexer.should_insert_between(&Token::INTEGER("2"), &Token::LATEX_PI));

        // Variables with Greek symbols: xα, yβ, zδ
        assert!(lexer.should_insert_between(&Token::IDENTIFIER("x"), &Token::LATEX_ALPHA));
        assert!(lexer.should_insert_between(&Token::IDENTIFIER("y"), &Token::LATEX_BETA));
        assert!(lexer.should_insert_between(&Token::IDENTIFIER("z"), &Token::LATEX_DELTA));

        // Constants with constants: πe, φΓ (Gamma function)
        assert!(lexer.should_insert_between(&Token::PI, &Token::E_CONST));
        assert!(lexer.should_insert_between(&Token::PHI, &Token::WOLFRAM_GAMMA));

        // Greek symbols with Greek symbols: αβ, δε
        assert!(lexer.should_insert_between(&Token::LATEX_ALPHA, &Token::LATEX_BETA));
        assert!(lexer.should_insert_between(&Token::LATEX_DELTA, &Token::LATEX_EPSILON));

        // Constants with parentheses: π(x), e(y)
        assert!(lexer.should_insert_between(&Token::PI, &Token::LPAREN));
        assert!(lexer.should_insert_between(&Token::E_CONST, &Token::LPAREN));

        // Greek symbols with parentheses: α(x), β(y)
        assert!(lexer.should_insert_between(&Token::LATEX_ALPHA, &Token::LPAREN));
        assert!(lexer.should_insert_between(&Token::WOLFRAM_BETA, &Token::LPAREN));

        // Parentheses with everything: (x)π, (y)α, (z)2
        assert!(lexer.should_insert_between(&Token::RPAREN, &Token::PI));
        assert!(lexer.should_insert_between(&Token::RPAREN, &Token::LATEX_ALPHA));
        assert!(lexer.should_insert_between(&Token::RPAREN, &Token::IDENTIFIER("x")));
    }
}
