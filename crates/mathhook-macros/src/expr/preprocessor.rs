//! Token preprocessing for expr!() macro with mathematical operator precedence
//!
//! Implements a Pratt parser at the token level to handle mathematical precedence:
//! - `^` (power) - highest precedence, right-associative
//! - `*`, `/` - middle precedence, left-associative
//! - `+`, `-` - lowest precedence, left-associative
//!
//! This allows users to write `expr!(2*x^2 + 4*x + 6)` naturally.

use proc_macro2::{Delimiter, Group, Punct, Spacing, TokenStream, TokenTree};
use quote::quote;

/// Operator precedence levels (higher = binds tighter)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Precedence {
    Lowest = 0,
    Comparison = 1, // ==, <, >, <=, >=
    Sum = 2,        // +, -
    Product = 3,    // *, /
    Power = 4,      // ^, **
    Unary = 5,      // -, !
}

/// Associativity of operators
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Associativity {
    Left,
    Right,
}

/// Parse result containing tokens and next position
struct ParseResult {
    tokens: TokenStream,
    next_pos: usize,
}

/// Token-level Pratt parser for mathematical expressions
pub struct MathPrecedenceParser;

impl MathPrecedenceParser {
    /// Parse tokens with mathematical precedence and return properly parenthesized stream
    pub fn parse(input: TokenStream) -> TokenStream {
        let tokens: Vec<TokenTree> = input.into_iter().collect();
        if tokens.is_empty() {
            return TokenStream::new();
        }

        // First, convert ** to ^ for uniform handling
        let tokens = Self::normalize_power_operator(&tokens);

        // Then apply mathematical precedence via Pratt parsing
        let result = Self::parse_expression(&tokens, 0);
        result.tokens
    }

    /// Convert ** sequences to ^ tokens for uniform handling
    fn normalize_power_operator(tokens: &[TokenTree]) -> Vec<TokenTree> {
        let mut result = Vec::new();
        let mut i = 0;

        while i < tokens.len() {
            // Check for ** sequence
            if i + 1 < tokens.len() {
                if let (TokenTree::Punct(p1), TokenTree::Punct(p2)) = (&tokens[i], &tokens[i + 1]) {
                    if p1.as_char() == '*' && p2.as_char() == '*' {
                        // Replace ** with ^
                        result.push(TokenTree::Punct(Punct::new('^', Spacing::Alone)));
                        i += 2;
                        continue;
                    }
                }
            }

            // Process groups recursively
            match &tokens[i] {
                TokenTree::Group(g) => {
                    let inner =
                        Self::normalize_power_operator(&g.stream().into_iter().collect::<Vec<_>>());
                    let new_group =
                        Group::new(g.delimiter(), inner.into_iter().collect::<TokenStream>());
                    result.push(TokenTree::Group(new_group));
                }
                other => result.push(other.clone()),
            }
            i += 1;
        }

        result
    }

    /// Get precedence of a binary operator
    fn get_precedence(op: char, next_char: Option<char>) -> Option<(Precedence, Associativity)> {
        match (op, next_char) {
            ('^', _) => Some((Precedence::Power, Associativity::Right)),
            ('*', Some('*')) => Some((Precedence::Power, Associativity::Right)), // ** (shouldn't reach here after normalization)
            ('*', _) => Some((Precedence::Product, Associativity::Left)),
            ('/', _) => Some((Precedence::Product, Associativity::Left)),
            ('+', _) => Some((Precedence::Sum, Associativity::Left)),
            ('-', _) => Some((Precedence::Sum, Associativity::Left)),
            ('=', Some('=')) => Some((Precedence::Comparison, Associativity::Left)),
            ('<', Some('=')) => Some((Precedence::Comparison, Associativity::Left)),
            ('>', Some('=')) => Some((Precedence::Comparison, Associativity::Left)),
            ('<', _) => Some((Precedence::Comparison, Associativity::Left)),
            ('>', _) => Some((Precedence::Comparison, Associativity::Left)),
            _ => None,
        }
    }

    /// Check if token is a binary operator
    fn is_binary_operator(tokens: &[TokenTree], pos: usize) -> Option<(char, usize)> {
        if pos >= tokens.len() {
            return None;
        }

        if let TokenTree::Punct(p) = &tokens[pos] {
            let c = p.as_char();
            let next = tokens.get(pos + 1).and_then(|t| {
                if let TokenTree::Punct(p2) = t {
                    Some(p2.as_char())
                } else {
                    None
                }
            });

            // Check for two-char operators
            match (c, next) {
                ('=', Some('=')) | ('<', Some('=')) | ('>', Some('=')) => return Some((c, 2)),
                ('*', Some('*')) => return Some((c, 2)), // ** power operator
                _ => {}
            }

            // Single char operators
            if matches!(c, '+' | '-' | '*' | '/' | '^' | '<' | '>') {
                return Some((c, 1));
            }
        }

        None
    }

    /// Parse expression with Pratt parsing
    fn parse_expression(tokens: &[TokenTree], start: usize) -> ParseResult {
        Self::parse_precedence(tokens, start, Precedence::Lowest)
    }

    /// Core Pratt parsing algorithm
    ///
    /// The key insight for Pratt parsing:
    /// - For left-associative operators: break when `prec <= min_prec`
    /// - For right-associative operators: break when `prec < min_prec`
    /// - Recursive calls use the operator's precedence as the new min_prec
    fn parse_precedence(tokens: &[TokenTree], start: usize, min_prec: Precedence) -> ParseResult {
        if start >= tokens.len() {
            return ParseResult {
                tokens: TokenStream::new(),
                next_pos: start,
            };
        }

        // Parse prefix/atom
        let mut result = Self::parse_prefix(tokens, start);
        let mut pos = result.next_pos;

        // Parse infix operators
        loop {
            if pos >= tokens.len() {
                break;
            }

            // Check for binary operator
            let op_info = Self::is_binary_operator(tokens, pos);
            if op_info.is_none() {
                break;
            }

            let (op_char, op_len) = op_info.unwrap();
            let next_char = if op_len == 2 {
                tokens.get(pos + 1).and_then(|t| {
                    if let TokenTree::Punct(p) = t {
                        Some(p.as_char())
                    } else {
                        None
                    }
                })
            } else {
                None
            };

            let prec_info = Self::get_precedence(op_char, next_char);
            if prec_info.is_none() {
                break;
            }

            let (prec, assoc) = prec_info.unwrap();

            // Standard Pratt parsing break conditions:
            // - Left-associative: break when prec <= min_prec (only consume higher precedence)
            // - Right-associative: break when prec < min_prec (consume same or higher precedence)
            let should_break = match assoc {
                Associativity::Left => prec <= min_prec,
                Associativity::Right => prec < min_prec,
            };
            if should_break {
                break;
            }

            // Consume the operator
            let op_tokens: Vec<TokenTree> = tokens[pos..pos + op_len].to_vec();
            pos += op_len;

            // Parse right side with the operator's precedence as the new minimum
            // This ensures operators at the same level get proper associativity handling
            let right = Self::parse_precedence(tokens, pos, prec);
            pos = right.next_pos;

            // Handle power operator specially - wrap in parentheses to ensure correct precedence
            if op_char == '^' || (op_char == '*' && next_char == Some('*')) {
                // For power: convert to .pow() method call for correct precedence
                let left = result.tokens;
                let right_tokens = right.tokens;
                result.tokens = quote! { (#left).pow(#right_tokens) };
            } else {
                // For other operators, just combine with the operator
                let left = result.tokens;
                let right_tokens = right.tokens;
                let op_stream: TokenStream = op_tokens.into_iter().collect();
                result.tokens = quote! { (#left) #op_stream (#right_tokens) };
            }
        }

        ParseResult {
            tokens: result.tokens,
            next_pos: pos,
        }
    }

    /// Parse prefix expression (unary ops, atoms)
    fn parse_prefix(tokens: &[TokenTree], start: usize) -> ParseResult {
        if start >= tokens.len() {
            return ParseResult {
                tokens: TokenStream::new(),
                next_pos: start,
            };
        }

        match &tokens[start] {
            // Unary minus
            TokenTree::Punct(p) if p.as_char() == '-' => {
                let inner = Self::parse_precedence(tokens, start + 1, Precedence::Unary);
                let inner_tokens = inner.tokens;
                ParseResult {
                    tokens: quote! { -(#inner_tokens) },
                    next_pos: inner.next_pos,
                }
            }

            // Parenthesized expression
            TokenTree::Group(g) if g.delimiter() == Delimiter::Parenthesis => {
                let inner_tokens: Vec<TokenTree> = g.stream().into_iter().collect();

                // Check if this is a function call (next token would be more parens)
                // For now, just parse the inner expression with precedence
                let inner_result = Self::parse_expression(&inner_tokens, 0);

                // Check for method call after the parentheses
                let mut result_tokens = inner_result.tokens;
                let mut pos = start + 1;

                // Handle chained method calls and operators
                while pos < tokens.len() {
                    if let TokenTree::Punct(p) = &tokens[pos] {
                        if p.as_char() == '.' {
                            // Method call
                            let method_result =
                                Self::parse_method_chain(tokens, pos, result_tokens);
                            result_tokens = method_result.tokens;
                            pos = method_result.next_pos;
                            continue;
                        }
                    }
                    break;
                }

                ParseResult {
                    tokens: quote! { (#result_tokens) },
                    next_pos: pos,
                }
            }

            // Identifier (variable or function call)
            TokenTree::Ident(_ident) => {
                let mut pos = start + 1;
                let ident_stream: TokenStream = std::iter::once(tokens[start].clone()).collect();

                // Check for function call
                if pos < tokens.len() {
                    if let TokenTree::Group(g) = &tokens[pos] {
                        if g.delimiter() == Delimiter::Parenthesis {
                            // Function call - process arguments
                            let args = Self::parse_function_args(g);
                            pos += 1;

                            // Check for method calls after function
                            let mut result = quote! { #ident_stream(#args) };

                            while pos < tokens.len() {
                                if let TokenTree::Punct(p) = &tokens[pos] {
                                    if p.as_char() == '.' {
                                        let method_result =
                                            Self::parse_method_chain(tokens, pos, result);
                                        result = method_result.tokens;
                                        pos = method_result.next_pos;
                                        continue;
                                    }
                                }
                                break;
                            }

                            return ParseResult {
                                tokens: result,
                                next_pos: pos,
                            };
                        }
                    }
                }

                // Check for method call on identifier
                let mut result = ident_stream;
                while pos < tokens.len() {
                    if let TokenTree::Punct(p) = &tokens[pos] {
                        if p.as_char() == '.' {
                            let method_result = Self::parse_method_chain(tokens, pos, result);
                            result = method_result.tokens;
                            pos = method_result.next_pos;
                            continue;
                        }
                    }
                    break;
                }

                ParseResult {
                    tokens: result,
                    next_pos: pos,
                }
            }

            // Literal (number)
            TokenTree::Literal(_lit) => {
                let lit_stream: TokenStream = std::iter::once(tokens[start].clone()).collect();
                let mut pos = start + 1;

                // Check for method call on literal
                let mut result = lit_stream;
                while pos < tokens.len() {
                    if let TokenTree::Punct(p) = &tokens[pos] {
                        if p.as_char() == '.' {
                            // Check if it's a float continuation or method call
                            if pos + 1 < tokens.len() {
                                if let TokenTree::Ident(_) = &tokens[pos + 1] {
                                    let method_result =
                                        Self::parse_method_chain(tokens, pos, result);
                                    result = method_result.tokens;
                                    pos = method_result.next_pos;
                                    continue;
                                }
                            }
                        }
                    }
                    break;
                }

                ParseResult {
                    tokens: result,
                    next_pos: pos,
                }
            }

            // Other punctuation that might start an expression
            other => ParseResult {
                tokens: std::iter::once(other.clone()).collect(),
                next_pos: start + 1,
            },
        }
    }

    /// Parse method chain (e.g., .pow(2), .abs(), .sqrt())
    fn parse_method_chain(
        tokens: &[TokenTree],
        start: usize,
        receiver: TokenStream,
    ) -> ParseResult {
        let mut pos = start;
        let mut result = receiver;

        while pos < tokens.len() {
            // Expect .
            if let TokenTree::Punct(p) = &tokens[pos] {
                if p.as_char() != '.' {
                    break;
                }
                pos += 1;
            } else {
                break;
            }

            // Expect method name
            if pos >= tokens.len() {
                break;
            }
            let method_name = if let TokenTree::Ident(ident) = &tokens[pos] {
                ident.clone()
            } else {
                break;
            };
            pos += 1;

            // Expect arguments (possibly empty)
            if pos < tokens.len() {
                if let TokenTree::Group(g) = &tokens[pos] {
                    if g.delimiter() == Delimiter::Parenthesis {
                        let args = Self::parse_function_args(g);
                        pos += 1;
                        result = quote! { (#result).#method_name(#args) };
                        continue;
                    }
                }
            }

            // Method with no arguments
            result = quote! { (#result).#method_name() };
        }

        ParseResult {
            tokens: result,
            next_pos: pos,
        }
    }

    /// Parse function arguments, applying precedence to each
    fn parse_function_args(group: &Group) -> TokenStream {
        let inner: Vec<TokenTree> = group.stream().into_iter().collect();
        if inner.is_empty() {
            return TokenStream::new();
        }

        // Split by commas and process each argument
        let mut args = Vec::new();
        let mut current_arg = Vec::new();

        for token in inner {
            if let TokenTree::Punct(p) = &token {
                if p.as_char() == ',' {
                    if !current_arg.is_empty() {
                        let arg_result = Self::parse_expression(&current_arg, 0);
                        args.push(arg_result.tokens);
                        current_arg.clear();
                    }
                    continue;
                }
            }
            current_arg.push(token);
        }

        // Don't forget the last argument
        if !current_arg.is_empty() {
            let arg_result = Self::parse_expression(&current_arg, 0);
            args.push(arg_result.tokens);
        }

        // Join with commas
        let mut result = TokenStream::new();
        for (i, arg) in args.into_iter().enumerate() {
            if i > 0 {
                result.extend(quote! { , });
            }
            result.extend(arg);
        }

        result
    }
}

/// Preprocess token stream to handle special operator sequences
///
/// This is the legacy API - delegates to MathPrecedenceParser
#[allow(dead_code)]
pub struct TokenPreprocessor;

#[allow(dead_code)]
impl TokenPreprocessor {
    /// Preprocess input tokens before syn parsing
    pub fn preprocess(input: TokenStream) -> TokenStream {
        MathPrecedenceParser::parse(input)
    }
}

/// Custom syn parser that handles mathematical precedence
pub struct PowerOperatorParser;

impl PowerOperatorParser {
    /// Parse expression with mathematical operator precedence support
    pub fn parse_with_power(input: proc_macro::TokenStream) -> syn::Result<syn::Expr> {
        let token_stream: TokenStream = input.into();
        let preprocessed = MathPrecedenceParser::parse(token_stream);
        syn::parse2(preprocessed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse_to_string(input: &str) -> String {
        let tokens: TokenStream = input.parse().unwrap();
        let result = MathPrecedenceParser::parse(tokens);
        result.to_string()
    }

    #[test]
    fn test_simple_power() {
        let result = parse_to_string("x ^ 2");
        assert!(result.contains("pow"), "Expected pow in: {}", result);
    }

    #[test]
    fn test_mul_before_power() {
        let result = parse_to_string("2 * x ^ 2");
        // Should be 2 * (x^2), not (2*x)^2
        // The result should have pow applied to x, then multiplied by 2
        assert!(result.contains("pow"), "Expected pow in: {}", result);
    }

    #[test]
    fn test_double_star_power() {
        let result = parse_to_string("x ** 2");
        assert!(result.contains("pow"), "Expected pow in: {}", result);
    }

    #[test]
    fn test_addition() {
        let result = parse_to_string("x + y");
        assert!(result.contains("+"), "Expected + in: {}", result);
    }

    #[test]
    fn test_complex_expression() {
        let result = parse_to_string("2 * x ^ 2 + 4 * x + 6");
        assert!(result.contains("pow"), "Expected pow in: {}", result);
        assert!(result.contains("+"), "Expected + in: {}", result);
    }

    #[test]
    fn test_addition_multiplication_debug() {
        let result = parse_to_string("2 * x + 3 * y");
        println!("DEBUG: 2 * x + 3 * y => {}", result);
        assert!(result.contains("+"), "Expected + in: {}", result);
    }

    #[test]
    fn test_comparison_debug() {
        let result = parse_to_string("x + 1 == y + 2");
        println!("DEBUG: x + 1 == y + 2 => {}", result);
        assert!(result.contains("=="), "Expected == in: {}", result);
    }

    #[test]
    fn test_power_chain_debug() {
        let result = parse_to_string("2 ^ 3 ^ 4");
        println!("DEBUG: 2 ^ 3 ^ 4 => {}", result);
        // For right-associative, should be: 2 ^ (3 ^ 4)
        // Which means: (2) .pow ((3) .pow (4))
    }
}
