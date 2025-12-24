use crate::export::common::{extract_type_name, has_unbindable_types};
use crate::export::types::{NameConverter, TypeInfo, TypeMapper};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{FnArg, ItemImpl, Pat, Result, ReturnType, Type};

pub fn generate_nodejs_impl_wrapper(
    impl_block: &ItemImpl,
    config: &crate::export::ExportConfig,
) -> Result<TokenStream> {
    let self_ty = &impl_block.self_ty;
    let type_name = extract_type_name(self_ty);
    let js_name = config.name.as_ref().map_or_else(
        || NameConverter::to_javascript_class_name(&type_name),
        |n| n.clone(),
    );
    let js_wrapper_name = syn::Ident::new(&js_name, proc_macro2::Span::call_site());

    let methods: Vec<_> = impl_block
        .items
        .iter()
        .filter_map(|item| {
            if let syn::ImplItem::Fn(method) = item {
                if !matches!(method.vis, syn::Visibility::Public(_)) {
                    return None;
                }

                // Skip methods with unbindable types
                if has_unbindable_types(method) {
                    return None;
                }

                generate_nodejs_method(method, self_ty).ok()
            } else {
                None
            }
        })
        .collect();

    Ok(quote! {
        #[napi_derive::napi]
        impl #js_wrapper_name {
            #(#methods)*
        }
    })
}

fn generate_nodejs_method(method: &syn::ImplItemFn, self_ty: &Type) -> Result<TokenStream> {
    let method_name = &method.sig.ident;
    let js_name = NameConverter::to_javascript_name(&method_name.to_string());

    let is_static = method
        .sig
        .inputs
        .first()
        .map(|arg| !matches!(arg, FnArg::Receiver(_)))
        .unwrap_or(true);

    let is_mut = method
        .sig
        .inputs
        .first()
        .map(|arg| {
            if let FnArg::Receiver(recv) = arg {
                recv.mutability.is_some()
            } else {
                false
            }
        })
        .unwrap_or(false);

    let type_name = extract_type_name(self_ty);

    if is_static {
        let params: Vec<_> = method
            .sig
            .inputs
            .iter()
            .filter_map(|arg| {
                if let FnArg::Typed(pat_type) = arg {
                    let pat = &pat_type.pat;
                    let ty = TypeInfo::from_type(&pat_type.ty);
                    let js_ty = TypeMapper::to_nodejs(&ty);
                    Some(quote! { #pat: #js_ty })
                } else {
                    None
                }
            })
            .collect();

        let param_names: Vec<_> = method
            .sig
            .inputs
            .iter()
            .filter_map(|arg| {
                if let FnArg::Typed(pat_type) = arg {
                    if let Pat::Ident(pat_ident) = pat_type.pat.as_ref() {
                        let name = &pat_ident.ident;
                        let ty = TypeInfo::from_type(&pat_type.ty);
                        if ty.needs_conversion() {
                            Some(quote! { #name.inner.clone() })
                        } else {
                            Some(quote! { #name })
                        }
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect();

        let return_type = match &method.sig.output {
            ReturnType::Default => quote! { () },
            ReturnType::Type(_, ty) => {
                let type_info = TypeInfo::from_type(ty);
                TypeMapper::to_nodejs(&type_info)
            }
        };

        let rust_type = syn::Ident::new(&type_name, proc_macro2::Span::call_site());

        let result_conversion =
            super::nodejs::generate_nodejs_result_conversion(&method.sig.output);

        Ok(quote! {
            #[napi(factory, js_name = #js_name)]
            pub fn #method_name(#(#params),*) -> napi::Result<#return_type> {
                let result = #rust_type::#method_name(#(#param_names),*);
                #result_conversion
            }
        })
    } else {
        let params: Vec<_> = method
            .sig
            .inputs
            .iter()
            .skip(1)
            .filter_map(|arg| {
                if let FnArg::Typed(pat_type) = arg {
                    let pat = &pat_type.pat;
                    let ty = TypeInfo::from_type(&pat_type.ty);
                    let js_ty = TypeMapper::to_nodejs(&ty);
                    Some(quote! { #pat: #js_ty })
                } else {
                    None
                }
            })
            .collect();

        let param_names: Vec<_> = method
            .sig
            .inputs
            .iter()
            .skip(1)
            .filter_map(|arg| {
                if let FnArg::Typed(pat_type) = arg {
                    if let Pat::Ident(pat_ident) = pat_type.pat.as_ref() {
                        let name = &pat_ident.ident;
                        let ty = TypeInfo::from_type(&pat_type.ty);
                        if ty.needs_conversion() {
                            Some(quote! { #name.inner.clone() })
                        } else {
                            Some(quote! { #name })
                        }
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect();

        let return_type = match &method.sig.output {
            ReturnType::Default => quote! { () },
            ReturnType::Type(_, ty) => {
                let type_info = TypeInfo::from_type(ty);
                TypeMapper::to_nodejs(&type_info)
            }
        };

        let result_conversion =
            super::nodejs::generate_nodejs_result_conversion(&method.sig.output);

        let receiver = if is_mut {
            quote! { &mut self }
        } else {
            quote! { &self }
        };

        Ok(quote! {
            #[napi(js_name = #js_name)]
            pub fn #method_name(#receiver, #(#params),*) -> napi::Result<#return_type> {
                let result = self.inner.#method_name(#(#param_names),*);
                #result_conversion
            }
        })
    }
}
