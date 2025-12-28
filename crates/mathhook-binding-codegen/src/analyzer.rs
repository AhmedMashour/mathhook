use crate::scanner::MethodInfo;
use crate::types::{AnalyzedMethod, MappedType};
use anyhow::Result;
use quote::ToTokens;
use std::collections::HashSet;
use syn::{
    GenericArgument, PathArguments, PathSegment, ReturnType, Type, TypeImplTrait, TypeParamBound,
    TypePath, TypeReference, TypeTraitObject, TypeTuple,
};

pub fn analyze() -> Result<()> {
    println!("Analyzing API patterns...");

    let example_types = vec![
        ("i32", "i64", "i64"),
        ("String", "String", "String"),
        ("Expression", "PyExpression", "JsExpression"),
    ];

    println!("\nExample type mappings:");
    for (rust, python, node) in example_types {
        let mapped = MappedType::Direct {
            rust_type: rust.to_string(),
            python_type: python.to_string(),
            node_type: node.to_string(),
        };
        println!("  {} -> Python: {}, Node: {}", rust, python, node);
        println!("    Supported: {}", mapped.is_supported());
        println!("    Complexity: {}", mapped.complexity());
    }

    Ok(())
}

#[derive(Clone, Default)]
pub struct TypeRegistry {
    wrapper_types: HashSet<String>,
}

impl TypeRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_types(types: impl IntoIterator<Item = String>) -> Self {
        Self {
            wrapper_types: types.into_iter().collect(),
        }
    }

    pub fn has_wrapper(&self, type_name: &str) -> bool {
        self.wrapper_types.contains(type_name)
    }

    pub fn add(&mut self, type_name: String) {
        self.wrapper_types.insert(type_name);
    }

    pub fn is_empty(&self) -> bool {
        self.wrapper_types.is_empty()
    }
}

#[derive(Clone)]
pub struct TypeContext {
    pub self_type: Option<String>,
    pub method_name: Option<String>,
    pub registry: TypeRegistry,
}

impl TypeContext {
    pub fn new(self_type: Option<String>) -> Self {
        Self {
            self_type,
            method_name: None,
            registry: TypeRegistry::new(),
        }
    }

    pub fn with_method(mut self, method_name: Option<String>) -> Self {
        self.method_name = method_name;
        self
    }

    pub fn with_registry(mut self, registry: TypeRegistry) -> Self {
        self.registry = registry;
        self
    }

    pub fn none() -> Self {
        Self {
            self_type: None,
            method_name: None,
            registry: TypeRegistry::new(),
        }
    }
}

pub fn analyze_type(ty: &Type) -> MappedType {
    analyze_type_with_context(ty, &TypeContext::none())
}

pub fn analyze_type_with_context(ty: &Type, ctx: &TypeContext) -> MappedType {
    match ty {
        Type::Path(type_path) => analyze_type_path(type_path, ctx),
        Type::Reference(type_ref) => analyze_reference(type_ref, ctx),
        Type::ImplTrait(impl_trait) => analyze_impl_trait(impl_trait, ctx),
        Type::TraitObject(trait_obj) => analyze_trait_object(trait_obj, ctx),
        Type::Tuple(type_tuple) => analyze_tuple(type_tuple, ctx),
        Type::Paren(paren) => analyze_type_with_context(&paren.elem, ctx),
        _ => MappedType::Unsupported {
            reason: "Unsupported type variant".to_string(),
            rust_type: ty.to_token_stream().to_string(),
        },
    }
}

fn is_generic_param(name: &str) -> bool {
    name.len() <= 2 && name.chars().all(|c| c.is_ascii_uppercase())
}

fn is_internal_type(name: &str, ctx: &TypeContext) -> bool {
    if !ctx.registry.is_empty() && ctx.registry.has_wrapper(name) {
        return false;
    }

    if name.contains("Internal") || name.contains("Private") {
        return true;
    }

    if name.starts_with("Cached") || name.starts_with("Raw") {
        return true;
    }

    if matches!(
        name,
        "BigInt"
            | "BigUint"
            | "BigRational"
            | "Ordering"
            | "PhantomData"
            | "Cow"
            | "Mutex"
            | "RwLock"
            | "Arc"
            | "Rc"
            | "Box"
            | "u128"
            | "i128"
            | "Zp"
            | "PolyZp"
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

fn analyze_type_path(type_path: &TypePath, ctx: &TypeContext) -> MappedType {
    let path = &type_path.path;
    let last_segment = match path.segments.last() {
        Some(seg) => seg,
        None => {
            return MappedType::Unsupported {
                reason: "Empty path".to_string(),
                rust_type: type_path.to_token_stream().to_string(),
            }
        }
    };

    let name = last_segment.ident.to_string();

    if name == "Self" {
        if let Some(self_type) = &ctx.self_type {
            if let Some((python_type, node_type)) = map_core_type(self_type, ctx) {
                return MappedType::Direct {
                    rust_type: "Self".to_string(),
                    python_type,
                    node_type,
                };
            } else {
                return MappedType::Direct {
                    rust_type: "Self".to_string(),
                    python_type: "Self".to_string(),
                    node_type: "Self".to_string(),
                };
            }
        }
        return MappedType::Direct {
            rust_type: "Self".to_string(),
            python_type: "Self".to_string(),
            node_type: "Self".to_string(),
        };
    }

    if is_generic_param(&name) {
        if let Some(mapped) = try_map_string_like_generic(&name, ctx) {
            return mapped;
        }
        return MappedType::Unsupported {
            reason: format!("Generic type parameter: {}", name),
            rust_type: type_path.to_token_stream().to_string(),
        };
    }

    match name.as_str() {
        "Result" => analyze_result(last_segment, type_path, ctx),
        "Option" => analyze_option(last_segment, type_path, ctx),
        "Vec" => analyze_vec(last_segment, type_path, ctx),
        "Box" | "Arc" | "Rc" => analyze_smart_pointer(last_segment, type_path, ctx),
        "HashMap" | "BTreeMap" => analyze_hashmap(last_segment, type_path, ctx),
        "HashSet" | "BTreeSet" => MappedType::Unsupported {
            reason: format!("Set type {} not yet supported", name),
            rust_type: type_path.to_token_stream().to_string(),
        },
        "Duration" | "Instant" | "SystemTime" => MappedType::Unsupported {
            reason: format!("Time type {} requires special handling", name),
            rust_type: type_path.to_token_stream().to_string(),
        },
        _ => {
            if is_internal_type(&name, ctx) {
                return MappedType::Unsupported {
                    reason: format!("Internal type: {}", name),
                    rust_type: name.clone(),
                };
            }

            if let Some((python_type, node_type)) = map_primitive(&name) {
                MappedType::Direct {
                    rust_type: name.clone(),
                    python_type: python_type.to_string(),
                    node_type: node_type.to_string(),
                }
            } else if let Some((python_type, node_type)) = map_core_type(&name, ctx) {
                MappedType::Direct {
                    rust_type: name.clone(),
                    python_type,
                    node_type,
                }
            } else {
                MappedType::Unsupported {
                    reason: format!("Unknown type without wrapper: {}", name),
                    rust_type: name.clone(),
                }
            }
        }
    }
}

fn try_map_string_like_generic(name: &str, ctx: &TypeContext) -> Option<MappedType> {
    let self_type = ctx.self_type.as_deref()?;
    let method_name = ctx.method_name.as_deref()?;

    if self_type == "Symbol" {
        match method_name {
            "new" | "scalar" | "matrix" | "operator" | "quaternion" => {
                if name == "S" {
                    return Some(MappedType::Direct {
                        rust_type: "String".to_string(),
                        python_type: "String".to_string(),
                        node_type: "String".to_string(),
                    });
                }
            }
            _ => {}
        }
    }

    if self_type == "Expression" && name == "S" {
        match method_name {
            "symbol" | "function" | "parse" => {
                return Some(MappedType::Direct {
                    rust_type: "String".to_string(),
                    python_type: "String".to_string(),
                    node_type: "String".to_string(),
                });
            }
            _ => {}
        }
    }

    None
}

fn analyze_result(segment: &PathSegment, type_path: &TypePath, ctx: &TypeContext) -> MappedType {
    match &segment.arguments {
        PathArguments::AngleBracketed(args) => {
            let mut iter = args.args.iter();
            let ok_type = match iter.next() {
                Some(GenericArgument::Type(ty)) => analyze_type_with_context(ty, ctx),
                _ => {
                    return MappedType::Unsupported {
                        reason: "Result missing Ok type".to_string(),
                        rust_type: type_path.to_token_stream().to_string(),
                    }
                }
            };

            let err_type = match iter.next() {
                Some(GenericArgument::Type(Type::Path(p))) => {
                    p.path.segments.last().map(|s| s.ident.to_string())
                }
                _ => None,
            }
            .unwrap_or_else(|| "Error".to_string());

            MappedType::Result {
                ok_type: Box::new(ok_type),
                err_type,
            }
        }
        _ => MappedType::Unsupported {
            reason: "Result without generic arguments".to_string(),
            rust_type: type_path.to_token_stream().to_string(),
        },
    }
}

fn analyze_option(segment: &PathSegment, type_path: &TypePath, ctx: &TypeContext) -> MappedType {
    match &segment.arguments {
        PathArguments::AngleBracketed(args) => {
            let inner_type = match args.args.first() {
                Some(GenericArgument::Type(ty)) => analyze_type_with_context(ty, ctx),
                _ => {
                    return MappedType::Unsupported {
                        reason: "Option missing inner type".to_string(),
                        rust_type: type_path.to_token_stream().to_string(),
                    }
                }
            };

            MappedType::Option {
                inner_type: Box::new(inner_type),
            }
        }
        _ => MappedType::Unsupported {
            reason: "Option without generic arguments".to_string(),
            rust_type: type_path.to_token_stream().to_string(),
        },
    }
}

fn analyze_vec(segment: &PathSegment, type_path: &TypePath, ctx: &TypeContext) -> MappedType {
    match &segment.arguments {
        PathArguments::AngleBracketed(args) => match args.args.first() {
            Some(GenericArgument::Type(ty)) => {
                let item_type = analyze_type_with_context(ty, ctx);
                MappedType::Collected {
                    item_type: Box::new(item_type),
                }
            }
            _ => MappedType::Unsupported {
                reason: "Vec missing item type".to_string(),
                rust_type: type_path.to_token_stream().to_string(),
            },
        },
        _ => MappedType::Unsupported {
            reason: "Vec without generic arguments".to_string(),
            rust_type: type_path.to_token_stream().to_string(),
        },
    }
}

fn analyze_hashmap(segment: &PathSegment, type_path: &TypePath, ctx: &TypeContext) -> MappedType {
    match &segment.arguments {
        PathArguments::AngleBracketed(args) => {
            let mut iter = args.args.iter();
            let key_type = match iter.next() {
                Some(GenericArgument::Type(ty)) => analyze_type_with_context(ty, ctx),
                _ => {
                    return MappedType::Unsupported {
                        reason: "HashMap missing key type".to_string(),
                        rust_type: type_path.to_token_stream().to_string(),
                    }
                }
            };

            let value_type = match iter.next() {
                Some(GenericArgument::Type(ty)) => analyze_type_with_context(ty, ctx),
                _ => {
                    return MappedType::Unsupported {
                        reason: "HashMap missing value type".to_string(),
                        rust_type: type_path.to_token_stream().to_string(),
                    }
                }
            };

            MappedType::HashMap {
                key_type: Box::new(key_type),
                value_type: Box::new(value_type),
            }
        }
        _ => MappedType::Unsupported {
            reason: "HashMap without generic arguments".to_string(),
            rust_type: type_path.to_token_stream().to_string(),
        },
    }
}

fn analyze_smart_pointer(
    segment: &PathSegment,
    type_path: &TypePath,
    ctx: &TypeContext,
) -> MappedType {
    match &segment.arguments {
        PathArguments::AngleBracketed(args) => match args.args.first() {
            Some(GenericArgument::Type(ty)) => analyze_type_with_context(ty, ctx),
            _ => MappedType::Unsupported {
                reason: "Smart pointer missing inner type".to_string(),
                rust_type: type_path.to_token_stream().to_string(),
            },
        },
        _ => MappedType::Unsupported {
            reason: "Smart pointer without generic arguments".to_string(),
            rust_type: type_path.to_token_stream().to_string(),
        },
    }
}

fn analyze_reference(type_ref: &TypeReference, ctx: &TypeContext) -> MappedType {
    let inner_type = analyze_type_with_context(&type_ref.elem, ctx);
    let is_mut = type_ref.mutability.is_some();

    MappedType::Reference {
        inner_type: Box::new(inner_type),
        is_mut,
    }
}

fn analyze_tuple(type_tuple: &TypeTuple, ctx: &TypeContext) -> MappedType {
    if type_tuple.elems.is_empty() {
        return MappedType::Direct {
            rust_type: "()".to_string(),
            python_type: "()".to_string(),
            node_type: "()".to_string(),
        };
    }

    let elements: Vec<MappedType> = type_tuple
        .elems
        .iter()
        .map(|ty| analyze_type_with_context(ty, ctx))
        .collect();

    MappedType::Tuple { elements }
}

fn analyze_impl_trait(impl_trait: &TypeImplTrait, ctx: &TypeContext) -> MappedType {
    for bound in &impl_trait.bounds {
        if let TypeParamBound::Trait(trait_bound) = bound {
            let path = &trait_bound.path;
            if let Some(segment) = path.segments.last() {
                let name = segment.ident.to_string();

                match name.as_str() {
                    "Iterator" => {
                        return analyze_iterator(segment, impl_trait, ctx);
                    }
                    "IntoIterator" => {
                        return analyze_into_iterator(segment, impl_trait, ctx);
                    }
                    "Fn" => {
                        return analyze_callback(segment, impl_trait, false, ctx);
                    }
                    "FnMut" => {
                        return analyze_callback(segment, impl_trait, true, ctx);
                    }
                    "FnOnce" => {
                        return MappedType::Unsupported {
                            reason: "FnOnce not supported (use Fn or FnMut)".to_string(),
                            rust_type: impl_trait.to_token_stream().to_string(),
                        };
                    }
                    _ => {}
                }
            }
        }
    }

    MappedType::Unsupported {
        reason: "Unsupported trait bound".to_string(),
        rust_type: impl_trait.to_token_stream().to_string(),
    }
}

fn analyze_trait_object(trait_obj: &TypeTraitObject, ctx: &TypeContext) -> MappedType {
    for bound in &trait_obj.bounds {
        if let TypeParamBound::Trait(trait_bound) = bound {
            let path = &trait_bound.path;
            if let Some(segment) = path.segments.last() {
                let name = segment.ident.to_string();

                match name.as_str() {
                    "Fn" => {
                        return analyze_callback(segment, trait_obj, false, ctx);
                    }
                    "FnMut" => {
                        return analyze_callback(segment, trait_obj, true, ctx);
                    }
                    "FnOnce" => {
                        return MappedType::Unsupported {
                            reason: "FnOnce not supported (use Fn or FnMut)".to_string(),
                            rust_type: trait_obj.to_token_stream().to_string(),
                        };
                    }
                    _ => {}
                }
            }
        }
    }

    MappedType::Unsupported {
        reason: "Unsupported trait object".to_string(),
        rust_type: trait_obj.to_token_stream().to_string(),
    }
}

fn analyze_iterator<T: ToTokens>(
    segment: &PathSegment,
    original: &T,
    ctx: &TypeContext,
) -> MappedType {
    match &segment.arguments {
        PathArguments::AngleBracketed(args) => {
            for arg in &args.args {
                if let GenericArgument::AssocType(assoc) = arg {
                    if assoc.ident == "Item" {
                        let item_type = analyze_type_with_context(&assoc.ty, ctx);
                        return MappedType::Collected {
                            item_type: Box::new(item_type),
                        };
                    }
                }
            }

            MappedType::Unsupported {
                reason: "Iterator missing Item type".to_string(),
                rust_type: original.to_token_stream().to_string(),
            }
        }
        _ => MappedType::Unsupported {
            reason: "Iterator without generic arguments".to_string(),
            rust_type: original.to_token_stream().to_string(),
        },
    }
}

fn analyze_into_iterator<T: ToTokens>(
    segment: &PathSegment,
    original: &T,
    ctx: &TypeContext,
) -> MappedType {
    match &segment.arguments {
        PathArguments::AngleBracketed(args) => {
            for arg in &args.args {
                if let GenericArgument::AssocType(assoc) = arg {
                    if assoc.ident == "Item" {
                        let item_type = analyze_type_with_context(&assoc.ty, ctx);
                        return MappedType::Collected {
                            item_type: Box::new(item_type),
                        };
                    }
                }
            }

            MappedType::Unsupported {
                reason: "IntoIterator missing Item type".to_string(),
                rust_type: original.to_token_stream().to_string(),
            }
        }
        _ => MappedType::Unsupported {
            reason: "IntoIterator without generic arguments".to_string(),
            rust_type: original.to_token_stream().to_string(),
        },
    }
}

fn analyze_callback<T: ToTokens>(
    segment: &PathSegment,
    original: &T,
    is_mut: bool,
    ctx: &TypeContext,
) -> MappedType {
    match &segment.arguments {
        PathArguments::Parenthesized(args) => {
            let arg_types = args
                .inputs
                .iter()
                .map(|ty| analyze_type_with_context(ty, ctx))
                .collect::<Vec<_>>();

            let return_type = match &args.output {
                ReturnType::Type(_, ty) => analyze_type_with_context(ty, ctx),
                ReturnType::Default => MappedType::Direct {
                    rust_type: "()".to_string(),
                    python_type: "()".to_string(),
                    node_type: "()".to_string(),
                },
            };

            MappedType::Callback {
                arg_types,
                return_type: Box::new(return_type),
                is_mut,
            }
        }
        _ => MappedType::Unsupported {
            reason: "Fn without parenthesized arguments".to_string(),
            rust_type: original.to_token_stream().to_string(),
        },
    }
}

fn map_primitive(name: &str) -> Option<(&str, &str)> {
    match name {
        "i8" | "i16" | "i32" | "isize" => Some(("i64", "i64")),
        "i64" => Some(("i64", "i64")),
        "u8" | "u16" | "u32" | "usize" => Some(("i64", "i64")),
        "u64" => Some(("i64", "i64")),
        "f32" => Some(("f64", "f64")),
        "f64" => Some(("f64", "f64")),
        "bool" => Some(("bool", "bool")),
        "String" | "str" => Some(("String", "String")),
        "()" => Some(("()", "()")),
        _ => None,
    }
}

fn map_core_type(name: &str, ctx: &TypeContext) -> Option<(String, String)> {
    if is_internal_type(name, ctx) {
        return None;
    }

    if !ctx.registry.is_empty() {
        if ctx.registry.has_wrapper(name) {
            return Some((format!("Py{}", name), format!("Js{}", name)));
        }
        return None;
    }

    Some((format!("Py{}", name), format!("Js{}", name)))
}

pub fn analyze_method(method: &MethodInfo) -> AnalyzedMethod {
    analyze_method_with_context(method, None)
}

fn is_mut_self_input(name: &str, ty: &MappedType) -> bool {
    // Detect &mut self
    let is_mut_ref = (name == "self" || name == "self_mut")
        && matches!(ty, MappedType::Reference { is_mut: true, .. });

    // Detect mut self (owned, consuming - builder pattern)
    // Check rust_type since python_type gets resolved to PyTypeName
    let is_consuming_self = name == "self_mut"
        && matches!(ty, MappedType::Direct { rust_type, .. } if rust_type == "Self");

    is_mut_ref || is_consuming_self
}

pub fn analyze_method_with_context(
    method: &MethodInfo,
    self_type: Option<String>,
) -> AnalyzedMethod {
    analyze_method_with_registry(method, self_type, TypeRegistry::new())
}

pub fn analyze_method_with_registry(
    method: &MethodInfo,
    self_type: Option<String>,
    registry: TypeRegistry,
) -> AnalyzedMethod {
    let ctx = TypeContext::new(self_type.clone())
        .with_method(Some(method.name.clone()))
        .with_registry(registry);

    let inputs = method
        .signature
        .inputs
        .iter()
        .map(|(name, ty)| (name.clone(), analyze_type_with_context(ty, &ctx)))
        .collect::<Vec<_>>();

    let output = method
        .signature
        .output
        .as_ref()
        .map(|ty| analyze_type_with_context(ty, &ctx))
        .unwrap_or_else(|| MappedType::Direct {
            rust_type: "()".to_string(),
            python_type: "()".to_string(),
            node_type: "()".to_string(),
        });

    let requires_mut_self = inputs.iter().any(|(name, ty)| is_mut_self_input(name, ty));

    let has_mutable_non_self_input = inputs.iter().any(|(name, ty)| {
        name != "self"
            && name != "self_mut"
            && matches!(ty, MappedType::Reference { is_mut: true, .. })
    });

    let all_inputs_supported = inputs.iter().all(|(_, ty)| ty.is_supported());
    let output_supported = output.is_supported();
    let is_supported = !has_mutable_non_self_input && all_inputs_supported && output_supported;

    AnalyzedMethod {
        name: method.name.clone(),
        original_name: None,
        inputs,
        output,
        is_supported,
        impl_type: self_type,
        requires_mut_self,
        doc_comment: method.doc_comment.clone(),
        skip_binding: method.skip_binding,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_internal_type_detection() {
        let ctx = TypeContext::none();

        assert!(is_internal_type("CacheInternal", &ctx));
        assert!(is_internal_type("PrivateData", &ctx));
        assert!(is_internal_type("CachedExpression", &ctx));
        assert!(is_internal_type("RawBuffer", &ctx));

        assert!(is_internal_type("XData", &ctx));
        assert!(is_internal_type("ABCache", &ctx));
        assert!(is_internal_type("MathError", &ctx));
        assert!(is_internal_type("CoreError", &ctx));
        assert!(is_internal_type("TestResult", &ctx));

        assert!(!is_internal_type("SolverResult", &ctx));
        assert!(!is_internal_type("ParserResult", &ctx));
        assert!(!is_internal_type("SolverError", &ctx));
        assert!(!is_internal_type("ParserError", &ctx));

        assert!(!is_internal_type("Expression", &ctx));
        assert!(!is_internal_type("Commutativity", &ctx));
        assert!(!is_internal_type("StepByStepExplanation", &ctx));
        assert!(!is_internal_type("FunctionProperties", &ctx));
        assert!(!is_internal_type("Symbol", &ctx));

        assert!(is_internal_type("BigInt", &ctx));
        assert!(is_internal_type("Mutex", &ctx));
        assert!(is_internal_type("Result", &ctx));
    }

    #[test]
    fn test_registry_overrides_internal() {
        let mut registry = TypeRegistry::new();
        registry.add("SolverResult".to_string());
        let ctx = TypeContext::none().with_registry(registry);

        assert!(!is_internal_type("SolverResult", &ctx));
    }

    #[test]
    fn test_primitive_mapping() {
        assert_eq!(map_primitive("bool"), Some(("bool", "bool")));
        assert_eq!(map_primitive("i32"), Some(("i64", "i64")));
        assert_eq!(map_primitive("f64"), Some(("f64", "f64")));
        assert_eq!(map_primitive("String"), Some(("String", "String")));
        assert_eq!(map_primitive("()"), Some(("()", "()")));
    }
}
