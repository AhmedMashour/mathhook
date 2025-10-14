//! Educational operation traits for step-by-step explanations
//!
//! This module defines the core architectural pattern for integrating educational
//! explanations across all mathematical operations in MathHook.

use crate::core::Expression;
use crate::educational::step_by_step::StepByStepExplanation;

/// Context information for educational operations
///
/// Provides metadata about the operation being performed, which can be used
/// to generate appropriate educational explanations.
#[derive(Debug, Clone, PartialEq)]
pub struct OperationContext {
    /// Type of operation (e.g., "solving_equation", "differentiation", "simplification")
    pub operation_type: String,
    /// Difficulty level (1-10, where 1 is basic and 10 is advanced)
    pub difficulty_level: u8,
    /// Mathematical domain (e.g., "algebra", "calculus", "linear_algebra")
    pub domain: String,
    /// Prerequisites required to understand this operation
    pub prerequisites: Vec<String>,
}

impl OperationContext {
    /// Create a new operation context
    ///
    /// # Arguments
    ///
    /// * `operation_type` - Type of mathematical operation
    /// * `difficulty_level` - Difficulty from 1-10
    /// * `domain` - Mathematical domain
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::educational::traits::OperationContext;
    ///
    /// let context = OperationContext::new(
    ///     "solving_quadratic",
    ///     5,
    ///     "algebra"
    /// );
    /// assert_eq!(context.operation_type, "solving_quadratic");
    /// assert_eq!(context.difficulty_level, 5);
    /// ```
    pub fn new<S: Into<String>>(
        operation_type: S,
        difficulty_level: u8,
        domain: S,
    ) -> Self {
        Self {
            operation_type: operation_type.into(),
            difficulty_level: difficulty_level.min(10),
            domain: domain.into(),
            prerequisites: Vec::new(),
        }
    }

    /// Add a prerequisite to this operation
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::educational::traits::OperationContext;
    ///
    /// let mut context = OperationContext::new("solving_quadratic", 5, "algebra");
    /// context.add_prerequisite("factoring");
    /// context.add_prerequisite("square_roots");
    /// assert_eq!(context.prerequisites.len(), 2);
    /// ```
    pub fn add_prerequisite<S: Into<String>>(&mut self, prerequisite: S) {
        self.prerequisites.push(prerequisite.into());
    }

    /// Create context for equation solving
    pub fn equation_solving(difficulty_level: u8) -> Self {
        let mut context = Self::new("solving_equation", difficulty_level, "algebra");
        context.add_prerequisite("basic_algebra");
        context
    }

    /// Create context for differentiation
    pub fn differentiation(difficulty_level: u8) -> Self {
        let mut context = Self::new("differentiation", difficulty_level, "calculus");
        context.add_prerequisite("limits");
        context.add_prerequisite("functions");
        context
    }

    /// Create context for simplification
    pub fn simplification(difficulty_level: u8) -> Self {
        let mut context = Self::new("simplification", difficulty_level, "algebra");
        context.add_prerequisite("basic_operations");
        context
    }
}

/// Trait for mathematical operations that provide educational explanations
///
/// This trait establishes the architectural pattern for integrating step-by-step
/// educational explanations into mathematical operations. All operations that want
/// to provide educational value should implement this trait.
///
/// The trait provides two execution paths:
/// 1. `execute_with_steps` - Full educational mode with detailed explanations
/// 2. `execute_fast` - Performance-optimized mode without explanation overhead
pub trait EducationalOperation {
    /// The output type of this operation
    type Output;

    /// Execute the operation and return result with step-by-step explanation
    ///
    /// This method performs the mathematical operation while generating a complete
    /// educational explanation of each step. It is the primary method for educational
    /// use cases.
    ///
    /// # Returns
    ///
    /// A tuple containing:
    /// - The mathematical result of the operation
    /// - A complete step-by-step explanation
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// // Example implementation for a solver
    /// let (result, explanation) = solver.execute_with_steps();
    /// println!("Solution: {:?}", result);
    /// println!("Steps taken: {}", explanation.steps.len());
    /// ```
    fn execute_with_steps(&self) -> (Self::Output, StepByStepExplanation);

    /// Get educational context for this operation
    ///
    /// Returns metadata about the operation including its type, difficulty level,
    /// domain, and prerequisites. This can be used by educational systems to
    /// provide appropriate scaffolding and prerequisites.
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// let context = operation.educational_context();
    /// if context.difficulty_level > 7 {
    ///     println!("This is an advanced operation");
    /// }
    /// ```
    fn educational_context(&self) -> OperationContext;

    /// Execute the operation without generating educational explanation
    ///
    /// This is a fast-path execution that skips all explanation generation.
    /// Use this when performance is critical and educational content is not needed.
    ///
    /// The default implementation calls `execute_with_steps()` and discards the
    /// explanation, but implementations should override this for better performance.
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// // Fast execution without explanation overhead
    /// let result = solver.execute_fast();
    /// ```
    fn execute_fast(&self) -> Self::Output {
        let (result, _explanation) = self.execute_with_steps();
        result
    }

    /// Check if this operation can generate explanations
    ///
    /// Some operations may not be able to provide educational explanations
    /// in certain contexts. This method allows operations to indicate whether
    /// explanation generation is supported.
    ///
    /// The default implementation returns `true`.
    fn can_explain(&self) -> bool {
        true
    }

    /// Get estimated number of steps this operation will generate
    ///
    /// This can be used by educational systems to provide progress indicators
    /// or to estimate complexity. Returns `None` if the number of steps cannot
    /// be estimated in advance.
    fn estimated_steps(&self) -> Option<usize> {
        None
    }
}

/// Extension trait for adding educational capabilities to existing operations
///
/// This trait allows existing mathematical operations to be wrapped with
/// educational functionality without modifying their core implementation.
pub trait EducationalExt {
    /// Wrap this operation to enable educational explanation generation
    ///
    /// This method creates a wrapper that implements `EducationalOperation`
    /// around the existing operation.
    fn with_education(self) -> impl EducationalOperation<Output = Self>
    where
        Self: Sized + Clone;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_operation_context_creation() {
        let context = OperationContext::new("test_operation", 5, "algebra");
        assert_eq!(context.operation_type, "test_operation");
        assert_eq!(context.difficulty_level, 5);
        assert_eq!(context.domain, "algebra");
        assert!(context.prerequisites.is_empty());
    }

    #[test]
    fn test_operation_context_difficulty_clamping() {
        let context = OperationContext::new("test", 15, "algebra");
        assert_eq!(context.difficulty_level, 10);
    }

    #[test]
    fn test_operation_context_prerequisites() {
        let mut context = OperationContext::new("advanced_operation", 8, "calculus");
        context.add_prerequisite("limits");
        context.add_prerequisite("derivatives");

        assert_eq!(context.prerequisites.len(), 2);
        assert!(context.prerequisites.contains(&"limits".to_string()));
        assert!(context.prerequisites.contains(&"derivatives".to_string()));
    }

    #[test]
    fn test_context_factory_methods() {
        let eq_context = OperationContext::equation_solving(5);
        assert_eq!(eq_context.operation_type, "solving_equation");
        assert_eq!(eq_context.domain, "algebra");
        assert!(!eq_context.prerequisites.is_empty());

        let diff_context = OperationContext::differentiation(7);
        assert_eq!(diff_context.operation_type, "differentiation");
        assert_eq!(diff_context.domain, "calculus");

        let simp_context = OperationContext::simplification(3);
        assert_eq!(simp_context.operation_type, "simplification");
        assert_eq!(simp_context.domain, "algebra");
    }

    struct TestOperation {
        value: i64,
    }

    impl EducationalOperation for TestOperation {
        type Output = i64;

        fn execute_with_steps(&self) -> (Self::Output, StepByStepExplanation) {
            use crate::educational::step_by_step::Step;

            let steps = vec![
                Step::new("Initialize", "Starting with value"),
                Step::new("Calculate", "Performing operation"),
                Step::new("Result", "Final result obtained"),
            ];

            (self.value * 2, StepByStepExplanation::new(steps))
        }

        fn educational_context(&self) -> OperationContext {
            OperationContext::new("test_operation", 3, "testing")
        }

        fn execute_fast(&self) -> Self::Output {
            self.value * 2
        }

        fn estimated_steps(&self) -> Option<usize> {
            Some(3)
        }
    }

    #[test]
    fn test_educational_operation_implementation() {
        let operation = TestOperation { value: 21 };

        let (result, explanation) = operation.execute_with_steps();
        assert_eq!(result, 42);
        assert_eq!(explanation.steps.len(), 3);
        assert_eq!(explanation.total_steps, 3);

        let fast_result = operation.execute_fast();
        assert_eq!(fast_result, 42);

        let context = operation.educational_context();
        assert_eq!(context.operation_type, "test_operation");
        assert_eq!(context.difficulty_level, 3);

        assert!(operation.can_explain());
        assert_eq!(operation.estimated_steps(), Some(3));
    }
}
