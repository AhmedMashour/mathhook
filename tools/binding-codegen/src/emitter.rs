use crate::analyzer::{analyze_method_with_registry, TypeRegistry};
use crate::classifier::ClassifiedApi;
use crate::scanner::{ImplInfo, ScannedApi, TypeInfo};
use crate::trait_analyzer::TraitAnalysis;
use crate::types::{AnalyzedMethod, MappedType};
use std::collections::{HashMap, HashSet};

pub mod node;
pub mod python;

pub use node::NodeEmitter;
pub use python::PythonEmitter;

pub trait Emitter {
    fn emit_type_file(&self, type_info: &TypeInfo, methods: &[AnalyzedMethod]) -> String;

    fn emit_type_file_with_traits(
        &self,
        type_info: &TypeInfo,
        methods: &[AnalyzedMethod],
        impls: &[ImplInfo],
        trait_path_map: &HashMap<String, String>,
    ) -> String {
        let _ = impls;
        let _ = trait_path_map;
        self.emit_type_file(type_info, methods)
    }

    fn emit_functions_file(&self, functions: &[AnalyzedMethod]) -> String;

    fn wrapper_name(&self, core_type: &str) -> String;

    fn target_name(&self) -> &'static str;

    fn generate_all(
        &self,
        classified: &ClassifiedApi,
        scanned: &ScannedApi,
    ) -> HashMap<String, String> {
        let mut files = HashMap::new();

        let registry = build_type_registry(classified);
        let trait_path_map = scanned.build_trait_path_map();
        let scanned_impls = &scanned.impls;

        for type_info in classified
            .primary_types
            .iter()
            .chain(classified.helper_types.iter())
        {
            let has_default_trait = scanned_impls.iter().any(|impl_block| {
                impl_block.target_type == type_info.name
                    && impl_block
                        .trait_name
                        .as_ref()
                        .is_some_and(|t| t == "Default")
            });

            let raw_methods = scanned_impls
                .iter()
                .filter(|impl_block| {
                    impl_block.target_type == type_info.name && impl_block.trait_name.is_none()
                })
                .flat_map(|impl_block| &impl_block.methods)
                .filter(|m| m.is_public)
                .filter(|m| {
                    if has_default_trait && m.name == "new" {
                        let has_no_non_self_params = m
                            .signature
                            .inputs
                            .iter()
                            .all(|(name, _)| name == "self" || name == "self_mut");
                        !has_no_non_self_params
                    } else {
                        true
                    }
                })
                .collect::<Vec<_>>();

            let mut analyzed_methods: Vec<AnalyzedMethod> = raw_methods
                .iter()
                .map(|m| {
                    analyze_method_with_registry(m, Some(type_info.name.clone()), registry.clone())
                })
                .collect();

            let trait_analysis = TraitAnalysis::from_impls_and_derives(
                &type_info.name,
                scanned_impls,
                &type_info.derived_traits,
            );

            for domain_method in &trait_analysis.domain_trait_methods {
                let analyzed = analyze_method_with_registry(
                    &domain_method.method_info,
                    Some(type_info.name.clone()),
                    registry.clone(),
                );
                analyzed_methods.push(analyzed);
            }

            let analyzed_methods = resolve_overloads(analyzed_methods);

            let filename = format!("{}.rs", to_snake_case(&type_info.name));
            let content = self.emit_type_file_with_traits(
                type_info,
                &analyzed_methods,
                scanned_impls,
                &trait_path_map,
            );
            files.insert(filename, content);
        }

        if !classified.standalone_functions.is_empty() {
            let analyzed_functions: Vec<AnalyzedMethod> = classified
                .standalone_functions
                .iter()
                .map(|f| {
                    let ctx =
                        crate::analyzer::TypeContext::new(None).with_registry(registry.clone());
                    let mut method = AnalyzedMethod {
                        name: f.name.clone(),
                        original_name: None,
                        inputs: f
                            .signature
                            .inputs
                            .iter()
                            .map(|(name, ty)| {
                                (
                                    name.clone(),
                                    crate::analyzer::analyze_type_with_context(ty, &ctx),
                                )
                            })
                            .collect(),
                        output: f
                            .signature
                            .output
                            .as_ref()
                            .map(|ty| crate::analyzer::analyze_type_with_context(ty, &ctx))
                            .unwrap_or_else(|| MappedType::Direct {
                                rust_type: "()".to_string(),
                                python_type: "()".to_string(),
                                node_type: "void".to_string(),
                            }),
                        is_supported: true,
                        impl_type: Some(if f.module_path.is_empty() {
                            "mathhook_core".to_string()
                        } else {
                            format!("mathhook_core::{}", f.module_path)
                        }),
                        requires_mut_self: false,
                        doc_comment: f.doc_comment.clone(),
                        skip_binding: f.skip_binding,
                    };
                    let has_unsupported_inputs =
                        method.inputs.iter().any(|(_, ty)| !ty.is_supported());
                    let has_unsupported_output = !method.output.is_supported();

                    if has_unsupported_inputs || has_unsupported_output {
                        method.is_supported = false;
                    }
                    method
                })
                .collect();

            let analyzed_functions = resolve_overloads(analyzed_functions);

            let content = self.emit_functions_file(&analyzed_functions);
            files.insert("functions.rs".to_string(), content);
        }

        files
    }
}

fn build_type_registry(classified: &ClassifiedApi) -> TypeRegistry {
    let type_names = classified
        .primary_types
        .iter()
        .chain(classified.helper_types.iter())
        .map(|t| t.name.clone());
    TypeRegistry::with_types(type_names)
}

#[allow(dead_code)]
fn type_signature(ty: &MappedType) -> String {
    match ty {
        MappedType::Direct { rust_type, .. } => rust_type.clone(),
        MappedType::Reference { inner_type, is_mut } => {
            format!(
                "&{}{}",
                if *is_mut { "mut " } else { "" },
                type_signature(inner_type)
            )
        }
        MappedType::Option { inner_type } => {
            format!("Option<{}>", type_signature(inner_type))
        }
        MappedType::Collected { item_type } => {
            format!("Vec<{}>", type_signature(item_type))
        }
        MappedType::HashMap {
            key_type,
            value_type,
        } => {
            format!(
                "HashMap<{},{}>",
                type_signature(key_type),
                type_signature(value_type)
            )
        }
        MappedType::Tuple { elements } => {
            let inner = elements
                .iter()
                .map(type_signature)
                .collect::<Vec<_>>()
                .join(",");
            format!("({})", inner)
        }
        MappedType::Result { ok_type, err_type } => {
            format!("Result<{},{}>", type_signature(ok_type), err_type)
        }
        MappedType::Callback {
            arg_types,
            return_type,
            is_mut,
        } => {
            let args = arg_types
                .iter()
                .map(type_signature)
                .collect::<Vec<_>>()
                .join(",");
            format!(
                "Fn{}({}):{}",
                if *is_mut { "Mut" } else { "" },
                args,
                type_signature(return_type)
            )
        }
        MappedType::Union { .. } => "Union".to_string(),
        MappedType::Unsupported { rust_type, .. } => rust_type.clone(),
    }
}

fn sanitize_param_name(name: &str) -> &str {
    name.trim_start_matches('_')
}

fn resolve_overloads(methods: Vec<AnalyzedMethod>) -> Vec<AnalyzedMethod> {
    let mut name_groups: HashMap<String, Vec<AnalyzedMethod>> = HashMap::new();

    for method in methods {
        name_groups
            .entry(method.name.clone())
            .or_default()
            .push(method);
    }

    let mut resolved = Vec::new();

    for (original_name, mut group) in name_groups {
        if group.len() == 1 {
            resolved.push(group.pop().unwrap());
        } else {
            group.sort_by_key(|m| m.inputs.iter().filter(|(n, _)| n != "self").count());

            let mut used_suffixes: HashSet<String> = HashSet::new();

            for (idx, mut method) in group.into_iter().enumerate() {
                if idx == 0 {
                    resolved.push(method);
                } else {
                    let mut suffix = generate_overload_suffix(&method);

                    if used_suffixes.contains(&suffix) {
                        let type_suffix = generate_type_based_suffix(&method);
                        suffix = format!("{}_{}", suffix, type_suffix);
                    }

                    let mut counter = 2;
                    let base_suffix = suffix.clone();
                    while used_suffixes.contains(&suffix) {
                        suffix = format!("{}_{}", base_suffix, counter);
                        counter += 1;
                    }

                    used_suffixes.insert(suffix.clone());
                    method.original_name = Some(original_name.clone());
                    method.name = format!("{}_{}", original_name, suffix);
                    resolved.push(method);
                }
            }
        }
    }

    resolved
}

fn generate_overload_suffix(method: &AnalyzedMethod) -> String {
    let non_self_params: Vec<_> = method
        .inputs
        .iter()
        .filter(|(name, _)| name != "self" && name != "self_mut")
        .collect();

    if non_self_params.is_empty() {
        return "all".to_string();
    }

    let param_names: Vec<_> = non_self_params
        .iter()
        .map(|(name, _)| sanitize_param_name(name))
        .collect();

    match param_names.as_slice() {
        ["key"] | ["name"] | ["id"] => "with_key".to_string(),
        ["var"] | ["variable"] => "with_var".to_string(),
        ["expr"] => "with_expr".to_string(),
        ["value"] => "with_value".to_string(),
        _ => format!("with_{}", param_names.join("_")),
    }
}

fn generate_type_based_suffix(method: &AnalyzedMethod) -> String {
    let non_self_params: Vec<_> = method
        .inputs
        .iter()
        .filter(|(name, _)| name != "self" && name != "self_mut")
        .collect();

    if non_self_params.is_empty() {
        return "void".to_string();
    }

    let type_names: Vec<String> = non_self_params
        .iter()
        .map(|(_, ty)| short_type_name(ty))
        .collect();

    type_names.join("_")
}

fn short_type_name(ty: &MappedType) -> String {
    match ty {
        MappedType::Direct { rust_type, .. } => match rust_type.as_str() {
            "i8" | "i16" | "i32" | "i64" | "isize" => "int".to_string(),
            "u8" | "u16" | "u32" | "u64" | "usize" => "uint".to_string(),
            "f32" | "f64" => "float".to_string(),
            "bool" => "bool".to_string(),
            "String" | "str" => "str".to_string(),
            "Expression" => "expr".to_string(),
            "Symbol" => "sym".to_string(),
            other => to_snake_case(other),
        },
        MappedType::Reference { inner_type, .. } => {
            format!("ref_{}", short_type_name(inner_type))
        }
        MappedType::Option { inner_type } => {
            format!("opt_{}", short_type_name(inner_type))
        }
        MappedType::Collected { item_type } => {
            format!("vec_{}", short_type_name(item_type))
        }
        MappedType::HashMap {
            key_type,
            value_type,
        } => {
            format!(
                "map_{}_{}",
                short_type_name(key_type),
                short_type_name(value_type)
            )
        }
        MappedType::Tuple { elements } => {
            let inner = elements
                .iter()
                .map(short_type_name)
                .collect::<Vec<_>>()
                .join("_");
            format!("tuple_{}", inner)
        }
        _ => "unknown".to_string(),
    }
}

pub fn to_snake_case(s: &str) -> String {
    let mut result = String::new();
    let chars: Vec<char> = s.chars().collect();

    for (i, ch) in chars.iter().enumerate() {
        if ch.is_uppercase() {
            let is_start = i == 0;
            let prev_is_lower = i > 0 && chars[i - 1].is_lowercase();
            let next_is_lower = i + 1 < chars.len() && chars[i + 1].is_lowercase();

            if !is_start && (prev_is_lower || next_is_lower) {
                result.push('_');
            }

            result.push(ch.to_lowercase().next().unwrap());
        } else {
            result.push(*ch);
        }
    }

    result
}

pub fn organize_methods_by_module(
    methods: &[AnalyzedMethod],
) -> Vec<(String, Vec<&AnalyzedMethod>)> {
    let mut by_module: HashMap<String, Vec<&AnalyzedMethod>> = HashMap::new();

    for method in methods {
        let module = extract_module_from_name(&method.name);
        by_module.entry(module).or_default().push(method);
    }

    let mut result: Vec<_> = by_module.into_iter().collect();
    result.sort_by(|a, b| a.0.cmp(&b.0));
    result
}

fn extract_module_from_name(name: &str) -> String {
    if name.starts_with("expand") || name.starts_with("factor") {
        "algebra".to_string()
    } else if name.starts_with("derivative")
        || name.starts_with("diff")
        || name.starts_with("integrate")
    {
        "calculus".to_string()
    } else if name.starts_with("simplify") {
        "simplify".to_string()
    } else {
        "core".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_snake_case() {
        assert_eq!(to_snake_case("Expression"), "expression");
        assert_eq!(to_snake_case("MathSolver"), "math_solver");
        assert_eq!(to_snake_case("ODESolver"), "ode_solver");
        assert_eq!(to_snake_case("PolyZp"), "poly_zp");
    }

    #[test]
    fn test_extract_module() {
        assert_eq!(extract_module_from_name("expand"), "algebra");
        assert_eq!(extract_module_from_name("factor"), "algebra");
        assert_eq!(extract_module_from_name("derivative"), "calculus");
        assert_eq!(extract_module_from_name("diff"), "calculus");
        assert_eq!(extract_module_from_name("simplify"), "simplify");
        assert_eq!(extract_module_from_name("parse"), "core");
    }

    #[test]
    fn test_generate_overload_suffix() {
        let method_with_key = AnalyzedMethod {
            name: "clear_cache".to_string(),
            original_name: None,
            inputs: vec![(
                "key".to_string(),
                MappedType::Direct {
                    rust_type: "String".to_string(),
                    python_type: "str".to_string(),
                    node_type: "string".to_string(),
                },
            )],
            output: MappedType::Direct {
                rust_type: "()".to_string(),
                python_type: "()".to_string(),
                node_type: "void".to_string(),
            },
            is_supported: true,
            impl_type: None,
            requires_mut_self: false,
            doc_comment: None,
            skip_binding: false,
        };

        assert_eq!(generate_overload_suffix(&method_with_key), "with_key");
    }

    #[test]
    fn test_generate_overload_suffix_underscore_prefix() {
        let method_with_underscore = AnalyzedMethod {
            name: "solve".to_string(),
            original_name: None,
            inputs: vec![(
                "_variable".to_string(),
                MappedType::Direct {
                    rust_type: "Symbol".to_string(),
                    python_type: "PySymbol".to_string(),
                    node_type: "JsSymbol".to_string(),
                },
            )],
            output: MappedType::Direct {
                rust_type: "()".to_string(),
                python_type: "()".to_string(),
                node_type: "void".to_string(),
            },
            is_supported: true,
            impl_type: None,
            requires_mut_self: false,
            doc_comment: None,
            skip_binding: false,
        };

        assert_eq!(
            generate_overload_suffix(&method_with_underscore),
            "with_var"
        );
    }

    #[test]
    fn test_sanitize_param_name() {
        assert_eq!(sanitize_param_name("_variable"), "variable");
        assert_eq!(sanitize_param_name("variable"), "variable");
        assert_eq!(sanitize_param_name("__x"), "x");
        assert_eq!(sanitize_param_name("x"), "x");
    }

    #[test]
    fn test_type_based_suffix() {
        let method_i64 = AnalyzedMethod {
            name: "from".to_string(),
            original_name: None,
            inputs: vec![(
                "value".to_string(),
                MappedType::Direct {
                    rust_type: "i64".to_string(),
                    python_type: "i64".to_string(),
                    node_type: "number".to_string(),
                },
            )],
            output: MappedType::Direct {
                rust_type: "Self".to_string(),
                python_type: "Self".to_string(),
                node_type: "Self".to_string(),
            },
            is_supported: true,
            impl_type: None,
            requires_mut_self: false,
            doc_comment: None,
            skip_binding: false,
        };

        let method_f64 = AnalyzedMethod {
            name: "from".to_string(),
            original_name: None,
            inputs: vec![(
                "value".to_string(),
                MappedType::Direct {
                    rust_type: "f64".to_string(),
                    python_type: "f64".to_string(),
                    node_type: "number".to_string(),
                },
            )],
            output: MappedType::Direct {
                rust_type: "Self".to_string(),
                python_type: "Self".to_string(),
                node_type: "Self".to_string(),
            },
            is_supported: true,
            impl_type: None,
            requires_mut_self: false,
            doc_comment: None,
            skip_binding: false,
        };

        assert_eq!(generate_type_based_suffix(&method_i64), "int");
        assert_eq!(generate_type_based_suffix(&method_f64), "float");
    }
}
