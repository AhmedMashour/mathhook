//! Symbol macro parser
use super::codegen::{CodeGenerator, SymbolType};
use super::errors;
use proc_macro::TokenStream;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{Ident, Lit, Token};

pub struct SymbolParser;

struct SingleSymbol {
    name: String,
    symbol_type: SymbolType,
}

impl Parse for SingleSymbol {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.peek(Lit) {
            let lit: Lit = input.parse()?;
            if let Lit::Str(s) = lit {
                let name = s.value();
                let symbol_type = if input.peek(Token![;]) {
                    input.parse::<Token![;]>()?;
                    parse_symbol_type(input)?
                } else {
                    SymbolType::Scalar
                };
                Ok(SingleSymbol { name, symbol_type })
            } else {
                Err(errors::invalid_syntax(
                    "expected string literal or identifier",
                    lit.span(),
                ))
            }
        } else if input.peek(Ident) {
            let ident: Ident = input.parse()?;
            let name = ident.to_string();
            let symbol_type = if input.peek(Token![;]) {
                input.parse::<Token![;]>()?;
                parse_symbol_type(input)?
            } else {
                SymbolType::Scalar
            };
            Ok(SingleSymbol { name, symbol_type })
        } else {
            Err(errors::invalid_syntax(
                "expected identifier or string literal",
                input.span(),
            ))
        }
    }
}

struct MultipleSymbols {
    names: Vec<String>,
    symbol_type: SymbolType,
}

impl Parse for MultipleSymbols {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let idents = Punctuated::<Ident, Token![,]>::parse_separated_nonempty(input)?;

        if idents.is_empty() {
            return Err(errors::empty_symbols_list(input.span()));
        }

        let names: Vec<String> = idents.iter().map(|i| i.to_string()).collect();

        let symbol_type = if input.peek(Token![=>]) {
            input.parse::<Token![=>]>()?;
            parse_symbol_type(input)?
        } else {
            SymbolType::Scalar
        };

        Ok(MultipleSymbols { names, symbol_type })
    }
}

fn parse_symbol_type(input: ParseStream) -> syn::Result<SymbolType> {
    let type_ident: Ident = input.parse()?;
    match type_ident.to_string().as_str() {
        "scalar" => Ok(SymbolType::Scalar),
        "matrix" => Ok(SymbolType::Matrix),
        "operator" => Ok(SymbolType::Operator),
        "quaternion" => Ok(SymbolType::Quaternion),
        other => Err(errors::unsupported_type(other, type_ident.span())),
    }
}

impl SymbolParser {
    pub fn parse_single(input: TokenStream) -> syn::Result<proc_macro2::TokenStream> {
        let parsed: SingleSymbol = syn::parse(input)?;
        Ok(CodeGenerator::generate_single(
            &parsed.name,
            parsed.symbol_type,
        ))
    }

    pub fn parse_multiple(input: TokenStream) -> syn::Result<proc_macro2::TokenStream> {
        let parsed: MultipleSymbols = syn::parse(input)?;
        Ok(CodeGenerator::generate_multiple(
            &parsed.names,
            parsed.symbol_type,
        ))
    }
}
