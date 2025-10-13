//! Special Function and Polynomial Properties
//!
//! Comprehensive mathematical properties for advanced special functions (gamma, bessel, zeta)
//! and polynomial families (Legendre, Hermite, Laguerre, Chebyshev, etc.).
//! Following SymPy's approach but optimized for performance.

use crate::core::Expression;
use super::rules::{
    AntiderivativeRule, RecurrenceRule, SpecialValue, ThreeTermRecurrence, EvaluationMethod
};

/// Special function properties (gamma, bessel, zeta, etc.)
///
/// Comprehensive mathematical properties for advanced special functions
/// following SymPy's approach but optimized for performance.
#[derive(Debug, Clone)]
pub struct SpecialProperties {
    /// Quick derivative check
    pub has_derivative: bool,

    /// Quick antiderivative check
    pub has_antiderivative: bool,

    /// Antiderivative rule (if known)
    pub antiderivative_rule: Option<AntiderivativeRule>,

    /// Recurrence relations for symbolic computation
    /// Examples: Γ(n+1) = n·Γ(n), J_{n+1} = (2n/x)J_n - J_{n-1}
    pub recurrence_relations: Vec<RecurrenceRule>,

    /// Differential equation the function satisfies
    /// Examples: Bessel DE, hypergeometric DE
    pub differential_equation: Option<DifferentialEquation>,

    /// Special values for exact computation
    pub special_values: Vec<SpecialValue>,

    /// Asymptotic behavior for large arguments
    pub asymptotic_behavior: Option<AsymptoticData>,
}

/// Polynomial function properties (legendre, hermite, laguerre, etc.)
///
/// Comprehensive properties for orthogonal polynomials and polynomial families
/// with focus on computational efficiency and mathematical correctness.
#[derive(Debug, Clone)]
pub struct PolynomialProperties {
    /// Polynomial family classification
    pub family: PolynomialFamily,

    /// Three-term recurrence relation
    /// Examples: P_{n+1} = ((2n+1)x P_n - n P_{n-1})/(n+1)
    pub recurrence: ThreeTermRecurrence,

    /// Orthogonality properties (if applicable)
    pub orthogonality: Option<OrthogonalityData>,

    /// Rodrigues' formula (if available)
    /// Examples: P_n(x) = (1/2^n n!) d^n/dx^n (x²-1)^n
    pub rodrigues_formula: Option<RodriguesFormula>,

    /// Generating function
    /// Examples: 1/√(1-2xt+t²) = Σ P_n(x) t^n
    pub generating_function: Option<GeneratingFunction>,

    /// Special values and boundary conditions
    pub special_values: Vec<SpecialValue>,

    /// Computational method for evaluation
    pub evaluation_method: EvaluationMethod,

    /// Antiderivative rule (for polynomial integration)
    /// All polynomials are integrable, so this is always Some(...)
    pub antiderivative_rule: AntiderivativeRule,
}

/// Polynomial family classification
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PolynomialFamily {
    Legendre,
    Hermite,
    Laguerre,
    Chebyshev,
    Jacobi,
    Gegenbauer,
}

/// Orthogonality properties for polynomial families
#[derive(Debug, Clone)]
pub struct OrthogonalityData {
    /// Weight function w(x)
    pub weight_function: Expression,

    /// Orthogonality interval [a, b]
    pub interval: (Expression, Expression),

    /// Normalization constant
    pub norm_squared: Expression,
}

/// Rodrigues' formula for polynomial construction
#[derive(Debug, Clone)]
pub struct RodriguesFormula {
    /// Formula template
    pub formula: String,

    /// Normalization constant
    pub normalization: Expression,

    /// Weight function
    pub weight_function: Expression,
}

/// Generating function for polynomial families
#[derive(Debug, Clone)]
pub struct GeneratingFunction {
    /// Generating function expression
    pub function: Expression,

    /// Type: ordinary or exponential
    pub gf_type: GeneratingFunctionType,
}

/// Types of generating functions
#[derive(Debug, Clone, Copy)]
pub enum GeneratingFunctionType {
    Ordinary,
    Exponential,
}

/// Differential equation representation
#[derive(Debug, Clone)]
pub struct DifferentialEquation {
    /// Order of the differential equation
    pub order: usize,

    /// Equation in standard form
    pub equation: String,

    /// Coefficients (if polynomial)
    pub coefficients: Vec<Expression>,
}

/// Asymptotic behavior data
#[derive(Debug, Clone)]
pub struct AsymptoticData {
    /// Behavior as x → ∞
    pub as_x_to_infinity: String,

    /// Behavior as x → 0
    pub as_x_to_zero: String,

    /// Leading term coefficient
    pub leading_coefficient: Expression,
}
