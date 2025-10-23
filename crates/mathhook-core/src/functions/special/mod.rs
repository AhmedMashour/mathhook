//! Special mathematical functions
//!
//! Comprehensive implementation of special functions including:
//! - Gamma function family (gamma, beta, polygamma)
//! - Bessel functions (J, Y)
//! - Riemann zeta function
//! - Error functions (erf, erfc, erfi)
//! - Hypergeometric functions (1F1, 2F1)
//! - Elliptic functions (Jacobi sn, cn, dn)

pub mod bessel;
pub mod gamma;
pub mod intelligence;
pub mod zeta;

pub use bessel::{bessel_j, bessel_y};
pub use gamma::{beta, digamma, gamma, polygamma};
pub use intelligence::SpecialIntelligence;
pub use zeta::zeta;
