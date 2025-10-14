//! Basic differentiation rules: constant, variable, sum, and power rules

use crate::calculus::derivatives::Derivative;
use crate::core::{Expression, Number, Symbol};
use crate::educational::message_registry::{MessageBuilder, MessageCategory, MessageType};
use crate::educational::step_by_step::{Step, StepByStepExplanation};
use crate::formatter::latex::LaTeXFormatter;
use crate::simplify::Simplify;

use super::format_expr;

/// Explain constant derivative (2+ steps)
pub fn explain_constant_derivative(expr: &Expression, variable: &Symbol) -> StepByStepExplanation {
    let mut steps = Vec::new();

    steps.push(Step {
        title: "Identify Constant".to_string(),
        description: format!(
            "{} is a constant (does not depend on {})",
            format_expr(expr),
            variable.name()
        ),
        expression: expr.clone(),
        rule_applied: "Identification".to_string(),
        latex: Some(expr.to_latex(None).unwrap_or_else(|_| "c".to_string())),
    });

    if let Some(step) = MessageBuilder::new(MessageCategory::Calculus, MessageType::DerivativeConstant, 0)
        .with_substitution("constant", &format_expr(expr))
        .build()
    {
        steps.push(step);
    }

    let result = Expression::integer(0);
    steps.push(Step {
        title: "Result".to_string(),
        description: "0".to_string(),
        expression: result.clone(),
        rule_applied: "Constant Rule".to_string(),
        latex: Some("0".to_string()),
    });

    let step_count = steps.len();
    StepByStepExplanation {
        initial_expression: expr.clone(),
        final_expression: result,
        steps,
        total_steps: step_count,
        rules_used: vec!["Constant Rule".to_string()],
    }
}

/// Explain variable derivative (2+ steps)
pub fn explain_variable_derivative(sym: &Symbol, variable: &Symbol) -> StepByStepExplanation {
    let mut steps = Vec::new();
    let expr = Expression::symbol(sym.clone());

    if sym == variable {
        steps.push(Step {
            title: "Differentiate Variable".to_string(),
            description: format!("Differentiating {} with respect to itself", variable.name()),
            expression: expr.clone(),
            rule_applied: "Identification".to_string(),
            latex: Some(variable.name().to_string()),
        });

        if let Some(step) = MessageBuilder::new(MessageCategory::Calculus, MessageType::DerivativeVariable, 0)
            .with_substitution("variable", variable.name())
            .build()
        {
            steps.push(step);
        }

        let result = Expression::integer(1);
        steps.push(Step {
            title: "Result".to_string(),
            description: "1".to_string(),
            expression: result.clone(),
            rule_applied: "Variable Rule".to_string(),
            latex: Some("1".to_string()),
        });

        let step_count = steps.len();
        StepByStepExplanation {
            initial_expression: expr,
            final_expression: result,
            steps,
            total_steps: step_count,
            rules_used: vec!["Variable Rule".to_string()],
        }
    } else {
        steps.push(Step {
            title: "Different Variable".to_string(),
            description: format!(
                "{} does not depend on {} (treating as constant)",
                sym.name(),
                variable.name()
            ),
            expression: expr.clone(),
            rule_applied: "Identification".to_string(),
            latex: Some(sym.name().to_string()),
        });

        let result = Expression::integer(0);
        steps.push(Step {
            title: "Result".to_string(),
            description: "0".to_string(),
            expression: result.clone(),
            rule_applied: "Constant Rule".to_string(),
            latex: Some("0".to_string()),
        });

        let step_count = steps.len();
        StepByStepExplanation {
            initial_expression: expr,
            final_expression: result,
            steps,
            total_steps: step_count,
            rules_used: vec!["Constant Rule".to_string()],
        }
    }
}

/// Explain sum/difference rule (4+ steps)
pub fn explain_sum_derivative(terms: &[Expression], variable: &Symbol) -> StepByStepExplanation {
    let mut steps = Vec::new();
    let expr = Expression::add(terms.to_vec());

    steps.push(Step {
        title: "Identify Sum".to_string(),
        description: format!("Function is a sum of {} terms", terms.len()),
        expression: expr.clone(),
        rule_applied: "Identification".to_string(),
        latex: Some(expr.to_latex(None).unwrap_or_else(|_| "sum".to_string())),
    });

    steps.push(Step {
        title: "Apply Sum Rule".to_string(),
        description: "d/dx[f + g + h + ...] = f' + g' + h' + ...\nDifferentiate each term separately".to_string(),
        expression: expr.clone(),
        rule_applied: "Sum Rule".to_string(),
        latex: Some(expr.to_latex(None).unwrap_or_else(|_| "sum".to_string())),
    });

    let mut derivative_terms = Vec::new();
    for (i, term) in terms.iter().enumerate() {
        let term_derivative = term.derivative(variable.clone());
        derivative_terms.push(term_derivative.clone());

        steps.push(Step {
            title: format!("Differentiate Term {}", i + 1),
            description: format!(
                "d/d{}({}) = {}",
                variable.name(),
                format_expr(term),
                format_expr(&term_derivative)
            ),
            expression: term_derivative.clone(),
            rule_applied: format!("Term {} Derivative", i + 1),
            latex: Some(term_derivative.to_latex(None).unwrap_or_else(|_| "term".to_string())),
        });
    }

    let result = Expression::add(derivative_terms).simplify();
    steps.push(Step {
        title: "Combine Results".to_string(),
        description: format!("Add all derivatives: {}", format_expr(&result)),
        expression: result.clone(),
        rule_applied: "Combination".to_string(),
        latex: Some(result.to_latex(None).unwrap_or_else(|_| "result".to_string())),
    });

    let step_count = steps.len();
    StepByStepExplanation {
        initial_expression: expr,
        final_expression: result,
        steps,
        total_steps: step_count,
        rules_used: vec!["Sum Rule".to_string()],
    }
}

/// Explain power rule (4+ steps)
pub fn explain_power_rule(base: &Expression, exp: &Expression, variable: &Symbol) -> StepByStepExplanation {
    let mut steps = Vec::new();
    let expr = Expression::pow(base.clone(), exp.clone());

    steps.push(Step {
        title: "Identify Power Function".to_string(),
        description: format!(
            "Function is a power: {}^{}",
            format_expr(base),
            format_expr(exp)
        ),
        expression: expr.clone(),
        rule_applied: "Identification".to_string(),
        latex: Some(expr.to_latex(None).unwrap_or_else(|_| "x^n".to_string())),
    });

    let exp_str = if let Expression::Number(Number::Integer(n)) = exp {
        format!("n = {}", n)
    } else {
        format!("n = {}", format_expr(exp))
    };

    if let Some(step) = MessageBuilder::new(MessageCategory::Calculus, MessageType::DerivativePowerRule, 0)
        .with_substitution("expression", &format_expr(&expr))
        .with_substitution("exponent", &exp_str)
        .build()
    {
        steps.push(step);
    }

    steps.push(Step {
        title: "Apply Power Rule".to_string(),
        description: "d/dx(x^n) = n*x^(n-1)".to_string(),
        expression: expr.clone(),
        rule_applied: "Power Rule".to_string(),
        latex: Some("n \\cdot x^{n-1}".to_string()),
    });

    let n_minus_1 = Expression::add(vec![exp.clone(), Expression::integer(-1)]).simplify();
    let result = Expression::mul(vec![
        exp.clone(),
        Expression::pow(base.clone(), n_minus_1.clone()),
    ]).simplify();

    steps.push(Step {
        title: "Simplify".to_string(),
        description: format!(
            "{}*{}^({})",
            format_expr(exp),
            format_expr(base),
            format_expr(&n_minus_1)
        ),
        expression: result.clone(),
        rule_applied: "Simplification".to_string(),
        latex: Some(result.to_latex(None).unwrap_or_else(|_| "result".to_string())),
    });

    let step_count = steps.len();
    StepByStepExplanation {
        initial_expression: expr,
        final_expression: result,
        steps,
        total_steps: step_count,
        rules_used: vec!["Power Rule".to_string()],
    }
}
