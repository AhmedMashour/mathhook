//! Procedural symbol!() and symbols!() macro implementations
use proc_macro::TokenStream;
mod codegen;
mod errors;
mod parser;
use parser::SymbolParser;

/// Implementation function for symbol!() procedural macro
pub(crate) fn symbol_impl(input: TokenStream) -> TokenStream {
    match SymbolParser::parse_single(input) {
        Ok(tokens) => tokens.into(),
        Err(err) => err.to_compile_error().into(),
    }
}

/// Implementation function for symbols!() procedural macro
pub(crate) fn symbols_impl(input: TokenStream) -> TokenStream {
    match SymbolParser::parse_multiple(input) {
        Ok(tokens) => tokens.into(),
        Err(err) => err.to_compile_error().into(),
    }
}
