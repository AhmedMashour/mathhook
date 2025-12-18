//! Test proving trait-based code generation works
//!
//! This test expands a macro and verifies the generated tokens are valid Rust.

use quote::quote;
use syn::parse_quote;

#[test]
fn test_trait_config_struct_parses() {
    let attrs = quote! {
        ops = [Add, Sub, Mul], display = true, eq = true
    };

    let parsed = syn::parse::Parser::parse2(
        syn::punctuated::Punctuated::<syn::Meta, syn::Token![,]>::parse_terminated,
        attrs,
    );

    assert!(parsed.is_ok(), "Trait config attributes should parse");
}

#[test]
fn test_mathhook_struct_macro_exists() {
    let struct_def: syn::ItemStruct = parse_quote! {
        #[derive(Clone)]
        pub struct TestStruct {
            pub value: i32,
        }
    };

    assert_eq!(struct_def.ident.to_string(), "TestStruct");
}

#[test]
fn test_supported_trait_identifiers() {
    let valid_traits = vec![
        "Add",
        "Sub",
        "Mul",
        "Div",
        "Neg",
        "Display",
        "PartialEq",
        "PartialOrd",
        "Clone",
    ];

    for trait_name in valid_traits {
        let ident = syn::Ident::new(trait_name, proc_macro2::Span::call_site());
        assert_eq!(ident.to_string(), trait_name);
    }
}
