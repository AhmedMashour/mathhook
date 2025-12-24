//! Type introspection and mapping for binding generation
//!
//! Provides utilities to analyze Rust types and map them to
//! Python (PyO3) and Node.js (NAPI-RS) equivalents.

use convert_case::{Case, Casing};
use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::{GenericArgument, Ident, PathArguments, Type, TypePath};

const MATHHOOK_CORE_TYPES: &[&str] = &[
    "Expression",
    "Symbol",
    "Matrix",
    "Number",
    "Polynomial",
    "SolverResult",
    "MathError",
    "ParseError",
];

fn is_mathhook_core_type(name: &str) -> bool {
    MATHHOOK_CORE_TYPES.contains(&name)
}

fn is_valid_rust_ident(s: &str) -> bool {
    !s.is_empty()
        && !s.contains(' ')
        && !s.contains('[')
        && !s.contains(']')
        && !s.contains('<')
        && !s.contains('>')
        && !s.contains('(')
        && !s.contains(')')
        && !s.contains('&')
        && !s.contains('*')
        && !s.contains(';')
        && s.chars()
            .next()
            .map(|c| c.is_alphabetic() || c == '_')
            .unwrap_or(false)
}

/// Represents analyzed type information
#[derive(Debug, Clone)]
pub struct TypeInfo {
    /// The original Rust type
    #[allow(dead_code)]
    pub rust_type: Type,
    /// Category of the type
    pub category: TypeCategory,
    /// Inner types (for generics like Vec<T>, Option<T>)
    pub inner_types: Vec<TypeInfo>,
    /// Whether the type is a reference
    pub is_ref: bool,
    /// Whether the type is mutable
    pub is_mut: bool,
}

/// Categories of types for different handling strategies
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TypeCategory {
    /// Primitive types: i32, i64, f64, bool, etc.
    Primitive(PrimitiveKind),
    /// String types: String, &str
    String,
    /// MathHook core types: Expression, Symbol, Matrix
    MathHookCore(String),
    /// Optional wrapper: Option<T>
    Option,
    /// Result wrapper: Result<T, E>
    Result,
    /// Vector/List: Vec<T>
    Vec,
    /// HashMap/Dict: HashMap<K, V>
    HashMap,
    /// HashSet: HashSet<T>
    HashSet,
    /// Tuple types: (A, B, C)
    Tuple,
    /// Unit type: ()
    Unit,
    /// Unknown/custom type
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
}

impl TypeInfo {
    /// Analyze a type and extract information
    pub fn from_type(ty: &Type) -> Self {
        match ty {
            Type::Path(type_path) => Self::from_type_path(type_path),
            Type::Reference(type_ref) => {
                let mut info = Self::from_type(&type_ref.elem);
                info.is_ref = true;
                info.is_mut = type_ref.mutability.is_some();
                info
            }
            Type::Tuple(tuple) => {
                if tuple.elems.is_empty() {
                    Self {
                        rust_type: ty.clone(),
                        category: TypeCategory::Unit,
                        inner_types: vec![],
                        is_ref: false,
                        is_mut: false,
                    }
                } else {
                    let inner_types: Vec<_> = tuple.elems.iter().map(Self::from_type).collect();
                    Self {
                        rust_type: ty.clone(),
                        category: TypeCategory::Tuple,
                        inner_types,
                        is_ref: false,
                        is_mut: false,
                    }
                }
            }
            Type::Slice(slice) => {
                let inner = Self::from_type(&slice.elem);
                Self {
                    rust_type: ty.clone(),
                    category: TypeCategory::Vec,
                    inner_types: vec![inner],
                    is_ref: true,
                    is_mut: false,
                }
            }
            _ => Self {
                rust_type: ty.clone(),
                category: TypeCategory::Custom(ty.to_token_stream().to_string()),
                inner_types: vec![],
                is_ref: false,
                is_mut: false,
            },
        }
    }

    fn from_type_path(type_path: &TypePath) -> Self {
        let path = &type_path.path;
        let segment = path.segments.last().unwrap();
        let name = segment.ident.to_string();

        let (category, inner_types) = Self::from_path_segment(&name, &segment.arguments);

        Self {
            rust_type: Type::Path(type_path.clone()),
            category,
            inner_types,
            is_ref: false,
            is_mut: false,
        }
    }

    fn from_path_segment(name: &str, args: &PathArguments) -> (TypeCategory, Vec<TypeInfo>) {
        match name {
            "i8" => (TypeCategory::Primitive(PrimitiveKind::I8), vec![]),
            "i16" => (TypeCategory::Primitive(PrimitiveKind::I16), vec![]),
            "i32" => (TypeCategory::Primitive(PrimitiveKind::I32), vec![]),
            "i64" => (TypeCategory::Primitive(PrimitiveKind::I64), vec![]),
            "u8" => (TypeCategory::Primitive(PrimitiveKind::U8), vec![]),
            "u16" => (TypeCategory::Primitive(PrimitiveKind::U16), vec![]),
            "u32" => (TypeCategory::Primitive(PrimitiveKind::U32), vec![]),
            "u64" => (TypeCategory::Primitive(PrimitiveKind::U64), vec![]),
            "usize" => (TypeCategory::Primitive(PrimitiveKind::U64), vec![]),
            "isize" => (TypeCategory::Primitive(PrimitiveKind::I64), vec![]),
            "f32" => (TypeCategory::Primitive(PrimitiveKind::F32), vec![]),
            "f64" => (TypeCategory::Primitive(PrimitiveKind::F64), vec![]),
            "bool" => (TypeCategory::Primitive(PrimitiveKind::Bool), vec![]),
            "char" => (TypeCategory::Primitive(PrimitiveKind::Char), vec![]),

            "String" | "str" => (TypeCategory::String, vec![]),

            "Option" => {
                let inner = Self::extract_generic_args(args);
                (TypeCategory::Option, inner)
            }
            "Result" => {
                let inner = Self::extract_generic_args(args);
                (TypeCategory::Result, inner)
            }
            "Vec" => {
                let inner = Self::extract_generic_args(args);
                (TypeCategory::Vec, inner)
            }
            "HashMap" | "BTreeMap" => {
                let inner = Self::extract_generic_args(args);
                (TypeCategory::HashMap, inner)
            }
            "HashSet" | "BTreeSet" => {
                let inner = Self::extract_generic_args(args);
                (TypeCategory::HashSet, inner)
            }

            "Box" | "Arc" | "Rc" => {
                let inner = Self::extract_generic_args(args);
                if let Some(first) = inner.first() {
                    return (first.category.clone(), first.inner_types.clone());
                }
                (TypeCategory::Custom(name.to_string()), inner)
            }

            "Cow" => {
                let inner = Self::extract_generic_args(args);
                if let Some(first) = inner.first() {
                    if matches!(first.category, TypeCategory::String) {
                        return (TypeCategory::String, vec![]);
                    }
                }
                (TypeCategory::String, vec![])
            }

            name if is_mathhook_core_type(name) => {
                (TypeCategory::MathHookCore(name.to_string()), vec![])
            }

            other => (TypeCategory::Custom(other.to_string()), vec![]),
        }
    }

    fn extract_generic_args(args: &PathArguments) -> Vec<TypeInfo> {
        match args {
            PathArguments::AngleBracketed(ab) => ab
                .args
                .iter()
                .filter_map(|arg| {
                    if let GenericArgument::Type(ty) = arg {
                        Some(Self::from_type(ty))
                    } else {
                        None
                    }
                })
                .collect(),
            _ => vec![],
        }
    }

    /// Check if this type needs conversion at FFI boundary
    pub fn needs_conversion(&self) -> bool {
        !matches!(
            self.category,
            TypeCategory::Primitive(_) | TypeCategory::Unit
        )
    }

    /// Check if this type is a MathHook core type
    pub fn is_mathhook_type(&self) -> bool {
        matches!(self.category, TypeCategory::MathHookCore(_))
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
        }
    }
}

/// Utility functions for name conversion
pub struct NameConverter;

impl NameConverter {
    /// Convert Rust snake_case to Python snake_case (usually no-op)
    pub fn to_python_name(rust_name: &str) -> String {
        rust_name.to_case(Case::Snake)
    }

    /// Convert Rust snake_case to JavaScript camelCase
    pub fn to_javascript_name(rust_name: &str) -> String {
        rust_name.to_case(Case::Camel)
    }

    /// Convert Rust PascalCase to Python wrapper name (Py prefix)
    pub fn to_python_class_name(rust_name: &str) -> String {
        format!("Py{}", rust_name.to_case(Case::Pascal))
    }

    /// Convert Rust PascalCase to JavaScript wrapper name (Js prefix)
    pub fn to_javascript_class_name(rust_name: &str) -> String {
        format!("Js{}", rust_name.to_case(Case::Pascal))
    }

    /// Convert enum variant to Python discriminator value
    pub fn variant_to_python_discriminator(variant: &str) -> String {
        variant.to_case(Case::Snake)
    }

    /// Convert enum variant to JavaScript discriminator value
    pub fn variant_to_javascript_discriminator(variant: &str) -> String {
        variant.to_case(Case::Camel)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name_conversion() {
        assert_eq!(NameConverter::to_python_name("get_value"), "get_value");
        assert_eq!(NameConverter::to_javascript_name("get_value"), "getValue");
        assert_eq!(
            NameConverter::to_python_class_name("Expression"),
            "PyExpression"
        );
        assert_eq!(
            NameConverter::to_javascript_class_name("Expression"),
            "JsExpression"
        );
    }

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
    fn test_is_valid_rust_ident() {
        assert!(is_valid_rust_ident("Expression"));
        assert!(is_valid_rust_ident("_private"));
        assert!(!is_valid_rust_ident("dyn SimplificationStrategy"));
        assert!(!is_valid_rust_ident("[Expression; 9]"));
        assert!(!is_valid_rust_ident("Py[Expression; 9]"));
        assert!(!is_valid_rust_ident("impl Trait"));
        assert!(!is_valid_rust_ident("Box<dyn Trait>"));
        assert!(!is_valid_rust_ident("&str"));
        assert!(!is_valid_rust_ident("*const u8"));
    }
}
