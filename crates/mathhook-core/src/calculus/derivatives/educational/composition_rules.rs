//! Composition-based differentiation rules: chain rule, product rule, and quotient rule

use crate::calculus::derivatives::{Derivative, FunctionDerivatives};
use crate::core::{Expression, Symbol};
use crate::educational::message_registry::{MessageBuilder, MessageCategory, MessageType};
use crate::educational::step_by_step::{Step, StepByStepExplanation};
use crate::formatter::latex::LaTeXFormatter;
use crate::simplify::Simplify;

use super::format_expr;

/// Explain chain rule (5+ steps)
pub fn explain_chain_rule(
    func_name: &str,
    arg: &Expression,
    variable: &Symbol,
) -> StepByStepExplanation {
    let mut steps = Vec::new();
    let expr = Expression::function(func_name, vec![arg.clone()]);

    steps.push(Step {
        title: "Identify Composite Function".to_string(),
        description: format!(
            "Outer function: {}(u)\nInner function: u = {}",
            func_name,
            format_expr(arg)
        ),
        expression: expr.clone(),
        rule_applied: "Identification".to_string(),
        latex: Some(
            expr.to_latex(None)
                .unwrap_or_else(|_| "f(g(x))".to_string()),
        ),
    });

    if let Some(step) = MessageBuilder::new(
        MessageCategory::Calculus,
        MessageType::DerivativeChainRule,
        0,
    )
    .with_substitution("outer_function", func_name)
    .with_substitution("inner_function", &format_expr(arg))
    .build()
    {
        steps.push(step);
    }

    steps.push(Step {
        title: "State Chain Rule".to_string(),
        description: "d/dx[f(g(x))] = f'(g(x)) * g'(x)".to_string(),
        expression: expr.clone(),
        rule_applied: "Chain Rule".to_string(),
        latex: Some("f'(g(x)) \\cdot g'(x)".to_string()),
    });

    let outer_derivative = FunctionDerivatives::get(func_name, arg, variable.clone());

    steps.push(Step {
        title: "Differentiate Outer Function".to_string(),
        description: format!(
            "d/du[{}(u)] = {}\nEvaluated at u = {}: {}",
            func_name,
            func_name,
            format_expr(arg),
            format_expr(&outer_derivative)
        ),
        expression: outer_derivative.clone(),
        rule_applied: "Outer Derivative".to_string(),
        latex: Some(
            outer_derivative
                .to_latex(None)
                .unwrap_or_else(|_| "f'(u)".to_string()),
        ),
    });

    let inner_derivative = arg.derivative(variable.clone());
    steps.push(Step {
        title: "Differentiate Inner Function".to_string(),
        description: format!(
            "d/d{}[{}] = {}",
            variable.name(),
            format_expr(arg),
            format_expr(&inner_derivative)
        ),
        expression: inner_derivative.clone(),
        rule_applied: "Inner Derivative".to_string(),
        latex: Some(
            inner_derivative
                .to_latex(None)
                .unwrap_or_else(|_| "g'(x)".to_string()),
        ),
    });

    let result = Expression::mul(vec![outer_derivative, inner_derivative]).simplify();
    steps.push(Step {
        title: "Multiply Results".to_string(),
        description: format!("Result: {}", format_expr(&result)),
        expression: result.clone(),
        rule_applied: "Chain Rule Application".to_string(),
        latex: Some(
            result
                .to_latex(None)
                .unwrap_or_else(|_| "result".to_string()),
        ),
    });

    let step_count = steps.len();
    StepByStepExplanation {
        initial_expression: expr,
        final_expression: result,
        steps,
        total_steps: step_count,
        rules_used: vec!["Chain Rule".to_string()],
    }
}

/// Explain product rule (5+ steps)
pub fn explain_product_rule(
    first: &Expression,
    second: &Expression,
    variable: &Symbol,
) -> StepByStepExplanation {
    let mut steps = Vec::new();
    let expr = Expression::mul(vec![first.clone(), second.clone()]);

    steps.push(Step {
        title: "Identify Product".to_string(),
        description: format!(
            "Two functions multiplied:\nf(x) = {}\ng(x) = {}",
            format_expr(first),
            format_expr(second)
        ),
        expression: expr.clone(),
        rule_applied: "Identification".to_string(),
        latex: Some(expr.to_latex(None).unwrap_or_else(|_| "f*g".to_string())),
    });

    if let Some(step) = MessageBuilder::new(
        MessageCategory::Calculus,
        MessageType::DerivativeProductRule,
        0,
    )
    .with_substitution("first_function", &format_expr(first))
    .with_substitution("second_function", &format_expr(second))
    .build()
    {
        steps.push(step);
    }

    steps.push(Step {
        title: "State Product Rule".to_string(),
        description: "d/dx[f*g] = f'*g + f*g'".to_string(),
        expression: expr.clone(),
        rule_applied: "Product Rule".to_string(),
        latex: Some("f' \\cdot g + f \\cdot g'".to_string()),
    });

    let first_derivative = first.derivative(variable.clone());
    steps.push(Step {
        title: "Differentiate First Function".to_string(),
        description: format!("f'(x) = {}", format_expr(&first_derivative)),
        expression: first_derivative.clone(),
        rule_applied: "First Derivative".to_string(),
        latex: Some(
            first_derivative
                .to_latex(None)
                .unwrap_or_else(|_| "f'".to_string()),
        ),
    });

    let second_derivative = second.derivative(variable.clone());
    steps.push(Step {
        title: "Differentiate Second Function".to_string(),
        description: format!("g'(x) = {}", format_expr(&second_derivative)),
        expression: second_derivative.clone(),
        rule_applied: "Second Derivative".to_string(),
        latex: Some(
            second_derivative
                .to_latex(None)
                .unwrap_or_else(|_| "g'".to_string()),
        ),
    });

    let result = Expression::add(vec![
        Expression::mul(vec![first_derivative, second.clone()]),
        Expression::mul(vec![first.clone(), second_derivative]),
    ])
    .simplify();

    steps.push(Step {
        title: "Apply Product Rule Formula".to_string(),
        description: format!("f'*g + f*g' = {}", format_expr(&result)),
        expression: result.clone(),
        rule_applied: "Product Rule Application".to_string(),
        latex: Some(
            result
                .to_latex(None)
                .unwrap_or_else(|_| "result".to_string()),
        ),
    });

    let step_count = steps.len();
    StepByStepExplanation {
        initial_expression: expr,
        final_expression: result,
        steps,
        total_steps: step_count,
        rules_used: vec!["Product Rule".to_string()],
    }
}

/// Explain quotient rule (6+ steps)
pub fn explain_quotient_rule(
    numerator: &Expression,
    denominator: &Expression,
    variable: &Symbol,
) -> StepByStepExplanation {
    let mut steps = Vec::new();

    let expr = Expression::mul(vec![
        numerator.clone(),
        Expression::pow(denominator.clone(), Expression::integer(-1)),
    ]);

    steps.push(Step {
        title: "Identify Quotient".to_string(),
        description: format!(
            "Numerator: {}\nDenominator: {}",
            format_expr(numerator),
            format_expr(denominator)
        ),
        expression: expr.clone(),
        rule_applied: "Identification".to_string(),
        latex: Some(format!(
            "\\frac{{{}}}{{{}}}",
            numerator.to_latex(None).unwrap_or_else(|_| "f".to_string()),
            denominator
                .to_latex(None)
                .unwrap_or_else(|_| "g".to_string())
        )),
    });

    if let Some(step) = MessageBuilder::new(
        MessageCategory::Calculus,
        MessageType::DerivativeQuotientRule,
        0,
    )
    .with_substitution("numerator", &format_expr(numerator))
    .with_substitution("denominator", &format_expr(denominator))
    .build()
    {
        steps.push(step);
    }

    steps.push(Step {
        title: "State Quotient Rule".to_string(),
        description: "d/dx[f/g] = (f'*g - f*g') / g^2".to_string(),
        expression: expr.clone(),
        rule_applied: "Quotient Rule".to_string(),
        latex: Some("\\frac{f' \\cdot g - f \\cdot g'}{g^2}".to_string()),
    });

    let numerator_derivative = numerator.derivative(variable.clone());
    steps.push(Step {
        title: "Differentiate Numerator".to_string(),
        description: format!("f'(x) = {}", format_expr(&numerator_derivative)),
        expression: numerator_derivative.clone(),
        rule_applied: "Numerator Derivative".to_string(),
        latex: Some(
            numerator_derivative
                .to_latex(None)
                .unwrap_or_else(|_| "f'".to_string()),
        ),
    });

    let denominator_derivative = denominator.derivative(variable.clone());
    steps.push(Step {
        title: "Differentiate Denominator".to_string(),
        description: format!("g'(x) = {}", format_expr(&denominator_derivative)),
        expression: denominator_derivative.clone(),
        rule_applied: "Denominator Derivative".to_string(),
        latex: Some(
            denominator_derivative
                .to_latex(None)
                .unwrap_or_else(|_| "g'".to_string()),
        ),
    });

    let result_numerator = Expression::add(vec![
        Expression::mul(vec![numerator_derivative, denominator.clone()]),
        Expression::mul(vec![
            Expression::integer(-1),
            numerator.clone(),
            denominator_derivative,
        ]),
    ])
    .simplify();

    steps.push(Step {
        title: "Apply Quotient Rule Formula".to_string(),
        description: format!("(f'*g - f*g') = {}", format_expr(&result_numerator)),
        expression: result_numerator.clone(),
        rule_applied: "Numerator Calculation".to_string(),
        latex: Some(
            result_numerator
                .to_latex(None)
                .unwrap_or_else(|_| "result_num".to_string()),
        ),
    });

    let result = Expression::mul(vec![
        result_numerator,
        Expression::pow(denominator.clone(), Expression::integer(-2)),
    ])
    .simplify();

    steps.push(Step {
        title: "Simplify".to_string(),
        description: format!("Result: {}", format_expr(&result)),
        expression: result.clone(),
        rule_applied: "Quotient Rule Application".to_string(),
        latex: Some(
            result
                .to_latex(None)
                .unwrap_or_else(|_| "result".to_string()),
        ),
    });

    let step_count = steps.len();
    StepByStepExplanation {
        initial_expression: expr,
        final_expression: result,
        steps,
        total_steps: step_count,
        rules_used: vec!["Quotient Rule".to_string()],
    }
}
