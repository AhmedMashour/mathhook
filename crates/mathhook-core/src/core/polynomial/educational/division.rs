//! Polynomial division educational explanations

use super::super::arithmetic::PolynomialArithmetic;
use super::super::classification::PolynomialClassification;
use super::super::properties::PolynomialProperties;
use super::create_explanation;
use crate::core::{Expression, Symbol};
use crate::educational::step_by_step::{Step, StepByStepExplanation};

pub fn explain_poly_division_impl(
    expr: &Expression,
    divisor: &Expression,
    var: &Symbol,
) -> StepByStepExplanation {
    let mut steps = Vec::new();

    steps.push(Step {
        title: "Polynomial Division".to_owned(),
        description: format!(
            "Divide {} by {} with respect to {}",
            expr,
            divisor,
            var.name()
        ),
        expression: expr.clone(),
        rule_applied: "Initial".to_owned(),
        latex: None,
    });

    if !expr.is_polynomial_in(std::slice::from_ref(var)) {
        steps.push(Step {
            title: "Not a Polynomial".to_owned(),
            description: format!(
                "The dividend {} is not a polynomial in {}",
                expr,
                var.name()
            ),
            expression: expr.clone(),
            rule_applied: "Validation".to_owned(),
            latex: None,
        });
        return create_explanation(expr.clone(), expr.clone(), steps);
    }

    if !divisor.is_polynomial_in(std::slice::from_ref(var)) {
        steps.push(Step {
            title: "Not a Polynomial".to_owned(),
            description: format!(
                "The divisor {} is not a polynomial in {}",
                divisor,
                var.name()
            ),
            expression: divisor.clone(),
            rule_applied: "Validation".to_owned(),
            latex: None,
        });
        return create_explanation(expr.clone(), expr.clone(), steps);
    }

    let dividend_deg = expr.degree(var).unwrap_or(0);
    let divisor_deg = divisor.degree(var).unwrap_or(0);

    steps.push(Step {
        title: "Identify Degrees".to_owned(),
        description: format!(
            "Dividend degree: {}\nDivisor degree: {}",
            dividend_deg, divisor_deg
        ),
        expression: expr.clone(),
        rule_applied: "Degree Computation".to_owned(),
        latex: None,
    });

    if divisor.is_zero() {
        steps.push(Step {
            title: "Division by Zero".to_owned(),
            description: "Cannot divide by zero polynomial".to_owned(),
            expression: divisor.clone(),
            rule_applied: "Error".to_owned(),
            latex: None,
        });
        return create_explanation(expr.clone(), expr.clone(), steps);
    }

    if divisor_deg > dividend_deg {
        steps.push(Step {
            title: "Divisor Degree Too High".to_owned(),
            description: format!(
                "Since divisor degree ({}) > dividend degree ({}), \
                 quotient = 0 and remainder = dividend",
                divisor_deg, dividend_deg
            ),
            expression: expr.clone(),
            rule_applied: "Degree Comparison".to_owned(),
            latex: None,
        });

        steps.push(Step {
            title: "Result".to_owned(),
            description: format!("Quotient: 0\nRemainder: {}", expr),
            expression: Expression::integer(0),
            rule_applied: "Final".to_owned(),
            latex: None,
        });

        return create_explanation(expr.clone(), Expression::integer(0), steps);
    }

    steps.push(Step {
        title: "Long Division Algorithm".to_owned(),
        description: "Repeatedly divide leading terms until remainder degree < divisor degree"
            .to_owned(),
        expression: expr.clone(),
        rule_applied: "Algorithm".to_owned(),
        latex: None,
    });

    match expr.poly_div(divisor, var) {
        Ok((quotient, remainder)) => {
            steps.push(Step {
                title: "Division Complete".to_owned(),
                description: format!(
                    "Quotient: {}\nRemainder: {}\n\nVerification: {} = ({}) * ({}) + ({})",
                    quotient, remainder, expr, divisor, quotient, remainder
                ),
                expression: quotient.clone(),
                rule_applied: "Result".to_owned(),
                latex: None,
            });

            create_explanation(expr.clone(), quotient, steps)
        }
        Err(e) => {
            steps.push(Step {
                title: "Error".to_owned(),
                description: format!("Division failed: {}", e),
                expression: expr.clone(),
                rule_applied: "Error".to_owned(),
                latex: None,
            });

            create_explanation(expr.clone(), expr.clone(), steps)
        }
    }
}
