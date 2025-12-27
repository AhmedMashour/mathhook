use crate::config::BindingConfig;
use crate::scanner::{FunctionInfo, ImplInfo, MethodSignature, ScannedApi, TypeInfo};

fn contains_private_module_segment(module_path: &str) -> bool {
    const PRIVATE_MODULES: &[&str] = &[
        "decomposition",
        "cache",
        "element",
        "degree_bounds",
        "trial_division",
        "variable_order",
        "interpolation",
        "univariate",
        "sparse",
        "helpers",
        "basic_rules",
        "engine",
        "implementation",
        "internal",
        "private",
        "basic",
        "generation",
        "buchberger",
        "efficient_buchberger",
        "reduction",
        "s_polynomial",
        "monomial_order",
    ];

    for segment in module_path.split("::") {
        if PRIVATE_MODULES.contains(&segment) {
            return true;
        }
    }
    false
}

fn is_internal_type_name(name: &str) -> bool {
    if name.contains("Internal") || name.contains("Private") {
        return true;
    }

    if name.starts_with("Cached") || name.starts_with("Raw") {
        return true;
    }

    if matches!(
        name,
        "Zp" | "PolyZp"
            | "PolyFlat"
            | "Poly"
            | "Number"
            | "TokenType"
            | "MessageKey"
            | "SimdOps"
            | "MonomialOrder"
            | "MonomialOrdering"
            | "SparsePolynomial"
            | "Monomial"
    ) {
        return true;
    }

    const WEAK_SUFFIXES: &[&str] = &[
        "Data",
        "Builder",
        "Matcher",
        "MatcherBuilder",
        "Info",
        "Registry",
        "Cache",
        "Stats",
        "Statistics",
        "Config",
        "Context",
        "Factory",
        "Profiler",
        "Optimizer",
        "Accelerator",
        "Formatter",
    ];

    for suffix in WEAK_SUFFIXES {
        if let Some(prefix) = name.strip_suffix(suffix) {
            if is_generic_prefix(prefix) {
                return true;
            }
        }
    }

    if let Some(prefix) = name.strip_suffix("Error") {
        if is_too_short_prefix(prefix) {
            return true;
        }
    }

    if name == "Result" {
        return true;
    }
    if let Some(prefix) = name.strip_suffix("Result") {
        if is_too_short_prefix(prefix) {
            return true;
        }
    }

    false
}

fn is_generic_prefix(prefix: &str) -> bool {
    if prefix.is_empty() {
        return true;
    }
    if prefix.len() < 3 {
        return true;
    }
    if prefix.starts_with('_') {
        return true;
    }
    if prefix.chars().all(|c| c.is_ascii_uppercase()) && prefix.len() <= 2 {
        return true;
    }
    false
}

fn is_too_short_prefix(prefix: &str) -> bool {
    if prefix.is_empty() {
        return true;
    }
    if prefix.len() < 5 {
        return true;
    }
    if prefix.starts_with('_') {
        return true;
    }
    false
}

#[derive(Debug, Clone, PartialEq)]
pub enum TypeClassification {
    Primary,
    Helper,
    Skip,
}

pub struct ClassifiedApi {
    pub primary_types: Vec<TypeInfo>,
    pub helper_types: Vec<TypeInfo>,
    pub skipped_types: Vec<TypeInfo>,
    pub standalone_functions: Vec<FunctionInfo>,
}

pub fn classify_type(
    type_info: &TypeInfo,
    all_impls: &[ImplInfo],
    all_functions: &[FunctionInfo],
    config: &BindingConfig,
) -> TypeClassification {
    let type_name = &type_info.name;

    if config.should_skip(type_name) {
        return TypeClassification::Skip;
    }
    if config.should_skip_module_path(&type_info.module_path) {
        return TypeClassification::Skip;
    }
    if config.force_include(type_name) {
        return TypeClassification::Primary;
    }

    if type_info.has_lifetimes {
        return TypeClassification::Skip;
    }

    if is_internal_type_name(type_name) {
        return TypeClassification::Skip;
    }

    if contains_private_module_segment(&type_info.module_path) {
        return TypeClassification::Skip;
    }

    let method_count = count_public_methods(type_name, all_impls);
    if method_count > 5 {
        return TypeClassification::Primary;
    }

    if appears_in_signatures(type_name, all_functions, all_impls) {
        return TypeClassification::Primary;
    }

    TypeClassification::Helper
}

pub fn classify_all(scanned: &ScannedApi, config: &BindingConfig) -> ClassifiedApi {
    let mut primary = Vec::new();
    let mut helper = Vec::new();
    let mut skipped = Vec::new();

    for type_info in &scanned.types {
        let classification = classify_type(type_info, &scanned.impls, &scanned.functions, config);
        match classification {
            TypeClassification::Primary => primary.push(type_info.clone()),
            TypeClassification::Helper => helper.push(type_info.clone()),
            TypeClassification::Skip => skipped.push(type_info.clone()),
        }
    }

    promote_helpers_used_by_primaries(&mut primary, &mut helper, &scanned.impls);

    let filtered_functions: Vec<FunctionInfo> = scanned
        .functions
        .iter()
        .filter(|f| !config.should_skip_module_path(&f.module_path))
        .cloned()
        .collect();

    ClassifiedApi {
        primary_types: primary,
        helper_types: helper,
        skipped_types: skipped,
        standalone_functions: filtered_functions,
    }
}

fn count_public_methods(type_name: &str, impls: &[ImplInfo]) -> usize {
    impls
        .iter()
        .filter(|imp| imp.target_type == type_name && imp.trait_name.is_none())
        .flat_map(|imp| &imp.methods)
        .filter(|m| m.is_public)
        .count()
}

fn appears_in_signatures(type_name: &str, functions: &[FunctionInfo], impls: &[ImplInfo]) -> bool {
    for func in functions {
        if signature_contains_type(&func.signature, type_name) {
            return true;
        }
    }

    for imp in impls {
        if imp.target_type == type_name {
            continue;
        }
        for method in &imp.methods {
            if method.is_public && signature_contains_type(&method.signature, type_name) {
                return true;
            }
        }
    }

    false
}

fn signature_contains_type(sig: &MethodSignature, type_name: &str) -> bool {
    for (_, ty) in &sig.inputs {
        if type_contains_name_syn(ty, type_name) {
            return true;
        }
    }
    if let Some(output) = &sig.output {
        if type_contains_name_syn(output, type_name) {
            return true;
        }
    }
    false
}

fn type_contains_name_syn(ty: &syn::Type, type_name: &str) -> bool {
    use quote::ToTokens;
    let ty_string = ty.to_token_stream().to_string();
    ty_string
        .split(&['<', '>', '(', ')', ',', ' ', '&', '*'][..])
        .any(|part| part == type_name)
}

fn promote_helpers_used_by_primaries(
    primary: &mut Vec<TypeInfo>,
    helper: &mut Vec<TypeInfo>,
    impls: &[ImplInfo],
) {
    let mut promoted = Vec::new();

    for primary_type in primary.iter() {
        for helper_type in helper.iter() {
            if promoted.contains(&helper_type.name) {
                continue;
            }

            if type_uses_other(&primary_type.name, &helper_type.name, impls) {
                promoted.push(helper_type.name.clone());
            }
        }
    }

    helper.retain(|h| {
        if promoted.contains(&h.name) {
            primary.push(h.clone());
            false
        } else {
            true
        }
    });
}

fn type_uses_other(source_type: &str, target_type: &str, impls: &[ImplInfo]) -> bool {
    for imp in impls {
        if imp.target_type != source_type {
            continue;
        }
        for method in &imp.methods {
            if !method.is_public {
                continue;
            }
            if signature_contains_type(&method.signature, target_type) {
                return true;
            }
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_internal_type_detection() {
        assert!(is_internal_type_name("CacheInternal"));
        assert!(is_internal_type_name("PrivateData"));
        assert!(is_internal_type_name("CachedExpression"));
        assert!(is_internal_type_name("RawBuffer"));

        assert!(is_internal_type_name("XData"));
        assert!(is_internal_type_name("ABCache"));
        assert!(is_internal_type_name("MathError"));
        assert!(is_internal_type_name("CoreError"));
        assert!(is_internal_type_name("TestResult"));

        assert!(!is_internal_type_name("SolverResult"));
        assert!(!is_internal_type_name("ParserResult"));
        assert!(!is_internal_type_name("SolverError"));
        assert!(!is_internal_type_name("ParserError"));

        assert!(!is_internal_type_name("Expression"));
        assert!(!is_internal_type_name("Commutativity"));
        assert!(!is_internal_type_name("StepByStepExplanation"));
        assert!(!is_internal_type_name("FunctionProperties"));
        assert!(!is_internal_type_name("Symbol"));

        assert!(is_internal_type_name("Zp"));
        assert!(is_internal_type_name("Number"));
        assert!(is_internal_type_name("Result"));
    }
}
