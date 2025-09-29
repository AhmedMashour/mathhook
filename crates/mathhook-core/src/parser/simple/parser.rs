use crate::core::Expression;
use crate::parser::simple::SimpleParser;
use crate::parser::ParseError;
use crate::utils::get_or_create_symbol;

impl SimpleParser {
    /// Parse simple mathematical expressions
    ///
    /// Handles standard mathematical notation without LaTeX commands or Wolfram syntax:
    /// - Numbers: `42`, `3.14`
    /// - Symbols: `x`, `alpha`, `var_1`
    /// - Functions: `sin(x)`, `log(a, b)`
    /// - Operations: `x + 2`, `2*x`, `x^2`
    /// - Parentheses: `(x + 1)/(x - 1)`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::parser::simple::SimpleParser;
    ///
    /// let mut parser = SimpleParser::new();
    /// let expr = parser.parse("x + 2").unwrap();
    /// let func = parser.parse("sin(x)").unwrap();
    /// let power = parser.parse("x^2").unwrap();
    /// ```
    pub fn parse(&mut self, input: &str) -> Result<Expression, ParseError> {
        let input = input.trim();

        if input.is_empty() {
            return Err(ParseError::EmptyInput);
        }

        // Handle numbers: 42, 3.14, -5
        if let Ok(num) = input.parse::<i64>() {
            return Ok(Expression::integer(num));
        }
        if let Ok(num) = input.parse::<f64>() {
            return Ok(Expression::number(num));
        }

        // Handle plus-minus operator: ± (Unicode-safe)
        if input.contains('±') {
            // Split on the ± character properly handling Unicode
            let parts: Vec<&str> = input.split('±').collect();
            if parts.len() == 2 && !parts[0].is_empty() && !parts[1].is_empty() {
                let before_expr = self.parse(parts[0].trim())?;
                let after_expr = self.parse(parts[1].trim())?;
                return Ok(Expression::function("pm", vec![before_expr, after_expr]));
            }
        }

        // Handle mathematical constants and LaTeX commands: ∞, π, e, \sin (Symbolica-inspired)
        match input.trim() {
            "∞" | "\\infty" | "{∞}" | "{\\infty}" => return Ok(Expression::infinity()),
            "π" | "\\pi" | "{π}" | "{\\pi}" => return Ok(Expression::pi()),
            "e" | "\\e" | "{e}" | "{\\e}" => return Ok(Expression::e()),
            "i" | "\\i" | "{i}" | "{\\i}" => return Ok(Expression::i()),
            // Handle standalone LaTeX functions (fallback for complex parsing)
            "\\sin" => return Ok(Expression::function("sin", vec![])),
            "\\cos" => return Ok(Expression::function("cos", vec![])),
            "\\tan" => return Ok(Expression::function("tan", vec![])),
            "\\ln" => return Ok(Expression::function("ln", vec![])),
            "\\log" => return Ok(Expression::function("log", vec![])),
            _ => {}
        }

        // Handle braced expressions: {content} → content (Symbolica pattern)
        if input.starts_with('{') && input.ends_with('}') && input.len() > 2 {
            let content = &input[1..input.len() - 1];
            return self.parse(content);
        }

        // Handle symbols: x, alpha, var_1
        if input.chars().all(|c| c.is_alphanumeric() || c == '_')
            && input.chars().next().unwrap().is_alphabetic()
        {
            return Ok(Expression::Symbol(get_or_create_symbol(input)));
        }

        // Handle open intervals: (0, 1) - check BEFORE general parentheses handling
        if input.starts_with('(') && input.ends_with(')') {
            let inner = &input[1..input.len() - 1];
            // Check if this looks like an interval (has comma, simple expressions)
            if inner.contains(',') && inner.split(',').count() == 2 {
                let parts: Vec<&str> = inner.split(',').collect();
                let left_part = parts[0].trim();
                let right_part = parts[1].trim();

                // If both parts are simple (numbers/symbols), treat as interval
                if self.looks_like_simple_expression(left_part)
                    && self.looks_like_simple_expression(right_part)
                {
                    let start = self.parse(left_part)?;
                    let end = self.parse(right_part)?;
                    return Ok(Expression::interval(start, end, false, false)); // open interval
                }
            }
        }

        // Handle parentheses: (x + 1) - do this after interval check
        if input.starts_with('(') && input.ends_with(')') && self.is_balanced_parentheses(input) {
            return self.parse(&input[1..input.len() - 1]);
        }

        // Handle intervals: [0, 1] - basic support for roundtrip
        if input.starts_with('[') && input.ends_with(']') {
            let inner = &input[1..input.len() - 1];
            if let Some(comma_pos) = inner.find(',') {
                let start = self.parse(inner[..comma_pos].trim())?;
                let end = self.parse(inner[comma_pos + 1..].trim())?;
                return Ok(Expression::interval(start, end, true, true)); // closed interval
            }
        }

        // Handle LaTeX function powers: \sin^2(x) (Symbolica-inspired)
        if input.contains("^") && input.contains("(") && input.starts_with("\\") {
            if let Some(caret_pos) = input.find('^') {
                let func_part = &input[..caret_pos];
                let after_caret = &input[caret_pos + 1..];

                if let Some(paren_pos) = after_caret.find('(') {
                    let power_str = &after_caret[..paren_pos];
                    let remaining = &after_caret[paren_pos..];

                    if remaining.ends_with(')') {
                        let args_str = &remaining[1..remaining.len() - 1];

                        // Parse the function, power, and arguments
                        let func_expr = self.parse(func_part)?;
                        let power_expr = self.parse(power_str)?;
                        let arg_expr = self.parse(args_str)?;

                        // Create sin(x)^2 structure
                        if let Expression::Function { name, .. } = func_expr {
                            let func_with_arg = Expression::function(&name, vec![arg_expr]);
                            return Ok(Expression::pow(func_with_arg, power_expr));
                        }
                    }
                }
            }
        }

        // Handle function calls: sin(x), log(x, 2)
        if let Some(paren_pos) = input.find('(') {
            if input.ends_with(')') && self.is_balanced_parentheses(&input[paren_pos..]) {
                let func_name = &input[..paren_pos];
                let args_str = &input[paren_pos + 1..input.len() - 1];

                if func_name.chars().all(|c| c.is_alphabetic()) || func_name.starts_with('\\') {
                    if args_str.trim().is_empty() {
                        return Ok(Expression::function(func_name, vec![]));
                    } else {
                        // Parse comma-separated arguments
                        let args: Result<Vec<_>, _> = self
                            .split_function_args(args_str)?
                            .iter()
                            .map(|arg| self.parse(arg.trim()))
                            .collect();
                        return Ok(Expression::function(func_name, args?));
                    }
                }
            }
        }

        // Handle equations: x = 5 or x == 5
        if input.contains("==") {
            if let Some(eq_pos) = input.find("==") {
                let left = self.parse(&input[..eq_pos])?;
                let right = self.parse(&input[eq_pos + 2..])?;
                return Ok(Expression::equation(left, right));
            }
        } else if let Some(eq_pos) = self.find_operator(input, '=') {
            let left = self.parse(&input[..eq_pos])?;
            let right = self.parse(&input[eq_pos + 1..])?;
            return Ok(Expression::equation(left, right));
        }

        // Handle inequalities: x < y, x > y
        if let Some(lt_pos) = self.find_operator(input, '<') {
            let left = self.parse(&input[..lt_pos])?;
            let right = self.parse(&input[lt_pos + 1..])?;
            return Ok(Expression::relation(
                left,
                right,
                crate::core::expression::RelationType::Less,
            ));
        }
        if let Some(gt_pos) = self.find_operator(input, '>') {
            let left = self.parse(&input[..gt_pos])?;
            let right = self.parse(&input[gt_pos + 1..])?;
            return Ok(Expression::relation(
                left,
                right,
                crate::core::expression::RelationType::Greater,
            ));
        }

        // Handle addition: x + 2 (right-to-left to handle precedence)
        if let Some(plus_pos) = self.find_operator(input, '+') {
            let left = self.parse(&input[..plus_pos])?;
            let right = self.parse(&input[plus_pos + 1..])?;
            return Ok(Expression::add(vec![left, right]));
        }

        // Handle subtraction: x - 2
        if let Some(minus_pos) = self.find_operator(input, '-') {
            let left = self.parse(&input[..minus_pos])?;
            let right = self.parse(&input[minus_pos + 1..])?;
            return Ok(Expression::add(vec![
                left,
                Expression::mul(vec![Expression::integer(-1), right]),
            ]));
        }

        // Handle multiplication: 2*x, x*y
        if let Some(mul_pos) = self.find_operator(input, '*') {
            let left = self.parse(&input[..mul_pos])?;
            let right = self.parse(&input[mul_pos + 1..])?;
            return Ok(Expression::mul(vec![left, right]));
        }

        // Handle division: x/2
        if let Some(div_pos) = self.find_operator(input, '/') {
            let left = self.parse(&input[..div_pos])?;
            let right = self.parse(&input[div_pos + 1..])?;
            return Ok(Expression::mul(vec![
                left,
                Expression::pow(right, Expression::integer(-1)),
            ]));
        }

        // Handle factorial: n! (before powers to handle n!^2 correctly)
        if input.ends_with('!') && input.len() > 1 {
            let base_str = &input[..input.len() - 1];
            // Only handle simple factorials like "n!" or "5!"
            if base_str.chars().all(|c| c.is_alphanumeric()) {
                let base_expr = self.parse(base_str)?;
                return Ok(Expression::function("factorial", vec![base_expr]));
            }
        }

        // Handle powers: x^2 (right-to-left for right associativity)
        if let Some(pow_pos) = self.find_operator_right_to_left(input, '^') {
            let base = self.parse(&input[..pow_pos])?;
            let exp = self.parse(&input[pow_pos + 1..])?;
            return Ok(Expression::pow(base, exp));
        }

        // Handle implicit multiplication: 2x, 3sin(x), 2π (Symbolica-inspired)
        if let Some(pos) = self.find_implicit_multiplication(input) {
            let left = self.parse(&input[..pos])?;
            let right = self.parse(&input[pos..])?;
            return Ok(Expression::mul(vec![left, right]));
        }

        // Handle number-constant multiplication: 2π, 3e, 4i (Enhanced Symbolica pattern)
        if input.len() > 1 {
            for (i, ch) in input.char_indices() {
                if matches!(ch, 'π' | '∞' | 'e' | 'i') && i > 0 {
                    let number_part = &input[..i];
                    let constant_part = &input[i..];

                    // Only proceed if the number part looks like a number
                    if number_part
                        .chars()
                        .all(|c| c.is_ascii_digit() || c == '.' || c == '-')
                    {
                        if let (Ok(num_expr), Ok(const_expr)) =
                            (self.parse(number_part), self.parse(constant_part))
                        {
                            return Ok(Expression::mul(vec![num_expr, const_expr]));
                        }
                    }
                }
            }
        }

        Err(ParseError::InvalidSyntax(format!(
            "Cannot parse: {}",
            input
        )))
    }

    /// Check if a string looks like a simple expression (number, symbol, or basic combination)
    fn looks_like_simple_expression(&self, input: &str) -> bool {
        if input.is_empty() {
            return false;
        }

        // Allow numbers, symbols, and basic operators
        input
            .chars()
            .all(|c| c.is_alphanumeric() || "._+-*/^()".contains(c))
    }

    /// Check if parentheses are balanced in the string
    fn is_balanced_parentheses(&self, s: &str) -> bool {
        let mut count = 0;
        for c in s.chars() {
            match c {
                '(' => count += 1,
                ')' => {
                    count -= 1;
                    if count < 0 {
                        return false;
                    }
                }
                _ => {}
            }
        }
        count == 0
    }

    /// Find operator position, respecting parentheses (left-to-right)
    fn find_operator(&self, input: &str, op: char) -> Option<usize> {
        let mut paren_count = 0;
        for (i, c) in input.char_indices().rev() {
            match c {
                ')' => paren_count += 1,
                '(' => paren_count -= 1,
                _ if c == op && paren_count == 0 => return Some(i),
                _ => {}
            }
        }
        None
    }

    /// Find operator position, respecting parentheses (right-to-left for right associativity)
    fn find_operator_right_to_left(&self, input: &str, op: char) -> Option<usize> {
        let mut paren_count = 0;
        for (i, c) in input.char_indices() {
            match c {
                '(' => paren_count += 1,
                ')' => paren_count -= 1,
                _ if c == op && paren_count == 0 => return Some(i),
                _ => {}
            }
        }
        None
    }

    /// Split function arguments respecting nested parentheses
    fn split_function_args(&self, args_str: &str) -> Result<Vec<String>, ParseError> {
        let mut args = Vec::new();
        let mut current_arg = String::new();
        let mut paren_count = 0;

        for c in args_str.chars() {
            match c {
                '(' => {
                    paren_count += 1;
                    current_arg.push(c);
                }
                ')' => {
                    paren_count -= 1;
                    current_arg.push(c);
                }
                ',' if paren_count == 0 => {
                    args.push(current_arg.trim().to_string());
                    current_arg.clear();
                }
                _ => current_arg.push(c),
            }
        }

        if !current_arg.trim().is_empty() {
            args.push(current_arg.trim().to_string());
        }

        if paren_count != 0 {
            return Err(ParseError::UnbalancedParentheses);
        }

        Ok(args)
    }

    /// Find position for implicit multiplication (e.g., 2x, 3sin)
    fn find_implicit_multiplication(&self, input: &str) -> Option<usize> {
        for (i, c) in input.char_indices() {
            if i > 0 && c.is_alphabetic() {
                let prev_char = input.chars().nth(i - 1).unwrap();
                if prev_char.is_numeric() || prev_char == ')' {
                    return Some(i);
                }
            }
        }
        None
    }
}
