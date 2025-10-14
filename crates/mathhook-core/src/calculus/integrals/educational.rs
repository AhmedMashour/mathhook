//! Educational explanations for integration operations
//!
//! Provides step-by-step explanations for various integration techniques
//! including power rule, substitution, integration by parts, and definite integrals.

use crate::core::{Expression, Symbol};
use crate::educational::message_registry::{MessageBuilder, MessageCategory, MessageType};
use crate::educational::step_by_step::{Step, StepByStepExplanation};

/// Generate educational explanation for power rule integration
///
/// # Arguments
///
/// * `base` - The base expression (typically a variable)
/// * `exponent` - The exponent expression
/// * `variable` - The variable of integration
///
/// # Examples
///
/// ```rust
/// use mathhook_core::{symbol, Expression};
/// use mathhook_core::calculus::integrals::educational::explain_power_rule;
///
/// let x = symbol!(x);
/// let base = Expression::symbol(x.clone());
/// let exponent = Expression::integer(2);
/// let explanation = explain_power_rule(&base, &exponent, &x);
/// assert!(explanation.steps.len() >= 3);
/// ```
pub fn explain_power_rule(
    base: &Expression,
    exponent: &Expression,
    variable: &Symbol,
) -> StepByStepExplanation {
    let mut steps = Vec::new();
    let original_expr = Expression::pow(base.clone(), exponent.clone());

    let step1 = MessageBuilder::new(MessageCategory::Calculus, MessageType::IntegralPowerRule, 0)
        .with_substitution("expression", format!("{}", original_expr))
        .with_substitution("exponent", format!("{}", exponent))
        .build();
    if let Some(s) = step1 {
        steps.push(s);
    }

    let exponent_plus_one = Expression::add(vec![exponent.clone(), Expression::integer(1)]);
    let step2 = MessageBuilder::new(MessageCategory::Calculus, MessageType::IntegralPowerRule, 1)
        .with_substitution("expression", format!("{}", original_expr))
        .with_substitution("base", format!("{}", base))
        .with_substitution("exponent", format!("{}", exponent))
        .with_substitution("variable", variable.name())
        .with_substitution("exponent_plus_one", format!("{}", exponent_plus_one))
        .build();
    if let Some(s) = step2 {
        steps.push(s);
    }

    let result = Expression::mul(vec![
        Expression::pow(base.clone(), exponent_plus_one.clone()),
        Expression::pow(exponent_plus_one.clone(), Expression::integer(-1)),
    ]);

    let step3 = Step::new(
        "Simplify Result",
        format!("Result: {} + C", result),
    );
    steps.push(step3);

    StepByStepExplanation {
        initial_expression: original_expr,
        final_expression: result.clone(),
        steps,
        total_steps: 3,
        rules_used: vec!["Power Rule for Integration".to_string()],
    }
}

/// Generate educational explanation for constant integration
///
/// # Arguments
///
/// * `constant` - The constant expression
/// * `variable` - The variable of integration
///
/// # Examples
///
/// ```rust
/// use mathhook_core::{symbol, Expression};
/// use mathhook_core::calculus::integrals::educational::explain_constant_rule;
///
/// let x = symbol!(x);
/// let constant = Expression::integer(5);
/// let explanation = explain_constant_rule(&constant, &x);
/// assert!(explanation.steps.len() >= 2);
/// ```
pub fn explain_constant_rule(
    constant: &Expression,
    variable: &Symbol,
) -> StepByStepExplanation {
    let mut steps = Vec::new();

    let step1 = MessageBuilder::new(MessageCategory::Calculus, MessageType::IntegralConstant, 0)
        .with_substitution("constant", format!("{}", constant))
        .build();
    if let Some(s) = step1 {
        steps.push(s);
    }

    let result = Expression::mul(vec![
        constant.clone(),
        Expression::symbol(variable.clone()),
    ]);

    let step2 = Step::new(
        "Apply Integration",
        format!("integral({} dx) = {}*{} + C", constant, constant, variable.name()),
    );
    steps.push(step2);

    StepByStepExplanation {
        initial_expression: constant.clone(),
        final_expression: result.clone(),
        steps,
        total_steps: 2,
        rules_used: vec!["Constant Rule".to_string()],
    }
}

/// Generate educational explanation for sum rule integration
///
/// # Arguments
///
/// * `terms` - The terms being integrated
/// * `variable` - The variable of integration
///
/// # Examples
///
/// ```rust
/// use mathhook_core::{symbol, Expression};
/// use mathhook_core::calculus::integrals::educational::explain_sum_rule;
///
/// let x = symbol!(x);
/// let terms = vec![
///     Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
///     Expression::mul(vec![Expression::integer(3), Expression::symbol(x.clone())]),
///     Expression::integer(5),
/// ];
/// let explanation = explain_sum_rule(&terms, &x);
/// assert!(explanation.steps.len() >= 3);
/// ```
pub fn explain_sum_rule(terms: &[Expression], _variable: &Symbol) -> StepByStepExplanation {
    let mut steps = Vec::new();
    let original_expr = Expression::add(terms.to_vec());

    let step1 = Step::new(
        "Apply Sum Rule",
        format!("integral((f + g + h) dx) = integral(f dx) + integral(g dx) + integral(h dx)\nSeparate into: {}",
            terms.iter().map(|t| format!("integral({} dx)", t)).collect::<Vec<_>>().join(" + "))
    );
    steps.push(step1);

    let mut integrated_terms = Vec::new();
    for term in terms {
        integrated_terms.push(format!("integral({} dx)", term));
    }

    let step2 = Step::new(
        "Integrate Each Term",
        format!("Evaluating each integral separately:\n{}", integrated_terms.join("\n")),
    );
    steps.push(step2);

    let step3 = Step::new(
        "Combine Results",
        "Sum all antiderivatives and add constant C",
    );
    steps.push(step3);

    StepByStepExplanation {
        initial_expression: original_expr.clone(),
        final_expression: original_expr,
        steps,
        total_steps: 3,
        rules_used: vec!["Sum Rule".to_string()],
    }
}

/// Generate educational explanation for u-substitution
///
/// # Arguments
///
/// * `integrand` - The integrand expression
/// * `substitution` - The u substitution
/// * `variable` - The variable of integration
///
/// # Examples
///
/// ```rust
/// use mathhook_core::{symbol, Expression};
/// use mathhook_core::calculus::integrals::educational::explain_u_substitution;
///
/// let x = symbol!(x);
/// let integrand = Expression::mul(vec![
///     Expression::integer(2),
///     Expression::symbol(x.clone()),
///     Expression::function("sin", vec![Expression::pow(Expression::symbol(x.clone()), Expression::integer(2))]),
/// ]);
/// let substitution = Expression::pow(Expression::symbol(x.clone()), Expression::integer(2));
/// let explanation = explain_u_substitution(&integrand, &substitution, &x);
/// assert!(explanation.steps.len() >= 6);
/// ```
pub fn explain_u_substitution(
    integrand: &Expression,
    substitution: &Expression,
    variable: &Symbol,
) -> StepByStepExplanation {
    let mut steps = Vec::new();

    let step1 = Step::new(
        "Identify Substitution Candidate",
        format!("Inner function: {} (whose derivative appears in integrand)", substitution),
    );
    steps.push(step1);

    let step2 = MessageBuilder::new(
        MessageCategory::Calculus,
        MessageType::IntegralUSubstitution,
        1,
    )
    .with_substitution("substitution", format!("{}", substitution))
    .with_substitution("expression", format!("{}", integrand))
    .build();
    if let Some(s) = step2 {
        steps.push(s);
    }

    let step3 = Step::new(
        "Find du",
        format!("du/d{} = d/d{}({})\ndu = ... d{}",
            variable.name(), variable.name(), substitution, variable.name()),
    );
    steps.push(step3);

    let step4 = Step::new(
        "Rewrite Integral",
        "Substitute u and du into the integral\nintegral(...) du",
    );
    steps.push(step4);

    let step5 = Step::new(
        "Integrate with Respect to u",
        "Perform integration in terms of u",
    );
    steps.push(step5);

    let step6 = MessageBuilder::new(
        MessageCategory::Calculus,
        MessageType::IntegralUSubstitution,
        4,
    )
    .with_substitution("result_in_u", "F(u)")
    .with_substitution("substitution", format!("{}", substitution))
    .with_substitution("final_result", format!("F({})", substitution))
    .build();
    if let Some(s) = step6 {
        steps.push(s);
    }

    StepByStepExplanation {
        initial_expression: integrand.clone(),
        final_expression: integrand.clone(),
        steps,
        total_steps: 6,
        rules_used: vec!["U-Substitution".to_string()],
    }
}

/// Generate educational explanation for integration by parts
///
/// # Arguments
///
/// * `u_choice` - The u function choice
/// * `dv_choice` - The dv function choice
/// * `variable` - The variable of integration
///
/// # Examples
///
/// ```rust
/// use mathhook_core::{symbol, Expression};
/// use mathhook_core::calculus::integrals::educational::explain_integration_by_parts;
///
/// let x = symbol!(x);
/// let u_choice = Expression::symbol(x.clone());
/// let dv_choice = Expression::function("exp", vec![Expression::symbol(x.clone())]);
/// let explanation = explain_integration_by_parts(&u_choice, &dv_choice, &x);
/// assert!(explanation.steps.len() >= 7);
/// ```
pub fn explain_integration_by_parts(
    u_choice: &Expression,
    dv_choice: &Expression,
    variable: &Symbol,
) -> StepByStepExplanation {
    let mut steps = Vec::new();
    let product = Expression::mul(vec![u_choice.clone(), dv_choice.clone()]);

    let step1 = Step::new(
        "Identify Integration by Parts",
        format!("Product of functions: {} and {}", u_choice, dv_choice),
    );
    steps.push(step1);

    let step2 = Step::new(
        "State Formula",
        "integral(u dv) = uv - integral(v du)",
    );
    steps.push(step2);

    let step3 = MessageBuilder::new(MessageCategory::Calculus, MessageType::IntegralByParts, 0)
        .with_substitution("u_choice", format!("{}", u_choice))
        .with_substitution("dv_choice", format!("{} d{}", dv_choice, variable.name()))
        .build();
    if let Some(s) = step3 {
        steps.push(s);
    }

    let step4 = Step::new(
        "Find du and v",
        format!("du = d/d{}({}) d{}\nv = integral({} d{})",
            variable.name(), u_choice, variable.name(), dv_choice, variable.name()),
    );
    steps.push(step4);

    let step5 = Step::new(
        "Apply Formula",
        format!("{}*v - integral(v du)", u_choice),
    );
    steps.push(step5);

    let step6 = Step::new(
        "Evaluate Remaining Integral",
        "Compute integral(v du)",
    );
    steps.push(step6);

    let step7 = Step::new(
        "Complete Solution",
        "Combine terms and add constant C",
    );
    steps.push(step7);

    StepByStepExplanation {
        initial_expression: product.clone(),
        final_expression: product,
        steps,
        total_steps: 7,
        rules_used: vec!["Integration by Parts".to_string()],
    }
}

/// Generate educational explanation for definite integral evaluation
///
/// # Arguments
///
/// * `integrand` - The integrand expression
/// * `variable` - The variable of integration
/// * `lower_bound` - The lower limit of integration
/// * `upper_bound` - The upper limit of integration
///
/// # Examples
///
/// ```rust
/// use mathhook_core::{symbol, Expression};
/// use mathhook_core::calculus::integrals::educational::explain_definite_integral;
///
/// let x = symbol!(x);
/// let integrand = Expression::pow(Expression::symbol(x.clone()), Expression::integer(2));
/// let lower = Expression::integer(0);
/// let upper = Expression::integer(2);
/// let explanation = explain_definite_integral(&integrand, &x, &lower, &upper);
/// assert!(explanation.steps.len() >= 5);
/// ```
pub fn explain_definite_integral(
    integrand: &Expression,
    variable: &Symbol,
    lower_bound: &Expression,
    upper_bound: &Expression,
) -> StepByStepExplanation {
    let mut steps = Vec::new();

    let step1 = Step::new(
        "Find Antiderivative",
        format!("integral({} d{}) = F({})", integrand, variable.name(), variable.name()),
    );
    steps.push(step1);

    let step2 = MessageBuilder::new(MessageCategory::Calculus, MessageType::IntegralDefinite, 1)
        .with_substitution("lower_bound", format!("{}", lower_bound))
        .with_substitution("upper_bound", format!("{}", upper_bound))
        .with_substitution("antiderivative", "F(x)")
        .build();
    if let Some(s) = step2 {
        steps.push(s);
    }

    let step3 = Step::new(
        "Evaluate at Upper Bound",
        format!("F({}) = ...", upper_bound),
    );
    steps.push(step3);

    let step4 = Step::new(
        "Evaluate at Lower Bound",
        format!("F({}) = ...", lower_bound),
    );
    steps.push(step4);

    let step5 = Step::new(
        "Calculate Difference",
        format!("F({}) - F({})", upper_bound, lower_bound),
    );
    steps.push(step5);

    let definite_expr =
        Expression::definite_integral(integrand.clone(), variable.clone(), lower_bound.clone(), upper_bound.clone());

    StepByStepExplanation {
        initial_expression: integrand.clone(),
        final_expression: definite_expr,
        steps,
        total_steps: 5,
        rules_used: vec!["Fundamental Theorem of Calculus".to_string()],
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::symbol;

    #[test]
    fn test_explain_power_rule_has_minimum_steps() {
        let x = symbol!(x);
        let base = Expression::symbol(x.clone());
        let exponent = Expression::integer(2);
        let explanation = explain_power_rule(&base, &exponent, &x);
        assert!(explanation.steps.len() >= 3);
        assert_eq!(explanation.total_steps, 3);
    }

    #[test]
    fn test_explain_constant_rule_has_minimum_steps() {
        let x = symbol!(x);
        let constant = Expression::integer(5);
        let explanation = explain_constant_rule(&constant, &x);
        assert!(explanation.steps.len() >= 2);
        assert_eq!(explanation.total_steps, 2);
    }

    #[test]
    fn test_explain_sum_rule_has_minimum_steps() {
        let x = symbol!(x);
        let terms = vec![
            Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
            Expression::mul(vec![Expression::integer(3), Expression::symbol(x.clone())]),
            Expression::integer(5),
        ];
        let explanation = explain_sum_rule(&terms, &x);
        assert!(explanation.steps.len() >= 3);
        assert_eq!(explanation.total_steps, 3);
    }

    #[test]
    fn test_explain_u_substitution_has_minimum_steps() {
        let x = symbol!(x);
        let integrand = Expression::mul(vec![
            Expression::integer(2),
            Expression::symbol(x.clone()),
        ]);
        let substitution = Expression::pow(Expression::symbol(x.clone()), Expression::integer(2));
        let explanation = explain_u_substitution(&integrand, &substitution, &x);
        assert!(explanation.steps.len() >= 6);
        assert_eq!(explanation.total_steps, 6);
    }

    #[test]
    fn test_explain_integration_by_parts_has_minimum_steps() {
        let x = symbol!(x);
        let u_choice = Expression::symbol(x.clone());
        let dv_choice = Expression::function("exp", vec![Expression::symbol(x.clone())]);
        let explanation = explain_integration_by_parts(&u_choice, &dv_choice, &x);
        assert!(explanation.steps.len() >= 7);
        assert_eq!(explanation.total_steps, 7);
    }

    #[test]
    fn test_explain_definite_integral_has_minimum_steps() {
        let x = symbol!(x);
        let integrand = Expression::pow(Expression::symbol(x.clone()), Expression::integer(2));
        let lower = Expression::integer(0);
        let upper = Expression::integer(2);
        let explanation = explain_definite_integral(&integrand, &x, &lower, &upper);
        assert!(explanation.steps.len() >= 5);
        assert_eq!(explanation.total_steps, 5);
    }
}
