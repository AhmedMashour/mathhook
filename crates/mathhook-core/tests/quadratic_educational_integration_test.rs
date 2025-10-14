//! Content validation tests for quadratic solver educational integration
//!
//! These tests validate the actual mathematical content of educational explanations,
//! not just their structure. This ensures the educational system provides correct
//! and meaningful explanations.

use mathhook_core::algebra::solvers::SolverResult;
use mathhook_core::core::{Expression, Number};
use mathhook_core::{expr, symbol};

#[test]
fn test_quadratic_solver_simple_integer_roots() {
    let x = symbol!(x);

    let equation = Expression::add(vec![
        Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
        Expression::mul(vec![Expression::integer(2), Expression::symbol(x.clone())]),
        Expression::integer(-3),
    ]);

    let (result, explanation) = equation.solve_equation(&x);

    assert!(
        explanation.steps.iter().any(|step| step
            .description
            .to_lowercase()
            .contains("quadratic equation")),
        "Missing equation type identification in steps: {:?}",
        explanation.steps
    );

    assert!(
        explanation.steps.iter().any(|step| {
            let desc = step.description.to_lowercase();
            desc.contains("coefficient") || desc.contains("a =") || desc.contains("b =")
        }),
        "Missing coefficient extraction explanation"
    );

    assert!(
        explanation.steps.iter().any(|step| {
            let desc = step.description.to_lowercase();
            desc.contains("discriminant") || desc.contains("δ") || desc.contains("b² - 4ac")
        }),
        "Missing discriminant calculation explanation"
    );

    assert!(
        explanation.steps.iter().any(|step| {
            let desc = step.description.to_lowercase();
            desc.contains("two distinct real solutions")
                || desc.contains("δ > 0")
                || desc.contains("positive")
        }),
        "Missing discriminant analysis (should indicate two real solutions)"
    );

    match result {
        SolverResult::Multiple(solutions) => {
            assert_eq!(solutions.len(), 2, "Quadratic should have 2 solutions");

            let sol_values: Vec<i64> = solutions
                .iter()
                .filter_map(|s| match s {
                    Expression::Number(Number::Integer(i)) => Some(*i),
                    Expression::Number(Number::Float(f)) => Some(*f as i64),
                    _ => None,
                })
                .collect();

            assert!(
                sol_values.contains(&1) && sol_values.contains(&-3),
                "Solutions should be x = 1 and x = -3, got: {:?}",
                sol_values
            );
        }
        _ => panic!("Expected multiple solutions, got: {:?}", result),
    }

    assert!(
        explanation.total_steps >= 5,
        "Educational explanation should have at least 5 steps for completeness"
    );
}

#[test]
fn test_quadratic_solver_repeated_root() {
    let x = symbol!(x);

    let equation = Expression::add(vec![
        Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
        Expression::mul(vec![Expression::integer(-4), Expression::symbol(x.clone())]),
        Expression::integer(4),
    ]);

    let (result, explanation) = equation.solve_equation(&x);

    assert!(
        explanation.steps.iter().any(|step| {
            let desc = step.description.to_lowercase();
            desc.contains("δ = 0")
                || desc.contains("discriminant") && desc.contains("0")
                || desc.contains("repeated")
                || desc.contains("one")
        }),
        "Missing discriminant = 0 analysis (repeated root)"
    );

    match result {
        SolverResult::Single(solution) => {
            let value = match solution {
                Expression::Number(Number::Integer(i)) => i,
                Expression::Number(Number::Float(f)) => f as i64,
                _ => panic!("Expected numeric solution"),
            };

            assert_eq!(value, 2, "Repeated root should be x = 2, got: {}", value);
        }
        _ => panic!("Expected single solution for repeated root, got: {:?}", result),
    }
}

#[test]
fn test_quadratic_solver_complex_roots() {
    let x = symbol!(x);

    let equation = Expression::add(vec![
        Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
        Expression::mul(vec![Expression::integer(2), Expression::symbol(x.clone())]),
        Expression::integer(5),
    ]);

    let (result, explanation) = equation.solve_equation(&x);

    assert!(
        explanation.steps.iter().any(|step| {
            let desc = step.description.to_lowercase();
            desc.contains("δ < 0")
                || desc.contains("complex")
                || desc.contains("negative")
        }),
        "Missing negative discriminant analysis (complex roots)"
    );

    match result {
        SolverResult::Multiple(solutions) => {
            assert_eq!(solutions.len(), 2, "Quadratic should have 2 solutions");
        }
        _ => panic!("Expected multiple (complex) solutions, got: {:?}", result),
    }
}

#[test]
fn test_linear_degenerate_case() {
    let x = symbol!(x);

    let equation = Expression::add(vec![
        Expression::mul(vec![Expression::integer(0), Expression::symbol(x.clone())]),
        Expression::mul(vec![Expression::integer(3), Expression::symbol(x.clone())]),
        Expression::integer(-9),
    ]);

    let (result, explanation) = equation.solve_equation(&x);

    assert!(
        explanation.steps.iter().any(|step| {
            let desc = step.description.to_lowercase();
            desc.contains("linear") || desc.contains("a = 0")
        }),
        "Missing degenerate case (linear equation) identification"
    );

    match result {
        SolverResult::Single(solution) => {
            let value = match solution {
                Expression::Number(Number::Integer(i)) => i,
                Expression::Number(Number::Float(f)) => f as i64,
                _ => panic!("Expected numeric solution"),
            };

            assert_eq!(value, 3, "Linear equation 3x - 9 = 0 should give x = 3");
        }
        _ => panic!(
            "Expected single solution for linear equation, got: {:?}",
            result
        ),
    }
}

#[test]
fn test_educational_steps_use_latex_formatting() {
    let x = symbol!(x);

    let equation = Expression::add(vec![
        Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
        Expression::integer(-4),
    ]);

    let (_result, explanation) = equation.solve_equation(&x);

    let has_mathematical_notation = explanation.steps.iter().any(|step| {
        let desc = &step.description;
        desc.contains("²")
            || desc.contains("√")
            || desc.contains("±")
            || desc.contains("Δ")
            || desc.contains("₁")
            || desc.contains("₂")
    });

    assert!(
        has_mathematical_notation,
        "Educational steps should use mathematical notation (Unicode or LaTeX)"
    );
}

#[test]
fn test_smart_solver_integration_with_analysis() {
    let x = symbol!(x);

    let equation = Expression::add(vec![
        Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
        Expression::mul(vec![Expression::integer(-5), Expression::symbol(x.clone())]),
        Expression::integer(6),
    ]);

    let (_result, explanation) = equation.solve_equation(&x);

    let has_analysis_step = explanation.steps.iter().any(|step| {
        step.title.to_lowercase().contains("analysis")
            || step
                .description
                .to_lowercase()
                .contains("detected quadratic")
            || step.description.to_lowercase().contains("degree: 2")
    });

    assert!(
        has_analysis_step,
        "Smart solver should include equation analysis as first step"
    );

    let has_solver_selection = explanation.steps.iter().any(|step| {
        step.title.to_lowercase().contains("solver")
            || step
                .description
                .to_lowercase()
                .contains("using quadratic")
            || step
                .description
                .to_lowercase()
                .contains("quadratic formula")
    });

    assert!(
        has_solver_selection,
        "Smart solver should explain which solver was selected"
    );
}

#[test]
fn test_complete_educational_flow_content() {
    let x = symbol!(x);

    let equation = Expression::add(vec![
        Expression::mul(vec![
            Expression::integer(2),
            Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
        ]),
        Expression::mul(vec![Expression::integer(-8), Expression::symbol(x.clone())]),
        Expression::integer(6),
    ]);

    let (result, explanation) = equation.solve_equation(&x);

    let educational_stages = [
        ("analysis", "Equation type identification"),
        ("solver", "Solver selection explanation"),
        ("coefficient", "Coefficient extraction"),
        ("discriminant", "Discriminant calculation"),
        ("solution", "Final solution"),
    ];

    for (keyword, stage_name) in educational_stages.iter() {
        let has_stage = explanation.steps.iter().any(|step| {
            step.title.to_lowercase().contains(keyword)
                || step.description.to_lowercase().contains(keyword)
        });

        assert!(
            has_stage,
            "Missing educational stage: {} (keyword: {})",
            stage_name, keyword
        );
    }

    match result {
        SolverResult::Multiple(solutions) => {
            assert_eq!(solutions.len(), 2);
        }
        _ => panic!("Expected solutions for 2x² - 8x + 6 = 0"),
    }
}
