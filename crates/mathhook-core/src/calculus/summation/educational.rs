//! Educational features for summation operations

use crate::calculus::summation::{ConvergenceResult, Summation, SummationMethods};
use crate::core::{Expression, Number, Symbol};
use crate::educational::message_registry::core::{
    MessageCategory, MessageKey, MessageType, MESSAGE_REGISTRY,
};
use crate::educational::step_by_step::{Step, StepByStepExplanation};
use crate::expr;
use crate::simplify::Simplify;

/// Educational extension trait for summation
pub trait SummationEducational {
    /// Explain finite sum computation with step-by-step guidance
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{expr, symbol};
    /// use mathhook_core::calculus::summation::educational::SummationEducational;
    ///
    /// let i = symbol!(i);
    /// let sum_expr = expr!(i);
    /// let explanation = sum_expr.explain_finite_sum(&i, &expr!(1), &expr!(10));
    ///
    /// for step in &explanation.steps {
    ///     println!("{}: {}", step.title, step.description);
    /// }
    /// ```
    fn explain_finite_sum(
        &self,
        variable: &Symbol,
        start: &Expression,
        end: &Expression,
    ) -> StepByStepExplanation;

    /// Explain infinite sum computation with convergence analysis
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{expr, symbol};
    /// use mathhook_core::calculus::summation::educational::SummationEducational;
    ///
    /// let n = symbol!(n);
    /// let sum_expr = expr!(n ^ (-2));
    /// let explanation = sum_expr.explain_infinite_sum(&n, &expr!(1));
    ///
    /// for step in &explanation.steps {
    ///     println!("{}: {}", step.title, step.description);
    /// }
    /// ```
    fn explain_infinite_sum(&self, variable: &Symbol, start: &Expression) -> StepByStepExplanation;
}

impl SummationEducational for Expression {
    fn explain_finite_sum(
        &self,
        variable: &Symbol,
        start: &Expression,
        end: &Expression,
    ) -> StepByStepExplanation {
        let mut steps = Vec::new();
        let initial_expr = self.clone();

        let num_terms = Expression::add(vec![
            end.clone(),
            Expression::mul(vec![expr!(-1), start.clone()]),
            expr!(1),
        ])
        .simplify();

        if let Some(template) = MESSAGE_REGISTRY.get(&MessageKey::new(
            MessageCategory::Calculus,
            MessageType::SummationIntroduction,
            0,
        )) {
            let mut desc = template.content.to_owned();
            desc = desc.replace("{variable}", variable.name());
            desc = desc.replace("{start}", &format!("{}", start));
            desc = desc.replace("{end}", &format!("{}", end));
            desc = desc.replace("{term_count}", &format!("{}", num_terms));

            steps.push(Step {
                title: template.title.to_owned(),
                description: desc,
                expression: self.clone(),
                rule_applied: "Introduction".to_owned(),
                latex: None,
            });
        }

        let series_type = detect_series_type(self, variable);
        match series_type {
            SeriesType::Arithmetic(first, diff) => {
                if let Some(template) = MESSAGE_REGISTRY.get(&MessageKey::new(
                    MessageCategory::Calculus,
                    MessageType::SummationArithmeticSeries,
                    0,
                )) {
                    let mut desc = template.content.to_owned();
                    desc = desc.replace("{first_term}", &format!("{}", first));
                    desc = desc.replace("{common_difference}", &format!("{}", diff));
                    desc = desc.replace("{term_count}", &format!("{}", num_terms));

                    steps.push(Step {
                        title: template.title.to_owned(),
                        description: desc,
                        expression: self.clone(),
                        rule_applied: "Series Detection".to_owned(),
                        latex: None,
                    });
                }

                if let Some(template) = MESSAGE_REGISTRY.get(&MessageKey::new(
                    MessageCategory::Calculus,
                    MessageType::SummationArithmeticSeries,
                    1,
                )) {
                    steps.push(Step {
                        title: template.title.to_owned(),
                        description: template.content.to_owned(),
                        expression: self.clone(),
                        rule_applied: "Formula".to_owned(),
                        latex: None,
                    });
                }
            }
            SeriesType::PowerSum(k) => {
                if let Some(template) = MESSAGE_REGISTRY.get(&MessageKey::new(
                    MessageCategory::Calculus,
                    MessageType::SummationPowerSum,
                    k.min(4) as u8,
                )) {
                    let mut desc = template.content.to_owned();
                    if k > 3 {
                        desc = desc.replace("{power}", &format!("{}", k));
                    }

                    steps.push(Step {
                        title: template.title.to_owned(),
                        description: desc,
                        expression: self.clone(),
                        rule_applied: "Power Sum Formula".to_owned(),
                        latex: None,
                    });
                }
            }
            SeriesType::General => {}
        }

        let result = self.finite_sum(variable, start, end);

        if let Some(template) = MESSAGE_REGISTRY.get(&MessageKey::new(
            MessageCategory::Calculus,
            MessageType::SummationResult,
            0,
        )) {
            let mut desc = template.content.to_owned();
            desc = desc.replace("{result}", &format!("{}", result));

            steps.push(Step {
                title: template.title.to_owned(),
                description: desc,
                expression: result.clone(),
                rule_applied: "Final Result".to_owned(),
                latex: None,
            });
        }

        let total_steps = steps.len().saturating_sub(2);

        StepByStepExplanation {
            initial_expression: initial_expr,
            final_expression: result,
            steps,
            total_steps,
            rules_used: vec!["Summation".to_owned()],
        }
    }

    fn explain_infinite_sum(&self, variable: &Symbol, start: &Expression) -> StepByStepExplanation {
        let mut steps = Vec::new();
        let initial_expr = self.clone();

        if let Some(template) = MESSAGE_REGISTRY.get(&MessageKey::new(
            MessageCategory::Calculus,
            MessageType::SummationIntroduction,
            1,
        )) {
            let mut desc = template.content.to_owned();
            desc = desc.replace("{expression}", &format!("{}", self));
            desc = desc.replace("{start}", &format!("{}", start));

            steps.push(Step {
                title: template.title.to_owned(),
                description: desc,
                expression: self.clone(),
                rule_applied: "Introduction".to_owned(),
                latex: None,
            });
        }

        let convergence = SummationMethods::convergence_test(self, variable);
        let (convergent, reason) = match convergence {
            ConvergenceResult::Convergent => (
                true,
                "the terms decay fast enough (p-series test with p > 1)",
            ),
            ConvergenceResult::Divergent => (
                false,
                "the terms do not decay to zero fast enough (p-series test with p ≤ 1)",
            ),
            _ => (false, "convergence cannot be determined automatically"),
        };

        let variant = if convergent { 0 } else { 1 };
        if let Some(template) = MESSAGE_REGISTRY.get(&MessageKey::new(
            MessageCategory::Calculus,
            MessageType::SummationConvergence,
            variant,
        )) {
            let mut desc = template.content.to_owned();
            desc = desc.replace("{reason}", reason);

            steps.push(Step {
                title: template.title.to_owned(),
                description: desc,
                expression: self.clone(),
                rule_applied: "Convergence Test".to_owned(),
                latex: None,
            });
        }

        let result = self.infinite_sum(variable, start);

        if let Some(template) = MESSAGE_REGISTRY.get(&MessageKey::new(
            MessageCategory::Calculus,
            MessageType::SummationResult,
            0,
        )) {
            let mut desc = template.content.to_owned();
            desc = desc.replace("{result}", &format!("{}", result));

            steps.push(Step {
                title: template.title.to_owned(),
                description: desc,
                expression: result.clone(),
                rule_applied: "Final Result".to_owned(),
                latex: None,
            });
        }

        let total_steps = steps.len().saturating_sub(2);

        StepByStepExplanation {
            initial_expression: initial_expr,
            final_expression: result,
            steps,
            total_steps,
            rules_used: vec!["Infinite Sum".to_owned()],
        }
    }
}

fn detect_series_type(expr: &Expression, variable: &Symbol) -> SeriesType {
    if let Expression::Symbol(sym) = expr {
        if sym == variable {
            return SeriesType::Arithmetic(expr!(1), expr!(1));
        }
    }

    if let Expression::Pow(base, exp) = expr {
        if let (Expression::Symbol(sym), Expression::Number(Number::Integer(k))) =
            (base.as_ref(), exp.as_ref())
        {
            if sym == variable && k.is_positive() && *k <= 100 {
                return SeriesType::PowerSum(*k as u32);
            }
        }
    }

    SeriesType::General
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
enum SeriesType {
    Arithmetic(Expression, Expression),
    PowerSum(u32),
    General,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::symbol;

    #[test]
    fn test_explain_finite_sum_arithmetic() {
        let i = symbol!(i);
        let expr_i: Expression = i.clone().into();
        let explanation = expr_i.explain_finite_sum(&i, &expr!(1), &expr!(10));

        assert!(explanation.steps.len() >= 3);
        assert_eq!(explanation.final_expression, expr!(55));
    }

    #[test]
    fn test_explain_finite_sum_power() {
        let i = symbol!(i);
        let expr = expr!(i ^ 2);
        let explanation = expr.explain_finite_sum(&i, &expr!(1), &expr!(3));

        assert!(
            !explanation.steps.is_empty(),
            "Should have at least some steps"
        );

        assert!(
            explanation.steps.len() >= 2,
            "Should have at least intro and result"
        );
    }

    #[test]
    fn test_explain_infinite_sum_convergent() {
        let n = symbol!(n);
        let expr = expr!(n ^ (-2));
        let explanation = expr.explain_infinite_sum(&n, &expr!(1));

        assert!(explanation.steps.len() >= 2);
    }
}
