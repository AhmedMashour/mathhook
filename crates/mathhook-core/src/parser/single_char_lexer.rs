//! Single-character variable lexer for implicit multiplication
//!
//! This lexer tokenizes single-character variables separately to enable
//! implicit multiplication detection between adjacent variables.

use crate::parser::lalrpop::lexer::tokens::Token;
use std::iter::Peekable;
use std::str::CharIndices;

/// Enhanced lexer that tokenizes single-character variables separately
pub struct SingleCharVariableLexer<'input> {
    input: &'input str,
    chars: Peekable<CharIndices<'input>>,
    current_pos: usize,
}

impl<'input> SingleCharVariableLexer<'input> {
    /// Create a new single-character variable lexer
    pub fn new(input: &'input str) -> Self {
        Self {
            input,
            chars: input.char_indices().peekable(),
            current_pos: 0,
        }
    }

    /// Get the next token, tokenizing single-char variables separately
    pub fn next_token(&mut self) -> Option<(usize, Token<'input>, usize)> {
        self.skip_whitespace();

        if let Some((start_pos, ch)) = self.chars.next() {
            self.current_pos = start_pos;

            match ch {
                // Numbers
                '0'..='9' => self.lex_number(start_pos),
                
                // Single-character variables (the key change!)
                'a'..='z' | 'A'..='Z' if self.should_tokenize_as_single_char(start_pos) => {
                    let end_pos = start_pos + ch.len_utf8();
                    let identifier = &self.input[start_pos..end_pos];
                    Some((start_pos, Token::Identifier(identifier), end_pos))
                }
                
                // Multi-character identifiers (functions, constants, etc.)
                'a'..='z' | 'A'..='Z' => self.lex_multi_char_identifier(start_pos),
                
                // Operators
                '+' => Some((start_pos, Token::Plus, start_pos + 1)),
                '-' => Some((start_pos, Token::Minus, start_pos + 1)),
                '*' => Some((start_pos, Token::Star, start_pos + 1)),
                '/' => Some((start_pos, Token::Slash, start_pos + 1)),
                '^' => Some((start_pos, Token::Caret, start_pos + 1)),
                '=' => Some((start_pos, Token::Equals, start_pos + 1)),
                
                // Delimiters
                '(' => Some((start_pos, Token::LParen, start_pos + 1)),
                ')' => Some((start_pos, Token::RParen, start_pos + 1)),
                '[' => Some((start_pos, Token::LBracket, start_pos + 1)),
                ']' => Some((start_pos, Token::RBracket, start_pos + 1)),
                '{' => Some((start_pos, Token::LBrace, start_pos + 1)),
                '}' => Some((start_pos, Token::RBrace, start_pos + 1)),
                ',' => Some((start_pos, Token::Comma, start_pos + 1)),
                '!' => Some((start_pos, Token::Exclamation, start_pos + 1)),
                
                // Unknown character
                _ => None,
            }
        } else {
            None
        }
    }

    /// Determine if this should be tokenized as a single character
    fn should_tokenize_as_single_char(&mut self, start_pos: usize) -> bool {
        // Look ahead to see what follows this character
        if let Some(&(next_pos, next_ch)) = self.chars.peek() {
            let current_ch = self.input.chars().nth(start_pos).unwrap();
            
            // Tokenize as single char if:
            // 1. Next character is also a single letter (for xy -> x, y)
            // 2. Next character is a number (for x2 -> x, 2)
            // 3. Next character is an operator or delimiter
            match next_ch {
                'a'..='z' | 'A'..='Z' => {
                    // Check if this looks like separate variables: xy, abc, etc.
                    self.looks_like_separate_variables(start_pos)
                }
                '0'..='9' => true, // x2 -> x, 2
                '+' | '-' | '*' | '/' | '^' | '=' | '(' | ')' | '[' | ']' | '{' | '}' | ',' | '!' => true,
                _ => false,
            }
        } else {
            // End of input - tokenize as single char
            true
        }
    }

    /// Check if a sequence looks like separate variables vs. a function name
    fn looks_like_separate_variables(&self, start_pos: usize) -> bool {
        // Get the full identifier to analyze
        let mut end_pos = start_pos + 1;
        let mut temp_chars = self.chars.clone();
        
        while let Some(&(pos, ch)) = temp_chars.peek() {
            if ch.is_alphabetic() {
                end_pos = pos + ch.len_utf8();
                temp_chars.next();
            } else {
                break;
            }
        }
        
        let full_identifier = &self.input[start_pos..end_pos];
        
        // Don't split known function names
        if self.is_known_function_or_constant(full_identifier) {
            return false;
        }
        
        // Don't split if it looks like a multi-letter variable name (contains numbers or underscores later)
        if full_identifier.len() > 1 {
            let rest = &full_identifier[1..];
            if rest.chars().any(|c| c.is_numeric() || c == '_') {
                return false;
            }
        }
        
        // Split if it's just consecutive single letters: xy, abc, etc.
        full_identifier.len() <= 3 && full_identifier.chars().all(|c| c.is_alphabetic())
    }

    /// Check if this is a known function or constant that shouldn't be split
    fn is_known_function_or_constant(&self, identifier: &str) -> bool {
        matches!(identifier,
            // Mathematical functions
            "sin" | "cos" | "tan" | "sec" | "csc" | "cot" |
            "sinh" | "cosh" | "tanh" | "sech" | "csch" | "coth" |
            "arcsin" | "arccos" | "arctan" | "arcsec" | "arccsc" | "arccot" |
            "asin" | "acos" | "atan" | "asec" | "acsc" | "acot" |
            "log" | "ln" | "exp" | "sqrt" | "abs" |
            "floor" | "ceil" | "round" | "sign" |
            "max" | "min" | "gcd" | "lcm" |
            "gamma" | "beta" | "zeta" | "erf" | "erfc" |
            
            // Constants
            "pi" | "e" | "euler_gamma" | "golden_ratio" | "infinity" | "undefined" |
            
            // Special functions
            "J" | "Y" | "I" | "K" | // Bessel functions (single letter but functions)
            "P" | "Q" | "L" | "H" | // Legendre, Hermite functions  
            "F" | "G" | "U" | "M" | "W" // Hypergeometric, Whittaker functions
        )
    }

    /// Lex a number (integer or float)
    fn lex_number(&mut self, start_pos: usize) -> Option<(usize, Token<'input>, usize)> {
        let mut end_pos = start_pos + 1;
        let mut has_dot = false;

        while let Some(&(pos, ch)) = self.chars.peek() {
            match ch {
                '0'..='9' => {
                    end_pos = pos + 1;
                    self.chars.next();
                }
                '.' if !has_dot => {
                    has_dot = true;
                    end_pos = pos + 1;
                    self.chars.next();
                }
                _ => break,
            }
        }

        let number_str = &self.input[start_pos..end_pos];
        Some((start_pos, Token::Number(number_str), end_pos))
    }

    /// Lex multi-character identifier (functions, constants, etc.)
    fn lex_multi_char_identifier(&mut self, start_pos: usize) -> Option<(usize, Token<'input>, usize)> {
        let mut end_pos = start_pos + 1;

        while let Some(&(pos, ch)) = self.chars.peek() {
            if ch.is_alphanumeric() || ch == '_' {
                end_pos = pos + ch.len_utf8();
                self.chars.next();
            } else {
                break;
            }
        }

        let identifier = &self.input[start_pos..end_pos];
        
        // Check for special constants and functions
        let token = match identifier {
            "pi" => Token::Pi,
            "e" => Token::E,
            "i" => Token::ImaginaryUnit,
            "infinity" => Token::Infinity,
            _ => Token::Identifier(identifier),
        };

        Some((start_pos, token, end_pos))
    }

    /// Skip whitespace characters
    fn skip_whitespace(&mut self) {
        while let Some(&(_, ch)) = self.chars.peek() {
            if ch.is_whitespace() {
                self.chars.next();
            } else {
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_char_variables() {
        let mut lexer = SingleCharVariableLexer::new("xy");
        
        let tokens: Vec<_> = std::iter::from_fn(|| lexer.next_token()).collect();
        
        // Should tokenize as two separate identifiers
        assert_eq!(tokens.len(), 2);
        assert!(matches!(tokens[0].1, Token::Identifier("x")));
        assert!(matches!(tokens[1].1, Token::Identifier("y")));
    }

    #[test]
    fn test_function_names_not_split() {
        let mut lexer = SingleCharVariableLexer::new("sin");
        
        let tokens: Vec<_> = std::iter::from_fn(|| lexer.next_token()).collect();
        
        // Should NOT split function names
        assert_eq!(tokens.len(), 1);
        assert!(matches!(tokens[0].1, Token::Identifier("sin")));
    }

    #[test]
    fn test_number_variable_sequence() {
        let mut lexer = SingleCharVariableLexer::new("2x");
        
        let tokens: Vec<_> = std::iter::from_fn(|| lexer.next_token()).collect();
        
        // Should tokenize as number and single-char variable
        assert_eq!(tokens.len(), 2);
        assert!(matches!(tokens[0].1, Token::Number("2")));
        assert!(matches!(tokens[1].1, Token::Identifier("x")));
    }

    #[test]
    fn test_complex_expression() {
        let mut lexer = SingleCharVariableLexer::new("2xy + sin(z)");
        
        let tokens: Vec<_> = std::iter::from_fn(|| lexer.next_token()).collect();
        
        // Should tokenize as: 2, x, y, +, sin, (, z, )
        assert!(tokens.len() >= 8);
        assert!(matches!(tokens[0].1, Token::Number("2")));
        assert!(matches!(tokens[1].1, Token::Identifier("x")));
        assert!(matches!(tokens[2].1, Token::Identifier("y")));
        assert!(matches!(tokens[3].1, Token::Plus));
        assert!(matches!(tokens[4].1, Token::Identifier("sin")));
        assert!(matches!(tokens[5].1, Token::LParen));
        assert!(matches!(tokens[6].1, Token::Identifier("z")));
        assert!(matches!(tokens[7].1, Token::RParen));
    }

    #[test]
    fn test_constants_not_split() {
        let mut lexer = SingleCharVariableLexer::new("pi");
        
        let tokens: Vec<_> = std::iter::from_fn(|| lexer.next_token()).collect();
        
        // Should NOT split constants
        assert_eq!(tokens.len(), 1);
        assert!(matches!(tokens[0].1, Token::Pi));
    }
}
