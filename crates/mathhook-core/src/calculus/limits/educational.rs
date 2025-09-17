use super::methods::LimitMethods;
use super::Limits;
use crate::calculus::derivatives::Derivative;
use crate::educational::enhanced_steps::{EnhancedStepBuilder, EnhancedStepExplanation};
use crate::formatter::latex::LaTeXFormatter;
use crate::{Expression, Symbol};

/// Educational explanations for limit operations
pub struct LimitEducation;

impl LimitEducation {
    /// Helper to safely format expression to LaTeX
    fn format_latex(expr: &Expression) -> String {
        expr.to_latex(None)
            .unwrap_or_else(|_| format!("{:?}", expr))
    }

    /// Find numerator and denominator in a two-factor Mul expression.
    /// Returns (numerator, denominator_base) where denominator is Pow(base, -1).
    /// Handles both Integer(-1) and Rational(-1/1) exponents, and checks both factor positions.
    fn find_num_denom_in_mul(factors: &[Expression]) -> (Option<&Expression>, Option<&Expression>) {
        use crate::core::Number;
        use num_bigint::BigInt;

        fn is_minus_one(exp: &Expression) -> bool {
            match exp {
                Expression::Number(Number::Integer(-1)) => true,
                Expression::Number(Number::Rational(r)) => {
                    r.denom() == &BigInt::from(1) && r.numer() == &BigInt::from(-1)
                }
                _ => false,
            }
        }

        if factors.len() != 2 {
            return (None, None);
        }

        // Check if factors[1] is Pow(_, -1)
        if let Expression::Pow(denom_base, exp) = &factors[1] {
            if is_minus_one(exp) {
                return (Some(&factors[0]), Some(denom_base.as_ref()));
            }
        }

        // Check if factors[0] is Pow(_, -1) (due to canonical ordering)
        if let Expression::Pow(denom_base, exp) = &factors[0] {
            if is_minus_one(exp) {
                return (Some(&factors[1]), Some(denom_base.as_ref()));
            }
        }

        (None, None)
    }

    /// Generate educational explanation for direct substitution limit
    ///
    /// Explains the process of evaluating a limit by directly substituting
    /// the limit point into the expression.
    ///
    /// # Arguments
    ///
    /// * `expr` - The expression to take the limit of
    /// * `variable` - The variable approaching the limit point
    /// * `point` - The value the variable approaches
    /// * `result` - The computed limit result
    ///
    /// # Returns
    ///
    /// Complete step-by-step explanation with at least 3 steps
    pub fn direct_substitution_explanation(
        expr: &Expression,
        variable: &Symbol,
        point: &Expression,
        result: &Expression,
    ) -> EnhancedStepExplanation {
        let mut steps = Vec::new();

        let expr_latex = Self::format_latex(expr);
        let var_name = variable.name();
        let point_latex = Self::format_latex(point);
        let result_latex = Self::format_latex(result);

        steps.push(
            EnhancedStepBuilder::new("limit_direct_1")
                .with_human_message(
                    "Attempt Direct Substitution",
                    &format!(
                        "To find lim({} -> {}) {}, first try direct substitution.\nSubstitute {} = {} into the expression.",
                        var_name, point_latex, expr_latex, var_name, point_latex
                    )
                )
                .with_api_data("limit", "direct_substitution", "substitute")
                .with_input("expression", &expr_latex)
                .with_input("variable", var_name)
                .with_input("point", &point_latex)
                .with_message_key("calculus", "limit_direct", 0)
                .build()
        );

        let substituted_expr = LimitMethods::substitute_and_evaluate(expr, variable, point);
        let substituted_latex = Self::format_latex(&substituted_expr);

        steps.push(
            EnhancedStepBuilder::new("limit_direct_2")
                .with_human_message(
                    "Evaluate Expression",
                    &format!(
                        "After substituting {} = {} into {}:\nResult: {}",
                        var_name, point_latex, expr_latex, substituted_latex
                    ),
                )
                .with_api_data("limit", "evaluation", "compute")
                .with_input("substituted_expression", &substituted_latex)
                .with_output("intermediate_result", &result_latex)
                .with_message_key("calculus", "limit_direct", 1)
                .build(),
        );

        steps.push(
            EnhancedStepBuilder::new("limit_direct_3")
                .with_human_message(
                    "Verify No Indeterminate Form",
                    &format!(
                        "The result {} is well-defined (not 0/0 or infinity/infinity).\nTherefore, the limit exists and equals {}.",
                        result_latex, result_latex
                    )
                )
                .with_api_data("limit", "verification", "check_form")
                .with_output("limit_result", &result_latex)
                .with_output("form_type", "determinate")
                .with_message_key("calculus", "limit_direct", 1)
                .build()
        );

        EnhancedStepExplanation::new(steps)
    }

    /// Generate educational explanation for indeterminate form detection
    ///
    /// Explains how to identify indeterminate forms (0/0, infinity/infinity, etc.)
    /// and the strategies needed to resolve them.
    ///
    /// # Arguments
    ///
    /// * `expr` - The expression producing an indeterminate form
    /// * `variable` - The variable in the limit
    /// * `point` - The limit point
    /// * `form_type` - The type of indeterminate form (e.g., "0/0")
    ///
    /// # Returns
    ///
    /// Complete step-by-step explanation with at least 4 steps
    pub fn indeterminate_form_explanation(
        expr: &Expression,
        variable: &Symbol,
        point: &Expression,
        form_type: &str,
    ) -> EnhancedStepExplanation {
        let mut steps = Vec::new();

        let expr_latex = Self::format_latex(expr);
        let var_name = variable.name();
        let point_latex = Self::format_latex(point);

        steps.push(
            EnhancedStepBuilder::new("limit_indet_1")
                .with_human_message(
                    "Attempt Direct Substitution",
                    &format!(
                        "To find lim({} -> {}) {}, substitute {} = {}:",
                        var_name, point_latex, expr_latex, var_name, point_latex
                    ),
                )
                .with_api_data("limit", "substitution", "direct_attempt")
                .with_input("expression", &expr_latex)
                .with_input("variable", var_name)
                .with_input("point", &point_latex)
                .with_message_key("calculus", "limit_indeterminate", 0)
                .build(),
        );

        if let Expression::Mul(factors) = expr {
            if factors.len() == 2 {
                // Find numerator and denominator - Pow(_, -1) could be at either index
                // due to canonical ordering. Also handle both Integer(-1) and Rational(-1/1).
                let (numerator, denominator) = Self::find_num_denom_in_mul(factors);

                if let Some((num, denom)) = numerator.zip(denominator) {
                    let num_latex = Self::format_latex(num);
                    let den_latex = Self::format_latex(denom);

                    let num_at_point = LimitMethods::substitute_and_evaluate(num, variable, point);
                    let den_at_point =
                        LimitMethods::substitute_and_evaluate(denom, variable, point);

                    let num_result_latex = Self::format_latex(&num_at_point);
                    let den_result_latex = Self::format_latex(&den_at_point);

                    steps.push(
                        EnhancedStepBuilder::new("limit_indet_2")
                            .with_human_message(
                                "Evaluate Components",
                                &format!(
                                    "Numerator at {} = {}: {} = {}\nDenominator at {} = {}: {} = {}",
                                    var_name, point_latex, num_latex, num_result_latex,
                                    var_name, point_latex, den_latex, den_result_latex
                                )
                            )
                            .with_api_data("limit", "component_evaluation", "rational")
                            .with_input("numerator", &num_latex)
                            .with_input("denominator", &den_latex)
                            .with_output("numerator_value", &num_result_latex)
                            .with_output("denominator_value", &den_result_latex)
                            .with_message_key("calculus", "limit_indeterminate", 1)
                            .build()
                    );
                }
            }
        }

        steps.push(
            EnhancedStepBuilder::new("limit_indet_3")
                .with_human_message(
                    "Identify Indeterminate Form",
                    &format!(
                        "The result is {} (indeterminate form).\nThis form is undefined and requires further analysis.",
                        form_type
                    )
                )
                .with_api_data("limit", "form_detection", "indeterminate")
                .with_output("form_type", form_type)
                .with_output("requires_resolution", "true")
                .with_message_key("calculus", "limit_indeterminate", 0)
                .build()
        );

        let resolution_strategy = match form_type {
            "0/0" => "Factorization: Factor numerator and denominator to cancel common terms, or apply L'Hopital's rule",
            "infinity/infinity" => "Divide by highest power: Divide numerator and denominator by highest power term",
            "0*infinity" => "Rewrite as fraction: Convert to 0/0 or infinity/infinity form",
            _ => "Apply algebraic manipulation or L'Hopital's rule"
        };

        steps.push(
            EnhancedStepBuilder::new("limit_indet_4")
                .with_human_message(
                    "Resolution Strategy",
                    &format!("To resolve {} form:\n{}", form_type, resolution_strategy),
                )
                .with_api_data("limit", "strategy", "resolution")
                .with_input("indeterminate_form", form_type)
                .with_output("strategy", resolution_strategy)
                .with_message_key("calculus", "limit_indeterminate", 1)
                .build(),
        );

        EnhancedStepExplanation::new(steps)
    }

    /// Generate educational explanation for L'Hopital's rule application
    ///
    /// Explains the complete process of applying L'Hopital's rule to resolve
    /// indeterminate forms by differentiating numerator and denominator.
    ///
    /// # Arguments
    ///
    /// * `numerator` - The numerator expression
    /// * `denominator` - The denominator expression
    /// * `variable` - The variable in the limit
    /// * `point` - The limit point
    ///
    /// # Returns
    ///
    /// Complete step-by-step explanation with at least 6 steps
    pub fn lhopital_rule_explanation(
        numerator: &Expression,
        denominator: &Expression,
        variable: &Symbol,
        point: &Expression,
    ) -> EnhancedStepExplanation {
        let mut steps = Vec::new();

        let num_latex = Self::format_latex(numerator);
        let den_latex = Self::format_latex(denominator);
        let var_name = variable.name();
        let point_latex = Self::format_latex(point);

        let num_at_point = LimitMethods::substitute_and_evaluate(numerator, variable, point);
        let den_at_point = LimitMethods::substitute_and_evaluate(denominator, variable, point);

        let form_type = if num_at_point.is_zero() && den_at_point.is_zero() {
            "0/0"
        } else {
            "infinity/infinity"
        };

        steps.push(
            EnhancedStepBuilder::new("limit_lhopital_1")
                .with_human_message(
                    "Check Conditions for L'Hopital's Rule",
                    &format!(
                        "For lim({} -> {}) {}/{}, direct substitution gives {}.\nL'Hopital's rule applies to indeterminate forms 0/0 and infinity/infinity.",
                        var_name, point_latex, num_latex, den_latex, form_type
                    )
                )
                .with_api_data("limit", "lhopital_check", "conditions")
                .with_input("numerator", &num_latex)
                .with_input("denominator", &den_latex)
                .with_output("form_type", form_type)
                .with_output("rule_applies", "true")
                .with_message_key("calculus", "limit_lhopital", 0)
                .build()
        );

        steps.push(
            EnhancedStepBuilder::new("limit_lhopital_2")
                .with_human_message(
                    "State L'Hopital's Rule",
                    "If lim(f(x)/g(x)) gives 0/0 or infinity/infinity, then:\nlim(f(x)/g(x)) = lim(f'(x)/g'(x))\nprovided the limit on the right exists."
                )
                .with_api_data("limit", "rule_statement", "lhopital_theorem")
                .with_output("theorem", "lhopital")
                .with_output("applicable_forms", "0/0, infinity/infinity")
                .with_message_key("calculus", "limit_lhopital", 0)
                .build()
        );

        let num_derivative = numerator.derivative(variable.clone());
        let num_deriv_latex = Self::format_latex(&num_derivative);

        steps.push(
            EnhancedStepBuilder::new("limit_lhopital_3")
                .with_human_message(
                    "Differentiate Numerator",
                    &format!(
                        "Find derivative of numerator with respect to {}:\nd/d{}[{}] = {}",
                        var_name, var_name, num_latex, num_deriv_latex
                    ),
                )
                .with_api_data("limit", "differentiation", "numerator")
                .with_input("original", &num_latex)
                .with_output("derivative", &num_deriv_latex)
                .with_message_key("calculus", "limit_lhopital", 1)
                .build(),
        );

        let den_derivative = denominator.derivative(variable.clone());
        let den_deriv_latex = Self::format_latex(&den_derivative);

        steps.push(
            EnhancedStepBuilder::new("limit_lhopital_4")
                .with_human_message(
                    "Differentiate Denominator",
                    &format!(
                        "Find derivative of denominator with respect to {}:\nd/d{}[{}] = {}",
                        var_name, var_name, den_latex, den_deriv_latex
                    ),
                )
                .with_api_data("limit", "differentiation", "denominator")
                .with_input("original", &den_latex)
                .with_output("derivative", &den_deriv_latex)
                .with_message_key("calculus", "limit_lhopital", 1)
                .build(),
        );

        steps.push(
            EnhancedStepBuilder::new("limit_lhopital_5")
                .with_human_message(
                    "Apply L'Hopital's Rule",
                    &format!(
                        "By L'Hopital's rule:\nlim({} -> {}) {}/{} = lim({} -> {}) {}/{}",
                        var_name,
                        point_latex,
                        num_latex,
                        den_latex,
                        var_name,
                        point_latex,
                        num_deriv_latex,
                        den_deriv_latex
                    ),
                )
                .with_api_data("limit", "rule_application", "substitute_derivatives")
                .with_input("original_limit", &format!("{}/{}", num_latex, den_latex))
                .with_output(
                    "new_limit",
                    &format!("{}/{}", num_deriv_latex, den_deriv_latex),
                )
                .with_message_key("calculus", "limit_lhopital", 1)
                .build(),
        );

        let new_limit_result =
            LimitMethods::rational_limit(&num_derivative, &den_derivative, variable, point);
        let result_latex = Self::format_latex(&new_limit_result);

        steps.push(
            EnhancedStepBuilder::new("limit_lhopital_6")
                .with_human_message(
                    "Evaluate New Limit",
                    &format!(
                        "Now evaluate lim({} -> {}) {}/{}:\nResult: {}",
                        var_name, point_latex, num_deriv_latex, den_deriv_latex, result_latex
                    ),
                )
                .with_api_data("limit", "final_evaluation", "result")
                .with_output("final_result", &result_latex)
                .with_message_key("calculus", "limit_lhopital", 1)
                .build(),
        );

        EnhancedStepExplanation::new(steps)
    }

    /// Generate educational explanation for limit laws application
    ///
    /// Explains how limit laws (sum, product, quotient, constant multiple)
    /// can be used to break down complex limits into simpler parts.
    ///
    /// # Arguments
    ///
    /// * `expr` - The expression whose limit to find
    /// * `variable` - The variable in the limit
    /// * `point` - The limit point
    ///
    /// # Returns
    ///
    /// Complete step-by-step explanation with at least 4 steps
    pub fn limit_laws_explanation(
        expr: &Expression,
        variable: &Symbol,
        point: &Expression,
    ) -> EnhancedStepExplanation {
        let mut steps = Vec::new();

        let expr_latex = Self::format_latex(expr);
        let var_name = variable.name();
        let point_latex = Self::format_latex(point);

        steps.push(
            EnhancedStepBuilder::new("limit_laws_1")
                .with_human_message(
                    "Identify Applicable Limit Laws",
                    &format!(
                        "To find lim({} -> {}) {}, we can apply limit laws:\n• Sum Law: lim[f(x) + g(x)] = lim f(x) + lim g(x)\n• Product Law: lim[f(x) * g(x)] = lim f(x) * lim g(x)\n• Constant Multiple Law: lim[c * f(x)] = c * lim f(x)",
                        var_name, point_latex, expr_latex
                    )
                )
                .with_api_data("limit", "laws", "introduction")
                .with_input("expression", &expr_latex)
                .with_message_key("calculus", "limit_laws", 0)
                .build()
        );

        match expr {
            Expression::Add(terms) => {
                steps.push(
                    EnhancedStepBuilder::new("limit_laws_2")
                        .with_human_message(
                            "Apply Sum Law",
                            &format!(
                                "The expression is a sum of {} terms.\nBy Sum Law: lim[f + g] = lim f + lim g\nEvaluate limit of each term separately.",
                                terms.len()
                            )
                        )
                        .with_api_data("limit", "law_application", "sum")
                        .with_input("term_count", &terms.len().to_string())
                        .with_output("law_used", "sum_law")
                        .with_message_key("calculus", "limit_laws", 0)
                        .build()
                );

                let term_limits: Vec<String> = terms
                    .iter()
                    .map(|term| {
                        let limit_result = term.limit(variable, point);
                        format!(
                            "lim {} = {}",
                            Self::format_latex(term),
                            Self::format_latex(&limit_result)
                        )
                    })
                    .collect();

                steps.push(
                    EnhancedStepBuilder::new("limit_laws_3")
                        .with_human_message(
                            "Evaluate Individual Terms",
                            &format!("Evaluate limit of each term:\n{}", term_limits.join("\n")),
                        )
                        .with_api_data("limit", "term_evaluation", "individual")
                        .with_message_key("calculus", "limit_laws", 1)
                        .build(),
                );
            }
            Expression::Mul(factors) => {
                let has_constant = factors.iter().any(|f| matches!(f, Expression::Number(_)));

                if has_constant {
                    steps.push(
                        EnhancedStepBuilder::new("limit_laws_2")
                            .with_human_message(
                                "Apply Constant Multiple Law",
                                "By Constant Multiple Law: lim[c * f(x)] = c * lim f(x)\nFactor out constants and evaluate limit of remaining expression."
                            )
                            .with_api_data("limit", "law_application", "constant_multiple")
                            .with_output("law_used", "constant_multiple_law")
                            .with_message_key("calculus", "limit_laws", 0)
                            .build()
                    );
                } else {
                    steps.push(
                        EnhancedStepBuilder::new("limit_laws_2")
                            .with_human_message(
                                "Apply Product Law",
                                &format!(
                                    "The expression is a product of {} factors.\nBy Product Law: lim[f * g] = lim f * lim g\nEvaluate limit of each factor separately.",
                                    factors.len()
                                )
                            )
                            .with_api_data("limit", "law_application", "product")
                            .with_input("factor_count", &factors.len().to_string())
                            .with_output("law_used", "product_law")
                            .with_message_key("calculus", "limit_laws", 1)
                            .build()
                    );
                }

                steps.push(
                    EnhancedStepBuilder::new("limit_laws_3")
                        .with_human_message(
                            "Evaluate Components",
                            "Evaluate limit of each component and combine results using the limit law."
                        )
                        .with_api_data("limit", "component_evaluation", "combine")
                        .with_message_key("calculus", "limit_laws", 1)
                        .build()
                );
            }
            _ => {
                steps.push(
                    EnhancedStepBuilder::new("limit_laws_2")
                        .with_human_message(
                            "Direct Evaluation",
                            "For simple expressions, directly substitute the limit point.",
                        )
                        .with_api_data("limit", "direct", "simple")
                        .with_message_key("calculus", "limit_laws", 0)
                        .build(),
                );
            }
        }

        let result = expr.limit(variable, point);
        let result_latex = Self::format_latex(&result);

        steps.push(
            EnhancedStepBuilder::new("limit_laws_4")
                .with_human_message(
                    "Combine Results",
                    &format!(
                        "Combining the individual limits:\nlim({} -> {}) {} = {}",
                        var_name, point_latex, expr_latex, result_latex
                    ),
                )
                .with_api_data("limit", "combination", "final")
                .with_output("final_result", &result_latex)
                .with_message_key("calculus", "limit_laws", 1)
                .build(),
        );

        EnhancedStepExplanation::new(steps)
    }

    /// Generate educational explanation for limits at infinity
    ///
    /// Explains the technique of dividing by the highest power to evaluate
    /// limits as the variable approaches infinity.
    ///
    /// # Arguments
    ///
    /// * `expr` - The expression whose limit to find
    /// * `variable` - The variable approaching infinity
    ///
    /// # Returns
    ///
    /// Complete step-by-step explanation with at least 4 steps
    pub fn limit_at_infinity_explanation(
        expr: &Expression,
        variable: &Symbol,
    ) -> EnhancedStepExplanation {
        let mut steps = Vec::new();

        let expr_latex = Self::format_latex(expr);
        let var_name = variable.name();

        steps.push(
            EnhancedStepBuilder::new("limit_infinity_1")
                .with_human_message(
                    "Identify Form",
                    &format!(
                        "To find lim({} -> infinity) {}, first identify the form.\nFor rational functions, both numerator and denominator may approach infinity (infinity/infinity form).",
                        var_name, expr_latex
                    )
                )
                .with_api_data("limit", "infinity", "identify_form")
                .with_input("expression", &expr_latex)
                .with_input("limit_point", "infinity")
                .with_message_key("calculus", "limit_infinity", 0)
                .build()
        );

        if let Expression::Mul(factors) = expr {
            if factors.len() == 2 {
                if let Expression::Pow(denom, exp) = &factors[1] {
                    if **exp == Expression::integer(-1) {
                        let numerator = &factors[0];
                        let num_latex = Self::format_latex(numerator);
                        let den_latex = Self::format_latex(denom);

                        steps.push(
                            EnhancedStepBuilder::new("limit_infinity_2")
                                .with_human_message(
                                    "Divide by Highest Power",
                                    &format!(
                                        "For rational function {}/{}:\nDivide both numerator and denominator by the highest power of {} in the denominator.\nThis transforms terms: {}/infinity -> 0",
                                        num_latex, den_latex, var_name, var_name
                                    )
                                )
                                .with_api_data("limit", "technique", "divide_highest_power")
                                .with_input("numerator", &num_latex)
                                .with_input("denominator", &den_latex)
                                .with_output("technique", "divide_by_highest_power")
                                .with_message_key("calculus", "limit_infinity", 1)
                                .build()
                        );

                        steps.push(
                            EnhancedStepBuilder::new("limit_infinity_3")
                                .with_human_message(
                                    "Evaluate as x Approaches Infinity",
                                    &format!(
                                        "As {} -> infinity:\n• Terms like 1/{}, 1/{}^2, etc. approach 0\n• Leading coefficients remain\n• Evaluate the simplified expression",
                                        var_name, var_name, var_name
                                    )
                                )
                                .with_api_data("limit", "evaluation", "infinity_behavior")
                                .with_message_key("calculus", "limit_infinity", 1)
                                .build()
                        );
                    }
                }
            }
        } else {
            steps.push(
                EnhancedStepBuilder::new("limit_infinity_2")
                    .with_human_message(
                        "Analyze Dominant Term",
                        &format!(
                            "For polynomial or simple expressions, identify the dominant term (highest power of {}).\nThe limit behavior is determined by this dominant term.",
                            var_name
                        )
                    )
                    .with_api_data("limit", "analysis", "dominant_term")
                    .with_message_key("calculus", "limit_infinity", 0)
                    .build()
            );

            steps.push(
                EnhancedStepBuilder::new("limit_infinity_3")
                    .with_human_message(
                        "Evaluate Limit",
                        &format!(
                            "As {} -> infinity, evaluate the behavior of the expression.",
                            var_name
                        ),
                    )
                    .with_api_data("limit", "evaluation", "determine")
                    .with_message_key("calculus", "limit_infinity", 1)
                    .build(),
            );
        }

        let result = expr.limit_at_infinity(variable);
        let result_latex = Self::format_latex(&result);

        steps.push(
            EnhancedStepBuilder::new("limit_infinity_4")
                .with_human_message(
                    "Final Result",
                    &format!(
                        "Therefore:\nlim({} -> infinity) {} = {}",
                        var_name, expr_latex, result_latex
                    ),
                )
                .with_api_data("limit", "result", "infinity")
                .with_output("final_result", &result_latex)
                .with_message_key("calculus", "limit_infinity", 1)
                .build(),
        );

        EnhancedStepExplanation::new(steps)
    }
}
