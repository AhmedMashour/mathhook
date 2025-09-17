//! ODE Solution Step Representation
//!
//! Provides structured representation of ODE solution steps including:
//! - Detection phase (identifying ODE type)
//! - Transformation phase (manipulating equation)
//! - Integration phase (solving integrals)
//! - Solution phase (final form and verification)

use crate::core::{Expression, Symbol};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// ODE solution step phase
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ODEPhase {
    /// Detection of ODE type and characteristics
    Detection,
    /// Transformation to standard form
    Transformation,
    /// Integration step
    Integration,
    /// Final solution construction
    Solution,
    /// Verification step
    Verification,
}

/// ODE solution step with mathematical and educational context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ODESolutionStep {
    /// Unique step identifier
    pub step_id: String,

    /// Phase of the solution process
    pub phase: ODEPhase,

    /// Human-readable title
    pub title: String,

    /// Detailed description
    pub description: String,

    /// Mathematical justification
    pub justification: String,

    /// Expression state before this step
    pub before: Expression,

    /// Expression state after this step
    pub after: Expression,

    /// LaTeX representation of the step
    pub latex: String,

    /// Additional metadata
    pub metadata: HashMap<String, String>,
}

impl ODESolutionStep {
    /// Create a new ODE solution step
    pub fn new(
        step_id: String,
        phase: ODEPhase,
        title: String,
        description: String,
        justification: String,
        before: Expression,
        after: Expression,
    ) -> Self {
        let latex = format!("{} \\rightarrow {}", before, after);
        Self {
            step_id,
            phase,
            title,
            description,
            justification,
            before,
            after,
            latex,
            metadata: HashMap::new(),
        }
    }

    /// Add metadata to the step
    pub fn with_metadata(mut self, key: String, value: String) -> Self {
        self.metadata.insert(key, value);
        self
    }

    /// Get LaTeX representation with custom formatting
    pub fn to_latex_detailed(&self) -> String {
        format!(
            "\\text{{{}}} &: \\quad {} \\\\\n\\text{{Justification}}: &\\quad \\text{{{}}}",
            self.title, self.latex, self.justification
        )
    }
}

/// Builder for ODE solution steps
pub struct ODESolutionStepBuilder {
    step_id: Option<String>,
    phase: Option<ODEPhase>,
    title: Option<String>,
    description: Option<String>,
    justification: Option<String>,
    before: Option<Expression>,
    after: Option<Expression>,
    metadata: HashMap<String, String>,
}

impl ODESolutionStepBuilder {
    /// Create a new builder
    pub fn new() -> Self {
        Self {
            step_id: None,
            phase: None,
            title: None,
            description: None,
            justification: None,
            before: None,
            after: None,
            metadata: HashMap::new(),
        }
    }

    /// Set step ID
    pub fn step_id(mut self, id: String) -> Self {
        self.step_id = Some(id);
        self
    }

    /// Set phase
    pub fn phase(mut self, phase: ODEPhase) -> Self {
        self.phase = Some(phase);
        self
    }

    /// Set title
    pub fn title(mut self, title: String) -> Self {
        self.title = Some(title);
        self
    }

    /// Set description
    pub fn description(mut self, desc: String) -> Self {
        self.description = Some(desc);
        self
    }

    /// Set justification
    pub fn justification(mut self, just: String) -> Self {
        self.justification = Some(just);
        self
    }

    /// Set before state
    pub fn before(mut self, expr: Expression) -> Self {
        self.before = Some(expr);
        self
    }

    /// Set after state
    pub fn after(mut self, expr: Expression) -> Self {
        self.after = Some(expr);
        self
    }

    /// Add metadata
    pub fn metadata(mut self, key: String, value: String) -> Self {
        self.metadata.insert(key, value);
        self
    }

    /// Build the step
    pub fn build(self) -> Result<ODESolutionStep, String> {
        let mut step = ODESolutionStep::new(
            self.step_id.ok_or("Missing step_id")?,
            self.phase.ok_or("Missing phase")?,
            self.title.ok_or("Missing title")?,
            self.description.ok_or("Missing description")?,
            self.justification.ok_or("Missing justification")?,
            self.before.ok_or("Missing before expression")?,
            self.after.ok_or("Missing after expression")?,
        );

        step.metadata = self.metadata;
        Ok(step)
    }
}

impl Default for ODESolutionStepBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Factory for creating common ODE solution steps
pub struct ODEStepFactory;

impl ODEStepFactory {
    /// Create detection step
    pub fn detection(ode_type: &str, equation: &Expression, reason: &str) -> ODESolutionStep {
        ODESolutionStepBuilder::new()
            .step_id(format!("detect_{}", ode_type))
            .phase(ODEPhase::Detection)
            .title(format!("Identify as {} ODE", ode_type))
            .description(format!(
                "Analyzing the equation, we identify it as a {} ordinary differential equation.",
                ode_type
            ))
            .justification(reason.to_owned())
            .before(equation.clone())
            .after(equation.clone())
            .metadata("ode_type".to_owned(), ode_type.to_owned())
            .build()
            .expect("Valid detection step")
    }

    /// Create separation step for separable ODEs
    pub fn separation(
        original: &Expression,
        separated: &Expression,
        g_x: &str,
        h_y: &str,
    ) -> ODESolutionStep {
        ODESolutionStepBuilder::new()
            .step_id("separate_variables".to_owned())
            .phase(ODEPhase::Transformation)
            .title("Separate Variables".to_owned())
            .description(format!(
                "Rewrite the equation to separate x and y terms: dy/h(y) = g(x)dx where g(x) = {} and h(y) = {}",
                g_x, h_y
            ))
            .justification("Variable separation allows us to integrate each side independently".to_owned())
            .before(original.clone())
            .after(separated.clone())
            .metadata("g_x".to_owned(), g_x.to_owned())
            .metadata("h_y".to_owned(), h_y.to_owned())
            .build()
            .expect("Valid separation step")
    }

    /// Create integration step
    pub fn integration(
        integrand: &Expression,
        result: &Expression,
        variable: &Symbol,
        side: &str,
    ) -> ODESolutionStep {
        ODESolutionStepBuilder::new()
            .step_id(format!("integrate_{}", side))
            .phase(ODEPhase::Integration)
            .title(format!("Integrate {} Side", side))
            .description(format!(
                "Compute the integral: ∫({}) d{}",
                integrand,
                variable.name()
            ))
            .justification(
                "Integration yields the antiderivative plus constant of integration".to_owned(),
            )
            .before(integrand.clone())
            .after(result.clone())
            .metadata("variable".to_owned(), variable.name().to_owned())
            .metadata("side".to_owned(), side.to_owned())
            .build()
            .expect("Valid integration step")
    }

    /// Create solution construction step
    pub fn solution_construction(
        implicit: &Expression,
        explicit: &Expression,
        method: &str,
    ) -> ODESolutionStep {
        ODESolutionStepBuilder::new()
            .step_id("construct_solution".to_owned())
            .phase(ODEPhase::Solution)
            .title("Construct Final Solution".to_owned())
            .description(format!("Solve for the dependent variable using {}", method))
            .justification(
                "Rearranging the integrated equation to express y explicitly in terms of x"
                    .to_owned(),
            )
            .before(implicit.clone())
            .after(explicit.clone())
            .metadata("method".to_owned(), method.to_owned())
            .build()
            .expect("Valid solution step")
    }

    /// Create verification step
    pub fn verification(
        solution: &Expression,
        original_ode: &Expression,
        verification_result: bool,
    ) -> ODESolutionStep {
        let status = if verification_result {
            "verified"
        } else {
            "pending"
        };
        ODESolutionStepBuilder::new()
            .step_id("verify_solution".to_owned())
            .phase(ODEPhase::Verification)
            .title("Verify Solution".to_owned())
            .description(format!(
                "Substitute the solution back into the original equation to verify correctness: {}",
                status
            ))
            .justification(
                "Verification confirms the solution satisfies the differential equation".to_owned(),
            )
            .before(solution.clone())
            .after(original_ode.clone())
            .metadata("verified".to_owned(), verification_result.to_string())
            .build()
            .expect("Valid verification step")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{expr, symbol};

    #[test]
    fn test_ode_solution_step_creation() {
        let before = expr!(x);
        let after = expr!(x ^ 2);

        let step = ODESolutionStep::new(
            "step1".to_string(),
            ODEPhase::Integration,
            "Integrate".to_string(),
            "Integrate both sides".to_string(),
            "Fundamental theorem of calculus".to_string(),
            before.clone(),
            after.clone(),
        );

        assert_eq!(step.step_id, "step1");
        assert_eq!(step.phase, ODEPhase::Integration);
        assert_eq!(step.title, "Integrate");
        assert_eq!(step.before, before);
        assert_eq!(step.after, after);
    }

    #[test]
    fn test_step_builder() {
        let before = expr!(1);
        let after = expr!(2);

        let step = ODESolutionStepBuilder::new()
            .step_id("test".to_string())
            .phase(ODEPhase::Detection)
            .title("Test".to_string())
            .description("Test step".to_string())
            .justification("Testing".to_string())
            .before(before.clone())
            .after(after.clone())
            .metadata("key".to_string(), "value".to_string())
            .build()
            .unwrap();

        assert_eq!(step.step_id, "test");
        assert_eq!(step.metadata.get("key"), Some(&"value".to_string()));
    }

    #[test]
    fn test_detection_factory() {
        let equation = expr!(x);
        let step = ODEStepFactory::detection("separable", &equation, "Can factor into g(x)h(y)");

        assert_eq!(step.phase, ODEPhase::Detection);
        assert!(step.title.contains("separable"));
        assert!(step.description.contains("separable"));
        assert_eq!(
            step.metadata.get("ode_type"),
            Some(&"separable".to_string())
        );
    }

    #[test]
    fn test_separation_factory() {
        let original = expr!(x);
        let separated = expr!(x ^ 2);

        let step = ODEStepFactory::separation(&original, &separated, "x", "1");

        assert_eq!(step.phase, ODEPhase::Transformation);
        assert!(step.title.contains("Separate"));
        assert_eq!(step.metadata.get("g_x"), Some(&"x".to_string()));
        assert_eq!(step.metadata.get("h_y"), Some(&"1".to_string()));
    }

    #[test]
    fn test_integration_factory() {
        let x = symbol!(x);
        let integrand = expr!(x);
        let result = expr!(x ^ 2);

        let step = ODEStepFactory::integration(&integrand, &result, &x, "left");

        assert_eq!(step.phase, ODEPhase::Integration);
        assert!(step.title.contains("Integrate"));
        assert_eq!(step.metadata.get("side"), Some(&"left".to_string()));
    }
}
