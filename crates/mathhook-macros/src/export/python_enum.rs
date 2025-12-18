use crate::export::common::generate_enum_variant_match_arms;
use crate::export::types::{NameConverter, TypeInfo, TypeMapper};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{ItemEnum, Result};

pub fn generate_python_enum_wrapper(
    enum_def: &ItemEnum,
    config: &crate::export::ExportConfig,
) -> Result<TokenStream> {
    let enum_name = &enum_def.ident;
    let py_name = config.name.as_ref().map_or_else(
        || NameConverter::to_python_class_name(&enum_name.to_string()),
        |n| n.clone(),
    );
    let py_enum_ident = format_ident!("{}", py_name);

    let variant_match_arms = generate_enum_variant_match_arms(enum_def);
    let variant_accessors = generate_python_variant_accessors(enum_def)?;
    let variant_extractors = generate_python_variant_extractors(enum_def)?;
    let variant_constructors = generate_python_variant_constructors(enum_def)?;

    Ok(quote! {
        #[pyo3::pyclass(name = #py_name)]
        #[derive(Clone)]
        pub struct #py_enum_ident {
            pub inner: #enum_name,
        }

        impl #py_enum_ident {
            fn get_variant_name(inner: &#enum_name) -> String {
                match inner {
                    #(#variant_match_arms)*
                }
            }
        }

        #[pyo3::pymethods]
        impl #py_enum_ident {
            #[getter]
            fn variant_type(&self) -> String {
                Self::get_variant_name(&self.inner)
            }

            #variant_accessors
            #variant_extractors
            #variant_constructors

            fn __str__(&self) -> String {
                format!("{:?}", self.inner)
            }

            fn __repr__(&self) -> String {
                format!("{}({:?})", #py_name, self.inner)
            }
        }

        impl From<#enum_name> for #py_enum_ident {
            fn from(inner: #enum_name) -> Self {
                Self { inner }
            }
        }
    })
}

fn generate_python_variant_accessors(enum_def: &ItemEnum) -> Result<TokenStream> {
    let enum_name = &enum_def.ident;
    let accessors: Vec<_> = enum_def
        .variants
        .iter()
        .map(|variant| {
            let variant_name = &variant.ident;
            let checker_name_str = NameConverter::to_python_name(&format!(
                "is_{}",
                variant_name.to_string().to_lowercase()
            ));
            let checker_name = format_ident!("{}", checker_name_str);

            match &variant.fields {
                syn::Fields::Unit => {
                    quote! {
                        pub fn #checker_name(&self) -> bool {
                            matches!(self.inner, #enum_name::#variant_name)
                        }
                    }
                }
                syn::Fields::Unnamed(_) => {
                    quote! {
                        pub fn #checker_name(&self) -> bool {
                            matches!(self.inner, #enum_name::#variant_name(..))
                        }
                    }
                }
                syn::Fields::Named(_) => {
                    quote! {
                        pub fn #checker_name(&self) -> bool {
                            matches!(self.inner, #enum_name::#variant_name { .. })
                        }
                    }
                }
            }
        })
        .collect();

    Ok(quote! { #(#accessors)* })
}

fn generate_python_variant_extractors(enum_def: &ItemEnum) -> Result<TokenStream> {
    let enum_name = &enum_def.ident;
    let extractors: Vec<_> = enum_def
        .variants
        .iter()
        .filter_map(|variant| {
            let variant_name = &variant.ident;
            let extractor_name = format_ident!(
                "as_{}",
                NameConverter::variant_to_python_discriminator(&variant_name.to_string())
            );

            match &variant.fields {
                syn::Fields::Unit => None,
                syn::Fields::Unnamed(fields) if fields.unnamed.len() == 1 => {
                    let field_ty = &fields.unnamed.first()?.ty;
                    let type_info = TypeInfo::from_type(field_ty);
                    let return_ty = TypeMapper::to_python(&type_info);
                    let conversion = if let Some(wrapper) = type_info.python_wrapper_ident() {
                        quote! { #wrapper { inner: v.clone() } }
                    } else {
                        quote! { v.clone() }
                    };

                    Some(quote! {
                        pub fn #extractor_name(&self) -> pyo3::PyResult<#return_ty> {
                            match &self.inner {
                                #enum_name::#variant_name(v) => Ok(#conversion),
                                _ => Err(pyo3::exceptions::PyTypeError::new_err(
                                    format!("expected {} variant", stringify!(#variant_name))
                                )),
                            }
                        }
                    })
                }
                syn::Fields::Unnamed(fields) => {
                    let field_count = fields.unnamed.len();
                    let field_types: Vec<_> = fields
                        .unnamed
                        .iter()
                        .map(|f| TypeInfo::from_type(&f.ty))
                        .collect();
                    let return_types: Vec<_> = field_types
                        .iter()
                        .map(TypeMapper::to_python)
                        .collect();
                    let field_names: Vec<_> = (0..field_count)
                        .map(|i| format_ident!("v{}", i))
                        .collect();
                    let conversions: Vec<_> = field_types
                        .iter()
                        .zip(field_names.iter())
                        .map(|(ty, name)| {
                            if let Some(wrapper) = ty.python_wrapper_ident() {
                                quote! { #wrapper { inner: #name.clone() } }
                            } else {
                                quote! { #name.clone() }
                            }
                        })
                        .collect();

                    Some(quote! {
                        pub fn #extractor_name(&self) -> pyo3::PyResult<(#(#return_types),*)> {
                            match &self.inner {
                                #enum_name::#variant_name(#(#field_names),*) => Ok((#(#conversions),*)),
                                _ => Err(pyo3::exceptions::PyTypeError::new_err(
                                    format!("expected {} variant", stringify!(#variant_name))
                                )),
                            }
                        }
                    })
                }
                syn::Fields::Named(fields) => {
                    let field_names: Vec<_> = fields
                        .named
                        .iter()
                        .filter_map(|f| f.ident.as_ref())
                        .collect();
                    let field_types: Vec<_> = fields
                        .named
                        .iter()
                        .map(|f| TypeInfo::from_type(&f.ty))
                        .collect();
                    let py_field_names: Vec<_> = field_names
                        .iter()
                        .map(|name| name.to_string())
                        .collect();
                    let conversions: Vec<_> = field_types
                        .iter()
                        .zip(field_names.iter())
                        .map(|(ty, name)| {
                            if let Some(wrapper) = ty.python_wrapper_ident() {
                                quote! { #wrapper { inner: #name.clone() } }
                            } else {
                                quote! { #name.clone() }
                            }
                        })
                        .collect();

                    Some(quote! {
                        pub fn #extractor_name(&self, py: pyo3::Python) -> pyo3::PyResult<pyo3::Py<pyo3::types::PyDict>> {
                            match &self.inner {
                                #enum_name::#variant_name { #(#field_names),* } => {
                                    let dict = pyo3::types::PyDict::new(py);
                                    #(
                                        dict.set_item(#py_field_names, #conversions)?;
                                    )*
                                    Ok(dict.unbind())
                                },
                                _ => Err(pyo3::exceptions::PyTypeError::new_err(
                                    format!("expected {} variant", stringify!(#variant_name))
                                )),
                            }
                        }
                    })
                }
            }
        })
        .collect();

    Ok(quote! { #(#extractors)* })
}

fn generate_python_variant_constructors(enum_def: &ItemEnum) -> Result<TokenStream> {
    let enum_name = &enum_def.ident;
    let constructors: Vec<_> = enum_def
        .variants
        .iter()
        .filter_map(|variant| {
            let variant_name = &variant.ident;
            let constructor_name = format_ident!(
                "{}",
                NameConverter::variant_to_python_discriminator(&variant_name.to_string())
            );

            match &variant.fields {
                syn::Fields::Unit => Some(quote! {
                    #[staticmethod]
                    pub fn #constructor_name() -> Self {
                        Self { inner: #enum_name::#variant_name }
                    }
                }),
                syn::Fields::Unnamed(fields) if fields.unnamed.len() == 1 => {
                    let field_ty = &fields.unnamed.first()?.ty;
                    let type_info = TypeInfo::from_type(field_ty);
                    let param_ty = TypeMapper::to_python(&type_info);
                    let conversion = if type_info.is_mathhook_type() {
                        quote! { value.inner.clone() }
                    } else {
                        quote! { value }
                    };

                    Some(quote! {
                        #[staticmethod]
                        pub fn #constructor_name(value: #param_ty) -> Self {
                            Self { inner: #enum_name::#variant_name(#conversion) }
                        }
                    })
                }
                syn::Fields::Unnamed(fields) => {
                    let field_count = fields.unnamed.len();
                    let field_types: Vec<_> = fields
                        .unnamed
                        .iter()
                        .map(|f| TypeInfo::from_type(&f.ty))
                        .collect();
                    let param_types: Vec<_> = field_types
                        .iter()
                        .map(TypeMapper::to_python)
                        .collect();
                    let param_names: Vec<_> = (0..field_count)
                        .map(|i| format_ident!("value{}", i))
                        .collect();
                    let conversions: Vec<_> = field_types
                        .iter()
                        .zip(param_names.iter())
                        .map(|(ty, name)| {
                            if ty.is_mathhook_type() {
                                quote! { #name.inner.clone() }
                            } else {
                                quote! { #name }
                            }
                        })
                        .collect();

                    Some(quote! {
                        #[staticmethod]
                        pub fn #constructor_name(#(#param_names: #param_types),*) -> Self {
                            Self { inner: #enum_name::#variant_name(#(#conversions),*) }
                        }
                    })
                }
                syn::Fields::Named(fields) => {
                    let field_names: Vec<_> = fields
                        .named
                        .iter()
                        .filter_map(|f| f.ident.as_ref())
                        .collect();
                    let field_types: Vec<_> = fields
                        .named
                        .iter()
                        .map(|f| TypeInfo::from_type(&f.ty))
                        .collect();
                    let param_types: Vec<_> = field_types
                        .iter()
                        .map(TypeMapper::to_python)
                        .collect();
                    let conversions: Vec<_> = field_types
                        .iter()
                        .zip(field_names.iter())
                        .map(|(ty, name)| {
                            if ty.is_mathhook_type() {
                                quote! { #name: #name.inner.clone() }
                            } else if matches!(&ty.rust_type, syn::Type::Path(p) if p.path.is_ident("String")) {
                                quote! { #name: #name.to_string() }
                            } else {
                                quote! { #name }
                            }
                        })
                        .collect();

                    Some(quote! {
                        #[staticmethod]
                        pub fn #constructor_name(#(#field_names: #param_types),*) -> Self {
                            Self { inner: #enum_name::#variant_name { #(#conversions),* } }
                        }
                    })
                }
            }
        })
        .collect();

    Ok(quote! { #(#constructors)* })
}
