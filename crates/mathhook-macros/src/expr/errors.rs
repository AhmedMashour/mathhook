//! Educational error messages for expr!() macro
//!
//! Provides helpful, actionable error messages that guide users to correct syntax.

use proc_macro2::Span;
use syn::Error;

/// Create error for unsupported binary operator
pub fn unsupported_operator(op: &str, span: Span) -> Error {
    Error::new(
        span,
        format!(
            "Unsupported operator '{}'\n\
             = help: Supported operators: +, -, *, /, ** (power), ==, <, >, <=, >=\n\
             = note: Use ** for exponentiation (e.g., x**2)\n\
             = note: Or use .pow() method (e.g., x.pow(2))\n\
             = note: Comparison operators: ==, <, >, <=, >= return boolean expressions",
            op
        ),
    )
}

/// Create error for unsupported expression type
pub fn unsupported_expression(expr_type: &str, span: Span) -> Error {
    Error::new(
        span,
        format!(
            "Unsupported expression type: {}\n\
             = help: expr!() supports literals, identifiers, binary operations, and function calls\n\
             = note: For complex expressions, use explicit Expression constructors",
            expr_type
        ),
    )
}

#[allow(dead_code)]
/// Create error for invalid power operator syntax
pub fn invalid_power_syntax(span: Span) -> Error {
    Error::new(
        span,
        "Invalid power operator syntax\n\
         = help: Use ** for exponentiation (e.g., x**2)\n\
         = note: Rust's ^ operator is XOR, not power\n\
         = note: Or use .pow() method (e.g., x.pow(2))",
    )
}

/// Create error for unsupported unary operator
pub fn unsupported_unary_operator(op: &str, span: Span) -> Error {
    Error::new(
        span,
        format!(
            "Unsupported unary operator '{}'\n\
             = help: Only unary negation (-) is supported\n\
             = note: Use explicit Expression constructors for other operations",
            op
        ),
    )
}

/// Create error for unsupported method call
pub fn unsupported_method_call(method: &str, span: Span) -> Error {
    Error::new(
        span,
        format!(
            "Unsupported method call: {}\n\
             = help: Supported methods: .pow(exp), .abs(), .sqrt(), .simplify()\n\
             = note: For other operations, use explicit Expression constructors\n\
             = note: Example: expr!(x.pow(2)) or expr!(x.abs())",
            method
        ),
    )
}
