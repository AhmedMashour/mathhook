use crate::export::common::{extract_type_name, has_unbindable_types}; // ← Add import
use crate::export::types::{NameConverter, TypeCategory, TypeInfo, TypeMapper};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{FnArg, ItemImpl, Pat, Result, ReturnType, Type};

pub fn generate_python_impl_wrapper(
    impl_block: &ItemImpl,
    config: &crate::export::ExportConfig,
) -> Result<TokenStream> {
    let self_ty = &impl_block.self_ty;
    let type_name = extract_type_name(self_ty);
    let py_name = config.name.as_ref().map_or_else(
        || NameConverter::to_python_class_name(&type_name),
        |n| n.clone(),
    );
    let py_wrapper_name = syn::Ident::new(&py_name, proc_macro2::Span::call_site());

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

                generate_python_method(method, self_ty, &py_wrapper_name).ok()
            } else {
                None
            }
        })
        .collect();

    Ok(quote! {
        #[::pyo3::pymethods]
        impl #py_wrapper_name {
            #(#methods)*
        }
    })
}

fn generate_python_method(
    method: &syn::ImplItemFn,
    self_ty: &Type,
    _wrapper_name: &syn::Ident,
) -> Result<TokenStream> {
    let method_name = &method.sig.ident;
    let py_name = NameConverter::to_python_name(&method_name.to_string());

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
                    let py_ty = TypeMapper::to_python(&ty);
                    Some(quote! { #pat: #py_ty })
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

        let returns_self = match &method.sig.output {
            ReturnType::Default => false,
            ReturnType::Type(_, ty) => is_self_type(ty),
        };

        let return_type = if returns_self {
            quote! { Self }
        } else {
            match &method.sig.output {
                ReturnType::Default => quote! { () },
                ReturnType::Type(_, ty) => {
                    let type_info = TypeInfo::from_type(ty);
                    TypeMapper::to_python(&type_info)
                }
            }
        };

        let rust_type = syn::Ident::new(&type_name, proc_macro2::Span::call_site());

        let result_conversion =
            generate_python_static_method_result_conversion(&method.sig.output, self_ty);

        Ok(quote! {
            #[staticmethod]
            #[pyo3(name = #py_name)]
            pub fn #method_name(#(#params),*) -> pyo3::PyResult<#return_type> {
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
                    let py_ty = TypeMapper::to_python(&ty);
                    Some(quote! { #pat: #py_ty })
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
                TypeMapper::to_python(&type_info)
            }
        };

        let result_conversion =
            super::python::generate_python_result_conversion(&method.sig.output);

        let receiver = if is_mut {
            quote! { &mut self }
        } else {
            quote! { &self }
        };

        Ok(quote! {
            #[pyo3(name = #py_name)]
            pub fn #method_name(#receiver, #(#params),*) -> pyo3::PyResult<#return_type> {
                let result = self.inner.#method_name(#(#param_names),*);
                #result_conversion
            }
        })
    }
}

fn generate_python_static_method_result_conversion(
    output: &ReturnType,
    _self_ty: &Type,
) -> TokenStream {
    match output {
        ReturnType::Default => quote! { Ok(()) },
        ReturnType::Type(_, ty) => {
            if is_self_type(ty) {
                quote! { Ok(Self { inner: result }) }
            } else {
                let type_info = TypeInfo::from_type(ty);
                match type_info.category {
                    TypeCategory::Result => {
                        if let Some(ok_type) = type_info.inner_types.first() {
                            if is_self_type(&ok_type.rust_type) {
                                quote! {
                                    result.map(|v| Self { inner: v }).map_err(|e| {
                                        pyo3::exceptions::PyValueError::new_err(format!("{:?}", e))
                                    })
                                }
                            } else {
                                super::python::generate_python_result_ok_conversion(ok_type)
                            }
                        } else {
                            quote! {
                                result.map_err(|e| {
                                    pyo3::exceptions::PyValueError::new_err(format!("{:?}", e))
                                })
                            }
                        }
                    }
                    TypeCategory::MathHookCore(_) => {
                        if let Some(wrapper) = type_info.python_wrapper_ident() {
                            quote! { Ok(#wrapper { inner: result }) }
                        } else {
                            quote! { Ok(result) }
                        }
                    }
                    _ => quote! { Ok(result) },
                }
            }
        }
    }
}

fn is_self_type(ty: &Type) -> bool {
    match ty {
        Type::Path(type_path) => {
            if let Some(segment) = type_path.path.segments.last() {
                segment.ident == "Self"
            } else {
                false
            }
        }
        _ => false,
    }
}
