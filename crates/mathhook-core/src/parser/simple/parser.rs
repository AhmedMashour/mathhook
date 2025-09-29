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

        // Handle symbols: x, alpha, var_1
        if input.chars().all(|c| c.is_alphanumeric() || c == '_')
            && input.chars().next().unwrap().is_alphabetic()
        {
            return Ok(Expression::Symbol(get_or_create_symbol(input)));
        }

        // Handle parentheses: (x + 1) - do this early to handle nested expressions
        if input.starts_with('(') && input.ends_with(')') && self.is_balanced_parentheses(input) {
            return self.parse(&input[1..input.len() - 1]);
        }

        // Handle function calls: sin(x), log(x, 2)
        if let Some(paren_pos) = input.find('(') {
            if input.ends_with(')') && self.is_balanced_parentheses(&input[paren_pos..]) {
                let func_name = &input[..paren_pos];
                let args_str = &input[paren_pos + 1..input.len() - 1];

                if func_name.chars().all(|c| c.is_alphabetic()) {
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

        // Handle powers: x^2 (right-to-left for right associativity)
        if let Some(pow_pos) = self.find_operator_right_to_left(input, '^') {
            let base = self.parse(&input[..pow_pos])?;
            let exp = self.parse(&input[pow_pos + 1..])?;
            return Ok(Expression::pow(base, exp));
        }

        // Handle implicit multiplication: 2x, 3sin(x)
        if let Some(pos) = self.find_implicit_multiplication(input) {
            let left = self.parse(&input[..pos])?;
            let right = self.parse(&input[pos..])?;
            return Ok(Expression::mul(vec![left, right]));
        }

        Err(ParseError::InvalidSyntax(format!(
            "Cannot parse: {}",
            input
        )))
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
