//! Step generation methods including factories and builders

use crate::core::{Expression, Symbol};
use crate::educational::step_by_step::Step;
use serde::{Deserialize, Serialize};
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

use super::formatting::FormatContext;

/// Enhanced step with human and API data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnhancedStep {
    pub step_id: String,
    pub title: String,
    pub human_message: String,
    pub api_data: StepApiData,
    pub message_key: MessageKey,
    pub math_context: MathContext,
    pub presentation: super::formatting::PresentationHints,
}

/// Step API data for external applications
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StepApiData {
    pub category: String,
    pub step_type: String,
    pub operation: String,
    pub inputs: HashMap<String, String>,
    pub outputs: HashMap<String, String>,
    pub properties: HashMap<String, serde_json::Value>,
}

/// Message key for external message systems
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageKey {
    pub category: String,
    pub message_type: String,
    pub variant: u32,
    pub hash: u64,
    pub template_params: Vec<String>,
}

/// Mathematical context information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MathContext {
    pub equation: String,
    pub variable: String,
    pub current_state: String,
    pub coefficients: HashMap<String, String>,
    pub progress: f64,
    pub equation_type: String,
}

/// Enhanced step builder
pub struct EnhancedStepBuilder {
    step_id: String,
    title: String,
    human_message: String,
    api_data: StepApiData,
    message_key: MessageKey,
    math_context: MathContext,
    presentation: super::formatting::PresentationHints,
}

impl EnhancedStepBuilder {
    /// Create new enhanced step builder
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
            presentation: super::formatting::PresentationHints::default(),
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

/// Smart step type (legacy compatibility)
pub type SmartStep = Step;

/// Smart step builder for legacy system
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

/// Step factory for creating enhanced steps
pub struct StepFactory;

/// Smart step factory (alias)
pub type SmartStepFactory = StepFactory;

/// Enhanced step factory (alias)
pub type EnhancedStepFactory = StepFactory;

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
                "Given Equation",
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
                "Solution Strategy",
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
                "Identify Components",
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
                "Calculate Solution",
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
                "Verify Solution",
                &format!(
                    "Let's check our answer:\n{}\nResult: The equation is satisfied.",
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

    /// Generate no solution step
    pub fn linear_no_solution(equation: &Expression) -> SmartStep {
        Self::linear_no_solution_with_format(equation, &FormatContext::default())
    }

    /// Generate no solution step with custom format
    pub fn linear_no_solution_with_format(
        equation: &Expression,
        context: &FormatContext,
    ) -> SmartStep {
        let equation_formatted = context.format_expression_safe(equation);

        SmartStepBuilder::new("linear_no_solution_001")
            .with_human_message(
                "No Solution",
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
                "Infinite Solutions",
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
