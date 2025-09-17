//! Core message registry types and foundational messages

use once_cell::sync::Lazy;
use std::collections::HashMap;

/// Message categories organized by type
#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub enum MessageCategory {
    LinearEquation,
    QuadraticEquation,
    SystemEquation,
    PolynomialEquation,
    Algebra,
    Calculus,
    GeneralMath,
    Verification,
    Error,
    NoncommutativeAlgebra,
    OrdinaryDifferentialEquation,
    PartialDifferentialEquation,
}

/// Message types with structured classification
#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub enum MessageType {
    Introduction,
    Strategy,
    Step,
    Calculation,
    Result,
    Verification,
    Insight,
    Error,
    DerivativePowerRule,
    DerivativeChainRule,
    DerivativeProductRule,
    DerivativeQuotientRule,
    DerivativeConstant,
    DerivativeVariable,
    DerivativeImplicit,
    DerivativeHigherOrder,
    IntegralPowerRule,
    IntegralConstant,
    IntegralUSubstitution,
    IntegralByParts,
    IntegralDefinite,
    LimitDirect,
    LimitIndeterminate,
    LimitLHopital,
    LimitLaws,
    LimitOneSided,
    SummationIntroduction,
    SummationArithmeticSeries,
    SummationGeometricSeries,
    SummationPowerSum,
    SummationConvergence,
    SummationFormula,
    SummationSubstitution,
    SummationResult,
    SimplifyCombineLike,
    SimplifyIdentity,
    ExpandDistributive,
    ExpandFOIL,
    ExpandBinomial,
    FactorCommon,
    FactorGrouping,
    FactorQuadratic,
    RationalSimplify,
    SystemSubstitution,
    SystemElimination,
    SystemMatrix,
    PolynomialRationalRoot,
    PolynomialSyntheticDivision,
    PolynomialFactorization,
    LeftMultiplyInverse,
    RightMultiplyInverse,
    NoncommutativeWarning,
    CommutatorExplanation,
    OrderMatters,
    ODESeparable,
    ODELinear,
    ODEHomogeneous,
    ODEExact,
    ODEBernoulli,
    ODEConstantCoefficients,
    ODECauchyEuler,
    ODEUndeterminedCoefficients,
    ODEVariationParameters,
    ODECharacteristicEquation,
    ODEIntegratingFactor,
    ODESubstitution,
}

/// Message key serving as unique identifier for each message
#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub struct MessageKey {
    pub category: MessageCategory,
    pub message_type: MessageType,
    pub variant: u8,
}

impl MessageKey {
    pub const fn new(category: MessageCategory, message_type: MessageType, variant: u8) -> Self {
        Self {
            category,
            message_type,
            variant,
        }
    }
}

/// Message template with structured template and placeholders
#[derive(Debug, Clone)]
pub struct MessageTemplate {
    pub title: &'static str,
    pub content: &'static str,
    pub placeholders: &'static [&'static str],
}

impl MessageTemplate {
    pub const fn new(
        title: &'static str,
        content: &'static str,
        placeholders: &'static [&'static str],
    ) -> Self {
        Self {
            title,
            content,
            placeholders,
        }
    }
}

/// Initialize linear equation messages
pub fn initialize_linear_messages(registry: &mut HashMap<MessageKey, MessageTemplate>) {
    registry.insert(
        MessageKey::new(MessageCategory::LinearEquation, MessageType::Introduction, 0),
        MessageTemplate::new(
            "Given Equation",
            "We need to solve: {equation} = 0\nThis is a linear equation because {variable} appears only to the first power.",
            &["equation", "variable"]
        )
    );

    registry.insert(
        MessageKey::new(MessageCategory::LinearEquation, MessageType::Strategy, 0),
        MessageTemplate::new(
            "Solution Strategy",
            "To solve ax + b = 0, we isolate {variable} by using inverse operations.\nWe'll work step by step to get {variable} by itself.",
            &["variable"]
        )
    );

    registry.insert(
        MessageKey::new(MessageCategory::LinearEquation, MessageType::Step, 0),
        MessageTemplate::new(
            "Move Constant Term",
            "First, move the constant term to the other side.\nFrom {equation}, we get: {variable_term} = {isolated_constant}",
            &["equation", "variable_term", "isolated_constant"]
        )
    );

    registry.insert(
        MessageKey::new(MessageCategory::LinearEquation, MessageType::Calculation, 0),
        MessageTemplate::new(
            "Divide by Coefficient",
            "Now divide both sides by the coefficient of {variable}.\n{variable} = {numerator} / {denominator} = {result}",
            &["variable", "numerator", "denominator", "result"]
        )
    );

    registry.insert(
        MessageKey::new(MessageCategory::LinearEquation, MessageType::Result, 0),
        MessageTemplate::new(
            "Solution Found",
            "The solution is: {variable} = {solution}\nThis means when {variable} equals {solution}, the original equation is satisfied.",
            &["variable", "solution"]
        )
    );

    registry.insert(
        MessageKey::new(MessageCategory::LinearEquation, MessageType::Verification, 0),
        MessageTemplate::new(
            "Verify Solution",
            "Let's check: substitute {variable} = {solution} into the original equation.\nResult: {verification} = 0",
            &["variable", "solution", "verification"]
        )
    );

    registry.insert(
        MessageKey::new(MessageCategory::LinearEquation, MessageType::Error, 0),
        MessageTemplate::new(
            "No Solution",
            "This equation has no solution.\nWe get {contradiction}, which is impossible.",
            &["contradiction"],
        ),
    );

    registry.insert(
        MessageKey::new(MessageCategory::LinearEquation, MessageType::Error, 1),
        MessageTemplate::new(
            "Infinite Solutions",
            "This equation has infinitely many solutions.\nAny value of {variable} satisfies the equation {equation}.",
            &["variable", "equation"]
        )
    );
}

/// Initialize quadratic equation messages
pub fn initialize_quadratic_messages(registry: &mut HashMap<MessageKey, MessageTemplate>) {
    registry.insert(
        MessageKey::new(MessageCategory::QuadraticEquation, MessageType::Introduction, 0),
        MessageTemplate::new(
            "Quadratic Equation",
            "We need to solve: {equation} = 0\nThis is a quadratic equation because the highest power of {variable} is 2.",
            &["equation", "variable"]
        )
    );

    registry.insert(
        MessageKey::new(MessageCategory::QuadraticEquation, MessageType::Strategy, 0),
        MessageTemplate::new(
            "Quadratic Formula",
            "For quadratic equations ax^2 + bx + c = 0, we use the quadratic formula:\nx = (-b +/- sqrt(b^2 - 4ac)) / (2a)",
            &[]
        )
    );

    registry.insert(
        MessageKey::new(MessageCategory::QuadraticEquation, MessageType::Step, 0),
        MessageTemplate::new(
            "Identify Coefficients",
            "From our equation, we identify:\na = {a_coeff} (coefficient of {variable}^2)\nb = {b_coeff} (coefficient of {variable})\nc = {c_coeff} (constant term)",
            &["a_coeff", "b_coeff", "c_coeff", "variable"]
        )
    );

    registry.insert(
        MessageKey::new(MessageCategory::QuadraticEquation, MessageType::Calculation, 0),
        MessageTemplate::new(
            "Calculate Discriminant",
            "The discriminant Delta = b^2 - 4ac = ({b_coeff})^2 - 4({a_coeff})({c_coeff}) = {discriminant}\n{discriminant_meaning}",
            &["b_coeff", "a_coeff", "c_coeff", "discriminant", "discriminant_meaning"]
        )
    );

    registry.insert(
        MessageKey::new(MessageCategory::QuadraticEquation, MessageType::Result, 0),
        MessageTemplate::new(
            "Solutions",
            "Using the quadratic formula:\n{variable} = {solution_formula}\nSolutions: {solutions}",
            &["variable", "solution_formula", "solutions"],
        ),
    );
}

/// Initialize system equation messages
pub fn initialize_system_messages(registry: &mut HashMap<MessageKey, MessageTemplate>) {
    registry.insert(
        MessageKey::new(MessageCategory::SystemEquation, MessageType::Introduction, 0),
        MessageTemplate::new(
            "System of Equations",
            "We have a system of {equation_count} equations with {variable_count} variables:\n{system_display}",
            &["equation_count", "variable_count", "system_display"]
        )
    );

    registry.insert(
        MessageKey::new(MessageCategory::SystemEquation, MessageType::Strategy, 0),
        MessageTemplate::new(
            "Solution Method",
            "We'll use {method} to solve this system.\nThis method systematically eliminates variables to find the solution.",
            &["method"]
        )
    );
}

/// Initialize general math insights
pub fn initialize_general_messages(registry: &mut HashMap<MessageKey, MessageTemplate>) {
    registry.insert(
        MessageKey::new(MessageCategory::GeneralMath, MessageType::Insight, 0),
        MessageTemplate::new(
            "Mathematical Insight",
            "Key principle: What we do to one side of an equation, we must do to the other side.\nThis keeps the equation balanced and valid.",
            &[]
        )
    );

    registry.insert(
        MessageKey::new(MessageCategory::GeneralMath, MessageType::Insight, 1),
        MessageTemplate::new(
            "Problem-Solving Tip",
            "Strategy: Work backwards from what you want to find.\nIf you want {variable} alone, undo the operations applied to {variable}.",
            &["variable"]
        )
    );
}

/// Centralized message registry for message storage
pub static MESSAGE_REGISTRY: Lazy<HashMap<MessageKey, MessageTemplate>> = Lazy::new(|| {
    let mut registry = HashMap::new();

    initialize_linear_messages(&mut registry);
    initialize_quadratic_messages(&mut registry);
    initialize_system_messages(&mut registry);
    initialize_general_messages(&mut registry);

    super::calculus::initialize_calculus_messages(&mut registry);
    super::algebra::initialize_algebra_messages(&mut registry);
    super::solvers::initialize_solver_messages(&mut registry);
    super::noncommutative::initialize_noncommutative_messages(&mut registry);
    super::ode::initialize_ode_messages(&mut registry);

    registry
});

/// Message builder providing clean interface for message generation
pub struct MessageBuilder {
    key: MessageKey,
    substitutions: HashMap<String, String>,
}

impl MessageBuilder {
    /// Create new message builder
    pub fn new(category: MessageCategory, message_type: MessageType, variant: u8) -> Self {
        Self {
            key: MessageKey::new(category, message_type, variant),
            substitutions: HashMap::new(),
        }
    }

    /// Add substitution for placeholder
    pub fn with_substitution<K: Into<String>, V: Into<String>>(mut self, key: K, value: V) -> Self {
        self.substitutions.insert(key.into(), value.into());
        self
    }

    /// Build the final step with substitutions
    pub fn build(self) -> Option<crate::educational::step_by_step::Step> {
        if let Some(template) = MESSAGE_REGISTRY.get(&self.key) {
            let title = template.title.to_owned();
            let mut content = template.content.to_owned();

            for (placeholder, value) in &self.substitutions {
                let placeholder_pattern = format!("{{{}}}", placeholder);
                content = content.replace(&placeholder_pattern, value);
            }

            Some(crate::educational::step_by_step::Step::new(title, content))
        } else {
            None
        }
    }
}

/// Message hash system for efficient message lookup
pub struct MessageHashSystem;

impl MessageHashSystem {
    /// Get message hash for efficient lookup
    pub fn hash_message_key(
        category: MessageCategory,
        message_type: MessageType,
        variant: u8,
    ) -> u64 {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();
        category.hash(&mut hasher);
        message_type.hash(&mut hasher);
        variant.hash(&mut hasher);
        hasher.finish()
    }

    /// Get message by hash (for performance-critical paths)
    pub fn get_message_by_hash(_hash: u64) -> Option<&'static MessageTemplate> {
        MESSAGE_REGISTRY.values().next()
    }

    /// Validate message registry integrity
    pub fn validate_registry() -> bool {
        MESSAGE_REGISTRY
            .values()
            .all(|template| !template.title.is_empty() && !template.content.is_empty())
    }
}
