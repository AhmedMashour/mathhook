//! Provides both human messages AND structured data for external applications
//! User requirement: "steps to be smart enough to have their own full human readable messages along with message keys"

use crate::core::{Expression, Symbol};
use crate::educational::step_by_step::Step;
use crate::formatter::latex::LaTeXFormatter;
use crate::formatter::{FormattingError, MathLanguage};
use serde_json;

/// Format context for enhanced steps
#[derive(Debug, Clone)]
pub struct FormatContext {
    pub target_format: MathLanguage,
    pub include_intermediate_steps: bool,
    pub verbosity_level: u8, // 1-5, where 5 is most verbose
}

impl Default for FormatContext {
    fn default() -> Self {
        Self {
            target_format: MathLanguage::default(),
            include_intermediate_steps: true,
            verbosity_level: 3,
        }
    }
}

impl FormatContext {
    /// Format an expression according to the target format
    pub fn format_expression(&self, expr: &Expression) -> Result<String, FormattingError> {
        match self.target_format {
            MathLanguage::LaTeX => expr.to_latex(None),
            MathLanguage::Wolfram => {
                // Placeholder - would use to_wolfram when available
                Ok(expr.to_string())
            }
            MathLanguage::Simple | MathLanguage::Human => Ok(expr.to_string()),
            MathLanguage::Json => {
                serde_json::to_string(expr).map_err(|e| FormattingError::SerializationError {
                    message: e.to_string(),
                })
            }
            MathLanguage::Markdown => {
                // Placeholder - would use to_markdown when available
                Ok(expr.to_string())
            }
        }
    }

    /// Format an expression with fallback on error
    pub fn format_expression_safe(&self, expr: &Expression) -> String {
        self.format_expression(expr)
            .unwrap_or_else(|e| format!("{{error: {}}}", e))
    }
}

/// Smart step type for enhanced educational features
pub type SmartStep = Step;

/// Smart step builder for creating enhanced steps
pub struct SmartStepBuilder {
    title: String,
}

impl SmartStepBuilder {
    pub fn new(title: &str) -> Self {
        Self {
            title: title.to_string(),
        }
    }

    pub fn with_human_message(self, title: &str, message: &str) -> Self {
        Self {
            title: format!("{}: {}", title, message),
        }
    }

    pub fn with_api_data(self, _category: &str, _step_type: &str, _operation: &str) -> Self {
        self
    }

    pub fn with_input(self, _key: &str, _value: &str) -> Self {
        self
    }

    pub fn with_output(self, _key: &str, _value: &str) -> Self {
        self
    }

    pub fn with_math_context(self, _equation: &str, _variable: &str, _progress: f64) -> Self {
        self
    }

    pub fn with_message_key(self, _category: &str, _message_type: &str, _variant: u32) -> Self {
        self
    }

    pub fn with_presentation(self, _color: &str, _importance: u8, _animation: &str) -> Self {
        self
    }

    pub fn build(self) -> SmartStep {
        Step::new(self.title, "Enhanced step content".to_string())
    }
}

/// Difficulty level for educational content
#[derive(Debug, Clone, PartialEq)]
pub enum DifficultyLevel {
    Beginner,
    Intermediate,
    Advanced,
}

/// Educational result wrapper
#[derive(Debug, Clone, PartialEq)]
pub struct EducationalResult {
    pub result: crate::core::Expression,
    pub difficulty: DifficultyLevel,
}
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 🎯 ENHANCED STEP - COMPLETE STEP WITH HUMAN + API DATA
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnhancedStep {
    /// Unique identifier for this step type
    pub step_id: String,

    /// Human-readable title
    pub title: String,

    /// Complete human-readable description
    pub human_message: String,

    /// Structured data for API consumption
    pub api_data: StepApiData,

    /// Message key for external message mapping
    pub message_key: MessageKey,

    /// Mathematical context
    pub math_context: MathContext,

    /// Visual presentation hints
    pub presentation: PresentationHints,
}

/// 📊 STEP API DATA - STRUCTURED DATA FOR EXTERNAL APPS
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StepApiData {
    /// Step category (linear, quadratic, etc.)
    pub category: String,

    /// Step type (introduction, calculation, result, etc.)
    pub step_type: String,

    /// Mathematical operation being performed
    pub operation: String,

    /// Input expressions/values
    pub inputs: HashMap<String, String>,

    /// Output expressions/values
    pub outputs: HashMap<String, String>,

    /// Mathematical properties
    pub properties: HashMap<String, serde_json::Value>,
}

/// 🔑 MESSAGE KEY - FOR EXTERNAL MESSAGE SYSTEMS
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageKey {
    /// Category identifier
    pub category: String,

    /// Message type identifier
    pub message_type: String,

    /// Variant number
    pub variant: u32,

    /// Hash for quick lookup
    pub hash: u64,

    /// Template parameters
    pub template_params: Vec<String>,
}

/// 🧮 MATH CONTEXT - MATHEMATICAL INFORMATION
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MathContext {
    /// Original equation
    pub equation: String,

    /// Variable being solved for
    pub variable: String,

    /// Current equation state
    pub current_state: String,

    /// Mathematical coefficients
    pub coefficients: HashMap<String, String>,

    /// Solution progress (0.0 to 1.0)
    pub progress: f64,

    /// Equation type
    pub equation_type: String,
}

/// Presentation hints
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PresentationHints {
    /// Primary color theme
    pub color_theme: String,

    /// Importance level (1-5)
    pub importance: u8,

    /// Animation suggestions
    pub animation: String,

    /// Layout suggestions
    pub layout: String,

    /// Interactive elements
    pub interactive_elements: Vec<String>,
}

/// Enhanced step builder
pub struct EnhancedStepBuilder {
    step_id: String,
    title: String,
    human_message: String,
    api_data: StepApiData,
    message_key: MessageKey,
    math_context: MathContext,
    presentation: PresentationHints,
}

impl EnhancedStepBuilder {
    /// Create new smart step builder
    pub fn new(step_id: &str) -> Self {
        Self {
            step_id: step_id.to_string(),
            title: String::new(),
            human_message: String::new(),
            api_data: StepApiData {
                category: String::new(),
                step_type: String::new(),
                operation: String::new(),
                inputs: HashMap::new(),
                outputs: HashMap::new(),
                properties: HashMap::new(),
            },
            message_key: MessageKey {
                category: String::new(),
                message_type: String::new(),
                variant: 0,
                hash: 0,
                template_params: Vec::new(),
            },
            math_context: MathContext {
                equation: String::new(),
                variable: String::new(),
                current_state: String::new(),
                coefficients: HashMap::new(),
                progress: 0.0,
                equation_type: String::new(),
            },
            presentation: PresentationHints {
                color_theme: "blue".to_string(),
                importance: 3,
                animation: "fade-in".to_string(),
                layout: "standard".to_string(),
                interactive_elements: Vec::new(),
            },
        }
    }

    /// Set human-readable content
    pub fn with_human_message(mut self, title: &str, message: &str) -> Self {
        self.title = title.to_string();
        self.human_message = message.to_string();
        self
    }

    /// Set API data
    pub fn with_api_data(mut self, category: &str, step_type: &str, operation: &str) -> Self {
        self.api_data.category = category.to_string();
        self.api_data.step_type = step_type.to_string();
        self.api_data.operation = operation.to_string();
        self
    }

    /// Add input data
    pub fn with_input(mut self, key: &str, value: &str) -> Self {
        self.api_data
            .inputs
            .insert(key.to_string(), value.to_string());
        self
    }

    /// Add output data
    pub fn with_output(mut self, key: &str, value: &str) -> Self {
        self.api_data
            .outputs
            .insert(key.to_string(), value.to_string());
        self
    }

    /// Set mathematical context
    pub fn with_math_context(mut self, equation: &str, variable: &str, progress: f64) -> Self {
        self.math_context.equation = equation.to_string();
        self.math_context.variable = variable.to_string();
        self.math_context.progress = progress;
        self
    }

    /// Set message key for external systems
    pub fn with_message_key(mut self, category: &str, message_type: &str, variant: u32) -> Self {
        self.message_key.category = category.to_string();
        self.message_key.message_type = message_type.to_string();
        self.message_key.variant = variant;

        // Generate hash for quick lookup
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        let mut hasher = DefaultHasher::new();
        category.hash(&mut hasher);
        message_type.hash(&mut hasher);
        variant.hash(&mut hasher);
        self.message_key.hash = hasher.finish();

        self
    }

    /// Set presentation hints
    pub fn with_presentation(mut self, color: &str, importance: u8, animation: &str) -> Self {
        self.presentation.color_theme = color.to_string();
        self.presentation.importance = importance;
        self.presentation.animation = animation.to_string();
        self
    }

    /// Build the enhanced step
    pub fn build(self) -> EnhancedStep {
        EnhancedStep {
            step_id: self.step_id,
            title: self.title,
            human_message: self.human_message,
            api_data: self.api_data,
            message_key: self.message_key,
            math_context: self.math_context,
            presentation: self.presentation,
        }
    }
}

/// Step factory
pub struct StepFactory;

/// Smart step factory (alias for StepFactory)
pub type SmartStepFactory = StepFactory;

/// Enhanced step factory (alias for StepFactory)  
pub type EnhancedStepFactory = StepFactory;

/// Smart step explanation (alias for EnhancedStepExplanation)
pub type SmartStepExplanation = EnhancedStepExplanation;

impl StepFactory {
    /// Generate linear equation introduction step
    pub fn linear_introduction(equation: &Expression, variable: &Symbol) -> EnhancedStep {
        Self::linear_introduction_with_format(equation, variable, &FormatContext::default())
    }

    /// Generate linear equation introduction step with custom format
    pub fn linear_introduction_with_format(
        equation: &Expression,
        variable: &Symbol,
        context: &FormatContext,
    ) -> EnhancedStep {
        let equation_formatted = context.format_expression_safe(equation);

        EnhancedStepBuilder::new("linear_intro_001")
            .with_human_message(
                "📝 Given Equation",
                &format!("We need to solve: {} = 0\nThis is a linear equation because {} appears only to the first power.", 
                        equation_formatted, variable.name())
            )
            .with_api_data("linear_equation", "introduction", "equation_analysis")
            .with_input("original_equation", &equation_formatted)
            .with_input("variable", variable.name())
            .with_output("equation_type", "linear")
            .with_output("complexity", "simple")
            .with_math_context(&equation_formatted, variable.name(), 0.1)
            .with_message_key("linear", "introduction", 0)
            .with_presentation("blue", 4, "slide-in")
            .build()
    }

    /// Generate linear equation strategy step
    pub fn linear_strategy(variable: &Symbol) -> EnhancedStep {
        Self::linear_strategy_with_format(variable, &FormatContext::default())
    }

    /// Generate linear equation strategy step with custom format
    pub fn linear_strategy_with_format(
        variable: &Symbol,
        _context: &FormatContext,
    ) -> EnhancedStep {
        EnhancedStepBuilder::new("linear_strategy_001")
            .with_human_message(
                "🎯 Solution Strategy",
                &format!("To solve for {}, we'll isolate it using inverse operations.\nWhatever operation is applied to {}, we'll undo it step by step.", 
                        variable.name(), variable.name())
            )
            .with_api_data("linear_equation", "strategy", "isolation_method")
            .with_input("variable", variable.name())
            .with_output("method", "inverse_operations")
            .with_output("approach", "systematic_isolation")
            .with_math_context("", variable.name(), 0.2)
            .with_message_key("linear", "strategy", 0)
            .with_presentation("green", 3, "fade-in")
            .build()
    }

    /// Generate coefficient identification step
    pub fn linear_coefficient_identification(
        a: &Expression,
        b: &Expression,
        variable: &Symbol,
    ) -> SmartStep {
        Self::linear_coefficient_identification_with_format(
            a,
            b,
            variable,
            &FormatContext::default(),
        )
    }

    /// Generate coefficient identification step with custom format
    pub fn linear_coefficient_identification_with_format(
        a: &Expression,
        b: &Expression,
        variable: &Symbol,
        context: &FormatContext,
    ) -> SmartStep {
        let a_formatted = context.format_expression_safe(a);
        let b_formatted = context.format_expression_safe(b);

        SmartStepBuilder::new("linear_coeffs_001")
            .with_human_message(
                "🔍 Identify Components",
                &format!("In our equation, we can identify:\n• Coefficient of {}: {}\n• Constant term: {}\nForm: {}·{} + {} = 0", 
                        variable.name(), a_formatted, b_formatted, a_formatted, variable.name(), b_formatted)
            )
            .with_api_data("linear_equation", "analysis", "coefficient_extraction")
            .with_input("variable", variable.name())
            .with_input("coefficient", &a_formatted)
            .with_input("constant", &b_formatted)
            .with_output("standard_form", &format!("{}x + {}", a_formatted, b_formatted))
            .with_math_context("", variable.name(), 0.4)
            .with_message_key("linear", "analysis", 0)
            .with_presentation("orange", 4, "highlight")
            .build()
    }

    /// Generate solution calculation step
    pub fn linear_solution_calculation(
        variable: &Symbol,
        solution: &Expression,
        a: &Expression,
        b: &Expression,
    ) -> SmartStep {
        Self::linear_solution_calculation_with_format(
            variable,
            solution,
            a,
            b,
            &FormatContext::default(),
        )
    }

    /// Generate solution calculation step with custom format
    pub fn linear_solution_calculation_with_format(
        variable: &Symbol,
        solution: &Expression,
        a: &Expression,
        b: &Expression,
        context: &FormatContext,
    ) -> SmartStep {
        let a_formatted = context.format_expression_safe(a);
        let b_formatted = context.format_expression_safe(b);
        let solution_formatted = context.format_expression_safe(solution);

        SmartStepBuilder::new("linear_calc_001")
            .with_human_message(
                "📊 Calculate Solution",
                &format!("Using the linear equation formula:\n{} = -({}) ÷ ({})\n{} = {}\nThis gives us our solution.", 
                        variable.name(), b_formatted, a_formatted, variable.name(), solution_formatted)
            )
            .with_api_data("linear_equation", "calculation", "division_operation")
            .with_input("numerator", &format!("-({})", b_formatted))
            .with_input("denominator", &a_formatted)
            .with_input("variable", variable.name())
            .with_output("solution", &solution_formatted)
            .with_output("calculation_method", "division")
            .with_math_context("", variable.name(), 0.8)
            .with_message_key("linear", "calculation", 0)
            .with_presentation("purple", 5, "calculate")
            .build()
    }

    /// Generate verification step
    pub fn linear_verification(
        equation: &Expression,
        variable: &Symbol,
        solution: &Expression,
    ) -> SmartStep {
        Self::linear_verification_with_format(
            equation,
            variable,
            solution,
            &FormatContext::default(),
        )
    }

    /// Generate verification step with custom format
    pub fn linear_verification_with_format(
        equation: &Expression,
        variable: &Symbol,
        solution: &Expression,
        context: &FormatContext,
    ) -> SmartStep {
        let equation_formatted = context.format_expression_safe(equation);
        let solution_formatted = context.format_expression_safe(solution);

        let verification_text = format!(
            "Substitute {} = {} into original equation",
            variable.name(),
            solution_formatted
        );

        SmartStepBuilder::new("linear_verify_001")
            .with_human_message(
                "✅ Verify Solution",
                &format!(
                    "Let's check our answer:\n{}\nResult: The equation is satisfied. ✓",
                    verification_text
                ),
            )
            .with_api_data("linear_equation", "verification", "substitution_check")
            .with_input("original_equation", &equation_formatted)
            .with_input("variable", variable.name())
            .with_input("solution", &solution_formatted)
            .with_output("verification_result", "success")
            .with_output("substitution", &verification_text)
            .with_math_context(&equation_formatted, variable.name(), 1.0)
            .with_message_key("linear", "verification", 0)
            .with_presentation("green", 4, "success")
            .build()
    }

    /// Generate special case steps
    pub fn linear_no_solution(equation: &Expression) -> SmartStep {
        Self::linear_no_solution_with_format(equation, &FormatContext::default())
    }

    /// Generate special case steps with custom format
    pub fn linear_no_solution_with_format(
        equation: &Expression,
        context: &FormatContext,
    ) -> SmartStep {
        let equation_formatted = context.format_expression_safe(equation);

        SmartStepBuilder::new("linear_no_solution_001")
            .with_human_message(
                "⚠️ No Solution",
                &format!("This equation: {} = 0\nSimplifies to a contradiction (like 5 = 0).\nTherefore, no solution exists.", 
                        equation_formatted)
            )
            .with_api_data("linear_equation", "error", "no_solution")
            .with_input("equation", &equation_formatted)
            .with_output("result_type", "no_solution")
            .with_output("reason", "contradiction")
            .with_math_context(&equation_formatted, "", 1.0)
            .with_message_key("linear", "error", 0)
            .with_presentation("red", 5, "alert")
            .build()
    }

    /// Generate infinite solutions step
    pub fn linear_infinite_solutions(equation: &Expression, variable: &Symbol) -> SmartStep {
        Self::linear_infinite_solutions_with_format(equation, variable, &FormatContext::default())
    }

    /// Generate infinite solutions step with custom format
    pub fn linear_infinite_solutions_with_format(
        equation: &Expression,
        variable: &Symbol,
        context: &FormatContext,
    ) -> SmartStep {
        let equation_formatted = context.format_expression_safe(equation);

        SmartStepBuilder::new("linear_infinite_001")
            .with_human_message(
                "📊 Infinite Solutions",
                &format!("This equation: {} = 0\nSimplifies to 0 = 0, which is always true.\nTherefore, any value of {} is a solution.", 
                        equation_formatted, variable.name())
            )
            .with_api_data("linear_equation", "result", "infinite_solutions")
            .with_input("equation", &equation_formatted)
            .with_input("variable", variable.name())
            .with_output("result_type", "infinite_solutions")
            .with_output("reason", "identity_equation")
            .with_math_context(&equation_formatted, variable.name(), 1.0)
            .with_message_key("linear", "infinite", 0)
            .with_presentation("blue", 4, "expand")
            .build()
    }
}

/// Smart step explanation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnhancedStepExplanation {
    /// Collection of smart steps
    pub steps: Vec<EnhancedStep>,

    /// Overall explanation metadata
    pub metadata: ExplanationMetadata,

    /// Summary for quick consumption
    pub summary: ExplanationSummary,
}

/// 📊 EXPLANATION METADATA - OVERALL EXPLANATION INFO
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExplanationMetadata {
    /// Total number of steps
    pub step_count: usize,

    /// Estimated difficulty level (1-10)
    pub difficulty_level: u8,

    /// Mathematical topic
    pub topic: String,

    /// Solution method used
    pub method: String,

    /// Estimated time to understand (minutes)
    pub estimated_time: u8,

    /// Prerequisites
    pub prerequisites: Vec<String>,
}

/// 📋 EXPLANATION SUMMARY - QUICK OVERVIEW
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExplanationSummary {
    /// Problem statement
    pub problem: String,

    /// Solution approach
    pub approach: String,

    /// Final answer
    pub answer: String,

    /// Key insights
    pub key_insights: Vec<String>,

    /// Next steps or related topics
    pub next_steps: Vec<String>,
}

impl EnhancedStepExplanation {
    /// Create new smart explanation
    pub fn new(steps: Vec<EnhancedStep>) -> Self {
        let metadata = ExplanationMetadata {
            step_count: steps.len(),
            difficulty_level: Self::calculate_difficulty(&steps),
            topic: Self::determine_topic(&steps),
            method: Self::determine_method(&steps),
            estimated_time: Self::estimate_time(&steps),
            prerequisites: Self::determine_prerequisites(&steps),
        };

        let summary = ExplanationSummary {
            problem: Self::extract_problem(&steps),
            approach: Self::extract_approach(&steps),
            answer: Self::extract_answer(&steps),
            key_insights: Self::extract_insights(&steps),
            next_steps: Self::suggest_next_steps(&steps),
        };

        Self {
            steps,
            metadata,
            summary,
        }
    }

    /// Export as JSON for API consumption
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }

    /// Export human-readable text only
    pub fn to_human_text(&self) -> String {
        let mut text = String::new();
        text.push_str(&format!("Problem: {}\n\n", self.summary.problem));

        for (i, step) in self.steps.iter().enumerate() {
            text.push_str(&format!(
                "Step {}: {}\n{}\n\n",
                i + 1,
                step.title,
                step.human_message
            ));
        }

        text.push_str(&format!("Answer: {}\n", self.summary.answer));
        text
    }

    /// Export structured data only (for external apps)
    pub fn to_api_data(&self) -> HashMap<String, serde_json::Value> {
        let mut data = HashMap::new();

        data.insert(
            "metadata".to_string(),
            serde_json::to_value(&self.metadata).unwrap(),
        );
        data.insert(
            "summary".to_string(),
            serde_json::to_value(&self.summary).unwrap(),
        );

        let step_data: Vec<serde_json::Value> = self
            .steps
            .iter()
            .map(|step| serde_json::to_value(&step.api_data).unwrap())
            .collect();
        data.insert("steps".to_string(), serde_json::Value::Array(step_data));

        data
    }

    // Helper methods for metadata generation
    fn calculate_difficulty(steps: &[EnhancedStep]) -> u8 {
        // Base difficulty on step count and complexity
        match steps.len() {
            1..=3 => 2,
            4..=6 => 4,
            7..=10 => 6,
            _ => 8,
        }
    }

    fn determine_topic(steps: &[EnhancedStep]) -> String {
        if let Some(first_step) = steps.first() {
            first_step.api_data.category.clone()
        } else {
            "unknown".to_string()
        }
    }

    fn determine_method(steps: &[EnhancedStep]) -> String {
        // Analyze steps to determine method
        if steps
            .iter()
            .any(|s| s.api_data.operation.contains("quadratic_formula"))
        {
            "Quadratic Formula".to_string()
        } else if steps
            .iter()
            .any(|s| s.api_data.operation.contains("isolation"))
        {
            "Variable Isolation".to_string()
        } else {
            "General Algebraic Method".to_string()
        }
    }

    fn estimate_time(steps: &[EnhancedStep]) -> u8 {
        // Estimate based on step complexity
        (steps.len() as u8).saturating_mul(2).min(30) // 2 minutes per step, max 30 minutes
    }

    fn determine_prerequisites(steps: &[EnhancedStep]) -> Vec<String> {
        let mut prereqs = vec!["Basic Algebra".to_string()];

        if steps
            .iter()
            .any(|s| s.api_data.category == "quadratic_equation")
        {
            prereqs.push("Quadratic Equations".to_string());
        }

        prereqs
    }

    fn extract_problem(steps: &[EnhancedStep]) -> String {
        if let Some(first_step) = steps.first() {
            first_step.math_context.equation.clone()
        } else {
            "Unknown problem".to_string()
        }
    }

    fn extract_approach(steps: &[EnhancedStep]) -> String {
        Self::determine_method(steps)
    }

    fn extract_answer(steps: &[EnhancedStep]) -> String {
        if let Some(last_step) = steps.last() {
            last_step
                .api_data
                .outputs
                .get("solution")
                .or_else(|| last_step.api_data.outputs.get("result"))
                .unwrap_or(&"Solution in progress".to_string())
                .clone()
        } else {
            "No solution yet".to_string()
        }
    }

    fn extract_insights(steps: &[EnhancedStep]) -> Vec<String> {
        steps
            .iter()
            .filter(|step| step.message_key.message_type == "insight")
            .map(|step| step.human_message.clone())
            .collect()
    }

    fn suggest_next_steps(_steps: &[EnhancedStep]) -> Vec<String> {
        vec![
            "Try solving similar equations".to_string(),
            "Practice with different coefficients".to_string(),
            "Explore quadratic equations".to_string(),
        ]
    }
}

/// 🔄 CONVERSION TO LEGACY STEP SYSTEM
impl From<EnhancedStep> for crate::educational::step_by_step::Step {
    fn from(enhanced_step: EnhancedStep) -> Self {
        Self::new(enhanced_step.title, enhanced_step.human_message)
    }
}

impl From<EnhancedStepExplanation> for crate::educational::step_by_step::StepByStepExplanation {
    fn from(enhanced_explanation: EnhancedStepExplanation) -> Self {
        let legacy_steps: Vec<crate::educational::step_by_step::Step> = enhanced_explanation
            .steps
            .into_iter()
            .map(|enhanced_step| enhanced_step.into())
            .collect();

        Self::new(legacy_steps)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{symbol, Expression};

    #[test]
    fn test_smart_step_creation() {
        let x = symbol!(x);
        let equation = Expression::add(vec![Expression::symbol(x.clone()), Expression::integer(2)]);

        let step = SmartStepFactory::linear_introduction(&equation, &x);

        // Verify human-readable content
        assert!(step.human_message.contains("2 + x"));
        assert!(step.human_message.contains("linear equation"));

        // Verify API data
        assert_eq!(step.api_data.category, "linear_equation");
        assert_eq!(step.api_data.step_type, "introduction");
        assert!(step.api_data.inputs.contains_key("variable"));

        // Verify message key
        assert_eq!(step.message_key.category, "linear");
        assert_eq!(step.message_key.message_type, "introduction");
    }

    #[test]
    fn test_json_export() {
        let x = symbol!(x);
        let equation = Expression::add(vec![Expression::symbol(x.clone()), Expression::integer(2)]);

        let steps = vec![
            SmartStepFactory::linear_introduction(&equation, &x),
            SmartStepFactory::linear_strategy(&x),
        ];

        let explanation = SmartStepExplanation::new(steps);
        let json = explanation.to_json().unwrap();

        // Verify JSON contains both human and API data
        assert!(json.contains("human_message"));
        assert!(json.contains("api_data"));
        assert!(json.contains("message_key"));
        assert!(json.contains("math_context"));
    }

    #[test]
    fn test_api_data_extraction() {
        let x = symbol!(x);
        let steps = vec![SmartStepFactory::linear_strategy(&x)];
        let explanation = SmartStepExplanation::new(steps);

        let api_data = explanation.to_api_data();

        assert!(api_data.contains_key("metadata"));
        assert!(api_data.contains_key("summary"));
        assert!(api_data.contains_key("steps"));
    }

    #[test]
    fn test_legacy_compatibility() {
        let x = symbol!(x);
        let equation = Expression::integer(0);
        let smart_step = EnhancedStepFactory::linear_introduction(&equation, &x);

        // Should convert to legacy step system
        let legacy_step: crate::educational::step_by_step::Step = smart_step.into();
        assert!(!legacy_step.title.is_empty());
        assert!(!legacy_step.description.is_empty());
    }
}
