//! Educational step-by-step explanations for derivative operations
//!
//! Provides detailed explanations for all derivative rules including power rule,
//! chain rule, product rule, quotient rule, and basic differentiation rules.

mod basic_rules;
mod composition_rules;

use crate::calculus::derivatives::Derivative;
use crate::core::{Expression, Symbol};
use crate::educational::step_by_step::{Step, StepByStepExplanation};
use crate::formatter::latex::LaTeXFormatter;

pub use basic_rules::{
    explain_constant_derivative, explain_power_rule, explain_sum_derivative,
    explain_variable_derivative,
};
pub use composition_rules::{explain_chain_rule, explain_product_rule, explain_quotient_rule};

/// Helper to format expressions for display
pub(crate) fn format_expr(expr: &Expression) -> String {
    format!("{}", expr)
}

/// Educational derivative operations trait
pub trait DerivativeWithSteps {
    /// Compute derivative with step-by-step explanation
    ///
    /// # Arguments
    ///
    /// * `variable` - The variable to differentiate with respect to
    /// * `order` - Order of derivative (1 for first derivative, 2 for second, etc.)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{expr, symbol};
    /// use mathhook_core::calculus::derivatives::DerivativeWithSteps;
    ///
    /// let x = symbol!(x);
    /// let expr = expr!(x ^ 3);
    /// let explanation = expr.derivative_with_steps(&x, 1);
    /// assert!(explanation.steps.len() >= 4);
    /// ```
    fn derivative_with_steps(&self, variable: &Symbol, order: u32) -> StepByStepExplanation;
}

impl DerivativeWithSteps for Expression {
    fn derivative_with_steps(&self, variable: &Symbol, order: u32) -> StepByStepExplanation {
        if order == 0 {
            let steps = vec![Step {
                title: "Zero Order Derivative".to_string(),
                description: "The 0th derivative of a function is the function itself".to_string(),
                expression: self.clone(),
                rule_applied: "Identity".to_string(),
                latex: Some(self.to_latex(None).unwrap_or_else(|_| "f(x)".to_string())),
            }];
            return StepByStepExplanation {
                initial_expression: self.clone(),
                final_expression: self.clone(),
                steps,
                total_steps: 1,
                rules_used: vec!["Identity".to_string()],
            };
        }

        let mut steps = Vec::new();

        steps.push(Step {
            title: if order == 1 {
                format!("Find Derivative of {}", format_expr(self))
            } else {
                format!("Find {}-Order Derivative", order)
            },
            description: format!(
                "Differentiate {} with respect to {}",
                format_expr(self),
                variable.name()
            ),
            expression: self.clone(),
            rule_applied: "Initial".to_string(),
            latex: Some(self.to_latex(None).unwrap_or_else(|_| "f(x)".to_string())),
        });

        let mut current = self.clone();

        for n in 1..=order {
            let derivative_steps = compute_single_derivative_steps(&current, variable);
            steps.extend(derivative_steps.steps.clone());
            current = derivative_steps.final_expression.clone();

            if n < order {
                steps.push(Step {
                    title: format!("{}-Order Derivative Complete", n),
                    description: format!("Result: {}", format_expr(&current)),
                    expression: current.clone(),
                    rule_applied: "Intermediate".to_string(),
                    latex: Some(
                        current
                            .to_latex(None)
                            .unwrap_or_else(|_| "f(x)".to_string()),
                    ),
                });
            }
        }

        let step_count = steps.len();
        StepByStepExplanation {
            initial_expression: self.clone(),
            final_expression: current.clone(),
            steps,
            total_steps: step_count,
            rules_used: vec!["Differentiation".to_string()],
        }
    }
}

/// Compute step-by-step derivative for a single differentiation
fn compute_single_derivative_steps(expr: &Expression, variable: &Symbol) -> StepByStepExplanation {
    match expr {
        Expression::Number(_) | Expression::Constant(_) => {
            explain_constant_derivative(expr, variable)
        }
        Expression::Symbol(sym) => explain_variable_derivative(sym, variable),
        Expression::Add(terms) => explain_sum_derivative(terms, variable),
        Expression::Pow(base, exp) => {
            if is_power_rule_applicable(base, exp, variable) {
                explain_power_rule(base, exp, variable)
            } else {
                explain_general_power_derivative(base, exp, variable)
            }
        }
        Expression::Mul(factors) => {
            if let Some((numerator, denominator)) = detect_quotient(factors) {
                explain_quotient_rule(&numerator, &denominator, variable)
            } else if factors.len() == 2 {
                explain_product_rule(&factors[0], &factors[1], variable)
            } else {
                explain_general_product_rule(factors, variable)
            }
        }
        Expression::Function { name, args } => {
            if args.len() == 1 {
                explain_chain_rule(name, &args[0], variable)
            } else {
                explain_general_function_derivative(name, args, variable)
            }
        }
        _ => {
            let result = expr.derivative(variable.clone());
            StepByStepExplanation {
                initial_expression: expr.clone(),
                final_expression: result.clone(),
                steps: vec![Step {
                    title: "Compute Derivative".to_string(),
                    description: format!(
                        "d/d{}({}) = {}",
                        variable.name(),
                        format_expr(expr),
                        format_expr(&result)
                    ),
                    expression: result.clone(),
                    rule_applied: "General Differentiation".to_string(),
                    latex: Some(
                        result
                            .to_latex(None)
                            .unwrap_or_else(|_| "result".to_string()),
                    ),
                }],
                total_steps: 1,
                rules_used: vec!["General Differentiation".to_string()],
            }
        }
    }
}

/// Check if power rule is directly applicable
fn is_power_rule_applicable(base: &Expression, _exp: &Expression, variable: &Symbol) -> bool {
    matches!(base, Expression::Symbol(sym) if sym == variable)
}

/// Explain general power derivative (using chain rule if needed)
fn explain_general_power_derivative(
    base: &Expression,
    exp: &Expression,
    variable: &Symbol,
) -> StepByStepExplanation {
    let expr = Expression::pow(base.clone(), exp.clone());
    let result = expr.derivative(variable.clone());

    StepByStepExplanation {
        initial_expression: expr.clone(),
        final_expression: result.clone(),
        steps: vec![Step {
            title: "Apply General Power Rule".to_string(),
            description: format!(
                "d/d{}({}) = {}",
                variable.name(),
                format_expr(&expr),
                format_expr(&result)
            ),
            expression: result.clone(),
            rule_applied: "General Power Rule".to_string(),
            latex: Some(
                result
                    .to_latex(None)
                    .unwrap_or_else(|_| "result".to_string()),
            ),
        }],
        total_steps: 1,
        rules_used: vec!["General Power Rule".to_string()],
    }
}

/// Explain general product rule for multiple factors
fn explain_general_product_rule(
    factors: &[Expression],
    variable: &Symbol,
) -> StepByStepExplanation {
    let expr = Expression::mul(factors.to_vec());
    let result = expr.derivative(variable.clone());

    StepByStepExplanation {
        initial_expression: expr.clone(),
        final_expression: result.clone(),
        steps: vec![Step {
            title: "Apply General Product Rule".to_string(),
            description: format!(
                "d/d{}({}) = {}",
                variable.name(),
                format_expr(&expr),
                format_expr(&result)
            ),
            expression: result.clone(),
            rule_applied: "General Product Rule".to_string(),
            latex: Some(
                result
                    .to_latex(None)
                    .unwrap_or_else(|_| "result".to_string()),
            ),
        }],
        total_steps: 1,
        rules_used: vec!["General Product Rule".to_string()],
    }
}

/// Explain general function derivative
fn explain_general_function_derivative(
    name: &str,
    args: &[Expression],
    variable: &Symbol,
) -> StepByStepExplanation {
    let expr = Expression::function(name, args.to_vec());
    let result = expr.derivative(variable.clone());

    StepByStepExplanation {
        initial_expression: expr.clone(),
        final_expression: result.clone(),
        steps: vec![Step {
            title: "Apply Function Derivative".to_string(),
            description: format!(
                "d/d{}({}) = {}",
                variable.name(),
                format_expr(&expr),
                format_expr(&result)
            ),
            expression: result.clone(),
            rule_applied: "Function Derivative".to_string(),
            latex: Some(
                result
                    .to_latex(None)
                    .unwrap_or_else(|_| "result".to_string()),
            ),
        }],
        total_steps: 1,
        rules_used: vec!["Function Derivative".to_string()],
    }
}

/// Detect if a multiplication represents a quotient (division)
pub(crate) fn detect_quotient(factors: &[Expression]) -> Option<(Expression, Expression)> {
    use crate::core::Number;

    if factors.len() != 2 {
        return None;
    }

    let (first, second) = (&factors[0], &factors[1]);

    if let Expression::Pow(base, exp) = second {
        if let Expression::Number(Number::Integer(-1)) = exp.as_ref() {
            return Some((first.clone(), base.as_ref().clone()));
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::symbol;

    #[test]
    fn test_constant_derivative_explanation() {
        let x = symbol!(x);
        let expr = Expression::integer(5);
        let explanation = expr.derivative_with_steps(&x, 1);
        assert!(explanation.steps.len() >= 2);
        assert_eq!(explanation.final_expression, Expression::integer(0));
    }

    #[test]
    fn test_variable_derivative_explanation() {
        let x = symbol!(x);
        let expr = Expression::symbol(x.clone());
        let explanation = expr.derivative_with_steps(&x, 1);
        assert!(explanation.steps.len() >= 2);
        assert_eq!(explanation.final_expression, Expression::integer(1));
    }

    #[test]
    fn test_power_rule_explanation() {
        let x = symbol!(x);
        let expr = Expression::pow(Expression::symbol(x.clone()), Expression::integer(3));
        let explanation = expr.derivative_with_steps(&x, 1);
        assert!(explanation.steps.len() >= 4);
    }

    #[test]
    fn test_sum_rule_explanation() {
        let x = symbol!(x);
        let expr = Expression::add(vec![
            Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
            Expression::mul(vec![Expression::integer(3), Expression::symbol(x.clone())]),
            Expression::integer(5),
        ]);
        let explanation = expr.derivative_with_steps(&x, 1);
        assert!(explanation.steps.len() >= 4);
    }

    #[test]
    fn test_product_rule_explanation() {
        let x = symbol!(x);
        let expr = Expression::mul(vec![
            Expression::symbol(x.clone()),
            Expression::function("sin", vec![Expression::symbol(x.clone())]),
        ]);
        let explanation = expr.derivative_with_steps(&x, 1);
        assert!(explanation.steps.len() >= 5);
    }

    #[test]
    fn test_chain_rule_explanation() {
        let x = symbol!(x);
        let expr = Expression::function(
            "sin",
            vec![Expression::pow(
                Expression::symbol(x.clone()),
                Expression::integer(2),
            )],
        );
        let explanation = expr.derivative_with_steps(&x, 1);
        assert!(explanation.steps.len() >= 5);
    }
}
