use super::LaTeXParser;
use crate::core::{Expression, Number};
use crate::parser::constants::*;
use crate::parser::simple::SimpleParser;
use crate::parser::ParseError;
use crate::utils::get_or_create_symbol;

impl LaTeXParser {
    /// Parse LaTeX expression with comprehensive LaTeX support
    /// # Examples
    /// ```rust
    /// use mathhook_core::parser::latex::LaTeXParser;
    /// let mut parser = LaTeXParser::new();
    /// let expr = parser.parse("\\frac{x^2}{2}").unwrap();
    /// assert_eq!(expr, Expression::fraction(Expression::symbol("x"), Expression::symbol("2")));
    /// ```
    pub fn parse(&mut self, input: &str) -> Result<Expression, ParseError> {
        if input.is_empty() {
            return Err(ParseError::EmptyInput);
        }

        // Handle LaTeX commands first (before preprocessing to catch \pi, etc.)
        if let Some(expr) = self.parse_commands(input)? {
            return Ok(expr);
        }

        let cleaned = self.preprocess(input);

        // Fall back to simple parser for basic expressions
        let mut simple_parser = SimpleParser::new();
        simple_parser.parse(&cleaned)
    }

    /// Preprocess LaTeX input
    /// # Examples
    /// ```rust
    /// use mathhook_core::parser::latex::LaTeXParser;
    /// let mut parser = LaTeXParser::new();
    /// let expr = parser.preprocess("\\frac{x^2}{2}").unwrap();
    /// assert_eq!(expr, "frac{x^2}{2}");
    /// ```
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
        // Handle LaTeX constants before preprocessing (to catch \pi before it becomes π)
        if let Some(const_result) = self.parse_constants(input)? {
            return Ok(Some(const_result));
        }

        // Handle derivatives BEFORE general fractions (to catch \frac{d}{dx})
        if input.starts_with("\\frac{d}") {
            return self.parse_derivative(input).map(Some);
        }

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

        // Handle calculus commands
        if let Some(calc_result) = self.parse_calculus(input)? {
            return Ok(Some(calc_result));
        }

        // Handle matrices
        if input.starts_with("\\begin{") {
            return self.parse_matrix(input).map(Some);
        }

        // Handle sets
        if input.starts_with("\\{") {
            return self.parse_set(input).map(Some);
        }

        Ok(None)
    }

    /// Parse LaTeX constants like \pi, \infty, etc.
    fn parse_constants(&self, input: &str) -> Result<Option<Expression>, ParseError> {
        use crate::core::MathConstant;

        // Check for LaTeX constants
        if input == "\\pi" {
            return Ok(Some(Expression::constant(MathConstant::Pi)));
        }
        if input == "\\infty" {
            return Ok(Some(Expression::constant(MathConstant::Infinity)));
        }
        if input == "\\e" {
            return Ok(Some(Expression::constant(MathConstant::E)));
        }

        Ok(None)
    }

    /// Parse LaTeX calculus commands like \int, \lim, etc.
    fn parse_calculus(&mut self, input: &str) -> Result<Option<Expression>, ParseError> {
        // Handle \int expressions
        if input.starts_with("\\int") {
            return self.parse_integral(input).map(Some);
        }

        // Handle \lim expressions
        if input.starts_with("\\lim") {
            return self.parse_limit(input).map(Some);
        }

        // Handle \sum expressions
        if input.starts_with("\\sum") {
            return self.parse_sum(input).map(Some);
        }

        // Handle \frac{d}{dx} derivatives
        if input.starts_with("\\frac{d}") {
            return self.parse_derivative(input).map(Some);
        }

        Ok(None)
    }

    /// Parse LaTeX fraction: \frac{numerator}{denominator}
    /// # Examples
    /// ```rust
    /// use mathhook_core::parser::latex::LaTeXParser;
    /// let mut parser = LaTeXParser::new();
    /// let expr = parser.parse_fraction("\\frac{x^2}{2}").unwrap();
    /// assert_eq!(expr, Expression::fraction(Expression::symbol("x"), Expression::symbol("2")));
    /// ```
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
    /// # Examples
    /// ```rust
    /// use mathhook_core::parser::latex::LaTeXParser;
    /// let mut parser = LaTeXParser::new();
    /// let (numerator, denominator) = parser.extract_fraction_parts("\\frac{x^2}{2}").unwrap();
    /// assert_eq!(numerator, "x^2");
    /// assert_eq!(denominator, "2");
    /// ```
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
    /// # Examples
    /// ```rust
    /// use mathhook_core::parser::latex::LaTeXParser;
    /// let mut parser = LaTeXParser::new();
    /// let brace_end = parser.find_matching_brace("\\frac{x^2}{2}", 0).unwrap();
    /// assert_eq!(brace_end, 2);
    /// ```
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

    /// Parse LaTeX functions like \sin(x), \cos(x), or \sin x
    /// # Examples
    /// ```rust
    /// use mathhook_core::parser::latex::LaTeXParser;
    /// let mut parser = LaTeXParser::new();
    /// let expr = parser.parse_function("\\sin(x)").unwrap();
    /// assert_eq!(expr, Expression::function("sin", vec![Expression::symbol("x")]));
    /// ```
    fn parse_function(&mut self, input: &str) -> Result<Option<Expression>, ParseError> {
        // Try parenthesized functions first: \sin(x)
        for (latex_pattern, func_name) in LATEX_SIMPLE_FUNCTIONS {
            if input.starts_with(latex_pattern) {
                let after_func = &input[latex_pattern.len()..];
                let arg_end = self.find_matching_paren(after_func, 0)?;
                let arg_str = &after_func[..arg_end];
                let arg_expr = self.parse(arg_str)?;
                return Ok(Some(Expression::function(*func_name, vec![arg_expr])));
            }
        }

        // Try space-separated functions: \sin x
        for (latex_pattern, func_name) in LATEX_SPACE_FUNCTIONS {
            if input.starts_with(latex_pattern) {
                let after_func = &input[latex_pattern.len()..];
                // Take the next token as the argument
                let tokens: Vec<&str> = after_func.trim().split_whitespace().collect();
                if !tokens.is_empty() {
                    let arg_expr = self.parse(tokens[0])?;
                    return Ok(Some(Expression::function(*func_name, vec![arg_expr])));
                }
            }
        }

        Ok(None)
    }

    /// Find matching closing parenthesis
    /// # Examples
    /// ```rust
    /// use mathhook_core::parser::latex::LaTeXParser;
    /// let mut parser = LaTeXParser::new();
    /// let paren_end = parser.find_matching_paren("\\sin(x)", 0).unwrap();
    /// assert_eq!(paren_end, 2);
    /// ```
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
    /// # Examples
    /// ```rust
    /// use mathhook_core::parser::latex::LaTeXParser;
    /// let mut parser = LaTeXParser::new();
    /// let expr = parser.parse_sqrt("\\sqrt{x}").unwrap();
    /// assert_eq!(expr, Expression::sqrt(Expression::symbol("x")));
    /// ```
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

    /// Parse LaTeX integral: \int f dx or \int_a^b f dx
    /// # Examples
    /// ```rust
    /// use mathhook_core::parser::latex::LaTeXParser;
    /// let mut parser = LaTeXParser::new();
    /// let expr = parser.parse_integral("\\int f dx").unwrap();
    /// assert_eq!(expr, Expression::integral(Expression::symbol("f"), Expression::symbol("x")));
    /// ```
    fn parse_integral(&mut self, input: &str) -> Result<Expression, ParseError> {
        if !input.starts_with("\\int") {
            return Err(ParseError::SyntaxError("Not an integral".to_string()));
        }

        let after_int = &input[4..]; // Skip "\\int"

        // Check for bounds: \int_a^b
        let (bounds, remaining) = if after_int.starts_with('_') {
            self.parse_integral_bounds(after_int)?
        } else {
            (None, after_int)
        };

        // Parse the integrand and variable
        // Pattern: "f dx" where f is the integrand and x is the variable
        let parts: Vec<&str> = remaining.trim().split_whitespace().collect();
        if parts.len() < 2 {
            return Err(ParseError::SyntaxError(
                "Invalid integral format".to_string(),
            ));
        }

        // Last part should be "dx", "dy", etc.
        let var_part = parts.last().unwrap();
        if !var_part.starts_with('d') || var_part.len() != 2 {
            return Err(ParseError::SyntaxError("Invalid differential".to_string()));
        }

        let variable = get_or_create_symbol(&var_part[1..]);

        // Everything except the last part is the integrand
        let integrand_str = parts[..parts.len() - 1].join(" ");
        let integrand = self.parse(&integrand_str)?;

        if let Some((start, end)) = bounds {
            Ok(Expression::definite_integral(
                integrand, variable, *start, *end,
            ))
        } else {
            Ok(Expression::integral(integrand, variable))
        }
    }

    /// Parse integral bounds: _a^b returns (Some((a, b)), remaining_string)
    /// # Examples
    /// ```rust
    /// use mathhook_core::parser::latex::LaTeXParser;
    /// let mut parser = LaTeXParser::new();
    /// let (bounds, remaining) = parser.parse_integral_bounds("\\int_0^1 f dx").unwrap();
    /// assert_eq!(bounds, Some((Expression::symbol("0"), Expression::symbol("1"))));
    /// assert_eq!(remaining, "f dx");
    /// ```
    fn parse_integral_bounds<'a>(
        &mut self,
        input: &'a str,
    ) -> Result<(Option<(Box<Expression>, Box<Expression>)>, &'a str), ParseError> {
        if !input.starts_with('_') {
            return Ok((None, input));
        }

        // For now, implement simplified bounds parsing
        // Full implementation would need proper subscript/superscript parsing

        // Look for pattern _a^b where a and b can be single characters or {expressions}
        let after_underscore = &input[1..];

        // Simple case: _0^1 (single characters)
        if let Some(caret_pos) = after_underscore.find('^') {
            let lower_str = &after_underscore[..caret_pos];
            let after_caret = &after_underscore[caret_pos + 1..];

            // Find end of upper bound (space or end of string)
            let upper_end = after_caret.find(' ').unwrap_or(after_caret.len());
            let upper_str = &after_caret[..upper_end];
            let remaining = &after_caret[upper_end..];

            let lower_expr = self.parse(lower_str)?;
            let upper_expr = self.parse(upper_str)?;

            Ok((
                Some((Box::new(lower_expr), Box::new(upper_expr))),
                remaining,
            ))
        } else {
            Ok((None, input))
        }
    }

    /// Parse LaTeX limit: \lim_{x \to a} f
    /// # Examples
    /// ```rust
    /// use mathhook_core::parser::latex::LaTeXParser;
    /// let mut parser = LaTeXParser::new();
    /// let expr = parser.parse_limit("\\lim_{x \\to a} f").unwrap();
    /// assert_eq!(expr, Expression::limit(Expression::symbol("f"), Expression::symbol("x"), Expression::symbol("a")));
    /// ```
    fn parse_limit(&mut self, input: &str) -> Result<Expression, ParseError> {
        if !input.starts_with("\\lim") {
            return Err(ParseError::SyntaxError("Not a limit".to_string()));
        }

        // Pattern: \lim_{x \to a} f
        let after_lim = &input[4..]; // Skip "\\lim"

        if !after_lim.starts_with("_{") {
            return Err(ParseError::SyntaxError(
                "Missing limit specification".to_string(),
            ));
        }

        // Find the closing brace for the limit specification
        let after_brace = &after_lim[2..]; // Skip "_{"
        let brace_end = self.find_matching_brace(after_brace, 0)?;

        let limit_spec = &after_brace[..brace_end];
        let expression_part = &after_brace[brace_end + 1..].trim();

        // Parse "x \to a" pattern
        let to_parts: Vec<&str> = limit_spec.split("\\to").collect();
        if to_parts.len() != 2 {
            return Err(ParseError::SyntaxError(
                "Invalid limit specification".to_string(),
            ));
        }

        let variable = get_or_create_symbol(to_parts[0].trim());
        let approach = self.parse(to_parts[1].trim())?;
        let expression = self.parse(expression_part)?;

        Ok(Expression::limit(expression, variable, approach))
    }

    /// Parse LaTeX summation: \sum_{i=1}^n f
    /// # Examples
    /// ```rust
    /// use mathhook_core::parser::latex::LaTeXParser;
    /// let mut parser = LaTeXParser::new();
    /// let expr = parser.parse_sum("\\sum_{i=1}^n f").unwrap();
    /// assert_eq!(expr, Expression::sum(Expression::symbol("f"), Expression::symbol("i"), Expression::symbol("1"), Expression::symbol("n")));
    /// ```
    fn parse_sum(&mut self, input: &str) -> Result<Expression, ParseError> {
        if !input.starts_with("\\sum") {
            return Err(ParseError::SyntaxError("Not a summation".to_string()));
        }

        let after_sum = &input[4..]; // Skip "\\sum"

        if !after_sum.starts_with("_{") {
            return Err(ParseError::SyntaxError(
                "Missing summation specification".to_string(),
            ));
        }

        let after_brace = &after_sum[2..]; // Skip "_{"
        let brace_end = self.find_matching_brace(after_brace, 0)?;

        let sum_spec = &after_brace[..brace_end]; // "i=1"
        let after_spec = &after_brace[brace_end + 1..]; // "^n i^2"

        // Parse variable and start from "i=1"
        let eq_parts: Vec<&str> = sum_spec.split('=').collect();
        if eq_parts.len() != 2 {
            return Err(ParseError::SyntaxError(
                "Invalid summation specification".to_string(),
            ));
        }

        let variable = get_or_create_symbol(eq_parts[0].trim());
        let start_expr = self.parse(eq_parts[1].trim())?;

        // Parse end and expression from "^n i^2"
        if !after_spec.starts_with('^') {
            return Err(ParseError::SyntaxError(
                "Missing summation upper bound".to_string(),
            ));
        }

        let after_caret = &after_spec[1..]; // "n i^2"
        let end_pos = after_caret.find(' ').unwrap_or(after_caret.len());
        let end_str = &after_caret[..end_pos]; // "n"
        let expression_part = &after_caret[end_pos..].trim(); // "i^2"

        let end_expr = self.parse(end_str)?;
        let expression = self.parse(expression_part)?;

        Ok(Expression::sum(expression, variable, start_expr, end_expr))
    }

    /// Parse LaTeX derivative: \frac{d}{dx} f
    /// # Examples
    /// ```rust
    /// use mathhook_core::parser::latex::LaTeXParser;
    /// let mut parser = LaTeXParser::new();
    /// let expr = parser.parse_derivative("\\frac{d}{dx} f").unwrap();
    /// assert_eq!(expr, Expression::derivative(Expression::symbol("f"), Expression::symbol("x"), Expression::symbol("1")));
    /// ```
    fn parse_derivative(&mut self, input: &str) -> Result<Expression, ParseError> {
        // Pattern: \frac{d}{dx} f or \frac{d^n}{dx^n} f
        if !input.starts_with("\\frac{d") {
            return Err(ParseError::SyntaxError("Not a derivative".to_string()));
        }

        // For now, handle simple case: \frac{d}{dx} f
        if input.starts_with("\\frac{d}{d") {
            // Find the variable after "\\frac{d}{d"
            let after_dd = &input[10..]; // Skip "\\frac{d}{d"

            // Find the variable (should be single character before "}")
            if let Some(close_pos) = after_dd.find('}') {
                let var_name = &after_dd[..close_pos];
                let variable = get_or_create_symbol(var_name);

                // Find the expression after the closing brace
                let after_frac = &after_dd[close_pos + 1..].trim();
                let expression = self.parse(after_frac)?;

                Ok(Expression::derivative(expression, variable, 1))
            } else {
                Err(ParseError::SyntaxError("Malformed derivative".to_string()))
            }
        } else {
            Err(ParseError::SyntaxError(
                "Unsupported derivative format".to_string(),
            ))
        }
    }

    /// Parse LaTeX matrix: \begin{pmatrix} 1 & 2 \\ 3 & 4 \end{pmatrix}
    /// # Examples
    /// ```rust
    /// use mathhook_core::parser::latex::LaTeXParser;
    /// let mut parser = LaTeXParser::new();
    /// let expr = parser.parse_matrix("\\begin{pmatrix} 1 & 2 \\\\ 3 & 4 \\end{pmatrix}").unwrap();
    /// assert_eq!(expr, Expression::matrix(vec![vec![Expression::integer(1), Expression::integer(2)], vec![Expression::integer(3), Expression::integer(4)]]));
    /// ```
    fn parse_matrix(&mut self, input: &str) -> Result<Expression, ParseError> {
        if !input.starts_with("\\begin{") {
            return Err(ParseError::SyntaxError("Not a matrix".to_string()));
        }

        // Find the matrix type (pmatrix, bmatrix, etc.)
        let after_begin = &input[7..]; // Skip "\\begin{"
        let type_end = after_begin
            .find('}')
            .ok_or_else(|| ParseError::SyntaxError("Malformed matrix begin".to_string()))?;

        let matrix_type = &after_begin[..type_end];
        let after_type = &after_begin[type_end + 1..];

        // Find the end of the matrix
        let end_pattern = format!("\\end{{{}}}", matrix_type);
        let content_end = after_type
            .find(&end_pattern)
            .ok_or_else(|| ParseError::SyntaxError("Missing matrix end".to_string()))?;

        let matrix_content = &after_type[..content_end].trim();

        // Parse matrix rows separated by \\
        let rows: Vec<&str> = matrix_content.split("\\\\").collect();
        let mut matrix_rows = Vec::new();

        for row_str in rows {
            // Parse elements separated by &
            let elements: Vec<&str> = row_str.split('&').collect();
            let mut row_elements = Vec::new();

            for element_str in elements {
                let element_expr = self.parse(element_str.trim())?;
                row_elements.push(element_expr);
            }

            if !row_elements.is_empty() {
                matrix_rows.push(row_elements);
            }
        }

        Ok(Expression::matrix(matrix_rows))
    }

    /// Parse LaTeX set: \{1, 2, 3\}
    /// # Examples
    /// ```rust
    /// use mathhook_core::parser::latex::LaTeXParser;
    /// let mut parser = LaTeXParser::new();
    /// let expr = parser.parse_set("\\{1, 2, 3\\}").unwrap();
    /// assert_eq!(expr, Expression::set(vec![Expression::integer(1), Expression::integer(2), Expression::integer(3)]));
    /// ```
    fn parse_set(&mut self, input: &str) -> Result<Expression, ParseError> {
        if !input.starts_with("\\{") || !input.ends_with("\\}") {
            return Err(ParseError::SyntaxError("Not a set".to_string()));
        }

        let content = &input[2..input.len() - 2]; // Remove \{ and \}

        if content.trim().is_empty() {
            return Ok(Expression::set(vec![])); // Empty set
        }

        // Parse elements separated by commas
        let element_strings: Vec<&str> = content.split(',').collect();
        let mut elements = Vec::new();

        for element_str in element_strings {
            let element_expr = self.parse(element_str.trim())?;
            elements.push(element_expr);
        }

        Ok(Expression::set(elements))
    }
}
