//! Trait-based operator and method generation for bindings

use proc_macro2::TokenStream;
use quote::quote;
use syn::{Ident, Meta, Result};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SupportedTrait {
    Add,
    Sub,
    Mul,
    Div,
    Neg,
    Display,
    PartialEq,
    PartialOrd,
    Clone,
}

#[allow(dead_code)]
impl SupportedTrait {
    pub fn from_ident(ident: &Ident) -> Option<Self> {
        let name = ident.to_string();
        match name.as_str() {
            "Add" => Some(Self::Add),
            "Sub" => Some(Self::Sub),
            "Mul" => Some(Self::Mul),
            "Div" => Some(Self::Div),
            "Neg" => Some(Self::Neg),
            "Display" => Some(Self::Display),
            "PartialEq" => Some(Self::PartialEq),
            "PartialOrd" => Some(Self::PartialOrd),
            "Clone" => Some(Self::Clone),
            _ => None,
        }
    }

    pub fn python_method_name(self) -> &'static str {
        match self {
            Self::Add => "__add__",
            Self::Sub => "__sub__",
            Self::Mul => "__mul__",
            Self::Div => "__truediv__",
            Self::Neg => "__neg__",
            Self::Display => "__str__",
            Self::PartialEq => "__eq__",
            Self::PartialOrd => "__lt__",
            Self::Clone => "clone",
        }
    }

    pub fn nodejs_method_name(self) -> Option<&'static str> {
        match self {
            Self::Display => Some("toString"),
            Self::PartialEq => Some("equals"),
            Self::PartialOrd => Some("compareTo"),
            Self::Clone => Some("clone"),
            _ => None,
        }
    }

    pub fn is_binary_op(self) -> bool {
        matches!(self, Self::Add | Self::Sub | Self::Mul | Self::Div)
    }

    pub fn is_unary_op(self) -> bool {
        matches!(self, Self::Neg)
    }

    pub fn rust_method_name(self) -> &'static str {
        match self {
            Self::Add => "add",
            Self::Sub => "sub",
            Self::Mul => "mul",
            Self::Div => "div",
            Self::Neg => "neg",
            Self::Display => "to_string",
            Self::PartialEq => "eq",
            Self::PartialOrd => "partial_cmp",
            Self::Clone => "clone",
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct TraitConfig {
    pub ops: Vec<SupportedTrait>,
    pub display: bool,
    pub eq: bool,
    pub ord: bool,
    pub clone: bool,
}

impl TraitConfig {
    pub fn from_attrs(attrs: &[Meta]) -> Result<Self> {
        let mut config = Self::default();

        for attr in attrs {
            match attr {
                Meta::NameValue(nv) if nv.path.is_ident("ops") => {
                    if let syn::Expr::Array(arr) = &nv.value {
                        for elem in &arr.elems {
                            if let syn::Expr::Path(path) = elem {
                                if let Some(ident) = path.path.get_ident() {
                                    if let Some(trait_type) = SupportedTrait::from_ident(ident) {
                                        config.ops.push(trait_type);
                                    }
                                }
                            }
                        }
                    }
                }
                Meta::NameValue(nv) if nv.path.is_ident("display") => {
                    if let syn::Expr::Lit(lit) = &nv.value {
                        if let syn::Lit::Bool(b) = &lit.lit {
                            config.display = b.value();
                        }
                    }
                }
                Meta::NameValue(nv) if nv.path.is_ident("eq") => {
                    if let syn::Expr::Lit(lit) = &nv.value {
                        if let syn::Lit::Bool(b) = &lit.lit {
                            config.eq = b.value();
                        }
                    }
                }
                Meta::NameValue(nv) if nv.path.is_ident("ord") => {
                    if let syn::Expr::Lit(lit) = &nv.value {
                        if let syn::Lit::Bool(b) = &lit.lit {
                            config.ord = b.value();
                        }
                    }
                }
                Meta::NameValue(nv) if nv.path.is_ident("clone") => {
                    if let syn::Expr::Lit(lit) = &nv.value {
                        if let syn::Lit::Bool(b) = &lit.lit {
                            config.clone = b.value();
                        }
                    }
                }
                _ => {}
            }
        }

        if config.display {
            config.ops.push(SupportedTrait::Display);
        }
        if config.eq {
            config.ops.push(SupportedTrait::PartialEq);
        }
        if config.ord {
            config.ops.push(SupportedTrait::PartialOrd);
        }
        if config.clone {
            config.ops.push(SupportedTrait::Clone);
        }

        config.ops.sort_by_key(|t| format!("{:?}", t));
        config.ops.dedup();

        Ok(config)
    }

    pub fn all_traits(&self) -> &[SupportedTrait] {
        &self.ops
    }
}

pub struct PythonTraitGenerator;

impl PythonTraitGenerator {
    pub fn generate_methods(
        struct_name: &Ident,
        wrapper_name: &Ident,
        traits: &[SupportedTrait],
    ) -> TokenStream {
        let methods: Vec<_> = traits
            .iter()
            .flat_map(|&trait_type| {
                Self::generate_method_body(struct_name, wrapper_name, trait_type)
            })
            .collect();

        quote! { #(#methods)* }
    }

    fn generate_method_body(
        struct_name: &Ident,
        wrapper_name: &Ident,
        trait_type: SupportedTrait,
    ) -> Option<TokenStream> {
        match trait_type {
            SupportedTrait::Add => Some(Self::generate_binary_op(wrapper_name, "__add__", "+")),
            SupportedTrait::Sub => Some(Self::generate_binary_op(wrapper_name, "__sub__", "-")),
            SupportedTrait::Mul => Some(Self::generate_binary_op(wrapper_name, "__mul__", "*")),
            SupportedTrait::Div => Some(Self::generate_binary_op(wrapper_name, "__truediv__", "/")),
            SupportedTrait::Neg => Some(Self::generate_unary_op(wrapper_name)),
            SupportedTrait::Display => Some(Self::generate_display(struct_name, wrapper_name)),
            SupportedTrait::PartialEq => Some(Self::generate_eq(struct_name, wrapper_name)),
            SupportedTrait::PartialOrd => Some(Self::generate_ord(struct_name, wrapper_name)),
            SupportedTrait::Clone => Some(Self::generate_clone(struct_name, wrapper_name)),
        }
    }

    fn generate_binary_op(wrapper_name: &Ident, py_method: &str, op: &str) -> TokenStream {
        let method_ident = syn::Ident::new(py_method, proc_macro2::Span::call_site());

        let operation = match op {
            "+" => quote! { self.inner.clone() + other.inner.clone() },
            "-" => quote! { self.inner.clone() - other.inner.clone() },
            "*" => quote! { self.inner.clone() * other.inner.clone() },
            "/" => quote! { self.inner.clone() / other.inner.clone() },
            _ => quote! { self.inner.clone() },
        };

        quote! {
            fn #method_ident(&self, other: &#wrapper_name) -> #wrapper_name {
                #wrapper_name { inner: #operation }
            }
        }
    }

    fn generate_unary_op(wrapper_name: &Ident) -> TokenStream {
        quote! {
            fn __neg__(&self) -> #wrapper_name {
                #wrapper_name { inner: -self.inner.clone() }
            }
        }
    }

    fn generate_display(_struct_name: &Ident, _wrapper_name: &Ident) -> TokenStream {
        quote! {
            fn __str__(&self) -> String {
                format!("{}", self.inner)
            }

            fn __repr__(&self) -> String {
                format!("{:?}", self.inner)
            }
        }
    }

    fn generate_eq(_struct_name: &Ident, wrapper_name: &Ident) -> TokenStream {
        quote! {
            fn __eq__(&self, other: &#wrapper_name) -> bool {
                self.inner == other.inner
            }

            fn __ne__(&self, other: &#wrapper_name) -> bool {
                self.inner != other.inner
            }
        }
    }

    fn generate_ord(_struct_name: &Ident, wrapper_name: &Ident) -> TokenStream {
        quote! {
            fn __lt__(&self, other: &#wrapper_name) -> bool {
                self.inner < other.inner
            }

            fn __le__(&self, other: &#wrapper_name) -> bool {
                self.inner <= other.inner
            }

            fn __gt__(&self, other: &#wrapper_name) -> bool {
                self.inner > other.inner
            }

            fn __ge__(&self, other: &#wrapper_name) -> bool {
                self.inner >= other.inner
            }
        }
    }

    fn generate_clone(_struct_name: &Ident, wrapper_name: &Ident) -> TokenStream {
        quote! {
            fn clone(&self) -> #wrapper_name {
                #wrapper_name {
                    inner: self.inner.clone(),
                }
            }
        }
    }
}

pub struct NodejsTraitGenerator;

impl NodejsTraitGenerator {
    pub fn generate_methods(
        struct_name: &Ident,
        wrapper_name: &Ident,
        traits: &[SupportedTrait],
    ) -> TokenStream {
        let methods: Vec<_> = traits
            .iter()
            .filter_map(|&trait_type| Self::generate_method(struct_name, wrapper_name, trait_type))
            .collect();

        quote! { #(#methods)* }
    }

    fn generate_method(
        _struct_name: &Ident,
        wrapper_name: &Ident,
        trait_type: SupportedTrait,
    ) -> Option<TokenStream> {
        match trait_type {
            SupportedTrait::Display => Some(Self::generate_display()),
            SupportedTrait::PartialEq => Some(Self::generate_eq(wrapper_name)),
            SupportedTrait::PartialOrd => Some(Self::generate_ord(wrapper_name)),
            SupportedTrait::Clone => Some(Self::generate_clone(wrapper_name)),
            _ => None,
        }
    }

    fn generate_display() -> TokenStream {
        quote! {
            #[napi]
            pub fn to_string(&self) -> String {
                format!("{}", self.inner)
            }
        }
    }

    fn generate_eq(wrapper_name: &Ident) -> TokenStream {
        quote! {
            #[napi]
            pub fn equals(&self, other: &#wrapper_name) -> bool {
                self.inner == other.inner
            }
        }
    }

    fn generate_ord(wrapper_name: &Ident) -> TokenStream {
        quote! {
            #[napi]
            pub fn compare_to(&self, other: &#wrapper_name) -> i32 {
                use std::cmp::Ordering;
                match self.inner.partial_cmp(&other.inner) {
                    Some(Ordering::Less) => -1,
                    Some(Ordering::Equal) => 0,
                    Some(Ordering::Greater) => 1,
                    None => 0,
                }
            }
        }
    }

    fn generate_clone(wrapper_name: &Ident) -> TokenStream {
        quote! {
            #[napi]
            pub fn clone(&self) -> #wrapper_name {
                #wrapper_name {
                    inner: self.inner.clone(),
                }
            }
        }
    }
}
