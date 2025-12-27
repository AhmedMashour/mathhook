use crate::doc_transformer::to_python_docstring;
use crate::scanner::{ImplInfo, TypeInfo};
use crate::trait_analyzer::{SupportedTrait, TraitAnalysis};
use crate::types::{AnalyzedMethod, BindabilityRegistry, MappedType};
use std::collections::{HashMap, HashSet};

const SKIPPED_FUNCTIONS: &[&str] = &[
    "get_universal_registry",
    "get_simplification_registry",
    "get_global_background_compute",
    "get_performance_optimizer",
    "get_cached_expression",
    "get_cache_stats",
    "build_expr_list",
];

fn is_skipped_function(name: &str) -> bool {
    SKIPPED_FUNCTIONS.contains(&name)
}

pub struct PythonEmitter {
    simple_enum_types: HashSet<String>,
    bindability_registry: BindabilityRegistry,
}

impl Default for PythonEmitter {
    fn default() -> Self {
        Self::new()
    }
}

impl PythonEmitter {
    pub fn new() -> Self {
        Self {
            simple_enum_types: HashSet::new(),
            bindability_registry: BindabilityRegistry::new(),
        }
    }

    pub fn with_bindability_registry(registry: BindabilityRegistry) -> Self {
        Self {
            simple_enum_types: HashSet::new(),
            bindability_registry: registry,
        }
    }

    pub fn with_simple_enum_types(types: impl IntoIterator<Item = String>) -> Self {
        let mut emitter = Self::new();
        emitter.simple_enum_types = types.into_iter().collect();
        emitter
    }

    pub fn add_simple_enum_type(&mut self, type_name: String) {
        self.simple_enum_types.insert(type_name);
    }

    pub fn set_bindability_registry(&mut self, registry: BindabilityRegistry) {
        self.bindability_registry = registry;
    }

    fn is_simple_enum_type(&self, type_name: &str) -> bool {
        self.simple_enum_types.contains(type_name)
    }

    fn is_non_clone_type(&self, type_name: &str) -> bool {
        !self.bindability_registry.is_clone_type(type_name)
    }

    fn is_wrapper_type(python_type: &str, rust_type: &str) -> bool {
        python_type.starts_with("Py") && python_type.len() > 2 && rust_type != "PyResult"
    }

    fn needs_integer_cast(rust_type: &str) -> bool {
        matches!(
            rust_type,
            "usize" | "isize" | "u8" | "u16" | "u32" | "u64" | "i8" | "i16" | "i32"
        )
    }

    fn is_consuming_method(method_name: &str) -> bool {
        matches!(method_name, "optimize" | "into_inner" | "take" | "consume")
    }

    fn is_type_bindable(&self, type_name: &str) -> bool {
        self.bindability_registry.is_bindable_type(type_name)
    }

    fn is_error_displayable(&self, err_type: &str) -> bool {
        self.bindability_registry.is_error_displayable(err_type)
            || self.bindability_registry.is_display_type(err_type)
    }

    fn has_tuple_reference_input(method: &AnalyzedMethod) -> bool {
        method.inputs.iter().any(|(_, ty)| {
            if let MappedType::Reference { inner_type, .. } = ty {
                matches!(&**inner_type, MappedType::Tuple { .. })
            } else {
                false
            }
        })
    }

    fn should_skip_type(&self, type_info: &TypeInfo) -> bool {
        !type_info.is_public
            || type_info.is_cfg_gated
            || type_info.skip_binding
            || !self.is_type_bindable(&type_info.name)
    }

    fn is_static_ref_return(&self, output: &MappedType) -> bool {
        if let MappedType::Reference { inner_type, .. } = output {
            if let MappedType::Direct { rust_type, .. } = &**inner_type {
                return !self.is_type_bindable(rust_type);
            }
        }
        false
    }

    fn returns_static_str_vec(method: &AnalyzedMethod) -> bool {
        if let MappedType::Collected { item_type } = &method.output {
            if let MappedType::Reference { inner_type, .. } = &**item_type {
                if let MappedType::Direct { rust_type, .. } = &**inner_type {
                    return rust_type == "str";
                }
            }
        }
        false
    }

    fn has_tuple_of_references(method: &AnalyzedMethod) -> bool {
        if let MappedType::Tuple { elements } = &method.output {
            for elem in elements {
                match elem {
                    MappedType::Reference { .. } => return true,
                    MappedType::Direct { python_type, .. } if python_type.starts_with("Py") => {
                        return true;
                    }
                    _ => {}
                }
            }
        }
        false
    }

    fn returns_tuple_needing_wrapping(method: &AnalyzedMethod) -> bool {
        match &method.output {
            MappedType::Tuple { elements } => elements.iter().any(Self::tuple_elem_needs_wrapping),
            MappedType::Option { inner_type } => {
                if let MappedType::Tuple { elements } = &**inner_type {
                    elements.iter().any(Self::tuple_elem_needs_wrapping)
                } else {
                    false
                }
            }
            _ => false,
        }
    }

    fn tuple_elem_needs_wrapping(elem: &MappedType) -> bool {
        match elem {
            MappedType::Direct {
                python_type,
                rust_type,
                ..
            } => python_type.starts_with("Py") || rust_type == "usize",
            _ => false,
        }
    }

    fn takes_u64_input(method: &AnalyzedMethod) -> bool {
        method
            .inputs
            .iter()
            .any(|(_, ty)| matches!(ty, MappedType::Direct { rust_type, .. } if rust_type == "u64"))
    }

    fn has_vec_of_tuples_input(method: &AnalyzedMethod) -> bool {
        method.inputs.iter().any(|(_, ty)| {
            if let MappedType::Collected { item_type } = ty {
                matches!(&**item_type, MappedType::Tuple { .. })
            } else if let MappedType::Reference { inner_type, .. } = ty {
                if let MappedType::Collected { item_type } = &**inner_type {
                    matches!(&**item_type, MappedType::Tuple { .. })
                } else {
                    false
                }
            } else {
                false
            }
        })
    }

    fn method_returns_problematic_type(&self, method: &AnalyzedMethod) -> bool {
        self.output_contains_problematic_type(&method.output)
    }

    fn output_contains_problematic_type(&self, output: &MappedType) -> bool {
        match output {
            MappedType::Direct { rust_type, .. } => !self.is_type_bindable(rust_type),
            MappedType::Option { inner_type } => self.output_contains_problematic_type(inner_type),
            MappedType::Collected { item_type } => self.output_contains_problematic_type(item_type),
            MappedType::Result { ok_type, err_type } => {
                self.output_contains_problematic_type(ok_type)
                    || !self.is_error_displayable(err_type)
            }
            MappedType::Reference { inner_type, .. } => {
                self.output_contains_problematic_type(inner_type)
            }
            _ => false,
        }
    }

    fn method_uses_problematic_input(&self, method: &AnalyzedMethod) -> bool {
        method
            .inputs
            .iter()
            .any(|(_, ty)| self.input_contains_problematic_type(ty))
    }

    fn input_contains_problematic_type(&self, input: &MappedType) -> bool {
        match input {
            MappedType::Direct { rust_type, .. } => !self.is_type_bindable(rust_type),
            MappedType::Option { inner_type } => self.input_contains_problematic_type(inner_type),
            MappedType::Collected { item_type } => self.input_contains_problematic_type(item_type),
            MappedType::Reference { inner_type, .. } => {
                self.input_contains_problematic_type(inner_type)
            }
            _ => false,
        }
    }

    fn function_returns_static_ref(&self, func: &AnalyzedMethod) -> bool {
        self.is_static_ref_return(&func.output)
    }

    pub fn emit_method(
        &self,
        method: &AnalyzedMethod,
        core_type: &str,
        reserved_names: &HashSet<String>,
    ) -> String {
        if reserved_names.contains(&method.name) {
            return format!(
                "    // Skipped: {} - name conflicts with trait-generated method\n",
                method.name
            );
        }

        if method.skip_binding {
            return format!(
                "    // Skipped: {} - marked with @no-binding\n",
                method.name
            );
        }

        if !method.is_supported {
            return format!("    // TODO: {} - unsupported signature\n", method.name);
        }

        if self.method_returns_problematic_type(method) {
            return format!(
                "    // TODO: {} - returns problematic type (path mismatch)\n",
                method.name
            );
        }

        if self.method_uses_problematic_input(method) {
            return format!(
                "    // TODO: {} - uses problematic input type\n",
                method.name
            );
        }

        if self.is_static_ref_return(&method.output) {
            return format!("    // TODO: {} - returns static reference\n", method.name);
        }

        if Self::has_tuple_of_references(method) {
            return format!(
                "    // TODO: {} - has tuple with references (needs manual binding)\n",
                method.name
            );
        }

        if Self::has_tuple_reference_input(method) {
            return format!(
                "    // TODO: {} - has tuple reference input (needs manual binding)\n",
                method.name
            );
        }

        if Self::returns_tuple_needing_wrapping(method) {
            return format!(
                "    // TODO: {} - returns tuple needing wrapper conversion (needs manual binding)\n",
                method.name
            );
        }

        if Self::has_vec_of_tuples_input(method) {
            return format!(
                "    // TODO: {} - takes Vec of tuples (needs manual binding)\n",
                method.name
            );
        }

        let params = self.method_params(method);
        let return_type = Self::method_return_type(&method.output);
        let body = self.method_body(method, true);

        let has_self = method.inputs.iter().any(|(name, _)| name == "self");

        if method.requires_mut_self {
            if self.is_non_clone_type(core_type) {
                return format!(
                    "    // TODO: {} - requires &mut self but type doesn't implement Clone\n",
                    method.name
                );
            }
            let wrapper_name = format!("Py{}", core_type);
            let body = self.method_body_mut_self(method, core_type);
            let mut_return_type = match &method.output {
                MappedType::Result { .. } => format!("PyResult<{}>", wrapper_name),
                _ => wrapper_name.clone(),
            };
            let doc_lines = Self::emit_doc_comment(&method.doc_comment, "    ");
            return format!(
                "{}    pub fn {}({}) -> {} {{\n        {}\n    }}\n",
                doc_lines, method.name, params, mut_return_type, body
            );
        }

        let doc_lines = Self::emit_doc_comment(&method.doc_comment, "    ");

        if has_self {
            format!(
                "{}    pub fn {}({}) -> {} {{\n        {}\n    }}\n",
                doc_lines, method.name, params, return_type, body
            )
        } else {
            let body = self.static_method_body(method, core_type);
            format!(
                "    #[staticmethod]\n{}    pub fn {}({}) -> {} {{\n        {}\n    }}\n",
                doc_lines, method.name, params, return_type, body
            )
        }
    }

    fn emit_doc_comment(doc_comment: &Option<String>, indent: &str) -> String {
        match doc_comment {
            Some(doc) if !doc.is_empty() => {
                let mut result = to_python_docstring(doc, indent.len());
                if !result.is_empty() {
                    result.push('\n');
                }
                result
            }
            _ => String::new(),
        }
    }

    fn method_body_mut_self(&self, method: &AnalyzedMethod, core_type: &str) -> String {
        let args: Vec<String> = method
            .inputs
            .iter()
            .filter(|(name, _)| name != "self")
            .map(|(name, mapped_type)| self.unwrap_arg(name, mapped_type))
            .collect();

        let method_name = method.original_name.as_ref().unwrap_or(&method.name);

        let call = format!(
            "{}::{}(self_mut.inner.clone(), {})",
            core_type,
            method_name,
            args.join(", ")
        );

        match &method.output {
            MappedType::Result { .. } => {
                format!(
                    "{}.map_err(|e| PyValueError::new_err(e.to_string())).map(|v| v.into())",
                    call
                )
            }
            _ => format!("{}.into()", call),
        }
    }

    fn static_method_body(&self, method: &AnalyzedMethod, core_type: &str) -> String {
        let args: Vec<String> = method
            .inputs
            .iter()
            .map(|(name, mapped_type)| self.unwrap_arg(name, mapped_type))
            .collect();

        let method_name = method.original_name.as_ref().unwrap_or(&method.name);
        let call = format!("{}::{}({})", core_type, method_name, args.join(", "));

        match &method.output {
            MappedType::Result { ok_type, .. } => {
                let wrapper = Self::wrap_result_value(ok_type, true);
                format!(
                    "{}.map_err(|e| PyValueError::new_err(e.to_string())){}",
                    call, wrapper
                )
            }
            MappedType::Direct { python_type, .. } if python_type == "Self" => {
                format!("Self {{ inner: {} }}", call)
            }
            MappedType::Direct {
                python_type,
                rust_type,
                ..
            } => {
                if Self::is_wrapper_type(python_type, rust_type) {
                    format!("{}.into()", call)
                } else if Self::needs_integer_cast(rust_type) {
                    format!("{} as i64", call)
                } else {
                    call
                }
            }
            MappedType::Option { inner_type } => Self::wrap_option_value(inner_type, &call, true),
            MappedType::Collected { item_type } => {
                Self::wrap_collected_value(item_type, &call, true)
            }
            MappedType::HashMap {
                key_type,
                value_type,
            } => Self::wrap_hashmap_value(key_type, value_type, &call, true),
            _ => call,
        }
    }

    fn method_params(&self, method: &AnalyzedMethod) -> String {
        method
            .inputs
            .iter()
            .map(|(name, mapped_type)| {
                let py_type = Self::map_to_python_type(mapped_type);
                if name == "self" {
                    "&self".to_string()
                } else if name == "self_mut" {
                    "self_mut: Self".to_string()
                } else {
                    format!("{}: {}", name, py_type)
                }
            })
            .collect::<Vec<_>>()
            .join(", ")
    }

    fn method_return_type(output: &MappedType) -> String {
        match output {
            MappedType::Result { ok_type, .. } => {
                let inner = Self::map_to_python_type(ok_type);
                format!("PyResult<{}>", inner)
            }
            other => Self::map_to_python_type(other),
        }
    }

    fn map_to_python_type(mapped: &MappedType) -> String {
        match mapped {
            MappedType::Direct {
                python_type,
                rust_type,
                ..
            } => {
                if python_type == "Self" {
                    "Self".to_string()
                } else if python_type == "()" {
                    "()".to_string()
                } else if Self::is_wrapper_type(python_type, rust_type) {
                    python_type.clone()
                } else if Self::needs_integer_cast(rust_type) {
                    "i64".to_string()
                } else if rust_type == "str" {
                    "String".to_string()
                } else {
                    python_type.clone()
                }
            }
            MappedType::Reference { inner_type, .. } => match &**inner_type {
                MappedType::Direct { rust_type, .. }
                    if rust_type == "str" || rust_type == "String" =>
                {
                    "String".to_string()
                }
                _ => Self::map_to_python_type(inner_type),
            },
            MappedType::Option { inner_type } => {
                let inner = Self::map_to_python_type(inner_type);
                format!("Option<{}>", inner)
            }
            MappedType::Collected { item_type } => {
                let inner = Self::map_to_python_type(item_type);
                format!("Vec<{}>", inner)
            }
            MappedType::HashMap {
                key_type,
                value_type,
            } => {
                let key = Self::map_to_python_type(key_type);
                let value = Self::map_to_python_type(value_type);
                format!("std::collections::HashMap<{}, {}>", key, value)
            }
            MappedType::Result { ok_type, .. } => {
                let inner = Self::map_to_python_type(ok_type);
                format!("PyResult<{}>", inner)
            }
            MappedType::Tuple { elements } => {
                let inner: Vec<String> = elements.iter().map(Self::map_to_python_type).collect();
                format!("({})", inner.join(", "))
            }
            MappedType::Unsupported { reason, rust_type } => {
                format!("/* unsupported: {} ({}) */", rust_type, reason)
            }
            MappedType::Callback { .. } => "/* callback */".to_string(),
            MappedType::Union { .. } => "/* union */".to_string(),
        }
    }

    fn wrap_result_value(ok_type: &MappedType, use_self: bool) -> String {
        match ok_type {
            MappedType::Direct {
                python_type,
                rust_type,
                ..
            } => {
                if use_self && python_type == "Self" {
                    ".map(|inner| Self { inner })".to_string()
                } else if Self::is_wrapper_type(python_type, rust_type) {
                    ".map(|v| v.into())".to_string()
                } else if Self::needs_integer_cast(rust_type) {
                    ".map(|v| v as i64)".to_string()
                } else if rust_type == "str" {
                    ".map(|s| s.to_string())".to_string()
                } else {
                    String::new()
                }
            }
            MappedType::Option { inner_type } => match &**inner_type {
                MappedType::Direct {
                    python_type,
                    rust_type,
                    ..
                } => {
                    if use_self && python_type == "Self" {
                        ".map(|opt| opt.map(|inner| Self { inner }))".to_string()
                    } else if Self::is_wrapper_type(python_type, rust_type) {
                        ".map(|opt| opt.map(|v| v.into()))".to_string()
                    } else if Self::needs_integer_cast(rust_type) {
                        ".map(|opt| opt.map(|v| v as i64))".to_string()
                    } else if rust_type == "str" {
                        ".map(|opt| opt.map(|s| s.to_string()))".to_string()
                    } else {
                        String::new()
                    }
                }
                MappedType::Reference { inner_type, .. } => match &**inner_type {
                    MappedType::Direct {
                        python_type,
                        rust_type,
                        ..
                    } => {
                        if Self::is_wrapper_type(python_type, rust_type) {
                            ".map(|opt| opt.map(|v| v.clone().into()))".to_string()
                        } else if rust_type == "str" {
                            ".map(|opt| opt.map(|s| s.to_string()))".to_string()
                        } else {
                            ".map(|opt| opt.cloned())".to_string()
                        }
                    }
                    _ => String::new(),
                },
                _ => String::new(),
            },
            MappedType::Collected { item_type } => match &**item_type {
                MappedType::Direct {
                    python_type,
                    rust_type,
                    ..
                } => {
                    if use_self && python_type == "Self" {
                        ".map(|v| v.into_iter().map(|inner| Self { inner }).collect())".to_string()
                    } else if Self::is_wrapper_type(python_type, rust_type) {
                        ".map(|v| v.into_iter().map(|item| item.into()).collect())".to_string()
                    } else if Self::needs_integer_cast(rust_type) {
                        ".map(|v| v.into_iter().map(|item| item as i64).collect())".to_string()
                    } else if rust_type == "str" {
                        ".map(|v| v.into_iter().map(|s| s.to_string()).collect())".to_string()
                    } else {
                        String::new()
                    }
                }
                MappedType::Reference { inner_type, .. } => match &**inner_type {
                    MappedType::Direct { rust_type, .. } if rust_type == "str" => {
                        ".map(|v| v.into_iter().map(|s| s.to_string()).collect())".to_string()
                    }
                    _ => ".map(|v| v.into_iter().cloned().collect())".to_string(),
                },
                _ => String::new(),
            },
            MappedType::Reference { inner_type, .. } => {
                let inner_wrap = Self::wrap_result_value(inner_type, use_self);
                if inner_wrap.is_empty() {
                    ".map(|v| v.clone())".to_string()
                } else {
                    inner_wrap
                }
            }
            MappedType::Tuple { elements } => {
                if Self::tuple_needs_wrapping(elements) {
                    let wrap_expr = Self::generate_tuple_wrap(elements, "v", use_self);
                    format!(".map(|v| {})", wrap_expr)
                } else {
                    String::new()
                }
            }
            MappedType::HashMap {
                key_type: _,
                value_type,
            } => match &**value_type {
                MappedType::Direct {
                    python_type,
                    rust_type,
                    ..
                } if Self::is_wrapper_type(python_type, rust_type) => {
                    ".map(|m| m.into_iter().map(|(k, v)| (k, v.into())).collect())".to_string()
                }
                _ => String::new(),
            },
            _ => String::new(),
        }
    }

    fn tuple_needs_wrapping(elements: &[MappedType]) -> bool {
        elements.iter().any(Self::type_needs_wrapping)
    }

    fn generate_tuple_wrap(elements: &[MappedType], var: &str, use_self: bool) -> String {
        let len = elements.len();
        let bindings: Vec<String> = (0..len).map(|i| format!("t{}", i)).collect();
        let pattern = format!("({})", bindings.join(", "));

        let wrapped: Vec<String> = elements
            .iter()
            .enumerate()
            .map(|(i, e)| Self::wrap_single_element(e, &bindings[i], use_self))
            .collect();

        format!("{{ let {} = {}; ({}) }}", pattern, var, wrapped.join(", "))
    }

    fn wrap_single_element(mapped: &MappedType, var: &str, use_self: bool) -> String {
        match mapped {
            MappedType::Direct {
                python_type,
                rust_type,
                ..
            } => {
                if use_self && python_type == "Self" {
                    format!("Self {{ inner: {} }}", var)
                } else if Self::is_wrapper_type(python_type, rust_type) {
                    format!("{}.into()", var)
                } else if Self::needs_integer_cast(rust_type) {
                    format!("{} as i64", var)
                } else {
                    var.to_string()
                }
            }
            MappedType::Option { inner_type } => {
                let inner_wrap = Self::wrap_single_element(inner_type, "v", use_self);
                if inner_wrap == "v" {
                    var.to_string()
                } else {
                    format!("{}.map(|v| {})", var, inner_wrap)
                }
            }
            _ => var.to_string(),
        }
    }

    fn unwrap_single_element(&self, mapped: &MappedType, var: &str) -> String {
        match mapped {
            MappedType::Direct {
                python_type,
                rust_type,
                ..
            } => {
                if Self::is_wrapper_type(python_type, rust_type) || python_type == "Self" {
                    if self.is_simple_enum_type(rust_type) {
                        format!("{}.into()", var)
                    } else {
                        format!("{}.inner.clone()", var)
                    }
                } else if rust_type == "String" {
                    format!("{}.to_string()", var)
                } else if Self::needs_integer_cast(rust_type) {
                    format!("{} as {}", var, rust_type)
                } else {
                    var.to_string()
                }
            }
            _ => var.to_string(),
        }
    }

    fn generate_tuple_unwrap(&self, elements: &[MappedType]) -> String {
        let len = elements.len();
        let bindings: Vec<String> = (0..len).map(|i| format!("t{}", i)).collect();
        let pattern = format!("({})", bindings.join(", "));

        let unwrapped: Vec<String> = elements
            .iter()
            .enumerate()
            .map(|(i, e)| self.unwrap_single_element(e, &bindings[i]))
            .collect();

        format!("{{ let {} = t; ({}) }}", pattern, unwrapped.join(", "))
    }

    fn tuple_needs_unwrapping(&self, elements: &[MappedType]) -> bool {
        elements.iter().any(|e| {
            matches!(e, MappedType::Direct { python_type, rust_type, .. } if Self::is_wrapper_type(python_type, rust_type) || python_type == "Self")
        })
    }

    fn method_body(&self, method: &AnalyzedMethod, use_self: bool) -> String {
        let has_self = method.inputs.iter().any(|(name, _)| name == "self");

        match &method.output {
            MappedType::Result { ok_type, .. } => {
                let inner_call = self.core_method_call(method, has_self);
                let wrapper = Self::wrap_result_value(ok_type, use_self);
                format!(
                    "{}.map_err(|e| PyValueError::new_err(e.to_string())){}",
                    inner_call, wrapper
                )
            }
            MappedType::Direct { .. } => {
                let call = self.core_method_call(method, has_self);
                Self::wrap_direct_value(&method.output, &call, use_self)
            }
            MappedType::Option { inner_type } => {
                let call = self.core_method_call(method, has_self);
                Self::wrap_option_value(inner_type, &call, use_self)
            }
            MappedType::Collected { item_type } => {
                let call = self.core_method_call(method, has_self);
                if Self::returns_static_str_vec(method) {
                    return format!("{}.into_iter().map(|v| v.to_string()).collect()", call);
                }
                Self::wrap_collected_value(item_type, &call, use_self)
            }
            MappedType::HashMap {
                key_type,
                value_type,
            } => {
                let call = self.core_method_call(method, has_self);
                Self::wrap_hashmap_value(key_type, value_type, &call, use_self)
            }
            MappedType::Reference { inner_type, .. } => {
                let call = self.core_method_call(method, has_self);
                Self::wrap_reference_value(inner_type, &call, use_self)
            }
            MappedType::Tuple { elements } => {
                let call = self.core_method_call(method, has_self);
                Self::wrap_tuple_value(elements, &call, use_self)
            }
            _ => "todo!()".to_string(),
        }
    }

    fn core_method_call(&self, method: &AnalyzedMethod, has_self: bool) -> String {
        let args = method
            .inputs
            .iter()
            .filter(|(name, _)| name != "self")
            .map(|(name, mapped_type)| self.unwrap_arg(name, mapped_type))
            .collect::<Vec<_>>()
            .join(", ");

        let method_name = method.original_name.as_ref().unwrap_or(&method.name);

        if has_self {
            if Self::is_consuming_method(method_name) {
                format!("self.inner.clone().{}({})", method_name, args)
            } else {
                format!("self.inner.{}({})", method_name, args)
            }
        } else {
            let core_type = method.impl_type.as_deref().unwrap_or("mathhook_core");
            format!("{}::{}({})", core_type, method_name, args)
        }
    }

    fn unwrap_arg(&self, name: &str, mapped_type: &MappedType) -> String {
        match mapped_type {
            MappedType::Direct {
                python_type,
                rust_type,
                ..
            } => {
                if Self::is_wrapper_type(python_type, rust_type) || python_type == "Self" {
                    if self.is_simple_enum_type(rust_type) {
                        format!("{}.into()", name)
                    } else {
                        format!("{}.inner.clone()", name)
                    }
                } else if rust_type == "String" {
                    format!("{}.to_string()", name)
                } else if Self::needs_integer_cast(rust_type) {
                    format!("{} as {}", name, rust_type)
                } else {
                    name.to_string()
                }
            }
            MappedType::Reference { inner_type, .. } => match &**inner_type {
                MappedType::Direct {
                    python_type,
                    rust_type,
                    ..
                } => {
                    if rust_type == "str" || rust_type == "String" {
                        format!("{}.as_str()", name)
                    } else if Self::is_wrapper_type(python_type, rust_type) || python_type == "Self"
                    {
                        format!("&{}.inner", name)
                    } else {
                        name.to_string()
                    }
                }
                MappedType::HashMap {
                    key_type: _,
                    value_type,
                } => self.unwrap_hashmap_arg(name, value_type, true),
                _ => name.to_string(),
            },
            MappedType::Option { inner_type } => match &**inner_type {
                MappedType::Direct {
                    python_type,
                    rust_type,
                    ..
                } => {
                    if Self::is_wrapper_type(python_type, rust_type) || python_type == "Self" {
                        if self.is_simple_enum_type(rust_type) {
                            format!("{}.map(|v| v.into())", name)
                        } else {
                            format!("{}.map(|v| v.inner.clone())", name)
                        }
                    } else {
                        name.to_string()
                    }
                }
                MappedType::Tuple { elements } => {
                    if self.tuple_needs_unwrapping(elements) {
                        let unwrap_expr = self.generate_tuple_unwrap(elements);
                        format!("{}.map(|t| {})", name, unwrap_expr)
                    } else {
                        name.to_string()
                    }
                }
                _ => name.to_string(),
            },
            MappedType::Collected { item_type } => match &**item_type {
                MappedType::Direct {
                    python_type,
                    rust_type,
                    ..
                } => {
                    if Self::is_wrapper_type(python_type, rust_type) || python_type == "Self" {
                        if self.is_simple_enum_type(rust_type) {
                            format!("{}.into_iter().map(|v| v.into()).collect()", name)
                        } else {
                            format!("{}.into_iter().map(|v| v.inner.clone()).collect()", name)
                        }
                    } else if Self::needs_integer_cast(rust_type) {
                        format!("{}.into_iter().map(|v| v as {}).collect()", name, rust_type)
                    } else {
                        name.to_string()
                    }
                }
                _ => name.to_string(),
            },
            MappedType::HashMap { value_type, .. } => {
                self.unwrap_hashmap_arg(name, value_type, false)
            }
            _ => name.to_string(),
        }
    }

    fn unwrap_hashmap_arg(&self, name: &str, value_type: &MappedType, is_ref: bool) -> String {
        let ref_prefix = if is_ref { "&" } else { "" };

        match value_type {
            MappedType::Direct {
                python_type,
                rust_type,
                ..
            } if Self::is_wrapper_type(python_type, rust_type) => {
                format!(
                    "{}{}.iter().map(|(k, v)| (k.clone(), v.inner.clone())).collect()",
                    ref_prefix, name
                )
            }
            MappedType::Collected { item_type } => match &**item_type {
                MappedType::Direct {
                    python_type,
                    rust_type,
                    ..
                } if Self::is_wrapper_type(python_type, rust_type) => {
                    format!(
                        "{}{}.iter().map(|(k, v)| (k.clone(), v.iter().map(|x| x.inner.clone()).collect())).collect()",
                        ref_prefix, name
                    )
                }
                _ => format!("{}{}", ref_prefix, name),
            },
            _ => format!("{}{}", ref_prefix, name),
        }
    }

    fn wrap_direct_value(output: &MappedType, call: &str, use_self: bool) -> String {
        match output {
            MappedType::Direct {
                python_type,
                rust_type,
                ..
            } => {
                if use_self && python_type == "Self" {
                    format!("Self {{ inner: {} }}", call)
                } else if Self::is_wrapper_type(python_type, rust_type) {
                    format!("{}.into()", call)
                } else if Self::needs_integer_cast(rust_type) {
                    format!("{} as i64", call)
                } else if rust_type == "str" {
                    format!("{}.to_string()", call)
                } else {
                    call.to_string()
                }
            }
            _ => call.to_string(),
        }
    }

    fn wrap_option_value(inner_type: &MappedType, call: &str, use_self: bool) -> String {
        match inner_type {
            MappedType::Direct {
                python_type,
                rust_type,
                ..
            } => {
                if use_self && python_type == "Self" {
                    format!("{}.map(|inner| Self {{ inner }})", call)
                } else if Self::is_wrapper_type(python_type, rust_type) {
                    format!("{}.map(|v| v.into())", call)
                } else if Self::needs_integer_cast(rust_type) {
                    format!("{}.map(|v| v as i64)", call)
                } else if rust_type == "str" {
                    format!("{}.map(|s| s.to_string())", call)
                } else {
                    call.to_string()
                }
            }
            MappedType::Reference { inner_type, .. } => match &**inner_type {
                MappedType::Direct {
                    python_type,
                    rust_type,
                    ..
                } => {
                    if Self::is_wrapper_type(python_type, rust_type) {
                        format!("{}.map(|v| v.clone().into())", call)
                    } else if rust_type == "str" {
                        format!("{}.map(|s| s.to_string())", call)
                    } else {
                        format!("{}.cloned()", call)
                    }
                }
                _ => call.to_string(),
            },
            MappedType::Collected { item_type } => {
                let inner_wrap = Self::wrap_collected_value(item_type, "v", use_self);
                if inner_wrap == "v" {
                    call.to_string()
                } else {
                    format!("{}.map(|v| {})", call, inner_wrap)
                }
            }
            _ => call.to_string(),
        }
    }

    fn wrap_collected_value(item_type: &MappedType, call: &str, use_self: bool) -> String {
        match item_type {
            MappedType::Direct {
                python_type,
                rust_type,
                ..
            } => {
                if use_self && python_type == "Self" {
                    format!(
                        "{}.into_iter().map(|inner| Self {{ inner }}).collect()",
                        call
                    )
                } else if Self::is_wrapper_type(python_type, rust_type) {
                    format!("{}.into_iter().map(|v| v.into()).collect()", call)
                } else if Self::needs_integer_cast(rust_type) {
                    format!("{}.into_iter().map(|v| v as i64).collect()", call)
                } else if rust_type == "str" {
                    format!("{}.into_iter().map(|s| s.to_string()).collect()", call)
                } else {
                    call.to_string()
                }
            }
            MappedType::Reference { inner_type, .. } => match &**inner_type {
                MappedType::Direct { rust_type, .. } if rust_type == "str" => {
                    format!("{}.into_iter().map(|s| s.to_string()).collect()", call)
                }
                MappedType::Direct {
                    python_type,
                    rust_type,
                    ..
                } if Self::is_wrapper_type(python_type, rust_type) => {
                    format!("{}.into_iter().map(|v| v.clone().into()).collect()", call)
                }
                _ => format!("{}.into_iter().cloned().collect()", call),
            },
            MappedType::Tuple { elements } => {
                if Self::tuple_needs_wrapping(elements) {
                    let wrap_expr = Self::generate_tuple_wrap(elements, "t", use_self);
                    format!("{}.into_iter().map(|t| {}).collect()", call, wrap_expr)
                } else {
                    call.to_string()
                }
            }
            _ => call.to_string(),
        }
    }

    fn wrap_hashmap_value(
        _key_type: &MappedType,
        value_type: &MappedType,
        call: &str,
        use_self: bool,
    ) -> String {
        match value_type {
            MappedType::Direct {
                python_type,
                rust_type,
                ..
            } => {
                if use_self && python_type == "Self" {
                    format!(
                        "{}.into_iter().map(|(k, v)| (k, Self {{ inner: v }})).collect()",
                        call
                    )
                } else if Self::is_wrapper_type(python_type, rust_type) {
                    format!("{}.into_iter().map(|(k, v)| (k, v.into())).collect()", call)
                } else if Self::needs_integer_cast(rust_type) {
                    format!("{}.into_iter().map(|(k, v)| (k, v as i64)).collect()", call)
                } else {
                    call.to_string()
                }
            }
            MappedType::Collected { item_type } => {
                let inner_wrap = Self::wrap_collected_value(item_type, "v", use_self);
                if inner_wrap == "v" {
                    call.to_string()
                } else {
                    format!(
                        "{}.into_iter().map(|(k, v)| (k, {})).collect()",
                        call, inner_wrap
                    )
                }
            }
            _ => call.to_string(),
        }
    }

    fn wrap_reference_value(inner_type: &MappedType, call: &str, use_self: bool) -> String {
        match inner_type {
            MappedType::Direct {
                python_type,
                rust_type,
                ..
            } => {
                if use_self && python_type == "Self" {
                    format!("Self {{ inner: {}.clone() }}", call)
                } else if Self::is_wrapper_type(python_type, rust_type) {
                    format!("{}.clone().into()", call)
                } else if Self::needs_integer_cast(rust_type) {
                    format!("*{} as i64", call)
                } else if rust_type == "str" || rust_type == "String" {
                    format!("{}.to_string()", call)
                } else {
                    format!("{}.clone()", call)
                }
            }
            MappedType::HashMap {
                key_type,
                value_type,
            } => {
                // Clone the reference and convert HashMap values
                Self::wrap_hashmap_value(
                    key_type,
                    value_type,
                    &format!("{}.clone()", call),
                    use_self,
                )
            }
            _ => call.to_string(),
        }
    }

    fn wrap_tuple_value(elements: &[MappedType], call: &str, use_self: bool) -> String {
        if Self::tuple_needs_wrapping(elements) {
            Self::generate_tuple_wrap(elements, call, use_self)
        } else {
            call.to_string()
        }
    }

    fn type_needs_wrapping(mapped: &MappedType) -> bool {
        match mapped {
            MappedType::Direct {
                python_type,
                rust_type,
                ..
            } => Self::is_wrapper_type(python_type, rust_type) || python_type == "Self",
            MappedType::Option { inner_type } => Self::type_needs_wrapping(inner_type),
            MappedType::Collected { item_type } => Self::type_needs_wrapping(item_type),
            MappedType::HashMap { value_type, .. } => Self::type_needs_wrapping(value_type),
            MappedType::Reference { inner_type, .. } => Self::type_needs_wrapping(inner_type),
            MappedType::Tuple { elements } => elements.iter().any(Self::type_needs_wrapping),
            _ => false,
        }
    }

    fn emit_type_file_impl(
        &self,
        type_info: &TypeInfo,
        methods: &[AnalyzedMethod],
        trait_analysis: &TraitAnalysis,
        trait_path_map: &HashMap<String, String>,
    ) -> String {
        let core_type = &type_info.name;
        let wrapper_name = format!("Py{}", core_type);

        if self.should_skip_type(type_info) {
            return format!(
                "// AUTO-GENERATED by binding-codegen - DO NOT EDIT\n\n// Skipped: {} (not public, cfg-gated, or not bindable)\n",
                core_type
            );
        }

        let import_path = if type_info.module_path.is_empty() {
            format!("use mathhook_core::{};", core_type)
        } else {
            format!(
                "use mathhook_core::{}::{};",
                type_info.module_path, core_type
            )
        };

        let trait_imports: Vec<String> = trait_analysis
            .unique_trait_imports(trait_path_map)
            .iter()
            .map(|p| format!("use {};", p))
            .collect();

        let has_clone = type_info.derived_traits.iter().any(|t| t == "Clone")
            || self.bindability_registry.is_clone_type(core_type);
        let skip_clone = !has_clone;

        let mut lines = vec![
            "// AUTO-GENERATED by binding-codegen - DO NOT EDIT".to_string(),
            String::new(),
            "#![allow(deprecated)]".to_string(), // Module-level inner attribute
            String::new(),
            "#[allow(unused_imports)]".to_string(),
            "use super::*;".to_string(),
            "use pyo3::prelude::*;".to_string(),
            "use pyo3::exceptions::PyValueError;".to_string(),
            import_path,
        ];

        for trait_import in trait_imports {
            lines.push(trait_import);
        }

        lines.push(String::new());
        lines.push(format!("#[pyclass(name = \"{}\")]", core_type));

        if !skip_clone {
            lines.push("#[derive(Clone)]".to_string());
        }

        lines.extend([
            format!("pub struct {} {{", wrapper_name),
            format!("    pub(crate) inner: {},", core_type),
            "}".to_string(),
            String::new(),
            "#[pymethods]".to_string(),
            format!("impl {} {{", wrapper_name),
        ]);

        let mut reserved_names: HashSet<String> = HashSet::new();

        if trait_analysis.has_trait(SupportedTrait::Default) {
            reserved_names.insert("new".to_string());
            lines.push("    #[new]".to_string());
            lines.push("    pub fn new() -> Self {".to_string());
            lines.push(format!(
                "        Self {{ inner: {}::default() }}",
                core_type
            ));
            lines.push("    }".to_string());
            lines.push(String::new());
        }

        lines.push("    // ===== OPERATORS (from trait impls) =====".to_string());
        lines.push(String::new());

        for op in trait_analysis.binary_ops() {
            if let Some(method_name) = op.python_method_name() {
                reserved_names.insert(method_name.to_string());
                let op_impl = self.generate_binary_op(op, &wrapper_name, core_type, skip_clone);
                lines.push(op_impl);
            }
        }

        for op in trait_analysis.unary_ops() {
            if let Some(method_name) = op.python_method_name() {
                reserved_names.insert(method_name.to_string());
                let op_impl = Self::generate_unary_op(op, &wrapper_name);
                lines.push(op_impl);
            }
        }

        if trait_analysis.has_trait(SupportedTrait::Display) {
            reserved_names.insert("__str__".to_string());
            reserved_names.insert("__repr__".to_string());
            lines.push(Self::generate_display(&wrapper_name));
        }

        if trait_analysis.has_trait(SupportedTrait::PartialEq) {
            reserved_names.insert("__eq__".to_string());
            lines.push(Self::generate_eq());
        }

        if trait_analysis.has_trait(SupportedTrait::Clone) && !skip_clone {
            reserved_names.insert("__copy__".to_string());
            reserved_names.insert("__deepcopy__".to_string());
            lines.push(Self::generate_clone(&wrapper_name));
        }

        if trait_analysis.has_trait(SupportedTrait::Hash) {
            reserved_names.insert("__hash__".to_string());
            lines.push(Self::generate_hash());
        }

        let grouped_methods = self.group_methods_by_category(methods);

        for (category, category_methods) in grouped_methods {
            if !category_methods.is_empty() {
                lines.push(format!("    // ===== {} =====", category.to_uppercase()));
                lines.push(String::new());
                for method in category_methods {
                    lines.push(self.emit_method(method, core_type, &reserved_names));
                }
            }
        }

        lines.push("}".to_string());
        lines.push(String::new());

        lines.push(format!("impl From<{}> for {} {{", core_type, wrapper_name));
        lines.push(format!("    fn from(core: {}) -> Self {{", core_type));
        lines.push("        Self { inner: core }".to_string());
        lines.push("    }".to_string());
        lines.push("}".to_string());

        if type_info.is_simple_enum() {
            lines.push(String::new());
            lines.push(format!("impl From<{}> for {} {{", wrapper_name, core_type));
            lines.push(format!("    fn from(wrapper: {}) -> Self {{", wrapper_name));
            lines.push("        wrapper.inner".to_string());
            lines.push("    }".to_string());
            lines.push("}".to_string());
        }

        lines.join("\n")
    }

    fn emit_functions_file_impl(&self, functions: &[AnalyzedMethod]) -> String {
        let mut lines = vec![
            "// AUTO-GENERATED by binding-codegen - DO NOT EDIT".to_string(),
            String::new(),
            "#![allow(deprecated)]".to_string(), // Module-level inner attribute
            String::new(),
            "#[allow(unused_imports)]".to_string(),
            "use super::*;".to_string(),
            "use pyo3::prelude::*;".to_string(),
            "use pyo3::exceptions::PyValueError;".to_string(),
            String::new(),
            "#[pymodule]".to_string(),
            "pub fn functions(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {".to_string(),
        ];

        let reserved_names: HashSet<String> = HashSet::new();

        for func in functions {
            if !func.is_supported {
                continue;
            }

            if is_skipped_function(&func.name) {
                lines.push(format!("    // Skipped: {} - in skip list", func.name));
                continue;
            }

            if self.function_returns_static_ref(func) {
                lines.push(format!(
                    "    // TODO: {} - returns static reference",
                    func.name
                ));
                continue;
            }

            if self.method_returns_problematic_type(func) {
                lines.push(format!(
                    "    // TODO: {} - returns problematic type",
                    func.name
                ));
                continue;
            }

            if self.method_uses_problematic_input(func) {
                lines.push(format!(
                    "    // TODO: {} - uses problematic input type",
                    func.name
                ));
                continue;
            }

            if Self::has_tuple_of_references(func) {
                lines.push(format!(
                    "    // TODO: {} - has tuple with references",
                    func.name
                ));
                continue;
            }

            if Self::has_tuple_reference_input(func) {
                lines.push(format!(
                    "    // TODO: {} - has tuple reference input",
                    func.name
                ));
                continue;
            }

            if Self::returns_tuple_needing_wrapping(func) {
                lines.push(format!(
                    "    // TODO: {} - returns tuple needing wrapper conversion",
                    func.name
                ));
                continue;
            }

            if Self::has_vec_of_tuples_input(func) {
                lines.push(format!("    // TODO: {} - takes Vec of tuples", func.name));
                continue;
            }

            if Self::takes_u64_input(func) {
                lines.push(format!(
                    "    // TODO: {} - takes u64 input (needs manual binding)",
                    func.name
                ));
                continue;
            }

            if reserved_names.contains(&func.name) {
                lines.push(format!(
                    "    // Skipped: {} - name conflicts with reserved name",
                    func.name
                ));
                continue;
            }

            let is_static = !func.inputs.iter().any(|(name, _)| name == "self");
            if is_static {
                let params = self.method_params(func);
                let return_type = Self::method_return_type(&func.output);
                let body = self.method_body(func, false);

                lines.push("    #[pyfn(m)]".to_string());
                lines.push(format!(
                    "    fn {}({}) -> {} {{",
                    func.name, params, return_type
                ));
                lines.push(format!("        {}", body));
                lines.push("    }".to_string());
                lines.push(String::new());
            }
        }

        lines.push("    Ok(())".to_string());
        lines.push("}".to_string());

        lines.join("\n")
    }

    fn group_methods_by_category<'a>(
        &self,
        methods: &'a [AnalyzedMethod],
    ) -> Vec<(&'static str, Vec<&'a AnalyzedMethod>)> {
        let categories = [
            "core",
            "arithmetic",
            "comparison",
            "conversion",
            "query",
            "manipulation",
            "io",
            "simplify",
            "educational",
            "advanced",
        ];

        let mut grouped: Vec<(&str, Vec<&AnalyzedMethod>)> =
            categories.iter().map(|c| (*c, Vec::new())).collect();

        for method in methods {
            let category = Self::categorize_method(&method.name);
            if let Some((_, methods)) = grouped.iter_mut().find(|(c, _)| *c == category) {
                methods.push(method);
            }
        }

        grouped.retain(|(_, methods)| !methods.is_empty());
        grouped
    }

    fn categorize_method(name: &str) -> &'static str {
        if name.starts_with("to_")
            || name.starts_with("from_")
            || name.starts_with("into_")
            || name.starts_with("as_")
        {
            "conversion"
        } else if name.starts_with("is_")
            || name.starts_with("has_")
            || name.starts_with("can_")
            || name.starts_with("contains_")
        {
            "query"
        } else if name.starts_with("add")
            || name.starts_with("sub")
            || name.starts_with("mul")
            || name.starts_with("div")
            || name.starts_with("pow")
            || name.starts_with("neg")
        {
            "arithmetic"
        } else if name.starts_with("cmp")
            || name.starts_with("eq")
            || name.starts_with("lt")
            || name.starts_with("gt")
            || name.starts_with("le")
            || name.starts_with("ge")
        {
            "comparison"
        } else if name.starts_with("read")
            || name.starts_with("write")
            || name.starts_with("load")
            || name.starts_with("save")
            || name.starts_with("parse")
            || name.starts_with("serialize")
        {
            "io"
        } else if name.contains("simplif") || name.contains("canonical") || name.contains("normal")
        {
            "simplify"
        } else if name.contains("step")
            || name.contains("explain")
            || name.contains("teach")
            || name.contains("learn")
        {
            "educational"
        } else if name.starts_with("set_")
            || name.starts_with("get_")
            || name.starts_with("with_")
            || name.starts_with("clear_")
            || name.starts_with("push")
            || name.starts_with("pop")
            || name.starts_with("insert")
            || name.starts_with("remove")
        {
            "manipulation"
        } else if name.contains("advanced")
            || name.contains("complex")
            || name.contains("special")
            || name.contains("internal")
        {
            "advanced"
        } else {
            "core"
        }
    }

    fn generate_binary_op(
        &self,
        op: SupportedTrait,
        wrapper_name: &str,
        _core_type: &str,
        skip_clone: bool,
    ) -> String {
        let (method_name, op_symbol) = match op {
            SupportedTrait::Add => ("__add__", "+"),
            SupportedTrait::Sub => ("__sub__", "-"),
            SupportedTrait::Mul => ("__mul__", "*"),
            SupportedTrait::Div => ("__truediv__", "/"),
            _ => return String::new(),
        };

        if skip_clone {
            format!(
                "    // TODO: {} - type doesn't implement Clone\n",
                method_name
            )
        } else {
            format!(
                "    pub fn {}(&self, other: &{}) -> {} {{\n        {} {{ inner: self.inner.clone() {} other.inner.clone() }}\n    }}\n",
                method_name, wrapper_name, wrapper_name, wrapper_name, op_symbol
            )
        }
    }

    fn generate_unary_op(op: SupportedTrait, wrapper_name: &str) -> String {
        let method_name = match op {
            SupportedTrait::Neg => "__neg__",
            _ => return String::new(),
        };

        format!(
            "    pub fn {}(&self) -> {} {{\n        {} {{ inner: -self.inner.clone() }}\n    }}\n",
            method_name, wrapper_name, wrapper_name
        )
    }

    fn generate_display(wrapper_name: &str) -> String {
        format!(
            "    pub fn __str__(&self) -> String {{\n        format!(\"{{}}\", self.inner)\n    }}\n\n    pub fn __repr__(&self) -> String {{\n        format!(\"{}({{}})\", self.inner)\n    }}\n",
            wrapper_name
        )
    }

    fn generate_eq() -> String {
        "    pub fn __eq__(&self, other: &Self) -> bool {\n        self.inner == other.inner\n    }\n".to_string()
    }

    fn generate_clone(wrapper_name: &str) -> String {
        format!(
            "    pub fn __copy__(&self) -> {} {{\n        self.clone()\n    }}\n\n    pub fn __deepcopy__(&self, _memo: &pyo3::Bound<'_, pyo3::types::PyDict>) -> {} {{\n        self.clone()\n    }}\n",
            wrapper_name, wrapper_name
        )
    }

    fn generate_hash() -> String {
        "    pub fn __hash__(&self) -> u64 {\n        use std::hash::{Hash, Hasher};\n        let mut hasher = std::collections::hash_map::DefaultHasher::new();\n        self.inner.hash(&mut hasher);\n        hasher.finish()\n    }\n".to_string()
    }
}

impl crate::emitter::Emitter for PythonEmitter {
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
        format!("Py{}", core_type)
    }

    fn target_name(&self) -> &'static str {
        "python"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_wrapper_type() {
        assert!(PythonEmitter::is_wrapper_type("PyExpression", "Expression"));
        assert!(PythonEmitter::is_wrapper_type("PySymbol", "Symbol"));
        assert!(!PythonEmitter::is_wrapper_type("String", "String"));
        assert!(!PythonEmitter::is_wrapper_type("bool", "bool"));
        assert!(!PythonEmitter::is_wrapper_type("PyResult", "PyResult"));
    }

    #[test]
    fn test_needs_integer_cast() {
        assert!(PythonEmitter::needs_integer_cast("usize"));
        assert!(PythonEmitter::needs_integer_cast("i32"));
        assert!(!PythonEmitter::needs_integer_cast("i64"));
        assert!(!PythonEmitter::needs_integer_cast("f64"));
    }

    #[test]
    fn test_simple_enum_types() {
        let mut emitter = PythonEmitter::new();
        assert!(!emitter.is_simple_enum_type("SomeEnum"));

        emitter.add_simple_enum_type("SomeEnum".to_string());
        assert!(emitter.is_simple_enum_type("SomeEnum"));
    }

    #[test]
    fn test_bindability_registry_integration() {
        use crate::types::TypeTraits;

        let mut registry = BindabilityRegistry::new();
        registry.register_type(
            "Expression".to_string(),
            TypeTraits {
                has_clone: true,
                has_display: true,
                has_debug: true,
                is_public: true,
                is_in_public_module: true,
                is_cfg_gated: false,
                skip_binding: false,
            },
        );
        registry.register_type(
            "InternalCache".to_string(),
            TypeTraits {
                has_clone: false,
                has_display: false,
                has_debug: false,
                is_public: false,
                is_in_public_module: false,
                is_cfg_gated: false,
                skip_binding: false,
            },
        );

        let emitter = PythonEmitter::with_bindability_registry(registry);

        assert!(emitter.is_type_bindable("Expression"));
        assert!(!emitter.is_type_bindable("InternalCache"));
    }
}
