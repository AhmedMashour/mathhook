//! Error types for symbol macros
use proc_macro2::Span;

pub fn invalid_syntax(msg: &str, span: Span) -> syn::Error {
    syn::Error::new(span, format!("Invalid symbol syntax: {}", msg))
}

pub fn unsupported_type(ty: &str, span: Span) -> syn::Error {
    syn::Error::new(
        span,
        format!(
            "Unsupported symbol type: {}. Valid types: scalar, matrix, operator, quaternion",
            ty
        ),
    )
}

pub fn empty_symbols_list(span: Span) -> syn::Error {
    syn::Error::new(span, "symbols!() requires at least one symbol name")
}
