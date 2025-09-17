//! Calculus integration tests (symbolic integration strategies)
//!
//! Tests for symbolic integration, differentiation, ODEs, and PDEs.
//! These validate the calculus subsystem working with other components.

pub mod api_tests;
pub mod integral_registry;
pub mod integral_strategies;
pub mod integration_strategy_tests;
pub mod numerical;
pub mod ode;
pub mod ode_separable;
pub mod pde;
pub mod pde_ode_bridge;
pub mod pde_separation;
pub mod rational_integrals;
pub mod risch_algorithm;
pub mod substitution;
pub mod table_lookup;
pub mod trig_integrals;
pub mod trig_product;

pub mod limits;
pub mod series;
