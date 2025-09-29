use crate::core::Expression;
use crate::core::Number;
use crate::parser::constants::*;
use crate::parser::wolfram::WolframParser;
use crate::parser::ParseError;

impl WolframParser {
    /// Parse Wolfram expression with comprehensive Wolfram support
    pub fn parse(&mut self, input: &str) -> Result<Expression, ParseError> {
        if input.is_empty() {
            return Err(ParseError::EmptyInput);
        }

        let cleaned = self.preprocess(input);

        // Handle Wolfram matrices first
        if cleaned.starts_with("{{") {
            return self.parse_matrix(&cleaned);
        }

        // Handle Wolfram sets (single-level braces)
        if cleaned.starts_with("{") && !cleaned.starts_with("{{") {
            return self.parse_set(&cleaned);
        }

        // Handle special Wolfram functions first
        if cleaned.starts_with("Gamma[") {
            let after_gamma = &cleaned[6..]; // Skip "Gamma["
            if let Some(bracket_pos) = after_gamma.find(']') {
                let arg_str = &after_gamma[..bracket_pos];
                // Use SimpleParser to parse the argument
                let mut simple_parser = crate::parser::simple::SimpleParser::new();
                let arg_expr = simple_parser.parse(arg_str)?;
                return Ok(Expression::function("gamma", vec![arg_expr]));
            }
        }

        // Handle Piecewise[{{...}}] (Symbolica-inspired nested list parsing)
        if cleaned.starts_with("Piecewise[") {
            let after_piecewise = &cleaned[10..]; // Skip "Piecewise["
            if let Some(bracket_pos) = after_piecewise.find(']') {
                let args_str = &after_piecewise[..bracket_pos];
                // Parse nested list structure: {{x, x > 0}, {-x, x <= 0}}
                if args_str.starts_with("{{") && args_str.ends_with("}}") {
                    let inner = &args_str[2..args_str.len() - 2]; // Remove outer {{}}
                    let cases = inner.split("}, {").collect::<Vec<_>>();

                    let mut pieces = Vec::new();
                    for case in cases {
                        let case_clean = case.trim_start_matches('{').trim_end_matches('}');
                        if let Some(comma_pos) = case_clean.find(',') {
                            let expr_str = case_clean[..comma_pos].trim();
                            let condition_str = case_clean[comma_pos + 1..].trim();

                            let mut simple_parser = crate::parser::simple::SimpleParser::new();
                            let expr = simple_parser.parse(expr_str)?;
                            let condition = simple_parser.parse(condition_str)?;
                            pieces.push((condition, expr));
                        }
                    }

                    return Ok(Expression::piecewise(pieces, None));
                }
            }
        }

        // Handle Wolfram functions first
        if let Some(expr) = self.parse_functions(&cleaned)? {
            return Ok(expr);
        }

        // Fall back to conversion and existing parser
        let converted = self.convert_to_latex_syntax(&cleaned);
        let mut latex_parser = crate::parser::latex::LaTeXParser::new();
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
                    "derivative" if args.len() == 2 => {
                        // D[f, x] = derivative of f with respect to x
                        if let Expression::Symbol(var) = &args[1] {
                            Expression::derivative(args[0].clone(), var.clone(), 1)
                        } else {
                            Expression::function("D", args)
                        }
                    }
                    "integral" if args.len() == 2 => {
                        // Integrate[f, x] = integral of f with respect to x
                        if let Expression::Symbol(var) = &args[1] {
                            Expression::integral(args[0].clone(), var.clone())
                        } else {
                            Expression::function("Integrate", args)
                        }
                    }
                    "integral" if args.len() == 4 => {
                        // Integrate[f, {x, a, b}] → args = [f, x, a, b]
                        if let Expression::Symbol(var) = &args[1] {
                            Expression::definite_integral(
                                args[0].clone(),
                                var.clone(),
                                args[2].clone(),
                                args[3].clone(),
                            )
                        } else {
                            Expression::function("Integrate", args)
                        }
                    }
                    "limit" if args.len() == 3 => {
                        // Limit[f, x -> a] → args = [f, x, a]
                        if let Expression::Symbol(var) = &args[1] {
                            Expression::limit(args[0].clone(), var.clone(), args[2].clone())
                        } else {
                            Expression::function("Limit", args)
                        }
                    }
                    "sum" if args.len() == 4 => {
                        // Sum[f, {i, start, end}] → args = [f, i, start, end]
                        if let Expression::Symbol(var) = &args[1] {
                            Expression::sum(
                                args[0].clone(),
                                var.clone(),
                                args[2].clone(),
                                args[3].clone(),
                            )
                        } else {
                            Expression::function("Sum", args)
                        }
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
            // Handle Wolfram-specific argument syntax
            if arg_str.trim().starts_with('{') && arg_str.trim().ends_with('}') {
                // Parse Wolfram list: {x, 0, 1} → flatten into separate arguments
                let list_args = self.parse_list_elements(&arg_str)?;
                args.extend(list_args);
            } else if arg_str.contains(" -> ") {
                // Parse Wolfram arrow: x -> 0 → flatten into [variable, target]
                let arrow_args = self.parse_arrow_elements(&arg_str)?;
                args.extend(arrow_args);
            } else {
                // Parse as Wolfram expression (recursive)
                let arg_expr = self.parse(&arg_str)?;
                args.push(arg_expr);
            }
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

    /// Parse Wolfram list elements: {x, 0, 1} → [x, 0, 1]
    fn parse_list_elements(&mut self, input: &str) -> Result<Vec<Expression>, ParseError> {
        let trimmed = input.trim();
        if !trimmed.starts_with('{') || !trimmed.ends_with('}') {
            return Err(ParseError::SyntaxError("Not a Wolfram list".to_string()));
        }

        let content = &trimmed[1..trimmed.len() - 1]; // Remove { and }
        let element_strings = self.split_args(content)?;

        let mut elements = Vec::new();
        for element_str in element_strings {
            let element_expr = self.parse(&element_str)?;
            elements.push(element_expr);
        }

        Ok(elements)
    }

    /// Parse Wolfram arrow elements: x -> 0 → [x, 0]
    fn parse_arrow_elements(&mut self, input: &str) -> Result<Vec<Expression>, ParseError> {
        let parts: Vec<&str> = input.split(" -> ").collect();
        if parts.len() != 2 {
            return Err(ParseError::SyntaxError("Invalid arrow syntax".to_string()));
        }

        let mut result = Vec::new();
        for part in parts {
            let expr = self.parse(part.trim())?;
            result.push(expr);
        }

        Ok(result)
    }

    /// Parse Wolfram matrix syntax: {{1, 2}, {3, 4}}
    fn parse_matrix(&mut self, input: &str) -> Result<Expression, ParseError> {
        if !input.starts_with("{{") || !input.ends_with("}}") {
            return Err(ParseError::SyntaxError("Not a Wolfram matrix".to_string()));
        }

        let content = &input[1..input.len() - 1]; // Remove outer { and }

        // Split by }, { pattern to get rows
        let row_strings: Vec<&str> = content.split("}, {").collect();
        let mut matrix_rows = Vec::new();

        for (i, row_str) in row_strings.iter().enumerate() {
            // Clean up the row string
            let clean_row = if i == 0 {
                // First row: remove leading {
                &row_str[1..]
            } else if i == row_strings.len() - 1 {
                // Last row: remove trailing }
                &row_str[..row_str.len() - 1]
            } else {
                // Middle rows: use as-is
                row_str
            };

            // Parse row elements
            let elements = self.split_args(clean_row)?;
            let mut row_elements = Vec::new();

            for element_str in elements {
                let element_expr = self.parse(&element_str)?;
                row_elements.push(element_expr);
            }

            matrix_rows.push(row_elements);
        }

        Ok(Expression::matrix(matrix_rows))
    }

    /// Parse Wolfram set syntax: {1, 2, 3}
    fn parse_set(&mut self, input: &str) -> Result<Expression, ParseError> {
        if !input.starts_with("{") || !input.ends_with("}") {
            return Err(ParseError::SyntaxError("Not a Wolfram set".to_string()));
        }

        let content = &input[1..input.len() - 1]; // Remove { and }

        if content.trim().is_empty() {
            return Ok(Expression::set(vec![])); // Empty set
        }

        // Parse elements
        let element_strings = self.split_args(content)?;
        let mut elements = Vec::new();

        for element_str in element_strings {
            let element_expr = self.parse(&element_str)?;
            elements.push(element_expr);
        }

        Ok(Expression::set(elements))
    }
}
