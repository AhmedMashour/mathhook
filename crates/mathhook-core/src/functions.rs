//! Function Intelligence System
//!
//! This module implements the revolutionary universal function intelligence system
//! that provides mathematical properties, high-performance evaluation, and
//! educational explanations for ALL functions in MathHook.
//!
//! ## Architecture
//!
//! - **Single Function Representation**: All functions use `Expression::Function`
//! - **Intelligence Registry**: O(1) property lookup for mathematical properties
//! - **Performance Optimized**: Compliant with SIMD support
//! - **Educational Integration**: Step-by-step explanations for all operations
//!

// Core function
pub mod accuracy;
pub mod education;
pub mod evaluation;
pub mod extensibility;
pub mod intelligence;
pub mod properties;
pub mod traits;

// Modular function
pub mod elementary;
pub mod number_theory;
pub mod number_theory_eval;
pub mod polynomials;
pub mod special;

pub use accuracy::{AccuracyVerifier, VerifiedConstant, VerifiedRelationship, ACCURACY_VERIFIER};
pub use education::{FunctionEducator, StepGenerator};
pub use evaluation::EvaluationResult;
pub use extensibility::{
    DefaultValidator, ExtensionError, ExtensionRegistry, FunctionFamilyExtension,
    FunctionValidator, ValidationMetrics, ValidationResult,
};
pub use intelligence::UniversalFunctionRegistry;
pub use properties::{
    ElementaryProperties, FunctionProperties, PolynomialProperties, SpecialProperties,
};
pub use traits::{
    CompatibilityInfo, CompleteFunctionIntelligence, ComplexityEstimate, EvaluationStrategy,
    FunctionEducator as FunctionEducatorTrait, FunctionIntelligence, FunctionOptimizer,
    IntelligenceConfig, IntelligenceFactory, IntelligenceReport, IssueSeverity, MetadataProvider,
    PropertyValidator, Reference, ValidationIssue, ValidationLevel,
    ValidationResult as TraitValidationResult, Version,
};

// Re-export modular intelligence systems
pub use elementary::ElementaryIntelligence;
pub use number_theory::NumberTheoryIntelligence;
pub use polynomials::PolynomialIntelligence;
pub use special::SpecialIntelligence;
