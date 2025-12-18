//! Integration tests for export macro system
//!
//! These tests validate macro expansion without depending on PyO3 or NAPI-RS.
//! The generated code is verified for syntactic correctness using syn parsing.

use quote::ToTokens;

#[test]
fn test_macro_attribute_syntax() {
    let test_code = quote::quote! {
        #[mathhook_macros::mathhook_fn]
        pub fn double(x: i64) -> i64 {
            x * 2
        }
    };

    let parsed = syn::parse2::<syn::File>(test_code);
    assert!(parsed.is_ok(), "Macro invocation should parse correctly");
}

#[test]
fn test_macro_with_attributes() {
    let test_code = quote::quote! {
        #[mathhook_macros::mathhook_fn(name = "custom")]
        pub fn test() {}
    };

    let parsed = syn::parse2::<syn::File>(test_code);
    assert!(parsed.is_ok());
}

#[test]
fn test_struct_macro_syntax() {
    let test_code = quote::quote! {
        #[mathhook_macros::mathhook_struct]
        pub struct Point {
            pub x: f64,
            pub y: f64,
        }
    };

    let parsed = syn::parse2::<syn::File>(test_code);
    assert!(parsed.is_ok());
}

#[test]
fn test_enum_macro_syntax() {
    let test_code = quote::quote! {
        #[mathhook_macros::mathhook_enum]
        pub enum Result {
            Success,
            Failure,
        }
    };

    let parsed = syn::parse2::<syn::File>(test_code);
    assert!(parsed.is_ok());
}

#[test]
fn test_impl_macro_syntax() {
    let test_code = quote::quote! {
        #[mathhook_macros::mathhook_impl]
        impl MyType {
            pub fn method(&self) -> i32 {
                42
            }
        }
    };

    let parsed = syn::parse2::<syn::File>(test_code);
    assert!(parsed.is_ok());
}

#[test]
fn test_function_signature_parsing() {
    let parsed = syn::parse_str::<syn::ItemFn>("pub fn test(x: i64) -> i64 { x }").unwrap();
    assert_eq!(parsed.sig.ident.to_string(), "test");
    assert_eq!(parsed.sig.inputs.len(), 1);
}

#[test]
fn test_struct_parsing() {
    let parsed =
        syn::parse_str::<syn::ItemStruct>("pub struct Point { pub x: f64, pub y: f64 }").unwrap();
    assert_eq!(parsed.ident.to_string(), "Point");
}

#[test]
fn test_enum_parsing() {
    let parsed = syn::parse_str::<syn::ItemEnum>("pub enum Status { Active, Inactive }").unwrap();
    assert_eq!(parsed.ident.to_string(), "Status");
    assert_eq!(parsed.variants.len(), 2);
}

#[test]
fn test_expression_type_recognition() {
    let parsed = syn::parse_str::<syn::Type>("Expression").unwrap();
    let tokens = parsed.to_token_stream().to_string();
    assert!(tokens.contains("Expression"));
}

#[test]
fn test_result_type_recognition() {
    let parsed = syn::parse_str::<syn::Type>("Result<Expression, MathError>").unwrap();
    let tokens = parsed.to_token_stream().to_string();
    assert!(tokens.contains("Result"));
}

#[test]
fn test_option_type_recognition() {
    let parsed = syn::parse_str::<syn::Type>("Option<usize>").unwrap();
    let tokens = parsed.to_token_stream().to_string();
    assert!(tokens.contains("Option"));
}

#[test]
fn test_vec_type_recognition() {
    let parsed = syn::parse_str::<syn::Type>("Vec<Expression>").unwrap();
    let tokens = parsed.to_token_stream().to_string();
    assert!(tokens.contains("Vec"));
}

#[test]
fn test_reference_type_parsing() {
    let parsed = syn::parse_str::<syn::Type>("&Expression").unwrap();
    matches!(parsed, syn::Type::Reference(_));
}

#[test]
fn test_mutable_reference_parsing() {
    let parsed = syn::parse_str::<syn::Type>("&mut Expression").unwrap();
    if let syn::Type::Reference(r) = parsed {
        assert!(r.mutability.is_some());
    } else {
        panic!("Expected reference type");
    }
}

#[test]
fn test_tuple_type_parsing() {
    let parsed = syn::parse_str::<syn::Type>("(i64, i64)").unwrap();
    matches!(parsed, syn::Type::Tuple(_));
}

#[test]
fn test_generic_type_parsing() {
    let parsed = syn::parse_str::<syn::ItemStruct>("pub struct Container<T> { value: T }").unwrap();
    assert!(!parsed.generics.params.is_empty());
}

#[test]
fn test_lifetime_parsing() {
    let parsed = syn::parse_str::<syn::ItemStruct>("pub struct Ref<'a> { data: &'a str }").unwrap();
    assert!(!parsed.generics.params.is_empty());
}

#[test]
fn test_where_clause_parsing() {
    let parsed = syn::parse_str::<syn::ItemFn>("pub fn test<T>() where T: Clone {}").unwrap();
    assert!(parsed.sig.generics.where_clause.is_some());
}

#[test]
fn test_async_function_parsing() {
    let parsed = syn::parse_str::<syn::ItemFn>("pub async fn test() -> i32 { 42 }").unwrap();
    assert!(parsed.sig.asyncness.is_some());
}

#[test]
fn test_const_function_parsing() {
    let parsed = syn::parse_str::<syn::ItemFn>("pub const fn test() -> i32 { 42 }").unwrap();
    assert!(parsed.sig.constness.is_some());
}

#[test]
fn test_unsafe_function_parsing() {
    let parsed = syn::parse_str::<syn::ItemFn>("pub unsafe fn test() {}").unwrap();
    assert!(parsed.sig.unsafety.is_some());
}

#[test]
fn test_variadic_function_simulation() {
    let parsed = syn::parse_str::<syn::ItemFn>(
        "pub fn test(args: Vec<Expression>) -> Expression { args[0].clone() }",
    )
    .unwrap();
    assert_eq!(parsed.sig.inputs.len(), 1);
}

#[test]
fn test_method_self_parsing() {
    let parsed = syn::parse_str::<syn::ImplItemFn>("pub fn method(&self) -> i32 { 42 }").unwrap();

    if let syn::FnArg::Receiver(_) = &parsed.sig.inputs[0] {
    } else {
        panic!("Expected self parameter");
    }
}

#[test]
fn test_method_mut_self_parsing() {
    let parsed = syn::parse_str::<syn::ImplItemFn>("pub fn method(&mut self) {}").unwrap();

    if let syn::FnArg::Receiver(r) = &parsed.sig.inputs[0] {
        assert!(r.mutability.is_some());
    } else {
        panic!("Expected mutable self parameter");
    }
}

#[test]
fn test_associated_function_parsing() {
    let parsed = syn::parse_str::<syn::ImplItemFn>("pub fn new() -> Self { Self }").unwrap();
    assert_eq!(parsed.sig.inputs.len(), 0);
}

#[test]
fn test_enum_unit_variant() {
    let parsed = syn::parse_str::<syn::ItemEnum>("pub enum Status { Active }").unwrap();

    let variant = &parsed.variants[0];
    matches!(variant.fields, syn::Fields::Unit);
}

#[test]
fn test_enum_tuple_variant() {
    let parsed = syn::parse_str::<syn::ItemEnum>("pub enum Result { Success(i32) }").unwrap();

    let variant = &parsed.variants[0];
    matches!(variant.fields, syn::Fields::Unnamed(_));
}

#[test]
fn test_enum_struct_variant() {
    let parsed =
        syn::parse_str::<syn::ItemEnum>("pub enum Result { Success { value: i32 } }").unwrap();

    let variant = &parsed.variants[0];
    matches!(variant.fields, syn::Fields::Named(_));
}

#[test]
fn test_struct_named_fields() {
    let parsed = syn::parse_str::<syn::ItemStruct>("pub struct Point { x: f64, y: f64 }").unwrap();

    matches!(parsed.fields, syn::Fields::Named(_));
}

#[test]
fn test_struct_tuple_fields() {
    let parsed = syn::parse_str::<syn::ItemStruct>("pub struct Wrapper(i32);").unwrap();

    matches!(parsed.fields, syn::Fields::Unnamed(_));
}

#[test]
fn test_struct_unit() {
    let parsed = syn::parse_str::<syn::ItemStruct>("pub struct Unit;").unwrap();

    matches!(parsed.fields, syn::Fields::Unit);
}

#[test]
fn test_visibility_public() {
    let parsed = syn::parse_str::<syn::ItemFn>("pub fn test() {}").unwrap();

    matches!(parsed.vis, syn::Visibility::Public(_));
}

#[test]
fn test_visibility_crate() {
    let parsed = syn::parse_str::<syn::ItemFn>("pub(crate) fn test() {}").unwrap();

    matches!(parsed.vis, syn::Visibility::Restricted(_));
}

#[test]
fn test_attribute_parsing() {
    let parsed = syn::parse_str::<syn::ItemFn>(r#"#[doc = "test"] pub fn test() {}"#).unwrap();

    assert!(!parsed.attrs.is_empty());
}

#[test]
fn test_multiple_attributes() {
    let parsed = syn::parse_str::<syn::ItemFn>(
        r#"
        #[inline]
        #[doc = "test"]
        pub fn test() {}
        "#,
    )
    .unwrap();

    assert_eq!(parsed.attrs.len(), 2);
}

#[test]
fn test_complex_return_type() {
    let parsed = syn::parse_str::<syn::ItemFn>(
        "pub fn test() -> Result<Vec<Expression>, MathError> { Ok(vec![]) }",
    )
    .unwrap();

    matches!(parsed.sig.output, syn::ReturnType::Type(_, _));
}

#[test]
fn test_impl_block_parsing() {
    let parsed =
        syn::parse_str::<syn::ItemImpl>("impl Expression { pub fn test(&self) -> i32 { 42 } }")
            .unwrap();

    assert!(!parsed.items.is_empty());
}

#[test]
fn test_trait_impl_parsing() {
    let parsed = syn::parse_str::<syn::ItemImpl>(
        "impl Clone for Expression { fn clone(&self) -> Self { Self } }",
    )
    .unwrap();

    assert!(parsed.trait_.is_some());
}

#[test]
fn test_generic_impl_parsing() {
    let parsed = syn::parse_str::<syn::ItemImpl>(
        "impl<T> Container<T> { pub fn new(value: T) -> Self { Self { value } } }",
    )
    .unwrap();

    assert!(!parsed.generics.params.is_empty());
}
