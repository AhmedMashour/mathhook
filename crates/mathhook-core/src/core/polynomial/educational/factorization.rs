//! Polynomial factorization educational explanations
//!
//! Provides step-by-step educational explanations for polynomial factorization.
//! Uses the `AdvancedPolynomial` trait for content extraction and primitive part
//! computation.

use super::super::classification::PolynomialClassification;
use super::create_explanation;
use crate::algebra::polynomial_advanced::AdvancedPolynomial;
use crate::core::{Expression, Symbol};
use crate::educational::step_by_step::{Step, StepByStepExplanation};

pub fn explain_poly_factorization_impl(expr: &Expression, var: &Symbol) -> StepByStepExplanation {
    let mut steps = Vec::new();

    steps.push(Step {
        title: "Polynomial Factorization".to_owned(),
        description: format!("Factor {} with respect to {}", expr, var.name()),
        expression: expr.clone(),
        rule_applied: "Initial".to_owned(),
        latex: None,
    });

    if !expr.is_polynomial_in(std::slice::from_ref(var)) {
        steps.push(Step {
            title: "Not a Polynomial".to_owned(),
            description: format!("{} is not a polynomial in {}", expr, var.name()),
            expression: expr.clone(),
            rule_applied: "Validation".to_owned(),
            latex: None,
        });
        return create_explanation(expr.clone(), expr.clone(), steps);
    }

    steps.push(Step {
        title: "Extract Content".to_owned(),
        description: "Find the GCD of all coefficients (the content)".to_owned(),
        expression: expr.clone(),
        rule_applied: "Content Extraction".to_owned(),
        latex: None,
    });

    let content = expr.polynomial_content();

    if content != Expression::integer(1) {
        steps.push(Step {
            title: "Content Found".to_owned(),
            description: format!("Content = {}", content),
            expression: content.clone(),
            rule_applied: "GCD of Coefficients".to_owned(),
            latex: None,
        });

        let primitive = expr.polynomial_primitive_part();
        steps.push(Step {
            title: "Primitive Part".to_owned(),
            description: "Divide by content to get primitive polynomial".to_owned(),
            expression: primitive.clone(),
            rule_applied: "Content Division".to_owned(),
            latex: None,
        });

        let factored = Expression::mul(vec![content, primitive]);
        create_explanation(expr.clone(), factored, steps)
    } else {
        steps.push(Step {
            title: "Already Primitive".to_owned(),
            description: "Content = 1, polynomial is already primitive".to_owned(),
            expression: expr.clone(),
            rule_applied: "Content Check".to_owned(),
            latex: None,
        });
        create_explanation(expr.clone(), expr.clone(), steps)
    }
}
