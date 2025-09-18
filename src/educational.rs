//! 🎓 EDUCATIONAL MODULE - STEP-BY-STEP EXPLANATIONS AND LEARNING FEATURES
//! Modern Rust module structure for all educational functionality
//! User requirement: "step by step or solutions should be a module, reasonably named"

// Step-by-step explanation system
pub mod step_by_step;

// Temporarily disabled - too complex for initial TDD
// pub mod enhanced_steps;
// pub mod messages;
// pub mod message_registry;

// Re-exports for easy access
// Temporarily disabled for TDD focus
// pub use enhanced_steps::{EnhancedStep, EnhancedStepExplanation, StepFactory};
// pub use message_registry::{EducationalMessageGenerator, MessageBuilder, MessageCategory, MessageType};
// pub use messages::{ClearMessageFactory, MessageContext};
pub use step_by_step::{Step, StepByStep, StepByStepExplanation};

/// 🎯 EDUCATIONAL TRAITS - COMMON INTERFACE FOR LEARNING FEATURES
pub trait Educational {
    /// Generate step-by-step explanation
    fn explain(&self) -> StepByStepExplanation;

    // Temporarily disabled
    // fn explain_enhanced(&self) -> EnhancedStepExplanation;

    /// Convert to LaTeX for educational display
    fn to_educational_latex(&self) -> String;

    /// Check if this concept is suitable for educational display
    fn is_educational(&self) -> bool;
}

/// 🎓 LEARNING DIFFICULTY LEVELS
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DifficultyLevel {
    Beginner,     // Basic arithmetic, simple equations
    Intermediate, // Algebra, quadratic equations
    Advanced,     // Calculus, complex systems
    Expert,       // Advanced mathematics
}

/// 📚 EDUCATIONAL CONTEXT - LEARNING ENVIRONMENT INFO
#[derive(Debug, Clone)]
pub struct EducationalContext {
    /// Target difficulty level
    pub difficulty: DifficultyLevel,

    /// Learning objectives
    pub objectives: Vec<String>,

    /// Prerequisites
    pub prerequisites: Vec<String>,

    /// Time estimate (minutes)
    pub estimated_time: u8,

    /// Interactive features enabled
    pub interactive: bool,
}

impl EducationalContext {
    /// Create context for beginner level
    pub fn beginner() -> Self {
        Self {
            difficulty: DifficultyLevel::Beginner,
            objectives: vec!["Understand basic concepts".to_string()],
            prerequisites: vec!["Basic arithmetic".to_string()],
            estimated_time: 5,
            interactive: true,
        }
    }

    /// Create context for intermediate level
    pub fn intermediate() -> Self {
        Self {
            difficulty: DifficultyLevel::Intermediate,
            objectives: vec!["Apply algebraic methods".to_string()],
            prerequisites: vec!["Basic algebra".to_string()],
            estimated_time: 10,
            interactive: true,
        }
    }

    /// Create context for advanced level
    pub fn advanced() -> Self {
        Self {
            difficulty: DifficultyLevel::Advanced,
            objectives: vec!["Master advanced techniques".to_string()],
            prerequisites: vec!["Intermediate algebra".to_string()],
            estimated_time: 20,
            interactive: false,
        }
    }
}
