//! Parsing and format conversion macros
//!
//! When `expr!` isn't enough for complex expressions, use these parsing macros.
//! They handle arbitrary mathematical expressions with proper operator precedence.

/// Parse mathematical expressions from strings with format detection
///
/// Use this macro when `expr!` limitations prevent you from expressing complex mathematics.
/// The parser handles operator precedence, nested functions, and multiple terms correctly.
///
/// # Auto-Detection Examples
///
/// ```rust
/// use mathhook::parse;
///
/// // Polynomials
/// let quadratic = parse!("a*x^2 + b*x + c").unwrap();
/// let cubic = parse!("x^3 - 2*x^2 + x - 1").unwrap();
///
/// // Nested functions  
/// let nested = parse!("sin(x^2 + 1)").unwrap();
/// let complex = parse!("log(sin(x) + cos(y))").unwrap();
///
/// // Fractions and parentheses
/// let fraction = parse!("(x + 1) / (x - 1)").unwrap();
/// let mixed = parse!("sin(x) / (x^2 + 1)").unwrap();
///
/// // Multiple operations
/// let multi = parse!("a*sin(x) + b*cos(x) + c").unwrap();
/// ```
///
/// # Format-Specific Parsing
///
/// ```rust
/// use mathhook::parse;
///
/// // LaTeX format
/// let latex_frac = parse!(latex: "\\frac{x^2 + 1}{x - 1}").unwrap();
/// let latex_func = parse!(latex: "\\sin(\\pi x)").unwrap();
///
/// // Wolfram/Mathematica format  
/// let wolfram_expr = parse!(wolfram: "Sin[Pi*x]").unwrap();
/// let wolfram_func = parse!(wolfram: "Power[x, 2] + 1").unwrap();
///
/// // Simple format (explicit)
/// let simple = parse!(simple: "x^2 + 2*x + 1").unwrap();
/// ```
///
/// # When to Use vs `expr!`
///
/// ```rust
/// use mathhook::{expr, parse};
///
/// // Simple - Use expr!
/// let simple = expr!(x + 1);
///
/// // Complex - Use parse!  
/// let complex = parse!("x^2 + 2*x + 1").unwrap();
///
/// // Mixed approach
/// let x = expr!(x);
/// let polynomial = parse!("x^2 + 2*x + 1").unwrap();
/// let result = polynomial + x;  // Combine parsed and macro expressions
/// ```
#[macro_export]
macro_rules! parse {
    // Auto-detect format
    ($input:expr) => {{
        let mut parser = $crate::core::parser::universal::UniversalParser::new();
        parser.parse($input)
    }};

    // Explicit LaTeX format
    (latex: $input:expr) => {{
        let mut parser = $crate::core::parser::universal::UniversalParser::new();
        parser.parse_with_language($input, $crate::core::parser::universal::MathLanguage::LaTeX)
    }};

    // Explicit Wolfram format
    (wolfram: $input:expr) => {{
        let mut parser = $crate::core::parser::universal::UniversalParser::new();
        parser.parse_with_language(
            $input,
            $crate::core::parser::universal::MathLanguage::Wolfram,
        )
    }};

    // Explicit Simple format
    (simple: $input:expr) => {{
        let mut parser = $crate::core::parser::UniversalParser::new();
        parser.parse_with_language(
            $input,
            $crate::core::parser::universal::MathLanguage::Simple,
        )
    }};
}

/// Convert expressions to different output formats
///
/// # Examples
///
/// ```rust
/// use mathhook::{expr, to_format};
///
/// let expr = expr!(x^2 + 1);
///
/// let simple_output = to_format!(simple: expr);   // "x^2 + 1"
/// let latex_output = to_format!(latex: expr);     // "x^{2} + 1"
/// let wolfram_output = to_format!(wolfram: expr); // "Plus[Power[x, 2], 1]"
/// ```
#[macro_export]
macro_rules! to_format {
    // Simple format
    (simple: $expr:expr) => {{
        let parser = $crate::core::parser::UniversalParser::new();
        parser.to_simple(&$expr)
    }};

    // LaTeX format
    (latex: $expr:expr) => {{
        let parser = $crate::core::parser::UniversalParser::new();
        parser.to_latex(&$expr)
    }};

    // Wolfram format
    (wolfram: $expr:expr) => {{
        let parser = $crate::core::parser::UniversalParser::new();
        parser.to_wolfram(&$expr)
    }};
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_parse_macro() {
        let result = parse!("x + 1");
        assert!(result.is_ok());

        let latex_result = parse!(latex: "\\frac{1}{2}");
        assert!(latex_result.is_ok());
    }
}
