use proc_macro2::TokenStream;
use quote::quote;
use syn::{Field, ItemEnum, ItemStruct, Type};

/// Check if a field has #[mathhook(skip)] attribute
pub fn is_field_skipped(field: &Field) -> bool {
    field.attrs.iter().any(|attr| {
        if attr.path().is_ident("mathhook") {
            attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("skip") {
                    return Ok(());
                }
                Err(meta.error(""))
            })
            .is_ok()
        } else {
            false
        }
    })
}

/// Check if a struct has #[derive(Default)]
pub fn has_default_derive(struct_def: &ItemStruct) -> bool {
    struct_def.attrs.iter().any(|attr| {
        if attr.path().is_ident("derive") {
            attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("Default") {
                    return Ok(());
                }
                Err(meta.error(""))
            })
            .is_ok()
        } else {
            false
        }
    })
}

/// Get public, non-skipped fields from a struct
pub fn get_public_fields(struct_def: &ItemStruct) -> Vec<(&syn::Ident, &Type)> {
    struct_def
        .fields
        .iter()
        .filter_map(|field| {
            if let Some(field_name) = &field.ident {
                if matches!(field.vis, syn::Visibility::Public(_)) && !is_field_skipped(field) {
                    Some((field_name, &field.ty))
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect()
}

/// Extract type name from a Type
pub fn extract_type_name(ty: &Type) -> String {
    match ty {
        Type::Path(type_path) => type_path
            .path
            .segments
            .last()
            .map(|seg| seg.ident.to_string())
            .unwrap_or_else(|| "Unknown".to_string()),
        _ => "Unknown".to_string(),
    }
}

/// Generate match arms for enum variant names
pub fn generate_enum_variant_match_arms(enum_def: &ItemEnum) -> Vec<TokenStream> {
    let enum_name = &enum_def.ident;
    enum_def
        .variants
        .iter()
        .map(|variant| {
            let variant_name = &variant.ident;
            let variant_str = variant_name.to_string().to_lowercase();
            match &variant.fields {
                syn::Fields::Unit => {
                    quote! { #enum_name::#variant_name => #variant_str.to_string(), }
                }
                syn::Fields::Unnamed(_) => {
                    quote! { #enum_name::#variant_name(..) => #variant_str.to_string(), }
                }
                syn::Fields::Named(_) => {
                    quote! { #enum_name::#variant_name { .. } => #variant_str.to_string(), }
                }
            }
        })
        .collect()
}
