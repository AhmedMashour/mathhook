//! Procedural expr!() macro implementation (internal module)
//!
//! This module provides the implementation for the procedural expr!() macro.
//! All exports are internal to the crate only.
use proc_macro::TokenStream;
mod codegen;
mod errors;
mod parser;
mod preprocessor;
use parser::ExprParser;
use preprocessor::PowerOperatorParser;
/// Implementation function for the expr!() procedural macro (crate-internal)
///
/// Includes ** power operator support via token-level preprocessing.
pub(crate) fn expr_impl(input: TokenStream) -> TokenStream {
    let syn_expr = match PowerOperatorParser::parse_with_power(input) {
        Ok(expr) => expr,
        Err(err) => return err.to_compile_error().into(),
    };
    match ExprParser::parse(&syn_expr) {
        Ok(tokens) => tokens.into(),
        Err(err) => err.to_compile_error().into(),
    }
}
