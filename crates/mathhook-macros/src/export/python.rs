use crate::export::common::{get_public_fields, has_default_derive, is_field_skipped};
use crate::export::docs::DocTransformer;
use crate::export::traits::PythonTraitGenerator;
use crate::export::types::{NameConverter, TypeCategory, TypeInfo, TypeMapper};
use crate::export::ExportConfig;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{FnArg, ItemFn, ItemStruct, Pat, Result, ReturnType};

pub use super::python_enum::generate_python_enum_wrapper;
pub use super::python_impl::generate_python_impl_wrapper;

pub fn generate_python_wrapper(func: &ItemFn, config: &ExportConfig) -> Result<TokenStream> {
    generate_python_function_wrapper(func, config)
}

fn generate_expr_ref_helper() -> TokenStream {
    quote! {
        enum __ExprRef<'a, T: Clone> {
            Borrowed(&'a T),
        }

        impl<'a, T: Clone> __ExprRef<'a, T> {
            #[inline]
            fn borrowed(value: &'a T) -> Self {
                Self::Borrowed(value)
            }

            #[inline]
            fn into_owned_if_needed(self) -> T {
                match self {
                    Self::Borrowed(e) => e.clone(),
                }
            }
        }
    }
}

pub fn generate_python_function_wrapper(
    func: &ItemFn,
    config: &ExportConfig,
) -> Result<TokenStream> {
    let func_name = &func.sig.ident;
    let py_wrapper_name = config
        .python_name
        .as_ref()
        .map(|name| format_ident!("py_{}", name.replace("-", "_")))
        .unwrap_or_else(|| format_ident!("py_{}", func_name));
    let py_display_name = config.name.as_ref().map_or_else(
        || NameConverter::to_python_name(&func_name.to_string()),
        |n| n.clone(),
    );

    let params: Vec<_> = func
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

    let has_mathhook_params = func.sig.inputs.iter().any(|arg| {
        if let FnArg::Typed(pat_type) = arg {
            TypeInfo::from_type(&pat_type.ty).is_mathhook_type()
        } else {
            false
        }
    });

    let param_names: Vec<_> = func
        .sig
        .inputs
        .iter()
        .filter_map(|arg| {
            if let FnArg::Typed(pat_type) = arg {
                if let Pat::Ident(pat_ident) = pat_type.pat.as_ref() {
                    let name = &pat_ident.ident;
                    let ty = TypeInfo::from_type(&pat_type.ty);
                    if ty.is_mathhook_type() {
                        Some(quote! { __ExprRef::borrowed(&#name.inner).into_owned_if_needed() })
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

    let return_type = match &func.sig.output {
        ReturnType::Default => quote! { () },
        ReturnType::Type(_, ty) => {
            let type_info = TypeInfo::from_type(ty);
            TypeMapper::to_python(&type_info)
        }
    };

    let result_conversion = generate_python_result_conversion(&func.sig.output);

    let expr_ref_helper = if has_mathhook_params {
        generate_expr_ref_helper()
    } else {
        quote! {}
    };

    let docs = DocTransformer::extract_doc_comments(&func.attrs);
    let docstring = if !docs.is_empty() {
        let doc_text = DocTransformer::to_python_docstring(&docs);
        quote! {
            #[doc = #doc_text]
        }
    } else {
        quote! {}
    };

    Ok(quote! {
        #docstring
        #[::pyo3::pyfunction(name = #py_display_name)]
        pub fn #py_wrapper_name(#(#params),*) -> pyo3::PyResult<#return_type> {
            #expr_ref_helper
            let result = #func_name(#(#param_names),*);
            #result_conversion
        }
    })
}

pub fn generate_python_struct_wrapper(
    struct_def: &ItemStruct,
    config: &ExportConfig,
) -> Result<TokenStream> {
    let struct_name = &struct_def.ident;
    let py_name = config.name.as_ref().map_or_else(
        || NameConverter::to_python_class_name(&struct_name.to_string()),
        |n| n.clone(),
    );
    let py_wrapper_name = format_ident!("{}", py_name);

    let has_default = has_default_derive(struct_def);
    let constructor =
        generate_python_constructor(struct_def, &py_wrapper_name, struct_name, has_default)?;
    let getters = generate_python_field_getters(struct_def)?;
    let setters = generate_python_field_setters(struct_def)?;
    let default_method = if has_default {
        generate_python_default_method(struct_name)
    } else {
        quote! {}
    };

    let trait_methods = PythonTraitGenerator::generate_methods(
        struct_name,
        &py_wrapper_name,
        config.trait_config.all_traits(),
    );

    let docs = DocTransformer::extract_doc_comments(&struct_def.attrs);
    let docstring = if !docs.is_empty() {
        let doc_text = DocTransformer::to_python_docstring(&docs);
        quote! {
            #[doc = #doc_text]
        }
    } else {
        quote! {}
    };

    Ok(quote! {
        #docstring
        #[::pyo3::pyclass(name = #py_name)]
        #[derive(Clone)]
        pub struct #py_wrapper_name {
            pub inner: #struct_name,
        }

        #[::pyo3::pymethods]
        impl #py_wrapper_name {
            #constructor
            #getters
            #setters
            #default_method
            #trait_methods
        }

        impl From<#struct_name> for #py_wrapper_name {
            fn from(inner: #struct_name) -> Self {
                Self { inner }
            }
        }
    })
}

fn generate_python_constructor(
    struct_def: &ItemStruct,
    _wrapper_name: &syn::Ident,
    struct_name: &syn::Ident,
    has_default: bool,
) -> Result<TokenStream> {
    let public_fields = get_public_fields(struct_def);

    if public_fields.is_empty() {
        if has_default {
            return Ok(quote! {
                #[new]
                pub fn new() -> Self {
                    Self { inner: #struct_name::default() }
                }
            });
        } else {
            return Ok(quote! {});
        }
    }

    if has_default {
        let param_names: Vec<_> = public_fields.iter().map(|(name, _)| name).collect();
        let param_types: Vec<_> = public_fields
            .iter()
            .map(|(_, ty)| {
                let type_info = TypeInfo::from_type(ty);
                TypeMapper::to_python(&type_info)
            })
            .collect();

        let signature_tokens: Vec<TokenStream> = param_names
            .iter()
            .map(|name| quote! { #name = None })
            .collect();

        let field_assignments: Vec<_> = public_fields
            .iter()
            .map(|(name, ty)| {
                let type_info = TypeInfo::from_type(ty);
                if type_info.is_mathhook_type() {
                    quote! {
                        #name: #name.map(|v| v.inner.clone()).unwrap_or_else(|| default_inner.#name.clone())
                    }
                } else {
                    quote! {
                        #name: #name.unwrap_or(default_inner.#name)
                    }
                }
            })
            .collect();

        Ok(quote! {
            #[new]
            #[pyo3(signature = (#(#signature_tokens),*))]
            pub fn new(#(#param_names: Option<#param_types>),*) -> Self {
                let default_inner = #struct_name::default();
                Self {
                    inner: #struct_name {
                        #(#field_assignments),*
                    }
                }
            }
        })
    } else {
        let param_names: Vec<_> = public_fields.iter().map(|(name, _)| name).collect();
        let param_types: Vec<_> = public_fields
            .iter()
            .map(|(_, ty)| {
                let type_info = TypeInfo::from_type(ty);
                TypeMapper::to_python(&type_info)
            })
            .collect();

        let field_assignments: Vec<_> = public_fields
            .iter()
            .map(|(name, ty)| {
                let type_info = TypeInfo::from_type(ty);
                if type_info.is_mathhook_type() {
                    quote! { #name: #name.inner.clone() }
                } else {
                    quote! { #name }
                }
            })
            .collect();

        Ok(quote! {
            #[new]
            pub fn new(#(#param_names: #param_types),*) -> Self {
                Self {
                    inner: #struct_name {
                        #(#field_assignments),*
                    }
                }
            }
        })
    }
}

fn generate_python_field_getters(struct_def: &ItemStruct) -> Result<TokenStream> {
    let getters: Vec<_> = struct_def
        .fields
        .iter()
        .filter_map(|field| {
            let field_name = field.ident.as_ref()?;
            if !matches!(field.vis, syn::Visibility::Public(_)) || is_field_skipped(field) {
                return None;
            }

            let ty = TypeInfo::from_type(&field.ty);
            if let Some(wrapper) = ty.python_wrapper_ident() {
                Some(quote! {
                    #[getter]
                    fn #field_name(&self) -> #wrapper {
                        #wrapper { inner: self.inner.#field_name.clone() }
                    }
                })
            } else {
                let rust_ty = &field.ty;
                Some(quote! {
                    #[getter]
                    fn #field_name(&self) -> #rust_ty {
                        self.inner.#field_name.clone()
                    }
                })
            }
        })
        .collect();

    Ok(quote! { #(#getters)* })
}

fn generate_python_field_setters(struct_def: &ItemStruct) -> Result<TokenStream> {
    let setters: Vec<_> = struct_def
        .fields
        .iter()
        .filter_map(|field| {
            let field_name = field.ident.as_ref()?;
            if !matches!(field.vis, syn::Visibility::Public(_)) || is_field_skipped(field) {
                return None;
            }

            let ty = TypeInfo::from_type(&field.ty);
            if let Some(wrapper) = ty.python_wrapper_ident() {
                Some(quote! {
                    #[setter]
                    fn #field_name(&mut self, value: #wrapper) {
                        self.inner.#field_name = value.inner.clone();
                    }
                })
            } else {
                let rust_ty = &field.ty;
                Some(quote! {
                    #[setter]
                    fn #field_name(&mut self, value: #rust_ty) {
                        self.inner.#field_name = value;
                    }
                })
            }
        })
        .collect();

    Ok(quote! { #(#setters)* })
}

fn generate_python_default_method(struct_name: &syn::Ident) -> TokenStream {
    quote! {
        #[staticmethod]
        pub fn default() -> Self {
            Self { inner: #struct_name::default() }
        }
    }
}

pub fn generate_python_result_conversion(output: &ReturnType) -> TokenStream {
    match output {
        ReturnType::Default => quote! { Ok(()) },
        ReturnType::Type(_, ty) => {
            let type_info = TypeInfo::from_type(ty);
            match type_info.category {
                TypeCategory::Result => {
                    if let Some(ok_type) = type_info.inner_types.first() {
                        generate_python_result_ok_conversion(ok_type)
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
                TypeCategory::Vec => {
                    if let Some(inner) = type_info.inner_types.first() {
                        if let Some(wrapper) = inner.python_wrapper_ident() {
                            quote! {
                                Ok(result.into_iter().map(|x| #wrapper { inner: x }).collect())
                            }
                        } else {
                            quote! { Ok(result) }
                        }
                    } else {
                        quote! { Ok(result) }
                    }
                }
                TypeCategory::Option => {
                    if let Some(inner) = type_info.inner_types.first() {
                        if let Some(wrapper) = inner.python_wrapper_ident() {
                            quote! { Ok(result.map(|x| #wrapper { inner: x })) }
                        } else {
                            quote! { Ok(result) }
                        }
                    } else {
                        quote! { Ok(result) }
                    }
                }
                _ => quote! { Ok(result) },
            }
        }
    }
}

pub fn generate_python_result_ok_conversion(ok_type: &TypeInfo) -> TokenStream {
    match &ok_type.category {
        TypeCategory::MathHookCore(_) => {
            if let Some(wrapper) = ok_type.python_wrapper_ident() {
                quote! {
                    result.map(|v| #wrapper { inner: v }).map_err(|e| {
                        pyo3::exceptions::PyValueError::new_err(format!("{:?}", e))
                    })
                }
            } else {
                quote! {
                    result.map_err(|e| {
                        pyo3::exceptions::PyValueError::new_err(format!("{:?}", e))
                    })
                }
            }
        }
        TypeCategory::Vec => {
            if let Some(inner) = ok_type.inner_types.first() {
                if let Some(wrapper) = inner.python_wrapper_ident() {
                    quote! {
                        result.map(|v| v.into_iter().map(|x| #wrapper { inner: x }).collect()).map_err(|e| {
                            pyo3::exceptions::PyValueError::new_err(format!("{:?}", e))
                        })
                    }
                } else {
                    quote! {
                        result.map_err(|e| {
                            pyo3::exceptions::PyValueError::new_err(format!("{:?}", e))
                        })
                    }
                }
            } else {
                quote! {
                    result.map_err(|e| {
                        pyo3::exceptions::PyValueError::new_err(format!("{:?}", e))
                    })
                }
            }
        }
        TypeCategory::Option => {
            if let Some(inner) = ok_type.inner_types.first() {
                if let Some(wrapper) = inner.python_wrapper_ident() {
                    quote! {
                        result.map(|v| v.map(|x| #wrapper { inner: x })).map_err(|e| {
                            pyo3::exceptions::PyValueError::new_err(format!("{:?}", e))
                        })
                    }
                } else {
                    quote! {
                        result.map_err(|e| {
                            pyo3::exceptions::PyValueError::new_err(format!("{:?}", e))
                        })
                    }
                }
            } else {
                quote! {
                    result.map_err(|e| {
                        pyo3::exceptions::PyValueError::new_err(format!("{:?}", e))
                    })
                }
            }
        }
        _ => quote! {
            result.map_err(|e| {
                pyo3::exceptions::PyValueError::new_err(format!("{:?}", e))
            })
        },
    }
}
