//! Code generation for Symbol constructors
use proc_macro2::TokenStream;
use quote::quote;

pub enum SymbolType {
    Scalar,
    Matrix,
    Operator,
    Quaternion,
}

pub struct CodeGenerator;

impl CodeGenerator {
    pub fn generate_single(name: &str, symbol_type: SymbolType) -> TokenStream {
        match symbol_type {
            SymbolType::Scalar => quote! {
                mathhook_core::Symbol::scalar(#name)
            },
            SymbolType::Matrix => quote! {
                mathhook_core::Symbol::matrix(#name)
            },
            SymbolType::Operator => quote! {
                mathhook_core::Symbol::operator(#name)
            },
            SymbolType::Quaternion => quote! {
                mathhook_core::Symbol::quaternion(#name)
            },
        }
    }

    pub fn generate_multiple(names: &[String], symbol_type: SymbolType) -> TokenStream {
        let symbols: Vec<TokenStream> = names
            .iter()
            .map(|name| {
                let name_str = name.as_str();
                match symbol_type {
                    SymbolType::Scalar => quote! { mathhook_core::Symbol::scalar(#name_str) },
                    SymbolType::Matrix => quote! { mathhook_core::Symbol::matrix(#name_str) },
                    SymbolType::Operator => quote! { mathhook_core::Symbol::operator(#name_str) },
                    SymbolType::Quaternion => {
                        quote! { mathhook_core::Symbol::quaternion(#name_str) }
                    }
                }
            })
            .collect();

        quote! {
            vec![#(#symbols),*]
        }
    }
}
