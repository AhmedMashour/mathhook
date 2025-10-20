//! Message registry with organized, mapped, indexed educational content
//! Clean separation of content from code logic
//! User requirement: "texts mapped and hashed and stuff like that, not bluntly in the code"

pub mod algebra;
pub mod calculus;
pub mod core;
pub mod solvers;
pub mod noncommutative;

pub use core::{
    MessageBuilder, MessageCategory, MessageHashSystem, MessageKey, MessageTemplate, MessageType,
    MESSAGE_REGISTRY,
};

use std::collections::HashMap;

/// Educational message generator providing high-level interface
pub struct EducationalMessageGenerator;

impl EducationalMessageGenerator {
    /// Generate linear equation explanation
    pub fn linear_equation_steps(
        equation: &str,
        variable: &str,
        solution: &str,
    ) -> Vec<crate::educational::step_by_step::Step> {
        vec![
            MessageBuilder::new(
                MessageCategory::LinearEquation,
                MessageType::Introduction,
                0,
            )
            .with_substitution("equation", equation)
            .with_substitution("variable", variable)
            .build()
            .unwrap(),
            MessageBuilder::new(MessageCategory::LinearEquation, MessageType::Strategy, 0)
                .with_substitution("variable", variable)
                .build()
                .unwrap(),
            MessageBuilder::new(MessageCategory::LinearEquation, MessageType::Result, 0)
                .with_substitution("variable", variable)
                .with_substitution("solution", solution)
                .build()
                .unwrap(),
            MessageBuilder::new(
                MessageCategory::LinearEquation,
                MessageType::Verification,
                0,
            )
            .with_substitution("variable", variable)
            .with_substitution("solution", solution)
            .with_substitution(
                "verification",
                &format!("{}({}) + constant", variable, solution),
            )
            .build()
            .unwrap(),
        ]
    }

    /// Generate quadratic equation explanation
    pub fn quadratic_equation_steps(
        equation: &str,
        variable: &str,
        a: &str,
        b: &str,
        c: &str,
        solutions: &str,
    ) -> Vec<crate::educational::step_by_step::Step> {
        vec![
            MessageBuilder::new(
                MessageCategory::QuadraticEquation,
                MessageType::Introduction,
                0,
            )
            .with_substitution("equation", equation)
            .with_substitution("variable", variable)
            .build()
            .unwrap(),
            MessageBuilder::new(MessageCategory::QuadraticEquation, MessageType::Strategy, 0)
                .build()
                .unwrap(),
            MessageBuilder::new(MessageCategory::QuadraticEquation, MessageType::Step, 0)
                .with_substitution("a_coeff", a)
                .with_substitution("b_coeff", b)
                .with_substitution("c_coeff", c)
                .with_substitution("variable", variable)
                .build()
                .unwrap(),
            MessageBuilder::new(MessageCategory::QuadraticEquation, MessageType::Result, 0)
                .with_substitution("variable", variable)
                .with_substitution("solution_formula", "(-b +/- sqrt(Delta))/(2a)")
                .with_substitution("solutions", solutions)
                .build()
                .unwrap(),
        ]
    }

    /// Generate error explanation
    pub fn error_explanation(
        category: MessageCategory,
        error_type: u8,
        context: &HashMap<String, String>,
    ) -> Option<crate::educational::step_by_step::Step> {
        let mut builder = MessageBuilder::new(category, MessageType::Error, error_type);

        for (key, value) in context {
            builder = builder.with_substitution(key, value);
        }

        builder.build()
    }

    /// Generate mathematical insight
    pub fn mathematical_insight(
        variant: u8,
        variable: &str,
    ) -> Option<crate::educational::step_by_step::Step> {
        MessageBuilder::new(MessageCategory::GeneralMath, MessageType::Insight, variant)
            .with_substitution("variable", variable)
            .build()
    }
}

/// Message performance optimizer
pub struct MessageOptimizer;

impl MessageOptimizer {
    /// Pre-compute common message combinations for performance
    pub fn precompute_common_messages(
    ) -> HashMap<String, Vec<crate::educational::step_by_step::Step>> {
        let mut cache = HashMap::new();

        cache.insert(
            "linear_simple".to_string(),
            EducationalMessageGenerator::linear_equation_steps("x + 2", "x", "3"),
        );

        cache.insert(
            "linear_coefficient".to_string(),
            EducationalMessageGenerator::linear_equation_steps("2x + 4", "x", "2"),
        );

        cache.insert(
            "quadratic_simple".to_string(),
            EducationalMessageGenerator::quadratic_equation_steps(
                "x^2 - 4", "x", "1", "0", "-4", "x = +/-2",
            ),
        );

        cache
    }

    /// Get cached message or generate new one
    pub fn get_optimized_message(
        scenario: &str,
    ) -> Option<Vec<crate::educational::step_by_step::Step>> {
        use once_cell::sync::Lazy;
        static CACHE: Lazy<HashMap<String, Vec<crate::educational::step_by_step::Step>>> =
            Lazy::new(|| MessageOptimizer::precompute_common_messages());

        CACHE.get(scenario).cloned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message_registry_integrity() {
        assert!(MessageHashSystem::validate_registry());
        assert!(!MESSAGE_REGISTRY.is_empty());
    }

    #[test]
    fn test_message_count() {
        let count = MESSAGE_REGISTRY.len();
        assert!(
            count >= 65,
            "Expected at least 65 messages, found {}",
            count
        );
    }

    #[test]
    fn test_message_builder() {
        let step = MessageBuilder::new(
            MessageCategory::LinearEquation,
            MessageType::Introduction,
            0,
        )
        .with_substitution("equation", "2x + 3")
        .with_substitution("variable", "x")
        .build();

        assert!(step.is_some());
        let step = step.unwrap();
        assert!(step.description.contains("2x + 3"));
        assert!(step.description.contains("linear equation"));
    }

    #[test]
    fn test_hash_system() {
        let hash1 = MessageHashSystem::hash_message_key(
            MessageCategory::LinearEquation,
            MessageType::Introduction,
            0,
        );

        let hash2 = MessageHashSystem::hash_message_key(
            MessageCategory::LinearEquation,
            MessageType::Introduction,
            1,
        );

        assert_ne!(hash1, hash2);
    }

    #[test]
    fn test_calculus_messages_exist() {
        let derivative_msg = MessageBuilder::new(
            MessageCategory::Calculus,
            MessageType::DerivativePowerRule,
            0,
        )
        .with_substitution("expression", "x^2")
        .with_substitution("exponent", "2")
        .build();

        assert!(derivative_msg.is_some());
    }

    #[test]
    fn test_algebra_messages_exist() {
        let algebra_msg = MessageBuilder::new(
            MessageCategory::Algebra,
            MessageType::SimplifyCombineLike,
            0,
        )
        .with_substitution("expression", "2x + 3x")
        .with_substitution("like_terms", "2x and 3x")
        .build();

        assert!(algebra_msg.is_some());
    }

    #[test]
    fn test_system_messages_exist() {
        let system_msg = MessageBuilder::new(
            MessageCategory::SystemEquation,
            MessageType::SystemSubstitution,
            0,
        )
        .build();

        assert!(system_msg.is_some());
    }
}
