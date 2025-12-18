//! Type stub and documentation generation utilities
//!
//! Provides methods to generate Python type hints and TypeScript type definitions
//! for documentation and IDE support.

use super::types::{PrimitiveKind, TypeCategory, TypeInfo};

pub struct StubGenerator;

impl StubGenerator {
    /// Map a type to Python type hint string (PEP 585 style for Python 3.9+)
    pub fn to_python_type_hint(info: &TypeInfo) -> String {
        match &info.category {
            TypeCategory::Primitive(kind) => match kind {
                PrimitiveKind::Bool => "bool".into(),
                PrimitiveKind::Char => "str".into(),
                PrimitiveKind::F32 | PrimitiveKind::F64 => "float".into(),
                _ => "int".into(),
            },
            TypeCategory::String => "str".into(),
            TypeCategory::Unit => "None".into(),
            TypeCategory::MathHookCore(name) => format!("Py{}", name),
            TypeCategory::Option => {
                if let Some(inner) = info.inner_types.first() {
                    format!("{} | None", Self::to_python_type_hint(inner))
                } else {
                    "Any | None".into()
                }
            }
            TypeCategory::Result => {
                if let Some(ok) = info.inner_types.first() {
                    Self::to_python_type_hint(ok)
                } else {
                    "Any".into()
                }
            }
            TypeCategory::Vec => {
                if let Some(inner) = info.inner_types.first() {
                    format!("list[{}]", Self::to_python_type_hint(inner))
                } else {
                    "list[Any]".into()
                }
            }
            TypeCategory::HashMap => {
                if info.inner_types.len() >= 2 {
                    format!(
                        "dict[{}, {}]",
                        Self::to_python_type_hint(&info.inner_types[0]),
                        Self::to_python_type_hint(&info.inner_types[1])
                    )
                } else {
                    "dict[str, Any]".into()
                }
            }
            TypeCategory::HashSet => {
                if let Some(inner) = info.inner_types.first() {
                    format!("set[{}]", Self::to_python_type_hint(inner))
                } else {
                    "set[Any]".into()
                }
            }
            TypeCategory::Tuple => {
                let inner: Vec<_> = info
                    .inner_types
                    .iter()
                    .map(Self::to_python_type_hint)
                    .collect();
                format!("tuple[{}]", inner.join(", "))
            }
            TypeCategory::Custom(name) => format!("Py{}", name),
        }
    }

    /// Map a type to TypeScript type string
    pub fn to_typescript_type(info: &TypeInfo) -> String {
        match &info.category {
            TypeCategory::Primitive(kind) => match kind {
                PrimitiveKind::Bool => "boolean".into(),
                PrimitiveKind::Char => "string".into(),
                _ => "number".into(),
            },
            TypeCategory::String => "string".into(),
            TypeCategory::Unit => "void".into(),
            TypeCategory::MathHookCore(name) => format!("Js{}", name),
            TypeCategory::Option => {
                if let Some(inner) = info.inner_types.first() {
                    format!("{} | null", Self::to_typescript_type(inner))
                } else {
                    "unknown | null".into()
                }
            }
            TypeCategory::Result => {
                if let Some(ok) = info.inner_types.first() {
                    Self::to_typescript_type(ok)
                } else {
                    "unknown".into()
                }
            }
            TypeCategory::Vec => {
                if let Some(inner) = info.inner_types.first() {
                    format!("{}[]", Self::to_typescript_type(inner))
                } else {
                    "unknown[]".into()
                }
            }
            TypeCategory::HashMap => {
                if info.inner_types.len() >= 2 {
                    format!(
                        "Record<{}, {}>",
                        Self::to_typescript_type(&info.inner_types[0]),
                        Self::to_typescript_type(&info.inner_types[1])
                    )
                } else {
                    "Record<string, unknown>".into()
                }
            }
            TypeCategory::HashSet => {
                if let Some(inner) = info.inner_types.first() {
                    format!("Set<{}>", Self::to_typescript_type(inner))
                } else {
                    "Set<unknown>".into()
                }
            }
            TypeCategory::Tuple => {
                let inner: Vec<_> = info
                    .inner_types
                    .iter()
                    .map(Self::to_typescript_type)
                    .collect();
                format!("[{}]", inner.join(", "))
            }
            TypeCategory::Custom(name) => format!("Js{}", name),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use syn::parse_quote;

    #[test]
    fn test_python_type_hints() {
        let info = TypeInfo::from_type(&parse_quote!(i32));
        assert_eq!(StubGenerator::to_python_type_hint(&info), "int");

        let info = TypeInfo::from_type(&parse_quote!(String));
        assert_eq!(StubGenerator::to_python_type_hint(&info), "str");

        let info = TypeInfo::from_type(&parse_quote!(Vec<i32>));
        assert_eq!(StubGenerator::to_python_type_hint(&info), "list[int]");
    }

    #[test]
    fn test_typescript_types() {
        let info = TypeInfo::from_type(&parse_quote!(i32));
        assert_eq!(StubGenerator::to_typescript_type(&info), "number");

        let info = TypeInfo::from_type(&parse_quote!(String));
        assert_eq!(StubGenerator::to_typescript_type(&info), "string");

        let info = TypeInfo::from_type(&parse_quote!(Vec<i32>));
        assert_eq!(StubGenerator::to_typescript_type(&info), "number[]");
    }
}
