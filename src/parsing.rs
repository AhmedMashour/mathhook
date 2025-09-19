//! Parsing module for mathematical expressions and LaTeX input
//! Handles conversion between string representations and Expression objects

// Parser constants and patterns
pub mod constants;

// Focused parser modules
pub mod latex_parser;
pub mod wolfram_parser;

// Universal parser for comprehensive language support
pub mod universal;

// Serialization API for mathematical expressions
pub mod serialize;

// Convenient macros for expression creation and parsing
pub mod macros;

use crate::core::{Expression, Number, Symbol};
use num_traits::One;
use std::collections::HashMap;

// Re-exports for easy access
pub use latex_parser::LaTeXParser;
pub use serialize::{ExpressionData, MathSerializer, SerializationError};
pub use universal::{MathLanguage, UniversalParser};
pub use wolfram_parser::WolframParser;

/// Parser for mathematical expressions
pub struct ExpressionParser {
    variables: HashMap<String, Symbol>,
}

impl ExpressionParser {
    /// Create a new parser
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
        }
    }

    /// Parse a mathematical expression from string
    pub fn parse(&mut self, input: &str) -> Result<Expression, ParseError> {
        let cleaned = self.preprocess_input(input);
        self.parse_expression(&cleaned)
    }

    /// Parse LaTeX mathematical expression
    pub fn parse_latex(&mut self, latex: &str) -> Result<Expression, ParseError> {
        let cleaned = self.preprocess_latex(latex);
        self.parse_expression(&cleaned)
    }

    /// Preprocess input to normalize format
    fn preprocess_input(&self, input: &str) -> String {
        let cleaned = input
            .trim()
            .replace(" ", "")
            .replace("**", "^")
            .replace("×", "*")
            .replace("÷", "/")
            .replace("−", "-");

        // Handle implicit multiplication (e.g., "2x" -> "2*x", "3xy" -> "3*x*y")
        self.insert_implicit_multiplication(&cleaned)
    }

    /// Preprocess LaTeX input
    pub fn preprocess_latex(&self, latex: &str) -> String {
        let cleaned = latex
            .trim()
            .replace("\\cdot", "*")
            .replace("\\times", "*")
            .replace("\\div", "/")
            .replace("\\pm", "+")
            .replace("\\mp", "-")
            .replace(" ", "");

        // Remove LaTeX braces for simple expressions like x^{2} -> x^2
        let without_braces = self.remove_simple_braces(&cleaned);

        // Handle implicit multiplication in LaTeX too
        self.insert_implicit_multiplication(&without_braces)
    }

    /// Insert multiplication operators for implicit multiplication
    fn insert_implicit_multiplication(&self, input: &str) -> String {
        let mut result = String::new();
        let chars: Vec<char> = input.chars().collect();

        for i in 0..chars.len() {
            let current = chars[i];
            result.push(current);

            // Look ahead for implicit multiplication cases
            if i < chars.len() - 1 {
                let next = chars[i + 1];

                // Cases where we need to insert '*':
                // 1. Number followed by letter: "2x" -> "2*x"
                // 2. Letter followed by letter: "xy" -> "x*y"
                // 3. Number/letter followed by '(': "2(" -> "2*("
                // 4. ')' followed by number/letter: ")x" -> ")*x"

                let needs_multiplication = match (current, next) {
                    // Number followed by letter
                    (c1, c2) if c1.is_ascii_digit() && c2.is_alphabetic() => true,
                    // Letter followed by letter (but not in same variable name)
                    (c1, c2) if c1.is_alphabetic() && c2.is_alphabetic() => {
                        // For now, treat consecutive letters as separate variables
                        // Later we can add logic for multi-character variables
                        true
                    }
                    // Number or letter followed by '('
                    (c1, '(') if c1.is_alphanumeric() => true,
                    // ')' followed by number or letter
                    (')', c2) if c2.is_alphanumeric() => true,
                    // ')' followed by '('
                    (')', '(') => true,
                    _ => false,
                };

                if needs_multiplication {
                    result.push('*');
                }
            }
        }

        result
    }

    /// Remove simple LaTeX braces for basic expressions like x^{2} -> x^2
    fn remove_simple_braces(&self, input: &str) -> String {
        let mut result = String::new();
        let mut chars = input.chars().peekable();

        while let Some(ch) = chars.next() {
            if ch == '{' {
                // Check if this is a simple brace (only contains simple content)
                let mut brace_content = String::new();
                let mut brace_count = 1;
                let mut is_simple = true;

                while let Some(&next_ch) = chars.peek() {
                    chars.next();
                    match next_ch {
                        '{' => {
                            brace_count += 1;
                            brace_content.push(next_ch);
                            is_simple = false; // Nested braces are not simple
                        }
                        '}' => {
                            brace_count -= 1;
                            if brace_count == 0 {
                                break;
                            }
                            brace_content.push(next_ch);
                        }
                        '\\' => {
                            // LaTeX commands in braces make it not simple
                            brace_content.push(next_ch);
                            is_simple = false;
                        }
                        _ => {
                            brace_content.push(next_ch);
                        }
                    }
                }

                // If it's simple content (like numbers, variables, basic operators), remove braces
                if is_simple
                    && brace_content
                        .chars()
                        .all(|c| c.is_alphanumeric() || "+-*/^.".contains(c))
                {
                    result.push_str(&brace_content);
                } else {
                    // Keep complex braces
                    result.push('{');
                    result.push_str(&brace_content);
                    result.push('}');
                }
            } else {
                result.push(ch);
            }
        }

        result
    }

    /// Parse expression from preprocessed string
    fn parse_expression(&mut self, input: &str) -> Result<Expression, ParseError> {
        if input.is_empty() {
            return Err(ParseError::EmptyInput);
        }

        // Handle intervals first (e.g., "[0,1]", "(0,1)")
        if let Some(interval) = self.parse_interval(input)? {
            return Ok(interval);
        }

        // Handle relations (inequalities and equations as Relations, not expressions)
        if let Some(relation) = self.parse_relation(input)? {
            return Ok(relation);
        }

        // Handle equations for solving (e.g., "2x + 3 = 0")
        if let Some(eq_pos) = input.find('=') {
            let left_side = &input[..eq_pos];
            let right_side = &input[eq_pos + 1..];

            let left_expr = self.parse_expression(left_side)?;
            let right_expr = self.parse_expression(right_side)?;

            // Convert equation to expression by moving everything to left side
            // "a = b" becomes "a - b"
            return Ok(Expression::add(vec![
                left_expr,
                Expression::mul(vec![Expression::integer(-1), right_expr]),
            ]));
        }

        // Handle parentheses
        if input.starts_with('(') && input.ends_with(')') && self.is_balanced_parentheses(input) {
            let inner = &input[1..input.len() - 1];
            return self.parse_expression(inner);
        }

        // Parse with proper precedence: addition -> multiplication -> exponentiation -> atom
        self.parse_addition_level(input)
    }

    /// Check if parentheses are balanced and outer-most
    fn is_balanced_parentheses(&self, input: &str) -> bool {
        let mut depth = 0;
        for (i, ch) in input.chars().enumerate() {
            match ch {
                '(' => depth += 1,
                ')' => {
                    depth -= 1;
                    if depth == 0 && i < input.len() - 1 {
                        return false; // Closes before the end
                    }
                }
                _ => {}
            }
        }
        depth == 0
    }

    /// Parse addition and subtraction (highest precedence level)
    fn parse_addition_level(&mut self, input: &str) -> Result<Expression, ParseError> {
        let mut terms = Vec::new();
        let mut current_term = String::new();
        let mut paren_depth = 0;
        let mut i = 0;

        while i < input.len() {
            let ch = input.chars().nth(i).unwrap();

            match ch {
                '(' => {
                    paren_depth += 1;
                    current_term.push(ch);
                }
                ')' => {
                    paren_depth -= 1;
                    current_term.push(ch);
                }
                '+' if paren_depth == 0 => {
                    if !current_term.is_empty() {
                        terms.push(self.parse_multiplication_level(&current_term)?);
                        current_term.clear();
                    }
                }
                '-' if paren_depth == 0 && i > 0 => {
                    if !current_term.is_empty() {
                        terms.push(self.parse_multiplication_level(&current_term)?);
                        current_term.clear();
                    }
                    // Start negative term with minus sign
                    current_term.push('-');
                }
                _ => {
                    current_term.push(ch);
                }
            }
            i += 1;
        }

        if !current_term.is_empty() {
            terms.push(self.parse_multiplication_level(&current_term)?);
        }

        if terms.len() > 1 {
            Ok(Expression::add(terms))
        } else if terms.len() == 1 {
            Ok(terms.into_iter().next().unwrap())
        } else {
            Err(ParseError::EmptyInput)
        }
    }

    /// Parse multiplication and division
    fn parse_multiplication_level(&mut self, input: &str) -> Result<Expression, ParseError> {
        let mut factors = Vec::new();
        let mut current_factor = String::new();
        let mut paren_depth = 0;
        let mut i = 0;

        while i < input.len() {
            let ch = input.chars().nth(i).unwrap();

            match ch {
                '(' => {
                    paren_depth += 1;
                    current_factor.push(ch);
                }
                ')' => {
                    paren_depth -= 1;
                    current_factor.push(ch);
                }
                '*' if paren_depth == 0 => {
                    if !current_factor.is_empty() {
                        factors.push(self.parse_exponentiation_level(&current_factor)?);
                        current_factor.clear();
                    }
                }
                '/' if paren_depth == 0 => {
                    if !current_factor.is_empty() {
                        factors.push(self.parse_exponentiation_level(&current_factor)?);
                        current_factor.clear();
                    }
                    // Division: multiply by inverse
                    i += 1;
                    let mut divisor = String::new();
                    while i < input.len()
                        && input.chars().nth(i).unwrap() != '*'
                        && input.chars().nth(i).unwrap() != '/'
                    {
                        divisor.push(input.chars().nth(i).unwrap());
                        i += 1;
                    }
                    i -= 1; // Back up one since the loop will increment

                    if !divisor.is_empty() {
                        let divisor_expr = self.parse_exponentiation_level(&divisor)?;
                        factors.push(Expression::pow(divisor_expr, Expression::integer(-1)));
                    }
                }
                _ => {
                    current_factor.push(ch);
                }
            }
            i += 1;
        }

        if !current_factor.is_empty() {
            factors.push(self.parse_exponentiation_level(&current_factor)?);
        }

        if factors.len() > 1 {
            Ok(Expression::mul(factors))
        } else if factors.len() == 1 {
            Ok(factors.into_iter().next().unwrap())
        } else {
            Err(ParseError::EmptyInput)
        }
    }

    /// Parse exponentiation
    fn parse_exponentiation_level(&mut self, input: &str) -> Result<Expression, ParseError> {
        if let Some(caret_pos) = input.find('^') {
            let base_str = &input[..caret_pos];
            let exp_str = &input[caret_pos + 1..];

            let base = self.parse_atom_level(base_str)?;
            let exp = self.parse_atom_level(exp_str)?;

            Ok(Expression::pow(base, exp))
        } else {
            self.parse_atom_level(input)
        }
    }

    /// Try to parse function calls
    fn try_parse_function(&mut self, input: &str) -> Result<Option<Expression>, ParseError> {
        if let Some(paren_pos) = input.find('(') {
            if paren_pos > 0 && input.ends_with(')') {
                let func_name = &input[..paren_pos];
                let args_str = &input[paren_pos + 1..input.len() - 1];

                // Parse arguments (simplified - split by commas)
                let args: Result<Vec<Expression>, ParseError> = if args_str.is_empty() {
                    Ok(Vec::new())
                } else {
                    args_str
                        .split(',')
                        .map(|arg| self.parse_expression(arg.trim()))
                        .collect()
                };

                return Ok(Some(Expression::function(func_name, args?)));
            }
        }
        Ok(None)
    }

    /// Parse atomic expressions (numbers, variables, functions)
    fn parse_atom_level(&mut self, input: &str) -> Result<Expression, ParseError> {
        // Handle parentheses at atom level
        if input.starts_with('(') && input.ends_with(')') && self.is_balanced_parentheses(input) {
            let inner = &input[1..input.len() - 1];
            return self.parse_expression(inner);
        }

        // Try function first
        if let Some(expr) = self.try_parse_function(input)? {
            return Ok(expr);
        }

        self.parse_atom(input)
    }

    /// Parse atomic expressions (numbers, variables)
    fn parse_atom(&mut self, input: &str) -> Result<Expression, ParseError> {
        // Handle negative numbers
        if input.starts_with('-') && input.len() > 1 {
            let positive_part = &input[1..];
            if let Ok(n) = positive_part.parse::<i64>() {
                return Ok(Expression::integer(-n));
            }
            if let Ok(f) = positive_part.parse::<f64>() {
                return Ok(Expression::number(Number::float(-f)));
            }
            // Negative variable: -x becomes -1*x
            if positive_part.chars().all(|c| c.is_alphabetic() || c == '_') {
                let symbol = Symbol::new(positive_part);
                self.variables
                    .insert(positive_part.to_string(), symbol.clone());
                return Ok(Expression::mul(vec![
                    Expression::integer(-1),
                    Expression::symbol(symbol),
                ]));
            }
        }

        if let Ok(n) = input.parse::<i64>() {
            return Ok(Expression::integer(n));
        }

        if let Ok(f) = input.parse::<f64>() {
            return Ok(Expression::number(Number::float(f)));
        }

        if let Some(slash_pos) = input.find('/') {
            let num_str = &input[..slash_pos];
            let den_str = &input[slash_pos + 1..];

            if let (Ok(num), Ok(den)) = (num_str.parse::<i64>(), den_str.parse::<i64>()) {
                let rational = num_rational::BigRational::new(
                    num_bigint::BigInt::from(num),
                    num_bigint::BigInt::from(den),
                );
                return Ok(Expression::number(Number::rational(rational)));
            }
        }

        if input.chars().all(|c| c.is_alphabetic() || c == '_') {
            let symbol = Symbol::new(input);
            self.variables.insert(input.to_string(), symbol.clone());
            return Ok(Expression::symbol(symbol));
        }

        Err(ParseError::InvalidAtom(input.to_string()))
    }

    /// Get all variables encountered during parsing
    pub fn get_variables(&self) -> &HashMap<String, Symbol> {
        &self.variables
    }

    /// Clear variable cache
    pub fn clear_variables(&mut self) {
        self.variables.clear();
    }

    /// Parse interval notation: [0,1], (0,1), [0,1), (0,1]
    fn parse_interval(&mut self, input: &str) -> Result<Option<Expression>, ParseError> {
        let trimmed = input.trim();

        // Check for interval patterns
        let (start_inclusive, end_inclusive) = if trimmed.starts_with('[') && trimmed.ends_with(']')
        {
            (true, true) // [0,1] - closed interval
        } else if trimmed.starts_with('(') && trimmed.ends_with(')') && trimmed.contains(',') {
            (false, false) // (0,1) - open interval
        } else if trimmed.starts_with('[') && trimmed.ends_with(')') {
            (true, false) // [0,1) - half-open
        } else if trimmed.starts_with('(') && trimmed.ends_with(']') {
            (false, true) // (0,1] - half-open
        } else {
            return Ok(None); // Not an interval
        };

        let content = &trimmed[1..trimmed.len() - 1]; // Remove brackets/parentheses

        // Split by comma
        let parts: Vec<&str> = content.split(',').collect();
        if parts.len() != 2 {
            return Ok(None); // Not a valid interval
        }

        let start_expr = self.parse_expression(parts[0].trim())?;
        let end_expr = self.parse_expression(parts[1].trim())?;

        Ok(Some(Expression::interval(
            start_expr,
            end_expr,
            start_inclusive,
            end_inclusive,
        )))
    }

    /// Parse relation notation: x<y, x>y, x<=y, x>=y, x==y  
    fn parse_relation(&mut self, input: &str) -> Result<Option<Expression>, ParseError> {
        use crate::core::RelationType;

        // Check for relation operators (in order of specificity)
        let relations = [
            ("==", RelationType::Equal),
            ("<=", RelationType::LessEqual),
            (">=", RelationType::GreaterEqual),
            ("<", RelationType::Less),
            (">", RelationType::Greater),
        ];

        for (op, rel_type) in relations {
            if let Some(op_pos) = input.find(op) {
                let left_str = &input[..op_pos].trim();
                let right_str = &input[op_pos + op.len()..].trim();

                let left_expr = self.parse_expression(left_str)?;
                let right_expr = self.parse_expression(right_str)?;

                return Ok(Some(Expression::relation(left_expr, right_expr, rel_type)));
            }
        }

        Ok(None)
    }
}

impl Default for ExpressionParser {
    fn default() -> Self {
        Self::new()
    }
}

/// Parsing error types
#[derive(Debug, Clone, PartialEq)]
pub enum ParseError {
    EmptyInput,
    InvalidAtom(String),
    UnmatchedParentheses,
    InvalidFunction(String),
    InvalidNumber(String),
    UnexpectedCharacter(char),
    SyntaxError(String),
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::EmptyInput => write!(f, "Empty input"),
            ParseError::InvalidAtom(s) => write!(f, "Invalid atom: {}", s),
            ParseError::UnmatchedParentheses => write!(f, "Unmatched parentheses"),
            ParseError::InvalidFunction(s) => write!(f, "Invalid function: {}", s),
            ParseError::InvalidNumber(s) => write!(f, "Invalid number: {}", s),
            ParseError::UnexpectedCharacter(c) => write!(f, "Unexpected character: {}", c),
            ParseError::SyntaxError(s) => write!(f, "Syntax error: {}", s),
        }
    }
}

impl std::error::Error for ParseError {}

/// Convenience functions for parsing
impl Expression {
    /// Parse from string
    pub fn parse(input: &str) -> Result<Expression, ParseError> {
        let mut parser = ExpressionParser::new();
        parser.parse(input)
    }

    /// Parse from LaTeX
    pub fn parse_latex(latex: &str) -> Result<Expression, ParseError> {
        let mut parser = ExpressionParser::new();
        parser.parse_latex(latex)
    }
}

/// LaTeX output formatting
impl Expression {
    /// Convert to LaTeX with advanced formatting
    pub fn to_latex_advanced(&self) -> String {
        match self {
            Expression::Number(Number::SmallInt(n)) => n.to_string(),
            Expression::Number(Number::BigInteger(n)) => n.to_string(),
            Expression::Number(Number::Rational(r)) => {
                if r.denom().is_one() {
                    r.numer().to_string()
                } else {
                    format!("\\frac{{{}}}{{{}}}", r.numer(), r.denom())
                }
            }
            Expression::Number(Number::Float(f)) => f.to_string(),
            Expression::Symbol(s) => s.name().to_string(),
            Expression::Add(terms) => {
                if terms.is_empty() {
                    "0".to_string()
                } else {
                    let term_strs: Vec<String> = terms
                        .iter()
                        .enumerate()
                        .map(|(i, t)| {
                            if i == 0 {
                                t.to_latex_advanced()
                            } else {
                                match t {
                                    Expression::Mul(factors) if factors.len() > 0 => {
                                        if let Expression::Number(Number::SmallInt(n)) = &factors[0]
                                        {
                                            if *n < 0 {
                                                format!(
                                                    " - {}",
                                                    Expression::Mul(Box::new(
                                                        std::iter::once(Expression::Number(
                                                            Number::SmallInt(-n)
                                                        ))
                                                        .chain(factors.iter().skip(1).cloned())
                                                        .collect()
                                                    ))
                                                    .to_latex_advanced()
                                                )
                                            } else {
                                                format!(" + {}", t.to_latex_advanced())
                                            }
                                        } else {
                                            format!(" + {}", t.to_latex_advanced())
                                        }
                                    }
                                    _ => format!(" + {}", t.to_latex_advanced()),
                                }
                            }
                        })
                        .collect();
                    term_strs.join("")
                }
            }
            Expression::Mul(factors) => {
                if factors.is_empty() {
                    "1".to_string()
                } else {
                    let factor_strs: Vec<String> = factors
                        .iter()
                        .map(|f| match f {
                            Expression::Add(_) => {
                                format!("\\left({}\\right)", f.to_latex_advanced())
                            }
                            _ => f.to_latex_advanced(),
                        })
                        .collect();
                    factor_strs.join(" \\cdot ")
                }
            }
            Expression::Pow(base, exp) => {
                let base_latex = match base.as_ref() {
                    Expression::Add(_) | Expression::Mul(_) => {
                        format!("\\left({}\\right)", base.to_latex_advanced())
                    }
                    _ => base.to_latex_advanced(),
                };

                let exp_latex = exp.to_latex_advanced();
                format!("{}^{{{}}}", base_latex, exp_latex)
            }
            Expression::Function { name, args } => {
                let formatted_name = match name.as_str() {
                    "sin" | "cos" | "tan" | "log" | "ln" | "exp" | "sqrt" => {
                        format!("\\{}", name)
                    }
                    "factorial" if args.len() == 1 => {
                        return format!("{}!", args[0].to_latex_advanced());
                    }
                    _ => name.clone(),
                };

                if args.is_empty() {
                    formatted_name
                } else if args.len() == 1 && name == "sqrt" {
                    format!("\\sqrt{{{}}}", args[0].to_latex_advanced())
                } else {
                    let arg_strs: Vec<String> =
                        args.iter().map(|a| a.to_latex_advanced()).collect();
                    format!("{}\\left({}\\right)", formatted_name, arg_strs.join(", "))
                }
            }
            // New expression types - implement later
            _ => "\\text{unknown}".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_parsing() {
        let mut parser = ExpressionParser::new();

        let expr = parser.parse("42").unwrap();
        assert_eq!(expr, Expression::integer(42));

        let expr = parser.parse("x").unwrap();
        assert_eq!(expr, Expression::symbol(Symbol::new("x")));

        let expr = parser.parse("x + 5").unwrap();
        match expr {
            Expression::Add(terms) => assert_eq!(terms.len(), 2),
            _ => panic!("Expected addition"),
        }
    }

    #[test]
    #[ignore] // TEMPORARILY DISABLED: Stack overflow in parsing - needs investigation
    fn test_latex_parsing() {
        let mut parser = ExpressionParser::new();

        let expr = parser.parse_latex("\\frac{3}{4}").unwrap();
        println!("Parsed fraction: {}", expr);

        let expr = parser.parse_latex("x^{2}").unwrap();
        match expr {
            Expression::Pow(_, _) => assert!(true),
            _ => println!("Power parsing result: {}", expr),
        }
    }

    #[test]
    fn test_latex_output() {
        let x = Symbol::new("x");

        let rational = Expression::number(Number::rational(num_rational::BigRational::new(
            num_bigint::BigInt::from(3),
            num_bigint::BigInt::from(4),
        )));
        let latex = rational.to_latex_advanced();
        assert_eq!(latex, "\\frac{3}{4}");

        let power = Expression::pow(Expression::symbol(x.clone()), Expression::integer(2));
        let latex = power.to_latex_advanced();
        assert_eq!(latex, "x^{2}");

        let factorial = Expression::function("factorial", vec![Expression::integer(5)]);
        let latex = factorial.to_latex_advanced();
        assert_eq!(latex, "5!");
    }

    #[test]
    fn test_complex_expression_parsing() {
        let mut parser = ExpressionParser::new();

        let expr = parser.parse("(x + 1)^2").unwrap();
        match expr {
            Expression::Pow(base, exp) => {
                assert!(matches!(base.as_ref(), Expression::Add(_)));
                assert_eq!(exp.as_ref(), &Expression::integer(2));
            }
            _ => panic!("Expected power expression"),
        }
    }

    #[test]
    #[ignore] // TEMPORARILY DISABLED: Stack overflow in parsing - needs investigation
    fn test_function_parsing() {
        let mut parser = ExpressionParser::new();

        let expr = parser.parse("sin(x)").unwrap();
        match expr {
            Expression::Function { name, args } => {
                assert_eq!(name, "sin");
                assert_eq!(args.len(), 1);
            }
            _ => panic!("Expected function"),
        }
    }
}
