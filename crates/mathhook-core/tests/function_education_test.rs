//! Content validation tests for function educational explanations
//!
//! These tests validate that function step-by-step explanations contain
//! actual mathematical content, not just structural scaffolding.

use mathhook_core::functions::education::FunctionEducator;
use mathhook_core::educational::step_by_step::StepByStepExplanation;
use mathhook_core::{Expression, Symbol};

/// Helper: Check if explanation contains text (case-insensitive)
fn has_step_containing(explanation: &StepByStepExplanation, text: &str) -> bool {
    let text_lower = text.to_lowercase();
    explanation.steps.iter().any(|step| {
        step.description.to_lowercase().contains(&text_lower)
            || step.title.to_lowercase().contains(&text_lower)
    })
}

/// Helper: Check if ALL texts appear in explanation
fn has_steps_containing_all(explanation: &StepByStepExplanation, texts: &[&str]) -> bool {
    texts.iter().all(|text| has_step_containing(explanation, text))
}

#[test]
fn test_sin_special_value_detection() {
    let educator = FunctionEducator::new();
    let args = vec![Expression::integer(0)];
    let explanation = educator.explain_function_operation("sin", &args, "evaluation");

    assert!(
        explanation.steps.len() >= 5,
        "Sin explanation should have at least 5 steps, got {}",
        explanation.steps.len()
    );

    assert!(
        has_step_containing(&explanation, "special") || has_step_containing(&explanation, "known"),
        "Must mention special values"
    );

    assert!(
        has_step_containing(&explanation, "unit circle") || has_step_containing(&explanation, "y-coordinate"),
        "Must explain unit circle context"
    );

    assert!(
        has_step_containing(&explanation, "domain") || has_step_containing(&explanation, "all real"),
        "Must specify domain"
    );
}

#[test]
fn test_cos_range_explained() {
    let educator = FunctionEducator::new();
    let args = vec![Expression::integer(0)];
    let explanation = educator.explain_function_operation("cos", &args, "evaluation");

    assert!(
        explanation.steps.len() >= 5,
        "Cos explanation should have at least 5 steps"
    );

    assert!(
        has_step_containing(&explanation, "range") && (has_step_containing(&explanation, "[-1, 1]") || has_step_containing(&explanation, "-1") && has_step_containing(&explanation, "1")),
        "Must specify range [-1, 1]"
    );

    assert!(
        has_step_containing(&explanation, "x-coordinate") || has_step_containing(&explanation, "unit circle"),
        "Must explain cosine meaning"
    );
}

#[test]
fn test_tan_asymptote_explained() {
    let educator = FunctionEducator::new();
    let args = vec![Expression::integer(0)];
    let explanation = educator.explain_function_operation("tan", &args, "evaluation");

    assert!(
        explanation.steps.len() >= 5,
        "Tan explanation should have at least 5 steps"
    );

    assert!(
        has_step_containing(&explanation, "undefined") || has_step_containing(&explanation, "asymptote") || has_step_containing(&explanation, "π/2"),
        "Must mention asymptotes or undefined values"
    );

    assert!(
        has_step_containing(&explanation, "sin") || has_step_containing(&explanation, "cos"),
        "Must relate to sin/cos"
    );
}

#[test]
fn test_arcsin_domain_restriction() {
    let educator = FunctionEducator::new();
    let args = vec![Expression::integer(0)];
    let explanation = educator.explain_function_operation("arcsin", &args, "evaluation");

    assert!(
        explanation.steps.len() >= 5,
        "Arcsin explanation should have at least 5 steps"
    );

    assert!(
        has_step_containing(&explanation, "[-1, 1]") || (has_step_containing(&explanation, "-1") && has_step_containing(&explanation, "1")),
        "Must specify domain [-1, 1]"
    );

    assert!(
        has_step_containing(&explanation, "inverse") || has_step_containing(&explanation, "angle"),
        "Must explain inverse function concept"
    );

    assert!(
        has_step_containing(&explanation, "principal") || has_step_containing(&explanation, "branch"),
        "Must mention principal branch"
    );
}

#[test]
fn test_arccos_range_principal_branch() {
    let educator = FunctionEducator::new();
    let args = vec![Expression::integer(1)];
    let explanation = educator.explain_function_operation("arccos", &args, "evaluation");

    assert!(
        has_step_containing(&explanation, "[0, π]") || (has_step_containing(&explanation, "0") && has_step_containing(&explanation, "π")),
        "Must specify range [0, π]"
    );

    assert!(
        has_step_containing(&explanation, "principal"),
        "Must mention principal branch for range"
    );
}

#[test]
fn test_ln_domain_positive_only() {
    let educator = FunctionEducator::new();
    let args = vec![Expression::integer(1)];
    let explanation = educator.explain_function_operation("ln", &args, "evaluation");

    assert!(
        explanation.steps.len() >= 5,
        "Ln explanation should have at least 5 steps"
    );

    assert!(
        has_step_containing(&explanation, "(0, ∞)") || has_step_containing(&explanation, "positive") || has_step_containing(&explanation, "> 0"),
        "Must specify domain: only positive numbers"
    );

    assert!(
        has_step_containing(&explanation, "natural") || has_step_containing(&explanation, "base e"),
        "Must explain natural logarithm"
    );

    assert!(
        has_step_containing(&explanation, "special"),
        "Must mention special values"
    );
}

#[test]
fn test_log_base_10_explained() {
    let educator = FunctionEducator::new();
    let args = vec![Expression::integer(10)];
    let explanation = educator.explain_function_operation("log", &args, "evaluation");

    assert!(
        has_step_containing(&explanation, "10") || has_step_containing(&explanation, "base 10") || has_step_containing(&explanation, "common"),
        "Must explain base 10"
    );

    assert!(
        has_step_containing(&explanation, "positive") || has_step_containing(&explanation, "(0, ∞)"),
        "Must specify domain restriction"
    );
}

#[test]
fn test_exp_always_positive() {
    let educator = FunctionEducator::new();
    let args = vec![Expression::integer(0)];
    let explanation = educator.explain_function_operation("exp", &args, "evaluation");

    assert!(
        explanation.steps.len() >= 5,
        "Exp explanation should have at least 5 steps"
    );

    assert!(
        has_step_containing(&explanation, "(0, ∞)") || has_step_containing(&explanation, "positive") || has_step_containing(&explanation, "always positive"),
        "Must explain that exp is always positive"
    );

    assert!(
        has_step_containing(&explanation, "e") || has_step_containing(&explanation, "2.718"),
        "Must mention base e"
    );
}

#[test]
fn test_sqrt_non_negative_domain() {
    let educator = FunctionEducator::new();
    let args = vec![Expression::integer(4)];
    let explanation = educator.explain_function_operation("sqrt", &args, "evaluation");

    assert!(
        has_step_containing(&explanation, "[0, ∞)") || has_step_containing(&explanation, "non-negative") || has_step_containing(&explanation, "≥ 0"),
        "Must specify non-negative domain"
    );

    assert!(
        has_step_containing(&explanation, "principal") || has_step_containing(&explanation, "positive"),
        "Must explain principal (positive) root"
    );
}

#[test]
fn test_cbrt_all_real_numbers() {
    let educator = FunctionEducator::new();
    let args = vec![Expression::integer(-8)];
    let explanation = educator.explain_function_operation("cbrt", &args, "evaluation");

    assert!(
        has_step_containing(&explanation, "all real") || has_step_containing(&explanation, "any number"),
        "Must specify all real numbers domain"
    );

    assert!(
        has_step_containing(&explanation, "cube"),
        "Must mention cube root"
    );
}

#[test]
fn test_factorial_non_negative_integers() {
    let educator = FunctionEducator::new();
    let args = vec![Expression::integer(5)];
    let explanation = educator.explain_function_operation("factorial", &args, "evaluation");

    assert!(
        explanation.steps.len() >= 5,
        "Factorial explanation should have at least 5 steps"
    );

    assert!(
        has_step_containing(&explanation, "non-negative") || has_step_containing(&explanation, "integer"),
        "Must specify non-negative integer domain"
    );

    assert!(
        has_step_containing(&explanation, "product") || has_step_containing(&explanation, "×") || has_step_containing(&explanation, "*"),
        "Must explain product concept"
    );

    assert!(
        has_step_containing(&explanation, "0! = 1") || (has_step_containing(&explanation, "0!") && has_step_containing(&explanation, "1")),
        "Must mention 0! = 1 special case"
    );
}

#[test]
fn test_gcd_euclidean_algorithm() {
    let educator = FunctionEducator::new();
    let args = vec![Expression::integer(12), Expression::integer(18)];
    let explanation = educator.explain_function_operation("gcd", &args, "evaluation");

    assert!(
        has_step_containing(&explanation, "euclidean") || has_step_containing(&explanation, "algorithm") || has_step_containing(&explanation, "mod"),
        "Must mention Euclidean algorithm"
    );

    assert!(
        has_step_containing(&explanation, "divisor") || has_step_containing(&explanation, "dividing"),
        "Must explain greatest common divisor concept"
    );
}

#[test]
fn test_lcm_formula_explained() {
    let educator = FunctionEducator::new();
    let args = vec![Expression::integer(4), Expression::integer(6)];
    let explanation = educator.explain_function_operation("lcm", &args, "evaluation");

    assert!(
        has_step_containing(&explanation, "gcd") || has_step_containing(&explanation, "×") || has_step_containing(&explanation, "/"),
        "Must mention formula involving gcd"
    );

    assert!(
        has_step_containing(&explanation, "multiple") || has_step_containing(&explanation, "divisible"),
        "Must explain least common multiple concept"
    );
}

#[test]
fn test_legendre_polynomial_recurrence() {
    let educator = FunctionEducator::new();
    let args = vec![Expression::integer(2)];
    let explanation = educator.explain_function_operation("legendre_p", &args, "evaluation");

    assert!(
        explanation.steps.len() >= 5,
        "Legendre explanation should have at least 5 steps"
    );

    assert!(
        has_step_containing(&explanation, "recurrence") || has_step_containing(&explanation, "P_n"),
        "Must mention recurrence relation"
    );

    assert!(
        has_step_containing(&explanation, "orthogonal") || has_step_containing(&explanation, "[-1,1]"),
        "Must explain orthogonality or domain"
    );
}

#[test]
fn test_chebyshev_approximation_context() {
    let educator = FunctionEducator::new();
    let args = vec![Expression::integer(1)];
    let explanation = educator.explain_function_operation("chebyshev_t", &args, "evaluation");

    assert!(
        has_step_containing(&explanation, "chebyshev") || has_step_containing(&explanation, "T_n"),
        "Must identify Chebyshev polynomial"
    );

    assert!(
        has_step_containing(&explanation, "approximation") || has_step_containing(&explanation, "error") || has_step_containing(&explanation, "minimize"),
        "Must mention approximation theory context"
    );
}

#[test]
fn test_hermite_quantum_mechanics() {
    let educator = FunctionEducator::new();
    let args = vec![Expression::integer(0)];
    let explanation = educator.explain_function_operation("hermite_h", &args, "evaluation");

    assert!(
        has_step_containing(&explanation, "hermite") || has_step_containing(&explanation, "H_n"),
        "Must identify Hermite polynomial"
    );

    assert!(
        has_step_containing(&explanation, "gaussian") || has_step_containing(&explanation, "quantum"),
        "Must mention application context"
    );
}

#[test]
fn test_laguerre_radial_wavefunctions() {
    let educator = FunctionEducator::new();
    let args = vec![Expression::integer(1)];
    let explanation = educator.explain_function_operation("laguerre_l", &args, "evaluation");

    assert!(
        has_step_containing(&explanation, "laguerre") || has_step_containing(&explanation, "L_n"),
        "Must identify Laguerre polynomial"
    );

    assert!(
        has_step_containing(&explanation, "exponential") || has_step_containing(&explanation, "quantum") || has_step_containing(&explanation, "radial"),
        "Must mention application or weight function"
    );
}

#[test]
fn test_step_completeness_all_functions() {
    let educator = FunctionEducator::new();
    let functions = vec![
        "sin", "cos", "tan", "arcsin", "arccos", "arctan",
        "exp", "ln", "log", "sqrt", "cbrt",
        "factorial", "gcd", "lcm",
        "legendre_p", "chebyshev_t", "hermite_h", "laguerre_l"
    ];

    for func in functions {
        let args = vec![Expression::integer(1)];
        let explanation = educator.explain_function_operation(func, &args, "evaluation");

        assert!(
            explanation.steps.len() >= 5,
            "Function {} should have at least 5 steps, got {}",
            func,
            explanation.steps.len()
        );

        let has_domain_info = has_step_containing(&explanation, "domain")
            || has_step_containing(&explanation, "valid")
            || has_step_containing(&explanation, "input")
            || has_step_containing(&explanation, "polynomial");
        assert!(
            has_domain_info,
            "Function {} must mention domain or provide input context",
            func
        );
    }
}

#[test]
fn test_function_count_minimum_20() {
    let educator = FunctionEducator::new();

    let test_functions = vec![
        "sin", "cos", "tan", "csc", "sec", "cot", "arcsin", "arccos", "arctan",
        "exp", "ln", "log", "log10", "sqrt", "cbrt",
        "factorial", "gcd", "lcm",
        "legendre_p", "chebyshev_t", "hermite_h", "laguerre_l"
    ];

    let mut working_count = 0;
    for func in &test_functions {
        let args = vec![Expression::integer(1)];
        let explanation = educator.explain_function_operation(func, &args, "evaluation");
        if explanation.steps.len() >= 5 {
            working_count += 1;
        }
    }

    assert!(
        working_count >= 20,
        "Should have at least 20 working functions, got {}",
        working_count
    );
}
