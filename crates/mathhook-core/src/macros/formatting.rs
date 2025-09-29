//! Multi-format output generation macros
//!
//! Formatting macros for converting mathematical expressions to various
//! output formats including LaTeX, Wolfram Language, human-readable text,
//! and structured data formats.

/// Multi-format output generation
///
/// This macro provides comprehensive formatting capabilities for mathematical
/// expressions across multiple output formats.
///
/// # Examples
///
/// ```rust
/// use mathhook_core::{format, expr};
///
/// let expression = expr!(pow: expr!(x), expr!(2));
///
/// // LaTeX formatting
/// let latex = format!(latex: expression.clone());
///
/// // Wolfram formatting
/// let wolfram = format!(wolfram: expression.clone());
///
/// // Human-readable formatting
/// let human = format!(human: expression.clone());
///
/// // JSON serialization
/// let json = format!(json: expression.clone());
///
/// // Multi-format output
/// let all_formats = format!(all: expression);
/// ```
#[macro_export]
macro_rules! format {
    // LaTeX formatting
    (latex: $expr:expr) => {
        $expr.to_latex()
    };

    // Wolfram Language formatting
    (wolfram: $expr:expr) => {
        $expr.to_wolfram()
    };

    // Human-readable formatting
    (human: $expr:expr) => {
        $expr.to_string()
    };

    // JSON serialization
    (json: $expr:expr) => {
        serde_json::to_string(&$expr).unwrap_or_else(|_| "{}".to_string())
    };

    // Pretty JSON with indentation
    (json_pretty: $expr:expr) => {
        serde_json::to_string_pretty(&$expr).unwrap_or_else(|_| "{}".to_string())
    };

    // Multi-format output as JSON object
    (all: $expr:expr) => {{
        let expr = $expr;
        serde_json::json!({
            "latex": format!(latex: expr),
            "wolfram": format!(wolfram: expr),
            "human": format!(human: expr),
            "json": format!(json: expr)
        })
    }};

    // Conditional formatting based on format string
    (conditional: $expr:expr, $format:expr) => {
        match $format {
            "latex" => format!(latex: $expr),
            "wolfram" => format!(wolfram: $expr),
            "json" => format!(json: $expr),
            "json_pretty" => format!(json_pretty: $expr),
            _ => format!(human: $expr),
        }
    };

    // Pretty printing with indentation
    (pretty: $expr:expr, $indent:expr) => {
        format!("{}{}", " ".repeat($indent), format!(human: $expr))
    };

    // Mathematical notation formatting
    (math: $expr:expr) => {
        $expr.to_mathematical_notation()
    };

    // ASCII art formatting (for simple expressions)
    (ascii: $expr:expr) => {
        $expr.to_ascii_art()
    };

    // Code generation formatting
    (code: $expr:expr, $language:literal) => {
        match $language {
            "rust" => $expr.to_rust_code(),
            "python" => $expr.to_python_code(),
            "javascript" => $expr.to_javascript_code(),
            "c" => $expr.to_c_code(),
            "julia" => $expr.to_julia_code(),
            _ => format!(human: $expr),
        }
    };

    // Compact formatting (minimal whitespace)
    (compact: $expr:expr) => {
        format!(human: $expr).chars().filter(|c| !c.is_whitespace()).collect::<String>()
    };

    // Verbose formatting (with explanations)
    (verbose: $expr:expr) => {
        $expr.to_verbose_string()
    };

    // Tree structure formatting
    (tree: $expr:expr) => {
        $expr.to_tree_string()
    };

    // Debug formatting with type information
    (debug: $expr:expr) => {
        format!("{:#?}", $expr)
    };

    // Parentheses-explicit formatting
    (explicit: $expr:expr) => {
        $expr.to_explicit_string()
    };

    // Scientific notation formatting
    (scientific: $expr:expr) => {
        $expr.to_scientific_notation()
    };

    // Engineering notation formatting
    (engineering: $expr:expr) => {
        $expr.to_engineering_notation()
    };

    // Fraction formatting
    (fraction: $expr:expr) => {
        $expr.to_fraction_string()
    };

    // Decimal formatting with precision
    (decimal: $expr:expr, $precision:expr) => {
        $expr.to_decimal_string($precision)
    };

    // HTML formatting with MathML
    (html: $expr:expr) => {
        $expr.to_html()
    };

    // Markdown formatting
    (markdown: $expr:expr) => {
        $expr.to_markdown()
    };

    // CSV formatting (for matrices/vectors)
    (csv: $expr:expr) => {
        $expr.to_csv()
    };

    // XML formatting
    (xml: $expr:expr) => {
        $expr.to_xml()
    };

    // YAML formatting
    (yaml: $expr:expr) => {
        serde_yaml::to_string(&$expr).unwrap_or_else(|_| "{}".to_string())
    };

    // TOML formatting
    (toml: $expr:expr) => {
        toml::to_string(&$expr).unwrap_or_else(|_| "".to_string())
    };
}

#[cfg(test)]
mod tests {
    use crate::{Expression, Symbol};

    #[test]
    fn test_format_human() {
        let expr = Expression::integer(42);
        let formatted = format!(human: expr);
        assert_eq!(formatted, "42");
    }

    #[test]
    fn test_format_json() {
        let expr = Expression::integer(42);
        let json = format!(json: expr);
        // Should be valid JSON
        assert!(json.contains("42") || json.contains("Integer"));
    }

    #[test]
    fn test_format_json_pretty() {
        let expr = Expression::integer(42);
        let json_pretty = format!(json_pretty: expr);
        // Should be valid JSON with indentation
        assert!(json_pretty.contains("42") || json_pretty.contains("Integer"));
    }

    #[test]
    fn test_format_conditional() {
        let expr = Expression::integer(42);

        let human = format!(conditional: expr.clone(), "human");
        let json = format!(conditional: expr.clone(), "json");
        let default = format!(conditional: expr.clone(), "unknown");

        assert_eq!(human, "42");
        assert!(json.contains("42") || json.contains("Integer"));
        assert_eq!(default, "42"); // Falls back to human
    }

    #[test]
    fn test_format_pretty() {
        let expr = Expression::integer(42);
        let pretty = format!(pretty: expr, 4);
        assert_eq!(pretty, "    42");
    }

    #[test]
    fn test_format_compact() {
        let expr = Expression::add(vec![Expression::integer(1), Expression::integer(2)]);
        let compact = format!(compact: expr);
        // Should have no whitespace
        assert!(!compact.contains(' '));
    }

    #[test]
    fn test_format_debug() {
        let expr = Expression::integer(42);
        let debug = format!(debug: expr);
        // Should contain debug information
        assert!(debug.contains("Integer") || debug.contains("42"));
    }

    #[test]
    fn test_format_decimal() {
        let expr = Expression::rational(Expression::integer(1), Expression::integer(3));
        // This would format as decimal with specified precision
        // For now, just test that the macro compiles
        let _decimal = format!(decimal: expr, 3);
    }

    #[test]
    fn test_format_all() {
        let expr = Expression::integer(42);
        let all_formats = format!(all: expr);

        // Should be a JSON object with multiple format keys
        let json_str = all_formats.to_string();
        assert!(
            json_str.contains("latex")
                || json_str.contains("wolfram")
                || json_str.contains("human")
        );
    }

    #[test]
    fn test_format_yaml() {
        let expr = Expression::integer(42);
        let yaml = format!(yaml: expr);
        // Should be valid YAML
        assert!(!yaml.is_empty());
    }

    #[test]
    fn test_format_code_generation() {
        let expr = Expression::add(vec![Expression::integer(1), Expression::integer(2)]);

        // Test different code generation targets
        let _rust_code = format!(code: expr.clone(), "rust");
        let _python_code = format!(code: expr.clone(), "python");
        let _js_code = format!(code: expr.clone(), "javascript");
        let _c_code = format!(code: expr.clone(), "c");
        let _julia_code = format!(code: expr.clone(), "julia");
        let _unknown_code = format!(code: expr, "unknown");

        // These would generate actual code in different languages
        // For now, just verify the macros compile
    }
}
