use proc_macro2::TokenStream;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
pub enum MappedType {
    Direct {
        rust_type: String,
        python_type: String,
        node_type: String,
    },

    Result {
        ok_type: Box<MappedType>,
        err_type: String,
    },

    Option {
        inner_type: Box<MappedType>,
    },

    Collected {
        item_type: Box<MappedType>,
    },

    HashMap {
        key_type: Box<MappedType>,
        value_type: Box<MappedType>,
    },

    Tuple {
        elements: Vec<MappedType>,
    },

    Callback {
        arg_types: Vec<MappedType>,
        return_type: Box<MappedType>,
        is_mut: bool,
    },

    Reference {
        inner_type: Box<MappedType>,
        is_mut: bool,
    },

    Union {
        variants: Vec<TokenStream>,
        target_type: TokenStream,
    },

    Unsupported {
        reason: String,
        rust_type: String,
    },
}

impl MappedType {
    fn is_non_display_error(err_type: &str) -> bool {
        matches!(
            err_type,
            "PDEError" | "InternalError" | "BuilderError" | "ConfigError"
        )
    }

    fn is_non_bindable_type(rust_type: &str) -> bool {
        const UNBINDABLE_SUFFIXES: &[&str] = &[
            "Cache",
            "Profiler",
            "Rule",
            "RuleType",
            "Recurrence",
            "Identity",
        ];

        const UNBINDABLE_PREFIXES: &[&str] = &["Lazy", "Arc", "Box"];

        const UNBINDABLE_CONTAINS: &[&str] = &["Persistent", "Runtime", "dyn "];

        for suffix in UNBINDABLE_SUFFIXES {
            if rust_type.ends_with(suffix) {
                return true;
            }
        }

        for prefix in UNBINDABLE_PREFIXES {
            if rust_type.starts_with(prefix) {
                return true;
            }
        }

        for pattern in UNBINDABLE_CONTAINS {
            if rust_type.contains(pattern) {
                return true;
            }
        }

        false
    }

    pub fn is_supported(&self) -> bool {
        match self {
            MappedType::Unsupported { .. } => false,
            MappedType::Direct { rust_type, .. } => !Self::is_non_bindable_type(rust_type),
            MappedType::Reference { inner_type, .. } => match &**inner_type {
                MappedType::Direct { rust_type, .. } => !Self::is_non_bindable_type(rust_type),
                _ => inner_type.is_supported(),
            },
            MappedType::Option { inner_type } => inner_type.is_supported(),
            MappedType::Collected { item_type } => {
                !matches!(**item_type, MappedType::Collected { .. }) && item_type.is_supported()
            }
            MappedType::HashMap {
                key_type,
                value_type,
            } => Self::is_string_key(key_type) && value_type.is_supported(),
            MappedType::Tuple { elements } => {
                elements.is_empty() || elements.iter().all(|e| e.is_supported())
            }
            MappedType::Result { ok_type, err_type } => {
                !Self::is_non_display_error(err_type) && ok_type.is_supported()
            }
            MappedType::Callback {
                arg_types,
                return_type,
                ..
            } => arg_types.iter().all(|t| t.is_supported()) && return_type.is_supported(),
            MappedType::Union { .. } => false,
        }
    }

    fn is_string_key(key_type: &MappedType) -> bool {
        match key_type {
            MappedType::Direct { rust_type, .. } => {
                matches!(rust_type.as_str(), "String" | "str")
            }
            MappedType::Reference { inner_type, .. } => Self::is_string_key(inner_type),
            _ => false,
        }
    }

    pub fn complexity(&self) -> u8 {
        match self {
            MappedType::Direct { .. } => 1,
            MappedType::Reference { .. } => 2,
            MappedType::Option { .. } => 2,
            MappedType::Collected { .. } => 3,
            MappedType::HashMap { .. } => 3,
            MappedType::Tuple { .. } => 3,
            MappedType::Result { .. } => 3,
            MappedType::Union { .. } => 4,
            MappedType::Callback { .. } => 5,
            MappedType::Unsupported { .. } => 255,
        }
    }
}

#[derive(Debug, Clone)]
pub struct AnalyzedMethod {
    pub name: String,
    pub original_name: Option<String>,
    pub inputs: Vec<(String, MappedType)>,
    pub output: MappedType,
    pub is_supported: bool,
    pub impl_type: Option<String>,
    pub requires_mut_self: bool,
    pub doc_comment: Option<String>,
    pub skip_binding: bool,
}

#[derive(Debug, Clone, Default)]
pub struct TypeTraits {
    pub has_clone: bool,
    pub has_display: bool,
    pub has_debug: bool,
    pub is_public: bool,
    pub is_in_public_module: bool,
    pub is_cfg_gated: bool,
    pub skip_binding: bool,
}

impl TypeTraits {
    pub fn is_bindable(&self) -> bool {
        self.is_public && self.is_in_public_module && !self.is_cfg_gated && !self.skip_binding
    }

    pub fn needs_clone_for_mut_self(&self) -> bool {
        !self.has_clone
    }

    pub fn error_has_display(&self) -> bool {
        self.has_display || self.has_debug
    }
}

#[derive(Debug, Clone, Default)]
pub struct BindabilityRegistry {
    type_traits: HashMap<String, TypeTraits>,
    public_modules: HashSet<String>,
}

impl BindabilityRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register_type(&mut self, name: String, traits: TypeTraits) {
        self.type_traits.insert(name, traits);
    }

    pub fn register_public_module(&mut self, module_path: String) {
        self.public_modules.insert(module_path);
    }

    pub fn get_traits(&self, type_name: &str) -> Option<&TypeTraits> {
        self.type_traits.get(type_name)
    }

    pub fn is_clone_type(&self, type_name: &str) -> bool {
        self.type_traits.get(type_name).is_some_and(|t| t.has_clone)
    }

    pub fn is_display_type(&self, type_name: &str) -> bool {
        self.type_traits
            .get(type_name)
            .is_some_and(|t| t.has_display)
    }

    pub fn is_bindable_type(&self, type_name: &str) -> bool {
        self.type_traits
            .get(type_name)
            .is_none_or(|t| t.is_bindable())
    }

    pub fn is_public_module(&self, module_path: &str) -> bool {
        if module_path.is_empty() {
            return true;
        }
        self.public_modules.contains(module_path)
    }

    pub fn is_error_displayable(&self, err_type: &str) -> bool {
        self.type_traits
            .get(err_type)
            .is_some_and(|t| t.error_has_display())
    }
}
