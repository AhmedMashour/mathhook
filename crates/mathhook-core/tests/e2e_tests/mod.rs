//! End-to-end integration tests for MathHook CAS
//!
//! These tests validate complete workflows across multiple components:
//! - Parser -> Simplifier -> Solver -> Formatter pipelines
//! - Cross-domain mathematical operations
//! - Real-world problem solving (physics, engineering, etc.)
//!
//! Tests here should exercise the INTEGRATION of components, not individual units.

pub mod evaluation_architecture;
pub mod function_dispatch;
pub mod mathematical_workflows;
pub mod system_solver;
pub mod systems_integration;
