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

        // Fall back to simple parser for basic expressions (with Symbolica-inspired error handling)
        if cleaned.is_empty() {
            return Err(ParseError::EmptyInput);
        }

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

        // Convert LaTeX braces in powers and subscripts to parentheses for simple parser
        // x^{2} -> x^(2), y^{-1} -> y^(-1), \int_{0} -> \int_(0)
        result = self.convert_power_braces(&result);
        result = self.convert_subscript_braces(&result);

        // Remove spaces
        result.replace(" ", "")
    }

    /// Convert LaTeX power braces to parentheses that SimpleParser can handle
    fn convert_power_braces(&self, input: &str) -> String {
        let mut result = String::new();
        let chars: Vec<char> = input.chars().collect();
        let mut i = 0;

        while i < chars.len() {
            if i + 1 < chars.len() && chars[i] == '^' && chars[i + 1] == '{' {
                result.push('^');
                result.push('(');
                i += 2; // Skip '^{'

                // Find matching closing brace
                let mut brace_count = 1;
                while i < chars.len() && brace_count > 0 {
                    if chars[i] == '{' {
                        brace_count += 1;
                    } else if chars[i] == '}' {
                        brace_count -= 1;
                    }

                    if brace_count > 0 {
                        result.push(chars[i]);
                    }
                    i += 1;
                }
                result.push(')');
            } else {
                result.push(chars[i]);
                i += 1;
            }
        }

        result
    }

    /// Convert LaTeX subscript braces to parentheses that SimpleParser can handle
    fn convert_subscript_braces(&self, input: &str) -> String {
        let mut result = String::new();
        let chars: Vec<char> = input.chars().collect();
        let mut i = 0;

        while i < chars.len() {
            if i + 1 < chars.len() && chars[i] == '_' && chars[i + 1] == '{' {
                result.push('_');
                result.push('(');
                i += 2; // Skip '_{'

                // Find matching closing brace
                let mut brace_count = 1;
                while i < chars.len() && brace_count > 0 {
                    if chars[i] == '{' {
                        brace_count += 1;
                    } else if chars[i] == '}' {
                        brace_count -= 1;
                    }

                    if brace_count > 0 {
                        result.push(chars[i]);
                    }
                    i += 1;
                }
                result.push(')');
            } else {
                result.push(chars[i]);
                i += 1;
            }
        }

        result
    }

    /// Parse subscript or superscript content, handling both braced and unbraced formats
    ///
    /// Returns (parsed_expression, remaining_text)
    fn parse_subscript_or_superscript(
        &mut self,
        input: &str,
    ) -> Result<(Expression, String), ParseError> {
        if input.starts_with('{') {
            // Braced format: {expr}
            let mut brace_count = 1;
            let mut end_pos = 0;

            for (i, ch) in input.chars().enumerate().skip(1) {
                match ch {
                    '{' => brace_count += 1,
                    '}' => {
                        brace_count -= 1;
                        if brace_count == 0 {
                            end_pos = i;
                            break;
                        }
                    }
                    _ => {}
                }
            }

            if brace_count != 0 {
                return Err(ParseError::SyntaxError("Unmatched braces".to_string()));
            }

            let content = &input[1..end_pos];
            let expr = self.parse(content)?;
            let remaining = input[end_pos + 1..].trim().to_string();
            Ok((expr, remaining))
        } else {
            // Unbraced format: expr (until space or special character)
            let end_pos = input.find(' ').unwrap_or(input.len());
            let content = &input[..end_pos];
            let expr = self.parse(content)?;
            let remaining = input[end_pos..].trim().to_string();
            Ok((expr, remaining))
        }
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

        // Handle special functions first - \Gamma(x)
        if input.starts_with("\\Gamma(") {
            let after_gamma = &input[7..]; // Skip "\\Gamma(" (7 chars)
            if let Some(paren_pos) = after_gamma.find(')') {
                let arg_str = &after_gamma[..paren_pos];
                // Use SimpleParser directly to avoid recursion issues
                let mut simple_parser = SimpleParser::new();
                match simple_parser.parse(arg_str) {
                    Ok(arg_expr) => return Ok(Some(Expression::function("gamma", vec![arg_expr]))),
                    Err(_) => {
                        // If argument parsing fails, fall through to other methods
                    }
                }
            }
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

        // Handle \begin{cases} piecewise functions (Symbolica-inspired)
        if input.starts_with("\\begin{cases}") {
            return self.parse_piecewise(input).map(Some);
        }

        // Handle matrices
        if input.starts_with("\\begin{") {
            return self.parse_matrix(input).map(Some);
        }

        // Handle sets
        if input.starts_with("\\{") {
            return self.parse_set(input).map(Some);
        }

        // Handle \left(...\right) delimiters (Symbolica-inspired delimiter stripping)
        if input.starts_with("\\left(") && input.contains("\\right)") {
            let start_pos = 6; // Length of "\\left("
            if let Some(end_pos) = input.find("\\right)") {
                let content = &input[start_pos..end_pos];
                let expr = self.parse(content)?;
                return Ok(Some(expr));
            }
        }

        // Handle other \left...\right pairs
        if input.starts_with("\\left") && input.contains("\\right") {
            // Find the delimiter after \left
            let after_left = &input[5..]; // Skip "\\left"
            if let Some(delim_char) = after_left.chars().next() {
                let start_pos = 6; // "\\left" + delimiter
                let right_pattern = format!("\\right{}", delim_char);
                if let Some(end_pos) = input.find(&right_pattern) {
                    let content = &input[start_pos..end_pos];
                    let expr = self.parse(content)?;
                    return Ok(Some(expr));
                }
            }
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

    /// Parse content within braces: "{content}" → content
    ///
    /// Returns (parsed_expression, remaining_text_after_closing_brace)
    fn parse_braced_content<'a>(
        &mut self,
        text: &'a str,
    ) -> Result<(Expression, &'a str), ParseError> {
        if !text.starts_with('{') {
            return Err(ParseError::SyntaxError(
                "Expected opening brace".to_string(),
            ));
        }

        // Symbolica-inspired robust bracket matching
        let mut brace_count = 1;
        let mut pos = 0;
        let chars: Vec<char> = text.chars().collect();

        for (i, &ch) in chars.iter().enumerate().skip(1) {
            match ch {
                '{' => brace_count += 1,
                '}' => {
                    brace_count -= 1;
                    if brace_count == 0 {
                        pos = i;
                        break;
                    }
                }
                _ => {}
            }
        }

        if brace_count != 0 {
            return Err(ParseError::SyntaxError("Unmatched braces".to_string()));
        }

        let content = &text[1..pos];
        let remaining = &text[pos + 1..];

        let expr = self.parse(content)?;
        Ok((expr, remaining))
    }

    /// Parse subscript or superscript with robust brace handling (Symbolica-inspired)
    fn parse_script_content<'a>(
        &mut self,
        text: &'a str,
    ) -> Result<(Expression, &'a str), ParseError> {
        if text.starts_with('{') {
            // Braced format: {content}
            self.parse_braced_content(text)
        } else {
            // Simple format: content (until space or special char)
            let end_pos = text.find(' ').unwrap_or(text.len());
            let content = &text[..end_pos];
            let remaining = &text[end_pos..];

            let expr = self.parse(content)?;
            Ok((expr, remaining))
        }
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
        // Handle function powers first: \sin^2(x), \sin^{2}(x) (Symbolica-inspired)
        for (latex_pattern, func_name) in LATEX_SIMPLE_FUNCTIONS {
            if input.starts_with(latex_pattern) {
                let after_func = &input[latex_pattern.len()..];

                // Check for function powers: \sin^2(x) or \sin^{2}(x)
                if after_func.starts_with("^") {
                    let after_caret = &after_func[1..];

                    // Handle both \sin^{2}(x) and \sin^2(x) formats
                    let (power_expr, remaining) = if after_caret.starts_with('{') {
                        // Braced power: \sin^{2}(x)
                        self.parse_script_content(after_caret)?
                    } else {
                        // Simple power: \sin^2(x) - find where power ends
                        if let Some(paren_pos) = after_caret.find('(') {
                            let power_str = &after_caret[..paren_pos];
                            let remaining = &after_caret[paren_pos..];
                            let power_expr = self.parse(power_str)?;
                            (power_expr, remaining)
                        } else {
                            continue; // Skip if no parentheses found
                        }
                    };

                    // Parse the function arguments
                    if remaining.starts_with('(') && remaining.ends_with(')') {
                        let args_str = &remaining[1..remaining.len() - 1];
                        let arg_expr = self.parse(args_str)?;
                        let func_expr = Expression::function(*func_name, vec![arg_expr]);
                        return Ok(Some(Expression::pow(func_expr, power_expr)));
                    }
                }

                // Regular parenthesized functions
                if *func_name == "gamma" {
                    // Simple approach for Gamma: find the closing parenthesis
                    if let Some(paren_pos) = after_func.find(')') {
                        let arg_str = &after_func[..paren_pos];
                        let arg_expr = self.parse(arg_str)?;
                        return Ok(Some(Expression::function(*func_name, vec![arg_expr])));
                    }
                } else {
                    // Use existing logic for other functions
                    let arg_end = self.find_matching_paren(after_func, 0)?;
                    let arg_str = &after_func[..arg_end];
                    let arg_expr = self.parse(arg_str)?;
                    return Ok(Some(Expression::function(*func_name, vec![arg_expr])));
                }
            }
        }

        // Try space-separated functions: \sin x, \sin^2 x (Symbolica-inspired)
        for (latex_pattern, func_name) in LATEX_SPACE_FUNCTIONS {
            if input.starts_with(latex_pattern) {
                let after_func = &input[latex_pattern.len()..];

                // Handle function powers: \sin^2(x)
                if after_func.starts_with('^') {
                    let after_caret = &after_func[1..];
                    if let Some(paren_pos) = after_caret.find('(') {
                        let power_str = &after_caret[..paren_pos];
                        let after_paren = &after_caret[paren_pos..];

                        if let Some(close_paren) = after_paren.find(')') {
                            let arg_str = &after_paren[1..close_paren];
                            let power_expr = self.parse(power_str)?;
                            let arg_expr = self.parse(arg_str)?;
                            let func_expr = Expression::function(*func_name, vec![arg_expr]);
                            return Ok(Some(Expression::pow(func_expr, power_expr)));
                        }
                    }
                }

                // Regular space-separated functions: \sin x
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

    /// Parse integral bounds: _a^b or _{a}^{b} returns (Some((a, b)), remaining_string)
    ///
    /// Handles both simple (_0^1) and braced (_{0}^{1}) formats for roundtrip consistency.
    fn parse_integral_bounds<'a>(
        &mut self,
        input: &'a str,
    ) -> Result<(Option<(Box<Expression>, Box<Expression>)>, &'a str), ParseError> {
        if !input.starts_with('_') {
            return Ok((None, input));
        }

        let after_underscore = &input[1..];

        // Use Symbolica-inspired robust script parsing for lower bound
        let (lower_expr, after_lower_str) = if after_underscore.starts_with('{') {
            self.parse_script_content(after_underscore)?
        } else {
            // For simple format, need to find the ^ to determine bound end
            let caret_pos = after_underscore.find('^').ok_or_else(|| {
                ParseError::SyntaxError("Missing upper bound in integral".to_string())
            })?;
            let lower_str = &after_underscore[..caret_pos];
            let lower_expr = self.parse(lower_str)?;
            (lower_expr, &after_underscore[caret_pos..])
        };
        let after_lower = after_lower_str;

        // Parse upper bound
        if !after_lower.starts_with('^') {
            return Err(ParseError::SyntaxError(
                "Missing ^ in integral bounds".to_string(),
            ));
        }

        let after_caret = &after_lower[1..]; // Skip '^'

        // Use Symbolica-inspired robust script parsing
        let (upper_expr, remaining) = self.parse_script_content(after_caret)?;

        Ok((
            Some((Box::new(lower_expr), Box::new(upper_expr))),
            remaining,
        ))
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

        // Parse end and expression from "^n i^2" or "^{n} i^{2}"
        if !after_spec.starts_with('^') {
            return Err(ParseError::SyntaxError(
                "Missing summation upper bound".to_string(),
            ));
        }

        let after_caret = &after_spec[1..]; // Skip '^'

        // Parse upper bound (simplified approach for now)
        let end_pos = after_caret.find(' ').unwrap_or(after_caret.len());
        let end_str = &after_caret[..end_pos];
        let expression_part = &after_caret[end_pos..].trim();

        let end_expr = self.parse(end_str)?;

        let expression = self.parse(expression_part)?;

        Ok(Expression::sum(expression, variable, start_expr, end_expr))
    }

    /// Parse LaTeX piecewise function: \begin{cases} ... \end{cases}
    fn parse_piecewise(&mut self, input: &str) -> Result<Expression, ParseError> {
        if !input.starts_with("\\begin{cases}") || !input.contains("\\end{cases}") {
            return Err(ParseError::SyntaxError(
                "Invalid piecewise syntax".to_string(),
            ));
        }

        let start_pos = 13; // Length of "\\begin{cases}"
        let end_pos = input
            .find("\\end{cases}")
            .ok_or_else(|| ParseError::SyntaxError("Missing \\end{cases}".to_string()))?;

        let content = &input[start_pos..end_pos].trim();

        // Simple piecewise parsing: split by \\ and parse each case
        let cases_str = content.split("\\\\").collect::<Vec<_>>();
        let mut pieces = Vec::new();

        for case_str in cases_str {
            if case_str.trim().is_empty() {
                continue;
            }

            // Parse: "x & \text{if } x > 0" → (condition, expression)
            if let Some(amp_pos) = case_str.find('&') {
                let expr_str = case_str[..amp_pos].trim();
                let condition_str = case_str[amp_pos + 1..].trim();

                // Skip \text{if} and parse the condition
                let clean_condition = condition_str
                    .replace("\\text{if }", "")
                    .replace("\\text{if}", "")
                    .trim()
                    .to_string();

                let expr = self.parse(expr_str)?;
                let condition = self.parse(&clean_condition)?;
                pieces.push((condition, expr));
            }
        }

        Ok(Expression::piecewise(pieces, None))
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
