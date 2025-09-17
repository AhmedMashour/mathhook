//! Function system trait architecture
//!
//! Clean trait-based architecture that defines clear boundaries between
//! different components of the function intelligence system.
//! Enables perfect modularity with zero coupling between modules.

use crate::core::Expression;
use crate::functions::properties::FunctionProperties;
use std::collections::HashMap;

/// Core trait for mathematical function intelligence
///
/// This trait defines the essential interface that all function intelligence
/// modules must implement, ensuring consistent behavior across the system.
///
/// ## Design Principles
/// - **Single Responsibility**: Each implementation handles one function family
/// - **Interface Segregation**: Minimal, focused interface
/// - **Dependency Inversion**: Depend on abstractions, not concretions
/// - **Open/Closed**: Open for extension, closed for modification
pub trait FunctionIntelligence: Send + Sync {
    /// Get the name of this function family
    fn family_name(&self) -> &'static str;

    /// Get all function properties managed by this intelligence
    fn get_all_properties(&self) -> HashMap<String, FunctionProperties>;

    /// Check if this intelligence manages a specific function
    fn has_function(&self, name: &str) -> bool;

    /// Get the number of functions managed by this intelligence
    fn function_count(&self) -> usize {
        self.get_all_properties().len()
    }

    /// Get function names managed by this intelligence
    fn function_names(&self) -> Vec<String> {
        self.get_all_properties().keys().cloned().collect()
    }
}

/// Trait for function educational capabilities
///
/// Provides step-by-step explanations and educational content
/// for mathematical functions, separated from core functionality.
pub trait FunctionEducator: Send + Sync {
    /// Generate step-by-step explanation for a function evaluation
    fn explain_evaluation(&self, name: &str, args: &[Expression]) -> Vec<String>;

    /// Generate LaTeX representation of a function
    fn to_latex(&self, name: &str, args: &[Expression]) -> String;

    /// Get mathematical background information
    fn get_background(&self, name: &str) -> Option<String>;

    /// Get related functions and concepts
    fn get_related_concepts(&self, name: &str) -> Vec<String>;
}

/// Trait for function optimization capabilities
///
/// Handles performance optimizations specific to different function families,
/// such as SIMD evaluation, caching, and special value detection.
pub trait FunctionOptimizer: Send + Sync {
    /// Optimize function evaluation for bulk operations
    fn optimize_bulk_evaluation(&self, name: &str, values: &[f64]) -> Option<Vec<f64>>;

    /// Check for special values that can be computed exactly
    fn detect_special_values(&self, name: &str, args: &[Expression]) -> Option<Expression>;

    /// Get optimal evaluation strategy for given input size
    fn optimal_strategy(&self, name: &str, input_size: usize) -> EvaluationStrategy;

    /// Estimate computational complexity
    fn complexity_estimate(&self, name: &str, input_size: usize) -> ComplexityEstimate;
}

/// Evaluation strategy for different scenarios
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EvaluationStrategy {
    /// Direct evaluation using standard library functions
    Direct,

    /// SIMD-optimized evaluation for bulk operations
    SIMD,

    /// Series expansion for high accuracy
    Series,

    /// Lookup table for frequently used values
    Lookup,

    /// Recursive evaluation using recurrence relations
    Recursive,
}

/// Computational complexity estimate
#[derive(Debug, Clone)]
pub struct ComplexityEstimate {
    /// Time complexity (operations per input)
    pub time_complexity: f64,

    /// Space complexity (memory per input)
    pub space_complexity: usize,

    /// Numerical accuracy (relative error)
    pub accuracy: f64,

    /// Recommended for input sizes up to this limit
    pub recommended_limit: usize,
}

/// Trait for function property validation
///
/// Ensures mathematical correctness and consistency of function properties
/// across different implementations and modules.
pub trait PropertyValidator: Send + Sync {
    /// Validate mathematical properties of a function
    fn validate_properties(&self, name: &str, properties: &FunctionProperties) -> ValidationResult;

    /// Check consistency between related functions
    fn validate_consistency(&self, functions: &[(&str, &FunctionProperties)]) -> ValidationResult;

    /// Validate numerical accuracy against known values
    fn validate_accuracy(&self, name: &str, test_cases: &[(Vec<f64>, f64)]) -> ValidationResult;
}

/// Result of property validation
#[derive(Debug, Clone)]
pub struct ValidationResult {
    /// Whether validation passed
    pub is_valid: bool,

    /// Validation score (0.0 to 1.0)
    pub score: f64,

    /// Detailed validation report
    pub report: String,

    /// Issues found during validation
    pub issues: Vec<ValidationIssue>,
}

/// Validation issue details
#[derive(Debug, Clone)]
pub struct ValidationIssue {
    /// Severity level of the issue
    pub severity: IssueSeverity,

    /// Description of the issue
    pub description: String,

    /// Suggested fix (if available)
    pub suggested_fix: Option<String>,
}

/// Severity levels for validation issues
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum IssueSeverity {
    /// Informational message
    Info,

    /// Warning that should be addressed
    Warning,

    /// Error that must be fixed
    Error,

    /// Critical error that breaks functionality
    Critical,
}

/// Trait for function metadata management
///
/// Handles metadata such as references, version information,
/// and compatibility data for function implementations.
pub trait MetadataProvider: Send + Sync {
    /// Get literature references for a function
    fn get_references(&self, name: &str) -> Vec<Reference>;

    /// Get version information
    fn get_version(&self, name: &str) -> Option<Version>;

    /// Get compatibility information
    fn get_compatibility(&self, name: &str) -> CompatibilityInfo;

    /// Get implementation notes
    fn get_implementation_notes(&self, name: &str) -> Option<String>;
}

/// Literature reference information
#[derive(Debug, Clone)]
pub struct Reference {
    /// Authors
    pub authors: Vec<String>,

    /// Title
    pub title: String,

    /// Publication details
    pub publication: String,

    /// Year of publication
    pub year: u32,

    /// DOI or URL (if available)
    pub identifier: Option<String>,
}

/// Version information
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Version {
    /// Major version number
    pub major: u32,

    /// Minor version number
    pub minor: u32,

    /// Patch version number
    pub patch: u32,
}

/// Compatibility information
#[derive(Debug, Clone)]
pub struct CompatibilityInfo {
    /// Minimum required version
    pub min_version: Version,

    /// Maximum supported version
    pub max_version: Option<Version>,

    /// Breaking changes in this version
    pub breaking_changes: Vec<String>,

    /// Deprecated features
    pub deprecated_features: Vec<String>,
}

/// Composite trait that combines all function capabilities
///
/// This trait provides a unified interface for complete function intelligence
/// modules that implement all aspects of function management.
pub trait CompleteFunctionIntelligence:
    FunctionIntelligence + FunctionEducator + FunctionOptimizer + PropertyValidator + MetadataProvider
{
    /// Get a comprehensive report about this function intelligence
    fn generate_report(&self) -> IntelligenceReport {
        IntelligenceReport {
            family_name: self.family_name().to_owned(),
            function_count: self.function_count(),
            function_names: self.function_names(),
            capabilities: vec![
                "Intelligence".to_owned(),
                "Evaluation".to_owned(),
                "Education".to_owned(),
                "Optimization".to_owned(),
                "Validation".to_owned(),
                "Metadata".to_owned(),
            ],
        }
    }
}

/// Comprehensive report about a function intelligence module
#[derive(Debug, Clone)]
pub struct IntelligenceReport {
    /// Name of the function family
    pub family_name: String,

    /// Number of functions managed
    pub function_count: usize,

    /// Names of all managed functions
    pub function_names: Vec<String>,

    /// Available capabilities
    pub capabilities: Vec<String>,
}

/// Factory trait for creating function intelligence modules
///
/// Provides a standardized way to create and configure function intelligence
/// modules with different parameters and options.
pub trait IntelligenceFactory {
    /// The type of intelligence this factory creates
    type Intelligence: FunctionIntelligence;

    /// Create a new intelligence instance with default configuration
    fn create_default() -> Self::Intelligence;

    /// Create a new intelligence instance with custom configuration
    fn create_with_config(config: &IntelligenceConfig) -> Self::Intelligence;

    /// Get the default configuration for this intelligence type
    fn default_config() -> IntelligenceConfig;
}

/// Configuration for function intelligence modules
#[derive(Debug, Clone)]
pub struct IntelligenceConfig {
    /// Enable high-precision mode
    pub high_precision: bool,

    /// Enable SIMD optimizations
    pub enable_simd: bool,

    /// Maximum cache size
    pub max_cache_size: usize,

    /// Validation level
    pub validation_level: ValidationLevel,

    /// Custom parameters
    pub custom_params: HashMap<String, String>,
}

/// Validation levels for function intelligence
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ValidationLevel {
    /// No validation
    None,

    /// Basic validation
    Basic,

    /// Standard validation
    Standard,

    /// Strict validation
    Strict,

    /// Research-grade validation
    Research,
}

impl Default for IntelligenceConfig {
    fn default() -> Self {
        Self {
            high_precision: false,
            enable_simd: true,
            max_cache_size: 1024,
            validation_level: ValidationLevel::Standard,
            custom_params: HashMap::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_ordering() {
        let v1 = Version {
            major: 1,
            minor: 0,
            patch: 0,
        };
        let v2 = Version {
            major: 1,
            minor: 0,
            patch: 1,
        };
        let v3 = Version {
            major: 1,
            minor: 1,
            patch: 0,
        };

        assert!(v1 < v2);
        assert!(v2 < v3);
        assert!(v1 < v3);
    }

    #[test]
    fn test_issue_severity_ordering() {
        assert!(IssueSeverity::Info < IssueSeverity::Warning);
        assert!(IssueSeverity::Warning < IssueSeverity::Error);
        assert!(IssueSeverity::Error < IssueSeverity::Critical);
    }

    #[test]
    fn test_default_config() {
        let config = IntelligenceConfig::default();
        assert!(!config.high_precision);
        assert!(config.enable_simd);
        assert_eq!(config.max_cache_size, 1024);
        assert_eq!(config.validation_level, ValidationLevel::Standard);
    }
}
