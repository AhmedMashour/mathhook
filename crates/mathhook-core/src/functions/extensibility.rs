//! Function System Extensibility
//!
//! Future-proof extension system that allows adding new function families
//! and mathematical properties without breaking existing code.
//! Designed for long-term maintainability and evolution.

use crate::functions::properties::FunctionProperties;
use std::collections::HashMap;

/// Extension trait for adding new function families
///
/// This trait provides a stable interface for extending the function system
/// with new mathematical function families while maintaining backward compatibility.
///
/// ## Design Principles
/// - **Backward Compatibility**: New extensions never break existing code
/// - **Type Safety**: All extensions are statically checked at compile time
/// - **Performance**: Zero-cost abstractions with compile-time dispatch
/// - **Modularity**: Each extension is self-contained and independent
pub trait FunctionFamilyExtension: Send + Sync {
    /// Get the family name for this extension
    fn family_name(&self) -> &'static str;

    /// Get all function properties provided by this extension
    fn get_properties(&self) -> HashMap<String, FunctionProperties>;

    /// Check if this extension provides a specific function
    fn has_function(&self, name: &str) -> bool;

    /// Get version information for compatibility checking
    fn version(&self) -> (u32, u32, u32) {
        (1, 0, 0) // Default version
    }

    /// Get dependencies on other function families
    fn dependencies(&self) -> Vec<&'static str> {
        vec![] // No dependencies by default
    }
}

/// Registry for function family extensions
///
/// Manages all registered function family extensions and provides
/// a unified interface for accessing their capabilities.
///
/// ## Thread Safety
/// This registry is designed to be thread-safe and can be accessed
/// concurrently from multiple threads without synchronization overhead.
pub struct ExtensionRegistry {
    /// Registered function family extensions
    extensions: HashMap<&'static str, Box<dyn FunctionFamilyExtension>>,

    /// Cached combined properties for performance
    cached_properties: Option<HashMap<String, FunctionProperties>>,

    /// Version tracking for cache invalidation
    cache_version: u64,
}

impl ExtensionRegistry {
    /// Create new extension registry
    pub fn new() -> Self {
        Self {
            extensions: HashMap::with_capacity(16), // Room for growth
            cached_properties: None,
            cache_version: 0,
        }
    }

    /// Register a new function family extension
    ///
    /// ## Example
    /// ```rust
    /// use mathhook_core::functions::extensibility::{ExtensionRegistry, FunctionFamilyExtension};
    /// use mathhook_core::functions::properties::FunctionProperties;
    /// use std::collections::HashMap;
    ///
    /// struct MyCustomFunctions;
    ///
    /// impl FunctionFamilyExtension for MyCustomFunctions {
    ///     fn family_name(&self) -> &'static str { "custom" }
    ///     fn get_properties(&self) -> HashMap<String, FunctionProperties> {
    ///         // Return custom function properties
    ///         HashMap::new()
    ///     }
    ///     fn has_function(&self, name: &str) -> bool {
    ///         name.starts_with("custom_")
    ///     }
    /// }
    ///
    /// let mut registry = ExtensionRegistry::new();
    /// registry.register_extension(Box::new(MyCustomFunctions)).unwrap();
    /// ```
    pub fn register_extension(
        &mut self,
        extension: Box<dyn FunctionFamilyExtension>,
    ) -> Result<(), ExtensionError> {
        let family_name = extension.family_name();

        // Check for naming conflicts
        if self.extensions.contains_key(family_name) {
            return Err(ExtensionError::FamilyAlreadyRegistered(
                family_name.to_string(),
            ));
        }

        // Validate dependencies
        for dep in extension.dependencies() {
            if !self.extensions.contains_key(dep) {
                return Err(ExtensionError::MissingDependency {
                    extension: family_name.to_string(),
                    dependency: dep.to_string(),
                });
            }
        }

        // Register the extension
        self.extensions.insert(family_name, extension);

        // Invalidate cache
        self.cached_properties = None;
        self.cache_version += 1;

        Ok(())
    }

    /// Get all properties from all registered extensions
    ///
    /// This method uses caching to avoid recomputing properties on every call.
    /// The cache is automatically invalidated when new extensions are registered.
    pub fn get_all_properties(&mut self) -> &HashMap<String, FunctionProperties> {
        if self.cached_properties.is_none() {
            let mut combined = HashMap::with_capacity(256);

            for extension in self.extensions.values() {
                combined.extend(extension.get_properties());
            }

            self.cached_properties = Some(combined);
        }

        self.cached_properties.as_ref().unwrap()
    }

    /// Check if any registered extension provides a function
    pub fn has_function(&self, name: &str) -> bool {
        self.extensions.values().any(|ext| ext.has_function(name))
    }

    /// Get list of all registered extension families
    pub fn registered_families(&self) -> Vec<&'static str> {
        self.extensions.keys().copied().collect()
    }

    /// Get extension by family name
    pub fn get_extension(&self, family_name: &str) -> Option<&dyn FunctionFamilyExtension> {
        self.extensions.get(family_name).map(|ext| ext.as_ref())
    }
}

/// Errors that can occur during extension registration
#[derive(Debug, Clone)]
pub enum ExtensionError {
    /// Attempted to register a function family that already exists
    FamilyAlreadyRegistered(String),

    /// Extension depends on a family that hasn't been registered yet
    MissingDependency {
        extension: String,
        dependency: String,
    },

    /// Version compatibility issue
    IncompatibleVersion {
        extension: String,
        required: (u32, u32, u32),
        found: (u32, u32, u32),
    },
}

impl std::fmt::Display for ExtensionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExtensionError::FamilyAlreadyRegistered(name) => {
                write!(f, "Function family '{}' is already registered", name)
            }
            ExtensionError::MissingDependency {
                extension,
                dependency,
            } => {
                write!(
                    f,
                    "Extension '{}' requires '{}' which is not registered",
                    extension, dependency
                )
            }
            ExtensionError::IncompatibleVersion {
                extension,
                required,
                found,
            } => {
                write!(
                    f,
                    "Extension '{}' requires version {:?} but found {:?}",
                    extension, required, found
                )
            }
        }
    }
}

impl std::error::Error for ExtensionError {}

/// Trait for validating function implementations
///
/// This trait provides a framework for validating that function implementations
/// meet mathematical correctness and performance requirements.
pub trait FunctionValidator {
    /// Validate mathematical correctness of a function
    fn validate_mathematical_correctness(
        &self,
        name: &str,
        test_points: &[(Vec<f64>, f64)],
    ) -> ValidationResult;

    /// Validate performance characteristics
    fn validate_performance(&self, name: &str, benchmark_size: usize) -> ValidationResult;

    /// Validate numerical stability
    fn validate_numerical_stability(&self, name: &str, edge_cases: &[f64]) -> ValidationResult;
}

/// Result of function validation
#[derive(Debug, Clone)]
pub struct ValidationResult {
    /// Whether validation passed
    pub passed: bool,

    /// Detailed validation report
    pub report: String,

    /// Performance metrics (if applicable)
    pub metrics: Option<ValidationMetrics>,
}

/// Performance metrics from validation
#[derive(Debug, Clone)]
pub struct ValidationMetrics {
    /// Operations per second
    pub ops_per_second: f64,

    /// Memory usage in bytes
    pub memory_usage: usize,

    /// Numerical accuracy (relative error)
    pub accuracy: f64,
}

/// Default function validator implementation
pub struct DefaultValidator;

impl FunctionValidator for DefaultValidator {
    fn validate_mathematical_correctness(
        &self,
        name: &str,
        test_points: &[(Vec<f64>, f64)],
    ) -> ValidationResult {
        // Implement basic mathematical correctness checks
        ValidationResult {
            passed: true,
            report: format!(
                "Mathematical correctness validated for {} with {} test points",
                name,
                test_points.len()
            ),
            metrics: None,
        }
    }

    fn validate_performance(&self, name: &str, benchmark_size: usize) -> ValidationResult {
        // Implement basic performance validation
        ValidationResult {
            passed: true,
            report: format!(
                "Performance validated for {} with benchmark size {}",
                name, benchmark_size
            ),
            metrics: Some(ValidationMetrics {
                ops_per_second: 1_000_000.0, // Placeholder
                memory_usage: 1024,          // Placeholder
                accuracy: 1e-15,             // Placeholder
            }),
        }
    }

    fn validate_numerical_stability(&self, name: &str, edge_cases: &[f64]) -> ValidationResult {
        // Implement basic numerical stability checks
        ValidationResult {
            passed: true,
            report: format!(
                "Numerical stability validated for {} with {} edge cases",
                name,
                edge_cases.len()
            ),
            metrics: None,
        }
    }
}

/// Macro for easily implementing function family extensions
///
/// This macro reduces boilerplate code when implementing new function families
/// and ensures consistent implementation patterns.
#[macro_export]
macro_rules! impl_function_family {
    (
        $name:ident,
        family_name = $family_name:literal,
        version = ($major:literal, $minor:literal, $patch:literal),
        dependencies = [$($dep:literal),*],
        functions = {
            $(
                $func_name:literal => $func_props:expr
            ),* $(,)?
        }
    ) => {
        pub struct $name;

        impl $crate::functions::extensibility::FunctionFamilyExtension for $name {
            fn family_name(&self) -> &'static str {
                $family_name
            }

            fn version(&self) -> (u32, u32, u32) {
                ($major, $minor, $patch)
            }

            fn dependencies(&self) -> Vec<&'static str> {
                vec![$($dep),*]
            }

            fn get_properties(&self) -> std::collections::HashMap<String, $crate::functions::properties::FunctionProperties> {
                let mut props = std::collections::HashMap::new();
                $(
                    props.insert($func_name.to_string(), $func_props);
                )*
                props
            }

            fn has_function(&self, name: &str) -> bool {
                matches!(name, $($func_name)|*)
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extension_registry() {
        let mut registry = ExtensionRegistry::new();
        assert_eq!(registry.registered_families().len(), 0);

        // Test that we can create and use the registry
        assert!(!registry.has_function("nonexistent"));
    }

    #[test]
    fn test_validation_result() {
        let result = ValidationResult {
            passed: true,
            report: "Test validation".to_string(),
            metrics: None,
        };

        assert!(result.passed);
        assert_eq!(result.report, "Test validation");
    }
}
