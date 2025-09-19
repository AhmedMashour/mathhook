//! LaTeX mathematical expression parser
//!
//! Handles LaTeX-specific syntax including fractions, functions, and commands.

use crate::core::{Expression, Number, Symbol};
use crate::parsing::{constants::*, ParseError};
use std::collections::HashMap;

/// LaTeX-specific parser
pub struct LaTeXParser {
    variables: HashMap<String, Symbol>,
}

impl LaTeXParser {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
        }
    }

    /// Parse LaTeX expression with comprehensive LaTeX support
    pub fn parse(&mut self, input: &str) -> Result<Expression, ParseError> {
        if input.is_empty() {
            return Err(ParseError::EmptyInput);
        }

        let cleaned = self.preprocess(input);

        // Handle LaTeX commands first
        if let Some(expr) = self.parse_commands(&cleaned)? {
            return Ok(expr);
        }

        // Fall back to existing parser for basic expressions
        let mut old_parser = crate::parsing::ExpressionParser::new();
        old_parser.parse_latex(&cleaned)
    }

    /// Preprocess LaTeX input
    fn preprocess(&self, latex: &str) -> String {
        let mut result = latex.trim().to_string();

        // Apply simple replacements only
        for (latex_symbol, replacement) in LATEX_SIMPLE_REPLACEMENTS {
            result = result.replace(latex_symbol, replacement);
        }

        // Remove spaces
        result.replace(" ", "")
    }

    /// Parse LaTeX commands like \frac{}{}, \sin(), etc.
    fn parse_commands(&mut self, input: &str) -> Result<Option<Expression>, ParseError> {
        // Handle \frac{numerator}{denominator}
        if input.starts_with("\\frac{") {
            return self.parse_fraction(input).map(Some);
        }

        // Handle \sin(x), \cos(x), etc.
        if let Some(func_result) = self.parse_function(input)? {
            return Ok(Some(func_result));
        }

        // Handle \sqrt{x}, \sqrt[n]{x}
        if input.starts_with("\\sqrt") {
            return self.parse_sqrt(input).map(Some);
        }

        Ok(None)
    }

    /// Parse LaTeX fraction: \frac{numerator}{denominator}
    fn parse_fraction(&mut self, input: &str) -> Result<Expression, ParseError> {
        if !input.starts_with("\\frac{") {
            return Err(ParseError::SyntaxError("Not a fraction".to_string()));
        }

        let after_frac = &input[6..];

        let mut brace_count = 1;
        let mut numerator_end = 0;

        for (i, ch) in after_frac.chars().enumerate() {
            match ch {
                '{' => brace_count += 1,
                '}' => {
                    brace_count -= 1;
                    if brace_count == 0 {
                        numerator_end = i;
                        break;
                    }
                }
                _ => {}
            }
        }

        if numerator_end == 0 {
            return Err(ParseError::SyntaxError("Malformed fraction".to_string()));
        }

        let numerator_str = &after_frac[..numerator_end];
        let remainder = &after_frac[numerator_end + 1..];

        if !remainder.starts_with('{') {
            return Err(ParseError::SyntaxError(
                "Missing denominator braces".to_string(),
            ));
        }

        let mut brace_count = 1;
        let mut denominator_end = 0;

        for (i, ch) in remainder.chars().enumerate().skip(1) {
            match ch {
                '{' => brace_count += 1,
                '}' => {
                    brace_count -= 1;
                    if brace_count == 0 {
                        denominator_end = i;
                        break;
                    }
                }
                _ => {}
            }
        }

        if denominator_end == 0 {
            return Err(ParseError::SyntaxError("Malformed denominator".to_string()));
        }

        let denominator_str = &remainder[1..denominator_end];

        let numerator = self.parse(numerator_str)?;
        let denominator = self.parse(denominator_str)?;

        Ok(Expression::mul(vec![
            numerator,
            Expression::pow(denominator, Expression::integer(-1)),
        ]))
    }

    /// Extract numerator and denominator from \frac{num}{den}
    fn extract_fraction_parts(&self, input: &str) -> Result<(String, String), ParseError> {
        let after_frac = &input[6..]; // Skip "\\frac{"

        let numerator_end = self.find_matching_brace(after_frac, 0)?;
        let numerator_str = after_frac[..numerator_end].to_string();

        let remainder = &after_frac[numerator_end + 1..];
        if !remainder.starts_with('{') {
            return Err(ParseError::SyntaxError(
                "Missing denominator braces".to_string(),
            ));
        }

        let denominator_end = self.find_matching_brace(remainder, 0)?;
        let denominator_str = remainder[1..denominator_end].to_string();

        Ok((numerator_str, denominator_str))
    }

    /// Find matching closing brace starting from position
    fn find_matching_brace(&self, text: &str, start_pos: usize) -> Result<usize, ParseError> {
        let mut brace_count = 1;

        for (i, ch) in text.chars().enumerate().skip(start_pos) {
            match ch {
                '{' => brace_count += 1,
                '}' => {
                    brace_count -= 1;
                    if brace_count == 0 {
                        return Ok(i);
                    }
                }
                _ => {}
            }
        }

        Err(ParseError::SyntaxError("Unmatched braces".to_string()))
    }

    /// Parse LaTeX functions like \sin(x), \cos(x)
    fn parse_function(&mut self, input: &str) -> Result<Option<Expression>, ParseError> {
        for (latex_pattern, func_name) in LATEX_SIMPLE_FUNCTIONS {
            if input.starts_with(latex_pattern) {
                let after_func = &input[latex_pattern.len()..];
                let arg_end = self.find_matching_paren(after_func, 0)?;
                let arg_str = &after_func[..arg_end];
                let arg_expr = self.parse(arg_str)?;

                return Ok(Some(Expression::function(*func_name, vec![arg_expr])));
            }
        }

        Ok(None)
    }

    /// Find matching closing parenthesis
    fn find_matching_paren(&self, text: &str, start_pos: usize) -> Result<usize, ParseError> {
        let mut paren_count = 1; // We start inside the parentheses

        for (i, ch) in text.chars().enumerate().skip(start_pos) {
            match ch {
                '(' => paren_count += 1,
                ')' => {
                    paren_count -= 1;
                    if paren_count == 0 {
                        return Ok(i);
                    }
                }
                _ => {}
            }
        }

        Err(ParseError::SyntaxError("Unmatched parentheses".to_string()))
    }

    /// Parse LaTeX square root: \sqrt{x} or \sqrt[n]{x}
    fn parse_sqrt(&mut self, input: &str) -> Result<Expression, ParseError> {
        if input.starts_with("\\sqrt[") {
            // Handle \sqrt[n]{x}
            let after_sqrt = &input[6..]; // Skip "\\sqrt["
            let bracket_end = after_sqrt.find(']').ok_or_else(|| {
                ParseError::SyntaxError("Missing closing bracket in root".to_string())
            })?;

            let index_str = &after_sqrt[..bracket_end];
            let remainder = &after_sqrt[bracket_end + 1..];

            if !remainder.starts_with('{') {
                return Err(ParseError::SyntaxError(
                    "Missing braces in root".to_string(),
                ));
            }

            let brace_end = self.find_matching_brace(remainder, 0)?;
            let radicand_str = &remainder[1..brace_end];

            let index_expr = self.parse(index_str)?;
            let radicand_expr = self.parse(radicand_str)?;

            // \sqrt[n]{x} = x^(1/n)
            Ok(Expression::pow(
                radicand_expr,
                Expression::pow(index_expr, Expression::integer(-1)),
            ))
        } else if input.starts_with("\\sqrt{") {
            // Handle \sqrt{x}
            let after_sqrt = &input[6..]; // Skip "\\sqrt{"
            let brace_end = self.find_matching_brace(after_sqrt, 0)?;
            let radicand_str = &after_sqrt[..brace_end];
            let radicand_expr = self.parse(radicand_str)?;

            // \sqrt{x} = x^(1/2)
            Ok(Expression::pow(
                radicand_expr,
                Expression::number(Number::rational(num_rational::BigRational::new(
                    num_bigint::BigInt::from(1),
                    num_bigint::BigInt::from(2),
                ))),
            ))
        } else {
            Err(ParseError::SyntaxError("Not a sqrt expression".to_string()))
        }
    }
}
