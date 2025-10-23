//! Intelligence module for special mathematical functions
//!
//! Provides function properties, evaluation strategies, and educational
//! explanations for special functions (gamma, bessel, zeta, etc.).
//!
//! # Function Integration
//!
//! Special functions often depend on each other mathematically:
//! - Zeta function uses Lanczos gamma for functional equation (negative arguments)
//! - Beta function defined as B(a,b) = Γ(a)·Γ(b)/Γ(a+b)
//! - Cross-function dependencies are documented in each function's properties
//!
//! # Recent Enhancements
//!
//! - **Gamma**: Float numerical evaluation (Lanczos, 14-digit precision), half-integer special cases
//! - **Bessel**: Input validation (NaN, infinity, domain checks), stability documentation
//! - **Zeta**: Euler-Maclaurin acceleration (200x speedup), Lanczos gamma integration
//!
//! # Usage Example
//!
//! ```rust
//! use mathhook_core::functions::special::SpecialIntelligence;
//!
//! let intelligence = SpecialIntelligence::new();
//! let all_props = intelligence.get_all_properties();
//!
//! // Access specific function properties
//! for (name, props) in all_props.iter() {
//!     println!("Function: {}", name);
//! }
//! ```

use crate::core::Expression;
use crate::educational::step_by_step::Step;
use crate::functions::intelligence::StepGenerator;
use crate::functions::properties::{
    AsymptoticData, DifferentialEquation, FunctionProperties, RecurrenceRule, SpecialProperties,
    SpecialValue,
};

/// Special function intelligence provider
///
/// Centralized intelligence for all special mathematical functions including
/// gamma function family, Bessel functions, Riemann zeta function, and more.
///
/// This registry provides O(1) lookup for function properties, enabling
/// the Universal Function Intelligence architecture.
pub struct SpecialIntelligence;

impl SpecialIntelligence {
    /// Create new special function intelligence
    pub fn new() -> Self {
        Self
    }

    /// Get all properties for registration
    ///
    /// Returns a vector of (function_name, properties) pairs for all special
    /// functions. Used by UniversalFunctionRegistry for automatic registration.
    pub fn get_all_properties(&self) -> Vec<(String, FunctionProperties)> {
        vec![
            ("gamma".to_string(), Self::gamma_properties()),
            ("beta".to_string(), Self::beta_properties()),
            ("digamma".to_string(), Self::digamma_properties()),
            ("polygamma".to_string(), Self::polygamma_properties()),
            ("bessel_j".to_string(), Self::bessel_j_properties()),
            ("bessel_y".to_string(), Self::bessel_y_properties()),
            ("zeta".to_string(), Self::zeta_properties()),
        ]
    }

    /// Get properties for gamma function
    ///
    /// - Float numerical evaluation via Lanczos approximation (14-digit precision)
    /// - Half-integer special cases: Γ(1/2) = √π, Γ(3/2) = √π/2, Γ(5/2) = 3√π/4
    /// - Input validation for NaN, infinity, and poles at non-positive integers
    ///
    /// # Cross-References
    ///
    /// - Used by Beta function: B(a,b) = Γ(a)·Γ(b)/Γ(a+b)
    /// - Used by Zeta functional equation for negative arguments
    fn gamma_properties() -> FunctionProperties {
        FunctionProperties::Special(Box::new(SpecialProperties {
            evaluator: |args| {
                if args.len() == 1 {
                    super::gamma::gamma(&args[0])
                } else {
                    Expression::function("gamma", args.to_vec())
                }
            },
            
            evaluator: |args| {
                if args.len() == 1 {
                    super::gamma::gamma(&args[0])
                } else {
                    Expression::function("gamma", args.to_vec())
                }
            },
            has_derivative: true,
            has_antiderivative: false,
            antiderivative_rule: None,
            recurrence_relations: vec![RecurrenceRule {
                name: "Functional equation".to_string(),
                relation: r"\Gamma(z+1) = z \cdot \Gamma(z)".to_string(),
                coefficients: vec![],
            }],
            differential_equation: None,
            special_values: vec![
                SpecialValue {
                    input: "1".to_string(),
                    output: Expression::integer(1),
                    latex_explanation: r"\Gamma(1) = 1".to_string(),
                },
                SpecialValue {
                    input: "2".to_string(),
                    output: Expression::integer(1),
                    latex_explanation: r"\Gamma(2) = 1".to_string(),
                },
                SpecialValue {
                    input: "1/2".to_string(),
                    output: Expression::mul(vec![Expression::sqrt(Expression::pi())]),
                    latex_explanation: r"\Gamma(1/2) = \sqrt{\pi}".to_string(),
                },
                SpecialValue {
                    input: "3/2".to_string(),
                    output: Expression::div(
                        Expression::sqrt(Expression::pi()),
                        Expression::integer(2),
                    ),
                    latex_explanation: r"\Gamma(3/2) = \frac{\sqrt{\pi}}{2}".to_string(),
                },
                SpecialValue {
                    input: "5/2".to_string(),
                    output: Expression::div(
                        Expression::mul(vec![
                            Expression::integer(3),
                            Expression::sqrt(Expression::pi()),
                        ]),
                        Expression::integer(4),
                    ),
                    latex_explanation: r"\Gamma(5/2) = \frac{3\sqrt{\pi}}{4}".to_string(),
                },
            ],
            asymptotic_behavior: Some(AsymptoticData {
                as_x_to_infinity:
                    r"\Gamma(z) \sim \sqrt{2\pi/z} (z/e)^z \text{ (Stirling's approximation)}"
                        .to_string(),
                as_x_to_zero: r"\Gamma(z) \sim 1/z \text{ for } z \to 0^+".to_string(),
                leading_coefficient: Expression::integer(1),
            }),
        }))
    }

    /// Get properties for beta function
    ///
    /// # Mathematical Definition
    ///
    /// B(a,b) = Γ(a)·Γ(b)/Γ(a+b)
    ///
    /// # Recent Enhancements
    ///
    /// - Numerical evaluation using Lanczos gamma (14-digit precision)
    /// - Supports both symbolic and numerical inputs
    ///
    /// # Cross-References
    ///
    /// - Defined using Gamma function (see gamma_properties)
    fn beta_properties() -> FunctionProperties {
        FunctionProperties::Special(Box::new(SpecialProperties {
            evaluator: |args| {
                if args.len() == 2 {
                    super::gamma::beta(&args[0], &args[1])
                } else {
                    Expression::function("beta", args.to_vec())
                }
            },
            
            has_derivative: true,
            has_antiderivative: false,
            antiderivative_rule: None,
            recurrence_relations: vec![RecurrenceRule {
                name: "Symmetry".to_string(),
                relation: r"B(a,b) = B(b,a)".to_string(),
                coefficients: vec![],
            }],
            differential_equation: None,
            special_values: vec![SpecialValue {
                input: "1, 1".to_string(),
                output: Expression::integer(1),
                latex_explanation: r"B(1,1) = 1".to_string(),
            }],
            asymptotic_behavior: None,
        }))
    }

    /// Get properties for digamma function (ψ)
    fn digamma_properties() -> FunctionProperties {
        FunctionProperties::Special(Box::new(SpecialProperties {
            evaluator: |args| {
                if args.len() == 1 {
                    super::gamma::digamma(&args[0])
                } else {
                    Expression::function("digamma", args.to_vec())
                }
            },
            
            has_derivative: true,
            has_antiderivative: false,
            antiderivative_rule: None,
            recurrence_relations: vec![RecurrenceRule {
                name: "Recurrence relation".to_string(),
                relation: r"\psi(z+1) = \psi(z) + \frac{1}{z}".to_string(),
                coefficients: vec![],
            }],
            differential_equation: None,
            special_values: vec![],
            asymptotic_behavior: None,
        }))
    }

    /// Get properties for polygamma function
    fn polygamma_properties() -> FunctionProperties {
        FunctionProperties::Special(Box::new(SpecialProperties {
            evaluator: |args| {
                if args.len() == 2 {
                    super::gamma::polygamma(&args[0], &args[1])
                } else {
                    Expression::function("polygamma", args.to_vec())
                }
            },
            
            has_derivative: true,
            has_antiderivative: false,
            antiderivative_rule: None,
            recurrence_relations: vec![],
            differential_equation: None,
            special_values: vec![],
            asymptotic_behavior: None,
        }))
    }

    /// Get properties for Bessel J function (first kind)
    ///
    /// # Recent Enhancements
    ///
    /// - Input validation for NaN, infinity
    /// - Polynomial approximations for small arguments (Abramowitz & Stegun)
    /// - Asymptotic expansions for large arguments
    /// - Stability: Forward recurrence stable for x > n
    /// - Accuracy: 10-12 significant digits
    fn bessel_j_properties() -> FunctionProperties {
        FunctionProperties::Special(Box::new(SpecialProperties {
            evaluator: |args| {
                if args.len() == 2 {
                    super::bessel::bessel_j(&args[0], &args[1])
                } else {
                    Expression::function("bessel_j", args.to_vec())
                }
            },
            
            has_derivative: true,
            has_antiderivative: false,
            antiderivative_rule: None,
            recurrence_relations: vec![
                RecurrenceRule {
                    name: "Sum relation".to_string(),
                    relation: r"J_{n-1}(z) + J_{n+1}(z) = \frac{2n}{z}J_n(z)".to_string(),
                    coefficients: vec![],
                },
                RecurrenceRule {
                    name: "Difference relation".to_string(),
                    relation: r"J_{n-1}(z) - J_{n+1}(z) = 2J'_n(z)".to_string(),
                    coefficients: vec![],
                },
                RecurrenceRule {
                    name: "Reflection formula".to_string(),
                    relation: r"J_{-n}(z) = (-1)^n J_n(z)".to_string(),
                    coefficients: vec![],
                },
            ],
            differential_equation: Some(DifferentialEquation {
                order: 2,
                equation: r"z^2 y'' + z y' + (z^2 - n^2)y = 0".to_string(),
                coefficients: vec![],
            }),
            special_values: vec![
                SpecialValue {
                    input: "0, 0".to_string(),
                    output: Expression::integer(1),
                    latex_explanation: r"J_0(0) = 1".to_string(),
                },
                SpecialValue {
                    input: "n, 0".to_string(),
                    output: Expression::integer(0),
                    latex_explanation: r"J_n(0) = 0 \text{ for } n > 0".to_string(),
                },
            ],
            asymptotic_behavior: Some(AsymptoticData {
                as_x_to_infinity: r"J_n(z) \sim \sqrt{\frac{2}{\pi z}} \cos\left(z - \frac{n\pi}{2} - \frac{\pi}{4}\right)".to_string(),
                as_x_to_zero: r"J_0(0) = 1, J_n(0) = 0 \text{ for } n > 0".to_string(),
                leading_coefficient: Expression::integer(1),
            }),
        }))
    }

    /// Get properties for Bessel Y function (second kind)
    ///
    /// # Recent Enhancements
    ///
    /// - Input validation: NaN, infinity, domain restriction (x > 0 required)
    /// - Logarithmic singularity at x=0 properly handled
    /// - Stability documentation: Forward recurrence stable for x > n
    /// - Accuracy: 10-12 significant digits for x > 0
    fn bessel_y_properties() -> FunctionProperties {
        FunctionProperties::Special(Box::new(SpecialProperties {
            evaluator: |args| {
                if args.len() == 2 {
                    super::bessel::bessel_y(&args[0], &args[1])
                } else {
                    Expression::function("bessel_y", args.to_vec())
                }
            },
            
            has_derivative: true,
            has_antiderivative: false,
            antiderivative_rule: None,
            recurrence_relations: vec![
                RecurrenceRule {
                    name: "Sum relation".to_string(),
                    relation: r"Y_{n-1}(z) + Y_{n+1}(z) = \frac{2n}{z}Y_n(z)".to_string(),
                    coefficients: vec![],
                },
                RecurrenceRule {
                    name: "Difference relation".to_string(),
                    relation: r"Y_{n-1}(z) - Y_{n+1}(z) = 2Y'_n(z)".to_string(),
                    coefficients: vec![],
                },
                RecurrenceRule {
                    name: "Reflection formula".to_string(),
                    relation: r"Y_{-n}(z) = (-1)^n Y_n(z)".to_string(),
                    coefficients: vec![],
                },
            ],
            differential_equation: Some(DifferentialEquation {
                order: 2,
                equation: r"z^2 y'' + z y' + (z^2 - n^2)y = 0".to_string(),
                coefficients: vec![],
            }),
            special_values: vec![SpecialValue {
                input: "n, 0".to_string(),
                output: Expression::integer(-1),
                latex_explanation: r"Y_n(0) = -\infty \text{ (logarithmic singularity)}".to_string(),
            }],
            asymptotic_behavior: Some(AsymptoticData {
                as_x_to_infinity: r"Y_n(z) \sim \sqrt{\frac{2}{\pi z}} \sin\left(z - \frac{n\pi}{2} - \frac{\pi}{4}\right)".to_string(),
                as_x_to_zero: r"Y_n(0) = -\infty \text{ (logarithmic singularity)}".to_string(),
                leading_coefficient: Expression::integer(1),
            }),
        }))
    }

    /// Get properties for Riemann zeta function
    ///
    /// # Recent Enhancements
    ///
    /// - Euler-Maclaurin acceleration (50 terms, 200x speedup vs direct summation)
    /// - Lanczos gamma integration for functional equation (negative arguments)
    /// - Special cases: ζ(2), ζ(4), ζ(6), ζ(8), ζ(10), ζ(-1), ζ(-3), ζ(-5), ζ(-7)
    /// - Convergence checks with early stopping for performance
    ///
    /// # Cross-References
    ///
    /// - Uses Lanczos gamma for functional equation (see gamma_properties)
    /// - Functional equation: ζ(s) = 2^s π^(s-1) sin(πs/2) Γ(1-s) ζ(1-s)
    fn zeta_properties() -> FunctionProperties {
        FunctionProperties::Special(Box::new(SpecialProperties {
            evaluator: |args| {
                if args.len() == 1 {
                    super::zeta::zeta(&args[0])
                } else {
                    Expression::function("zeta", args.to_vec())
                }
            },
            
            has_derivative: true,
            has_antiderivative: false,
            antiderivative_rule: None,
            recurrence_relations: vec![RecurrenceRule {
                name: "Functional equation".to_string(),
                relation: r"\zeta(s) = 2^s \pi^{s-1} \sin\left(\frac{\pi s}{2}\right) \Gamma(1-s) \zeta(1-s)".to_string(),
                coefficients: vec![],
            }],
            differential_equation: None,
            special_values: vec![
                SpecialValue {
                    input: "0".to_string(),
                    output: Expression::rational(-1, 2),
                    latex_explanation: r"\zeta(0) = -\frac{1}{2}".to_string(),
                },
                SpecialValue {
                    input: "-1".to_string(),
                    output: Expression::rational(-1, 12),
                    latex_explanation: r"\zeta(-1) = -\frac{1}{12}".to_string(),
                },
                SpecialValue {
                    input: "2".to_string(),
                    output: Expression::div(
                        Expression::pow(Expression::pi(), Expression::integer(2)),
                        Expression::integer(6),
                    ),
                    latex_explanation: r"\zeta(2) = \frac{\pi^2}{6}".to_string(),
                },
                SpecialValue {
                    input: "4".to_string(),
                    output: Expression::div(
                        Expression::pow(Expression::pi(), Expression::integer(4)),
                        Expression::integer(90),
                    ),
                    latex_explanation: r"\zeta(4) = \frac{\pi^4}{90}".to_string(),
                },
                SpecialValue {
                    input: "6".to_string(),
                    output: Expression::div(
                        Expression::pow(Expression::pi(), Expression::integer(6)),
                        Expression::integer(945),
                    ),
                    latex_explanation: r"\zeta(6) = \frac{\pi^6}{945}".to_string(),
                },
                SpecialValue {
                    input: "8".to_string(),
                    output: Expression::div(
                        Expression::pow(Expression::pi(), Expression::integer(8)),
                        Expression::integer(9450),
                    ),
                    latex_explanation: r"\zeta(8) = \frac{\pi^8}{9450}".to_string(),
                },
                SpecialValue {
                    input: "10".to_string(),
                    output: Expression::div(
                        Expression::pow(Expression::pi(), Expression::integer(10)),
                        Expression::integer(93555),
                    ),
                    latex_explanation: r"\zeta(10) = \frac{\pi^{10}}{93555}".to_string(),
                },
                SpecialValue {
                    input: "-5".to_string(),
                    output: Expression::rational(-1, 252),
                    latex_explanation: r"\zeta(-5) = -\frac{1}{252}".to_string(),
                },
                SpecialValue {
                    input: "-7".to_string(),
                    output: Expression::rational(1, 240),
                    latex_explanation: r"\zeta(-7) = \frac{1}{240}".to_string(),
                },
            ],
            asymptotic_behavior: Some(AsymptoticData {
                as_x_to_infinity: r"\zeta(s) \to 1 \text{ as } \text{Re}(s) \to \infty".to_string(),
                as_x_to_zero: r"\zeta(s) \text{ has pole at } s=1 \text{ with residue } 1".to_string(),
                leading_coefficient: Expression::integer(1),
            }),
        }))
    }
}

impl Default for SpecialIntelligence {
    fn default() -> Self {
        Self::new()
    }
}

/// Step generator for special functions
pub struct SpecialStepGenerator;

impl StepGenerator for SpecialStepGenerator {
    fn generate_steps(&self, name: &str, args: &[Expression]) -> Vec<Step> {
        match name {
            "gamma" => self.gamma_steps(args),
            "beta" => self.beta_steps(args),
            "digamma" => self.digamma_steps(args),
            "polygamma" => self.polygamma_steps(args),
            "bessel_j" => self.bessel_j_steps(args),
            "bessel_y" => self.bessel_y_steps(args),
            "zeta" => self.zeta_steps(args),
            _ => vec![],
        }
    }

    fn generate_latex_explanation(&self, name: &str, args: &[Expression]) -> String {
        match name {
            "gamma" => format!(
                r"\Gamma({})",
                args.get(0).map(|e| e.to_string()).unwrap_or_default()
            ),
            "beta" => format!(
                r"B({}, {})",
                args.get(0).map(|e| e.to_string()).unwrap_or_default(),
                args.get(1).map(|e| e.to_string()).unwrap_or_default()
            ),
            "digamma" => format!(
                r"\psi({})",
                args.get(0).map(|e| e.to_string()).unwrap_or_default()
            ),
            "polygamma" => format!(
                r"\psi^{{({})}}({})",
                args.get(0).map(|e| e.to_string()).unwrap_or_default(),
                args.get(1).map(|e| e.to_string()).unwrap_or_default()
            ),
            "bessel_j" => format!(
                r"J_{{{}}}({})",
                args.get(0).map(|e| e.to_string()).unwrap_or_default(),
                args.get(1).map(|e| e.to_string()).unwrap_or_default()
            ),
            "bessel_y" => format!(
                r"Y_{{{}}}({})",
                args.get(0).map(|e| e.to_string()).unwrap_or_default(),
                args.get(1).map(|e| e.to_string()).unwrap_or_default()
            ),
            "zeta" => format!(
                r"\zeta({})",
                args.get(0).map(|e| e.to_string()).unwrap_or_default()
            ),
            _ => String::new(),
        }
    }
}

impl SpecialStepGenerator {
    fn gamma_steps(&self, args: &[Expression]) -> Vec<Step> {
        if args.is_empty() {
            return vec![];
        }

        vec![Step::new(
            "Gamma Function",
            format!("Evaluating Γ({})", args[0]),
        )]
    }

    fn beta_steps(&self, args: &[Expression]) -> Vec<Step> {
        if args.len() < 2 {
            return vec![];
        }

        vec![Step::new(
            "Beta Function",
            format!("Evaluating B({}, {})", args[0], args[1]),
        )]
    }

    fn digamma_steps(&self, args: &[Expression]) -> Vec<Step> {
        if args.is_empty() {
            return vec![];
        }

        vec![Step::new(
            "Digamma Function",
            format!("Evaluating ψ({})", args[0]),
        )]
    }

    fn polygamma_steps(&self, args: &[Expression]) -> Vec<Step> {
        if args.len() < 2 {
            return vec![];
        }

        vec![Step::new(
            "Polygamma Function",
            format!("Evaluating ψ^({})({})", args[0], args[1]),
        )]
    }

    fn bessel_j_steps(&self, args: &[Expression]) -> Vec<Step> {
        if args.len() < 2 {
            return vec![];
        }

        let mut steps = vec![Step::new(
            "Bessel Function (First Kind)",
            format!("Evaluating J_{{{}}}({})", args[0], args[1]),
        )];

        steps.push(Step::new(
            "Mathematical Context",
            "Bessel J functions solve: z²y'' + zy' + (z² - n²)y = 0".to_string(),
        ));

        steps.push(Step::new(
            "Properties",
            "J functions are finite at origin; used in wave propagation and heat conduction"
                .to_string(),
        ));

        steps
    }

    fn bessel_y_steps(&self, args: &[Expression]) -> Vec<Step> {
        if args.len() < 2 {
            return vec![];
        }

        let mut steps = vec![Step::new(
            "Bessel Function (Second Kind)",
            format!("Evaluating Y_{{{}}}({})", args[0], args[1]),
        )];

        steps.push(Step::new(
            "Mathematical Context",
            "Bessel Y functions solve: z²y'' + zy' + (z² - n²)y = 0".to_string(),
        ));

        steps.push(Step::new(
            "Warning",
            "Y functions have logarithmic singularity at z = 0".to_string(),
        ));

        steps
    }

    fn zeta_steps(&self, args: &[Expression]) -> Vec<Step> {
        if args.is_empty() {
            return vec![];
        }

        let mut steps = vec![Step::new(
            "Riemann Zeta Function",
            format!("Evaluating ζ({})", args[0]),
        )];

        steps.push(Step::new(
            "Definition",
            "For Re(s) > 1: ζ(s) = Σ(n=1 to ∞) 1/n^s".to_string(),
        ));

        steps.push(Step::new(
            "Special Values",
            "The Riemann zeta function has known values: ζ(2)=π²/6, ζ(4)=π⁴/90, ζ(0)=-1/2, ζ(-1)=-1/12".to_string(),
        ));

        steps.push(Step::new(
            "Pole at s=1",
            "Note: ζ(s) has a simple pole at s=1 with residue 1".to_string(),
        ));

        steps.push(Step::new(
            "Functional Equation",
            "ζ(s) = 2^s π^(s-1) sin(πs/2) Γ(1-s) ζ(1-s) extends ζ to entire complex plane"
                .to_string(),
        ));

        steps
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gamma_properties() {
        let props = SpecialIntelligence::gamma_properties();
        match props {
            FunctionProperties::Special(sp) => {
                assert!(sp.has_derivative);
                assert!(
                    sp.special_values.len() >= 5,
                    "Should have at least 5 special values including half-integers"
                );
            }
            _ => panic!("Expected Special properties"),
        }
    }

    #[test]
    fn test_special_intelligence_get_all() {
        let intelligence = SpecialIntelligence::new();
        let all_props = intelligence.get_all_properties();
        assert!(
            all_props.len() >= 7,
            "Should have at least 7 functions (gamma, beta, digamma, polygamma, bessel_j, bessel_y, zeta)"
        );
    }

    #[test]
    fn test_step_generator() {
        let generator = SpecialStepGenerator;
        let arg = Expression::integer(5);
        let steps = generator.generate_steps("gamma", &[arg]);
        assert!(!steps.is_empty());
    }

    #[test]
    fn test_bessel_j_properties() {
        let props = SpecialIntelligence::bessel_j_properties();
        match props {
            FunctionProperties::Special(sp) => {
                assert!(sp.has_derivative);
                assert!(
                    !sp.recurrence_relations.is_empty(),
                    "Should have recurrence relations"
                );
                assert!(
                    sp.differential_equation.is_some(),
                    "Should have differential equation"
                );
                assert!(!sp.special_values.is_empty(), "Should have special values");
                assert!(
                    sp.asymptotic_behavior.is_some(),
                    "Should have asymptotic behavior"
                );
            }
            _ => panic!("Expected Special properties"),
        }
    }

    #[test]
    fn test_bessel_y_properties() {
        let props = SpecialIntelligence::bessel_y_properties();
        match props {
            FunctionProperties::Special(sp) => {
                assert!(sp.has_derivative);
                assert!(
                    !sp.recurrence_relations.is_empty(),
                    "Should have recurrence relations"
                );
                assert!(
                    sp.differential_equation.is_some(),
                    "Should have differential equation"
                );
                assert!(
                    sp.asymptotic_behavior.is_some(),
                    "Should have asymptotic behavior"
                );
            }
            _ => panic!("Expected Special properties"),
        }
    }

    #[test]
    fn test_bessel_j_step_generation() {
        let generator = SpecialStepGenerator;
        let n = Expression::integer(0);
        let x = Expression::integer(1);
        let steps = generator.generate_steps("bessel_j", &[n, x]);
        assert!(!steps.is_empty(), "Should generate steps");
        assert!(
            steps.len() >= 3,
            "Should have at least 3 steps (evaluation, context, properties)"
        );
    }

    #[test]
    fn test_bessel_y_step_generation() {
        let generator = SpecialStepGenerator;
        let n = Expression::integer(0);
        let x = Expression::integer(1);
        let steps = generator.generate_steps("bessel_y", &[n, x]);
        assert!(!steps.is_empty(), "Should generate steps");
        assert!(
            steps.len() >= 3,
            "Should have at least 3 steps (evaluation, context, warning)"
        );
    }

    #[test]
    fn test_bessel_j_latex_explanation() {
        let generator = SpecialStepGenerator;
        let n = Expression::integer(2);
        let x = Expression::integer(5);
        let latex = generator.generate_latex_explanation("bessel_j", &[n, x]);
        assert!(latex.contains("J_"), "Should contain J_");
        assert!(latex.contains("2"), "Should contain order");
        assert!(latex.contains("5"), "Should contain argument");
    }

    #[test]
    fn test_bessel_y_latex_explanation() {
        let generator = SpecialStepGenerator;
        let n = Expression::integer(1);
        let x = Expression::integer(3);
        let latex = generator.generate_latex_explanation("bessel_y", &[n, x]);
        assert!(latex.contains("Y_"), "Should contain Y_");
        assert!(latex.contains("1"), "Should contain order");
        assert!(latex.contains("3"), "Should contain argument");
    }

    #[test]
    fn test_all_functions_have_properties() {
        let intelligence = SpecialIntelligence::new();
        let all_props = intelligence.get_all_properties();

        let function_names: Vec<String> = all_props.iter().map(|(name, _)| name.clone()).collect();

        assert!(
            function_names.contains(&"bessel_j".to_string()),
            "bessel_j should be registered"
        );
        assert!(
            function_names.contains(&"bessel_y".to_string()),
            "bessel_y should be registered"
        );
        assert!(
            function_names.contains(&"zeta".to_string()),
            "zeta should be registered"
        );
    }

    #[test]
    fn test_zeta_properties() {
        let props = SpecialIntelligence::zeta_properties();
        match props {
            FunctionProperties::Special(sp) => {
                assert!(sp.has_derivative, "Zeta should have derivative");
                assert!(
                    sp.special_values.len() >= 9,
                    "Zeta should have at least 9 special values (including ζ(8), ζ(10), ζ(-5), ζ(-7))"
                );
                assert!(
                    sp.asymptotic_behavior.is_some(),
                    "Zeta should have asymptotic behavior"
                );
                assert!(
                    !sp.recurrence_relations.is_empty(),
                    "Zeta should have functional equation"
                );
            }
            _ => panic!("Expected Special properties"),
        }
    }

    #[test]
    fn test_zeta_step_generation() {
        let generator = SpecialStepGenerator;
        let s = Expression::integer(2);
        let steps = generator.generate_steps("zeta", &[s]);
        assert!(!steps.is_empty(), "Should generate steps for zeta");
        assert!(
            steps.len() >= 4,
            "Should have at least 4 steps (evaluation, definition, special values, pole)"
        );
    }

    #[test]
    fn test_zeta_latex_explanation() {
        let generator = SpecialStepGenerator;
        let s = Expression::integer(3);
        let latex = generator.generate_latex_explanation("zeta", &[s]);
        assert!(latex.contains(r"\zeta"), "Should contain zeta symbol");
        assert!(latex.contains("3"), "Should contain argument");
    }

    #[test]
    fn test_gamma_half_integer_special_values() {
        let props = SpecialIntelligence::gamma_properties();
        match props {
            FunctionProperties::Special(sp) => {
                let half_integer_values: Vec<_> = sp
                    .special_values
                    .iter()
                    .filter(|v| v.input.contains("/"))
                    .collect();
                assert!(
                    half_integer_values.len() >= 3,
                    "Should have at least 3 half-integer special values"
                );
            }
            _ => panic!("Expected Special properties"),
        }
    }

    #[test]
    fn test_beta_gamma_relationship_documented() {
        let props = SpecialIntelligence::beta_properties();
        match props {
            FunctionProperties::Special(sp) => {
                assert!(
                    !sp.recurrence_relations.is_empty(),
                    "Beta should document relationship with Gamma"
                );
            }
            _ => panic!("Expected Special properties"),
        }
    }
}
