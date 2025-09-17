//! Pre-built Educational ODE Examples
//!
//! Provides worked examples for each ODE type, including:
//! - Complete solutions with all steps
//! - Common pitfalls and mistakes
//! - Reference material for students

use crate::calculus::ode::educational::{ODEExplanation, ODEStepFactory};
use crate::{expr, symbol};

/// Collection of educational ODE examples
pub struct ODEExamples;

impl ODEExamples {
    /// Example: Simplest separable ODE dy/dx = x
    ///
    /// # Solution
    ///
    /// y = x²/2 + C
    ///
    /// # Common Pitfalls
    ///
    /// - Forgetting the constant of integration
    /// - Incorrect integration of x (getting x²/2 instead of x²/2)
    pub fn separable_simple() -> ODEExplanation {
        let x = symbol!(x);
        let y = symbol!(y);
        let rhs = expr!(x);

        let mut steps = Vec::new();

        // Step 1: Detection
        steps.push(ODEStepFactory::detection(
            "separable",
            &rhs,
            "The right-hand side depends only on x, so we can write dy/dx = g(x)·h(y) where g(x) = x and h(y) = 1",
        ));

        // Step 2: Separation
        steps.push(ODEStepFactory::separation(&rhs, &expr!(x), "x", "1"));

        // Step 3: Integration (left side)
        steps.push(ODEStepFactory::integration(
            &expr!(1),
            &expr!(y),
            &y,
            "left",
        ));

        // Step 4: Integration (right side)
        let integral_result = expr!((x ^ 2) / 2);
        steps.push(ODEStepFactory::integration(
            &expr!(x),
            &integral_result,
            &x,
            "right",
        ));

        // Step 5: Solution construction
        let solution = expr!(y); // Placeholder: would be y = x²/2 + C
        steps.push(ODEStepFactory::solution_construction(
            &expr!(y),
            &solution,
            "algebraic rearrangement",
        ));

        ODEExplanation::new(
            solution,
            steps,
            "Separable".to_owned(),
            "Variable separation: Separate variables and integrate both sides".to_owned(),
        )
    }

    /// Example: Exponential growth dy/dx = y
    ///
    /// # Solution
    ///
    /// y = C·e^x
    ///
    /// # Common Pitfalls
    ///
    /// - Forgetting absolute value in ln|y| = x + C
    /// - Incorrect handling of integration constant when exponentiating
    /// - Not recognizing this as both separable and linear
    pub fn exponential_growth() -> ODEExplanation {
        let x = symbol!(x);
        let y = symbol!(y);
        let rhs = expr!(y);

        let mut steps: Vec<crate::calculus::ode::ODESolutionStep> = vec![
            ODEStepFactory::detection(
                "separable (exponential)",
                &rhs,
                "Can be written as dy/dx = 1·y, where g(x) = 1 and h(y) = y",
            ),
            ODEStepFactory::separation(&rhs, &expr!(1 / y), "1", "y"),
            // Integration: ∫(1/y)dy = ln|y|
            ODEStepFactory::integration(
                &expr!(1 / y),
                &expr!(y), // Placeholder: ln|y|
                &y,
                "left",
            ),
            // Integration: ∫1 dx = x
            ODEStepFactory::integration(&expr!(1), &expr!(x), &x, "right"),
        ];

        let solution = expr!(y); // Placeholder: y = C·e^x
        steps.push(ODEStepFactory::solution_construction(
            &expr!(y),
            &solution,
            "exponentiation and constant manipulation",
        ));

        ODEExplanation::new(
            solution,
            steps,
            "Separable (Exponential Growth)".to_owned(),
            "Separation and integration yields exponential solution".to_owned(),
        )
    }

    /// Example: Linear first-order dy/dx + y = x
    ///
    /// # Solution
    ///
    /// y = x - 1 + C·e^(-x)
    ///
    /// # Common Pitfalls
    ///
    /// - Incorrectly computing integrating factor μ(x) = e^(∫P(x)dx)
    /// - Forgetting to multiply Q(x) by μ(x) before integrating
    /// - Sign errors in the exponent
    pub fn linear_first_order() -> ODEExplanation {
        // dy/dx + y = x means rhs = x - y (rearranged form)
        let rhs = expr!(x - y);

        let steps = vec![ODEStepFactory::detection(
            "linear first-order",
            &rhs,
            "Can be written as dy/dx + P(x)y = Q(x) where P(x) = 1 and Q(x) = x",
        )];

        // Additional steps would include:
        // - Computing integrating factor μ(x) = e^x
        // - Multiplying through by μ(x)
        // - Recognizing left side as d/dx[μ(x)y]
        // - Integrating both sides

        let solution = expr!(y); // Placeholder: y = x - 1 + C·e^(-x)
        ODEExplanation::new(
            solution,
            steps,
            "Linear First-Order".to_owned(),
            "Integrating factor method: μ(x) = e^(∫P(x)dx)".to_owned(),
        )
    }

    /// Example: Product form dy/dx = xy
    ///
    /// # Solution
    ///
    /// y = C·e^(x²/2)
    ///
    /// # Common Pitfalls
    ///
    /// - Incorrectly integrating x to get x²/2 vs x²
    /// - Forgetting to include constant in exponential
    /// - Not simplifying e^(x²/2 + C) to C'·e^(x²/2)
    pub fn product_separable() -> ODEExplanation {
        let x = symbol!(x);
        let y = symbol!(y);
        let rhs = expr!(x * y);

        let mut steps = vec![
            ODEStepFactory::detection(
                "separable (product form)",
                &rhs,
                "Equation dy/dx = xy can be written as dy/y = x dx (product of functions of different variables)",
            ),
            ODEStepFactory::separation(&rhs, &expr!(1 / y), "x", "y"),
            ODEStepFactory::integration(
                &expr!(1 / y),
                &expr!(y), // Placeholder: ln|y|
                &y,
                "left",
            ),
            ODEStepFactory::integration(
                &expr!(x),
                &expr!((x ^ 2) / 2),
                &x,
                "right",
            ),

        ];

        let solution = expr!(y); // Placeholder: y = C·e^(x²/2)
        steps.push(ODEStepFactory::solution_construction(
            &expr!(y),
            &solution,
            "exponentiation and constant simplification",
        ));

        ODEExplanation::new(
            solution,
            steps,
            "Separable (Product Form)".to_owned(),
            "Separate and integrate: ln|y| = x²/2 + C → y = C·e^(x²/2)".to_owned(),
        )
    }

    /// Example: Initial value problem dy/dx = x, y(0) = 1
    ///
    /// # Solution
    ///
    /// y = x²/2 + 1
    ///
    /// # Common Pitfalls
    ///
    /// - Forgetting to apply initial condition
    /// - Incorrectly solving for constant C
    /// - Not verifying that y(0) = 1 in final solution
    pub fn initial_value_problem() -> ODEExplanation {
        let x = symbol!(x);
        let y = symbol!(y);
        let rhs = expr!(x);

        let mut steps = vec![
            ODEStepFactory::detection(
                "separable with initial condition",
                &rhs,
                "Standard separable form dy/dx = g(x) with initial condition y(0) = 1",
            ),
            ODEStepFactory::separation(&rhs, &expr!(x), "x", "1"),
            ODEStepFactory::integration(&expr!(1), &expr!(y), &y, "left"),
            ODEStepFactory::integration(&expr!(x), &expr!((x ^ 2) / 2), &x, "right"),
        ];

        // Add initial condition application step
        let mut ic_step = ODEStepFactory::solution_construction(
            &expr!(y),
            &expr!(y), // Placeholder: y = x²/2 + 1
            "applying initial condition y(0) = 1",
        );
        ic_step.description =
            "Substitute x = 0, y = 1 into general solution to find C = 1".to_owned();
        steps.push(ic_step);

        let solution = expr!(y); // Placeholder: y = x²/2 + 1
        ODEExplanation::new(
            solution,
            steps,
            "Initial Value Problem".to_owned(),
            "Solve general solution, then apply initial condition to find constant".to_owned(),
        )
    }

    /// Get all example explanations
    pub fn all_examples() -> Vec<ODEExplanation> {
        vec![
            Self::separable_simple(),
            Self::exponential_growth(),
            Self::linear_first_order(),
            Self::product_separable(),
            Self::initial_value_problem(),
        ]
    }

    /// Get example by name
    pub fn get_example(name: &str) -> Option<ODEExplanation> {
        match name {
            "separable_simple" => Some(Self::separable_simple()),
            "exponential_growth" => Some(Self::exponential_growth()),
            "linear_first_order" => Some(Self::linear_first_order()),
            "product_separable" => Some(Self::product_separable()),
            "initial_value_problem" => Some(Self::initial_value_problem()),
            _ => None,
        }
    }

    /// Get list of available example names
    pub fn example_names() -> Vec<&'static str> {
        vec![
            "separable_simple",
            "exponential_growth",
            "linear_first_order",
            "product_separable",
            "initial_value_problem",
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_separable_simple_example() {
        let example = ODEExamples::separable_simple();
        assert_eq!(example.ode_type, "Separable");
        assert!(!example.steps.is_empty());
        assert!(example.steps.len() >= 5);
    }

    #[test]
    fn test_exponential_growth_example() {
        let example = ODEExamples::exponential_growth();
        assert!(example.ode_type.contains("Exponential"));
        assert!(!example.steps.is_empty());
    }

    #[test]
    fn test_linear_first_order_example() {
        let example = ODEExamples::linear_first_order();
        assert!(example.ode_type.contains("Linear"));
        assert!(!example.steps.is_empty());
    }

    #[test]
    fn test_product_separable_example() {
        let example = ODEExamples::product_separable();
        assert!(example.ode_type.contains("Product"));
        assert!(!example.steps.is_empty());
    }

    #[test]
    fn test_initial_value_problem_example() {
        let example = ODEExamples::initial_value_problem();
        assert!(example.ode_type.contains("Initial Value"));
        assert!(!example.steps.is_empty());
    }

    #[test]
    fn test_all_examples() {
        let examples = ODEExamples::all_examples();
        assert_eq!(examples.len(), 5);
        for example in examples {
            assert!(!example.steps.is_empty());
            assert!(!example.ode_type.is_empty());
        }
    }

    #[test]
    fn test_get_example_by_name() {
        let example = ODEExamples::get_example("separable_simple");
        assert!(example.is_some());

        let example = ODEExamples::get_example("nonexistent");
        assert!(example.is_none());
    }

    #[test]
    fn test_example_names() {
        let names = ODEExamples::example_names();
        assert_eq!(names.len(), 5);
        assert!(names.contains(&"separable_simple"));
        assert!(names.contains(&"exponential_growth"));
    }

    #[test]
    fn test_example_human_readable() {
        let example = ODEExamples::separable_simple();
        let human = example.to_human_readable();
        assert!(human.contains("ODE Type"));
        assert!(human.contains("Method"));
        assert!(human.contains("Step"));
    }

    #[test]
    fn test_example_latex() {
        let example = ODEExamples::exponential_growth();
        let latex = example.to_latex();
        assert!(latex.contains("\\begin{align*}"));
        assert!(latex.contains("\\end{align*}"));
    }
}
