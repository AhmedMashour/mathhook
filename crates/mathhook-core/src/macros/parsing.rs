//! Parsing macros for mathematical expressions
//!
//! Provides ergonomic parsing capabilities for mathematical expressions
//! from string inputs.

/// Parse mathematical expressions from strings
///
/// This macro provides convenient parsing of mathematical expressions.
///
/// # Examples
///
/// ```rust
/// use mathhook_core::parse;
///
/// // Parse mathematical expression
/// let expr = parse!("x^2 + 1").unwrap();
///
/// // Mathematical constants
/// let pi = parse!(constant: pi);
/// let e = parse!(constant: e);
/// ```
#[macro_export]
macro_rules! parse {
    // Simple string parsing
    ($input:expr) => {{
        $crate::parser::grammar::ExpressionParser::new().parse($input)
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
}

#[cfg(test)]
mod tests {
    use crate::{Expression, MathConstant};

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
    fn test_parse_simple() {
        let result = parse!("x").unwrap();
        assert!(matches!(result, Expression::Symbol(_)));
    }
}
