use crate::trait_analyzer::SupportedTrait;
use crate::types::{BindabilityRegistry, TypeTraits};
use anyhow::{Context, Result};
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};
use syn::{
    visit::Visit, File, ItemEnum, ItemFn, ItemImpl, ItemMod, ItemStruct, ItemTrait, ItemUse,
    Visibility,
};

#[derive(Debug, Default)]
pub struct ModuleVisibilityGraph {
    pub public_modules: HashSet<String>,
    pub public_files: HashSet<PathBuf>,
}

impl ModuleVisibilityGraph {
    pub fn build(crate_root: &Path) -> Result<Self> {
        let mut graph = Self::default();
        let src_dir = crate_root.join("src");
        let lib_rs = src_dir.join("lib.rs");

        if !lib_rs.exists() {
            anyhow::bail!("lib.rs not found at {}", lib_rs.display());
        }

        graph.public_modules.insert(String::new());
        graph.public_files.insert(lib_rs.clone());

        graph.discover_public_modules(&src_dir, &lib_rs, "")?;

        Ok(graph)
    }

    fn discover_public_modules(
        &mut self,
        src_dir: &Path,
        file_path: &Path,
        current_module: &str,
    ) -> Result<()> {
        let content = fs::read_to_string(file_path)
            .with_context(|| format!("Failed to read {}", file_path.display()))?;

        let syntax_tree: File = syn::parse_file(&content)
            .with_context(|| format!("Failed to parse {}", file_path.display()))?;

        for item in &syntax_tree.items {
            if let syn::Item::Mod(item_mod) = item {
                self.process_module(src_dir, file_path, current_module, item_mod)?;
            }
        }

        Ok(())
    }

    fn process_module(
        &mut self,
        src_dir: &Path,
        current_file: &Path,
        current_module: &str,
        item_mod: &ItemMod,
    ) -> Result<()> {
        if !matches!(item_mod.vis, Visibility::Public(_)) {
            return Ok(());
        }

        if has_conditional_cfg(&item_mod.attrs) {
            return Ok(());
        }

        let mod_name = item_mod.ident.to_string();
        let full_module_path = if current_module.is_empty() {
            mod_name.clone()
        } else {
            format!("{}::{}", current_module, mod_name)
        };

        self.public_modules.insert(full_module_path.clone());

        if item_mod.content.is_some() {
            return Ok(());
        }

        let parent_dir = current_file.parent().unwrap_or(src_dir);
        let current_file_name = current_file.file_stem().unwrap_or_default();
        let is_mod_rs = current_file_name == "mod";

        let submodule_dir = if is_mod_rs {
            parent_dir.to_path_buf()
        } else {
            parent_dir.join(current_file_name)
        };

        let mod_file = submodule_dir.join(format!("{}.rs", mod_name));
        let mod_dir_file = submodule_dir.join(&mod_name).join("mod.rs");
        let sibling_file = parent_dir.join(format!("{}.rs", mod_name));

        let module_file = if mod_file.exists() {
            mod_file
        } else if mod_dir_file.exists() {
            mod_dir_file
        } else if sibling_file.exists() {
            sibling_file
        } else {
            return Ok(());
        };

        self.public_files.insert(module_file.clone());

        self.discover_public_modules(src_dir, &module_file, &full_module_path)?;

        Ok(())
    }

    pub fn is_public(&self, module_path: &str) -> bool {
        if module_path.is_empty() {
            return true;
        }
        self.public_modules.contains(module_path)
    }

    #[allow(dead_code)]
    pub fn is_public_file(&self, file_path: &Path) -> bool {
        self.public_files.contains(file_path)
    }
}

fn has_conditional_cfg(attrs: &[syn::Attribute]) -> bool {
    attrs.iter().any(|attr| {
        if !attr.path().is_ident("cfg") {
            return false;
        }
        if let Ok(list) = attr.meta.require_list() {
            let tokens = list.tokens.to_string();
            if tokens.contains("test") {
                return true;
            }
            if tokens.contains("feature") {
                return true;
            }
            if tokens.contains("target_") || tokens.contains("windows") || tokens.contains("unix") {
                return true;
            }
        }
        false
    })
}

fn has_no_binding_directive(doc_comment: &Option<String>) -> bool {
    doc_comment.as_ref().is_some_and(|doc| {
        doc.lines().any(|line| {
            let trimmed = line.trim();
            trimmed == "@no-binding" || trimmed.starts_with("@no-binding ")
        })
    })
}

pub fn clean_doc_for_binding(doc: &str) -> String {
    doc.lines()
        .filter(|line| {
            let trimmed = line.trim();
            trimmed != "@no-binding" && !trimmed.starts_with("@no-binding ")
        })
        .collect::<Vec<_>>()
        .join("\n")
}

#[derive(Debug, Clone)]
pub enum TypeKind {
    Struct,
    Enum,
}

#[derive(Clone)]
pub struct FieldInfo {
    pub name: String,
    pub ty: syn::Type,
    pub is_public: bool,
}

impl std::fmt::Debug for FieldInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FieldInfo")
            .field("name", &self.name)
            .field("ty", &"...")
            .field("is_public", &self.is_public)
            .finish()
    }
}

#[derive(Debug, Clone)]
pub struct EnumVariant {
    pub name: String,
    pub fields: Vec<FieldInfo>,
    pub doc_comment: Option<String>,
}

#[derive(Debug, Clone)]
pub struct TypeInfo {
    pub name: String,
    pub module_path: String,
    pub source_file: PathBuf,
    pub kind: TypeKind,
    pub is_public: bool,
    pub has_lifetimes: bool,
    pub doc_comment: Option<String>,
    pub fields: Vec<FieldInfo>,
    pub enum_variants: Vec<EnumVariant>,
    pub derived_traits: Vec<String>,
    pub is_cfg_gated: bool,
    pub skip_binding: bool,
}

impl TypeInfo {
    pub fn is_simple_enum(&self) -> bool {
        matches!(self.kind, TypeKind::Enum)
            && !self.enum_variants.is_empty()
            && self.enum_variants.iter().all(|v| v.fields.is_empty())
    }

    pub fn has_clone(&self) -> bool {
        self.derived_traits.iter().any(|t| t == "Clone")
    }

    pub fn has_display(&self) -> bool {
        self.derived_traits.iter().any(|t| t == "Display")
    }

    pub fn has_debug(&self) -> bool {
        self.derived_traits.iter().any(|t| t == "Debug")
    }
}

#[derive(Debug, Clone)]
pub struct FunctionInfo {
    pub name: String,
    pub module_path: String,
    pub source_file: PathBuf,
    pub signature: MethodSignature,
    pub doc_comment: Option<String>,
    pub skip_binding: bool,
}

#[derive(Debug, Clone)]
pub struct TraitDefinition {
    pub name: String,
    pub module_path: String,
    pub source_file: PathBuf,
    pub is_public: bool,
    pub skip_binding: bool,
}

impl TraitDefinition {
    pub fn full_import_path(&self) -> String {
        if self.module_path.is_empty() {
            format!("mathhook_core::{}", self.name)
        } else {
            format!("mathhook_core::{}::{}", self.module_path, self.name)
        }
    }
}

#[derive(Debug, Clone)]
pub struct ReexportInfo {
    pub item_name: String,
    pub reexport_module: String,
}

#[derive(Debug, Clone)]
pub struct ImplInfo {
    pub target_type: String,
    pub trait_name: Option<String>,
    pub trait_path: Option<String>,
    pub methods: Vec<MethodInfo>,
    pub source_file: PathBuf,
    pub module_path: String,
    pub skip_binding: bool,
    pub associated_output_type: Option<String>,
}

const STD_TRAIT_PREFIXES: &[&str] = &[
    "std::",
    "core::",
    "alloc::",
    "fmt::",
    "ops::",
    "cmp::",
    "hash::",
    "iter::",
    "convert::",
    "default::",
    "clone::",
    "marker::",
    "borrow::",
    "any::",
    "error::",
    "io::",
    "num::",
    "str::",
    "string::",
    "vec::",
    "collections::",
    "serde::",
];
const STD_TRAIT_NAMES: &[&str] = &[
    "From",
    "Into",
    "AsRef",
    "AsMut",
    "TryFrom",
    "TryInto",
    "Borrow",
    "BorrowMut",
    "ToOwned",
    "ToString",
    "Iterator",
    "IntoIterator",
    "Extend",
    "Drop",
    "Deref",
    "DerefMut",
    "Index",
    "IndexMut",
    "Fn",
    "FnMut",
    "FnOnce",
    "Send",
    "Sync",
    "Sized",
    "Copy",
    "Ord",
    "Eq",
];

impl ImplInfo {
    pub fn is_mathhook_core_trait(&self) -> bool {
        if self.trait_name.is_none() {
            return false;
        }

        if let Some(ref path) = self.trait_path {
            if path.starts_with("mathhook_core::") || path.starts_with("crate::") {
                return true;
            }

            for prefix in STD_TRAIT_PREFIXES {
                if path.starts_with(prefix) {
                    return false;
                }
            }

            if !path.contains("::") {
                if let Some(ref name) = self.trait_name {
                    if STD_TRAIT_NAMES.contains(&name.as_str()) {
                        return false;
                    }
                }
                return !self.is_std_bindable_trait();
            }
        }

        false
    }

    pub fn is_std_bindable_trait(&self) -> bool {
        self.trait_name
            .as_deref()
            .is_some_and(|name| SupportedTrait::from_name(name).is_some())
    }

    pub fn is_bindable_trait(&self) -> bool {
        self.is_std_bindable_trait() || self.is_mathhook_core_trait()
    }

    pub fn is_clone_impl(&self) -> bool {
        self.trait_name.as_deref() == Some("Clone")
    }

    pub fn is_display_impl(&self) -> bool {
        self.trait_name.as_deref() == Some("Display")
    }

    pub fn is_debug_impl(&self) -> bool {
        self.trait_name.as_deref() == Some("Debug")
    }
}

#[derive(Debug, Clone)]
pub struct MethodInfo {
    pub name: String,
    pub signature: MethodSignature,
    pub is_public: bool,
    pub doc_comment: Option<String>,
    pub skip_binding: bool,
}

#[derive(Clone)]
pub struct MethodSignature {
    pub inputs: Vec<(String, syn::Type)>,
    pub output: Option<syn::Type>,
    pub is_async: bool,
}

impl std::fmt::Debug for MethodSignature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MethodSignature")
            .field("inputs", &format!("{} args", self.inputs.len()))
            .field(
                "output",
                &if self.output.is_some() {
                    "Some(..)"
                } else {
                    "None"
                },
            )
            .field("is_async", &self.is_async)
            .finish()
    }
}

#[derive(Debug, Default)]
pub struct ScannedApi {
    pub types: Vec<TypeInfo>,
    pub functions: Vec<FunctionInfo>,
    pub impls: Vec<ImplInfo>,
    pub trait_definitions: Vec<TraitDefinition>,
    pub reexports: Vec<ReexportInfo>,
}

impl ScannedApi {
    pub fn build_trait_path_map(&self) -> HashMap<String, String> {
        let mut map = HashMap::new();

        let mut reexport_map: HashMap<String, Vec<String>> = HashMap::new();
        for reexport in &self.reexports {
            reexport_map
                .entry(reexport.item_name.clone())
                .or_default()
                .push(reexport.reexport_module.clone());
        }

        for trait_def in &self.trait_definitions {
            if trait_def.is_public {
                let trait_name = &trait_def.name;

                if let Some(reexport_paths) = reexport_map.get(trait_name) {
                    let shortest_path = reexport_paths
                        .iter()
                        .min_by_key(|p| p.matches("::").count())
                        .cloned();

                    if let Some(path) = shortest_path {
                        let import_path = if path.is_empty() {
                            format!("mathhook_core::{}", trait_name)
                        } else {
                            format!("mathhook_core::{}::{}", path, trait_name)
                        };
                        map.insert(trait_name.clone(), import_path);
                        continue;
                    }
                }

                map.insert(trait_name.clone(), trait_def.full_import_path());
            }
        }
        map
    }
}

struct ApiVisitor {
    types: Vec<TypeInfo>,
    functions: Vec<FunctionInfo>,
    impls: Vec<ImplInfo>,
    trait_definitions: Vec<TraitDefinition>,
    reexports: Vec<ReexportInfo>,
    current_file: PathBuf,
    current_module: String,
}

impl ApiVisitor {
    fn new(file_path: PathBuf, module_path: String) -> Self {
        Self {
            types: Vec::new(),
            functions: Vec::new(),
            impls: Vec::new(),
            trait_definitions: Vec::new(),
            reexports: Vec::new(),
            current_file: file_path,
            current_module: module_path,
        }
    }

    fn is_public(vis: &Visibility) -> bool {
        matches!(vis, Visibility::Public(_))
    }

    fn extract_doc_comment(attrs: &[syn::Attribute]) -> Option<String> {
        let mut docs = Vec::new();
        for attr in attrs {
            if attr.path().is_ident("doc") {
                if let syn::Meta::NameValue(nv) = &attr.meta {
                    if let syn::Expr::Lit(expr_lit) = &nv.value {
                        if let syn::Lit::Str(lit_str) = &expr_lit.lit {
                            docs.push(lit_str.value().trim().to_string());
                        }
                    }
                }
            }
        }
        if docs.is_empty() {
            None
        } else {
            Some(docs.join("\n"))
        }
    }

    fn extract_derived_traits(attrs: &[syn::Attribute]) -> Vec<String> {
        let mut derived_traits = Vec::new();
        for attr in attrs {
            if attr.path().is_ident("derive") {
                if let syn::Meta::List(meta_list) = &attr.meta {
                    let tokens_str = meta_list.tokens.to_string();
                    for trait_name in tokens_str.split(',') {
                        let trait_name = trait_name.trim();
                        if !trait_name.is_empty() {
                            let simple_name =
                                trait_name.rsplit("::").next().unwrap_or(trait_name).trim();
                            if !simple_name.is_empty() {
                                derived_traits.push(simple_name.to_string());
                            }
                        }
                    }
                }
            }
        }
        derived_traits
    }

    fn extract_signature(sig: &syn::Signature) -> MethodSignature {
        let inputs = sig
            .inputs
            .iter()
            .map(|arg| match arg {
                syn::FnArg::Typed(pat_type) => {
                    let name = match &*pat_type.pat {
                        syn::Pat::Ident(ident) => ident.ident.to_string(),
                        _ => "arg".to_string(),
                    };
                    (name, (*pat_type.ty).clone())
                }
                syn::FnArg::Receiver(receiver) => {
                    let name = if receiver.mutability.is_some() {
                        "self_mut".to_string()
                    } else {
                        "self".to_string()
                    };
                    let self_ty: syn::Type = if receiver.reference.is_some() {
                        if receiver.mutability.is_some() {
                            syn::parse_quote!(&mut Self)
                        } else {
                            syn::parse_quote!(&Self)
                        }
                    } else {
                        syn::parse_quote!(Self)
                    };
                    (name, self_ty)
                }
            })
            .collect();

        let output = match &sig.output {
            syn::ReturnType::Default => None,
            syn::ReturnType::Type(_, ty) => Some((**ty).clone()),
        };

        MethodSignature {
            inputs,
            output,
            is_async: sig.asyncness.is_some(),
        }
    }

    fn extract_enum_variants(node: &ItemEnum) -> Vec<EnumVariant> {
        node.variants
            .iter()
            .map(|variant| {
                let fields = match &variant.fields {
                    syn::Fields::Named(named) => named
                        .named
                        .iter()
                        .map(|field| FieldInfo {
                            name: field.ident.as_ref().unwrap().to_string(),
                            ty: field.ty.clone(),
                            is_public: Self::is_public(&field.vis),
                        })
                        .collect(),
                    syn::Fields::Unnamed(unnamed) => unnamed
                        .unnamed
                        .iter()
                        .enumerate()
                        .map(|(i, field)| FieldInfo {
                            name: i.to_string(),
                            ty: field.ty.clone(),
                            is_public: Self::is_public(&field.vis),
                        })
                        .collect(),
                    syn::Fields::Unit => vec![],
                };

                EnumVariant {
                    name: variant.ident.to_string(),
                    fields,
                    doc_comment: Self::extract_doc_comment(&variant.attrs),
                }
            })
            .collect()
    }

    fn extract_trait_path(path: &syn::Path) -> String {
        use quote::ToTokens;
        path.segments
            .iter()
            .map(|seg| {
                let ident = seg.ident.to_string();
                match &seg.arguments {
                    syn::PathArguments::AngleBracketed(args) => {
                        let args_str = args.args.to_token_stream().to_string();
                        format!("{}<{}>", ident, args_str)
                    }
                    _ => ident,
                }
            })
            .collect::<Vec<_>>()
            .join("::")
    }

    fn extract_use_items(tree: &syn::UseTree, prefix: &str) -> Vec<String> {
        match tree {
            syn::UseTree::Path(path) => {
                let new_prefix = if prefix.is_empty() {
                    path.ident.to_string()
                } else {
                    format!("{}::{}", prefix, path.ident)
                };
                Self::extract_use_items(&path.tree, &new_prefix)
            }
            syn::UseTree::Name(name) => {
                vec![name.ident.to_string()]
            }
            syn::UseTree::Rename(rename) => {
                vec![rename.rename.to_string()]
            }
            syn::UseTree::Glob(_) => {
                vec![]
            }
            syn::UseTree::Group(group) => group
                .items
                .iter()
                .flat_map(|item| Self::extract_use_items(item, prefix))
                .collect(),
        }
    }
}

impl<'ast> Visit<'ast> for ApiVisitor {
    fn visit_item_struct(&mut self, node: &'ast ItemStruct) {
        let is_cfg_gated = has_conditional_cfg(&node.attrs);
        if is_cfg_gated {
            return;
        }
        let is_public = Self::is_public(&node.vis);
        if is_public {
            let has_lifetimes = node.generics.lifetimes().count() > 0;
            let doc_comment = Self::extract_doc_comment(&node.attrs);
            let skip_binding = has_no_binding_directive(&doc_comment);

            let fields = match &node.fields {
                syn::Fields::Named(named) => named
                    .named
                    .iter()
                    .filter_map(|field| {
                        let field_name = field.ident.as_ref()?.to_string();
                        let field_public = Self::is_public(&field.vis);
                        Some(FieldInfo {
                            name: field_name,
                            ty: field.ty.clone(),
                            is_public: field_public,
                        })
                    })
                    .collect(),
                _ => vec![],
            };

            let derived_traits = Self::extract_derived_traits(&node.attrs);

            self.types.push(TypeInfo {
                name: node.ident.to_string(),
                module_path: self.current_module.clone(),
                source_file: self.current_file.clone(),
                kind: TypeKind::Struct,
                is_public,
                has_lifetimes,
                doc_comment,
                fields,
                enum_variants: vec![],
                derived_traits,
                is_cfg_gated: false,
                skip_binding,
            });
        }
    }

    fn visit_item_enum(&mut self, node: &'ast ItemEnum) {
        let is_cfg_gated = has_conditional_cfg(&node.attrs);
        if is_cfg_gated {
            return;
        }
        let is_public = Self::is_public(&node.vis);
        if is_public {
            let has_lifetimes = node.generics.lifetimes().count() > 0;
            let variants = Self::extract_enum_variants(node);
            let derived_traits = Self::extract_derived_traits(&node.attrs);
            let doc_comment = Self::extract_doc_comment(&node.attrs);
            let skip_binding = has_no_binding_directive(&doc_comment);

            self.types.push(TypeInfo {
                name: node.ident.to_string(),
                module_path: self.current_module.clone(),
                source_file: self.current_file.clone(),
                kind: TypeKind::Enum,
                is_public,
                has_lifetimes,
                doc_comment,
                fields: vec![],
                enum_variants: variants,
                derived_traits,
                is_cfg_gated: false,
                skip_binding,
            });
        }
    }

    fn visit_item_fn(&mut self, node: &'ast ItemFn) {
        if has_conditional_cfg(&node.attrs) {
            return;
        }
        if Self::is_public(&node.vis) {
            let doc_comment = Self::extract_doc_comment(&node.attrs);
            let skip_binding = has_no_binding_directive(&doc_comment);

            self.functions.push(FunctionInfo {
                name: node.sig.ident.to_string(),
                module_path: self.current_module.clone(),
                source_file: self.current_file.clone(),
                signature: Self::extract_signature(&node.sig),
                doc_comment,
                skip_binding,
            });
        }
    }

    fn visit_item_trait(&mut self, node: &'ast ItemTrait) {
        if has_conditional_cfg(&node.attrs) {
            return;
        }
        let is_public = Self::is_public(&node.vis);
        let doc_comment = Self::extract_doc_comment(&node.attrs);
        let skip_binding = has_no_binding_directive(&doc_comment);

        self.trait_definitions.push(TraitDefinition {
            name: node.ident.to_string(),
            module_path: self.current_module.clone(),
            source_file: self.current_file.clone(),
            is_public,
            skip_binding,
        });
    }

    fn visit_item_use(&mut self, node: &'ast ItemUse) {
        if !Self::is_public(&node.vis) {
            return;
        }

        let items = Self::extract_use_items(&node.tree, "");
        for item_name in items {
            self.reexports.push(ReexportInfo {
                item_name,
                reexport_module: self.current_module.clone(),
            });
        }
    }

    fn visit_item_mod(&mut self, node: &'ast ItemMod) {
        if has_conditional_cfg(&node.attrs) {
            return;
        }

        if !Self::is_public(&node.vis) {
            return;
        }

        if let Some((_, items)) = &node.content {
            let mod_name = node.ident.to_string();
            let nested_module = if self.current_module.is_empty() {
                mod_name
            } else {
                format!("{}::{}", self.current_module, mod_name)
            };

            let old_module = std::mem::replace(&mut self.current_module, nested_module);

            for item in items {
                match item {
                    syn::Item::Use(use_item) => {
                        if Self::is_public(&use_item.vis) {
                            let extracted = Self::extract_use_items(&use_item.tree, "");
                            for item_name in extracted {
                                self.reexports.push(ReexportInfo {
                                    item_name,
                                    reexport_module: self.current_module.clone(),
                                });
                            }
                        }
                    }
                    syn::Item::Mod(nested_mod) => {
                        self.visit_item_mod(nested_mod);
                    }
                    _ => {}
                }
            }

            self.current_module = old_module;
        }
    }

    fn visit_item_impl(&mut self, node: &'ast ItemImpl) {
        if has_conditional_cfg(&node.attrs) {
            return;
        }

        let doc_comment = Self::extract_doc_comment(&node.attrs);
        let impl_skip_binding = has_no_binding_directive(&doc_comment);

        let target_type = match &*node.self_ty {
            syn::Type::Path(type_path) => {
                type_path.path.segments.last().map(|s| s.ident.to_string())
            }
            _ => None,
        };

        let (trait_name, trait_path) = node
            .trait_
            .as_ref()
            .map(|(_, path, _)| {
                let name = path.segments.last().map(|s| s.ident.to_string());
                let full_path = Self::extract_trait_path(path);
                (name, Some(full_path))
            })
            .unwrap_or((None, None));

        if let Some(target) = target_type {
            let mut methods = Vec::new();
            let is_trait_impl = trait_name.is_some();

            for item in &node.items {
                if let syn::ImplItem::Fn(method) = item {
                    let is_public = is_trait_impl || Self::is_public(&method.vis);
                    let method_doc = Self::extract_doc_comment(&method.attrs);
                    let method_skip = impl_skip_binding || has_no_binding_directive(&method_doc);

                    methods.push(MethodInfo {
                        name: method.sig.ident.to_string(),
                        signature: Self::extract_signature(&method.sig),
                        is_public,
                        doc_comment: method_doc,
                        skip_binding: method_skip,
                    });
                }
            }

            self.impls.push(ImplInfo {
                target_type: target,
                trait_name,
                trait_path,
                methods,
                source_file: self.current_file.clone(),
                module_path: self.current_module.clone(),
                skip_binding: impl_skip_binding,
                associated_output_type: None,
            });
        }
    }
}

struct ImplOnlyVisitor {
    impls: Vec<ImplInfo>,
    trait_definitions: Vec<TraitDefinition>,
    current_file: PathBuf,
    current_module: String,
}

impl ImplOnlyVisitor {
    fn new(file_path: PathBuf, module_path: String) -> Self {
        Self {
            impls: Vec::new(),
            trait_definitions: Vec::new(),
            current_file: file_path,
            current_module: module_path,
        }
    }
}

impl<'ast> Visit<'ast> for ImplOnlyVisitor {
    fn visit_item_trait(&mut self, node: &'ast ItemTrait) {
        if has_conditional_cfg(&node.attrs) {
            return;
        }
        let is_public = matches!(node.vis, Visibility::Public(_));
        let doc_comment = ApiVisitor::extract_doc_comment(&node.attrs);
        let skip_binding = has_no_binding_directive(&doc_comment);

        self.trait_definitions.push(TraitDefinition {
            name: node.ident.to_string(),
            module_path: self.current_module.clone(),
            source_file: self.current_file.clone(),
            is_public,
            skip_binding,
        });
    }

    fn visit_item_impl(&mut self, node: &'ast ItemImpl) {
        if has_conditional_cfg(&node.attrs) {
            return;
        }

        let doc_comment = ApiVisitor::extract_doc_comment(&node.attrs);
        let impl_skip_binding = has_no_binding_directive(&doc_comment);

        let target_type = match &*node.self_ty {
            syn::Type::Path(type_path) => {
                type_path.path.segments.last().map(|s| s.ident.to_string())
            }
            _ => None,
        };

        let (trait_name, trait_path) = node
            .trait_
            .as_ref()
            .map(|(_, path, _)| {
                let name = path.segments.last().map(|s| s.ident.to_string());
                let full_path = ApiVisitor::extract_trait_path(path);
                (name, Some(full_path))
            })
            .unwrap_or((None, None));

        if let Some(target) = target_type {
            let mut methods = Vec::new();
            let mut associated_output_type = None;
            let is_trait_impl = trait_name.is_some();

            for item in &node.items {
                match item {
                    syn::ImplItem::Fn(method) => {
                        let is_public =
                            is_trait_impl || matches!(method.vis, Visibility::Public(_));
                        let method_doc = ApiVisitor::extract_doc_comment(&method.attrs);
                        let method_skip =
                            impl_skip_binding || has_no_binding_directive(&method_doc);

                        methods.push(MethodInfo {
                            name: method.sig.ident.to_string(),
                            signature: ApiVisitor::extract_signature(&method.sig),
                            is_public,
                            doc_comment: method_doc,
                            skip_binding: method_skip,
                        });
                    }
                    syn::ImplItem::Type(assoc_type) if assoc_type.ident == "Output" => {
                        associated_output_type = Some({
                            use quote::ToTokens;
                            assoc_type.ty.to_token_stream().to_string()
                        });
                    }
                    _ => {}
                }
            }

            self.impls.push(ImplInfo {
                target_type: target,
                trait_name,
                trait_path,
                methods,
                source_file: self.current_file.clone(),
                module_path: self.current_module.clone(),
                skip_binding: impl_skip_binding,
                associated_output_type,
            });
        }
    }
}

fn compute_module_path(crate_root: &Path, file_path: &Path) -> Result<String> {
    let relative = file_path
        .strip_prefix(crate_root.join("src"))
        .context("File not in src directory")?;

    let mut components: Vec<String> = relative
        .parent()
        .unwrap_or(Path::new(""))
        .components()
        .filter_map(|c| {
            if let std::path::Component::Normal(s) = c {
                s.to_str().map(|s| s.to_string())
            } else {
                None
            }
        })
        .collect();

    if let Some(stem) = file_path.file_stem().and_then(|s| s.to_str()) {
        if stem != "lib" && stem != "mod" {
            components.push(stem.to_string());
        }
    }

    Ok(components.join("::"))
}

fn should_skip_file(path: &Path) -> bool {
    let file_name = path.file_name().and_then(|s| s.to_str()).unwrap_or("");

    if file_name.starts_with("test_") || file_name.ends_with("_test.rs") {
        return true;
    }

    if let Some(parent) = path.parent() {
        if parent.ends_with("tests") || parent.ends_with("benches") {
            return true;
        }
    }

    false
}

fn scan_file(crate_root: &Path, file_path: &Path) -> Result<ApiVisitor> {
    let content = fs::read_to_string(file_path)
        .with_context(|| format!("Failed to read {}", file_path.display()))?;

    let syntax_tree: File = syn::parse_file(&content)
        .with_context(|| format!("Failed to parse {}", file_path.display()))?;

    let has_cfg_test = syntax_tree.attrs.iter().any(|attr| {
        attr.path().is_ident("cfg")
            && attr
                .meta
                .require_list()
                .ok()
                .and_then(|list| list.tokens.to_string().contains("test").then_some(true))
                .unwrap_or(false)
    });

    if has_cfg_test {
        return Ok(ApiVisitor::new(file_path.to_path_buf(), String::new()));
    }

    let module_path = compute_module_path(crate_root, file_path)?;
    let mut visitor = ApiVisitor::new(file_path.to_path_buf(), module_path);

    visitor.visit_file(&syntax_tree);

    Ok(visitor)
}

fn scan_file_impls_only(crate_root: &Path, file_path: &Path) -> Result<ImplOnlyVisitor> {
    let content = fs::read_to_string(file_path)
        .with_context(|| format!("Failed to read {}", file_path.display()))?;

    let syntax_tree: File = syn::parse_file(&content)
        .with_context(|| format!("Failed to parse {}", file_path.display()))?;

    let has_cfg_test = syntax_tree.attrs.iter().any(|attr| {
        attr.path().is_ident("cfg")
            && attr
                .meta
                .require_list()
                .ok()
                .and_then(|list| list.tokens.to_string().contains("test").then_some(true))
                .unwrap_or(false)
    });

    if has_cfg_test {
        return Ok(ImplOnlyVisitor::new(file_path.to_path_buf(), String::new()));
    }

    let module_path = compute_module_path(crate_root, file_path)?;
    let mut visitor = ImplOnlyVisitor::new(file_path.to_path_buf(), module_path);

    visitor.visit_file(&syntax_tree);

    Ok(visitor)
}

pub fn scan_crate(crate_path: &Path) -> Result<ScannedApi> {
    let src_dir = crate_path.join("src");
    if !src_dir.exists() {
        anyhow::bail!("Source directory not found: {}", src_dir.display());
    }

    println!("Building module visibility graph...");
    let visibility_graph = ModuleVisibilityGraph::build(crate_path)?;
    println!(
        "Found {} public modules",
        visibility_graph.public_modules.len()
    );

    let mut all_types = Vec::new();
    let mut all_functions = Vec::new();
    let mut all_impls = Vec::new();
    let mut all_trait_definitions = Vec::new();
    let mut all_reexports = Vec::new();
    let mut files_scanned = 0;
    let mut files_skipped_private = 0;
    let mut files_skipped_test = 0;
    let mut private_files_scanned_for_impls = 0;

    for entry in walkdir::WalkDir::new(&src_dir)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) != Some("rs") {
            continue;
        }

        if should_skip_file(path) {
            files_skipped_test += 1;
            continue;
        }

        let module_path = compute_module_path(crate_path, path).unwrap_or_default();
        let is_public_module = module_path.is_empty() || visibility_graph.is_public(&module_path);

        if is_public_module {
            match scan_file(crate_path, path) {
                Ok(visitor) => {
                    all_types.extend(visitor.types);
                    all_functions.extend(visitor.functions);
                    all_impls.extend(visitor.impls);
                    all_trait_definitions.extend(visitor.trait_definitions);
                    all_reexports.extend(visitor.reexports);
                    files_scanned += 1;
                }
                Err(e) => {
                    eprintln!("Warning: Failed to scan {}: {}", path.display(), e);
                }
            }
        } else {
            match scan_file_impls_only(crate_path, path) {
                Ok(visitor) => {
                    if !visitor.impls.is_empty() || !visitor.trait_definitions.is_empty() {
                        all_impls.extend(visitor.impls);
                        all_trait_definitions.extend(visitor.trait_definitions);
                        private_files_scanned_for_impls += 1;
                    } else {
                        files_skipped_private += 1;
                    }
                }
                Err(e) => {
                    eprintln!("Warning: Failed to scan {}: {}", path.display(), e);
                    files_skipped_private += 1;
                }
            }
        }
    }

    println!(
        "Scanned {} files (skipped {} private, {} test)",
        files_scanned, files_skipped_private, files_skipped_test
    );
    if private_files_scanned_for_impls > 0 {
        println!(
            "Also scanned {} private files for impl blocks",
            private_files_scanned_for_impls
        );
    }
    println!(
        "Found {} types, {} functions, {} impl blocks",
        all_types.len(),
        all_functions.len(),
        all_impls.len()
    );

    Ok(ScannedApi {
        types: all_types,
        functions: all_functions,
        impls: all_impls,
        trait_definitions: all_trait_definitions,
        reexports: all_reexports,
    })
}

impl ScannedApi {
    pub fn total_items(&self) -> usize {
        self.types.len() + self.functions.len() + self.impls.len()
    }

    pub fn public_types(&self) -> impl Iterator<Item = &TypeInfo> {
        self.types.iter().filter(|t| t.is_public)
    }

    pub fn methods_for_type(&self, type_name: &str) -> Vec<&MethodInfo> {
        self.impls
            .iter()
            .filter(|impl_block| impl_block.target_type == type_name)
            .flat_map(|impl_block| &impl_block.methods)
            .collect()
    }

    pub fn trait_impls_for_type(&self, type_name: &str) -> Vec<(&str, &[MethodInfo])> {
        self.impls
            .iter()
            .filter(|impl_block| {
                impl_block.target_type == type_name && impl_block.trait_name.is_some()
            })
            .map(|impl_block| {
                (
                    impl_block.trait_name.as_ref().unwrap().as_str(),
                    impl_block.methods.as_slice(),
                )
            })
            .collect()
    }

    pub fn bindable_trait_impls_for_type(&self, type_name: &str) -> Vec<&ImplInfo> {
        self.impls
            .iter()
            .filter(|impl_block| {
                impl_block.target_type == type_name && impl_block.is_bindable_trait()
            })
            .collect()
    }

    pub fn stats(&self) -> ScanStats {
        let mut type_counts = HashMap::new();
        for type_info in &self.types {
            *type_counts.entry(&type_info.module_path).or_insert(0) += 1;
        }

        let total_methods: usize = self.impls.iter().map(|i| i.methods.len()).sum();
        let public_methods: usize = self
            .impls
            .iter()
            .flat_map(|i| &i.methods)
            .filter(|m| m.is_public)
            .count();

        ScanStats {
            total_types: self.types.len(),
            total_structs: self
                .types
                .iter()
                .filter(|t| matches!(t.kind, TypeKind::Struct))
                .count(),
            total_enums: self
                .types
                .iter()
                .filter(|t| matches!(t.kind, TypeKind::Enum))
                .count(),
            total_functions: self.functions.len(),
            total_impl_blocks: self.impls.len(),
            total_methods,
            public_methods,
            modules: type_counts.len(),
        }
    }

    pub fn build_bindability_registry(
        &self,
        visibility_graph: &ModuleVisibilityGraph,
    ) -> BindabilityRegistry {
        let mut registry = BindabilityRegistry::new();

        for module_path in &visibility_graph.public_modules {
            registry.register_public_module(module_path.clone());
        }

        for type_info in &self.types {
            let mut has_clone = type_info.has_clone();
            let mut has_display = type_info.has_display();
            let mut has_debug = type_info.has_debug();

            for impl_info in &self.impls {
                if impl_info.target_type == type_info.name {
                    if impl_info.is_clone_impl() {
                        has_clone = true;
                    }
                    if impl_info.is_display_impl() {
                        has_display = true;
                    }
                    if impl_info.is_debug_impl() {
                        has_debug = true;
                    }
                }
            }

            let is_in_public_module = visibility_graph.is_public(&type_info.module_path);

            let traits = TypeTraits {
                has_clone,
                has_display,
                has_debug,
                is_public: type_info.is_public,
                is_in_public_module,
                is_cfg_gated: type_info.is_cfg_gated,
                skip_binding: type_info.skip_binding,
            };

            registry.register_type(type_info.name.clone(), traits);
        }

        registry
    }
}

#[derive(Debug)]
pub struct ScanStats {
    pub total_types: usize,
    pub total_structs: usize,
    pub total_enums: usize,
    pub total_functions: usize,
    pub total_impl_blocks: usize,
    pub total_methods: usize,
    pub public_methods: usize,
    pub modules: usize,
}

pub fn scan() -> Result<ScannedApi> {
    use std::env;
    let crate_path = env::current_dir()?.join("crates/mathhook-core");
    scan_crate(&crate_path)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_has_no_binding_directive() {
        assert!(has_no_binding_directive(&Some("@no-binding".to_string())));
        assert!(has_no_binding_directive(&Some(
            "@no-binding - internal use only".to_string()
        )));
        assert!(has_no_binding_directive(&Some(
            "Some doc.\n\n@no-binding".to_string()
        )));
        assert!(has_no_binding_directive(&Some(
            "Some doc.\n@no-binding reason here\nMore doc.".to_string()
        )));

        assert!(!has_no_binding_directive(&None));
        assert!(!has_no_binding_directive(&Some("".to_string())));
        assert!(!has_no_binding_directive(&Some(
            "Regular documentation".to_string()
        )));
        assert!(!has_no_binding_directive(&Some(
            "no-binding without @".to_string()
        )));
        assert!(!has_no_binding_directive(&Some(
            "@NO-BINDING uppercase".to_string()
        )));
    }

    #[test]
    fn test_clean_doc_for_binding() {
        assert_eq!(
            clean_doc_for_binding("Some doc.\n\n@no-binding\n\nMore doc."),
            "Some doc.\n\n\nMore doc."
        );
        assert_eq!(
            clean_doc_for_binding("@no-binding - reason\nActual doc."),
            "Actual doc."
        );
        assert_eq!(
            clean_doc_for_binding("Clean doc without directive"),
            "Clean doc without directive"
        );
    }
}
