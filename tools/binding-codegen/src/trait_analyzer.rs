use crate::scanner::{ImplInfo, MethodInfo};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SupportedTrait {
    Add,
    Sub,
    Mul,
    Div,
    Neg,
    Display,
    Debug,
    PartialEq,
    PartialOrd,
    Hash,
    Clone,
    Default,
    FromStr,
}

impl SupportedTrait {
    pub fn from_name(name: &str) -> Option<Self> {
        match name {
            "Add" => Some(Self::Add),
            "Sub" => Some(Self::Sub),
            "Mul" => Some(Self::Mul),
            "Div" => Some(Self::Div),
            "Neg" => Some(Self::Neg),
            "Display" => Some(Self::Display),
            "Debug" => Some(Self::Debug),
            "PartialEq" => Some(Self::PartialEq),
            "PartialOrd" => Some(Self::PartialOrd),
            "Hash" => Some(Self::Hash),
            "Clone" => Some(Self::Clone),
            "Default" => Some(Self::Default),
            "FromStr" => Some(Self::FromStr),
            _ => None,
        }
    }

    pub fn is_binary_op(self) -> bool {
        matches!(self, Self::Add | Self::Sub | Self::Mul | Self::Div)
    }

    pub fn is_unary_op(self) -> bool {
        matches!(self, Self::Neg)
    }

    pub fn is_constructor(self) -> bool {
        matches!(self, Self::Default)
    }

    pub fn is_static_method(self) -> bool {
        matches!(self, Self::FromStr)
    }

    pub fn python_method_name(self) -> Option<&'static str> {
        match self {
            Self::Add => Some("__add__"),
            Self::Sub => Some("__sub__"),
            Self::Mul => Some("__mul__"),
            Self::Div => Some("__truediv__"),
            Self::Neg => Some("__neg__"),
            Self::Display => Some("__str__"),
            Self::PartialEq => Some("__eq__"),
            Self::PartialOrd => Some("__lt__"),
            Self::Clone => Some("__copy__"),
            Self::Hash => Some("__hash__"),
            Self::Debug | Self::Default | Self::FromStr => None,
        }
    }

    pub fn nodejs_method_name(self) -> Option<&'static str> {
        match self {
            Self::Display => Some("toString"),
            Self::PartialEq => Some("equals"),
            Self::PartialOrd => Some("compareTo"),
            Self::Clone => Some("clone"),
            Self::Add | Self::Sub | Self::Mul | Self::Div | Self::Neg => None,
            Self::Debug | Self::Default | Self::FromStr | Self::Hash => None,
        }
    }
}

fn is_prelude_trait(trait_name: &str) -> bool {
    matches!(
        trait_name,
        "Clone"
            | "Copy"
            | "Debug"
            | "Default"
            | "Display"
            | "Drop"
            | "Eq"
            | "Hash"
            | "Ord"
            | "PartialEq"
            | "PartialOrd"
            | "Send"
            | "Sync"
            | "Sized"
            | "Add"
            | "Sub"
            | "Mul"
            | "Div"
            | "Neg"
            | "Not"
            | "Index"
            | "IndexMut"
            | "Deref"
            | "DerefMut"
            | "From"
            | "Into"
            | "TryFrom"
            | "TryInto"
            | "AsRef"
            | "AsMut"
            | "Borrow"
            | "BorrowMut"
            | "ToString"
            | "ToOwned"
            | "Iterator"
            | "IntoIterator"
            | "Extend"
            | "FromIterator"
            | "FromStr"
    )
}

fn is_non_bindable_trait(trait_name: &str) -> bool {
    matches!(
        trait_name,
        "IntoVec" | "ComplexAnalysis" | "Serialize" | "Deserialize"
    )
}

fn is_trait_in_private_module(trait_path: Option<&str>) -> bool {
    let Some(path) = trait_path else {
        return false;
    };

    for segment in path.split("::") {
        if segment.starts_with('_') {
            return true;
        }
        if segment == "internal" || segment == "private" || segment == "impl_trait" {
            return true;
        }
    }

    false
}

fn is_result_type(type_str: &str) -> bool {
    type_str.starts_with("Result") || type_str.contains("Result <") || type_str.contains("Result<")
}

#[derive(Debug, Clone)]
pub struct OperatorTraitInfo {
    pub trait_type: SupportedTrait,
    pub rhs_type: Option<String>,
    pub output_type: Option<String>,
    pub returns_result: bool,
    pub is_self_to_self: bool,
}

impl OperatorTraitInfo {
    pub fn is_bindable_binary_op(&self) -> bool {
        self.is_self_to_self && !self.returns_result
    }
}

#[derive(Debug, Clone)]
pub struct DomainTraitMethod {
    pub trait_name: String,
    pub trait_path: Option<String>,
    pub module_path: String,
    pub method_info: MethodInfo,
    pub is_static: bool,
    pub returns_self: bool,
    pub takes_mut_self: bool,
}

impl DomainTraitMethod {
    pub fn method_name(&self) -> &str {
        &self.method_info.name
    }

    pub fn import_path_with_map(&self, trait_path_map: &HashMap<String, String>) -> Option<String> {
        if is_prelude_trait(&self.trait_name) {
            return None;
        }

        trait_path_map.get(&self.trait_name).cloned()
    }
}

#[derive(Debug)]
pub struct TraitAnalysis {
    pub type_name: String,
    pub implemented_traits: Vec<SupportedTrait>,
    pub domain_trait_methods: Vec<DomainTraitMethod>,
    pub operator_infos: Vec<OperatorTraitInfo>,
}

impl TraitAnalysis {
    pub fn from_impls(type_name: &str, impls: &[ImplInfo]) -> Self {
        Self::from_impls_and_derives(type_name, impls, &[])
    }

    pub fn from_impls_and_derives(
        type_name: &str,
        impls: &[ImplInfo],
        derived_traits: &[String],
    ) -> Self {
        let mut trait_set: HashSet<SupportedTrait> = HashSet::new();
        let mut operator_infos: Vec<OperatorTraitInfo> = Vec::new();

        for impl_info in impls.iter().filter(|i| i.target_type == type_name) {
            if let Some(ref trait_name) = impl_info.trait_name {
                if let Some(supported) = SupportedTrait::from_name(trait_name) {
                    if supported.is_binary_op() || supported.is_unary_op() {
                        let op_info = Self::analyze_operator_impl(impl_info, type_name, supported);
                        operator_infos.push(op_info);
                    }
                    trait_set.insert(supported);
                }
            }
        }

        for derived in derived_traits {
            if let Some(supported) = SupportedTrait::from_name(derived) {
                trait_set.insert(supported);
            }
        }

        let implemented_traits: Vec<SupportedTrait> = trait_set.into_iter().collect();

        let domain_trait_methods: Vec<DomainTraitMethod> = impls
            .iter()
            .filter(|impl_info| {
                impl_info.target_type == type_name
                    && impl_info.is_mathhook_core_trait()
                    && !impl_info.is_std_bindable_trait()
                    && impl_info
                        .trait_name
                        .as_deref()
                        .is_none_or(|name| !is_non_bindable_trait(name))
                    && !is_trait_in_private_module(impl_info.trait_path.as_deref())
            })
            .flat_map(|impl_info| {
                impl_info.methods.iter().filter(|m| m.is_public).map(|m| {
                    let first_input = m.signature.inputs.first();
                    let is_static = first_input.is_none_or(|(name, _)| !name.starts_with("self"));
                    let takes_mut_self = first_input
                        .map(|(name, _)| name == "self_mut")
                        .unwrap_or(false);

                    let returns_self = m
                        .signature
                        .output
                        .as_ref()
                        .map(|ty| {
                            let ty_str = quote::quote!(#ty).to_string();
                            ty_str == "Self" || ty_str.contains("Self")
                        })
                        .unwrap_or(false);

                    DomainTraitMethod {
                        trait_name: impl_info.trait_name.clone().unwrap_or_default(),
                        trait_path: impl_info.trait_path.clone(),
                        module_path: impl_info.module_path.clone(),
                        method_info: m.clone(),
                        is_static,
                        returns_self,
                        takes_mut_self,
                    }
                })
            })
            .collect();

        Self {
            type_name: type_name.to_string(),
            implemented_traits,
            domain_trait_methods,
            operator_infos,
        }
    }

    fn analyze_operator_impl(
        impl_info: &ImplInfo,
        type_name: &str,
        trait_type: SupportedTrait,
    ) -> OperatorTraitInfo {
        let trait_path = impl_info.trait_path.as_deref().unwrap_or("");
        let rhs_type = Self::extract_rhs_type_from_path(trait_path);
        let is_self_to_self = rhs_type.as_ref().is_none_or(|rhs| {
            rhs == "Self" || rhs == type_name || rhs.ends_with(&format!("::{}", type_name))
        });

        let op_method = impl_info.methods.iter().find(|m| {
            matches!(
                m.name.as_str(),
                "add" | "sub" | "mul" | "div" | "neg" | "not"
            )
        });

        let (output_type, returns_result) = {
            if let Some(ref assoc_output) = impl_info.associated_output_type {
                let is_result = is_result_type(assoc_output);
                (Some(assoc_output.clone()), is_result)
            } else if let Some(method) = op_method {
                if let Some(ref output) = method.signature.output {
                    let output_str = quote::quote!(#output).to_string();
                    let is_result = is_result_type(&output_str);
                    (Some(output_str), is_result)
                } else {
                    (None, false)
                }
            } else {
                (None, false)
            }
        };

        OperatorTraitInfo {
            trait_type,
            rhs_type,
            output_type,
            returns_result,
            is_self_to_self,
        }
    }

    fn extract_rhs_type_from_path(trait_path: &str) -> Option<String> {
        if trait_path.contains('<') && trait_path.contains('>') {
            let start = trait_path.find('<')? + 1;
            let end = trait_path.rfind('>')?;
            if start < end {
                let rhs = trait_path[start..end].trim().to_string();
                if !rhs.is_empty() {
                    return Some(rhs);
                }
            }
        }
        None
    }

    pub fn has_trait(&self, trait_type: SupportedTrait) -> bool {
        self.implemented_traits.contains(&trait_type)
    }

    pub fn binary_ops(&self) -> Vec<SupportedTrait> {
        let unique: HashSet<SupportedTrait> = self
            .operator_infos
            .iter()
            .filter(|info| info.trait_type.is_binary_op() && info.is_bindable_binary_op())
            .map(|info| info.trait_type)
            .collect();
        unique.into_iter().collect()
    }

    pub fn unary_ops(&self) -> Vec<SupportedTrait> {
        let unique: HashSet<SupportedTrait> = self
            .operator_infos
            .iter()
            .filter(|info| info.trait_type.is_unary_op() && !info.returns_result)
            .map(|info| info.trait_type)
            .collect();
        unique.into_iter().collect()
    }

    pub fn get_operator_info(&self, trait_type: SupportedTrait) -> Option<&OperatorTraitInfo> {
        self.operator_infos
            .iter()
            .find(|info| info.trait_type == trait_type)
    }

    pub fn has_domain_trait_methods(&self) -> bool {
        !self.domain_trait_methods.is_empty()
    }

    pub fn unique_trait_imports(&self, trait_path_map: &HashMap<String, String>) -> Vec<String> {
        let mut imports: HashSet<String> = self
            .domain_trait_methods
            .iter()
            .filter_map(|m| m.import_path_with_map(trait_path_map))
            .collect();

        let mut result: Vec<String> = imports.drain().collect();
        result.sort();
        result
    }
}

pub fn is_bindable_trait_name(name: &str) -> bool {
    SupportedTrait::from_name(name).is_some()
}

pub fn is_bindable_trait(impl_info: &ImplInfo) -> bool {
    impl_info.is_bindable_trait()
}
