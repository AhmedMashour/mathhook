//! Mathematical property-based tests
//!
//! These tests validate mathematical correctness through fundamental properties
//! rather than comparing against external references (like SymPy).
//!
//! Properties tested:
//! - Commutativity: a + b = b + a, a * b = b * a
//! - Associativity: (a + b) + c = a + (b + c)
//! - Identities: sin^2(x) + cos^2(x) = 1, e^(ln x) = x
//! - Inverse operations: d/dx(integral(f)) = f, expand(factor(x)) = x
//! - Domain correctness: sqrt(-1) handling, log(0) errors

pub mod associativity;
pub mod commutativity;
pub mod domain_correctness;
pub mod identities;
pub mod inverse_operations;
pub mod numerical_stability;
