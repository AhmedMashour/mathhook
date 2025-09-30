pub mod constants;
/// Custom lexer for mathematical expressions
///
/// Provides Unicode-aware tokenization of mathematical symbols and operators
/// optimized for LALRPOP parsing performance.
pub mod tokens;
pub mod unicode;

use constants::*;
use std::iter::Peekable;
use std::str::CharIndices;
use tokens::Token;
use unicode::UnicodeSymbols;

/// Mathematical expression lexer
///
/// Handles Unicode mathematical symbols, operators, and complex tokenization
/// patterns required for comprehensive mathematical expression parsing.
pub struct MathLexer<'input> {
    input: &'input str,
    chars: Peekable<CharIndices<'input>>,
    current_pos: usize,
    unicode_symbols: UnicodeSymbols,
}

impl<'input> MathLexer<'input> {
    /// Create a new lexer for the given input
    pub fn new(input: &'input str) -> Self {
        Self {
            input,
            chars: input.char_indices().peekable(),
            current_pos: 0,
            unicode_symbols: UnicodeSymbols::new(),
        }
    }

    /// Get the next token from the input
    pub fn next_token(&mut self) -> Option<(usize, Token<'input>, usize)> {
        self.skip_whitespace();

        if let Some((start_pos, ch)) = self.chars.next() {
            self.current_pos = start_pos;

            match ch {
                // Single character operators
                '+' => Some((start_pos, Token::Plus, start_pos + 1)),
                '*' => Some((start_pos, Token::Star, start_pos + 1)),
                '/' => Some((start_pos, Token::Slash, start_pos + 1)),
                '^' => Some((start_pos, Token::Caret, start_pos + 1)),
                '(' => Some((start_pos, Token::LParen, start_pos + 1)),
                ')' => Some((start_pos, Token::RParen, start_pos + 1)),
                '[' => Some((start_pos, Token::LBracket, start_pos + 1)),
                ']' => Some((start_pos, Token::RBracket, start_pos + 1)),
                '{' => Some((start_pos, Token::LBrace, start_pos + 1)),
                '}' => Some((start_pos, Token::RBrace, start_pos + 1)),
                ',' => Some((start_pos, Token::Comma, start_pos + 1)),
                ';' => Some((start_pos, Token::Semicolon, start_pos + 1)),
                '&' => Some((start_pos, Token::Ampersand, start_pos + 1)),

                // Multi-character operators and special cases
                '-' => self.lex_minus_or_arrow(start_pos),
                '=' => self.lex_equals_or_arrow(start_pos),
                '<' => self.lex_less_or_equal(start_pos),
                '>' => self.lex_greater_or_equal(start_pos),
                '!' => self.lex_exclamation_or_not_equal(start_pos),

                // Unicode mathematical symbols
                'π' => Some((start_pos, Token::Pi, start_pos + ch.len_utf8())),
                '∞' => Some((start_pos, Token::Infinity, start_pos + ch.len_utf8())),
                '±' => Some((start_pos, Token::PlusMinus, start_pos + ch.len_utf8())),
                '∓' => Some((start_pos, Token::MinusPlus, start_pos + ch.len_utf8())),
                '·' => Some((start_pos, Token::Cdot, start_pos + ch.len_utf8())),
                '×' => Some((start_pos, Token::Times, start_pos + ch.len_utf8())),
                '÷' => Some((start_pos, Token::Div, start_pos + ch.len_utf8())),
                'φ' => Some((start_pos, Token::Phi, start_pos + ch.len_utf8())),
                'γ' => Some((start_pos, Token::Gamma, start_pos + ch.len_utf8())),

                // Greek letters
                'α' => Some((start_pos, Token::Alpha, start_pos + ch.len_utf8())),
                'β' => Some((start_pos, Token::Beta, start_pos + ch.len_utf8())),
                'δ' => Some((start_pos, Token::Delta, start_pos + ch.len_utf8())),
                'ε' => Some((start_pos, Token::Epsilon, start_pos + ch.len_utf8())),
                'θ' => Some((start_pos, Token::Theta, start_pos + ch.len_utf8())),
                'λ' => Some((start_pos, Token::Lambda, start_pos + ch.len_utf8())),
                'μ' => Some((start_pos, Token::Mu, start_pos + ch.len_utf8())),
                'σ' => Some((start_pos, Token::Sigma, start_pos + ch.len_utf8())),
                'ω' => Some((start_pos, Token::Omega, start_pos + ch.len_utf8())),

                // Numbers (including negative numbers)
                '0'..='9' => self.lex_number(start_pos),

                // Identifiers, keywords, and LaTeX commands
                'a'..='z' | 'A'..='Z' | '_' => self.lex_identifier_or_keyword(start_pos),
                '\\' => self.lex_latex_command(start_pos),

                _ => {
                    // Check for other Unicode mathematical symbols
                    if self.unicode_symbols.is_math_symbol(ch) {
                        self.lex_unicode_symbol(start_pos, ch)
                    } else {
                        None // Skip unknown characters
                    }
                }
            }
        } else {
            None
        }
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

    /// Lex minus or arrow operator
    fn lex_minus_or_arrow(&mut self, start_pos: usize) -> Option<(usize, Token<'input>, usize)> {
        if let Some(&(_, '>')) = self.chars.peek() {
            self.chars.next(); // consume '>'
            Some((start_pos, Token::Arrow, start_pos + 2))
        } else {
            Some((start_pos, Token::Minus, start_pos + 1))
        }
    }

    /// Lex equals or double arrow
    fn lex_equals_or_arrow(&mut self, start_pos: usize) -> Option<(usize, Token<'input>, usize)> {
        if let Some(&(_, '=')) = self.chars.peek() {
            self.chars.next(); // consume second '='
            Some((start_pos, Token::NotEquals, start_pos + 2))
        } else if let Some(&(_, '>')) = self.chars.peek() {
            self.chars.next(); // consume '>'
            Some((start_pos, Token::DoubleArrow, start_pos + 2))
        } else {
            Some((start_pos, Token::Equals, start_pos + 1))
        }
    }

    /// Lex less than or less equal
    fn lex_less_or_equal(&mut self, start_pos: usize) -> Option<(usize, Token<'input>, usize)> {
        if let Some(&(_, '=')) = self.chars.peek() {
            self.chars.next(); // consume '='
            Some((start_pos, Token::LessEqual, start_pos + 2))
        } else {
            Some((start_pos, Token::Less, start_pos + 1))
        }
    }

    /// Lex greater than or greater equal
    fn lex_greater_or_equal(&mut self, start_pos: usize) -> Option<(usize, Token<'input>, usize)> {
        if let Some(&(_, '=')) = self.chars.peek() {
            self.chars.next(); // consume '='
            Some((start_pos, Token::GreaterEqual, start_pos + 2))
        } else {
            Some((start_pos, Token::Greater, start_pos + 1))
        }
    }

    /// Lex exclamation or not equal
    fn lex_exclamation_or_not_equal(
        &mut self,
        start_pos: usize,
    ) -> Option<(usize, Token<'input>, usize)> {
        if let Some(&(_, '=')) = self.chars.peek() {
            self.chars.next(); // consume '='
            Some((start_pos, Token::NotEquals, start_pos + 2))
        } else {
            Some((start_pos, Token::Exclamation, start_pos + 1))
        }
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

    /// Lex identifier or keyword with context awareness
    fn lex_identifier_or_keyword(
        &mut self,
        start_pos: usize,
    ) -> Option<(usize, Token<'input>, usize)> {
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

        // Check for Wolfram functions and constants
        if let Some(wolfram_token) = self.match_wolfram_pattern(&self.input[start_pos..]) {
            return Some(wolfram_token);
        }

        // Check for keywords and constants (optimized match for performance)
        let token = match identifier {
            // Control flow keywords
            "if" => Token::If,
            "then" => Token::Then,
            "else" => Token::Else,
            "true" => Token::True,
            "false" => Token::False,

            // Mathematical constants
            "e" => Token::E,
            "i" => Token::ImaginaryUnit,
            "d" => Token::DifferentialD,

            // Wolfram constants
            "Pi" => Token::WolframPi,
            "E" => Token::WolframE,
            "I" => Token::WolframI,
            "Infinity" => Token::WolframInfinity,

            // Default: treat as identifier
            _ => Token::Identifier(identifier),
        };

        Some((start_pos, token, end_pos))
    }

    /// Lex Unicode mathematical symbol
    fn lex_unicode_symbol(
        &mut self,
        start_pos: usize,
        ch: char,
    ) -> Option<(usize, Token<'input>, usize)> {
        let token = match ch {
            '∅' => Token::Identifier("emptyset"), // Set theory symbols as identifiers for now
            '∈' => Token::Identifier("in"),
            '∉' => Token::Identifier("notin"),
            '⊂' => Token::Identifier("subset"),
            '⊃' => Token::Identifier("supset"),
            '∪' => Token::Identifier("union"),
            '∩' => Token::Identifier("intersection"),
            _ => return None,
        };

        Some((start_pos, token, start_pos + ch.len_utf8()))
    }

    /// Match Wolfram function patterns
    fn match_wolfram_pattern(&mut self, input: &str) -> Option<(usize, Token<'input>, usize)> {
        let start_pos = self.current_pos;

        if let Some(pattern) = matches_wolfram_function(input) {
            let end_pos = start_pos + pattern.len();

            // Advance the lexer position
            for _ in 0..pattern.chars().count() - 1 {
                // -1 because we already consumed first char
                self.chars.next();
            }

            let token = match pattern {
                "Plus[" => Token::WolframPlus,
                "Times[" => Token::WolframTimes,
                "Power[" => Token::WolframPower,
                "Sin[" => Token::WolframSin,
                "Cos[" => Token::WolframCos,
                "Tan[" => Token::WolframTan,
                "Log[" => Token::WolframLog,
                "Exp[" => Token::WolframExp,
                "Sqrt[" => Token::WolframSqrt,
                "Gamma[" => Token::WolframGamma,
                "D[" => Token::WolframD,
                "Integrate[" => Token::WolframIntegrate,
                "Limit[" => Token::WolframLimit,
                "Sum[" => Token::WolframSum,
                "Product[" => Token::WolframProduct,
                "Piecewise[{" => Token::WolframPiecewise,
                _ => return None,
            };

            Some((start_pos, token, end_pos))
        } else {
            None
        }
    }

    /// Lex LaTeX command with comprehensive pattern matching
    fn lex_latex_command(&mut self, start_pos: usize) -> Option<(usize, Token<'input>, usize)> {
        let remaining_input = &self.input[start_pos..];

        if let Some(pattern) = matches_latex_command(remaining_input) {
            let end_pos = start_pos + pattern.len();

            // Advance the lexer position
            for _ in 0..pattern.chars().count() - 1 {
                // -1 because we already consumed backslash
                self.chars.next();
            }

            let token = match pattern {
                "\\frac{" => Token::FracStart,
                "\\sqrt{" => Token::SqrtStart,
                "\\sin(" => Token::SinFunc,
                "\\cos(" => Token::CosFunc,
                "\\tan(" => Token::TanFunc,
                "\\ln(" => Token::LnFunc,
                "\\log(" => Token::LogFunc,
                "\\Gamma(" => Token::GammaFunc,
                "\\exp(" => Token::ExpFunc,
                "\\sin^{" => Token::SinPower,
                "\\cos^{" => Token::CosPower,
                "\\tan^{" => Token::TanPower,
                "\\int" => Token::IntegralStart,
                "\\int_" => Token::IntegralBounds,
                "\\sum_{" => Token::SumStart,
                "\\prod_{" => Token::ProdStart,
                "\\lim_{" => Token::LimitStart,
                "\\begin{cases}" => Token::CasesStart,
                "\\end{cases}" => Token::CasesEnd,
                "\\begin{pmatrix}" => Token::PMatrixStart,
                "\\end{pmatrix}" => Token::PMatrixEnd,
                "\\left(" => Token::LeftParen,
                "\\right)" => Token::RightParen,
                "\\left[" => Token::LeftBracket,
                "\\right]" => Token::RightBracket,
                "\\left\\{" => Token::LeftBrace,
                "\\right\\}" => Token::RightBrace,
                "\\text{if}" => Token::TextIf,
                "\\to" => Token::To,
                "\\leq" => Token::Leq,
                "\\geq" => Token::Geq,
                "\\neq" => Token::Neq,
                "\\pi" => Token::Pi,
                "\\infty" => Token::Infinity,
                "\\cdot" => Token::Cdot,
                "\\times" => Token::Times,
                "\\pm" => Token::PlusMinus,
                "\\mp" => Token::MinusPlus,
                "\\\\" => Token::LineBreak,
                _ => {
                    // Fallback: lex as generic LaTeX command
                    let mut end_pos = start_pos + 1; // Skip backslash
                    while let Some(&(pos, ch)) = self.chars.peek() {
                        if ch.is_alphabetic() {
                            end_pos = pos + ch.len_utf8();
                            self.chars.next();
                        } else {
                            break;
                        }
                    }
                    return Some((
                        start_pos,
                        Token::Identifier(&self.input[start_pos..end_pos]),
                        end_pos,
                    ));
                }
            };

            Some((start_pos, token, end_pos))
        } else {
            // Fallback: lex as generic LaTeX command
            let mut end_pos = start_pos + 1; // Skip backslash
            while let Some(&(pos, ch)) = self.chars.peek() {
                if ch.is_alphabetic() {
                    end_pos = pos + ch.len_utf8();
                    self.chars.next();
                } else {
                    break;
                }
            }
            Some((
                start_pos,
                Token::Identifier(&self.input[start_pos..end_pos]),
                end_pos,
            ))
        }
    }
}

/// Iterator implementation for LALRPOP integration
impl<'input> Iterator for MathLexer<'input> {
    type Item = Result<(usize, Token<'input>, usize), ()>;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_token().map(Ok)
    }
}
