//! SymPy validation test suite
//!
//! This module contains comprehensive validation tests comparing MathHook output
//! to SymPy reference implementation (~/Documents/work/math/sympy/).
//!
//! Test categories:
//! - simplification_tests: Simplification operations (30 tests)
//! - derivative_tests: Derivative operations (30 tests)
//! - solver_tests: Equation solving (26 tests)
//! - special_functions_tests: Special functions and identities (38 tests)
//!
//! Note: Integration tests (29 tests) are disabled until integration is implemented
//! (see integration_tests.rs.disabled)
//!
//! Total: 124 active validation tests (153 including disabled integration tests)

pub mod derivative_tests;
// pub mod integration_tests;  // Disabled until integration is implemented
pub mod simplification_tests;
pub mod solver_tests;
pub mod special_functions_tests;
