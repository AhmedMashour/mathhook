use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
pub struct BindingConfig {
    pub targets: Vec<String>,
    pub skip_patterns: Vec<String>,
    pub skip_types: Vec<String>,
    pub skip_module_paths: Vec<String>,
    pub force_include: Vec<String>,
    pub custom_mappings: Vec<CustomMapping>,
    #[serde(default)]
    pub external_types: Vec<ExternalTypeMapping>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CustomMapping {
    pub rust_type: String,
    pub python_type: Option<String>,
    pub node_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalTypeMapping {
    pub rust_type: String,
    pub python_type: String,
    pub node_type: String,
    pub source_crate: String,
    #[serde(default = "default_strategy")]
    pub strategy: String,
}

fn default_strategy() -> String {
    "wrapper".to_string()
}

impl BindingConfig {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let config: BindingConfig = toml::from_str(&content)?;
        Ok(config)
    }

    pub fn default_config() -> Self {
        BindingConfig {
            targets: vec!["python".to_string(), "node".to_string()],
            skip_patterns: vec![
                "*Data".to_string(),
                "*Internal".to_string(),
                "*Builder".to_string(),
            ],
            skip_module_paths: vec![],
            skip_types: Self::hand_written_types(),
            force_include: vec![],
            custom_mappings: vec![],
            external_types: vec![],
        }
    }

    fn hand_written_types() -> Vec<String> {
        vec![]
    }

    pub fn should_skip(&self, type_name: &str) -> bool {
        self.skip_types.contains(&type_name.to_string())
            || self
                .skip_patterns
                .iter()
                .any(|p| matches_pattern(type_name, p))
    }

    pub fn should_skip_module_path(&self, module_path: &str) -> bool {
        self.skip_module_paths
            .iter()
            .any(|pattern| module_path.contains(pattern))
    }

    pub fn force_include(&self, type_name: &str) -> bool {
        self.force_include.contains(&type_name.to_string())
    }
}

fn matches_pattern(type_name: &str, pattern: &str) -> bool {
    if pattern.starts_with('*') && pattern.ends_with('*') {
        let core = &pattern[1..pattern.len() - 1];
        type_name.contains(core)
    } else if let Some(suffix) = pattern.strip_prefix('*') {
        type_name.ends_with(suffix)
    } else if let Some(prefix) = pattern.strip_suffix('*') {
        type_name.starts_with(prefix)
    } else {
        type_name == pattern
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pattern_matching() {
        assert!(matches_pattern("FooData", "*Data"));
        assert!(matches_pattern("InternalBar", "Internal*"));
        assert!(matches_pattern("FooInternalBar", "*Internal*"));
        assert!(matches_pattern("ExactMatch", "ExactMatch"));
        assert!(!matches_pattern("Foo", "*Data"));
    }

    #[test]
    fn test_should_skip() {
        let config = BindingConfig {
            targets: vec![],
            skip_patterns: vec!["*Data".to_string(), "*Internal".to_string()],
            skip_types: vec!["SkipMe".to_string()],
            skip_module_paths: vec![],
            force_include: vec![],
            custom_mappings: vec![],
            external_types: vec![],
        };

        assert!(config.should_skip("FooData"));
        assert!(config.should_skip("BarInternal"));
        assert!(config.should_skip("SkipMe"));
        assert!(!config.should_skip("KeepMe"));
    }

    #[test]
    fn test_should_skip_module_path() {
        let config = BindingConfig {
            targets: vec![],
            skip_patterns: vec![],
            skip_types: vec![],
            skip_module_paths: vec!["bridge".to_string(), "internal".to_string()],
            force_include: vec![],
            custom_mappings: vec![],
            external_types: vec![],
        };

        assert!(config.should_skip_module_path("foo::bridge::bar"));
        assert!(config.should_skip_module_path("internal::utils"));
        assert!(!config.should_skip_module_path("public::api"));
    }

    #[test]
    fn test_force_include() {
        let config = BindingConfig {
            targets: vec![],
            skip_patterns: vec![],
            skip_types: vec![],
            skip_module_paths: vec![],
            force_include: vec!["Important".to_string()],
            custom_mappings: vec![],
            external_types: vec![],
        };

        assert!(config.force_include("Important"));
        assert!(!config.force_include("NotImportant"));
    }
}
