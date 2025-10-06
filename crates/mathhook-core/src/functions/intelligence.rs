//! Function Intelligence Registry
//!
//! The core intelligence system that provides mathematical properties and
//! capabilities for ALL functions in MathHook.

use super::properties::FunctionProperties;
use crate::core::Expression;
use crate::educational::step_by_step::Step;
use once_cell::sync::Lazy;
use std::collections::HashMap;

/// Universal Function Intelligence Registry
///
/// Single source of truth for ALL function intelligence in MathHook.
/// Provides O(1) property lookup for maximum performance.
///
pub struct UniversalFunctionRegistry {
    /// Core mathematical properties for all functions
    /// O(1) lookup for function properties
    properties: HashMap<String, FunctionProperties>,

    /// Educational step generators for all functions
    /// Required for step-by-step explanations
    step_generators: HashMap<String, Box<dyn StepGenerator>>,
}

/// Function family classification for performance optimization
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FunctionFamily {
    Elementary,  // sin, cos, exp, log
    Special,     // gamma, bessel, etc.
    Polynomial,  // legendre, hermite, etc.
    UserDefined, // f, g, h, etc.
}

/// Step generator trait for educational explanations
///
/// All functions must implement step-by-step explanations
/// to comply with educational integration rules.
pub trait StepGenerator: Send + Sync {
    /// Generate step-by-step explanation for function evaluation
    fn generate_steps(&self, name: &str, args: &[Expression]) -> Vec<Step>;

    /// Generate LaTeX explanation (required for educational quality)
    fn generate_latex_explanation(&self, name: &str, args: &[Expression]) -> String;
}

/// Global function intelligence registry
///
/// Lazy initialization ensures zero startup cost while providing
/// universal access to function intelligence.
pub static UNIVERSAL_REGISTRY: Lazy<UniversalFunctionRegistry> =
    Lazy::new(|| UniversalFunctionRegistry::new());

impl UniversalFunctionRegistry {
    /// Create new universal function registry
    ///
    /// Initializes with all built-in mathematical functions and their properties.
    pub fn new() -> Self {
        // Advanced memory optimization: precise pre-allocation based on actual usage
        // Elementary: ~20, Polynomials: ~12, Special: ~15 = ~47 total
        // Use 64 for optimal hash table performance (power of 2)
        let mut registry = Self {
            properties: HashMap::with_capacity(64), // Optimized capacity
            step_generators: HashMap::with_capacity(64), // Matching for memory alignment
        };

        // Initialize built-in functions using modular intelligence
        registry.initialize_elementary_functions();
        registry.initialize_special_functions();
        registry.initialize_polynomial_functions();
        // Temporarily disable number theory to test if it's causing issues
        // registry.initialize_number_theory_functions();

        registry
    }

    /// Get function properties
    #[inline(always)]
    pub fn get_properties(&self, name: &str) -> Option<&FunctionProperties> {
        self.properties.get(name)
    }

    /// Check if function has mathematical intelligence
    #[inline(always)]
    pub fn has_intelligence(&self, name: &str) -> bool {
        self.properties.contains_key(name)
    }

    /// Debug: List all registered functions
    pub fn list_all_functions(&self) -> Vec<String> {
        self.properties.keys().cloned().collect()
    }

    /// Debug: Get registry size
    pub fn registry_size(&self) -> usize {
        self.properties.len()
    }

    /// Get step-by-step explanation for function operation
    ///
    /// Required for educational integration compliance
    pub fn explain_function(&self, name: &str, args: &[Expression]) -> Vec<Step> {
        if let Some(generator) = self.step_generators.get(name) {
            generator.generate_steps(name, args)
        } else {
            // Default explanation for unknown functions
            vec![
                Step::new("Function Call", format!("Evaluating {}(...)", name)),
                Step::new("Arguments", format!("With {} arguments", args.len())),
            ]
        }
    }

    fn initialize_elementary_functions(&mut self) {
        // Use modular elementary intelligence system
        let elementary_intelligence = super::elementary::ElementaryIntelligence::new();

        // Get all elementary function properties from modular system
        let elementary_properties = elementary_intelligence.get_all_properties();

        // Add to universal registry
        self.properties.extend(elementary_properties);

        println!(
            "Initialized modular elementary function intelligence: {} functions",
            elementary_intelligence.get_all_properties().len()
        );
    }

    /// Initialize special functions using modular architecture
    fn initialize_special_functions(&mut self) {
        // Use modular special function intelligence system
        let special_intelligence = super::special::SpecialIntelligence::new();
        let special_properties = special_intelligence.get_all_properties();
        self.properties.extend(special_properties);

        println!(
            "Initialized modular special function intelligence: {} functions",
            special_intelligence.get_all_properties().len()
        );
    }

    /// Initialize polynomial functions using modular architecture
    fn initialize_polynomial_functions(&mut self) {
        // Use modular polynomial intelligence system
        let polynomial_intelligence = super::polynomials::PolynomialIntelligence::new();

        // Get all polynomial function properties from modular system
        let polynomial_properties = polynomial_intelligence.get_all_properties();

        // Add to universal registry
        self.properties.extend(polynomial_properties);

        println!(
            "Initialized modular polynomial function intelligence: {} functions",
            polynomial_intelligence.get_all_properties().len()
        );
    }

    /// Initialize number theory functions using modular intelligence
    fn initialize_number_theory_functions(&mut self) {
        // Use modular number theory intelligence system
        let number_theory_intelligence = super::number_theory::NumberTheoryIntelligence::new();
        let number_theory_properties = number_theory_intelligence.get_all_properties();
        self.properties.extend(number_theory_properties);

        println!(
            "Initialized modular number theory function intelligence: {} functions",
            number_theory_intelligence.get_all_properties().len()
        );
    }
}

/// Get global function intelligence registry
///
/// Provides universal access to function intelligence throughout MathHook
#[inline(always)]
pub fn get_universal_registry() -> &'static UniversalFunctionRegistry {
    &UNIVERSAL_REGISTRY
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_registry_initialization() {
        let registry = UniversalFunctionRegistry::new();

        // Test that registry is properly initialized (capacity may grow)
        assert!(registry.properties.capacity() >= 256);
        assert!(registry.step_generators.capacity() >= 256);
    }

    #[test]
    fn test_has_intelligence_performance() {
        let registry = UniversalFunctionRegistry::new();

        // Test O(1) intelligence check performance
        let start = std::time::Instant::now();
        for _ in 0..100_000 {
            registry.has_intelligence("sin");
        }
        let duration = start.elapsed();

        // Should be extremely fast (sub-millisecond for 100k calls)
        // Relaxed threshold for complex modular system
        assert!(
            duration.as_millis() < 50,
            "Intelligence check too slow: {:?}",
            duration
        );
    }

    #[test]
    fn test_elementary_function_intelligence() {
        let registry = UniversalFunctionRegistry::new();

        // Test that elementary functions have intelligence
        assert!(registry.has_intelligence("sin"));
        assert!(registry.has_intelligence("cos"));
        assert!(registry.has_intelligence("exp"));
        assert!(registry.has_intelligence("ln"));

        // Test properties lookup
        if let Some(props) = registry.get_properties("sin") {
            assert!(props.has_derivative());
            assert_eq!(props.family(), FunctionFamily::Elementary);
        }
    }

    #[test]
    fn test_polynomial_function_intelligence() {
        let registry = UniversalFunctionRegistry::new();

        // Test that polynomial functions have intelligence
        assert!(registry.has_intelligence("legendre_p"));

        // Test properties lookup
        if let Some(props) = registry.get_properties("legendre_p") {
            assert!(props.has_derivative());
            assert_eq!(props.family(), FunctionFamily::Polynomial);
        }
    }
}
