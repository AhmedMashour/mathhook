use crate::doc_transformer::to_napi_doc;
use crate::scanner::{clean_doc_for_binding, ImplInfo, TypeInfo};
use crate::trait_analyzer::{SupportedTrait, TraitAnalysis};
use crate::types::{AnalyzedMethod, MappedType};
use std::collections::{HashMap, HashSet};

const SKIPPED_FUNCTIONS: &[&str] = &[
    "get_universal_registry",
    "get_simplification_registry",
    "get_global_background_compute",
    "get_performance_optimizer",
    "get_cached_expression",
    "get_cache_stats",
];

fn to_js_class_name(rust_name: &str) -> String {
    rust_name
        .strip_prefix("Js")
        .unwrap_or(rust_name)
        .to_string()
}

fn to_camel_case(snake: &str) -> String {
    let mut result = String::new();
    let mut capitalize_next = false;

    for c in snake.chars() {
        if c == '_' {
            capitalize_next = true;
        } else if capitalize_next {
            result.push(c.to_ascii_uppercase());
            capitalize_next = false;
        } else {
            result.push(c);
        }
    }

    result
}

pub struct NodeEmitter {
    simple_enum_types: HashSet<String>,
}

impl Default for NodeEmitter {
    fn default() -> Self {
        Self::new()
    }
}

fn build_import_path(type_info: &TypeInfo) -> String {
    if type_info.module_path.is_empty() {
        format!("mathhook_core::{}", type_info.name)
    } else {
        format!(
            "mathhook_core::{}::{}",
            type_info.module_path, type_info.name
        )
    }
}

fn is_unsupported_for_node(mapped_type: &MappedType, is_input: bool) -> bool {
    match mapped_type {
        MappedType::Unsupported { .. } => true,
        MappedType::Direct { node_type, .. } => node_type == "JsObject",
        MappedType::Reference { inner_type, .. } => is_unsupported_for_node(inner_type, is_input),
        MappedType::Option { inner_type } => is_unsupported_for_node(inner_type, is_input),
        MappedType::Collected { item_type } => is_unsupported_for_node(item_type, is_input),
        MappedType::Result { ok_type, .. } => is_unsupported_for_node(ok_type, false),
        MappedType::HashMap {
            key_type,
            value_type,
        } => {
            is_unsupported_for_node(key_type, is_input)
                || is_unsupported_for_node(value_type, is_input)
        }
        MappedType::Tuple { elements } => {
            if is_input {
                true
            } else {
                elements.iter().any(|e| is_unsupported_for_node(e, false))
            }
        }
        MappedType::Callback { .. } => true,
        MappedType::Union { .. } => true,
    }
}

fn method_has_unsupported_types(method: &AnalyzedMethod) -> bool {
    if is_unsupported_for_node(&method.output, false) {
        return true;
    }
    method
        .inputs
        .iter()
        .any(|(_, mapped_type)| is_unsupported_for_node(mapped_type, true))
}

fn is_consuming_method_name(method_name: &str) -> bool {
    method_name.starts_with("with_")
        || method_name.starts_with("into_")
        || matches!(
            method_name,
            "optimize"
                | "take"
                | "consume"
                | "tolerance"
                | "max_iterations"
                | "simplify"
                | "educational"
        )
}

fn method_is_consuming(method: &AnalyzedMethod) -> bool {
    let has_self = method
        .inputs
        .iter()
        .any(|(name, _)| name == "self" || name == "self_mut");
    if !has_self {
        return false;
    }
    let method_name = method.original_name.as_ref().unwrap_or(&method.name);
    is_consuming_method_name(method_name)
}

fn is_skipped_function(func_name: &str) -> bool {
    SKIPPED_FUNCTIONS.contains(&func_name)
}

fn escape_doc_string(s: &str) -> String {
    s.replace('\\', "\\\\").replace('"', "\\\"")
}

impl NodeEmitter {
    pub fn new() -> Self {
        Self {
            simple_enum_types: HashSet::new(),
        }
    }

    pub fn with_simple_enum_types(types: impl IntoIterator<Item = String>) -> Self {
        Self {
            simple_enum_types: types.into_iter().collect(),
        }
    }

    pub fn add_simple_enum_type(&mut self, type_name: String) {
        self.simple_enum_types.insert(type_name);
    }

    fn is_simple_enum_type(&self, type_name: &str) -> bool {
        self.simple_enum_types.contains(type_name)
    }

    fn should_skip_type(&self, type_info: &TypeInfo) -> bool {
        !type_info.is_public || type_info.is_cfg_gated || type_info.skip_binding
    }

    fn is_owned_parameter(mapped_type: &MappedType) -> bool {
        !matches!(mapped_type, MappedType::Reference { .. })
    }

    fn emit_type_doc_with_variants(type_info: &TypeInfo, indent: usize) -> String {
        let indent_str = " ".repeat(indent);
        let mut doc_lines: Vec<String> = Vec::new();

        if let Some(ref doc) = type_info.doc_comment {
            let cleaned = clean_doc_for_binding(doc);
            for line in cleaned.lines() {
                doc_lines.push(line.trim().to_string());
            }
        }

        if type_info.is_simple_enum() && !type_info.enum_variants.is_empty() {
            let has_any_variant_doc = type_info
                .enum_variants
                .iter()
                .any(|v| v.doc_comment.is_some());
            if has_any_variant_doc || !doc_lines.is_empty() {
                if !doc_lines.is_empty() {
                    doc_lines.push(String::new());
                }
                doc_lines.push("# Values".to_string());
                doc_lines.push(String::new());
                for variant in &type_info.enum_variants {
                    if let Some(ref doc) = variant.doc_comment {
                        let cleaned = clean_doc_for_binding(doc);
                        let first_line = cleaned.lines().next().unwrap_or("").trim();
                        doc_lines.push(format!("* `{}` - {}", variant.name, first_line));
                    } else {
                        doc_lines.push(format!("* `{}`", variant.name));
                    }
                }
            }
        }

        if doc_lines.is_empty() {
            return String::new();
        }

        doc_lines
            .iter()
            .map(|line| format!("{}#[doc = \"{}\"]", indent_str, escape_doc_string(line)))
            .collect::<Vec<_>>()
            .join("\n")
    }

    fn emit_type_file_impl(
        &self,
        type_info: &TypeInfo,
        methods: &[AnalyzedMethod],
        trait_analysis: &TraitAnalysis,
        trait_path_map: &HashMap<String, String>,
    ) -> String {
        let core_type = &type_info.name;

        if self.should_skip_type(type_info) {
            return format!(
                "// AUTO-GENERATED by binding-codegen - DO NOT EDIT\n\n// Skipped: {} (not public or cfg-gated)\n",
                core_type
            );
        }

        let mut output = String::new();
        let struct_name = format!("Js{}", core_type);
        let type_has_clone = type_info.has_clone();

        output.push_str("// AUTO-GENERATED by binding-codegen - DO NOT EDIT\n\n");
        output.push_str("#[allow(unused_imports)]\n");
        output.push_str("use super::*;\n");
        output.push_str(&format!("use {};\n", build_import_path(type_info)));
        output.push_str("use napi::bindgen_prelude::*;\n");
        output.push_str("use napi_derive::napi;\n");

        let unique_imports = trait_analysis.unique_trait_imports(trait_path_map);
        for trait_path in &unique_imports {
            output.push_str(&format!("use {};\n", trait_path));
        }

        output.push('\n');

        let type_doc = Self::emit_type_doc_with_variants(type_info, 0);
        if !type_doc.is_empty() {
            output.push_str(&type_doc);
            output.push('\n');
        }

        let js_class_name = to_js_class_name(&struct_name);
        output.push_str(&format!("#[napi(js_name = \"{}\")]\n", js_class_name));
        if type_has_clone {
            output.push_str("#[derive(Clone)]\n");
        }
        output.push_str(&format!("pub struct {} {{\n", struct_name));
        output.push_str(&format!("    pub(crate) inner: {},\n", core_type));
        output.push_str("}\n\n");

        output.push_str("#[napi]\n");
        output.push_str(&format!("impl {} {{\n", struct_name));

        let has_constructor = methods.iter().any(|m| m.name == "new");
        let has_default_trait = trait_analysis
            .implemented_traits
            .contains(&SupportedTrait::Default);
        let has_default = has_default_trait && !has_constructor;

        if has_default {
            output.push_str("    #[napi(constructor)]\n");
            output.push_str("    pub fn new() -> Self {\n");
            output.push_str(&format!(
                "        Self {{ inner: {}::default() }}\n",
                core_type
            ));
            output.push_str("    }\n\n");
        }

        for method in methods {
            if !method.is_supported || method_has_unsupported_types(method) {
                continue;
            }
            if method_is_consuming(method) && !type_has_clone {
                continue;
            }
            output.push_str(&self.emit_method(method, &struct_name, core_type, type_has_clone));
        }

        let has_partial_eq = trait_analysis
            .implemented_traits
            .contains(&SupportedTrait::PartialEq);
        let has_eq_method = methods.iter().any(|m| m.name == "eq" || m.name == "equals");
        if has_partial_eq && !has_eq_method {
            output.push_str("    #[napi]\n");
            output.push_str(&format!(
                "    pub fn equals(&self, other: &{}) -> bool {{\n",
                struct_name
            ));
            output.push_str("        self.inner == other.inner\n");
            output.push_str("    }\n\n");
        }

        let has_clone_method = methods.iter().any(|m| m.name == "clone");
        if type_has_clone && !has_clone_method {
            output.push_str("    #[napi(js_name = \"cloneValue\")]\n");
            output.push_str(&format!(
                "    pub fn clone_value(&self) -> {} {{\n",
                struct_name
            ));
            output.push_str(&format!(
                "        {} {{ inner: self.inner.clone() }}\n",
                struct_name
            ));
            output.push_str("    }\n\n");
        }

        output.push_str("}\n\n");

        output.push_str(&format!(
            "impl From<{}> for {} {{\n",
            core_type, struct_name
        ));
        output.push_str(&format!("    fn from(core: {}) -> Self {{\n", core_type));
        output.push_str("        Self { inner: core }\n");
        output.push_str("    }\n");
        output.push_str("}\n");

        if type_info.is_simple_enum() {
            output.push('\n');
            output.push_str(&format!(
                "impl From<{}> for {} {{\n",
                struct_name, core_type
            ));
            output.push_str(&format!(
                "    fn from(wrapper: {}) -> Self {{\n",
                struct_name
            ));
            output.push_str("        wrapper.inner\n");
            output.push_str("    }\n");
            output.push_str("}\n");
        }

        output
    }

    fn emit_method(
        &self,
        method: &AnalyzedMethod,
        struct_name: &str,
        core_type: &str,
        type_has_clone: bool,
    ) -> String {
        let mut output = String::new();

        if method.skip_binding {
            return format!(
                "    // Skipped: {} - marked with @no-binding\n",
                method.name
            );
        }

        let has_self = method.inputs.iter().any(|(name, _)| name == "self");
        let has_mut_self = method.inputs.iter().any(|(name, _)| name == "self_mut");
        let is_constructor = method.name == "new";

        if let Some(doc) = &method.doc_comment {
            let doc_attr = to_napi_doc(doc, 4);
            if !doc_attr.is_empty() {
                output.push_str(&doc_attr);
                output.push('\n');
            }
        }

        let js_method_name = to_camel_case(&method.name);
        if is_constructor {
            output.push_str("    #[napi(constructor)]\n");
        } else if js_method_name != method.name {
            output.push_str(&format!("    #[napi(js_name = \"{}\")]\n", js_method_name));
        } else {
            output.push_str("    #[napi]\n");
        }

        let params = self.method_params(method);
        let return_type = Self::method_return_type(&method.output);
        let return_annotation = if return_type.is_empty() || return_type == "()" {
            " -> ()".to_string()
        } else {
            format!(" -> {}", return_type)
        };

        let signature = if is_constructor {
            format!(
                "    pub fn {}({}){}",
                method.name, params, return_annotation
            )
        } else if has_mut_self {
            let params_without_self = params
                .trim_start_matches("&mut self, ")
                .trim_start_matches("&mut self");
            format!(
                "    pub fn {}(&mut self, {}){}",
                method.name, params_without_self, return_annotation
            )
        } else if has_self {
            let params_without_self = params
                .trim_start_matches("&self, ")
                .trim_start_matches("&self");
            format!(
                "    pub fn {}(&self, {}){}",
                method.name, params_without_self, return_annotation
            )
        } else {
            format!(
                "    pub fn {}({}){}",
                method.name, params, return_annotation
            )
        };

        output.push_str(&signature);
        output.push_str(" {\n");

        let body = if is_constructor {
            self.constructor_body(method, struct_name, core_type)
        } else {
            self.method_body(method, has_self || has_mut_self, type_has_clone)
        };

        output.push_str(&format!("        {}\n", body));
        output.push_str("    }\n\n");

        output
    }

    fn constructor_body(
        &self,
        method: &AnalyzedMethod,
        struct_name: &str,
        core_type: &str,
    ) -> String {
        let args = method
            .inputs
            .iter()
            .filter(|(name, _)| name != "self" && name != "self_mut")
            .map(|(name, mapped_type)| {
                let is_owned = Self::is_owned_parameter(mapped_type);
                self.unwrap_arg(name, mapped_type, is_owned)
            })
            .collect::<Vec<_>>()
            .join(", ");

        format!("{} {{ inner: {}::new({}) }}", struct_name, core_type, args)
    }

    fn method_body(&self, method: &AnalyzedMethod, has_self: bool, type_has_clone: bool) -> String {
        match &method.output {
            MappedType::Direct { rust_type, .. } if rust_type == "()" => {
                self.core_method_call(method, has_self, type_has_clone)
            }
            MappedType::Direct { .. } => {
                let call = self.core_method_call(method, has_self, type_has_clone);
                self.wrap_direct_value(&method.output, &call)
            }
            MappedType::Result { ok_type, .. } => {
                let call = self.core_method_call(method, has_self, type_has_clone);
                self.wrap_result_value(ok_type, &call)
            }
            MappedType::Option { inner_type } => {
                let call = self.core_method_call(method, has_self, type_has_clone);
                self.wrap_option_value(inner_type, &call)
            }
            MappedType::Collected { item_type } => {
                let call = self.core_method_call(method, has_self, type_has_clone);
                self.wrap_collected_value(item_type, &call)
            }
            MappedType::Reference { inner_type, .. } => {
                let call = self.core_method_call(method, has_self, type_has_clone);
                self.wrap_reference_value(inner_type, &call)
            }
            MappedType::HashMap { value_type, .. } => {
                let call = self.core_method_call(method, has_self, type_has_clone);
                self.wrap_hashmap_value(value_type, &call)
            }
            MappedType::Tuple { elements } => {
                let call = self.core_method_call(method, has_self, type_has_clone);
                self.wrap_tuple_value(elements, &call)
            }
            _ => "todo!()".to_string(),
        }
    }

    fn core_method_call(
        &self,
        method: &AnalyzedMethod,
        has_self: bool,
        type_has_clone: bool,
    ) -> String {
        let args = method
            .inputs
            .iter()
            .filter(|(name, _)| name != "self" && name != "self_mut")
            .map(|(name, mapped_type)| {
                let is_owned = Self::is_owned_parameter(mapped_type);
                self.unwrap_arg(name, mapped_type, is_owned)
            })
            .collect::<Vec<_>>()
            .join(", ");

        let method_name = method.original_name.as_ref().unwrap_or(&method.name);

        if has_self {
            if is_consuming_method_name(method_name) && type_has_clone {
                format!("self.inner.clone().{}({})", method_name, args)
            } else {
                format!("self.inner.{}({})", method_name, args)
            }
        } else {
            let core_type = method.impl_type.as_deref().unwrap_or("mathhook_core");
            format!("{}::{}({})", core_type, method_name, args)
        }
    }

    fn unwrap_arg(&self, name: &str, mapped_type: &MappedType, is_owned: bool) -> String {
        match mapped_type {
            MappedType::Direct {
                node_type,
                rust_type,
                ..
            } => {
                if node_type.starts_with("Js") && node_type != "JsObject" {
                    if self.is_simple_enum_type(rust_type) {
                        format!("{}.inner.clone().into()", name)
                    } else if is_owned {
                        format!("{}.inner", name)
                    } else {
                        format!("{}.inner.clone()", name)
                    }
                } else if Self::needs_integer_cast(rust_type) {
                    format!("{} as {}", name, rust_type)
                } else if rust_type == "str" || rust_type == "&str" {
                    format!("{}.as_str()", name)
                } else {
                    name.to_string()
                }
            }
            MappedType::Reference { inner_type, .. } => match &**inner_type {
                MappedType::Direct { rust_type, .. }
                    if rust_type == "str" || rust_type == "&str" =>
                {
                    format!("{}.as_str()", name)
                }
                MappedType::Direct { node_type, .. }
                    if node_type.starts_with("Js") && node_type != "JsObject" =>
                {
                    format!("&{}.inner", name)
                }
                MappedType::HashMap {
                    key_type,
                    value_type,
                } => {
                    let needs_key_conversion = matches!(&**key_type, MappedType::Direct { node_type, .. } if node_type.starts_with("Js") && node_type != "JsObject");
                    let needs_value_conversion = matches!(&**value_type, MappedType::Direct { node_type, .. } if node_type.starts_with("Js") && node_type != "JsObject");

                    if needs_value_conversion {
                        format!(
                            "&{}.iter().map(|(k, v)| (k.clone(), v.inner.clone())).collect()",
                            name
                        )
                    } else if needs_key_conversion {
                        format!(
                            "&{}.iter().map(|(k, v)| (k.inner.clone(), v.clone())).collect()",
                            name
                        )
                    } else {
                        format!("&{}", name)
                    }
                }
                _ => format!("&{}", name),
            },
            MappedType::Option { inner_type } => match &**inner_type {
                MappedType::Direct {
                    node_type,
                    rust_type,
                    ..
                } if node_type.starts_with("Js") && node_type != "JsObject" => {
                    if self.is_simple_enum_type(rust_type) {
                        format!("{}.map(|v| v.inner.clone().into())", name)
                    } else if is_owned {
                        format!("{}.map(|v| v.inner)", name)
                    } else {
                        format!("{}.map(|v| v.inner.clone())", name)
                    }
                }
                MappedType::Reference {
                    inner_type: ref_inner,
                    ..
                } => match &**ref_inner {
                    MappedType::Direct { node_type, .. }
                        if node_type.starts_with("Js") && node_type != "JsObject" =>
                    {
                        format!("{}.as_ref().map(|v| &v.inner)", name)
                    }
                    _ => name.to_string(),
                },
                MappedType::Collected { item_type } => match &**item_type {
                    MappedType::Direct { node_type, .. }
                        if node_type.starts_with("Js") && node_type != "JsObject" =>
                    {
                        if is_owned {
                            format!("{}.map(|v| v.into_iter().map(|x| x.inner).collect())", name)
                        } else {
                            format!(
                                "{}.map(|v| v.into_iter().map(|x| x.inner.clone()).collect())",
                                name
                            )
                        }
                    }
                    _ => name.to_string(),
                },
                _ => name.to_string(),
            },
            MappedType::Collected { item_type } => {
                match &**item_type {
                    MappedType::Direct {
                        node_type,
                        rust_type,
                        ..
                    } if node_type.starts_with("Js") && node_type != "JsObject" => {
                        if self.is_simple_enum_type(rust_type) {
                            format!("{}.into_iter().map(|v| v.inner.clone().into()).collect::<Vec<_>>()", name)
                        } else if is_owned {
                            format!("{}.into_iter().map(|v| v.inner).collect::<Vec<_>>()", name)
                        } else {
                            format!(
                                "{}.into_iter().map(|v| v.inner.clone()).collect::<Vec<_>>()",
                                name
                            )
                        }
                    }
                    MappedType::Direct { rust_type, .. } if Self::needs_integer_cast(rust_type) => {
                        format!(
                            "{}.into_iter().map(|v| v as {}).collect::<Vec<_>>()",
                            name, rust_type
                        )
                    }
                    MappedType::Reference {
                        inner_type: ref_inner,
                        ..
                    } => match &**ref_inner {
                        MappedType::Direct { node_type, .. }
                            if node_type.starts_with("Js") && node_type != "JsObject" =>
                        {
                            format!("{}.iter().map(|v| &v.inner).collect::<Vec<_>>()", name)
                        }
                        _ => name.to_string(),
                    },
                    _ => name.to_string(),
                }
            }
            MappedType::HashMap {
                key_type,
                value_type,
            } => {
                let needs_key_conversion = matches!(&**key_type, MappedType::Direct { node_type, .. } if node_type.starts_with("Js") && node_type != "JsObject");
                let needs_value_conversion = matches!(&**value_type, MappedType::Direct { node_type, .. } if node_type.starts_with("Js") && node_type != "JsObject");

                if needs_value_conversion && needs_key_conversion {
                    if is_owned {
                        format!(
                            "{}.into_iter().map(|(k, v)| (k.inner, v.inner)).collect()",
                            name
                        )
                    } else {
                        format!(
                            "{}.into_iter().map(|(k, v)| (k.inner.clone(), v.inner.clone())).collect()",
                            name
                        )
                    }
                } else if needs_value_conversion {
                    if is_owned {
                        format!("{}.into_iter().map(|(k, v)| (k, v.inner)).collect()", name)
                    } else {
                        format!(
                            "{}.into_iter().map(|(k, v)| (k, v.inner.clone())).collect()",
                            name
                        )
                    }
                } else if needs_key_conversion {
                    if is_owned {
                        format!("{}.into_iter().map(|(k, v)| (k.inner, v)).collect()", name)
                    } else {
                        format!(
                            "{}.into_iter().map(|(k, v)| (k.inner.clone(), v)).collect()",
                            name
                        )
                    }
                } else {
                    name.to_string()
                }
            }
            _ => name.to_string(),
        }
    }

    fn wrap_direct_value(&self, output: &MappedType, call: &str) -> String {
        match output {
            MappedType::Direct {
                node_type,
                rust_type,
                ..
            } => {
                if node_type.starts_with("Js") && node_type != "JsObject" {
                    format!("{} {{ inner: {} }}", node_type, call)
                } else if Self::needs_integer_cast(rust_type) {
                    format!("{} as i64", call)
                } else if rust_type == "str" || rust_type == "&str" {
                    format!("{}.to_string()", call)
                } else {
                    call.to_string()
                }
            }
            _ => call.to_string(),
        }
    }

    fn wrap_option_value(&self, inner_type: &MappedType, call: &str) -> String {
        match inner_type {
            MappedType::Direct {
                node_type,
                rust_type,
                ..
            } => {
                if node_type.starts_with("Js") && node_type != "JsObject" {
                    format!("{}.map(|v| {} {{ inner: v }})", call, node_type)
                } else if Self::needs_integer_cast(rust_type) {
                    format!("{}.map(|v| v as i64)", call)
                } else if rust_type == "str" || rust_type == "&str" {
                    format!("{}.map(|v| v.to_string())", call)
                } else {
                    call.to_string()
                }
            }
            MappedType::Reference {
                inner_type: ref_inner,
                ..
            } => match &**ref_inner {
                MappedType::Direct {
                    node_type,
                    rust_type,
                    ..
                } => {
                    if node_type.starts_with("Js") && node_type != "JsObject" {
                        format!("{}.map(|v| {} {{ inner: v.clone() }})", call, node_type)
                    } else if rust_type == "str" || rust_type == "&str" {
                        format!("{}.map(|v| v.to_string())", call)
                    } else {
                        call.to_string()
                    }
                }
                _ => call.to_string(),
            },
            MappedType::Tuple { elements } => self.wrap_option_tuple_value(elements, call),
            MappedType::Collected { item_type } => {
                self.wrap_option_collected_value(item_type, call)
            }
            _ => call.to_string(),
        }
    }

    fn wrap_option_collected_value(&self, item_type: &MappedType, call: &str) -> String {
        match item_type {
            MappedType::Direct { node_type, .. }
                if node_type.starts_with("Js") && node_type != "JsObject" =>
            {
                format!(
                    "{}.map(|v| v.into_iter().map(|x| {} {{ inner: x }}).collect())",
                    call, node_type
                )
            }
            MappedType::Reference {
                inner_type: ref_inner,
                ..
            } => match &**ref_inner {
                MappedType::Direct { node_type, .. }
                    if node_type.starts_with("Js") && node_type != "JsObject" =>
                {
                    format!(
                        "{}.map(|v| v.into_iter().map(|x| {} {{ inner: x.clone() }}).collect())",
                        call, node_type
                    )
                }
                _ => call.to_string(),
            },
            _ => call.to_string(),
        }
    }

    fn wrap_option_tuple_value(&self, elements: &[MappedType], call: &str) -> String {
        let conversions: Vec<String> = elements
            .iter()
            .enumerate()
            .map(|(i, elem)| match elem {
                MappedType::Direct {
                    node_type,
                    rust_type,
                    ..
                } => {
                    if node_type.starts_with("Js") && node_type != "JsObject" {
                        format!("{} {{ inner: t.{} }}", node_type, i)
                    } else if Self::needs_integer_cast(rust_type) {
                        format!("t.{} as i64", i)
                    } else {
                        format!("t.{}", i)
                    }
                }
                _ => format!("t.{}", i),
            })
            .collect();

        format!("{}.map(|t| ({}))", call, conversions.join(", "))
    }

    fn wrap_collected_value(&self, item_type: &MappedType, call: &str) -> String {
        match item_type {
            MappedType::Direct {
                node_type,
                rust_type,
                ..
            } => {
                if node_type.starts_with("Js") && node_type != "JsObject" {
                    format!(
                        "{}.into_iter().map(|v| {} {{ inner: v }}).collect()",
                        call, node_type
                    )
                } else if Self::needs_integer_cast(rust_type) {
                    format!("{}.into_iter().map(|v| v as i64).collect()", call)
                } else if rust_type == "str" || rust_type == "&str" {
                    format!("{}.into_iter().map(|v| v.to_string()).collect()", call)
                } else {
                    call.to_string()
                }
            }
            MappedType::Reference {
                inner_type: ref_inner,
                ..
            } => match &**ref_inner {
                MappedType::Direct {
                    node_type,
                    rust_type,
                    ..
                } => {
                    if node_type.starts_with("Js") && node_type != "JsObject" {
                        format!(
                            "{}.into_iter().map(|v| {} {{ inner: v.clone() }}).collect()",
                            call, node_type
                        )
                    } else if rust_type == "str" || rust_type == "&str" {
                        format!("{}.into_iter().map(|v| v.to_string()).collect()", call)
                    } else {
                        call.to_string()
                    }
                }
                _ => call.to_string(),
            },
            MappedType::Tuple { elements } => self.wrap_collected_tuple_value(elements, call),
            _ => call.to_string(),
        }
    }

    fn wrap_collected_tuple_value(&self, elements: &[MappedType], call: &str) -> String {
        let conversions: Vec<String> = elements
            .iter()
            .enumerate()
            .map(|(i, elem)| match elem {
                MappedType::Direct {
                    node_type,
                    rust_type,
                    ..
                } => {
                    if node_type.starts_with("Js") && node_type != "JsObject" {
                        format!("{} {{ inner: t.{} }}", node_type, i)
                    } else if Self::needs_integer_cast(rust_type) {
                        format!("t.{} as i64", i)
                    } else {
                        format!("t.{}", i)
                    }
                }
                _ => format!("t.{}", i),
            })
            .collect();

        format!(
            "{}.into_iter().map(|t| ({})).collect()",
            call,
            conversions.join(", ")
        )
    }

    fn wrap_reference_value(&self, inner_type: &MappedType, call: &str) -> String {
        match inner_type {
            MappedType::Direct {
                node_type,
                rust_type,
                ..
            } => {
                if node_type.starts_with("Js") && node_type != "JsObject" {
                    format!("{} {{ inner: {}.clone() }}", node_type, call)
                } else if rust_type == "str" || rust_type == "&str" {
                    format!("{}.to_string()", call)
                } else {
                    format!("{}.clone()", call)
                }
            }
            MappedType::HashMap { value_type, .. } => {
                self.wrap_hashmap_value(value_type, &format!("{}.clone()", call))
            }
            _ => format!("{}.clone()", call),
        }
    }

    fn wrap_result_value(&self, ok_type: &MappedType, call: &str) -> String {
        match ok_type {
            MappedType::Direct {
                node_type,
                rust_type,
                ..
            } => {
                if node_type.starts_with("Js") && node_type != "JsObject" {
                    format!(
                        "{}.map(|v| {} {{ inner: v }}).map_err(|e| napi::Error::from_reason(format!(\"{{:?}}\", e)))",
                        call, node_type
                    )
                } else if Self::needs_integer_cast(rust_type) {
                    format!(
                        "{}.map(|v| v as i64).map_err(|e| napi::Error::from_reason(format!(\"{{:?}}\", e)))",
                        call
                    )
                } else {
                    format!(
                        "{}.map_err(|e| napi::Error::from_reason(format!(\"{{:?}}\", e)))",
                        call
                    )
                }
            }
            MappedType::Option { inner_type } => self.wrap_result_option_value(inner_type, call),
            MappedType::Collected { item_type } => {
                self.wrap_result_collected_value(item_type, call)
            }
            MappedType::Tuple { elements } => self.wrap_result_tuple_value(elements, call),
            _ => format!(
                "{}.map_err(|e| napi::Error::from_reason(format!(\"{{:?}}\", e)))",
                call
            ),
        }
    }

    fn wrap_result_option_value(&self, inner_type: &MappedType, call: &str) -> String {
        match inner_type {
            MappedType::Direct { node_type, .. }
                if node_type.starts_with("Js") && node_type != "JsObject" =>
            {
                format!(
                    "{}.map(|opt| opt.map(|v| {} {{ inner: v }})).map_err(|e| napi::Error::from_reason(format!(\"{{:?}}\", e)))",
                    call, node_type
                )
            }
            _ => format!(
                "{}.map_err(|e| napi::Error::from_reason(format!(\"{{:?}}\", e)))",
                call
            ),
        }
    }

    fn wrap_result_collected_value(&self, item_type: &MappedType, call: &str) -> String {
        match item_type {
            MappedType::Direct { node_type, .. }
                if node_type.starts_with("Js") && node_type != "JsObject" =>
            {
                format!(
                    "{}.map(|v| v.into_iter().map(|x| {} {{ inner: x }}).collect()).map_err(|e| napi::Error::from_reason(format!(\"{{:?}}\", e)))",
                    call, node_type
                )
            }
            _ => format!(
                "{}.map_err(|e| napi::Error::from_reason(format!(\"{{:?}}\", e)))",
                call
            ),
        }
    }

    fn wrap_result_tuple_value(&self, elements: &[MappedType], call: &str) -> String {
        let conversions: Vec<String> = elements
            .iter()
            .enumerate()
            .map(|(i, elem)| match elem {
                MappedType::Direct {
                    node_type,
                    rust_type,
                    ..
                } => {
                    if node_type.starts_with("Js") && node_type != "JsObject" {
                        format!("{} {{ inner: t.{} }}", node_type, i)
                    } else if Self::needs_integer_cast(rust_type) {
                        format!("t.{} as i64", i)
                    } else {
                        format!("t.{}", i)
                    }
                }
                _ => format!("t.{}", i),
            })
            .collect();

        format!(
            "{}.map(|t| ({})).map_err(|e| napi::Error::from_reason(format!(\"{{:?}}\", e)))",
            call,
            conversions.join(", ")
        )
    }

    fn wrap_hashmap_value(&self, value_type: &MappedType, call: &str) -> String {
        match value_type {
            MappedType::Direct { node_type, .. }
                if node_type.starts_with("Js") && node_type != "JsObject" =>
            {
                format!(
                    "{}.into_iter().map(|(k, v)| (k, {} {{ inner: v }})).collect()",
                    call, node_type
                )
            }
            MappedType::Collected { item_type } => match &**item_type {
                MappedType::Direct { node_type, .. }
                    if node_type.starts_with("Js") && node_type != "JsObject" =>
                {
                    format!(
                            "{}.into_iter().map(|(k, v)| (k, v.into_iter().map(|x| {} {{ inner: x }}).collect())).collect()",
                            call, node_type
                        )
                }
                _ => call.to_string(),
            },
            _ => call.to_string(),
        }
    }

    fn wrap_tuple_value(&self, elements: &[MappedType], call: &str) -> String {
        let needs_conversion = elements.iter().any(|e| match e {
            MappedType::Direct {
                node_type,
                rust_type,
                ..
            } => {
                (node_type.starts_with("Js") && node_type != "JsObject")
                    || Self::needs_integer_cast(rust_type)
            }
            _ => false,
        });

        if !needs_conversion {
            return call.to_string();
        }

        let conversions: Vec<String> = elements
            .iter()
            .enumerate()
            .map(|(i, elem)| {
                let field = format!("_t.{}", i);
                match elem {
                    MappedType::Direct {
                        node_type,
                        rust_type,
                        ..
                    } => {
                        if node_type.starts_with("Js") && node_type != "JsObject" {
                            format!("{} {{ inner: {} }}", node_type, field)
                        } else if Self::needs_integer_cast(rust_type) {
                            format!("{} as i64", field)
                        } else {
                            field
                        }
                    }
                    _ => field,
                }
            })
            .collect();

        format!("{{ let _t = {}; ({}) }}", call, conversions.join(", "))
    }

    fn method_params(&self, method: &AnalyzedMethod) -> String {
        method
            .inputs
            .iter()
            .map(|(name, mapped_type)| {
                if name == "self" {
                    "&self".to_string()
                } else if name == "self_mut" {
                    "&mut self".to_string()
                } else {
                    let node_type = Self::map_input_type(mapped_type);
                    format!("{}: {}", name, node_type)
                }
            })
            .collect::<Vec<_>>()
            .join(", ")
    }

    fn map_input_type(mapped_type: &MappedType) -> String {
        match mapped_type {
            MappedType::Direct {
                node_type,
                rust_type,
                ..
            } => {
                if Self::needs_integer_cast(rust_type) {
                    "i64".to_string()
                } else if rust_type == "str" || rust_type == "&str" {
                    "String".to_string()
                } else if node_type.starts_with("Js") && node_type != "JsObject" {
                    format!("&{}", node_type)
                } else {
                    node_type.clone()
                }
            }
            MappedType::Reference { inner_type, .. } => match &**inner_type {
                MappedType::Direct { rust_type, .. }
                    if rust_type == "str" || rust_type == "&str" =>
                {
                    "String".to_string()
                }
                _ => Self::map_input_type(inner_type),
            },
            MappedType::Option { inner_type } => {
                let inner = Self::map_input_type(inner_type);
                format!("Option<{}>", inner)
            }
            MappedType::Collected { item_type } => {
                let inner = Self::map_input_type(item_type);
                format!("Vec<{}>", inner)
            }
            MappedType::HashMap {
                key_type,
                value_type,
            } => {
                let key = Self::map_input_type(key_type);
                let value = Self::map_input_type(value_type);
                format!("std::collections::HashMap<{}, {}>", key, value)
            }
            _ => "JsObject".to_string(),
        }
    }

    fn method_return_type(output: &MappedType) -> String {
        match output {
            MappedType::Direct {
                node_type,
                rust_type,
                ..
            } => {
                if rust_type == "()" {
                    "()".to_string()
                } else if Self::needs_integer_cast(rust_type) {
                    "i64".to_string()
                } else if rust_type == "str" || rust_type == "&str" {
                    "String".to_string()
                } else {
                    node_type.clone()
                }
            }
            MappedType::Reference { inner_type, .. } => Self::method_return_type(inner_type),
            MappedType::Option { inner_type } => {
                let inner = Self::method_return_type(inner_type);
                format!("Option<{}>", inner)
            }
            MappedType::Collected { item_type } => {
                let inner = Self::method_return_type(item_type);
                format!("Vec<{}>", inner)
            }
            MappedType::Result { ok_type, .. } => {
                let inner = Self::method_return_type(ok_type);
                if inner == "()" {
                    "napi::Result<()>".to_string()
                } else {
                    format!("napi::Result<{}>", inner)
                }
            }
            MappedType::HashMap {
                key_type,
                value_type,
            } => {
                let key = Self::method_return_type(key_type);
                let value = Self::method_return_type(value_type);
                format!("std::collections::HashMap<{}, {}>", key, value)
            }
            MappedType::Tuple { elements } => {
                let inner: Vec<String> = elements.iter().map(Self::method_return_type).collect();
                format!("({})", inner.join(", "))
            }
            _ => "JsObject".to_string(),
        }
    }

    fn needs_integer_cast(rust_type: &str) -> bool {
        matches!(
            rust_type,
            "u8" | "u16" | "u32" | "u64" | "usize" | "i8" | "i16" | "i32" | "isize"
        )
    }

    fn emit_functions_file_impl(&self, functions: &[AnalyzedMethod]) -> String {
        let mut output = String::new();

        output.push_str("// AUTO-GENERATED by binding-codegen - DO NOT EDIT\n\n");
        output.push_str("#[allow(unused_imports)]\n");
        output.push_str("use super::*;\n");
        output.push_str("use napi::bindgen_prelude::*;\n");
        output.push_str("use napi_derive::napi;\n\n");

        for func in functions {
            if !func.is_supported || method_has_unsupported_types(func) {
                continue;
            }
            if is_skipped_function(&func.name) {
                continue;
            }

            if let Some(doc) = &func.doc_comment {
                let doc_attr = to_napi_doc(doc, 0);
                if !doc_attr.is_empty() {
                    output.push_str(&doc_attr);
                    output.push('\n');
                }
            }

            let js_func_name = to_camel_case(&func.name);
            if js_func_name != func.name {
                output.push_str(&format!("#[napi(js_name = \"{}\")]\n", js_func_name));
            } else {
                output.push_str("#[napi]\n");
            }
            let params = self.method_params(func);
            let return_type = Self::method_return_type(&func.output);
            let return_annotation = if return_type.is_empty() || return_type == "()" {
                " -> ()".to_string()
            } else {
                format!(" -> {}", return_type)
            };

            output.push_str(&format!(
                "pub fn {}({}){} {{\n",
                func.name, params, return_annotation
            ));

            let body = self.function_body(func);
            output.push_str(&format!("    {}\n", body));
            output.push_str("}\n\n");
        }

        output
    }

    fn function_body(&self, func: &AnalyzedMethod) -> String {
        match &func.output {
            MappedType::Direct { rust_type, .. } if rust_type == "()" => {
                self.core_function_call(func)
            }
            MappedType::Direct { .. } => {
                let call = self.core_function_call(func);
                self.wrap_direct_value(&func.output, &call)
            }
            MappedType::Result { ok_type, .. } => {
                let call = self.core_function_call(func);
                self.wrap_result_value(ok_type, &call)
            }
            MappedType::Option { inner_type } => {
                let call = self.core_function_call(func);
                self.wrap_option_value(inner_type, &call)
            }
            MappedType::Collected { item_type } => {
                let call = self.core_function_call(func);
                self.wrap_collected_value(item_type, &call)
            }
            MappedType::Reference { inner_type, .. } => {
                let call = self.core_function_call(func);
                self.wrap_reference_value(inner_type, &call)
            }
            MappedType::HashMap { value_type, .. } => {
                let call = self.core_function_call(func);
                self.wrap_hashmap_value(value_type, &call)
            }
            MappedType::Tuple { elements } => {
                let call = self.core_function_call(func);
                self.wrap_tuple_value(elements, &call)
            }
            _ => "todo!()".to_string(),
        }
    }

    fn core_function_call(&self, func: &AnalyzedMethod) -> String {
        let args = func
            .inputs
            .iter()
            .filter(|(name, _)| name != "self" && name != "self_mut")
            .map(|(name, mapped_type)| {
                let is_owned = Self::is_owned_parameter(mapped_type);
                self.unwrap_arg(name, mapped_type, is_owned)
            })
            .collect::<Vec<_>>()
            .join(", ");

        let func_name = func.original_name.as_ref().unwrap_or(&func.name);
        let impl_type = func.impl_type.as_deref().unwrap_or("mathhook_core");
        format!("{}::{}({})", impl_type, func_name, args)
    }
}

impl crate::emitter::Emitter for NodeEmitter {
    fn emit_type_file(&self, type_info: &TypeInfo, methods: &[AnalyzedMethod]) -> String {
        self.emit_type_file_with_traits(type_info, methods, &[], &HashMap::new())
    }

    fn emit_type_file_with_traits(
        &self,
        type_info: &TypeInfo,
        methods: &[AnalyzedMethod],
        impls: &[ImplInfo],
        trait_path_map: &HashMap<String, String>,
    ) -> String {
        let trait_analysis = TraitAnalysis::from_impls_and_derives(
            &type_info.name,
            impls,
            &type_info.derived_traits,
        );
        self.emit_type_file_impl(type_info, methods, &trait_analysis, trait_path_map)
    }

    fn emit_functions_file(&self, functions: &[AnalyzedMethod]) -> String {
        self.emit_functions_file_impl(functions)
    }

    fn wrapper_name(&self, core_type: &str) -> String {
        format!("Js{}", core_type)
    }

    fn target_name(&self) -> &'static str {
        "node"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scanner::{EnumVariant, TypeKind};

    #[test]
    fn test_emit_type_doc_with_variants_no_doc() {
        let type_info = TypeInfo {
            name: "TestEnum".to_string(),
            module_path: String::new(),
            source_file: std::path::PathBuf::new(),
            kind: TypeKind::Enum,
            is_public: true,
            has_lifetimes: false,
            doc_comment: None,
            fields: vec![],
            enum_variants: vec![
                EnumVariant {
                    name: "A".to_string(),
                    fields: vec![],
                    doc_comment: None,
                },
                EnumVariant {
                    name: "B".to_string(),
                    fields: vec![],
                    doc_comment: None,
                },
            ],
            derived_traits: vec![],
            is_cfg_gated: false,
            skip_binding: false,
        };

        let result = NodeEmitter::emit_type_doc_with_variants(&type_info, 0);
        assert!(result.is_empty());
    }

    #[test]
    fn test_emit_type_doc_with_variants_with_docs() {
        let type_info = TypeInfo {
            name: "TestEnum".to_string(),
            module_path: String::new(),
            source_file: std::path::PathBuf::new(),
            kind: TypeKind::Enum,
            is_public: true,
            has_lifetimes: false,
            doc_comment: Some("An enum for testing.".to_string()),
            fields: vec![],
            enum_variants: vec![
                EnumVariant {
                    name: "First".to_string(),
                    fields: vec![],
                    doc_comment: Some("The first variant.".to_string()),
                },
                EnumVariant {
                    name: "Second".to_string(),
                    fields: vec![],
                    doc_comment: Some("The second variant.".to_string()),
                },
            ],
            derived_traits: vec![],
            is_cfg_gated: false,
            skip_binding: false,
        };

        let result = NodeEmitter::emit_type_doc_with_variants(&type_info, 0);
        assert!(result.contains("An enum for testing."));
        assert!(result.contains("# Values"));
        assert!(result.contains("`First` - The first variant."));
        assert!(result.contains("`Second` - The second variant."));
    }

    #[test]
    fn test_emit_type_doc_with_variants_partial_docs() {
        let type_info = TypeInfo {
            name: "TestEnum".to_string(),
            module_path: String::new(),
            source_file: std::path::PathBuf::new(),
            kind: TypeKind::Enum,
            is_public: true,
            has_lifetimes: false,
            doc_comment: Some("An enum.".to_string()),
            fields: vec![],
            enum_variants: vec![
                EnumVariant {
                    name: "Documented".to_string(),
                    fields: vec![],
                    doc_comment: Some("Has docs.".to_string()),
                },
                EnumVariant {
                    name: "Undocumented".to_string(),
                    fields: vec![],
                    doc_comment: None,
                },
            ],
            derived_traits: vec![],
            is_cfg_gated: false,
            skip_binding: false,
        };

        let result = NodeEmitter::emit_type_doc_with_variants(&type_info, 0);
        assert!(result.contains("# Values"));
        assert!(result.contains("`Documented` - Has docs."));
        assert!(result.contains("`Undocumented`"));
    }

    #[test]
    fn test_is_owned_parameter() {
        let owned = MappedType::Direct {
            rust_type: "Expression".to_string(),
            python_type: "PyExpression".to_string(),
            node_type: "JsExpression".to_string(),
        };
        assert!(NodeEmitter::is_owned_parameter(&owned));

        let borrowed = MappedType::Reference {
            inner_type: Box::new(MappedType::Direct {
                rust_type: "Expression".to_string(),
                python_type: "PyExpression".to_string(),
                node_type: "JsExpression".to_string(),
            }),
            is_mut: false,
        };
        assert!(!NodeEmitter::is_owned_parameter(&borrowed));
    }

    #[test]
    fn test_unwrap_arg_owned_vs_borrowed() {
        let emitter = NodeEmitter::new();

        let wrapper_type = MappedType::Direct {
            rust_type: "Expression".to_string(),
            python_type: "PyExpression".to_string(),
            node_type: "JsExpression".to_string(),
        };

        let owned_result = emitter.unwrap_arg("expr", &wrapper_type, true);
        assert_eq!(owned_result, "expr.inner");

        let borrowed_result = emitter.unwrap_arg("expr", &wrapper_type, false);
        assert_eq!(borrowed_result, "expr.inner.clone()");
    }

    #[test]
    fn test_unwrap_arg_vec_owned_vs_borrowed() {
        let emitter = NodeEmitter::new();

        let vec_type = MappedType::Collected {
            item_type: Box::new(MappedType::Direct {
                rust_type: "Expression".to_string(),
                python_type: "PyExpression".to_string(),
                node_type: "JsExpression".to_string(),
            }),
        };

        let owned_result = emitter.unwrap_arg("exprs", &vec_type, true);
        assert_eq!(
            owned_result,
            "exprs.into_iter().map(|v| v.inner).collect::<Vec<_>>()"
        );

        let borrowed_result = emitter.unwrap_arg("exprs", &vec_type, false);
        assert_eq!(
            borrowed_result,
            "exprs.into_iter().map(|v| v.inner.clone()).collect::<Vec<_>>()"
        );
    }

    #[test]
    fn test_unwrap_arg_option_owned_vs_borrowed() {
        let emitter = NodeEmitter::new();

        let option_type = MappedType::Option {
            inner_type: Box::new(MappedType::Direct {
                rust_type: "Expression".to_string(),
                python_type: "PyExpression".to_string(),
                node_type: "JsExpression".to_string(),
            }),
        };

        let owned_result = emitter.unwrap_arg("maybe_expr", &option_type, true);
        assert_eq!(owned_result, "maybe_expr.map(|v| v.inner)");

        let borrowed_result = emitter.unwrap_arg("maybe_expr", &option_type, false);
        assert_eq!(borrowed_result, "maybe_expr.map(|v| v.inner.clone())");
    }

    #[test]
    fn test_unwrap_arg_hashmap_owned_vs_borrowed() {
        let emitter = NodeEmitter::new();

        let hashmap_type = MappedType::HashMap {
            key_type: Box::new(MappedType::Direct {
                rust_type: "String".to_string(),
                python_type: "String".to_string(),
                node_type: "String".to_string(),
            }),
            value_type: Box::new(MappedType::Direct {
                rust_type: "Expression".to_string(),
                python_type: "PyExpression".to_string(),
                node_type: "JsExpression".to_string(),
            }),
        };

        let owned_result = emitter.unwrap_arg("map", &hashmap_type, true);
        assert_eq!(
            owned_result,
            "map.into_iter().map(|(k, v)| (k, v.inner)).collect()"
        );

        let borrowed_result = emitter.unwrap_arg("map", &hashmap_type, false);
        assert_eq!(
            borrowed_result,
            "map.into_iter().map(|(k, v)| (k, v.inner.clone())).collect()"
        );
    }
}
