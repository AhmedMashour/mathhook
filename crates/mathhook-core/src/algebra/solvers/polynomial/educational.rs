//! Educational explanations for polynomial equation solving

use crate::algebra::solvers::{EquationSolver, PolynomialSolver, SolverResult};
use crate::core::{Expression, Symbol};
use crate::educational::step_by_step::StepByStepExplanation;

/// Generate step-by-step explanation for polynomial solving
pub fn solve_with_explanation(
    solver: &PolynomialSolver,
    equation: &Expression,
    variable: &Symbol,
) -> (SolverResult, StepByStepExplanation) {
    use crate::educational::message_registry::{MessageBuilder, MessageCategory, MessageType};
    use crate::formatter::latex::LaTeXFormatter;

    let to_latex = |expr: &Expression| -> String {
        expr.to_latex(None).unwrap_or_else(|_| expr.to_string())
    };

    let degree = solver.find_polynomial_degree(equation, variable);
    let mut steps = Vec::new();

    let degree_name = match degree {
        3 => "cubic equation",
        4 => "quartic equation",
        _ => "polynomial equation",
    };

    steps.push(
        MessageBuilder::new(MessageCategory::PolynomialEquation, MessageType::Introduction, 0)
            .with_substitution("equation", &to_latex(equation))
            .with_substitution("degree", &degree.to_string())
            .with_substitution("degree_name", degree_name)
            .build()
            .unwrap(),
    );

    let strategy = match degree {
        3 => "Test rational roots using the Rational Root Theorem, then factor completely using synthetic division",
        4 => "Test rational roots using the Rational Root Theorem, then reduce to lower-degree polynomials",
        _ => "Apply appropriate polynomial solution techniques",
    };

    steps.push(
        MessageBuilder::new(MessageCategory::PolynomialEquation, MessageType::Strategy, 0)
            .with_substitution("degree", &degree.to_string())
            .with_substitution("strategy_description", strategy)
            .build()
            .unwrap(),
    );

    let (constant_term, leading_coef) = solver.extract_constant_and_leading(equation, variable);

    if constant_term != 0 && leading_coef != 0 {
        let constant_factors = solver.get_divisors(constant_term.abs());
        let leading_factors = solver.get_divisors(leading_coef.abs());

        let mut candidates = Vec::new();
        for p in &constant_factors {
            for q in &leading_factors {
                if *q != 0 {
                    let positive = if *p % *q == 0 { *p / *q } else { continue; };
                    let negative = -positive;
                    if !candidates.contains(&positive) {
                        candidates.push(positive);
                    }
                    if !candidates.contains(&negative) {
                        candidates.push(negative);
                    }
                }
            }
        }
        candidates.sort();

        let candidates_str = candidates
            .iter()
            .map(|c| c.to_string())
            .collect::<Vec<_>>()
            .join(", ");

        steps.push(
            MessageBuilder::new(
                MessageCategory::PolynomialEquation,
                MessageType::PolynomialRationalRoot,
                0,
            )
            .with_substitution("constant_term", &constant_term.to_string())
            .with_substitution("leading_coeff", &leading_coef.to_string())
            .with_substitution("candidates", &candidates_str)
            .build()
            .unwrap(),
        );

        let mut found_roots = Vec::new();
        for &candidate in &candidates {
            let test_value = Expression::integer(candidate);
            let evaluation = solver.evaluate_polynomial_at(equation, variable, &test_value);

            let result_str = if evaluation.is_zero() {
                found_roots.push(candidate);
                format!("= 0 (Root found!)")
            } else {
                format!("= {} (Not a root)", to_latex(&evaluation))
            };

            if evaluation.is_zero() {
                steps.push(
                    MessageBuilder::new(
                        MessageCategory::PolynomialEquation,
                        MessageType::PolynomialRationalRoot,
                        1,
                    )
                    .with_substitution("variable", variable.name())
                    .with_substitution("candidate", &candidate.to_string())
                    .with_substitution("equation", &to_latex(equation))
                    .with_substitution("evaluation", &to_latex(&evaluation))
                    .with_substitution("result", &result_str)
                    .build()
                    .unwrap(),
                );

                steps.push(
                    MessageBuilder::new(
                        MessageCategory::PolynomialEquation,
                        MessageType::PolynomialSyntheticDivision,
                        0,
                    )
                    .with_substitution("variable", variable.name())
                    .with_substitution("root", &candidate.to_string())
                    .with_substitution("polynomial", &to_latex(equation))
                    .build()
                    .unwrap(),
                );

                break;
            }

            if found_roots.len() >= 2 {
                break;
            }
        }

        if !found_roots.is_empty() {
            let factored_parts: Vec<String> = found_roots
                .iter()
                .map(|&r| {
                    if r >= 0 {
                        format!("({} - {})", variable.name(), r)
                    } else {
                        format!("({} + {})", variable.name(), -r)
                    }
                })
                .collect();

            let factored_form = factored_parts.join(" * ");

            steps.push(
                MessageBuilder::new(
                    MessageCategory::PolynomialEquation,
                    MessageType::PolynomialFactorization,
                    0,
                )
                .with_substitution("original_polynomial", &to_latex(equation))
                .with_substitution("factored_form", &factored_form)
                .build()
                .unwrap(),
            );
        }
    }

    let result = solver.solve(equation, variable);

    let solutions_str = match &result {
        SolverResult::Multiple(sols) | SolverResult::Partial(sols) => {
            sols.iter()
                .map(|s| format!("{} = {}", variable.name(), to_latex(s)))
                .collect::<Vec<_>>()
                .join("\n")
        }
        SolverResult::Single(sol) => format!("{} = {}", variable.name(), to_latex(sol)),
        _ => "No solutions found".to_string(),
    };

    let count = match &result {
        SolverResult::Multiple(sols) | SolverResult::Partial(sols) => sols.len(),
        SolverResult::Single(_) => 1,
        _ => 0,
    };

    steps.push(
        MessageBuilder::new(MessageCategory::PolynomialEquation, MessageType::Result, 0)
            .with_substitution("equation", &to_latex(equation))
            .with_substitution("solutions", &solutions_str)
            .with_substitution("count", &count.to_string())
            .build()
            .unwrap(),
    );

    if let SolverResult::Multiple(sols) | SolverResult::Partial(sols) = &result {
        if !sols.is_empty() {
            let first_root = &sols[0];
            let verification = solver.evaluate_polynomial_at(equation, variable, first_root);

            steps.push(
                MessageBuilder::new(
                    MessageCategory::PolynomialEquation,
                    MessageType::Verification,
                    0,
                )
                .with_substitution("variable", variable.name())
                .with_substitution("root", &to_latex(first_root))
                .with_substitution("verification_expression", &to_latex(equation))
                .with_substitution("result", &to_latex(&verification))
                .build()
                .unwrap(),
            );
        }
    }

    steps.push(
        MessageBuilder::new(MessageCategory::PolynomialEquation, MessageType::Insight, 0)
            .with_substitution("degree", &degree.to_string())
            .with_substitution("real_count", &count.to_string())
            .build()
            .unwrap(),
    );

    (result, StepByStepExplanation::new(steps))
}
