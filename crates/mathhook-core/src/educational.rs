//! Educational features and step-by-step explanations
//!
//! This module provides educational functionality including step-by-step
//! explanations, teaching solvers, and difficulty assessment.

pub mod enhanced_steps;
pub mod message_registry;
pub mod step_by_step;

// Re-export educational types
pub use enhanced_steps::{
    DifficultyLevel, EducationalResult, EnhancedStep, EnhancedStepExplanation,
};
pub use step_by_step::{Step, StepByStep, StepByStepExplanation};
