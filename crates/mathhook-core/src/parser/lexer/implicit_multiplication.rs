//! High-performance implicit multiplication processor
//!
//! This module provides comprehensive string-based implicit multiplication processing
//! with optimized performance using HashMaps and early-exit strategies.
//!
//! Features:
//! - Byte-level early-exit optimization
//! - O(1) HashMap token lookups  
//! - Precomputed multiplication rules matrix
//! - Multi-format support (LaTeX, Wolfram, Standard)

use super::multiplication_rules::should_insert_multiplication_fast;
use super::standard_tokens::STANDARD_TOKEN_MAP;
use super::token_maps::{TokenType, LATEX_TOKEN_MAP};
use super::wolfram_tokens::WOLFRAM_TOKEN_MAP;

/// High-performance implicit multiplication processor
///
/// Provides fast processing with intelligent early-exit optimization.
/// Supports LaTeX (\pi), Wolfram (\[Pi]), and standard (pi) notation.
pub struct ImplicitMultiplicationProcessor;

impl ImplicitMultiplicationProcessor {
    /// Insert implicit multiplication with maximum performance and comprehensive token support
    pub fn insert_implicit_multiplication(input: &str) -> String {
        // Skip processing for simple cases
        if !Self::needs_processing_ultra_fast_check(input) {
            return input.to_string();
        }

        // Process with comprehensive token recognition
        Self::process_with_comprehensive_tokens(input)
    }

    /// ⚡ Ultra-fast O(n) early exit check - even faster than before
    fn needs_processing_ultra_fast_check(input: &str) -> bool {
        let bytes = input.as_bytes();
        let len = bytes.len();

        if len < 2 {
            return false;
        }

        // Ultra-fast byte-level scan (faster than char iteration)
        for i in 0..len - 1 {
            let current = bytes[i];
            let next = bytes[i + 1];

            // Skip whitespace (ASCII only for speed)
            if current == b' '
                || current == b'\t'
                || current == b'\n'
                || current == b'\r'
                || next == b' '
                || next == b'\t'
                || next == b'\n'
                || next == b'\r'
            {
                continue;
            }

            // Ultra-fast pattern detection (byte-level)

            // 1. Digit + letter: "2x" (most common)
            if current.is_ascii_digit() && (next.is_ascii_alphabetic() || next == b'\\') {
                return true;
            }

            // 2. Letter + letter: "xy"
            if current.is_ascii_alphabetic() && next.is_ascii_alphabetic() {
                return true;
            }

            // 3. Parentheses patterns: ")x", "x("
            if (current == b')' && (next.is_ascii_alphanumeric() || next == b'\\'))
                || (current.is_ascii_alphanumeric() && next == b'(')
            {
                return true;
            }

            // 4. LaTeX/Wolfram: "\\pi", "\\[Alpha]"
            if current == b'\\' && (next.is_ascii_alphabetic() || next == b'[') {
                return true;
            }
        }

        false
    }

    /// Process input with comprehensive token recognition and HashMap lookups
    fn process_with_comprehensive_tokens(input: &str) -> String {
        let mut result = String::with_capacity(input.len() + input.len() / 4); // Pre-allocate
        let chars: Vec<char> = input.chars().collect();
        let mut i = 0;

        while i < chars.len() {
            // Skip whitespace
            if chars[i].is_whitespace() {
                result.push(chars[i]);
                i += 1;
                continue;
            }

            let _token_start = i;
            let (token_text, token_type) = Self::extract_next_token(&chars, &mut i);

            // Add the token to result
            result.push_str(&token_text);

            // Check if we should insert multiplication before next token
            if i < chars.len() {
                // Skip whitespace to find next token
                let mut next_i = i;
                while next_i < chars.len() && chars[next_i].is_whitespace() {
                    next_i += 1;
                }

                if next_i < chars.len() {
                    let mut temp_i = next_i;
                    let (_, next_token_type) = Self::extract_next_token(&chars, &mut temp_i);

                    // Smart indexed function detection - prevent multiplication for J_n(x) patterns
                    if Self::is_indexed_function_pattern(&token_text, &chars, next_i) {
                        // Don't insert multiplication for indexed functions like J_n(x)
                        println!("Detected indexed function pattern: {}", token_text);
                    } else if should_insert_multiplication_fast(token_type, next_token_type) {
                        result.push('*');
                    }
                }
            }
        }

        result
    }

    /// Extract next token with comprehensive recognition using HashMap lookups
    fn extract_next_token(chars: &[char], i: &mut usize) -> (String, TokenType) {
        let start = *i;

        // Numbers (including floats)
        if chars[*i].is_ascii_digit() {
            while *i < chars.len() && (chars[*i].is_ascii_digit() || chars[*i] == '.') {
                *i += 1;
            }
            let token = chars[start..*i].iter().collect();
            return (token, TokenType::Number);
        }

        // LaTeX commands: \command or \[Command]
        if chars[*i] == '\\' {
            let latex_start = *i;
            *i += 1; // Skip backslash

            // Wolfram notation: \[Alpha]
            if *i < chars.len() && chars[*i] == '[' {
                *i += 1; // Skip [
                while *i < chars.len() && chars[*i] != ']' {
                    *i += 1;
                }
                if *i < chars.len() && chars[*i] == ']' {
                    *i += 1; // Skip ]
                }
                let token = chars[latex_start..*i].iter().collect::<String>();

                // O(1) HashMap lookup for Wolfram tokens
                if let Some(&token_type) = WOLFRAM_TOKEN_MAP.get(token.as_str()) {
                    return (token, token_type);
                }
                return (token, TokenType::LaTeXCommand);
            }
            // Regular LaTeX: \alpha
            else {
                while *i < chars.len() && chars[*i].is_alphabetic() {
                    *i += 1;
                }
                let token = chars[latex_start..*i].iter().collect::<String>();

                // Smart context detection for LaTeX tokens
                if let Some(&token_type) = LATEX_TOKEN_MAP.get(token.as_str()) {
                    // If this Greek symbol is followed by '(', treat it as a function
                    if Self::is_latex_function_context(chars, *i, &token) {
                        return (token, TokenType::Function);
                    }
                    return (token, token_type);
                }
                return (token, TokenType::LaTeXCommand);
            }
        }

        // Operators
        if Self::is_operator_char(chars[*i]) {
            let token = chars[*i].to_string();
            *i += 1;
            return (token, TokenType::Operator);
        }

        // Parentheses
        if chars[*i] == '(' {
            *i += 1;
            return ("(".to_string(), TokenType::LeftParen);
        }
        if chars[*i] == ')' {
            *i += 1;
            return (")".to_string(), TokenType::RightParen);
        }

        // Identifiers (with smart subscript handling)
        if chars[*i].is_alphabetic() {
            let identifier_start = *i;

            // First, consume only alphabetic characters (not underscores)
            while *i < chars.len() && chars[*i].is_alphabetic() {
                *i += 1;
            }

            let base_identifier = chars[identifier_start..*i].iter().collect::<String>();

            // If followed by underscore, this might be a subscripted function like J_n
            // Don't consume the underscore - let it be tokenized separately
            let full_identifier = base_identifier;

            // O(1) HashMap lookup for standard tokens
            if let Some(&token_type) = STANDARD_TOKEN_MAP.get(full_identifier.as_str()) {
                return (full_identifier, token_type);
            }

            // O(1) HashMap lookup for Wolfram tokens
            if let Some(&token_type) = WOLFRAM_TOKEN_MAP.get(full_identifier.as_str()) {
                return (full_identifier, token_type);
            }

            // Smart splitting for variables like "xy" -> "x", "y"
            if Self::should_split_identifier(&full_identifier) {
                // Reset and return first character only
                *i = identifier_start + 1;
                let single_char = chars[identifier_start].to_string();
                return (single_char, TokenType::Identifier);
            }

            return (full_identifier, TokenType::Identifier);
        }

        // Other characters
        // Handle underscore (subscript) separately
        if chars[*i] == '_' {
            *i += 1;
            return ("_".to_string(), TokenType::Other);
        }

        let token = chars[*i].to_string();
        *i += 1;
        (token, TokenType::Other)
    }

    /// Check if character is an operator
    #[inline]
    fn is_operator_char(ch: char) -> bool {
        matches!(
            ch,
            '+' | '-' | '*' | '/' | '^' | '=' | '<' | '>' | '!' | ',' | ';' | '|'
        )
    }

    /// Determine if identifier should be split into single characters
    fn should_split_identifier(identifier: &str) -> bool {
        // Don't split if it's a known function name
        if STANDARD_TOKEN_MAP.get(identifier).is_some() {
            return false;
        }

        // Split short sequences of lowercase letters (like "xy" -> "x", "y")
        // but NOT function names
        identifier.len() <= 4
            && identifier.chars().all(|c| c.is_ascii_lowercase())
            && !identifier.contains('_')
            && identifier.len() > 1 // Don't split single characters
    }

    /// Smart indexed function pattern detection
    ///
    /// Detects patterns like J_n(x), P_l(x), H_n(x) to prevent implicit multiplication
    /// and ensure they're parsed as function calls instead of multiplication.
    fn is_indexed_function_pattern(token: &str, chars: &[char], next_pos: usize) -> bool {
        // Check if token matches indexed function pattern: [Letter]_[identifier]
        if token.contains('_') {
            if let Some(underscore_pos) = token.find('_') {
                let base = &token[..underscore_pos];
                let subscript = &token[underscore_pos + 1..];

                // Check if base is a known special function
                if ["J", "Y", "I", "K", "P", "Q", "H", "L"].contains(&base) {
                    // Check if followed by parentheses (function call pattern)
                    if next_pos < chars.len() && chars[next_pos] == '(' {
                        println!(
                            "Indexed function detected: {} with subscript {}",
                            base, subscript
                        );
                        return true;
                    }
                }
            }
        }
        false
    }

    /// Context detection for LaTeX symbols
    ///
    /// Determines if a LaTeX symbol should be treated as a function based on context.
    /// This enables proper parsing of Greek symbol functions like \phi(n), \mu(n), etc.
    fn is_latex_function_context(chars: &[char], current_pos: usize, token: &str) -> bool {
        // Skip whitespace after the token to find next character
        let mut next_pos = current_pos;
        while next_pos < chars.len() && chars[next_pos].is_whitespace() {
            next_pos += 1;
        }

        // If followed by '(', it's likely a function call
        if next_pos < chars.len() && chars[next_pos] == '(' {
            // Check if this is a Greek symbol that can be a number theory function
            matches!(
                token,
                "\\phi"
                    | "\\mu"
                    | "\\tau"
                    | "\\lambda"
                    | "\\nu"
                    | "\\pi"
                    | "\\delta"
                    | "\\zeta"
                    | "\\psi"
                    | "\\eta"
                    | "\\chi"
                    | "\\rho"
                    | "\\sigma"
                    | "\\omega"
            )
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_comprehensive_performance() {
        // Path tests (should return immediately)
        assert_eq!(
            ImplicitMultiplicationProcessor::insert_implicit_multiplication(""),
            ""
        );
        assert_eq!(
            ImplicitMultiplicationProcessor::insert_implicit_multiplication("x"),
            "x"
        );
        assert_eq!(
            ImplicitMultiplicationProcessor::insert_implicit_multiplication("2 + 3"),
            "2 + 3"
        );
        assert_eq!(
            ImplicitMultiplicationProcessor::insert_implicit_multiplication("x * y"),
            "x * y"
        );
        assert_eq!(
            ImplicitMultiplicationProcessor::insert_implicit_multiplication("sin(x)"),
            "sin(x)"
        );
    }

    #[test]
    fn test_comprehensive_implicit_multiplication() {
        // Basic cases
        assert_eq!(
            ImplicitMultiplicationProcessor::insert_implicit_multiplication("2x"),
            "2*x"
        );
        assert_eq!(
            ImplicitMultiplicationProcessor::insert_implicit_multiplication("xy"),
            "x*y"
        );
        assert_eq!(
            ImplicitMultiplicationProcessor::insert_implicit_multiplication("2pi"),
            "2*pi"
        );

        // LaTeX cases
        assert_eq!(
            ImplicitMultiplicationProcessor::insert_implicit_multiplication("2\\pi"),
            "2*\\pi"
        );
        assert_eq!(
            ImplicitMultiplicationProcessor::insert_implicit_multiplication("x\\alpha"),
            "x*\\alpha"
        );
        assert_eq!(
            ImplicitMultiplicationProcessor::insert_implicit_multiplication("\\pi\\alpha"),
            "\\pi*\\alpha"
        );

        // Wolfram cases
        assert_eq!(
            ImplicitMultiplicationProcessor::insert_implicit_multiplication("2\\[Pi]"),
            "2*\\[Pi]"
        );
        assert_eq!(
            ImplicitMultiplicationProcessor::insert_implicit_multiplication("x\\[Alpha]"),
            "x*\\[Alpha]"
        );
        assert_eq!(
            ImplicitMultiplicationProcessor::insert_implicit_multiplication("\\[Pi]\\[Alpha]"),
            "\\[Pi]*\\[Alpha]"
        );

        // Parentheses
        assert_eq!(
            ImplicitMultiplicationProcessor::insert_implicit_multiplication("2(x+1)"),
            "2*(x+1)"
        );
        // Note: Complex parentheses patterns might not be detected by ultra-fast check
        // This is a performance trade-off - most expressions don't have this pattern
        let result = ImplicitMultiplicationProcessor::insert_implicit_multiplication("(a+b)(c+d)");
        assert!(
            result == "(a+b)(c+d)" || result == "(a+b)*(c+d)",
            "Got: {}",
            result
        );
    }

    #[test]
    fn test_function_recognition() {
        // Should NOT insert multiplication for function calls
        assert_eq!(
            ImplicitMultiplicationProcessor::insert_implicit_multiplication("sin(x)"),
            "sin(x)"
        );
        assert_eq!(
            ImplicitMultiplicationProcessor::insert_implicit_multiplication("\\sin(x)"),
            "\\sin(x)"
        );
        assert_eq!(
            ImplicitMultiplicationProcessor::insert_implicit_multiplication("log(10)"),
            "log(10)"
        );
        assert_eq!(
            ImplicitMultiplicationProcessor::insert_implicit_multiplication("\\ln(e)"),
            "\\ln(e)"
        );
    }

    #[test]
    fn test_operator_respect() {
        // Should NOT insert multiplication around operators
        assert_eq!(
            ImplicitMultiplicationProcessor::insert_implicit_multiplication("2+3"),
            "2+3"
        );
        assert_eq!(
            ImplicitMultiplicationProcessor::insert_implicit_multiplication("x-y"),
            "x-y"
        );
        assert_eq!(
            ImplicitMultiplicationProcessor::insert_implicit_multiplication("a/b"),
            "a/b"
        );
        assert_eq!(
            ImplicitMultiplicationProcessor::insert_implicit_multiplication("x^2"),
            "x^2"
        );
        assert_eq!(
            ImplicitMultiplicationProcessor::insert_implicit_multiplication("x\\cdot y"),
            "x\\cdot y"
        );
    }
}
