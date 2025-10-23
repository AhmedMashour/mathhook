//! Ordinary Differential Equation (ODE) solvers
//!
//! Comprehensive ODE solving capabilities including:
//! - First-order methods (separable, linear, exact, homogeneous)
//! - Second-order methods (constant coefficients, Cauchy-Euler, variation of parameters)
//! - System of ODEs (linear systems with constant coefficients)
//! - Numerical methods (Euler, Runge-Kutta 4th order, adaptive RKF45)
//! - ODE classification and automatic method selection
//! - Step-by-step educational explanations

pub mod classifier;
pub mod educational;
pub mod first_order;
pub mod numerical;
pub mod second_order;
pub mod systems;

pub use classifier::{ODEClassifier, ODEType};
pub use educational::{
    EducationalODESolver, ODEExamples, ODEExplanation, ODEPhase, ODESolutionStep,
    ODESolutionStepBuilder, ODEStepFactory,
};
pub use first_order::{
    BernoulliODESolver, ExactODESolver, HomogeneousODESolver, LinearFirstOrderSolver, ODEError,
    ODEResult, SeparableODESolver,
};
pub use numerical::{euler_method, rk4_method, rkf45_method, AdaptiveConfig};
pub use systems::LinearSystemSolver;

pub type Result = first_order::ODEResult;
