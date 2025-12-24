//! Export macro system for automatic Python and Node.js binding generation
//!
//! This module provides procedural macros that automatically generate
//! Python (PyO3) and Node.js (NAPI-RS) bindings from Rust code.
//!
//! ## Macros
//!
//! - `#[mathhook_fn]` - Export standalone functions
//! - `#[mathhook_struct]` - Export struct wrappers
//! - `#[mathhook_impl]` - Export methods on structs and enums
//! - `#[mathhook_enum]` - Export tagged union wrappers
//! - `mathhook_module!` - Define module structure

mod codegen;
mod common;
mod docs;
pub mod expr_ref;
mod nodejs;
mod nodejs_enum;
mod nodejs_impl;
mod python;
mod python_enum;
mod python_impl;
mod stubs;
pub mod traits;
mod types;

pub use codegen::{generate_nodejs_wrapper, generate_python_wrapper};
use convert_case::{Case, Casing};

use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse2, ItemEnum, ItemFn, ItemImpl, ItemStruct, Result};

#[derive(Debug, Clone, Default)]
pub struct ExportConfig {
    pub name: Option<String>,
    pub module: Option<String>,
    pub python_name: Option<String>,
    pub nodejs_name: Option<String>,
    pub skip_python: bool,
    pub skip_nodejs: bool,
    pub trait_config: traits::TraitConfig,
}

impl ExportConfig {
    pub fn from_attrs(attrs: &TokenStream) -> Result<Self> {
        let mut config = Self::default();

        if attrs.is_empty() {
            return Ok(config);
        }

        let parsed = syn::parse::Parser::parse2(
            syn::punctuated::Punctuated::<syn::Meta, syn::Token![,]>::parse_terminated,
            attrs.clone(),
        )?;

        let metas: Vec<_> = parsed.into_iter().collect();

        for meta in &metas {
            match meta {
                syn::Meta::NameValue(nv) if nv.path.is_ident("name") => {
                    if let syn::Expr::Lit(lit) = &nv.value {
                        if let syn::Lit::Str(s) = &lit.lit {
                            config.name = Some(s.value());
                        }
                    }
                }
                syn::Meta::NameValue(nv) if nv.path.is_ident("module") => {
                    if let syn::Expr::Lit(lit) = &nv.value {
                        if let syn::Lit::Str(s) = &lit.lit {
                            config.module = Some(s.value());
                        }
                    }
                }
                syn::Meta::NameValue(nv) if nv.path.is_ident("python_name") => {
                    if let syn::Expr::Lit(lit) = &nv.value {
                        if let syn::Lit::Str(s) = &lit.lit {
                            config.python_name = Some(s.value());
                        }
                    }
                }
                syn::Meta::NameValue(nv) if nv.path.is_ident("nodejs_name") => {
                    if let syn::Expr::Lit(lit) = &nv.value {
                        if let syn::Lit::Str(s) = &lit.lit {
                            config.nodejs_name = Some(s.value());
                        }
                    }
                }
                syn::Meta::Path(p) if p.is_ident("skip_python") => {
                    config.skip_python = true;
                }
                syn::Meta::Path(p) if p.is_ident("skip_nodejs") => {
                    config.skip_nodejs = true;
                }
                _ => {}
            }
        }

        config.trait_config = traits::TraitConfig::from_attrs(&metas)?;

        Ok(config)
    }
}

pub fn process_function(attrs: TokenStream, item: TokenStream) -> Result<TokenStream> {
    let config = ExportConfig::from_attrs(&attrs)?;
    let func: ItemFn = parse2(item.clone())?;
    let func_name = &func.sig.ident;

    let mut output = item;

    if !config.skip_python {
        let python_wrapper = generate_python_wrapper(&func, &config)?;
        output = quote! {
            #output
            #[cfg(feature = "python-bindings")]
            #python_wrapper
        };
    }

    if !config.skip_nodejs {
        let nodejs_wrapper = generate_nodejs_wrapper(&func, &config)?;
        let mod_name = quote::format_ident!("__js_{}", func_name.to_string().to_case(Case::Snake));
        output = quote! {
            #output

            #[cfg(feature = "nodejs-bindings")]
            pub mod #mod_name {
                use super::*;
                #nodejs_wrapper
            }
        };
    }

    Ok(output)
}

pub fn process_struct(attrs: TokenStream, item: TokenStream) -> Result<TokenStream> {
    let config = ExportConfig::from_attrs(&attrs)?;
    let struct_def: ItemStruct = parse2(item.clone())?;
    let struct_name = &struct_def.ident;

    let mut output = item;

    if !config.skip_python {
        let python_wrapper = python::generate_python_struct_wrapper(&struct_def, &config)?;
        output = quote! {
            #output
            #[cfg(feature = "python-bindings")]
            #python_wrapper
        };
    }

    if !config.skip_nodejs {
        let nodejs_wrapper = nodejs::generate_nodejs_struct_wrapper(&struct_def, &config)?;
        let mod_name =
            quote::format_ident!("__js_{}", struct_name.to_string().to_case(Case::Snake));
        output = quote! {
            #output

            #[cfg(feature = "nodejs-bindings")]
            pub mod #mod_name {
                use super::*;
                #nodejs_wrapper
            }
        };
    }

    Ok(output)
}

pub fn process_impl(attrs: TokenStream, item: TokenStream) -> Result<TokenStream> {
    let config = ExportConfig::from_attrs(&attrs)?;
    let impl_block: ItemImpl = parse2(item.clone())?;

    let type_name = match &*impl_block.self_ty {
        syn::Type::Path(tp) => tp
            .path
            .segments
            .last()
            .map(|s| s.ident.to_string())
            .unwrap_or_else(|| "impl".to_string()),
        _ => "impl".to_string(),
    };
    let type_ident = quote::format_ident!("{}", type_name.to_string().to_case(Case::Snake));

    let mut output = item;

    if !config.skip_python {
        let python_methods = python::generate_python_impl_wrapper(&impl_block, &config)?;
        output = quote! {
            #output
            #[cfg(feature = "python-bindings")]
            #python_methods
        };
    }

    if !config.skip_nodejs {
        let nodejs_methods = nodejs::generate_nodejs_impl_wrapper(&impl_block, &config)?;
        let mod_name =
            quote::format_ident!("__js_impl_{}", type_ident.to_string().to_case(Case::Snake));
        output = quote! {
            #output

            #[cfg(feature = "nodejs-bindings")]
            pub mod #mod_name {
                use super::*;
                #nodejs_methods
            }
        };
    }

    Ok(output)
}

pub fn process_enum(attrs: TokenStream, item: TokenStream) -> Result<TokenStream> {
    let config = ExportConfig::from_attrs(&attrs)?;
    let enum_def: ItemEnum = parse2(item.clone())?;
    let enum_name = &enum_def.ident;

    let mut output = item;

    if !config.skip_python {
        let python_wrapper = python::generate_python_enum_wrapper(&enum_def, &config)?;
        output = quote! {
            #output
            #[cfg(feature = "python-bindings")]
            #python_wrapper
        };
    }

    if !config.skip_nodejs {
        let nodejs_wrapper = nodejs::generate_nodejs_enum_wrapper(&enum_def, &config)?;
        let mod_name = quote::format_ident!("__js_{}", enum_name.to_string().to_case(Case::Snake));
        output = quote! {
            #output

            #[cfg(feature = "nodejs-bindings")]
            pub mod #mod_name {
                use super::*;
                #nodejs_wrapper
            }
        };
    }

    Ok(output)
}
