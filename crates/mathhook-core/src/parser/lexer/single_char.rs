//! Single-character variable lexer for implicit multiplication
//!
//! This lexer tokenizes single-character variables separately to enable
//! implicit multiplication detection between adjacent variables.

use super::tokens::Token;
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
                    Some((start_pos, Token::IDENTIFIER(identifier), end_pos))
                }
                
                // Multi-character identifiers (functions, constants, etc.)
                'a'..='z' | 'A'..='Z' => self.lex_multi_char_identifier(start_pos),
                
                // Operators
                '+' => Some((start_pos, Token::PLUS, start_pos + 1)),
                '-' => Some((start_pos, Token::MINUS, start_pos + 1)),
                '*' => Some((start_pos, Token::MULTIPLY, start_pos + 1)),
                '/' => Some((start_pos, Token::DIVIDE, start_pos + 1)),
                '^' => Some((start_pos, Token::POWER, start_pos + 1)),
                '=' => Some((start_pos, Token::EQUALS, start_pos + 1)),
                
                // Delimiters
                '(' => Some((start_pos, Token::LPAREN, start_pos + 1)),
                ')' => Some((start_pos, Token::RPAREN, start_pos + 1)),
                '[' => Some((start_pos, Token::LBRACKET, start_pos + 1)),
                ']' => Some((start_pos, Token::RBRACKET, start_pos + 1)),
                '{' => Some((start_pos, Token::LBRACE, start_pos + 1)),
                '}' => Some((start_pos, Token::RBRACE, start_pos + 1)),
                ',' => Some((start_pos, Token::COMMA, start_pos + 1)),
                '!' => Some((start_pos, Token::FACTORIAL, start_pos + 1)),
                
                // LaTeX commands starting with backslash
                '\\' => self.lex_latex_command(start_pos),
                
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
        if let Some(&(_next_pos, next_ch)) = self.chars.peek() {
            let _current_ch = self.input.chars().nth(start_pos).unwrap();
            
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
        if has_dot {
            Some((start_pos, Token::FLOAT(number_str), end_pos))
        } else {
            Some((start_pos, Token::INTEGER(number_str), end_pos))
        }
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
            "pi" => Token::PI,
            "e" => Token::E_CONST,
            "i" => Token::I_CONST,
            "infinity" => Token::INFINITY,
            _ => Token::IDENTIFIER(identifier),
        };

        Some((start_pos, token, end_pos))
    }

    /// Lex LaTeX command starting with backslash
    fn lex_latex_command(&mut self, start_pos: usize) -> Option<(usize, Token<'input>, usize)> {
        // Skip the backslash
        let mut end_pos = start_pos + 1;
        
        // Read the command name
        while let Some(&(pos, ch)) = self.chars.peek() {
            if ch.is_alphabetic() {
                end_pos = pos + ch.len_utf8();
                self.chars.next();
            } else {
                break;
            }
        }
        
        let command = &self.input[start_pos..end_pos];
        
        // Map LaTeX commands to tokens
        let token = match command {
            "\\pi" => Token::LATEX_PI,
            "\\alpha" => Token::LATEX_ALPHA,
            "\\beta" => Token::LATEX_BETA,
            "\\gamma" => Token::LATEX_GAMMA,
            "\\delta" => Token::LATEX_DELTA,
            "\\epsilon" => Token::LATEX_EPSILON,
            "\\zeta" => Token::LATEX_ZETA,
            "\\eta" => Token::LATEX_ETA,
            "\\theta" => Token::LATEX_THETA,
            "\\iota" => Token::LATEX_IOTA,
            "\\kappa" => Token::LATEX_KAPPA,
            "\\lambda" => Token::LATEX_LAMBDA,
            "\\mu" => Token::LATEX_MU,
            "\\nu" => Token::LATEX_NU,
            "\\xi" => Token::LATEX_XI,
            "\\omicron" => Token::LATEX_OMICRON,
            "\\rho" => Token::LATEX_RHO,
            "\\sigma" => Token::LATEX_SIGMA,
            "\\tau" => Token::LATEX_TAU,
            "\\upsilon" => Token::LATEX_UPSILON,
            "\\phi" => Token::LATEX_PHI,
            "\\chi" => Token::LATEX_CHI,
            "\\psi" => Token::LATEX_PSI,
            "\\omega" => Token::LATEX_OMEGA,
            "\\varphi" => Token::LATEX_VARPHI,
            "\\infty" => Token::LATEX_INFTY,
            "\\sin" => Token::LATEX_SIN,
            "\\cos" => Token::LATEX_COS,
            "\\tan" => Token::LATEX_TAN,
            "\\ln" => Token::LATEX_LN,
            "\\log" => Token::LATEX_LOG,
            "\\sqrt" => Token::LATEX_SQRT,
            "\\frac" => Token::LATEX_FRAC,
            "\\cdot" => Token::LATEX_CDOT,
            "\\times" => Token::LATEX_TIMES,
            "\\div" => Token::LATEX_DIV,
            _ => {
                // If we don't recognize the LaTeX command, treat it as an identifier
                Token::IDENTIFIER(command)
            }
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
        assert!(matches!(tokens[0].1, Token::IDENTIFIER("x")));
        assert!(matches!(tokens[1].1, Token::IDENTIFIER("y")));
    }

    #[test]
    fn test_function_names_not_split() {
        let mut lexer = SingleCharVariableLexer::new("sin");
        
        let tokens: Vec<_> = std::iter::from_fn(|| lexer.next_token()).collect();
        
        // Should NOT split function names
        assert_eq!(tokens.len(), 1);
        assert!(matches!(tokens[0].1, Token::IDENTIFIER("sin")));
    }

    #[test]
    fn test_number_variable_sequence() {
        let mut lexer = SingleCharVariableLexer::new("2x");
        
        let tokens: Vec<_> = std::iter::from_fn(|| lexer.next_token()).collect();
        
        // Should tokenize as number and single-char variable
        assert_eq!(tokens.len(), 2);
        assert!(matches!(tokens[0].1, Token::INTEGER("2")));
        assert!(matches!(tokens[1].1, Token::IDENTIFIER("x")));
    }

    #[test]
    fn test_complex_expression() {
        let mut lexer = SingleCharVariableLexer::new("2xy + sin(z)");
        
        let tokens: Vec<_> = std::iter::from_fn(|| lexer.next_token()).collect();
        
        // Should tokenize as: 2, x, y, +, sin, (, z, )
        assert!(tokens.len() >= 8);
        assert!(matches!(tokens[0].1, Token::INTEGER("2")));
        assert!(matches!(tokens[1].1, Token::IDENTIFIER("x")));
        assert!(matches!(tokens[2].1, Token::IDENTIFIER("y")));
        assert!(matches!(tokens[3].1, Token::PLUS));
        assert!(matches!(tokens[4].1, Token::IDENTIFIER("sin")));
        assert!(matches!(tokens[5].1, Token::LPAREN));
        assert!(matches!(tokens[6].1, Token::IDENTIFIER("z")));
        assert!(matches!(tokens[7].1, Token::RPAREN));
    }

    #[test]
    fn test_constants_not_split() {
        let mut lexer = SingleCharVariableLexer::new("pi");
        
        let tokens: Vec<_> = std::iter::from_fn(|| lexer.next_token()).collect();
        
        // Should NOT split constants
        assert_eq!(tokens.len(), 1);
        assert!(matches!(tokens[0].1, Token::PI));
    }

    #[test]
    fn test_latex_tokens() {
        let mut lexer = SingleCharVariableLexer::new("\\pi");
        
        let tokens: Vec<_> = std::iter::from_fn(|| lexer.next_token()).collect();
        
        // Should tokenize LaTeX \pi as LATEX_PI
        assert_eq!(tokens.len(), 1);
        assert!(matches!(tokens[0].1, Token::LATEX_PI));
    }

    #[test]
    fn test_latex_greek_symbols() {
        let mut lexer = SingleCharVariableLexer::new("\\alpha");
        
        let tokens: Vec<_> = std::iter::from_fn(|| lexer.next_token()).collect();
        
        // Should tokenize LaTeX \alpha as LATEX_ALPHA
        assert_eq!(tokens.len(), 1);
        assert!(matches!(tokens[0].1, Token::LATEX_ALPHA));
    }
}
