use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use syn::Type;

/// Type category classification
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TypeCategory {
    Primitive(PrimitiveKind),
    String,
    Unit,
    MathHookCore(String),
    Option,
    Result,
    Vec,
    HashMap,
    HashSet,
    Tuple,
    Custom(String),
}

impl TypeCategory {
    /// Check if this type category can be bound to Python/Node.js
    pub fn is_bindable(&self) -> bool {
        match self {
            TypeCategory::Custom(name) => {
                is_valid_rust_ident(name)
                    && !name.starts_with("dyn ")
                    && !name.starts_with("impl ")
                    && !name.starts_with('[')
                    && !name.contains("Fn(")
                    && !name.contains("FnMut(")
                    && !name.contains("FnOnce(")
                    && !name.contains("Box<dyn")
                    && !name.contains('&')
                    && !name.contains('*')
                    && !name.contains("HashMap")
                    && !name.contains("HashSet")
            }
            _ => true,
        }
    }
}

/// Primitive type kinds
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PrimitiveKind {
    I8,
    I16,
    I32,
    I64,
    U8,
    U16,
    U32,
    U64,
    F32,
    F64,
    Bool,
    Char,
    Usize,
}

/// Type information extracted from Rust type
#[derive(Debug, Clone)]
pub struct TypeInfo {
    pub category: TypeCategory,
    pub inner_types: Vec<TypeInfo>,
    pub rust_type: syn::Type,
}

impl TypeInfo {
    /// Create from syn::Type
    pub fn from_type(ty: &Type) -> Self {
        let rust_type = ty.clone();
        match ty {
            Type::Path(type_path) => {
                let segments: Vec<_> = type_path
                    .path
                    .segments
                    .iter()
                    .map(|s| s.ident.to_string())
                    .collect();
                let type_str = segments.join("::");

                let last_segment = type_path.path.segments.last();
                let args = last_segment.and_then(|s| match &s.arguments {
                    syn::PathArguments::AngleBracketed(args) => Some(&args.args),
                    _ => None,
                });

                if type_str == "String" || type_str.ends_with("::String") {
                    TypeInfo {
                        category: TypeCategory::String,
                        inner_types: vec![],
                        rust_type: rust_type.clone(),
                    }
                } else if let Some(prim) = Self::parse_primitive(&type_str) {
                    TypeInfo {
                        category: TypeCategory::Primitive(prim),
                        inner_types: vec![],
                        rust_type: rust_type.clone(),
                    }
                } else if type_str == "Option" || type_str.ends_with("::Option") {
                    let inner = args
                        .and_then(|a| a.first())
                        .and_then(|arg| {
                            if let syn::GenericArgument::Type(ty) = arg {
                                Some(Self::from_type(ty))
                            } else {
                                None
                            }
                        })
                        .unwrap_or_else(|| TypeInfo {
                            category: TypeCategory::Unit,
                            inner_types: vec![],
                            rust_type: syn::parse_quote!(()),
                        });
                    TypeInfo {
                        category: TypeCategory::Option,
                        inner_types: vec![inner],
                        rust_type: rust_type.clone(),
                    }
                } else if type_str == "Result" || type_str.ends_with("::Result") {
                    let ok_type = args
                        .and_then(|a| a.first())
                        .and_then(|arg| {
                            if let syn::GenericArgument::Type(ty) = arg {
                                Some(Self::from_type(ty))
                            } else {
                                None
                            }
                        })
                        .unwrap_or_else(|| TypeInfo {
                            category: TypeCategory::Unit,
                            inner_types: vec![],
                            rust_type: syn::parse_quote!(()),
                        });
                    TypeInfo {
                        category: TypeCategory::Result,
                        inner_types: vec![ok_type],
                        rust_type: rust_type.clone(),
                    }
                } else if type_str == "Vec" || type_str.ends_with("::Vec") {
                    let inner = args
                        .and_then(|a| a.first())
                        .and_then(|arg| {
                            if let syn::GenericArgument::Type(ty) = arg {
                                Some(Self::from_type(ty))
                            } else {
                                None
                            }
                        })
                        .unwrap_or_else(|| TypeInfo {
                            category: TypeCategory::Unit,
                            inner_types: vec![],
                            rust_type: syn::parse_quote!(()),
                        });
                    TypeInfo {
                        category: TypeCategory::Vec,
                        inner_types: vec![inner],
                        rust_type: rust_type.clone(),
                    }
                } else if type_str == "HashMap" || type_str.ends_with("::HashMap") {
                    let mut inner_types = vec![];
                    if let Some(args) = args {
                        for arg in args {
                            if let syn::GenericArgument::Type(ty) = arg {
                                inner_types.push(Self::from_type(ty));
                            }
                        }
                    }
                    TypeInfo {
                        category: TypeCategory::HashMap,
                        inner_types,
                        rust_type: rust_type.clone(),
                    }
                } else if type_str == "HashSet" || type_str.ends_with("::HashSet") {
                    let inner = args
                        .and_then(|a| a.first())
                        .and_then(|arg| {
                            if let syn::GenericArgument::Type(ty) = arg {
                                Some(Self::from_type(ty))
                            } else {
                                None
                            }
                        })
                        .unwrap_or_else(|| TypeInfo {
                            category: TypeCategory::Unit,
                            inner_types: vec![],
                            rust_type: syn::parse_quote!(()),
                        });
                    TypeInfo {
                        category: TypeCategory::HashSet,
                        inner_types: vec![inner],
                        rust_type: rust_type.clone(),
                    }
                } else if Self::is_mathhook_core_type(&type_str) {
                    TypeInfo {
                        category: TypeCategory::MathHookCore(
                            type_str.split("::").last().unwrap_or(&type_str).to_string(),
                        ),
                        inner_types: vec![],
                        rust_type: rust_type.clone(),
                    }
                } else {
                    TypeInfo {
                        category: TypeCategory::Custom(type_str.clone()),
                        inner_types: vec![],
                        rust_type: rust_type.clone(),
                    }
                }
            }
            Type::Tuple(tuple) => {
                let inner_types: Vec<_> = tuple.elems.iter().map(Self::from_type).collect();
                TypeInfo {
                    category: TypeCategory::Tuple,
                    inner_types,
                    rust_type: rust_type.clone(),
                }
            }
            Type::Reference(_) => TypeInfo {
                category: TypeCategory::Custom("reference".to_string()),
                inner_types: vec![],
                rust_type: rust_type.clone(),
            },
            _ => TypeInfo {
                category: TypeCategory::Custom("unknown".to_string()),
                inner_types: vec![],
                rust_type: rust_type.clone(),
            },
        }
    }

    fn parse_primitive(type_str: &str) -> Option<PrimitiveKind> {
        match type_str {
            "i8" => Some(PrimitiveKind::I8),
            "i16" => Some(PrimitiveKind::I16),
            "i32" => Some(PrimitiveKind::I32),
            "i64" => Some(PrimitiveKind::I64),
            "u8" => Some(PrimitiveKind::U8),
            "u16" => Some(PrimitiveKind::U16),
            "u32" => Some(PrimitiveKind::U32),
            "u64" => Some(PrimitiveKind::U64),
            "f32" => Some(PrimitiveKind::F32),
            "f64" => Some(PrimitiveKind::F64),
            "bool" => Some(PrimitiveKind::Bool),
            "char" => Some(PrimitiveKind::Char),
            "usize" => Some(PrimitiveKind::Usize),
            _ => None,
        }
    }

    fn is_mathhook_core_type(type_str: &str) -> bool {
        type_str.starts_with("mathhook_core::")
            || matches!(
                type_str,
                "Expression"
                    | "Symbol"
                    | "Pattern"
                    | "Number"
                    | "Rational"
                    | "Matrix"
                    | "Function"
                    | "Derivative"
                    | "Integral"
                    | "Limit"
                    | "Sum"
                    | "Product"
                    | "EvalContext"
            )
    }

    #[allow(dead_code)]
    /// Check if value is a primitive
    pub fn is_primitive(&self) -> bool {
        matches!(
            self.category,
            TypeCategory::Primitive(_) | TypeCategory::Unit
        )
    }

    /// Check if this type is a MathHook core type
    pub fn is_mathhook_type(&self) -> bool {
        matches!(self.category, TypeCategory::MathHookCore(_))
    }

    /// Check if this type can be used in field getters/setters
    /// Returns true only for simple types that can be directly exposed
    pub fn is_field_bindable(&self) -> bool {
        match &self.category {
            TypeCategory::Primitive(_) | TypeCategory::String | TypeCategory::Unit => true,
            TypeCategory::MathHookCore(_) => true,
            TypeCategory::Vec | TypeCategory::HashMap | TypeCategory::HashSet => false,
            TypeCategory::Option => self
                .inner_types
                .first()
                .is_some_and(|inner| inner.is_field_bindable()),
            TypeCategory::Result => false,
            TypeCategory::Tuple => false,
            TypeCategory::Custom(name) => {
                is_valid_rust_ident(name)
                    && !name.starts_with("dyn ")
                    && !name.starts_with("impl ")
                    && !name.starts_with('[')
                    && !name.contains("Fn(")
                    && !name.contains("FnMut(")
                    && !name.contains("FnOnce(")
                    && !name.contains("Box<dyn")
                    && !name.contains('&')
                    && !name.contains('*')
                    && !name.contains("Duration")
                    && !name.contains("Instant")
                    && !name.contains("PathBuf")
                    && !name.contains("Path")
            }
        }
    }

    /// Get the Python wrapper identifier for this type
    pub fn python_wrapper_ident(&self) -> Option<Ident> {
        match &self.category {
            TypeCategory::MathHookCore(name) => Some(format_ident!("Py{}", name)),
            _ => None,
        }
    }

    /// Get the Node.js wrapper identifier for this type
    pub fn nodejs_wrapper_ident(&self) -> Option<Ident> {
        match &self.category {
            TypeCategory::MathHookCore(name) => Some(format_ident!("Js{}", name)),
            _ => None,
        }
    }

    #[allow(dead_code)]
    /// Get the Rust type name (for MathHook core types)
    pub fn rust_type_name(&self) -> Option<&str> {
        match &self.category {
            TypeCategory::MathHookCore(name) => Some(name.as_str()),
            _ => None,
        }
    }

    /// Check if this type needs conversion in bindings
    /// Simple types like primitives don't need conversion, complex types do
    pub fn needs_conversion(&self) -> bool {
        !matches!(
            &self.category,
            TypeCategory::Primitive(_) | TypeCategory::String | TypeCategory::Unit
        )
    }
}

/// Maps Rust types to Python and Node.js types
pub struct TypeMapper;

impl TypeMapper {
    /// Map a type to its Python equivalent
    pub fn to_python(info: &TypeInfo) -> TokenStream {
        match &info.category {
            TypeCategory::Primitive(kind) => Self::primitive_to_python(kind),
            TypeCategory::String => quote! { &str },
            TypeCategory::Unit => quote! { () },
            TypeCategory::MathHookCore(name) if is_valid_rust_ident(name) => {
                let py_name = format!("Py{}", name);
                let ident = syn::Ident::new(&py_name, proc_macro2::Span::call_site());
                quote! { #ident }
            }
            TypeCategory::MathHookCore(_) => quote! { pyo3::PyObject },
            TypeCategory::Option => {
                if let Some(inner) = info.inner_types.first() {
                    let inner_py = Self::to_python(inner);
                    quote! { Option<#inner_py> }
                } else {
                    quote! { Option<pyo3::PyObject> }
                }
            }
            TypeCategory::Result => {
                if let Some(ok_type) = info.inner_types.first() {
                    let ok_py = Self::to_python(ok_type);
                    quote! { pyo3::PyResult<#ok_py> }
                } else {
                    quote! { pyo3::PyResult<pyo3::PyObject> }
                }
            }
            TypeCategory::Vec => {
                if let Some(inner) = info.inner_types.first() {
                    let inner_py = Self::to_python(inner);
                    quote! { Vec<#inner_py> }
                } else {
                    quote! { Vec<pyo3::PyObject> }
                }
            }
            TypeCategory::HashMap => {
                if info.inner_types.len() >= 2 {
                    let key_py = Self::to_python(&info.inner_types[0]);
                    let val_py = Self::to_python(&info.inner_types[1]);
                    quote! { std::collections::HashMap<#key_py, #val_py> }
                } else {
                    quote! { std::collections::HashMap<String, pyo3::PyObject> }
                }
            }
            TypeCategory::HashSet => {
                if let Some(inner) = info.inner_types.first() {
                    let inner_py = Self::to_python(inner);
                    quote! { std::collections::HashSet<#inner_py> }
                } else {
                    quote! { std::collections::HashSet<pyo3::PyObject> }
                }
            }
            TypeCategory::Tuple => {
                let inner_types: Vec<_> = info.inner_types.iter().map(Self::to_python).collect();
                quote! { (#(#inner_types),*) }
            }
            TypeCategory::Custom(name) => {
                if !info.category.is_bindable() || !is_valid_rust_ident(name) {
                    quote! { pyo3::PyObject }
                } else {
                    let py_name = format!("Py{}", name);
                    let ident = syn::Ident::new(&py_name, proc_macro2::Span::call_site());
                    quote! { #ident }
                }
            }
        }
    }

    /// Map a type to its Node.js equivalent
    pub fn to_nodejs(info: &TypeInfo) -> TokenStream {
        match &info.category {
            TypeCategory::Primitive(kind) => Self::primitive_to_nodejs(kind),
            TypeCategory::String => quote! { String },
            TypeCategory::Unit => quote! { () },
            TypeCategory::MathHookCore(name) if is_valid_rust_ident(name) => {
                let js_name = format!("Js{}", name);
                let ident = syn::Ident::new(&js_name, proc_macro2::Span::call_site());
                quote! { #ident }
            }
            TypeCategory::MathHookCore(_) => quote! { napi::JsUnknown },
            TypeCategory::Option => {
                if let Some(inner) = info.inner_types.first() {
                    let inner_js = Self::to_nodejs(inner);
                    quote! { Option<#inner_js> }
                } else {
                    quote! { Option<napi::JsUnknown> }
                }
            }
            TypeCategory::Result => {
                if let Some(ok_type) = info.inner_types.first() {
                    let ok_js = Self::to_nodejs(ok_type);
                    quote! { napi::Result<#ok_js> }
                } else {
                    quote! { napi::Result<napi::JsUnknown> }
                }
            }
            TypeCategory::Vec => {
                if let Some(inner) = info.inner_types.first() {
                    let inner_js = Self::to_nodejs(inner);
                    quote! { Vec<#inner_js> }
                } else {
                    quote! { Vec<napi::JsUnknown> }
                }
            }
            TypeCategory::HashMap => {
                if info.inner_types.len() >= 2 {
                    let key_js = Self::to_nodejs(&info.inner_types[0]);
                    let val_js = Self::to_nodejs(&info.inner_types[1]);
                    quote! { std::collections::HashMap<#key_js, #val_js> }
                } else {
                    quote! { std::collections::HashMap<String, napi::JsUnknown> }
                }
            }
            TypeCategory::HashSet => {
                if let Some(inner) = info.inner_types.first() {
                    let inner_js = Self::to_nodejs(inner);
                    quote! { std::collections::HashSet<#inner_js> }
                } else {
                    quote! { std::collections::HashSet<napi::JsUnknown> }
                }
            }
            TypeCategory::Tuple => {
                let inner_types: Vec<_> = info.inner_types.iter().map(Self::to_nodejs).collect();
                quote! { (#(#inner_types),*) }
            }
            TypeCategory::Custom(name) => {
                if !info.category.is_bindable() || !is_valid_rust_ident(name) {
                    quote! { napi::JsUnknown }
                } else {
                    let js_name = format!("Js{}", name);
                    let ident = syn::Ident::new(&js_name, proc_macro2::Span::call_site());
                    quote! { #ident }
                }
            }
        }
    }

    fn primitive_to_python(kind: &PrimitiveKind) -> TokenStream {
        match kind {
            PrimitiveKind::I8 => quote! { i8 },
            PrimitiveKind::I16 => quote! { i16 },
            PrimitiveKind::I32 => quote! { i32 },
            PrimitiveKind::I64 => quote! { i64 },
            PrimitiveKind::U8 => quote! { u8 },
            PrimitiveKind::U16 => quote! { u16 },
            PrimitiveKind::U32 => quote! { u32 },
            PrimitiveKind::U64 => quote! { u64 },
            PrimitiveKind::F32 => quote! { f32 },
            PrimitiveKind::F64 => quote! { f64 },
            PrimitiveKind::Bool => quote! { bool },
            PrimitiveKind::Char => quote! { char },
            PrimitiveKind::Usize => quote! { usize },
        }
    }

    fn primitive_to_nodejs(kind: &PrimitiveKind) -> TokenStream {
        match kind {
            PrimitiveKind::I8 => quote! { i8 },
            PrimitiveKind::I16 => quote! { i16 },
            PrimitiveKind::I32 => quote! { i32 },
            PrimitiveKind::I64 => quote! { i64 },
            PrimitiveKind::U8 => quote! { u8 },
            PrimitiveKind::U16 => quote! { u16 },
            PrimitiveKind::U32 => quote! { u32 },
            PrimitiveKind::U64 => quote! { u64 },
            PrimitiveKind::F32 => quote! { f32 },
            PrimitiveKind::F64 => quote! { f64 },
            PrimitiveKind::Bool => quote! { bool },
            PrimitiveKind::Char => quote! { char },
            PrimitiveKind::Usize => quote! { i64 },
        }
    }
}

/// Name conversion utilities
pub struct NameConverter;

impl NameConverter {
    pub fn to_python_name(rust_name: &str) -> String {
        rust_name.to_string()
    }

    pub fn to_python_class_name(rust_name: &str) -> String {
        rust_name
            .trim_start_matches("Py")
            .trim_start_matches("Js")
            .to_string()
    }

    #[allow(dead_code)]
    pub fn to_nodejs_name(rust_name: &str) -> String {
        Self::snake_to_camel(rust_name)
    }

    pub fn to_javascript_name(rust_name: &str) -> String {
        Self::snake_to_camel(rust_name)
    }

    pub fn to_javascript_class_name(rust_name: &str) -> String {
        rust_name
            .trim_start_matches("Py")
            .trim_start_matches("Js")
            .to_string()
    }

    pub fn variant_to_python_discriminator(variant: &str) -> String {
        variant.to_lowercase()
    }

    pub fn variant_to_javascript_discriminator(variant: &str) -> String {
        variant.to_lowercase()
    }

    fn snake_to_camel(s: &str) -> String {
        let mut result = String::new();
        let mut capitalize_next = false;

        for ch in s.chars() {
            if ch == '_' {
                capitalize_next = true;
            } else if capitalize_next {
                result.push(ch.to_ascii_uppercase());
                capitalize_next = false;
            } else {
                result.push(ch);
            }
        }

        result
    }
}

pub fn is_valid_rust_ident(name: &str) -> bool {
    !name.is_empty()
        && name
            .chars()
            .next()
            .is_some_and(|c| c.is_alphabetic() || c == '_')
        && name.chars().all(|c| c.is_alphanumeric() || c == '_')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_bindable() {
        assert!(TypeCategory::Custom("Expression".to_string()).is_bindable());
        assert!(!TypeCategory::Custom("dyn SimplificationStrategy".to_string()).is_bindable());
        assert!(!TypeCategory::Custom("[Expression; 9]".to_string()).is_bindable());
        assert!(!TypeCategory::Custom("impl Trait".to_string()).is_bindable());
        assert!(!TypeCategory::Custom("Fn() -> i32".to_string()).is_bindable());
        assert!(!TypeCategory::Custom("Box<dyn Trait>".to_string()).is_bindable());
    }

    #[test]
    fn test_name_conversion() {
        assert_eq!(NameConverter::to_python_name("get_value"), "get_value");
        assert_eq!(
            NameConverter::to_python_class_name("Expression"),
            "Expression"
        );
        assert_eq!(
            NameConverter::to_python_class_name("PyExpression"),
            "Expression"
        );
        assert_eq!(NameConverter::to_nodejs_name("get_value"), "getValue");
    }
}
