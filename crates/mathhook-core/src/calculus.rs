//! Calculus operations module
//!
//! Comprehensive symbolic calculus including differentiation, integration,
//! limits, series expansions, and advanced calculus operations.

pub mod derivatives;
pub mod integrals;
pub mod limits;
pub mod residues;
pub mod series;
pub mod summation;

// Re-export main traits and types
pub use derivatives::{
    BasicDerivatives, ChainRule, Derivative, DifferentiabilityChecker, FunctionDerivatives,
    GeneralProductRule, HigherOrderDerivatives, PowerRule, ProductRule,
};
pub use integrals::{
    BasicIntegrals,
    FunctionIntegrals,
    Integration,
    IntegrationMethods,
    // TODO: Re-export when implemented
    // DefiniteIntegrals, IntegrationByParts, IntegrationBySubstitution,
    // RationalIntegrals, TrigonometricIntegrals,
};
pub use limits::{LimitDirection, Limits};
pub use residues::{ComplexAnalysis, ResidueCalculus};
pub use series::{SeriesExpansion, SeriesType};
pub use summation::{Summation, SummationMethods};

/// Main calculus operations trait
pub trait CalculusOperations:
    Derivative + Integration + Limits + SeriesExpansion + Summation + ResidueCalculus
{
}

// Blanket implementation
impl<T> CalculusOperations for T where
    T: Derivative + Integration + Limits + SeriesExpansion + Summation + ResidueCalculus
{
}
