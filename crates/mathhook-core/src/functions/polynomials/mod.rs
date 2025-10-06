//! Polynomial Function Intelligence
//!
//! Dedicated module for polynomial families with complete mathematical properties,
//! recurrence relations, orthogonality, and educational explanations.

pub mod chebyshev;
pub mod hermite;
pub mod laguerre;
pub mod legendre;

use crate::functions::properties::FunctionProperties;
use std::collections::HashMap;

/// Polynomial Function Intelligence Registry
///
/// Manages mathematical intelligence for all polynomial families
/// with proper modular separation by polynomial type.
pub struct PolynomialIntelligence {
    /// Legendre polynomials P_n(x)
    legendre: legendre::LegendreIntelligence,

    /// Hermite polynomials H_n(x)
    hermite: hermite::HermiteIntelligence,

    /// Laguerre polynomials L_n(x)
    laguerre: laguerre::LaguerreIntelligence,

    /// Chebyshev polynomials T_n(x), U_n(x)
    chebyshev: chebyshev::ChebyshevIntelligence,
}

impl PolynomialIntelligence {
    /// Create new polynomial intelligence system
    pub fn new() -> Self {
        Self {
            legendre: legendre::LegendreIntelligence::new(),
            hermite: hermite::HermiteIntelligence::new(),
            laguerre: laguerre::LaguerreIntelligence::new(),
            chebyshev: chebyshev::ChebyshevIntelligence::new(),
        }
    }

    /// Get all polynomial function properties
    ///
    /// Returns a HashMap of all polynomial functions and their properties
    /// for integration with the universal registry.
    pub fn get_all_properties(&self) -> HashMap<String, FunctionProperties> {
        let mut properties = HashMap::with_capacity(64);

        // Add Legendre polynomial properties
        properties.extend(self.legendre.get_properties());

        // Add Hermite polynomial properties
        properties.extend(self.hermite.get_properties());

        // Add Laguerre polynomial properties
        properties.extend(self.laguerre.get_properties());

        // Add Chebyshev polynomial properties
        properties.extend(self.chebyshev.get_properties());

        properties
    }

    /// Check if function is a polynomial
    pub fn is_polynomial_function(&self, name: &str) -> bool {
        self.legendre.has_function(name)
            || self.hermite.has_function(name)
            || self.laguerre.has_function(name)
            || self.chebyshev.has_function(name)
    }
}
