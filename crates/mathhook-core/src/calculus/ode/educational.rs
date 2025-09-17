//! Educational ODE Features
//!
//! Provides step-by-step explanations for ODE solving, including:
//! - Solution steps with mathematical justifications
//! - Educational wrappers around solvers
//! - Worked examples for common ODE types

pub mod examples;
pub mod steps;
pub mod wrapper;

pub use examples::ODEExamples;
pub use steps::{ODEPhase, ODESolutionStep, ODESolutionStepBuilder, ODEStepFactory};
pub use wrapper::{EducationalODESolver, ODEExplanation};
