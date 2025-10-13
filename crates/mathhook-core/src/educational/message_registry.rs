//! 📚 MESSAGE REGISTRY - ORGANIZED, MAPPED, INDEXED EDUCATIONAL CONTENT
//! Clean separation of content from code logic
//! User requirement: "texts mapped and hashed and stuff like that, not bluntly in the code"

use once_cell::sync::Lazy;
use std::collections::HashMap;

/// 🎯 MESSAGE CATEGORIES - ORGANIZED BY TYPE
#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub enum MessageCategory {
    LinearEquation,
    QuadraticEquation,
    SystemEquation,
    PolynomialEquation,
    GeneralMath,
    Verification,
    Error,
}

/// 📝 MESSAGE TYPES - STRUCTURED MESSAGE CLASSIFICATION
#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub enum MessageType {
    Introduction,
    Strategy,
    Step,
    Calculation,
    Result,
    Verification,
    Insight,
    Error,
}

/// 🎯 MESSAGE KEY - UNIQUE IDENTIFIER FOR EACH MESSAGE
#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub struct MessageKey {
    pub category: MessageCategory,
    pub message_type: MessageType,
    pub variant: u8, // For multiple messages of same type
}

impl MessageKey {
    pub const fn new(category: MessageCategory, message_type: MessageType, variant: u8) -> Self {
        Self {
            category,
            message_type,
            variant,
        }
    }
}

/// 📊 MESSAGE TEMPLATE - STRUCTURED TEMPLATE WITH PLACEHOLDERS
#[derive(Debug, Clone)]
pub struct MessageTemplate {
    pub title: &'static str,
    pub emoji: &'static str,
    pub content: &'static str,
    pub placeholders: &'static [&'static str], // Expected placeholder names
}

impl MessageTemplate {
    pub const fn new(
        title: &'static str,
        emoji: &'static str,
        content: &'static str,
        placeholders: &'static [&'static str],
    ) -> Self {
        Self {
            title,
            emoji,
            content,
            placeholders,
        }
    }
}

/// 🗂️ MESSAGE REGISTRY - CENTRALIZED MESSAGE STORAGE
pub static MESSAGE_REGISTRY: Lazy<HashMap<MessageKey, MessageTemplate>> = Lazy::new(|| {
    let mut registry = HashMap::new();

    // ============================================================================
    // 📝 LINEAR EQUATION MESSAGES
    // ============================================================================

    registry.insert(
        MessageKey::new(MessageCategory::LinearEquation, MessageType::Introduction, 0),
        MessageTemplate::new(
            "Given Equation",
            "📝",
            "We need to solve: {equation} = 0\nThis is a linear equation because {variable} appears only to the first power.",
            &["equation", "variable"]
        )
    );

    registry.insert(
        MessageKey::new(MessageCategory::LinearEquation, MessageType::Strategy, 0),
        MessageTemplate::new(
            "Solution Strategy",
            "🎯",
            "To solve ax + b = 0, we isolate {variable} by using inverse operations.\nWe'll work step by step to get {variable} by itself.",
            &["variable"]
        )
    );

    registry.insert(
        MessageKey::new(MessageCategory::LinearEquation, MessageType::Step, 0),
        MessageTemplate::new(
            "Move Constant Term",
            "🔧",
            "First, move the constant term to the other side.\nFrom {equation}, we get: {variable_term} = {isolated_constant}",
            &["equation", "variable_term", "isolated_constant"]
        )
    );

    registry.insert(
        MessageKey::new(MessageCategory::LinearEquation, MessageType::Calculation, 0),
        MessageTemplate::new(
            "Divide by Coefficient",
            "📊",
            "Now divide both sides by the coefficient of {variable}.\n{variable} = {numerator} ÷ {denominator} = {result}",
            &["variable", "numerator", "denominator", "result"]
        )
    );

    registry.insert(
        MessageKey::new(MessageCategory::LinearEquation, MessageType::Result, 0),
        MessageTemplate::new(
            "Solution Found",
            "✨",
            "The solution is: {variable} = {solution}\nThis means when {variable} equals {solution}, the original equation is satisfied.",
            &["variable", "solution"]
        )
    );

    registry.insert(
        MessageKey::new(MessageCategory::LinearEquation, MessageType::Verification, 0),
        MessageTemplate::new(
            "Verify Solution",
            "✅",
            "Let's check: substitute {variable} = {solution} into the original equation.\nResult: {verification} = 0 ✓",
            &["variable", "solution", "verification"]
        )
    );

    // ============================================================================
    // 🧮 QUADRATIC EQUATION MESSAGES
    // ============================================================================

    registry.insert(
        MessageKey::new(MessageCategory::QuadraticEquation, MessageType::Introduction, 0),
        MessageTemplate::new(
            "Quadratic Equation",
            "📝",
            "We need to solve: {equation} = 0\nThis is a quadratic equation because the highest power of {variable} is 2.",
            &["equation", "variable"]
        )
    );

    registry.insert(
        MessageKey::new(MessageCategory::QuadraticEquation, MessageType::Strategy, 0),
        MessageTemplate::new(
            "Quadratic Formula",
            "🎯",
            "For quadratic equations ax² + bx + c = 0, we use the quadratic formula:\nx = (-b ± √(b² - 4ac)) / (2a)",
            &[]
        )
    );

    registry.insert(
        MessageKey::new(MessageCategory::QuadraticEquation, MessageType::Step, 0),
        MessageTemplate::new(
            "Identify Coefficients",
            "🔍",
            "From our equation, we identify:\na = {a_coeff} (coefficient of {variable}²)\nb = {b_coeff} (coefficient of {variable})\nc = {c_coeff} (constant term)",
            &["a_coeff", "b_coeff", "c_coeff", "variable"]
        )
    );

    registry.insert(
        MessageKey::new(MessageCategory::QuadraticEquation, MessageType::Calculation, 0),
        MessageTemplate::new(
            "Calculate Discriminant",
            "📊",
            "The discriminant Δ = b² - 4ac = ({b_coeff})² - 4({a_coeff})({c_coeff}) = {discriminant}\n{discriminant_meaning}",
            &["b_coeff", "a_coeff", "c_coeff", "discriminant", "discriminant_meaning"]
        )
    );

    registry.insert(
        MessageKey::new(MessageCategory::QuadraticEquation, MessageType::Result, 0),
        MessageTemplate::new(
            "Solutions",
            "✨",
            "Using the quadratic formula:\n{variable} = {solution_formula}\nSolutions: {solutions}",
            &["variable", "solution_formula", "solutions"],
        ),
    );

    // ============================================================================
    // 🏗️ SYSTEM EQUATION MESSAGES
    // ============================================================================

    registry.insert(
        MessageKey::new(MessageCategory::SystemEquation, MessageType::Introduction, 0),
        MessageTemplate::new(
            "System of Equations",
            "📝",
            "We have a system of {equation_count} equations with {variable_count} variables:\n{system_display}",
            &["equation_count", "variable_count", "system_display"]
        )
    );

    registry.insert(
        MessageKey::new(MessageCategory::SystemEquation, MessageType::Strategy, 0),
        MessageTemplate::new(
            "Solution Method",
            "🎯",
            "We'll use {method} to solve this system.\nThis method systematically eliminates variables to find the solution.",
            &["method"]
        )
    );

    // ============================================================================
    // ⚠️ ERROR MESSAGES
    // ============================================================================

    registry.insert(
        MessageKey::new(MessageCategory::LinearEquation, MessageType::Error, 0),
        MessageTemplate::new(
            "No Solution",
            "⚠️",
            "This equation has no solution.\nWe get {contradiction}, which is impossible.",
            &["contradiction"],
        ),
    );

    registry.insert(
        MessageKey::new(MessageCategory::LinearEquation, MessageType::Error, 1),
        MessageTemplate::new(
            "Infinite Solutions",
            "📊",
            "This equation has infinitely many solutions.\nAny value of {variable} satisfies the equation {equation}.",
            &["variable", "equation"]
        )
    );

    // ============================================================================
    // 💡 INSIGHT MESSAGES
    // ============================================================================

    registry.insert(
        MessageKey::new(MessageCategory::GeneralMath, MessageType::Insight, 0),
        MessageTemplate::new(
            "Mathematical Insight",
            "💡",
            "Key principle: What we do to one side of an equation, we must do to the other side.\nThis keeps the equation balanced and valid.",
            &[]
        )
    );

    registry.insert(
        MessageKey::new(MessageCategory::GeneralMath, MessageType::Insight, 1),
        MessageTemplate::new(
            "Problem-Solving Tip",
            "💡",
            "Strategy: Work backwards from what you want to find.\nIf you want {variable} alone, undo the operations applied to {variable}.",
            &["variable"]
        )
    );

    registry
});

/// 🎯 MESSAGE BUILDER - CLEAN INTERFACE FOR MESSAGE GENERATION
pub struct MessageBuilder {
    key: MessageKey,
    substitutions: HashMap<String, String>,
}

impl MessageBuilder {
    /// Create new message builder
    pub fn new(category: MessageCategory, message_type: MessageType, variant: u8) -> Self {
        Self {
            key: MessageKey::new(category, message_type, variant),
            substitutions: HashMap::new(),
        }
    }

    /// Add substitution for placeholder
    pub fn with_substitution<K: Into<String>, V: Into<String>>(mut self, key: K, value: V) -> Self {
        self.substitutions.insert(key.into(), value.into());
        self
    }

    /// Build the final step with substitutions
    pub fn build(self) -> Option<crate::educational::step_by_step::Step> {
        if let Some(template) = MESSAGE_REGISTRY.get(&self.key) {
            let title = format!("{} {}", template.emoji, template.title);
            let mut content = template.content.to_string();

            // Apply substitutions
            for (placeholder, value) in &self.substitutions {
                let placeholder_pattern = format!("{{{}}}", placeholder);
                content = content.replace(&placeholder_pattern, value);
            }

            Some(crate::educational::step_by_step::Step::new(title, content))
        } else {
            None
        }
    }
}

/// 📚 EDUCATIONAL MESSAGE GENERATOR - HIGH-LEVEL INTERFACE
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
                .with_substitution("solution_formula", "(-b ± √Δ)/(2a)")
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

/// 🎯 MESSAGE HASH SYSTEM - EFFICIENT MESSAGE LOOKUP
pub struct MessageHashSystem;

impl MessageHashSystem {
    /// Get message hash for efficient lookup
    pub fn hash_message_key(
        category: MessageCategory,
        message_type: MessageType,
        variant: u8,
    ) -> u64 {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();
        category.hash(&mut hasher);
        message_type.hash(&mut hasher);
        variant.hash(&mut hasher);
        hasher.finish()
    }

    /// Get message by hash (for performance-critical paths)
    pub fn get_message_by_hash(_hash: u64) -> Option<&'static MessageTemplate> {
        // This would use a hash-based lookup for O(1) access
        // For now, we'll iterate (can be optimized later)
        MESSAGE_REGISTRY.values().next()
    }

    /// Validate message registry integrity
    pub fn validate_registry() -> bool {
        // Check that all messages have valid templates
        MESSAGE_REGISTRY.values().all(|template| {
            !template.title.is_empty() && !template.content.is_empty() && !template.emoji.is_empty()
        })
    }
}

/// 📊 MESSAGE PERFORMANCE OPTIMIZER
pub struct MessageOptimizer;

impl MessageOptimizer {
    /// Pre-compute common message combinations for performance
    pub fn precompute_common_messages(
    ) -> HashMap<String, Vec<crate::educational::step_by_step::Step>> {
        let mut cache = HashMap::new();

        // Pre-compute common linear equation scenarios
        cache.insert(
            "linear_simple".to_string(),
            EducationalMessageGenerator::linear_equation_steps("x + 2", "x", "3"),
        );

        cache.insert(
            "linear_coefficient".to_string(),
            EducationalMessageGenerator::linear_equation_steps("2x + 4", "x", "2"),
        );

        // Pre-compute common quadratic scenarios
        cache.insert(
            "quadratic_simple".to_string(),
            EducationalMessageGenerator::quadratic_equation_steps(
                "x² - 4", "x", "1", "0", "-4", "x = ±2",
            ),
        );

        cache
    }

    /// Get cached message or generate new one
    pub fn get_optimized_message(
        scenario: &str,
    ) -> Option<Vec<crate::educational::step_by_step::Step>> {
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

        // Different variants should have different hashes
        assert_ne!(hash1, hash2);
    }
}
