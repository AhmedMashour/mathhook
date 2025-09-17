//! Polynomial GCD educational explanations

use super::super::algorithms::integer_gcd;
use super::super::algorithms::zippel_gcd::educational;
use super::super::classification::PolynomialClassification;
use super::super::gcd_ops::PolynomialGcdOps;
use super::super::properties::PolynomialProperties;
use super::create_explanation;
use crate::core::{Expression, Number};
use crate::educational::step_by_step::{Step, StepByStepExplanation};

pub fn explain_poly_gcd_impl(expr: &Expression, other: &Expression) -> StepByStepExplanation {
    let mut steps = Vec::new();

    steps.push(Step {
        title: "Polynomial GCD".to_owned(),
        description: format!("Compute GCD of {} and {}", expr, other),
        expression: expr.clone(),
        rule_applied: "Initial".to_owned(),
        latex: None,
    });

    if let (Expression::Number(Number::Integer(n1)), Expression::Number(Number::Integer(n2))) =
        (expr, other)
    {
        let gcd = integer_gcd(*n1, *n2);
        steps.push(Step {
            title: "Integer GCD".to_owned(),
            description: format!(
                "Both inputs are integers. Using Euclidean algorithm:\n\
                 GCD({}, {}) = {}",
                n1, n2, gcd
            ),
            expression: Expression::integer(gcd),
            rule_applied: "Euclidean Algorithm".to_owned(),
            latex: None,
        });

        return create_explanation(expr.clone(), Expression::integer(gcd), steps);
    }

    if expr.is_zero() {
        steps.push(Step {
            title: "Zero Input".to_owned(),
            description: format!("GCD(0, b) = |b|\nResult: {}", other),
            expression: other.clone(),
            rule_applied: "Zero Property".to_owned(),
            latex: None,
        });
        return create_explanation(expr.clone(), other.clone(), steps);
    }

    if other.is_zero() {
        steps.push(Step {
            title: "Zero Input".to_owned(),
            description: format!("GCD(a, 0) = |a|\nResult: {}", expr),
            expression: expr.clone(),
            rule_applied: "Zero Property".to_owned(),
            latex: None,
        });
        return create_explanation(expr.clone(), expr.clone(), steps);
    }

    if expr.is_one() || other.is_one() {
        steps.push(Step {
            title: "Coprime with 1".to_owned(),
            description: "GCD with 1 is always 1".to_owned(),
            expression: Expression::integer(1),
            rule_applied: "Identity Property".to_owned(),
            latex: None,
        });
        return create_explanation(expr.clone(), Expression::integer(1), steps);
    }

    let vars_self = expr.polynomial_variables();
    let vars_other = other.polynomial_variables();
    let mut all_vars: Vec<_> = vars_self.clone();
    for v in vars_other.iter() {
        if !all_vars.iter().any(|s| s.name() == v.name()) {
            all_vars.push(v.clone());
        }
    }
    let is_univariate = all_vars.len() <= 1;

    let max_degree = if let Some(var) = all_vars.first() {
        let d1 = expr.degree(var).unwrap_or(0);
        let d2 = other.degree(var).unwrap_or(0);
        d1.max(d2)
    } else {
        0
    };

    let has_large_coeffs = max_degree >= 20;
    let is_sparse = max_degree >= 15;

    steps.push(Step {
        title: "Analyze Polynomial Characteristics".to_owned(),
        description: format!(
            "Variables: {} ({})\n\
             Maximum degree: {}\n\
             Characteristics: {}",
            if all_vars.is_empty() {
                "constant".to_owned()
            } else {
                all_vars
                    .iter()
                    .map(|v| v.name().to_owned())
                    .collect::<Vec<_>>()
                    .join(", ")
            },
            if is_univariate {
                "univariate"
            } else {
                "multivariate"
            },
            max_degree,
            if max_degree >= 10 {
                "high-degree (modular methods preferred)"
            } else {
                "low-degree (classical methods efficient)"
            }
        ),
        expression: expr.clone(),
        rule_applied: "Analysis".to_owned(),
        latex: None,
    });

    let rationale = educational::explain_selection_rationale(
        is_univariate,
        max_degree,
        is_sparse,
        has_large_coeffs,
    );
    steps.push(Step {
        title: "Algorithm Selection".to_owned(),
        description: rationale,
        expression: expr.clone(),
        rule_applied: "Selection".to_owned(),
        latex: None,
    });

    let use_modular = is_sparse || has_large_coeffs || max_degree >= 10 || !is_univariate;

    if use_modular {
        steps.push(Step {
            title: "Zippel Modular GCD Overview".to_owned(),
            description: educational::algorithm_overview().to_owned(),
            expression: expr.clone(),
            rule_applied: "Algorithm Description".to_owned(),
            latex: None,
        });

        steps.push(Step {
            title: "Content Extraction".to_owned(),
            description: educational::explain_content_extraction().to_owned(),
            expression: expr.clone(),
            rule_applied: "Step 1".to_owned(),
            latex: None,
        });

        steps.push(Step {
            title: "CRT Reconstruction".to_owned(),
            description: educational::explain_crt_reconstruction().to_owned(),
            expression: expr.clone(),
            rule_applied: "Step 2-3".to_owned(),
            latex: None,
        });

        steps.push(Step {
            title: "Trial Division Verification".to_owned(),
            description: educational::explain_trial_division().to_owned(),
            expression: expr.clone(),
            rule_applied: "Step 4".to_owned(),
            latex: None,
        });
    } else {
        steps.push(Step {
            title: "Euclidean Algorithm".to_owned(),
            description: "The polynomial GCD is computed using the Euclidean algorithm:\n\
                         1. Divide larger degree polynomial by smaller\n\
                         2. Replace larger with remainder\n\
                         3. Repeat until remainder is zero\n\
                         4. Last non-zero remainder is the GCD"
                .to_owned(),
            expression: expr.clone(),
            rule_applied: "Algorithm".to_owned(),
            latex: None,
        });
    }

    match expr.polynomial_gcd(other) {
        Ok(gcd) => {
            steps.push(Step {
                title: "GCD Result".to_owned(),
                description: format!("GCD({}, {}) = {}", expr, other, gcd),
                expression: gcd.clone(),
                rule_applied: "Final".to_owned(),
                latex: None,
            });

            create_explanation(expr.clone(), gcd, steps)
        }
        Err(e) => {
            steps.push(Step {
                title: "Error".to_owned(),
                description: format!("GCD computation failed: {}", e),
                expression: expr.clone(),
                rule_applied: "Error".to_owned(),
                latex: None,
            });

            create_explanation(expr.clone(), Expression::integer(1), steps)
        }
    }
}
