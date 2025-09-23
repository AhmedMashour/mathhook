//! Universal mathematical expression parser
//!
//! Provides format-aware parsing with automatic language detection for LaTeX,
//! Wolfram Language, and simple mathematical notation.

use crate::core::{Expression, Number, Symbol};
use crate::parser::ParseError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Supported mathematical notation languages
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum MathLanguage {
    Simple,  // Basic mathematical notation
    LaTeX,   // LaTeX mathematical notation
    Wolfram, // Wolfram Language (Mathematica)
    Auto,    // Auto-detect
}

/// Universal parser that handles multiple mathematical notation languages
pub struct UniversalParser {
    /// Variable cache for consistent symbol handling
    variables: HashMap<String, Symbol>,
}

impl UniversalParser {
    /// Create new universal parser
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
        }
    }

    /// Main parse method with automatic language detection
    pub fn parse(&mut self, input: &str) -> Result<Expression, ParseError> {
        let language = self.detect_language(input);
        self.parse_with_language(input, language)
    }

    /// Parse with explicit language specification
    pub fn parse_with_language(
        &mut self,
        input: &str,
        language: MathLanguage,
    ) -> Result<Expression, ParseError> {
        match language {
            MathLanguage::Simple => self.parse_simple(input),
            MathLanguage::LaTeX => self.parse_latex(input),
            MathLanguage::Wolfram => self.parse_wolfram(input),
            MathLanguage::Auto => {
                let detected = self.detect_language(input);
                self.parse_with_language(input, detected)
            }
        }
    }

    /// Detect mathematical notation language from input
    pub fn detect_language(&self, input: &str) -> MathLanguage {
        use crate::parser::constants::*;

        let latex_score = LATEX_DETECTION_PATTERNS
            .iter()
            .map(|&indicator| input.matches(indicator).count())
            .sum::<usize>();

        let wolfram_score = WOLFRAM_DETECTION_PATTERNS
            .iter()
            .map(|&indicator| input.matches(indicator).count())
            .sum::<usize>();

        // Special case: Wolfram set vs LaTeX set detection
        if input.trim().starts_with('{') && !input.contains('\\') {
            return MathLanguage::Wolfram; // Pure {1,2,3} is Wolfram
        }

        if latex_score > 0 && latex_score > wolfram_score {
            MathLanguage::LaTeX
        } else if wolfram_score > 0 && wolfram_score > latex_score {
            MathLanguage::Wolfram
        } else {
            // No special patterns detected - simple mathematical notation
            MathLanguage::Simple
        }
    }

    /// Parse simple mathematical expression
    fn parse_simple(&mut self, input: &str) -> Result<Expression, ParseError> {
        let parser = crate::MathParser::new();
        parser.parse_standard(input)
    }

    /// Parse LaTeX mathematical expression
    fn parse_latex(&mut self, latex: &str) -> Result<Expression, ParseError> {
        let mut latex_parser = crate::parser::latex_parser::LaTeXParser::new();
        latex_parser.parse(latex)
    }

    /// Parse Wolfram Language mathematical expression
    fn parse_wolfram(&mut self, wolfram: &str) -> Result<Expression, ParseError> {
        let mut wolfram_parser = crate::parser::wolfram_parser::WolframParser::new();
        wolfram_parser.parse(wolfram)
    }

    /// Generate simple mathematical notation (no LaTeX commands)
    pub fn to_simple(&self, expr: &Expression) -> String {
        self.expression_to_simple(expr)
    }

    /// Generate LaTeX notation with commands
    pub fn to_latex(&self, expr: &Expression) -> String {
        self.expression_to_latex(expr, &LaTeXContext::default())
    }

    /// Generate Wolfram Language notation
    pub fn to_wolfram(&self, expr: &Expression) -> String {
        self.expression_to_wolfram(expr, &WolframContext::default())
    }

    /// Generate output in the same format as the detected input language
    pub fn to_format(&self, expr: &Expression, format: MathLanguage) -> String {
        match format {
            MathLanguage::Simple => self.to_simple(expr),
            MathLanguage::LaTeX => self.to_latex(expr),
            MathLanguage::Wolfram => self.to_wolfram(expr),
            MathLanguage::Auto => self.to_simple(expr), // Default to simple for auto
        }
    }

    /// Preprocess LaTeX input
    fn preprocess_latex(&self, latex: &str) -> String {
        latex
            .trim()
            // Basic symbol replacements
            .replace("\\cdot", "*")
            .replace("\\times", "*")
            .replace("\\div", "/")
            .replace("\\pm", "±")
            .replace("\\mp", "∓")
            // Greek letters
            .replace("\\alpha", "α")
            .replace("\\beta", "β")
            .replace("\\gamma", "γ")
            .replace("\\delta", "δ")
            .replace("\\epsilon", "ε")
            .replace("\\theta", "θ")
            .replace("\\lambda", "λ")
            .replace("\\mu", "μ")
            .replace("\\pi", "π")
            .replace("\\sigma", "σ")
            .replace("\\phi", "φ")
            .replace("\\omega", "ω")
            // Special constants
            .replace("\\infty", "∞")
            .replace("\\emptyset", "∅")
            // Remove spaces
            .replace(" ", "")
    }

    /// Preprocess Wolfram Language input
    fn preprocess_wolfram(&self, wolfram: &str) -> String {
        wolfram
            .trim()
            // Normalize constants
            .replace("Pi", "π")
            .replace("E", "e")
            .replace("Infinity", "∞")
            .replace("I", "i")
            // Remove extra spaces
            .replace("  ", " ")
    }

    /// Parse LaTeX expression with comprehensive LaTeX support
    fn parse_latex_expression(&mut self, input: &str) -> Result<Expression, ParseError> {
        if input.is_empty() {
            return Err(ParseError::EmptyInput);
        }

        // Handle LaTeX commands first
        if let Some(expr) = self.parse_latex_commands(input)? {
            return Ok(expr);
        }

        // If no LaTeX commands found, fall back to basic parsing
        self.parse_simple(input)
    }

    /// Parse LaTeX commands like \frac{}{}, \sin(), etc.
    fn parse_latex_commands(&mut self, input: &str) -> Result<Option<Expression>, ParseError> {
        // Handle \frac{numerator}{denominator}
        if input.starts_with("\\frac{") {
            return self.parse_latex_fraction(input).map(Some);
        }

        // Handle \sin(x), \cos(x), etc.
        if let Some(func_result) = self.parse_latex_function(input)? {
            return Ok(Some(func_result));
        }

        // Handle \sqrt{x}, \sqrt[n]{x}
        if input.starts_with("\\sqrt") {
            return self.parse_latex_sqrt(input).map(Some);
        }

        Ok(None)
    }

    /// Parse LaTeX fraction: \frac{numerator}{denominator}
    fn parse_latex_fraction(&mut self, input: &str) -> Result<Expression, ParseError> {
        if !input.starts_with("\\frac{") {
            return Err(ParseError::SyntaxError("Not a fraction".to_string()));
        }

        // Find the numerator and denominator
        let after_frac = &input[6..]; // Skip "\\frac{"

        let mut brace_count = 0;
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

        // Parse denominator
        if !remainder.starts_with('{') {
            return Err(ParseError::SyntaxError(
                "Missing denominator braces".to_string(),
            ));
        }

        let mut brace_count = 0;
        let mut denominator_end = 0;

        for (i, ch) in remainder.chars().enumerate() {
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

        let denominator_str = &remainder[1..denominator_end]; // Skip opening brace

        // Parse numerator and denominator recursively
        let numerator = self.parse_latex_expression(numerator_str)?;
        let denominator = self.parse_latex_expression(denominator_str)?;

        // Return as multiplication: numerator * (denominator^-1)
        Ok(Expression::mul(vec![
            numerator,
            Expression::pow(denominator, Expression::integer(-1)),
        ]))
    }

    /// Parse LaTeX functions like \sin(x), \cos(x)
    fn parse_latex_function(&mut self, input: &str) -> Result<Option<Expression>, ParseError> {
        let latex_functions = [
            ("\\sin(", "sin"),
            ("\\cos(", "cos"),
            ("\\tan(", "tan"),
            ("\\ln(", "ln"),
            ("\\log(", "log"),
            ("\\exp(", "exp"),
        ];

        for (latex_pattern, func_name) in &latex_functions {
            if input.starts_with(latex_pattern) {
                let after_func = &input[latex_pattern.len()..];

                // Find matching closing parenthesis
                let mut paren_count = 1;
                let mut arg_end = 0;

                for (i, ch) in after_func.chars().enumerate() {
                    match ch {
                        '(' => paren_count += 1,
                        ')' => {
                            paren_count -= 1;
                            if paren_count == 0 {
                                arg_end = i;
                                break;
                            }
                        }
                        _ => {}
                    }
                }

                if arg_end == 0 {
                    return Err(ParseError::SyntaxError(
                        "Unmatched parentheses in function".to_string(),
                    ));
                }

                let arg_str = &after_func[..arg_end];
                let arg_expr = self.parse_latex_expression(arg_str)?;

                return Ok(Some(Expression::function(*func_name, vec![arg_expr])));
            }
        }

        Ok(None)
    }

    /// Parse LaTeX square root: \sqrt{x} or \sqrt[n]{x}
    fn parse_latex_sqrt(&mut self, input: &str) -> Result<Expression, ParseError> {
        if input.starts_with("\\sqrt[") {
            // Handle \sqrt[n]{x}
            let after_sqrt = &input[6..]; // Skip "\\sqrt["

            // Find the closing bracket for the index
            let bracket_end = after_sqrt.find(']').ok_or_else(|| {
                ParseError::SyntaxError("Missing closing bracket in root".to_string())
            })?;

            let index_str = &after_sqrt[..bracket_end];
            let remainder = &after_sqrt[bracket_end + 1..];

            // Parse the radicand
            if !remainder.starts_with('{') {
                return Err(ParseError::SyntaxError(
                    "Missing braces in root".to_string(),
                ));
            }

            let brace_end = remainder.rfind('}').ok_or_else(|| {
                ParseError::SyntaxError("Missing closing brace in root".to_string())
            })?;

            let radicand_str = &remainder[1..brace_end];

            let index_expr = self.parse_latex_expression(index_str)?;
            let radicand_expr = self.parse_latex_expression(radicand_str)?;

            // \sqrt[n]{x} = x^(1/n)
            Ok(Expression::pow(
                radicand_expr,
                Expression::pow(index_expr, Expression::integer(-1)),
            ))
        } else if input.starts_with("\\sqrt{") {
            // Handle \sqrt{x}
            let after_sqrt = &input[6..]; // Skip "\\sqrt{"

            let brace_end = after_sqrt.rfind('}').ok_or_else(|| {
                ParseError::SyntaxError("Missing closing brace in sqrt".to_string())
            })?;

            let radicand_str = &after_sqrt[..brace_end];
            let radicand_expr = self.parse_latex_expression(radicand_str)?;

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

    /// Parse Wolfram expression with comprehensive Wolfram support
    fn parse_wolfram_expression(&mut self, input: &str) -> Result<Expression, ParseError> {
        if input.is_empty() {
            return Err(ParseError::EmptyInput);
        }

        // Handle Wolfram functions first
        if let Some(expr) = self.parse_wolfram_functions(input)? {
            return Ok(expr);
        }

        // Fall back to conversion and existing parser
        let converted = self.wolfram_to_latex_syntax(input);
        self.parse_latex_expression(&converted)
    }

    /// Parse Wolfram functions like Sin[x], Times[x, y], etc.
    fn parse_wolfram_functions(&mut self, input: &str) -> Result<Option<Expression>, ParseError> {
        // Handle basic functions
        let wolfram_functions = [
            ("Sin[", "sin"),
            ("Cos[", "cos"),
            ("Tan[", "tan"),
            ("Log[", "ln"),
            ("Exp[", "exp"),
            ("Sqrt[", "sqrt"),
        ];

        for (wolfram_pattern, func_name) in &wolfram_functions {
            if input.starts_with(wolfram_pattern) {
                let args = self.parse_wolfram_function_args(&input[wolfram_pattern.len()..])?;

                if func_name == &"sqrt" && args.len() == 1 {
                    // Sqrt[x] = x^(1/2)
                    return Ok(Some(Expression::pow(
                        args[0].clone(),
                        Expression::number(Number::rational(num_rational::BigRational::new(
                            num_bigint::BigInt::from(1),
                            num_bigint::BigInt::from(2),
                        ))),
                    )));
                } else if func_name == &"exp" && args.len() == 1 {
                    // Exp[x] = e^x
                    return Ok(Some(Expression::pow(Expression::e(), args[0].clone())));
                } else {
                    return Ok(Some(Expression::function(*func_name, args)));
                }
            }
        }

        // Handle special operators
        if input.starts_with("Times[") {
            let args = self.parse_wolfram_function_args(&input[6..])?;
            return Ok(Some(Expression::mul(args)));
        }

        if input.starts_with("Plus[") {
            let args = self.parse_wolfram_function_args(&input[5..])?;
            return Ok(Some(Expression::add(args)));
        }

        if input.starts_with("Power[") {
            let args = self.parse_wolfram_function_args(&input[6..])?;
            if args.len() == 2 {
                return Ok(Some(Expression::pow(args[0].clone(), args[1].clone())));
            }
        }

        Ok(None)
    }

    /// Parse Wolfram function arguments: "x, y, z]" → [x, y, z]
    fn parse_wolfram_function_args(&mut self, input: &str) -> Result<Vec<Expression>, ParseError> {
        // Find the closing bracket
        let mut bracket_count = 1;
        let mut args_end = 0;

        for (i, ch) in input.chars().enumerate() {
            match ch {
                '[' => bracket_count += 1,
                ']' => {
                    bracket_count -= 1;
                    if bracket_count == 0 {
                        args_end = i;
                        break;
                    }
                }
                _ => {}
            }
        }

        if args_end == 0 {
            return Err(ParseError::SyntaxError(
                "Unmatched brackets in function".to_string(),
            ));
        }

        let args_str = &input[..args_end];

        if args_str.is_empty() {
            return Ok(vec![]);
        }

        // Split by commas, but respect nested structures
        let arg_strings = self.split_wolfram_args(args_str)?;

        // Parse each argument
        let mut args = Vec::new();
        for arg_str in arg_strings {
            let arg_expr = self.parse_wolfram_expression(&arg_str)?;
            args.push(arg_expr);
        }

        Ok(args)
    }

    /// Split Wolfram function arguments by commas, respecting nesting
    fn split_wolfram_args(&self, args_str: &str) -> Result<Vec<String>, ParseError> {
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
    fn wolfram_to_latex_syntax(&self, wolfram: &str) -> String {
        wolfram
            // Functions
            .replace("Sin[", "\\sin(")
            .replace("Cos[", "\\cos(")
            .replace("Tan[", "\\tan(")
            .replace("Log[", "\\ln(")
            .replace("Exp[", "\\exp(")
            .replace("Sqrt[", "\\sqrt{")
            // Operators
            .replace("Power[", "")
            .replace("Plus[", "")
            .replace("Times[", "")
            // Brackets
            .replace("[", "(")
            .replace("]", ")")
            .replace("{", "(")
            .replace("}", ")")
    }

    /// Convert Expression to simple mathematical notation (no LaTeX commands)
    fn expression_to_simple(&self, expr: &Expression) -> String {
        match expr {
            Expression::Number(Number::Integer(n)) => n.to_string(),
            Expression::Number(Number::BigInteger(n)) => n.to_string(),
            Expression::Number(Number::Rational(r)) => {
                if r.denom() == &num_bigint::BigInt::from(1) {
                    r.numer().to_string()
                } else {
                    format!("{}/{}", r.numer(), r.denom())
                }
            }
            Expression::Number(Number::Float(f)) => f.to_string(),
            Expression::Symbol(s) => s.name().to_string(),
            Expression::Add(terms) => {
                let term_strs: Vec<String> = terms
                    .iter()
                    .enumerate()
                    .map(|(i, term)| {
                        if i == 0 {
                            self.expression_to_simple(term)
                        } else {
                            format!(" + {}", self.expression_to_simple(term))
                        }
                    })
                    .collect();
                term_strs.join("")
            }
            Expression::Mul(factors) => {
                let factor_strs: Vec<String> = factors
                    .iter()
                    .map(|f| {
                        let needs_parens = matches!(f, Expression::Add(_));
                        if needs_parens {
                            format!("({})", self.expression_to_simple(f))
                        } else {
                            self.expression_to_simple(f)
                        }
                    })
                    .collect();
                factor_strs.join(" * ")
            }
            Expression::Pow(base, exp) => {
                let base_simple = self.expression_to_simple(base);
                let exp_simple = self.expression_to_simple(exp);
                format!("{}^{}", base_simple, exp_simple)
            }
            Expression::Function { name, args } => {
                if args.is_empty() {
                    name.clone()
                } else {
                    let arg_strs: Vec<String> =
                        args.iter().map(|a| self.expression_to_simple(a)).collect();
                    format!("{}({})", name, arg_strs.join(", "))
                }
            }
            _ => format!("unknown"),
        }
    }

    /// Convert Expression to LaTeX
    fn expression_to_latex(&self, expr: &Expression, context: &LaTeXContext) -> String {
        match expr {
            Expression::Number(Number::Integer(n)) => n.to_string(),
            Expression::Number(Number::BigInteger(n)) => n.to_string(),
            Expression::Number(Number::Rational(r)) => {
                if r.denom() == &num_bigint::BigInt::from(1) {
                    r.numer().to_string()
                } else {
                    format!("\\frac{{{}}}{{{}}}", r.numer(), r.denom())
                }
            }
            Expression::Number(Number::Float(f)) => f.to_string(),
            Expression::Symbol(s) => s.name().to_string(),
            Expression::Add(terms) => {
                let term_strs: Vec<String> = terms
                    .iter()
                    .enumerate()
                    .map(|(i, term)| {
                        if i == 0 {
                            self.expression_to_latex(term, context)
                        } else {
                            format!(" + {}", self.expression_to_latex(term, context))
                        }
                    })
                    .collect();
                if context.needs_parentheses {
                    format!("\\left({}\\right)", term_strs.join(""))
                } else {
                    term_strs.join("")
                }
            }
            Expression::Mul(factors) => {
                let factor_strs: Vec<String> = factors
                    .iter()
                    .map(|f| {
                        let needs_parens = matches!(f, Expression::Add(_));
                        if needs_parens {
                            format!("\\left({}\\right)", self.expression_to_latex(f, context))
                        } else {
                            self.expression_to_latex(f, context)
                        }
                    })
                    .collect();
                factor_strs.join(" \\cdot ")
            }
            Expression::Pow(base, exp) => {
                // Check if this is a square root: x^(1/2) -> \sqrt{x}
                if let Expression::Number(Number::Rational(r)) = exp.as_ref() {
                    if r.numer() == &num_bigint::BigInt::from(1)
                        && r.denom() == &num_bigint::BigInt::from(2)
                    {
                        return format!("\\sqrt{{{}}}", self.expression_to_latex(base, context));
                    }
                }

                let base_str = match base.as_ref() {
                    Expression::Add(_) | Expression::Mul(_) => {
                        format!("\\left({}\\right)", self.expression_to_latex(base, context))
                    }
                    _ => self.expression_to_latex(base, context),
                };
                format!(
                    "{}^{{{}}}",
                    base_str,
                    self.expression_to_latex(exp, context)
                )
            }
            Expression::Function { name, args } => self.function_to_latex(name, args, context),
            // New expression types - implement later
            Expression::Complex(complex_data) => format!(
                "{} + {}i",
                self.expression_to_latex(&complex_data.real, context),
                self.expression_to_latex(&complex_data.imag, context)
            ),
            Expression::Matrix(matrix_data) => {
                let row_strs: Vec<String> = matrix_data
                    .rows
                    .iter()
                    .map(|row| {
                        row.iter()
                            .map(|elem| self.expression_to_latex(elem, context))
                            .collect::<Vec<_>>()
                            .join(" & ")
                    })
                    .collect();
                format!(
                    "\\begin{{pmatrix}} {} \\end{{pmatrix}}",
                    row_strs.join(" \\\\ ")
                )
            }
            Expression::Constant(c) => format!("{:?}", c),
            Expression::Relation { .. } => "\\text{relation}".to_string(),
            Expression::Piecewise { .. } => "\\text{piecewise}".to_string(),
            Expression::Set(elements) => {
                if elements.is_empty() {
                    "\\{\\}".to_string()
                } else {
                    let element_strs: Vec<String> = elements
                        .iter()
                        .map(|elem| self.expression_to_latex(elem, context))
                        .collect();
                    format!("\\{{{}\\}}", element_strs.join(", "))
                }
            }
            Expression::Interval { .. } => "\\text{interval}".to_string(),
            // Calculus expressions with proper LaTeX formatting
            Expression::Calculus(calculus_data) => {
                use crate::core::expression::CalculusData;
                match calculus_data.as_ref() {
                    CalculusData::Derivative {
                        expression,
                        variable,
                        order,
                    } => {
                        if *order == 1 {
                            format!(
                                "\\frac{{d}}{{d{}}} {}",
                                variable.name(),
                                self.expression_to_latex(expression, context)
                            )
                        } else {
                            format!(
                                "\\frac{{d^{}}}{{d{}^{}}} {}",
                                order,
                                variable.name(),
                                order,
                                self.expression_to_latex(expression, context)
                            )
                        }
                    }
                    CalculusData::Integral {
                        integrand,
                        variable,
                        bounds,
                    } => match bounds {
                        None => format!(
                            "\\int {} d{}",
                            self.expression_to_latex(integrand, context),
                            variable.name()
                        ),
                        Some((start, end)) => format!(
                            "\\int_{{{}}}^{{{}}} {} d{}",
                            self.expression_to_latex(start, context),
                            self.expression_to_latex(end, context),
                            self.expression_to_latex(integrand, context),
                            variable.name()
                        ),
                    },
                    CalculusData::Limit {
                        expression,
                        variable,
                        direction,
                        ..
                    } => {
                        format!(
                            "\\lim_{{{}\\to{}}} {}",
                            variable.name(),
                            match direction {
                                crate::core::expression::LimitDirection::Left => "0^-",
                                crate::core::expression::LimitDirection::Right => "0^+",
                                crate::core::expression::LimitDirection::Both => "0",
                            },
                            self.expression_to_latex(expression, context)
                        )
                    }
                    CalculusData::Sum {
                        expression,
                        variable,
                        start,
                        end,
                    } => {
                        format!(
                            "\\sum_{{{}={}}}^{{{}}} {}",
                            variable.name(),
                            self.expression_to_latex(start, context),
                            self.expression_to_latex(end, context),
                            self.expression_to_latex(expression, context)
                        )
                    }
                    CalculusData::Product {
                        expression,
                        variable,
                        start,
                        end,
                    } => {
                        format!(
                            "\\prod_{{{}={}}}^{{{}}} {}",
                            variable.name(),
                            self.expression_to_latex(start, context),
                            self.expression_to_latex(end, context),
                            self.expression_to_latex(expression, context)
                        )
                    }
                }
            }
        }
    }

    /// Convert Expression to Wolfram Language
    fn expression_to_wolfram(&self, expr: &Expression, context: &WolframContext) -> String {
        match expr {
            Expression::Number(Number::Integer(n)) => n.to_string(),
            Expression::Number(Number::BigInteger(n)) => n.to_string(),
            Expression::Number(Number::Rational(r)) => {
                if r.denom() == &num_bigint::BigInt::from(1) {
                    r.numer().to_string()
                } else {
                    // Use Power[denominator, -1] for proper Wolfram syntax
                    format!("Times[{}, Power[{}, -1]]", r.numer(), r.denom())
                }
            }
            Expression::Number(Number::Float(f)) => f.to_string(),
            Expression::Symbol(s) => s.name().to_string(),
            Expression::Add(terms) => {
                if terms.len() == 1 {
                    self.expression_to_wolfram(&terms[0], context)
                } else {
                    let term_strs: Vec<String> = terms
                        .iter()
                        .map(|t| self.expression_to_wolfram(t, context))
                        .collect();
                    format!("Plus[{}]", term_strs.join(", "))
                }
            }
            Expression::Mul(factors) => {
                if factors.len() == 1 {
                    self.expression_to_wolfram(&factors[0], context)
                } else {
                    let factor_strs: Vec<String> = factors
                        .iter()
                        .map(|f| self.expression_to_wolfram(f, context))
                        .collect();
                    format!("Times[{}]", factor_strs.join(", "))
                }
            }
            Expression::Pow(base, exp) => {
                format!(
                    "Power[{}, {}]",
                    self.expression_to_wolfram(base, context),
                    self.expression_to_wolfram(exp, context)
                )
            }
            Expression::Function { name, args } => self.function_to_wolfram(name, args, context),
            // New expression types - implement later
            Expression::Complex(complex_data) => format!(
                "Complex[{}, {}]",
                self.expression_to_wolfram(&complex_data.real, context),
                self.expression_to_wolfram(&complex_data.imag, context)
            ),
            Expression::Matrix(_) => "matrix".to_string(),
            Expression::Constant(c) => format!("{:?}", c),
            Expression::Relation(_) => "relation".to_string(),
            Expression::Piecewise(_) => "piecewise".to_string(),
            Expression::Set(elements) => {
                if elements.is_empty() {
                    "{}".to_string()
                } else {
                    let element_strs: Vec<String> = elements
                        .iter()
                        .map(|elem| self.expression_to_wolfram(elem, context))
                        .collect();
                    format!("{{{}}}", element_strs.join(", "))
                }
            }
            Expression::Interval(_) => "interval".to_string(),
            // Calculus expressions with proper Wolfram formatting
            Expression::Calculus(calculus_data) => {
                use crate::core::expression::CalculusData;
                match calculus_data.as_ref() {
                    CalculusData::Derivative {
                        expression,
                        variable,
                        order,
                    } => {
                        if *order == 1 {
                            format!(
                                "D[{}, {}]",
                                self.expression_to_wolfram(expression, context),
                                variable.name()
                            )
                        } else {
                            format!(
                                "D[{}, {{{}, {}}}]",
                                self.expression_to_wolfram(expression, context),
                                variable.name(),
                                order
                            )
                        }
                    }
                    CalculusData::Integral {
                        integrand,
                        variable,
                        bounds,
                    } => match bounds {
                        None => format!(
                            "Integrate[{}, {}]",
                            self.expression_to_wolfram(integrand, context),
                            variable.name()
                        ),
                        Some((start, end)) => format!(
                            "Integrate[{}, {{{}, {}, {}}}]",
                            self.expression_to_wolfram(integrand, context),
                            variable.name(),
                            self.expression_to_wolfram(start, context),
                            self.expression_to_wolfram(end, context)
                        ),
                    },
                    CalculusData::Limit {
                        expression,
                        variable,
                        direction,
                        ..
                    } => {
                        format!(
                            "Limit[{}, {} -> {}]",
                            self.expression_to_wolfram(expression, context),
                            variable.name(),
                            match direction {
                                crate::core::expression::LimitDirection::Left => "-1",
                                crate::core::expression::LimitDirection::Right => "+1",
                                crate::core::expression::LimitDirection::Both =>
                                    "Direction -> \"TwoSided\"",
                            }
                        )
                    }
                    CalculusData::Sum {
                        expression,
                        variable,
                        start,
                        end,
                    } => {
                        format!(
                            "Sum[{}, {{{}, {}, {}}}]",
                            self.expression_to_wolfram(expression, context),
                            variable.name(),
                            self.expression_to_wolfram(start, context),
                            self.expression_to_wolfram(end, context)
                        )
                    }
                    CalculusData::Product {
                        expression,
                        variable,
                        start,
                        end,
                    } => {
                        format!(
                            "Product[{}, {{{}, {}, {}}}]",
                            self.expression_to_wolfram(expression, context),
                            variable.name(),
                            self.expression_to_wolfram(start, context),
                            self.expression_to_wolfram(end, context)
                        )
                    }
                }
            }
        }
    }

    /// Convert function to LaTeX
    fn function_to_latex(&self, name: &str, args: &[Expression], context: &LaTeXContext) -> String {
        match name {
            // Trigonometric functions
            "sin" | "cos" | "tan" | "sec" | "csc" | "cot" => {
                if args.len() == 1 {
                    format!(
                        "\\{}({})",
                        name,
                        self.expression_to_latex(&args[0], context)
                    )
                } else {
                    format!(
                        "\\{}({})",
                        name,
                        args.iter()
                            .map(|a| self.expression_to_latex(a, context))
                            .collect::<Vec<_>>()
                            .join(", ")
                    )
                }
            }
            // Inverse trigonometric
            "arcsin" | "arccos" | "arctan" => {
                format!(
                    "\\{}({})",
                    name,
                    self.expression_to_latex(&args[0], context)
                )
            }
            // Logarithmic functions
            "ln" => format!("\\ln({})", self.expression_to_latex(&args[0], context)),
            "log" => {
                if args.len() == 1 {
                    format!("\\log({})", self.expression_to_latex(&args[0], context))
                } else if args.len() == 2 {
                    format!(
                        "\\log_{{{}}}({})",
                        self.expression_to_latex(&args[1], context),
                        self.expression_to_latex(&args[0], context)
                    )
                } else {
                    format!(
                        "\\log({})",
                        args.iter()
                            .map(|a| self.expression_to_latex(a, context))
                            .collect::<Vec<_>>()
                            .join(", ")
                    )
                }
            }
            "exp" => format!("\\exp({})", self.expression_to_latex(&args[0], context)),
            // Special functions
            "sqrt" => {
                if args.len() == 1 {
                    format!("\\sqrt{{{}}}", self.expression_to_latex(&args[0], context))
                } else if args.len() == 2 {
                    format!(
                        "\\sqrt[{}]{{{}}}",
                        self.expression_to_latex(&args[1], context),
                        self.expression_to_latex(&args[0], context)
                    )
                } else {
                    format!("\\sqrt{{{}}}", self.expression_to_latex(&args[0], context))
                }
            }
            "factorial" => {
                if args.len() == 1 {
                    format!("{}!", self.expression_to_latex(&args[0], context))
                } else {
                    format!(
                        "\\text{{factorial}}({})",
                        args.iter()
                            .map(|a| self.expression_to_latex(a, context))
                            .collect::<Vec<_>>()
                            .join(", ")
                    )
                }
            }
            // Calculus
            "integrate" => {
                if args.len() == 2 {
                    format!(
                        "\\int {} \\, d{}",
                        self.expression_to_latex(&args[0], context),
                        self.expression_to_latex(&args[1], context)
                    )
                } else {
                    format!(
                        "\\int({})",
                        args.iter()
                            .map(|a| self.expression_to_latex(a, context))
                            .collect::<Vec<_>>()
                            .join(", ")
                    )
                }
            }
            "derivative" => {
                if args.len() == 2 {
                    format!(
                        "\\frac{{d}}{{d{}}} {}",
                        self.expression_to_latex(&args[1], context),
                        self.expression_to_latex(&args[0], context)
                    )
                } else {
                    format!(
                        "\\frac{{d}}{{dx}}({})",
                        self.expression_to_latex(&args[0], context)
                    )
                }
            }
            "sum" => {
                if args.len() == 4 {
                    format!(
                        "\\sum_{{{}={}}}^{{{}}} {}",
                        self.expression_to_latex(&args[1], context),
                        self.expression_to_latex(&args[2], context),
                        self.expression_to_latex(&args[3], context),
                        self.expression_to_latex(&args[0], context)
                    )
                } else {
                    format!(
                        "\\sum({})",
                        args.iter()
                            .map(|a| self.expression_to_latex(a, context))
                            .collect::<Vec<_>>()
                            .join(", ")
                    )
                }
            }
            // Default case
            _ => {
                if args.is_empty() {
                    name.to_string()
                } else {
                    format!(
                        "\\text{{{}}}({})",
                        name,
                        args.iter()
                            .map(|a| self.expression_to_latex(a, context))
                            .collect::<Vec<_>>()
                            .join(", ")
                    )
                }
            }
        }
    }

    /// Convert function to Wolfram Language
    fn function_to_wolfram(
        &self,
        name: &str,
        args: &[Expression],
        context: &WolframContext,
    ) -> String {
        let wolfram_name = match name {
            // Trigonometric functions
            "sin" => "Sin",
            "cos" => "Cos",
            "tan" => "Tan",
            "sec" => "Sec",
            "csc" => "Csc",
            "cot" => "Cot",
            "arcsin" => "ArcSin",
            "arccos" => "ArcCos",
            "arctan" => "ArcTan",
            // Logarithmic
            "ln" => "Log",
            "log" => "Log",
            "exp" => "Exp",
            // Special functions
            "sqrt" => "Sqrt",
            "factorial" => "Factorial",
            "gamma" => "Gamma",
            "zeta" => "Zeta",
            "beta" => "Beta",
            // Calculus
            "integrate" => "Integrate",
            "derivative" => "D",
            "sum" => "Sum",
            "product" => "Product",
            "limit" => "Limit",
            // Default
            _ => name,
        };

        if args.is_empty() {
            wolfram_name.to_string()
        } else {
            let arg_strs: Vec<String> = args
                .iter()
                .map(|a| self.expression_to_wolfram(a, context))
                .collect();
            format!("{}[{}]", wolfram_name, arg_strs.join(", "))
        }
    }

    /// Get all variables encountered during parsing
    pub fn get_variables(&self) -> &HashMap<String, Symbol> {
        &self.variables
    }

    /// Clear variable cache
    pub fn clear_variables(&mut self) {
        self.variables.clear();
    }
}

/// LaTeX formatting context
#[derive(Debug, Default)]
struct LaTeXContext {
    needs_parentheses: bool,
    in_fraction: bool,
    in_exponent: bool,
}

/// Wolfram formatting context
#[derive(Debug, Default)]
struct WolframContext {
    function_style: WolframFunctionStyle,
    precedence_level: u8,
}

/// Wolfram function call style
#[derive(Debug, Clone, Copy, PartialEq)]
enum WolframFunctionStyle {
    Functional, // Sin[x]
    Infix,      // x + y
    Prefix,     // -x
}

impl Default for WolframFunctionStyle {
    fn default() -> Self {
        WolframFunctionStyle::Functional
    }
}

impl Default for UniversalParser {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_language_detection() {
        let parser = UniversalParser::new();

        // LaTeX detection
        assert_eq!(parser.detect_language("\\frac{1}{2}"), MathLanguage::LaTeX);
        assert_eq!(
            parser.detect_language("\\sin(x) + \\cos(y)"),
            MathLanguage::LaTeX
        );
        assert_eq!(parser.detect_language("\\int x dx"), MathLanguage::LaTeX);

        // Wolfram detection
        assert_eq!(
            parser.detect_language("Sin[x] + Cos[y]"),
            MathLanguage::Wolfram
        );
        assert_eq!(
            parser.detect_language("Integrate[x, x]"),
            MathLanguage::Wolfram
        );
        assert_eq!(parser.detect_language("Power[x, 2]"), MathLanguage::Wolfram);

        // Ambiguous cases default to LaTeX
        assert_eq!(parser.detect_language("x + y"), MathLanguage::LaTeX);
    }

    #[test]
    fn test_basic_latex_parsing() {
        let mut parser = UniversalParser::new();

        let result = parser.parse_latex("x + 1");
        assert!(result.is_ok());

        let result = parser.parse_latex("\\frac{1}{2}");
        assert!(result.is_ok());
    }

    #[test]
    fn test_basic_wolfram_parsing() {
        let mut parser = UniversalParser::new();

        let result = parser.parse_wolfram("x + 1");
        assert!(result.is_ok());

        let result = parser.parse_wolfram("Sin[x]");
        assert!(result.is_ok());
    }

    #[test]
    fn test_latex_output() {
        let parser = UniversalParser::new();

        let expr = Expression::add(vec![
            Expression::symbol(Symbol::new("x")),
            Expression::integer(1),
        ]);

        let latex = parser.to_latex(&expr);
        assert!(latex.contains("x"));
        assert!(latex.contains("1"));
    }

    #[test]
    fn test_wolfram_output() {
        let parser = UniversalParser::new();

        let expr = Expression::add(vec![
            Expression::symbol(Symbol::new("x")),
            Expression::integer(1),
        ]);

        let wolfram = parser.to_wolfram(&expr);
        assert!(wolfram.contains("Plus"));
        assert!(wolfram.contains("x"));
        assert!(wolfram.contains("1"));
    }
}
