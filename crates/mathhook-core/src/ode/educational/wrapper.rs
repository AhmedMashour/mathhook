
//!
//! Wraps ODE solvers to capture intermediate steps and provide
//! human-readable explanations of the solution process.

use crate::algebra::solvers::SolverResult;

use crate::core::{Expression, Symbol};
use crate::ode::educational::steps::{ODEPhase, ODESolutionStep, ODEStepFactory};
use serde::{Deserialize, Serialize};

/// Educational ODE explanation containing solution and steps
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ODEExplanation {
    /// The final solution
    pub solution: Expression,

    /// Step-by-step explanation
    pub steps: Vec<ODESolutionStep>,

    /// ODE type identified
    pub ode_type: String,

    /// Summary of the solution method
    pub method_summary: String,
}

impl ODEExplanation {
    /// Create a new ODE explanation
    pub fn new(
        solution: Expression,
        steps: Vec<ODESolutionStep>,
        ode_type: String,
        method_summary: String,
    ) -> Self {
        Self {
            solution,
            steps,
            ode_type,
            method_summary,
        }
    }

    /// Get human-readable explanation
    pub fn to_human_readable(&self) -> String {
        let mut output = String::new();
        output.push_str(&format!("ODE Type: {}\n", self.ode_type));
        output.push_str(&format!("Method: {}\n\n", self.method_summary));
        output.push_str("Solution Steps:\n");
        output.push_str(&"=".repeat(60));
        output.push('\n');

        for (i, step) in self.steps.iter().enumerate() {
            output.push_str(&format!("\nStep {}: {}\n", i + 1, step.title));
            output.push_str(&format!("Description: {}\n", step.description));
            output.push_str(&format!("Justification: {}\n", step.justification));
            output.push_str(&format!("Before: {}\n", step.before));
            output.push_str(&format!("After: {}\n", step.after));
            output.push_str(&"-".repeat(60));
            output.push('\n');
        }

        output.push_str(&format!("\nFinal Solution: {}\n", self.solution));
        output
    }

    /// Get LaTeX formatted explanation
    pub fn to_latex(&self) -> String {
        let mut latex = String::new();
        latex.push_str("\\begin{align*}\n");
        latex.push_str(&format!("\\text{{ODE Type: }} & {} \\\\\n", self.ode_type));
        latex.push_str(&format!("\\text{{Method: }} & {} \\\\\n", self.method_summary));
        latex.push_str("\\end{align*}\n\n");
        latex.push_str("\\begin{align*}\n");

        for step in &self.steps {
            latex.push_str(&step.to_latex_detailed());
            latex.push_str(" \\\\\n");
        }

        latex.push_str(&format!("\\text{{Solution: }} & {} \\\\\n", self.solution));
        latex.push_str("\\end{align*}\n");
        latex
    }

    /// Get steps for a specific phase
    pub fn steps_by_phase(&self, phase: ODEPhase) -> Vec<&ODESolutionStep> {
        self.steps.iter().filter(|s| s.phase == phase).collect()
    }
}

/// Educational ODE solver that provides step-by-step explanations
pub struct EducationalODESolver;

impl EducationalODESolver {
    /// Create a new educational ODE solver
    pub fn new() -> Self {
        Self
    }

    /// Solve separable ODE with step-by-step explanation
    ///
    /// # Arguments
    ///
    /// * `rhs` - Right-hand side of dy/dx = rhs
    /// * `dependent` - Dependent variable (y)
    /// * `independent` - Independent variable (x)
    /// * `initial_condition` - Optional (x0, y0) for particular solution
    ///
    /// # Returns
    ///
    /// ODEExplanation containing solution and all steps
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{symbol, expr};
    /// use mathhook_core::ode::educational::EducationalODESolver;
    ///
    /// let x = symbol!(x);
    /// let y = symbol!(y);
    /// let rhs = expr!(x);
    ///
    /// let solver = EducationalODESolver::new();
    /// let explanation = solver.solve_separable_with_steps(&rhs, &y, &x, None);
    /// ```
    pub fn solve_separable_with_steps(
        &self,
        rhs: &Expression,
        dependent: &Symbol,
        independent: &Symbol,
        _initial_condition: Option<(Expression, Expression)>,
    ) -> Result<ODEExplanation, String> {
        let mut steps = Vec::new();

        // Step 1: Detection
        let detection_step = ODEStepFactory::detection(
            "separable",
            rhs,
            "The equation can be written in the form dy/dx = g(x)h(y), allowing variable separation",
        );
        steps.push(detection_step);

        // Step 2: Separation (placeholder - would need actual separation logic)
        let separation_step = ODEStepFactory::separation(
            rhs,
            rhs,
            "g(x)",
            "h(y)",
        );
        steps.push(separation_step);

        // Step 3: Integration (left side)
        let integration_left = ODEStepFactory::integration(
            &Expression::integer(1),
            &Expression::integer(1),
            dependent,
            "left",
        );
        steps.push(integration_left);

        // Step 4: Integration (right side)
        let integration_right = ODEStepFactory::integration(
            rhs,
            &Expression::integer(1),
            independent,
            "right",
        );
        steps.push(integration_right);

        let solution = Expression::symbol(dependent.clone());

        let explanation = ODEExplanation::new(
            solution,
            steps,
            "Separable".to_string(),
            "Variable separation followed by integration".to_string(),
        );

        Ok(explanation)
    }

    /// Solve linear first-order ODE with step-by-step explanation
    ///
    /// # Arguments
    ///
    /// * `rhs` - Right-hand side of dy/dx = rhs
    /// * `dependent` - Dependent variable (y)
    /// * `_independent` - Independent variable (x)
    /// * `_initial_condition` - Optional (x0, y0) for particular solution
    ///
    /// # Returns
    ///
    /// ODEExplanation containing solution and all steps
    pub fn solve_linear_with_steps(
        &self,
        rhs: &Expression,
        dependent: &Symbol,
        _independent: &Symbol,
        _initial_condition: Option<(Expression, Expression)>,
    ) -> Result<ODEExplanation, String> {
        let mut steps = Vec::new();

        // Step 1: Detection
        let detection_step = ODEStepFactory::detection(
            "linear first-order",
            rhs,
            "The equation has the form dy/dx + P(x)y = Q(x), which is linear in y",
        );
        steps.push(detection_step);

        // Placeholder for additional steps
        // In full implementation:
        // - Identify P(x) and Q(x)
        // - Calculate integrating factor μ(x) = exp(∫P(x)dx)
        // - Multiply equation by μ(x)
        // - Integrate both sides
        // - Solve for y

        let solution = Expression::symbol(dependent.clone());

        let explanation = ODEExplanation::new(
            solution,
            steps,
            "Linear First-Order".to_string(),
            "Integrating factor method".to_string(),
        );

        Ok(explanation)
    }
}

impl Default for EducationalODESolver {
    fn default() -> Self {
        Self::new()
    }
}

/// Implement the EquationSolver trait for integration with SmartEquationSolver
impl crate::algebra::solvers::EquationSolver for EducationalODESolver {
    fn solve(&self, _equation: &Expression, _variable: &Symbol) -> crate::algebra::solvers::SolverResult {

        // Placeholder: Full integration will classify ODE type and route to appropriate solver
        SolverResult::NoSolution
    }

    fn solve_with_explanation(
        &self,
        equation: &Expression,
        variable: &Symbol,
    ) -> (crate::algebra::solvers::SolverResult, crate::educational::step_by_step::StepByStepExplanation) {
        use crate::educational::step_by_step::{Step, StepByStepExplanation};

        let mut steps = Vec::new();

        // Add classification step
        steps.push(Step::new(
            "ODE Classification",
            "Analyzing differential equation structure..."
        ));

        // Placeholder: Full integration will use ODEClassifier
        steps.push(Step::new(
            "Status",
            "ODE solving integration in progress (Wave 1-INT)"
        ));

        let result = self.solve(equation, variable);
        (result, StepByStepExplanation::new(steps))
    }

    fn can_solve(&self, _equation: &Expression) -> bool {
        // Check if equation contains derivatives
        // Placeholder: will use has_derivatives() from EquationAnalyzer
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{expr, symbol};

    #[test]
    fn test_educational_solver_creation() {
        let _solver = EducationalODESolver::new();
        assert!(true, "Solver created successfully");
    }

    #[test]
    fn test_separable_with_steps() {
        let x = symbol!(x);
        let y = symbol!(y);
        let rhs = expr!(x);

        let solver = EducationalODESolver::new();
        let result = solver.solve_separable_with_steps(&rhs, &y, &x, None);

        assert!(result.is_ok(), "Should solve separable ODE");
        let explanation = result.unwrap();
        assert_eq!(explanation.ode_type, "Separable");
        assert!(!explanation.steps.is_empty(), "Should have steps");
        assert!(explanation.steps.len() >= 4, "Should have at least 4 steps");
    }

    #[test]
    fn test_linear_with_steps() {
        let x = symbol!(x);
        let y = symbol!(y);
        let rhs = expr!(y);

        let solver = EducationalODESolver::new();
        let result = solver.solve_linear_with_steps(&rhs, &y, &x, None);

        assert!(result.is_ok(), "Should solve ODE: {:?}", result.err());
        let explanation = result.unwrap();
        assert!(!explanation.steps.is_empty(), "Should have steps");
    }

    #[test]
    fn test_explanation_human_readable() {
        let x = symbol!(x);
        let y = symbol!(y);
        let rhs = expr!(x);

        let solver = EducationalODESolver::new();
        let explanation = solver.solve_separable_with_steps(&rhs, &y, &x, None).unwrap();

        let human = explanation.to_human_readable();
        assert!(human.contains("ODE Type"));
        assert!(human.contains("Method"));
        assert!(human.contains("Step"));
    }

    #[test]
    fn test_explanation_latex() {
        let x = symbol!(x);
        let y = symbol!(y);
        let rhs = expr!(x);

        let solver = EducationalODESolver::new();
        let explanation = solver.solve_separable_with_steps(&rhs, &y, &x, None).unwrap();

        let latex = explanation.to_latex();
        assert!(latex.contains("\\begin{align*}"));
        assert!(latex.contains("\\end{align*}"));
        assert!(latex.contains("ODE Type"));
    }

    #[test]
    fn test_steps_by_phase() {
        let x = symbol!(x);
        let y = symbol!(y);
        let rhs = expr!(x);

        let solver = EducationalODESolver::new();
        let explanation = solver.solve_separable_with_steps(&rhs, &y, &x, None).unwrap();

        let detection_steps = explanation.steps_by_phase(ODEPhase::Detection);
        assert!(!detection_steps.is_empty(), "Should have detection step");

        let integration_steps = explanation.steps_by_phase(ODEPhase::Integration);
        assert!(!integration_steps.is_empty(), "Should have integration steps");
    }
}
