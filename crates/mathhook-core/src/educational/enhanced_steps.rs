//! Enhanced steps with human messages and structured API data
//!
//! Provides both human-readable educational content and machine-consumable
//! structured data for external applications.

mod formatting;
mod generation;

pub use formatting::{
    EnhancedStepExplanation, ExplanationMetadata, ExplanationSummary, FormatContext,
    PresentationHints,
};

pub use generation::{
    DifficultyLevel, EducationalResult, EnhancedStep, EnhancedStepBuilder, EnhancedStepFactory,
    MathContext, MessageKey, SmartStep, SmartStepBuilder, SmartStepFactory, StepApiData,
    StepFactory,
};

pub type SmartStepExplanation = EnhancedStepExplanation;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{symbol, Expression};

    #[test]
    fn test_smart_step_creation() {
        let x = symbol!(x);
        let equation = Expression::add(vec![Expression::symbol(x.clone()), Expression::integer(2)]);

        let step = SmartStepFactory::linear_introduction(&equation, &x);

        assert!(step.human_message.contains("2 + x"));
        assert!(step.human_message.contains("linear equation"));

        assert_eq!(step.api_data.category, "linear_equation");
        assert_eq!(step.api_data.step_type, "introduction");
        assert!(step.api_data.inputs.contains_key("variable"));

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

        let legacy_step: crate::educational::step_by_step::Step = smart_step.into();
        assert!(!legacy_step.title.is_empty());
        assert!(!legacy_step.description.is_empty());
    }
}
