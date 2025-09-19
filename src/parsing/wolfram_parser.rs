//! Wolfram Language mathematical expression parser
//!
//! Handles Wolfram Language (Mathematica) syntax including functions and operators.

use crate::core::{Expression, Number, Symbol};
use crate::parsing::{constants::*, ParseError};
use std::collections::HashMap;

/// Wolfram Language specific parser
pub struct WolframParser {
    variables: HashMap<String, Symbol>,
}

impl WolframParser {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
        }
    }

    /// Parse Wolfram expression with comprehensive Wolfram support
    pub fn parse(&mut self, input: &str) -> Result<Expression, ParseError> {
        if input.is_empty() {
            return Err(ParseError::EmptyInput);
        }

        let cleaned = self.preprocess(input);

        // Handle Wolfram functions first
        if let Some(expr) = self.parse_functions(&cleaned)? {
            return Ok(expr);
        }

        // Fall back to conversion and existing parser
        let converted = self.convert_to_latex_syntax(&cleaned);
        let mut latex_parser = crate::parsing::latex_parser::LaTeXParser::new();
        latex_parser.parse(&converted)
    }

    /// Preprocess Wolfram Language input
    fn preprocess(&self, wolfram: &str) -> String {
        let mut result = wolfram.trim().to_string();

        // Apply simple replacements only
        for (wolfram_const, replacement) in WOLFRAM_SIMPLE_REPLACEMENTS {
            result = result.replace(wolfram_const, replacement);
        }

        // Remove extra spaces but keep single spaces for function arguments
        result.replace("  ", " ")
    }

    /// Parse Wolfram functions like Sin[x], Times[x, y], etc.
    fn parse_functions(&mut self, input: &str) -> Result<Option<Expression>, ParseError> {
        // Handle simple functions
        for (wolfram_pattern, func_name) in WOLFRAM_SIMPLE_FUNCTIONS {
            if input.starts_with(wolfram_pattern) {
                let args = self.parse_function_args(&input[wolfram_pattern.len()..])?;
                return Ok(Some(Expression::function(*func_name, args)));
            }
        }

        // Handle special functions
        for (wolfram_pattern, special_type) in WOLFRAM_SPECIAL_FUNCTIONS {
            if input.starts_with(wolfram_pattern) {
                let args = self.parse_function_args(&input[wolfram_pattern.len()..])?;

                return Ok(Some(match *special_type {
                    "sqrt_to_power" if args.len() == 1 => {
                        // Sqrt[x] = x^(1/2)
                        Expression::pow(
                            args[0].clone(),
                            Expression::number(Number::rational(num_rational::BigRational::new(
                                num_bigint::BigInt::from(1),
                                num_bigint::BigInt::from(2),
                            ))),
                        )
                    }
                    "exp_to_power" if args.len() == 1 => {
                        // Exp[x] = e^x
                        Expression::pow(Expression::e(), args[0].clone())
                    }
                    _ => Expression::function("unknown", args),
                }));
            }
        }

        // Handle operators
        for (wolfram_pattern, op_name) in WOLFRAM_OPERATORS {
            if input.starts_with(wolfram_pattern) {
                let args = self.parse_function_args(&input[wolfram_pattern.len()..])?;

                return Ok(Some(match *op_name {
                    "add" => Expression::add(args),
                    "mul" => Expression::mul(args),
                    "pow" if args.len() == 2 => Expression::pow(args[0].clone(), args[1].clone()),
                    _ => Expression::function(*op_name, args),
                }));
            }
        }

        Ok(None)
    }

    /// Parse Wolfram function arguments: "x, y, z]" → [x, y, z]
    fn parse_function_args(&mut self, input: &str) -> Result<Vec<Expression>, ParseError> {
        let args_end = self.find_closing_bracket(input)?;
        let args_str = &input[..args_end];

        if args_str.is_empty() {
            return Ok(vec![]);
        }

        let arg_strings = self.split_args(args_str)?;

        let mut args = Vec::new();
        for arg_str in arg_strings {
            let arg_expr = self.parse(&arg_str)?;
            args.push(arg_expr);
        }

        Ok(args)
    }

    /// Find closing bracket for function arguments
    fn find_closing_bracket(&self, input: &str) -> Result<usize, ParseError> {
        let mut bracket_count = 1;

        for (i, ch) in input.chars().enumerate() {
            match ch {
                '[' => bracket_count += 1,
                ']' => {
                    bracket_count -= 1;
                    if bracket_count == 0 {
                        return Ok(i);
                    }
                }
                _ => {}
            }
        }

        Err(ParseError::SyntaxError(
            "Unmatched brackets in function".to_string(),
        ))
    }

    /// Split function arguments by commas, respecting nesting
    fn split_args(&self, args_str: &str) -> Result<Vec<String>, ParseError> {
        let mut args = Vec::new();
        let mut current_arg = String::new();
        let mut bracket_count = 0;
        let mut brace_count = 0;

        for ch in args_str.chars() {
            match ch {
                '[' => {
                    bracket_count += 1;
                    current_arg.push(ch);
                }
                ']' => {
                    bracket_count -= 1;
                    current_arg.push(ch);
                }
                '{' => {
                    brace_count += 1;
                    current_arg.push(ch);
                }
                '}' => {
                    brace_count -= 1;
                    current_arg.push(ch);
                }
                ',' if bracket_count == 0 && brace_count == 0 => {
                    args.push(current_arg.trim().to_string());
                    current_arg.clear();
                }
                _ => {
                    current_arg.push(ch);
                }
            }
        }

        if !current_arg.is_empty() {
            args.push(current_arg.trim().to_string());
        }

        Ok(args)
    }

    /// Convert Wolfram syntax to LaTeX-like syntax (temporary bridge)
    fn convert_to_latex_syntax(&self, wolfram: &str) -> String {
        let mut result = wolfram.to_string();

        // Convert functions
        result = result.replace("Sin[", "\\sin(");
        result = result.replace("Cos[", "\\cos(");
        result = result.replace("Tan[", "\\tan(");
        result = result.replace("Log[", "\\ln(");
        result = result.replace("Exp[", "\\exp(");
        result = result.replace("Sqrt[", "\\sqrt{");

        // Convert brackets
        result = result.replace("[", "(");
        result = result.replace("]", ")");
        result = result.replace("{", "(");
        result = result.replace("}", ")");

        result
    }
}
