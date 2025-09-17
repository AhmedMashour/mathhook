//! Derivative and Antiderivative Rules
//!
//! Defines rule types and structures for automatic differentiation and integration.
//! These rules enable the Universal Function Intelligence System to compute derivatives
//! and antiderivatives efficiently using pattern matching and rule-based evaluation.

use crate::core::{Expression, Symbol};
use std::sync::Arc;

/// Derivative rule for automatic differentiation
#[derive(Clone)]
pub struct DerivativeRule {
    /// Rule type for efficient computation
    pub rule_type: DerivativeRuleType,

    /// Result expression template (for documentation)
    pub result_template: String,
}

/// Types of derivative rules for performance optimization
pub enum DerivativeRuleType {
    /// Simple function substitution: d/dx sin(x) = cos(x)
    ///
    /// Stores the derivative function name for simple cases where
    /// the derivative is another elementary function of the same argument.
    ///
    /// Examples: sin → cos, exp → exp, sinh → cosh, cosh → sinh
    ///
    /// Note: For non-trivial derivatives (like ln → 1/x or sqrt → 1/(2√x)),
    /// use the Custom variant instead to build the actual expression.
    SimpleFunctionSubstitution(String),

    /// Custom derivative with expression builder
    ///
    /// Stores a closure that constructs the derivative expression
    /// given the function's argument. Used for complex derivatives like
    /// arcsin, arctan, sqrt, and composite expressions.
    ///
    /// Example: d/dx arcsin(u) = builder(u) → 1/√(1-u²)
    ///
    /// The builder takes the argument expression and returns
    /// the derivative with respect to that argument (without chain rule).
    /// The chain rule (multiplying by du/dx) is applied externally.
    Custom {
        #[allow(clippy::type_complexity)]
        builder: Arc<dyn Fn(&Expression) -> Expression + Send + Sync>,
    },

    /// Chain rule application: d/dx sin(u) = cos(u) * du/dx
    ///
    /// Legacy variant - prefer Custom for new implementations
    ChainRule(String),

    /// Product rule: d/dx (uv) = u'v + uv'
    ProductRule,

    /// Quotient rule: d/dx (u/v) = (u'v - uv')/v²
    QuotientRule,
}

impl Clone for DerivativeRuleType {
    fn clone(&self) -> Self {
        match self {
            DerivativeRuleType::SimpleFunctionSubstitution(name) => {
                DerivativeRuleType::SimpleFunctionSubstitution(name.clone())
            }
            DerivativeRuleType::Custom { builder } => DerivativeRuleType::Custom {
                builder: Arc::clone(builder),
            },
            DerivativeRuleType::ChainRule(name) => DerivativeRuleType::ChainRule(name.clone()),
            DerivativeRuleType::ProductRule => DerivativeRuleType::ProductRule,
            DerivativeRuleType::QuotientRule => DerivativeRuleType::QuotientRule,
        }
    }
}

impl std::fmt::Debug for DerivativeRuleType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DerivativeRuleType::SimpleFunctionSubstitution(name) => f
                .debug_struct("SimpleFunctionSubstitution")
                .field("function", name)
                .finish(),
            DerivativeRuleType::Custom { .. } => f
                .debug_struct("Custom")
                .field("builder", &"<closure>")
                .finish(),
            DerivativeRuleType::ChainRule(name) => {
                f.debug_struct("ChainRule").field("function", name).finish()
            }
            DerivativeRuleType::ProductRule => f.debug_struct("ProductRule").finish(),
            DerivativeRuleType::QuotientRule => f.debug_struct("QuotientRule").finish(),
        }
    }
}

impl std::fmt::Debug for DerivativeRule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DerivativeRule")
            .field("rule_type", &self.rule_type)
            .field("result_template", &self.result_template)
            .finish()
    }
}

/// Antiderivative rule for automatic integration
///
/// Stores the antiderivative formula for a function, analogous to DerivativeRule.
/// Supports simple antiderivatives, substitution patterns, and special techniques.
///
/// # Examples
///
/// ```rust
/// use mathhook_core::functions::properties::{AntiderivativeRule, AntiderivativeRuleType, ConstantOfIntegration};
/// use mathhook_core::expr;
///
/// let sin_antiderivative = AntiderivativeRule {
///     rule_type: AntiderivativeRuleType::Simple {
///         antiderivative_fn: "cos".to_string(),
///         coefficient: expr!(-1),
///     },
///     result_template: "-cos(x) + C".to_string(),
///     constant_handling: ConstantOfIntegration::AddConstant,
/// };
/// ```
#[derive(Debug, Clone)]
pub struct AntiderivativeRule {
    /// Rule type for efficient computation
    pub rule_type: AntiderivativeRuleType,

    /// Result expression template (for documentation and validation)
    /// Example: "∫sin(x)dx = -cos(x) + C"
    pub result_template: String,

    /// Constant of integration behavior
    pub constant_handling: ConstantOfIntegration,
}

/// Types of antiderivative rules for performance optimization
///
/// Each variant represents a different integration technique or pattern,
/// enabling the registry to efficiently compute integrals based on function properties.
pub enum AntiderivativeRuleType {
    /// Simple substitution: ∫sin(x)dx = -cos(x) + C
    ///
    /// Contains the antiderivative function name and multiplicative coefficient.
    /// This is the most common case for elementary functions.
    Simple {
        /// Name of the antiderivative function
        antiderivative_fn: String,

        /// Multiplicative coefficient (e.g., -1 for sin → -cos)
        coefficient: Expression,
    },

    /// Custom antiderivative with expression builder
    ///
    /// Stores a closure that constructs the antiderivative expression
    /// given the integration variable. Used for complex expressions like
    /// tan, cot, sec, csc, ln, log, arcsin, arccos, arctan, tanh, sqrt.
    ///
    /// Example: ∫tan(x)dx = builder(x) → -ln|cos(x)|
    Custom {
        #[allow(clippy::type_complexity)]
        builder: Arc<dyn Fn(Symbol) -> Expression + Send + Sync>,
    },

    /// Linear substitution: ∫f(ax)dx = (1/a)F(ax) + C
    ///
    /// Used for patterns like ∫sin(3x)dx = -(1/3)cos(3x) + C
    /// where the inner function is a linear transformation.
    LinearSubstitution {
        coefficient: Expression,
        inner_rule: Box<AntiderivativeRule>,
    },

    /// Trigonometric substitution patterns
    ///
    /// Used for integrals like ∫1/√(1-x²)dx = arcsin(x) + C
    /// where a trigonometric substitution simplifies the integral.
    TrigSubstitution { substitution_type: String },

    /// Partial fraction decomposition (for rational functions)
    ///
    /// Indicates that the integral requires partial fraction decomposition
    /// before integration can proceed.
    PartialFractions { decomposition: Vec<Expression> },
}

impl Clone for AntiderivativeRuleType {
    fn clone(&self) -> Self {
        match self {
            AntiderivativeRuleType::Simple {
                antiderivative_fn,
                coefficient,
            } => AntiderivativeRuleType::Simple {
                antiderivative_fn: antiderivative_fn.clone(),
                coefficient: coefficient.clone(),
            },
            AntiderivativeRuleType::Custom { builder } => AntiderivativeRuleType::Custom {
                builder: Arc::clone(builder),
            },
            AntiderivativeRuleType::LinearSubstitution {
                coefficient,
                inner_rule,
            } => AntiderivativeRuleType::LinearSubstitution {
                coefficient: coefficient.clone(),
                inner_rule: inner_rule.clone(),
            },
            AntiderivativeRuleType::TrigSubstitution { substitution_type } => {
                AntiderivativeRuleType::TrigSubstitution {
                    substitution_type: substitution_type.clone(),
                }
            }
            AntiderivativeRuleType::PartialFractions { decomposition } => {
                AntiderivativeRuleType::PartialFractions {
                    decomposition: decomposition.clone(),
                }
            }
        }
    }
}

impl std::fmt::Debug for AntiderivativeRuleType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AntiderivativeRuleType::Simple {
                antiderivative_fn,
                coefficient,
            } => f
                .debug_struct("Simple")
                .field("antiderivative_fn", antiderivative_fn)
                .field("coefficient", coefficient)
                .finish(),
            AntiderivativeRuleType::Custom { .. } => f
                .debug_struct("Custom")
                .field("builder", &"<closure>")
                .finish(),
            AntiderivativeRuleType::LinearSubstitution {
                coefficient,
                inner_rule,
            } => f
                .debug_struct("LinearSubstitution")
                .field("coefficient", coefficient)
                .field("inner_rule", inner_rule)
                .finish(),
            AntiderivativeRuleType::TrigSubstitution { substitution_type } => f
                .debug_struct("TrigSubstitution")
                .field("substitution_type", substitution_type)
                .finish(),
            AntiderivativeRuleType::PartialFractions { decomposition } => f
                .debug_struct("PartialFractions")
                .field("decomposition", decomposition)
                .finish(),
        }
    }
}

/// Constant of integration handling
///
/// Specifies how the constant of integration should be handled
/// in the antiderivative result.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ConstantOfIntegration {
    /// Automatically add +C to result
    AddConstant,

    /// Definite integral (no constant)
    DefiniteIntegral,

    /// User will handle constant explicitly
    UserHandled,
}

/// Recurrence relation for symbolic computation
#[derive(Debug, Clone)]
pub struct RecurrenceRule {
    /// Recurrence relation name
    pub name: String,

    /// Relation template (e.g., "f_{n+1} = a*f_n + b*f_{n-1}")
    pub relation: String,

    /// Coefficients for computation
    pub coefficients: Vec<Expression>,
}

/// Three-term recurrence for orthogonal polynomials
#[derive(Debug, Clone)]
pub struct ThreeTermRecurrence {
    /// Forward coefficient: α_n in P_{n+1} = (α_n x + β_n)P_n - γ_n P_{n-1}
    pub alpha_coeff: Expression,

    /// Linear coefficient: β_n
    pub beta_coeff: Expression,

    /// Backward coefficient: γ_n
    pub gamma_coeff: Expression,

    /// Initial conditions: P_0, P_1
    pub initial_conditions: (Expression, Expression),
}

/// Mathematical identity for symbolic computation
#[derive(Debug, Clone)]
pub struct MathIdentity {
    /// Identity name for reference
    pub name: String,

    /// Left side of identity
    pub lhs: Expression,

    /// Right side of identity
    pub rhs: Expression,

    /// Conditions for identity validity
    pub conditions: Vec<String>,
}

/// Special value for exact computation
#[derive(Debug, Clone)]
pub struct SpecialValue {
    /// Input value (e.g., "0", "π/2", "1")
    pub input: String,

    /// Exact output value
    pub output: Expression,

    /// LaTeX representation for educational display
    pub latex_explanation: String,
}

/// Domain and range information
#[derive(Debug, Clone)]
pub struct DomainRangeData {
    /// Function domain
    pub domain: Domain,

    /// Function range
    pub range: Range,

    /// Singularities (if any)
    pub singularities: Vec<Expression>,
}

/// Mathematical domain
#[derive(Debug, Clone, PartialEq)]
pub enum Domain {
    Real,
    Complex,
    Integer,
    PositiveInteger,
    NonNegativeInteger,
    Interval(Expression, Expression),
    Union(Vec<Domain>),
}

/// Mathematical range
#[derive(Debug, Clone)]
pub enum Range {
    Real,
    Integer,
    PositiveInteger,
    NonNegativeInteger,
    Boolean,
    Bounded(Expression, Expression),
    Unbounded,
}

/// Evaluation method for computational efficiency
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EvaluationMethod {
    /// Direct formula evaluation
    Direct,

    /// Recurrence relation (most efficient for polynomials)
    Recurrence,

    /// Horner's method for polynomial evaluation
    Horner,

    /// Series expansion
    Series,
}
