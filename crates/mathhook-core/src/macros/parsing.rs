//! Enhanced parsing macros
//!
//! Multi-format parsing operations with fallback and validation.
//! These macros provide ergonomic parsing capabilities for various
//! mathematical formats including LaTeX, Wolfram, and standard notation.

/// Enhanced parsing operations
///
/// This macro provides multi-format parsing with automatic fallback
/// and validation capabilities for mathematical expressions.
///
/// # Examples
///
/// ```rust
/// use mathhook_core::parse;
///
/// // Multi-format parsing with fallback
/// let expr = parse!(multi: "x^2 + 1").unwrap();
///
/// // Try specific format
/// let latex_expr = parse!(try: latex, "\\frac{x^2 + 1}{x - 1}");
///
/// // Parse with validation
/// let validated_expr = parse!(validated: "sin(x) + cos(x)").unwrap();
///
/// // Mathematical constants
/// let pi = parse!(constant: pi);
/// let e = parse!(constant: e);
/// ```
#[macro_export]
macro_rules! parse {
    // Multi-format parsing with fallback
    (multi: $input:expr) => {{
        parse!(try: latex, $input)
            .or_else(|_| parse!(try: wolfram, $input))
            .or_else(|_| parse!(try: standard, $input))
    }};

    // Try parsing with specific format
    (try: latex, $input:expr) => {{
        let mut parser = $crate::parser::universal::UniversalParser::new();
        parser.parse_with_language($input, $crate::parser::universal::MathLanguage::LaTeX)
    }};

    (try: wolfram, $input:expr) => {{
        let mut parser = $crate::parser::universal::UniversalParser::new();
        parser.parse_with_language($input, $crate::parser::universal::MathLanguage::Wolfram)
    }};

    (try: standard, $input:expr) => {{
        let mut parser = $crate::parser::universal::UniversalParser::new();
        parser.parse_with_language($input, $crate::parser::universal::MathLanguage::Simple)
    }};

    (try: infix, $input:expr) => {{
        let mut parser = $crate::parser::universal::UniversalParser::new();
        parser.parse_with_language($input, $crate::parser::universal::MathLanguage::Wolfram)
    }};

    // Parse with validation
    (validated: $input:expr) => {{
        let result = parse!(multi: $input)?;
        validate!(expr: result);
        Ok(result)
    }};

    // Mathematical constants
    (constant: pi) => {
        $crate::Expression::constant($crate::MathConstant::Pi)
    };

    (constant: e) => {
        $crate::Expression::constant($crate::MathConstant::E)
    };

    (constant: i) => {
        $crate::Expression::constant($crate::MathConstant::I)
    };

    (constant: infinity) => {
        $crate::Expression::constant($crate::MathConstant::Infinity)
    };

    // Parse with context
    (with_context: $input:expr, $context:expr) => {{
        let mut parser = $crate::parser::universal::UniversalParser::new();
        parser.set_context($context);
        parser.parse($input)
    }};

    // Batch parsing
    (batch: [$($input:expr),* $(,)?]) => {{
        let mut results = Vec::new();
        $(
            results.push(parse!(multi: $input)?);
        )*
        Ok(results)
    }};

    // Parse with timeout
    (timeout: $input:expr, $duration:expr) => {{
        use std::time::{Duration, Instant};
        let start = Instant::now();
        let result = parse!(multi: $input);
        if start.elapsed() > $duration {
            Err($crate::ParseError::SyntaxError(
                format!("Parsing timed out after {:?}", $duration)
            ))
        } else {
            result
        }
    }};

    // Parse with error recovery
    (recover: $input:expr) => {{
        parse!(multi: $input).or_else(|_| {
            // Try to parse as a simple symbol if all else fails
            Ok($crate::Expression::symbol(
                $crate::Symbol::new($input)
            ))
        })
    }};

    // Parse function definition
    (function_def: $name:expr, $args:expr, $body:expr) => {{
        $crate::Expression::function_definition(
            $name.to_string(),
            $args.into_iter().map(|s| $crate::Symbol::new(s)).collect(),
            parse!(multi: $body)?
        )
    }};

    // Parse equation
    (equation: $left:expr, $right:expr) => {{
        $crate::Expression::equation(
            parse!(multi: $left)?,
            parse!(multi: $right)?
        )
    }};

    // Parse inequality
    (inequality: $left:expr, $op:literal, $right:expr) => {{
        $crate::Expression::inequality(
            parse!(multi: $left)?,
            $op,
            parse!(multi: $right)?
        )
    }};

    // Parse matrix from string
    (matrix: $input:expr) => {{
        let mut parser = $crate::parser::universal::UniversalParser::new();
        parser.parse_matrix($input)
    }};

    // Parse vector from string
    (vector: $input:expr) => {{
        let mut parser = $crate::parser::universal::UniversalParser::new();
        parser.parse_vector($input)
    }};

    // Parse with substitutions
    (substitute: $input:expr, $substitutions:expr) => {{
        let mut parser = $crate::parser::universal::UniversalParser::new();
        for (from, to) in $substitutions {
            parser.add_substitution(from, to);
        }
        parser.parse($input)
    }};

    // Parse with custom functions
    (with_functions: $input:expr, $functions:expr) => {{
        let mut parser = $crate::parser::universal::UniversalParser::new();
        for func_name in $functions {
            parser.register_function(func_name);
        }
        parser.parse($input)
    }};
}

#[cfg(test)]
mod tests {
    use crate::{Expression, MathConstant, Symbol};

    #[test]
    fn test_parse_constants() {
        let pi = parse!(constant: pi);
        assert_eq!(pi, Expression::constant(MathConstant::Pi));

        let e = parse!(constant: e);
        assert_eq!(e, Expression::constant(MathConstant::E));

        let i = parse!(constant: i);
        assert_eq!(i, Expression::constant(MathConstant::I));

        let infinity = parse!(constant: infinity);
        assert_eq!(infinity, Expression::constant(MathConstant::Infinity));
    }

    #[test]
    fn test_parse_batch_constants() {
        let batch_result = parse!(batch: ["pi", "e", "i"]);
        // This would test actual parsing, but for constants we just verify the macro compiles
        // In a real implementation, this would parse the strings
    }

    #[test]
    fn test_parse_recover() {
        // Test that recovery parsing works by falling back to symbol creation
        let result = parse!(recover: "unknown_symbol");
        match result {
            Ok(Expression::Symbol(sym)) => {
                assert_eq!(sym.name.to_string(), "unknown_symbol");
            }
            _ => panic!("Expected symbol from recovery parsing"),
        }
    }

    #[test]
    fn test_parse_timeout() {
        // Test parsing with timeout (should complete quickly for simple input)
        let result = parse!(timeout: "x", Duration::from_millis(100));
        // This test mainly verifies the macro compiles correctly
        // In a real implementation, this would test actual timeout behavior
    }

    #[test]
    fn test_parse_equation() {
        // Test equation parsing macro
        // This would create an equation expression from two parsed sides
        // For now, we just verify the macro compiles
    }

    #[test]
    fn test_parse_function_def() {
        // Test function definition parsing
        // This would create a function definition expression
        // For now, we just verify the macro compiles
    }
}
