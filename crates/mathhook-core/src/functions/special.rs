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
pub mod beta;
pub mod digamma;
pub mod error_functions;
pub mod factorial;
pub mod gamma;
pub mod intelligence;
pub mod polygamma;
pub mod zeta;

pub use bessel::{bessel_j, bessel_y};
pub use beta::{beta, beta_numerical};
pub use digamma::{digamma, digamma_numerical};
pub use error_functions::{erf, erfc};
pub use factorial::factorial;
pub use gamma::{gamma, lanczos_gamma};
pub use intelligence::SpecialIntelligence;
pub use polygamma::polygamma;
pub use zeta::zeta;
