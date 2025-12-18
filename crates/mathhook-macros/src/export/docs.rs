//! Documentation transformation for Python and Node.js bindings
//!
//! Converts Rust doc comments to Python docstrings and JSDoc format.

use crate::export::stubs::StubGenerator as TypeStubGenerator;
use crate::export::types::TypeInfo;
use syn::{FnArg, ItemEnum, ItemFn, ItemStruct, Pat, ReturnType};

pub struct DocTransformer;

impl DocTransformer {
    pub fn extract_doc_comments(attrs: &[syn::Attribute]) -> Vec<String> {
        attrs
            .iter()
            .filter_map(|attr| {
                if attr.path().is_ident("doc") {
                    if let syn::Meta::NameValue(meta) = &attr.meta {
                        if let syn::Expr::Lit(expr_lit) = &meta.value {
                            if let syn::Lit::Str(lit_str) = &expr_lit.lit {
                                return Some(lit_str.value());
                            }
                        }
                    }
                }
                None
            })
            .collect()
    }

    pub fn to_python_docstring(docs: &[String]) -> String {
        if docs.is_empty() {
            return String::new();
        }

        let mut result = String::new();
        for doc in docs {
            let trimmed = doc.trim();
            if !trimmed.is_empty() {
                result.push_str(trimmed);
                result.push('\n');
            }
        }

        result.trim().to_string()
    }

    pub fn to_jsdoc(docs: &[String]) -> String {
        if docs.is_empty() {
            return String::new();
        }

        let mut result = String::from("/**\n");
        for doc in docs {
            let trimmed = doc.trim();
            if !trimmed.is_empty() {
                result.push_str(" * ");
                result.push_str(trimmed);
                result.push('\n');
            }
        }
        result.push_str(" */");

        result
    }

    #[allow(dead_code)]
    pub fn to_markdown(docs: &[String]) -> String {
        docs.iter()
            .map(|line| line.trim())
            .filter(|line| !line.is_empty())
            .collect::<Vec<_>>()
            .join("\n")
    }

    #[allow(dead_code)]
    pub fn to_pyi_stub(docs: &[String], params: &[(String, String)], return_type: &str) -> String {
        let mut result = String::new();

        if !docs.is_empty() {
            result.push_str("    \"\"\"");
            result.push_str(docs.first().unwrap_or(&String::new()).trim());
            result.push_str("\n\n");

            if !params.is_empty() {
                result.push_str("    Args:\n");
                for (name, ty) in params {
                    result.push_str(&format!("        {}: {}\n", name, ty));
                }
            }

            if !return_type.is_empty() && return_type != "None" {
                result.push_str(&format!("\n    Returns:\n        {}\n", return_type));
            }

            result.push_str("    \"\"\"\n");
        }

        result
    }

    #[allow(dead_code)]
    pub fn to_dts_comment(
        docs: &[String],
        params: &[(String, String)],
        return_type: &str,
    ) -> String {
        let mut result = String::from("/**\n");

        for line in docs.iter().take(1) {
            result.push_str(&format!(" * {}\n", line.trim()));
        }

        if !params.is_empty() {
            result.push_str(" *\n");
            for (name, ty) in params {
                result.push_str(&format!(" * @param {} - {}\n", name, ty));
            }
        }

        if !return_type.is_empty() && return_type != "void" {
            result.push_str(&format!(" * @returns {}\n", return_type));
        }

        result.push_str(" */\n");
        result
    }
}

#[allow(dead_code)]
pub struct StubGenerator;

#[allow(dead_code)]
impl StubGenerator {
    pub fn generate_pyi_function_stub(func: &ItemFn) -> String {
        let func_name = &func.sig.ident;
        let docs = DocTransformer::extract_doc_comments(&func.attrs);

        let params: Vec<(String, String)> = func
            .sig
            .inputs
            .iter()
            .filter_map(|arg| {
                if let FnArg::Typed(pat_type) = arg {
                    if let Pat::Ident(pat_ident) = pat_type.pat.as_ref() {
                        let name = pat_ident.ident.to_string();
                        let ty = TypeInfo::from_type(&pat_type.ty);
                        let type_hint = TypeStubGenerator::to_python_type_hint(&ty);
                        return Some((name, type_hint));
                    }
                }
                None
            })
            .collect();

        let return_type = match &func.sig.output {
            ReturnType::Default => "None".to_string(),
            ReturnType::Type(_, ty) => {
                let type_info = TypeInfo::from_type(ty);
                TypeStubGenerator::to_python_type_hint(&type_info)
            }
        };

        let param_list = if params.is_empty() {
            String::new()
        } else {
            params
                .iter()
                .map(|(name, ty)| format!("{}: {}", name, ty))
                .collect::<Vec<_>>()
                .join(", ")
        };

        let mut stub = format!("def {}({}) -> {}:\n", func_name, param_list, return_type);
        stub.push_str(&DocTransformer::to_pyi_stub(&docs, &params, &return_type));
        stub.push_str("    ...\n\n");

        stub
    }

    pub fn generate_dts_function_stub(func: &ItemFn, js_name: &str) -> String {
        let docs = DocTransformer::extract_doc_comments(&func.attrs);

        let params: Vec<(String, String)> = func
            .sig
            .inputs
            .iter()
            .filter_map(|arg| {
                if let FnArg::Typed(pat_type) = arg {
                    if let Pat::Ident(pat_ident) = pat_type.pat.as_ref() {
                        let name = pat_ident.ident.to_string();
                        let ty = TypeInfo::from_type(&pat_type.ty);
                        let ts_type = TypeStubGenerator::to_typescript_type(&ty);
                        return Some((name, ts_type));
                    }
                }
                None
            })
            .collect();

        let return_type = match &func.sig.output {
            ReturnType::Default => "void".to_string(),
            ReturnType::Type(_, ty) => {
                let type_info = TypeInfo::from_type(ty);
                TypeStubGenerator::to_typescript_type(&type_info)
            }
        };

        let param_list = if params.is_empty() {
            String::new()
        } else {
            params
                .iter()
                .map(|(name, ty)| format!("{}: {}", name, ty))
                .collect::<Vec<_>>()
                .join(", ")
        };

        let mut stub = DocTransformer::to_dts_comment(&docs, &params, &return_type);
        stub.push_str(&format!(
            "export function {}({}): {};\n\n",
            js_name, param_list, return_type
        ));

        stub
    }

    pub fn generate_pyi_struct_stub(struct_def: &ItemStruct, class_name: &str) -> String {
        let docs = DocTransformer::extract_doc_comments(&struct_def.attrs);
        let mut stub = format!("class {}:\n", class_name);

        if !docs.is_empty() {
            stub.push_str(&format!("    \"\"\"{}\"\"\"\n\n", docs.join(" ")));
        }

        for field in &struct_def.fields {
            if let Some(field_name) = &field.ident {
                if matches!(field.vis, syn::Visibility::Public(_)) {
                    let ty = TypeInfo::from_type(&field.ty);
                    let type_hint = TypeStubGenerator::to_python_type_hint(&ty);
                    stub.push_str(&format!("    {}: {}\n", field_name, type_hint));
                }
            }
        }

        stub.push_str("\n    def __init__(self, ...) -> None: ...\n\n");

        stub
    }

    pub fn generate_dts_struct_stub(struct_def: &ItemStruct, class_name: &str) -> String {
        let docs = DocTransformer::extract_doc_comments(&struct_def.attrs);
        let mut stub = String::new();

        if !docs.is_empty() {
            stub.push_str(&format!("/**\n * {}\n */\n", docs.join(" ")));
        }

        stub.push_str(&format!("export class {} {{\n", class_name));

        for field in &struct_def.fields {
            if let Some(field_name) = &field.ident {
                if matches!(field.vis, syn::Visibility::Public(_)) {
                    let ty = TypeInfo::from_type(&field.ty);
                    let ts_type = TypeStubGenerator::to_typescript_type(&ty);
                    stub.push_str(&format!("  {}: {};\n", field_name, ts_type));
                }
            }
        }

        stub.push_str("\n  constructor(...args: any[]);\n");
        stub.push_str("}\n\n");

        stub
    }

    pub fn generate_pyi_enum_stub(enum_def: &ItemEnum, enum_name: &str) -> String {
        let docs = DocTransformer::extract_doc_comments(&enum_def.attrs);
        let mut stub = format!("class {}:\n", enum_name);

        if !docs.is_empty() {
            stub.push_str(&format!("    \"\"\"{}\"\"\"\n\n", docs.join(" ")));
        }

        for variant in &enum_def.variants {
            let variant_name = &variant.ident;
            stub.push_str(&format!("    {}: str\n", variant_name));
        }

        stub.push('\n');
        stub
    }

    pub fn generate_dts_enum_stub(enum_def: &ItemEnum, enum_name: &str) -> String {
        let docs = DocTransformer::extract_doc_comments(&enum_def.attrs);
        let mut stub = String::new();

        if !docs.is_empty() {
            stub.push_str(&format!("/**\n * {}\n */\n", docs.join(" ")));
        }

        let variants: Vec<String> = enum_def
            .variants
            .iter()
            .map(|v| format!("\"{}\"", v.ident))
            .collect();

        stub.push_str(&format!(
            "export type {} = {};\n\n",
            enum_name,
            variants.join(" | ")
        ));

        stub
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_python_docstring() {
        let docs = vec!["This is a test function".to_string()];
        let docstring = DocTransformer::to_python_docstring(&docs);
        assert_eq!(docstring, "This is a test function");
    }

    #[test]
    fn test_jsdoc() {
        let docs = vec!["This is a test function".to_string()];
        let jsdoc = DocTransformer::to_jsdoc(&docs);
        assert!(jsdoc.contains("This is a test function"));
        assert!(jsdoc.starts_with("/**"));
        assert!(jsdoc.ends_with(" */"));
    }

    #[test]
    fn test_empty_docs() {
        let docs: Vec<String> = vec![];
        assert_eq!(DocTransformer::to_python_docstring(&docs), "");
        assert_eq!(DocTransformer::to_jsdoc(&docs), "");
    }

    #[test]
    fn test_pyi_stub_generation() {
        let docs = vec!["Test function".to_string()];
        let params = vec![("x".to_string(), "int".to_string())];
        let stub = DocTransformer::to_pyi_stub(&docs, &params, "str");

        assert!(stub.contains("Test function"));
        assert!(stub.contains("Args:"));
        assert!(stub.contains("x: int"));
        assert!(stub.contains("Returns:"));
        assert!(stub.contains("str"));
    }

    #[test]
    fn test_dts_comment_generation() {
        let docs = vec!["Test function".to_string()];
        let params = vec![("x".to_string(), "number".to_string())];
        let comment = DocTransformer::to_dts_comment(&docs, &params, "string");

        assert!(comment.contains("Test function"));
        assert!(comment.contains("@param x - number"));
        assert!(comment.contains("@returns string"));
    }
}
