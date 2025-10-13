//! Format conversion and presentation methods for enhanced steps

use crate::core::Expression;
use crate::formatter::latex::LaTeXFormatter;
use crate::formatter::{FormattingError, MathLanguage};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Format context for enhanced steps
#[derive(Debug, Clone)]
pub struct FormatContext {
    pub target_format: MathLanguage,
    pub include_intermediate_steps: bool,
    pub verbosity_level: u8,
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
                Ok(expr.to_string())
            }
            MathLanguage::Simple | MathLanguage::Human => Ok(expr.to_string()),
            MathLanguage::Json => {
                serde_json::to_string(expr).map_err(|e| FormattingError::SerializationError {
                    message: e.to_string(),
                })
            }
            MathLanguage::Markdown => {
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

/// Presentation hints for visual rendering
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PresentationHints {
    pub color_theme: String,
    pub importance: u8,
    pub animation: String,
    pub layout: String,
    pub interactive_elements: Vec<String>,
}

impl Default for PresentationHints {
    fn default() -> Self {
        Self {
            color_theme: "blue".to_string(),
            importance: 3,
            animation: "fade-in".to_string(),
            layout: "standard".to_string(),
            interactive_elements: Vec::new(),
        }
    }
}

/// Enhanced step explanation with multiple output formats
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnhancedStepExplanation {
    pub steps: Vec<super::EnhancedStep>,
    pub metadata: ExplanationMetadata,
    pub summary: ExplanationSummary,
}

/// Explanation metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExplanationMetadata {
    pub step_count: usize,
    pub difficulty_level: u8,
    pub topic: String,
    pub method: String,
    pub estimated_time: u8,
    pub prerequisites: Vec<String>,
}

/// Explanation summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExplanationSummary {
    pub problem: String,
    pub approach: String,
    pub answer: String,
    pub key_insights: Vec<String>,
    pub next_steps: Vec<String>,
}

impl EnhancedStepExplanation {
    /// Create new explanation from steps
    pub fn new(steps: Vec<super::EnhancedStep>) -> Self {
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

    fn calculate_difficulty(steps: &[super::EnhancedStep]) -> u8 {
        match steps.len() {
            1..=3 => 2,
            4..=6 => 4,
            7..=10 => 6,
            _ => 8,
        }
    }

    fn determine_topic(steps: &[super::EnhancedStep]) -> String {
        if let Some(first_step) = steps.first() {
            first_step.api_data.category.clone()
        } else {
            "unknown".to_string()
        }
    }

    fn determine_method(steps: &[super::EnhancedStep]) -> String {
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

    fn estimate_time(steps: &[super::EnhancedStep]) -> u8 {
        (steps.len() as u8).saturating_mul(2).min(30)
    }

    fn determine_prerequisites(steps: &[super::EnhancedStep]) -> Vec<String> {
        let mut prereqs = vec!["Basic Algebra".to_string()];

        if steps
            .iter()
            .any(|s| s.api_data.category == "quadratic_equation")
        {
            prereqs.push("Quadratic Equations".to_string());
        }

        prereqs
    }

    fn extract_problem(steps: &[super::EnhancedStep]) -> String {
        if let Some(first_step) = steps.first() {
            first_step.math_context.equation.clone()
        } else {
            "Unknown problem".to_string()
        }
    }

    fn extract_approach(steps: &[super::EnhancedStep]) -> String {
        Self::determine_method(steps)
    }

    fn extract_answer(steps: &[super::EnhancedStep]) -> String {
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

    fn extract_insights(steps: &[super::EnhancedStep]) -> Vec<String> {
        steps
            .iter()
            .filter(|step| step.message_key.message_type == "insight")
            .map(|step| step.human_message.clone())
            .collect()
    }

    fn suggest_next_steps(_steps: &[super::EnhancedStep]) -> Vec<String> {
        vec![
            "Try solving similar equations".to_string(),
            "Practice with different coefficients".to_string(),
            "Explore quadratic equations".to_string(),
        ]
    }
}

/// Conversion to legacy step system
impl From<super::EnhancedStep> for crate::educational::step_by_step::Step {
    fn from(enhanced_step: super::EnhancedStep) -> Self {
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
