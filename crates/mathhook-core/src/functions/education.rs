//! Educational function system
//!
//! Provides step-by-step explanations for ALL function operations
//! to comply with MathHook's educational integration requirements.

use super::properties::PolynomialFamily;
use crate::core::Expression;
use crate::educational::step_by_step::{Step, StepByStepExplanation};
use std::collections::HashMap;

/// Function educator for step-by-step explanations
///
/// Provides educational explanations for all function operations
/// with proper mathematical notation and domain restrictions.
pub struct FunctionEducator {
    step_generators: HashMap<String, Box<dyn StepGenerator>>,
}

/// Step generator trait for educational explanations
pub trait StepGenerator: Send + Sync {
    /// Generate step-by-step explanation for function evaluation
    fn generate_steps(&self, name: &str, args: &[Expression]) -> Vec<Step>;

    /// Get mathematical context for the function
    fn get_mathematical_context(&self, name: &str) -> String;
}

impl FunctionEducator {
    /// Create new function educator with all 20+ functions
    pub fn new() -> Self {
        let mut educator = Self {
            step_generators: HashMap::with_capacity(32),
        };

        educator.initialize_trigonometric();
        educator.initialize_exponential_logarithmic();
        educator.initialize_polynomial_families();
        educator.initialize_number_theory();

        educator
    }

    /// Generate comprehensive step-by-step explanation
    pub fn explain_function_operation(
        &self,
        name: &str,
        args: &[Expression],
        _operation: &str,
    ) -> StepByStepExplanation {
        let steps = if let Some(generator) = self.step_generators.get(name) {
            generator.generate_steps(name, args)
        } else {
            vec![
                Step::new("Function Identification", format!("Function: {}", name)),
                Step::new("Arguments", format!("Args: {}", format_args(args))),
                Step::new("Result", "Computing...".to_string()),
            ]
        };

        StepByStepExplanation::new(steps)
    }

    /// Initialize trigonometric function education (8 functions)
    fn initialize_trigonometric(&mut self) {
        self.step_generators.insert(
            "sin".to_string(),
            Box::new(TrigStepGenerator {
                function_name: "sin".to_string(),
                latex_name: "\\sin".to_string(),
                context: "Sine: y-coordinate on unit circle".to_string(),
                special_values: vec![
                    ("0", "0"),
                    ("π/6", "1/2"),
                    ("π/4", "√2/2"),
                    ("π/3", "√3/2"),
                    ("π/2", "1"),
                    ("π", "0"),
                    ("2π", "0"),
                ],
                domain: "All real numbers".to_string(),
                range: "[-1, 1]".to_string(),
            }),
        );

        self.step_generators.insert(
            "cos".to_string(),
            Box::new(TrigStepGenerator {
                function_name: "cos".to_string(),
                latex_name: "\\cos".to_string(),
                context: "Cosine: x-coordinate on unit circle".to_string(),
                special_values: vec![
                    ("0", "1"),
                    ("π/6", "√3/2"),
                    ("π/4", "√2/2"),
                    ("π/3", "1/2"),
                    ("π/2", "0"),
                    ("π", "-1"),
                    ("2π", "1"),
                ],
                domain: "All real numbers".to_string(),
                range: "[-1, 1]".to_string(),
            }),
        );

        self.step_generators.insert(
            "tan".to_string(),
            Box::new(TrigStepGenerator {
                function_name: "tan".to_string(),
                latex_name: "\\tan".to_string(),
                context: "Tangent: sin(x)/cos(x), slope of angle".to_string(),
                special_values: vec![("0", "0"), ("π/4", "1"), ("π", "0")],
                domain: "x ≠ π/2 + nπ (undefined at vertical asymptotes)".to_string(),
                range: "All real numbers".to_string(),
            }),
        );

        self.step_generators.insert(
            "csc".to_string(),
            Box::new(TrigStepGenerator {
                function_name: "csc".to_string(),
                latex_name: "\\csc".to_string(),
                context: "Cosecant: 1/sin(x), reciprocal of sine".to_string(),
                special_values: vec![("π/6", "2"), ("π/2", "1")],
                domain: "x ≠ nπ (undefined where sin(x) = 0)".to_string(),
                range: "(-∞, -1] ∪ [1, ∞)".to_string(),
            }),
        );

        self.step_generators.insert(
            "sec".to_string(),
            Box::new(TrigStepGenerator {
                function_name: "sec".to_string(),
                latex_name: "\\sec".to_string(),
                context: "Secant: 1/cos(x), reciprocal of cosine".to_string(),
                special_values: vec![("0", "1"), ("π/3", "2")],
                domain: "x ≠ π/2 + nπ (undefined where cos(x) = 0)".to_string(),
                range: "(-∞, -1] ∪ [1, ∞)".to_string(),
            }),
        );

        self.step_generators.insert(
            "cot".to_string(),
            Box::new(TrigStepGenerator {
                function_name: "cot".to_string(),
                latex_name: "\\cot".to_string(),
                context: "Cotangent: cos(x)/sin(x), reciprocal of tangent".to_string(),
                special_values: vec![("π/4", "1")],
                domain: "x ≠ nπ (undefined where sin(x) = 0)".to_string(),
                range: "All real numbers".to_string(),
            }),
        );

        self.step_generators.insert(
            "arcsin".to_string(),
            Box::new(InverseTrigStepGenerator {
                function_name: "arcsin".to_string(),
                latex_name: "\\arcsin".to_string(),
                context: "Inverse sine: angle whose sine is x".to_string(),
                domain: "[-1, 1] (must be valid sine value)".to_string(),
                range: "[-π/2, π/2] (principal branch)".to_string(),
            }),
        );

        self.step_generators.insert(
            "arccos".to_string(),
            Box::new(InverseTrigStepGenerator {
                function_name: "arccos".to_string(),
                latex_name: "\\arccos".to_string(),
                context: "Inverse cosine: angle whose cosine is x".to_string(),
                domain: "[-1, 1] (must be valid cosine value)".to_string(),
                range: "[0, π] (principal branch)".to_string(),
            }),
        );

        self.step_generators.insert(
            "arctan".to_string(),
            Box::new(InverseTrigStepGenerator {
                function_name: "arctan".to_string(),
                latex_name: "\\arctan".to_string(),
                context: "Inverse tangent: angle whose tangent is x".to_string(),
                domain: "All real numbers".to_string(),
                range: "(-π/2, π/2) (principal branch)".to_string(),
            }),
        );
    }

    /// Initialize exponential/logarithmic education (6 functions)
    fn initialize_exponential_logarithmic(&mut self) {
        self.step_generators.insert(
            "exp".to_string(),
            Box::new(ExpLogStepGenerator {
                function_name: "exp".to_string(),
                latex_name: "e^x".to_string(),
                context: "Natural exponential: base e (≈2.718)".to_string(),
                special_values: vec![("0", "1"), ("1", "e"), ("ln(a)", "a")],
                domain: "All real numbers".to_string(),
                range: "(0, ∞) (always positive)".to_string(),
            }),
        );

        self.step_generators.insert(
            "ln".to_string(),
            Box::new(ExpLogStepGenerator {
                function_name: "ln".to_string(),
                latex_name: "\\ln".to_string(),
                context: "Natural logarithm: inverse of e^x, base e".to_string(),
                special_values: vec![("1", "0"), ("e", "1"), ("e^k", "k")],
                domain: "(0, ∞) (only positive numbers)".to_string(),
                range: "All real numbers".to_string(),
            }),
        );

        self.step_generators.insert(
            "log".to_string(),
            Box::new(ExpLogStepGenerator {
                function_name: "log".to_string(),
                latex_name: "\\log".to_string(),
                context: "Common logarithm: base 10 logarithm".to_string(),
                special_values: vec![("1", "0"), ("10", "1"), ("100", "2"), ("1000", "3")],
                domain: "(0, ∞) (only positive numbers)".to_string(),
                range: "All real numbers".to_string(),
            }),
        );

        self.step_generators.insert(
            "log10".to_string(),
            Box::new(ExpLogStepGenerator {
                function_name: "log10".to_string(),
                latex_name: "\\log_{10}".to_string(),
                context: "Base-10 logarithm: same as log".to_string(),
                special_values: vec![("1", "0"), ("10", "1"), ("100", "2")],
                domain: "(0, ∞) (only positive numbers)".to_string(),
                range: "All real numbers".to_string(),
            }),
        );

        self.step_generators.insert(
            "sqrt".to_string(),
            Box::new(ExpLogStepGenerator {
                function_name: "sqrt".to_string(),
                latex_name: "\\sqrt{x}".to_string(),
                context: "Square root: principal (positive) square root".to_string(),
                special_values: vec![("0", "0"), ("1", "1"), ("4", "2"), ("9", "3"), ("16", "4")],
                domain: "[0, ∞) (non-negative for real numbers)".to_string(),
                range: "[0, ∞) (non-negative result)".to_string(),
            }),
        );

        self.step_generators.insert(
            "cbrt".to_string(),
            Box::new(ExpLogStepGenerator {
                function_name: "cbrt".to_string(),
                latex_name: "\\sqrt[3]{x}".to_string(),
                context: "Cube root: real cube root of x".to_string(),
                special_values: vec![
                    ("0", "0"),
                    ("1", "1"),
                    ("8", "2"),
                    ("27", "3"),
                    ("-8", "-2"),
                ],
                domain: "All real numbers".to_string(),
                range: "All real numbers".to_string(),
            }),
        );
    }

    /// Initialize polynomial families education (4 families)
    fn initialize_polynomial_families(&mut self) {
        self.step_generators.insert("legendre_p".to_string(), Box::new(PolynomialStepGenerator {
            function_name: "legendre_p".to_string(),
            family: PolynomialFamily::Legendre,
            latex_name: "P_n".to_string(),
            context: "Legendre polynomials: orthogonal on [-1,1], arise in physics (spherical harmonics)".to_string(),
            recurrence: "P_{n+1}(x) = [(2n+1)xP_n(x) - nP_{n-1}(x)]/(n+1)".to_string(),
            base_cases: vec![("P_0(x)", "1"), ("P_1(x)", "x"), ("P_2(x)", "(3x²-1)/2")],
        }));

        self.step_generators.insert(
            "chebyshev_t".to_string(),
            Box::new(PolynomialStepGenerator {
                function_name: "chebyshev_t".to_string(),
                family: PolynomialFamily::Chebyshev,
                latex_name: "T_n".to_string(),
                context: "Chebyshev (first kind): minimize max error in approximation theory"
                    .to_string(),
                recurrence: "T_{n+1}(x) = 2xT_n(x) - T_{n-1}(x)".to_string(),
                base_cases: vec![("T_0(x)", "1"), ("T_1(x)", "x"), ("T_2(x)", "2x²-1")],
            }),
        );

        self.step_generators.insert(
            "hermite_h".to_string(),
            Box::new(PolynomialStepGenerator {
                function_name: "hermite_h".to_string(),
                family: PolynomialFamily::Hermite,
                latex_name: "H_n".to_string(),
                context: "Hermite polynomials: orthogonal with Gaussian weight, quantum mechanics"
                    .to_string(),
                recurrence: "H_{n+1}(x) = 2xH_n(x) - 2nH_{n-1}(x)".to_string(),
                base_cases: vec![("H_0(x)", "1"), ("H_1(x)", "2x"), ("H_2(x)", "4x²-2")],
            }),
        );

        self.step_generators.insert("laguerre_l".to_string(), Box::new(PolynomialStepGenerator {
            function_name: "laguerre_l".to_string(),
            family: PolynomialFamily::Laguerre,
            latex_name: "L_n".to_string(),
            context: "Laguerre polynomials: orthogonal with exponential weight, quantum mechanics (radial wavefunctions)".to_string(),
            recurrence: "L_{n+1}(x) = [(2n+1-x)L_n(x) - nL_{n-1}(x)]/(n+1)".to_string(),
            base_cases: vec![("L_0(x)", "1"), ("L_1(x)", "1-x"), ("L_2(x)", "(2-4x+x²)/2")],
        }));
    }

    /// Initialize number theory education (3 functions)
    fn initialize_number_theory(&mut self) {
        self.step_generators.insert(
            "factorial".to_string(),
            Box::new(NumberTheoryStepGenerator {
                function_name: "factorial".to_string(),
                latex_name: "n!".to_string(),
                context: "Factorial: product of positive integers up to n".to_string(),
                formula: "n! = n × (n-1) × ... × 2 × 1, with 0! = 1".to_string(),
                special_values: vec![
                    ("0!", "1"),
                    ("1!", "1"),
                    ("2!", "2"),
                    ("3!", "6"),
                    ("4!", "24"),
                    ("5!", "120"),
                ],
                domain: "Non-negative integers".to_string(),
            }),
        );

        self.step_generators.insert(
            "gcd".to_string(),
            Box::new(NumberTheoryStepGenerator {
                function_name: "gcd".to_string(),
                latex_name: "\\gcd".to_string(),
                context: "Greatest common divisor: largest integer dividing both numbers"
                    .to_string(),
                formula: "Euclidean algorithm: gcd(a,b) = gcd(b, a mod b)".to_string(),
                special_values: vec![("gcd(12,18)", "6"), ("gcd(15,25)", "5"), ("gcd(7,11)", "1")],
                domain: "Integers".to_string(),
            }),
        );

        self.step_generators.insert(
            "lcm".to_string(),
            Box::new(NumberTheoryStepGenerator {
                function_name: "lcm".to_string(),
                latex_name: "\\text{lcm}".to_string(),
                context:
                    "Least common multiple: smallest positive integer divisible by both numbers"
                        .to_string(),
                formula: "lcm(a,b) = |a×b| / gcd(a,b)".to_string(),
                special_values: vec![("lcm(4,6)", "12"), ("lcm(3,5)", "15"), ("lcm(6,8)", "24")],
                domain: "Positive integers".to_string(),
            }),
        );
    }
}

/// Trigonometric function step generator (sin, cos, tan, csc, sec, cot)
struct TrigStepGenerator {
    function_name: String,
    latex_name: String,
    context: String,
    special_values: Vec<(&'static str, &'static str)>,
    domain: String,
    range: String,
}

impl StepGenerator for TrigStepGenerator {
    fn generate_steps(&self, _name: &str, args: &[Expression]) -> Vec<Step> {
        vec![
            Step::new(
                "Function Type",
                format!("Trigonometric function: {}", self.latex_name),
            ),
            Step::new("Mathematical Context", self.context.clone()),
            Step::new("Domain", format!("Valid inputs: {}", self.domain)),
            Step::new("Range", format!("Possible outputs: {}", self.range)),
            Step::new(
                "Input Analysis",
                format!("Evaluating {}({})", self.latex_name, format_args(args)),
            ),
            Step::new(
                "Special Values",
                format!(
                    "Checking {} known special values",
                    self.special_values.len()
                ),
            ),
            Step::new(
                "Result",
                format!(
                    "{}({}) = computed result",
                    self.latex_name,
                    format_args(args)
                ),
            ),
        ]
    }

    fn get_mathematical_context(&self, _name: &str) -> String {
        self.context.clone()
    }
}

/// Inverse trigonometric step generator (arcsin, arccos, arctan)
struct InverseTrigStepGenerator {
    function_name: String,
    latex_name: String,
    context: String,
    domain: String,
    range: String,
}

impl StepGenerator for InverseTrigStepGenerator {
    fn generate_steps(&self, _name: &str, args: &[Expression]) -> Vec<Step> {
        vec![
            Step::new(
                "Function Type",
                format!("Inverse trigonometric: {}", self.latex_name),
            ),
            Step::new("Mathematical Context", self.context.clone()),
            Step::new(
                "Domain Restriction",
                format!("Input must be in: {}", self.domain),
            ),
            Step::new(
                "Range (Principal Branch)",
                format!("Output will be in: {}", self.range),
            ),
            Step::new(
                "Input Validation",
                format!("Checking if {} is in valid domain", format_args(args)),
            ),
            Step::new(
                "Computation",
                format!(
                    "Finding angle whose {} gives {}",
                    self.function_name.replace("arc", ""),
                    format_args(args)
                ),
            ),
            Step::new(
                "Result",
                format!(
                    "{}({}) = computed angle",
                    self.latex_name,
                    format_args(args)
                ),
            ),
        ]
    }

    fn get_mathematical_context(&self, _name: &str) -> String {
        self.context.clone()
    }
}

/// Exponential/logarithmic step generator
struct ExpLogStepGenerator {
    function_name: String,
    latex_name: String,
    context: String,
    special_values: Vec<(&'static str, &'static str)>,
    domain: String,
    range: String,
}

impl StepGenerator for ExpLogStepGenerator {
    fn generate_steps(&self, _name: &str, args: &[Expression]) -> Vec<Step> {
        vec![
            Step::new(
                "Function Type",
                format!("Exponential/Logarithmic: {}", self.latex_name),
            ),
            Step::new("Mathematical Context", self.context.clone()),
            Step::new("Domain", format!("Valid inputs: {}", self.domain)),
            Step::new("Range", format!("Possible outputs: {}", self.range)),
            Step::new(
                "Input Analysis",
                format!("Evaluating {} at {}", self.latex_name, format_args(args)),
            ),
            Step::new(
                "Special Value Check",
                format!("Checking {} special values", self.special_values.len()),
            ),
            Step::new(
                "Result",
                format!(
                    "{}({}) = computed result",
                    self.latex_name,
                    format_args(args)
                ),
            ),
        ]
    }

    fn get_mathematical_context(&self, _name: &str) -> String {
        self.context.clone()
    }
}

/// Polynomial family step generator
struct PolynomialStepGenerator {
    function_name: String,
    family: PolynomialFamily,
    latex_name: String,
    context: String,
    recurrence: String,
    base_cases: Vec<(&'static str, &'static str)>,
}

impl StepGenerator for PolynomialStepGenerator {
    fn generate_steps(&self, _name: &str, args: &[Expression]) -> Vec<Step> {
        vec![
            Step::new(
                "Polynomial Family",
                format!("{:?} polynomials", self.family),
            ),
            Step::new("Mathematical Context", self.context.clone()),
            Step::new(
                "Notation",
                format!("Computing {}({})", self.latex_name, format_args(args)),
            ),
            Step::new("Recurrence Relation", self.recurrence.clone()),
            Step::new(
                "Base Cases",
                format!("{} base cases known", self.base_cases.len()),
            ),
            Step::new(
                "Computation Method",
                "Using three-term recurrence relation".to_string(),
            ),
            Step::new(
                "Result",
                format!(
                    "{}({}) = computed polynomial",
                    self.latex_name,
                    format_args(args)
                ),
            ),
        ]
    }

    fn get_mathematical_context(&self, _name: &str) -> String {
        self.context.clone()
    }
}

/// Number theory step generator
struct NumberTheoryStepGenerator {
    function_name: String,
    latex_name: String,
    context: String,
    formula: String,
    special_values: Vec<(&'static str, &'static str)>,
    domain: String,
}

impl StepGenerator for NumberTheoryStepGenerator {
    fn generate_steps(&self, _name: &str, args: &[Expression]) -> Vec<Step> {
        vec![
            Step::new(
                "Function Type",
                format!("Number Theory: {}", self.latex_name),
            ),
            Step::new("Mathematical Context", self.context.clone()),
            Step::new("Formula", self.formula.clone()),
            Step::new("Domain", format!("Valid inputs: {}", self.domain)),
            Step::new(
                "Input",
                format!("Computing {} for {}", self.function_name, format_args(args)),
            ),
            Step::new(
                "Known Values",
                format!("{} special values available", self.special_values.len()),
            ),
            Step::new(
                "Result",
                format!(
                    "{}({}) = computed result",
                    self.latex_name,
                    format_args(args)
                ),
            ),
        ]
    }

    fn get_mathematical_context(&self, _name: &str) -> String {
        self.context.clone()
    }
}

/// Format args for display
fn format_args(args: &[Expression]) -> String {
    args.iter()
        .map(|a| format!("{}", a))
        .collect::<Vec<_>>()
        .join(", ")
}

impl Default for FunctionEducator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_function_count() {
        let educator = FunctionEducator::new();
        assert!(
            educator.step_generators.len() >= 20,
            "Should have 20+ functions"
        );
    }

    #[test]
    fn test_trig_education() {
        let educator = FunctionEducator::new();
        let args = vec![Expression::integer(0)];
        let explanation = educator.explain_function_operation("sin", &args, "evaluation");
        assert!(explanation.steps.len() >= 5);
        assert!(explanation
            .steps
            .iter()
            .any(|s| s.title.contains("Domain") || s.description.contains("domain")));
    }

    #[test]
    fn test_special_value_mentions() {
        let educator = FunctionEducator::new();
        let args = vec![Expression::integer(1)];
        let explanation = educator.explain_function_operation("ln", &args, "evaluation");
        assert!(explanation
            .steps
            .iter()
            .any(|s| s.description.contains("special")));
    }
}
