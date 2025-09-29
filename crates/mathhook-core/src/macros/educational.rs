//! Educational step generation macros
//!
//! Step-by-step explanation generation for mathematical operations.
//! These macros streamline the creation of educational content and
//! provide consistent formatting across all mathematical operations.

/// Educational step generation
///
/// This macro provides ergonomic creation of educational steps and
/// step-by-step explanations for mathematical operations.
///
/// # Examples
///
/// ```rust
/// use mathhook_core::step;
/// use mathhook_core::educational::step_by_step::Step;
///
/// // Basic step
/// let basic = step!("Simplify Expression", "Combine like terms");
/// assert_eq!(basic.title, "Simplify Expression");
/// assert_eq!(basic.description, "Combine like terms");
///
/// // Educational sequence
/// let sequence = step!(sequence:
///     ("Step 1", "Identify the equation type"),
///     ("Step 2", "Apply appropriate solving method"),
///     ("Step 3", "Simplify the result")
/// );
/// assert_eq!(sequence.len(), 3);
/// ```
#[macro_export]
macro_rules! step {
    // Basic step
    ($title:literal, $desc:literal) => {
        $crate::educational::step_by_step::Step::new($title, $desc)
    };

    // Enhanced step with LaTeX
    (enhanced: $title:literal, $desc:literal, $latex:expr) => {{
        let mut step = $crate::educational::step_by_step::Step::new($title, $desc);
        step.latex = Some($latex);
        step
    }};

    // Educational sequence
    (sequence: $(($title:literal, $desc:literal)),* $(,)?) => {
        vec![$($crate::educational::step_by_step::Step::new($title, $desc)),*]
    };

    // Solver step with explanation
    (solver: $operation:literal, $equation:expr, $variable:expr, $result:expr) => {{
        use $crate::educational::enhanced_steps::EnhancedStepBuilder;

        EnhancedStepBuilder::new(&format!("{}_step", $operation))
            .with_human_message($operation, &format!("Solving {} for {}", $equation, $variable))
            .with_api_data("solver", $operation, "equation_solving")
            .with_input("equation", &$equation.to_latex())
            .with_output("result", &$result.to_latex())
            .build()
    }};

    // Mathematical rule explanation
    (rule: $rule_name:literal, $before:expr, $after:expr, $explanation:literal) => {{
        use $crate::educational::enhanced_steps::EnhancedStepBuilder;

        EnhancedStepBuilder::new(&format!("rule_{}", $rule_name))
            .with_human_message("Mathematical Rule", $explanation)
            .with_api_data("simplification", $rule_name, "rule_application")
            .with_input("before", &$before.to_latex())
            .with_output("after", &$after.to_latex())
            .build()
    }};

    // Progress tracking
    (progress: $current:expr, $total:expr, $description:literal) => {{
        use $crate::educational::enhanced_steps::EnhancedStepBuilder;

        EnhancedStepBuilder::new("progress")
            .with_math_context("", "", ($current as f64) / ($total as f64))
            .with_human_message("Progress", &format!("{} (Step {} of {})", $description, $current, $total))
            .build()
    }};

    // Explanation with context
    (explain: $title:literal, $explanation:literal, $context:expr) => {{
        use $crate::educational::enhanced_steps::EnhancedStepBuilder;

        EnhancedStepBuilder::new("explanation")
            .with_human_message($title, $explanation)
            .with_api_data("explanation", "context", "mathematical_reasoning")
            .with_input("context", &format!("{}", $context))
            .build()
    }};

    // Hint generation
    (hint: $title:literal, $hint_text:literal) => {{
        use $crate::educational::enhanced_steps::EnhancedStepBuilder;

        EnhancedStepBuilder::new("hint")
            .with_human_message($title, $hint_text)
            .with_api_data("educational", "hint", "guidance")
            .with_presentation("yellow", 3, "fade-in")
            .build()
    }};

    // Warning or caution step
    (warning: $title:literal, $warning_text:literal) => {{
        use $crate::educational::enhanced_steps::EnhancedStepBuilder;

        EnhancedStepBuilder::new("warning")
            .with_human_message($title, $warning_text)
            .with_api_data("educational", "warning", "caution")
            .with_presentation("red", 5, "pulse")
            .build()
    }};

    // Success or completion step
    (success: $title:literal, $success_text:literal) => {{
        use $crate::educational::enhanced_steps::EnhancedStepBuilder;

        EnhancedStepBuilder::new("success")
            .with_human_message($title, $success_text)
            .with_api_data("educational", "success", "completion")
            .with_presentation("green", 4, "slide-up")
            .build()
    }};
}

#[cfg(test)]
mod tests {
    use crate::educational::step_by_step::Step;

    #[test]
    fn test_step_basic() {
        let step = step!("Test Title", "Test Description");
        assert_eq!(step.title, "Test Title");
        assert_eq!(step.description, "Test Description");
    }

    #[test]
    fn test_step_enhanced() {
        let step = step!(enhanced: "Enhanced Step", "Description", "x^2 + 1".to_string());
        assert_eq!(step.title, "Enhanced Step");
        assert_eq!(step.description, "Description");
        assert_eq!(step.latex, Some("x^2 + 1".to_string()));
    }

    #[test]
    fn test_step_sequence() {
        let sequence = step!(sequence:
            ("Step 1", "First step"),
            ("Step 2", "Second step"),
            ("Step 3", "Third step")
        );

        assert_eq!(sequence.len(), 3);
        assert_eq!(sequence[0].title, "Step 1");
        assert_eq!(sequence[1].title, "Step 2");
        assert_eq!(sequence[2].title, "Step 3");
    }

    #[test]
    fn test_step_sequence_empty() {
        let sequence: Vec<Step> = step!(sequence:);
        assert_eq!(sequence.len(), 0);
    }

    #[test]
    fn test_step_sequence_single() {
        let sequence: Vec<Step> = step!(sequence: ("Only Step", "Single step"));
        assert_eq!(sequence.len(), 1);
        assert_eq!(sequence[0].title, "Only Step");
        assert_eq!(sequence[0].description, "Single step");
    }
}
