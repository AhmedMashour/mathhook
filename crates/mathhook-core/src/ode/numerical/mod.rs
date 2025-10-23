//! Numerical methods for solving ODEs
//!
//! Provides numerical solvers for first-order ordinary differential equations
//! when symbolic solutions are not available or practical. Includes:
//!
//! - **Euler method**: Simple first-order method for basic problems
//! - **Runge-Kutta 4th order (RK4)**: Classic fourth-order accurate method
//! - **Adaptive RKF45**: Runge-Kutta-Fehlberg with automatic step size control
//!
//! All methods work with floating-point functions f(x, y) representing dy/dx = f(x, y).

pub mod adaptive;
pub mod euler;
pub mod runge_kutta;

pub use adaptive::{rkf45_method, solve_adaptive, AdaptiveConfig};
pub use euler::{euler_method, solve_euler};
pub use runge_kutta::{rk4_method, solve_rk4};
