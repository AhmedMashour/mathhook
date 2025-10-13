//! Elementary Function Properties
//!
//! Properties and mathematical intelligence for elementary functions including
//! trigonometric, exponential, logarithmic, and hyperbolic functions.
//! Performance-optimized with hot path data first for cache-friendly access patterns.

use crate::core::Expression;
use super::rules::{
    DerivativeRule, AntiderivativeRule, SpecialValue, MathIdentity,
    DomainRangeData, NumericalEvaluator
};
use std::collections::HashMap;

/// Elementary function properties (sin, cos, exp, log)
///
/// Performance-optimized layout with hot path data first
/// for cache-friendly access patterns.
#[derive(Debug, Clone)]
pub struct ElementaryProperties {
    /// Most frequently accessed property (hot path data first)
    pub derivative_rule: Option<DerivativeRule>,

    /// Antiderivative rule for integration
    /// Placed second for cache locality with derivative_rule
    pub antiderivative_rule: Option<AntiderivativeRule>,

    /// Special values for exact computation
    /// Examples: sin(0) = 0, cos(π/2) = 0, exp(0) = 1
    pub special_values: Vec<SpecialValue>,

    /// Mathematical identities (boxed to keep struct small)
    /// Examples: sin²(x) + cos²(x) = 1, e^(ln(x)) = x
    pub identities: Box<Vec<MathIdentity>>,

    /// Domain and range information (cold path data)
    pub domain_range: Box<DomainRangeData>,

    /// Periodicity information (if applicable)
    pub periodicity: Option<Expression>,

    /// Numerical evaluation method for intelligence-driven computation
    pub numerical_evaluator: Option<NumericalEvaluator>,
}

/// User-defined function properties
///
/// Properties for functions defined by users (f, g, h, etc.)
/// Minimal overhead while supporting mathematical analysis.
#[derive(Debug, Clone)]
pub struct UserProperties {
    /// Function definition (if provided by user)
    pub definition: Option<Expression>,

    /// Known mathematical properties
    pub properties: Vec<UserProperty>,

    /// Known derivatives (if computed or provided)
    pub derivatives: HashMap<crate::core::Symbol, Expression>,

    /// Domain restriction (if specified)
    pub domain: Option<super::rules::Domain>,
}

/// User-defined function properties
#[derive(Debug, Clone)]
pub enum UserProperty {
    Even,
    Odd,
    Periodic(Expression),
    Monotonic,
    Bounded,
}
