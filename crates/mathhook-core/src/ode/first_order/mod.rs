//! First-order ODE solvers
//!
//! Implements various methods for solving first-order ordinary differential equations:
//! - Separable equations
//! - Linear first-order (integrating factor method)
//! - Exact equations
//! - Homogeneous equations
//! - Bernoulli equations

pub mod bernoulli;
pub mod exact;
pub mod homogeneous;
pub mod linear;
pub mod separable;

pub use bernoulli::BernoulliODESolver;
pub use exact::ExactODESolver;
pub use homogeneous::HomogeneousODESolver;
pub use linear::{LinearFirstOrderSolver, ODEError, ODEResult};
pub use separable::SeparableODESolver;
