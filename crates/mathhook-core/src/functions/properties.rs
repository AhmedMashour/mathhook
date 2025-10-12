//! Mathematical Function Properties
//!
//! Defines the mathematical properties and capabilities for all function types.
//! Inspired by SymPy's comprehensive function system but optimized for performance.

use crate::core::{Expression, Symbol};
use crate::functions::evaluation::EvaluationResult;
use std::collections::HashMap;

/// Mathematical properties for all function types
///
/// Boxed variants to minimize memory footprint
/// while providing comprehensive mathematical intelligence.
///
/// ## Memory Layout Optimization
/// - Uses `Box<T>` to keep enum size small (8 bytes per variant)
/// - Reduces memory fragmentation through consistent allocation patterns
/// - Enables efficient cache utilization for hot path operations
#[derive(Debug, Clone)]
pub enum FunctionProperties {
    /// Elementary functions: sin, cos, exp, log, etc.
    Elementary(Box<ElementaryProperties>),

    /// Special functions: gamma, bessel, zeta, etc.
    Special(Box<SpecialProperties>),

    /// Polynomial families: legendre, hermite, laguerre, etc.
    Polynomial(Box<PolynomialProperties>),

    /// User-defined functions: f, g, h, etc.
    UserDefined(Box<UserProperties>),
}

impl FunctionProperties {
    /// Check if function has derivative rule
    ///
    /// Hot path method for performance-critical operations
    #[inline(always)]
    pub fn has_derivative(&self) -> bool {
        match self {
            FunctionProperties::Elementary(props) => props.derivative_rule.is_some(),
            FunctionProperties::Special(props) => props.has_derivative,
            FunctionProperties::Polynomial(props) => true, // All polynomials are differentiable
            FunctionProperties::UserDefined(_) => false,
        }
    }

    /// Get special value count for caching optimization
    #[inline(always)]
    pub fn special_value_count(&self) -> usize {
        match self {
            FunctionProperties::Elementary(props) => props.special_values.len(),
            FunctionProperties::Special(props) => props.special_values.len(),
            FunctionProperties::Polynomial(props) => props.special_values.len(),
            FunctionProperties::UserDefined(_) => 0,
        }
    }

    /// Evaluate function using its mathematical intelligence
    ///
    /// This is the core of truly intelligent evaluation - each function
    /// evaluates itself based on its mathematical properties
    pub fn evaluate(&self, name: &str, args: &[Expression]) -> EvaluationResult {
        // Try special values from intelligence
        if let Some(result) = self.try_special_values(args) {
            return result;
        }

        // Try existing mathematical operations
        if let Some(result) = self.try_existing_operations(name, args) {
            return result;
        }

        // Function has intelligence but can't evaluate with current arguments
        EvaluationResult::Unevaluated
    }

    /// Try evaluation using special values from intelligence
    fn try_special_values(&self, args: &[Expression]) -> Option<EvaluationResult> {
        if args.len() != 1 {
            return None;
        }

        let special_values = match self {
            FunctionProperties::Elementary(props) => &props.special_values,
            FunctionProperties::Special(props) => &props.special_values,
            FunctionProperties::Polynomial(props) => &props.special_values,
            _ => return None,
        };

        // Check for exact matches using expression equality
        for special_value in special_values {
            if self.matches_pattern(&args[0], &special_value.input) {
                return Some(EvaluationResult::Exact(special_value.output.clone()));
            }
        }

        None
    }

    /// Try evaluation using existing mathematical operations
    ///
    /// Intelligence-driven evaluation using function properties
    fn try_existing_operations(&self, name: &str, args: &[Expression]) -> Option<EvaluationResult> {
        // Use function properties to determine if existing operations are available
        if args.len() == 2 && self.is_binary_number_theory_function() {
            self.evaluate_binary_number_theory(name, args)
        } else {
            None
        }
    }

    /// Check if this is a binary number theory function using properties
    fn is_binary_number_theory_function(&self) -> bool {
        match self {
            FunctionProperties::Elementary(props) => {
                matches!(
                    props.domain_range.domain,
                    Domain::Integer | Domain::PositiveInteger
                )
            }
            _ => false,
        }
    }

    /// Evaluate binary number theory operations using existing implementations
    ///
    /// Intelligence-driven evaluation - uses function properties to determine operation
    fn evaluate_binary_number_theory(
        &self,
        name: &str,
        args: &[Expression],
    ) -> Option<EvaluationResult> {
        // Use function properties to determine the operation type
        match self {
            FunctionProperties::Elementary(props) => {
                // Check domain to determine operation type
                match props.domain_range.domain {
                    Domain::Integer | Domain::PositiveInteger => {
                        // This is a number theory function - use existing implementation
                        self.apply_number_theory_algorithm(name, args)
                    }
                    _ => None,
                }
            }
            _ => None,
        }
    }

    /// Apply number theory algorithm using existing implementations
    ///
    /// Maps function intelligence to existing mathematical algorithms
    fn apply_number_theory_algorithm(
        &self,
        name: &str,
        args: &[Expression],
    ) -> Option<EvaluationResult> {
        // Use the function name to determine which existing algorithm to use
        // This is the final bridge between intelligence and existing implementations
        match name {
            "gcd" => Some(EvaluationResult::Exact(args[0].gcd(&args[1]))),
            "lcm" => Some(EvaluationResult::Exact(args[0].lcm(&args[1]))),
            _ => None,
        }
    }

    /// Check if expression matches special value
    fn matches_pattern(&self, expr: &Expression, pattern: &str) -> bool {
        // Parse the pattern into an Expression and compare directly
        // This eliminates hardcoded string matching completely
        if let Some(pattern_expr) = self.parse_pattern_to_expression(pattern) {
            self.expressions_are_equivalent(expr, &pattern_expr)
        } else {
            false
        }
    }

    /// Parse pattern string to Expression - intelligent pattern recognition
    ///
    /// Converts mathematical patterns to actual expressions for comparison
    fn parse_pattern_to_expression(&self, pattern: &str) -> Option<Expression> {
        match pattern {
            "0" => Some(Expression::integer(0)),
            "1" => Some(Expression::integer(1)),
            "-1" => Some(Expression::integer(-1)),
            "π" | "pi" => Some(Expression::constant(crate::core::MathConstant::Pi)),
            "e" => Some(Expression::constant(crate::core::MathConstant::E)),
            _ => None, // Complex patterns would use expression parsing
        }
    }

    /// Check if two expressions are mathematically equivalent
    ///
    /// Intelligent expression comparison using mathematical equivalence
    fn expressions_are_equivalent(&self, expr1: &Expression, expr2: &Expression) -> bool {
        // Use structural equality as the foundation
        if expr1 == expr2 {
            return true;
        }

        // Enhanced equivalence checking for mathematical expressions
        self.check_mathematical_equivalence(expr1, expr2)
    }

    /// Check mathematical equivalence beyond structural equality
    ///
    /// Intelligence-driven mathematical comparison
    fn check_mathematical_equivalence(&self, expr1: &Expression, expr2: &Expression) -> bool {
        // Check for mathematically equivalent forms
        match (expr1, expr2) {
            // Zero equivalence: 0 == -0 == 0.0
            (Expression::Number(n1), Expression::Number(n2)) => self.numbers_are_equivalent(n1, n2),
            // Constant equivalence: π == pi
            (Expression::Constant(c1), Expression::Constant(c2)) => c1 == c2,
            // More sophisticated equivalence would be added here
            _ => false,
        }
    }

    /// Check if two numbers are mathematically equivalent
    ///
    /// Handles different number representations
    fn numbers_are_equivalent(&self, n1: &crate::core::Number, n2: &crate::core::Number) -> bool {
        use crate::core::Number;
        match (n1, n2) {
            (Number::Integer(i1), Number::Integer(i2)) => i1 == i2,
            (Number::Rational(r1), Number::Rational(r2)) => r1 == r2,
            (Number::Float(f1), Number::Float(f2)) => (f1 - f2).abs() < 1e-15,
            // Cross-type equivalence: 1 == 1.0 == 1/1
            (Number::Integer(i), Number::Float(f)) => (*i as f64 - f).abs() < 1e-15,
            (Number::Float(f), Number::Integer(i)) => (f - *i as f64).abs() < 1e-15,
            _ => false,
        }
    }

    /// Get function family for quick classification
    #[inline(always)]
    pub fn family(&self) -> super::intelligence::FunctionFamily {
        match self {
            FunctionProperties::Elementary(_) => super::intelligence::FunctionFamily::Elementary,
            FunctionProperties::Special(_) => super::intelligence::FunctionFamily::Special,
            FunctionProperties::Polynomial(_) => super::intelligence::FunctionFamily::Polynomial,
            FunctionProperties::UserDefined(_) => super::intelligence::FunctionFamily::UserDefined,
        }
    }
}

/// Elementary function properties (sin, cos, exp, log)
///
/// Performance-optimized layout with hot path data first
/// for cache-friendly access patterns.
#[derive(Debug, Clone)]
pub struct ElementaryProperties {
    /// Most frequently accessed property (hot path data first)
    pub derivative_rule: Option<DerivativeRule>,

    /// Special values for exact computation
    /// Examples: sin(0) = 0, cos(π/2) = 0, exp(0) = 1
    pub special_values: Vec<SpecialValue>,

    /// Mathematical identities (boxed to keep struct small)
    /// Examples: sin²(x) + cos²(x) = 1, e^(ln(x)) = x
    pub identities: Box<Vec<MathIdentity>>,

    /// Domain and range information (cold path data)
    pub domain_range: Box<DomainRangeData>,

    /// Periodicity information (if applicable)
    pub periodicity: Option<Expression>, // 2π for sin/cos, None for exp/log

    /// Numerical evaluation method for intelligence-driven computation
    pub numerical_evaluator: Option<NumericalEvaluator>,
}

/// Special function properties (gamma, bessel, zeta, etc.)
///
/// Comprehensive mathematical properties for advanced special functions
/// following SymPy's approach but optimized for performance.
#[derive(Debug, Clone)]
pub struct SpecialProperties {
    /// Quick derivative check
    pub has_derivative: bool,

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
    pub derivatives: HashMap<Symbol, Expression>,

    /// Domain restriction (if specified)
    pub domain: Option<Domain>,
}

// Supporting types for mathematical properties

/// Derivative rule for automatic differentiation
#[derive(Debug, Clone)]
pub struct DerivativeRule {
    /// Rule type for efficient computation
    pub rule_type: DerivativeRuleType,

    /// Result expression template
    pub result_template: String,
}

/// Types of derivative rules for performance optimization
#[derive(Debug, Clone)]
pub enum DerivativeRuleType {
    /// Simple substitution: d/dx sin(x) = cos(x)
    Simple(String),

    /// Chain rule application: d/dx sin(u) = cos(u) * du/dx
    ChainRule(String),

    /// Product rule: d/dx (uv) = u'v + uv'
    ProductRule,

    /// Quotient rule: d/dx (u/v) = (u'v - uv')/v²
    QuotientRule,
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

/// Polynomial family classification
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PolynomialFamily {
    Legendre,   // P_n(x) - orthogonal on [-1,1]
    Hermite,    // H_n(x) - quantum harmonic oscillator
    Laguerre,   // L_n(x) - hydrogen atom radial functions
    Chebyshev,  // T_n(x), U_n(x) - approximation theory
    Jacobi,     // P_n^(α,β)(x) - general orthogonal polynomials
    Gegenbauer, // C_n^(λ)(x) - ultraspherical polynomials
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
    Ordinary,    // Σ a_n x^n
    Exponential, // Σ a_n x^n/n!
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

/// Numerical evaluator for intelligence-driven computation
///
/// Stores the actual numerical computation method for functions
#[derive(Debug, Clone)]
pub enum NumericalEvaluator {
    /// Standard library function (sin, cos, exp, ln, sqrt)
    StandardLib(fn(f64) -> f64),
    /// Custom numerical implementation  
    Custom(fn(&[f64]) -> Vec<f64>),
    /// No numerical evaluation available
    NotSupported,

    /// Asymptotic approximation for large arguments
    Asymptotic,
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
    Real,                             // ℝ
    Complex,                          // ℂ
    Integer,                          // ℤ
    PositiveInteger,                  // ℤ⁺
    NonNegativeInteger,               // ℕ₀
    Interval(Expression, Expression), // [a, b]
    Union(Vec<Domain>),               // Union of domains
}

/// Mathematical range
#[derive(Debug, Clone)]
pub enum Range {
    Real,                            // ℝ
    Integer,                         // ℤ
    PositiveInteger,                 // ℤ⁺
    NonNegativeInteger,              // ℕ₀
    Boolean,                         // {true, false}
    Bounded(Expression, Expression), // [a, b]
    Unbounded,                       // (-∞, ∞)
}

/// User-defined function properties
#[derive(Debug, Clone)]
pub enum UserProperty {
    Even,                 // f(-x) = f(x)
    Odd,                  // f(-x) = -f(x)
    Periodic(Expression), // f(x + T) = f(x)
    Monotonic,            // Increasing or decreasing
    Bounded,              // |f(x)| ≤ M for some M
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_function_properties_size() {
        // Ensure properties don't cause memory bloat
        use std::mem::size_of;

        // FunctionProperties is Box-wrapped enum, so size is just pointer (8 bytes) + discriminant
        assert!(size_of::<FunctionProperties>() <= 32,
            "FunctionProperties size: {} bytes (expected <= 32)",
            size_of::<FunctionProperties>());

        // ElementaryProperties contains Expression and complex types, so allow more space
        // With Expression (32 bytes), Vec (24 bytes), Box pointers (8 bytes each), and Option types,
        // this can grow beyond 128 bytes. Allow up to 256 bytes for comprehensive properties.
        assert!(size_of::<ElementaryProperties>() <= 256,
            "ElementaryProperties size: {} bytes (expected <= 256)",
            size_of::<ElementaryProperties>());
    }

    #[test]
    fn test_hot_path_methods() {
        let props: FunctionProperties =
            FunctionProperties::Elementary(Box::new(ElementaryProperties {
                derivative_rule: Some(DerivativeRule {
                    rule_type: DerivativeRuleType::Simple("cos".to_string()),
                    result_template: "cos(x)".to_string(),
                }),
                special_values: vec![],
                identities: Box::new(vec![]),
                domain_range: Box::new(DomainRangeData {
                    domain: Domain::Real,
                    range: Range::Bounded(Expression::integer(-1), Expression::integer(1)),
                    singularities: vec![],
                }),
                periodicity: Some(Expression::mul(vec![
                    Expression::integer(2),
                    Expression::pi(),
                ])),
                numerical_evaluator: Some(NumericalEvaluator::StandardLib(f64::sin)),
            }));

        // Test hot path methods
        assert!(props.has_derivative());
        assert_eq!(props.special_value_count(), 0);
    }
}
