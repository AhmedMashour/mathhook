//! Step-by-step explanation tests for noncommutative algebra
//!
//! Tests cover educational explanations and integration tests

use mathhook_core::educational::message_registry::{
    MessageBuilder, MessageCategory, MessageType,
};

/// Step-by-Step Explanation Tests (12 tests)

#[test]
fn test_left_division_explanation_clarity() {
    let step = MessageBuilder::new(
        MessageCategory::NoncommutativeAlgebra,
        MessageType::LeftMultiplyInverse,
        1,
    )
    .with_substitution("A", "A")
    .with_substitution("X", "X")
    .with_substitution("B", "B")
    .with_substitution("A_inv", "A^(-1)")
    .build();

    assert!(step.is_some());
    let desc = step.unwrap().description;

    assert!(
        desc.contains("LEFT") || desc.contains("left"),
        "Explanation should mention LEFT"
    );
    assert!(
        desc.contains("RIGHT") || desc.contains("right"),
        "Explanation should mention position context"
    );
}

#[test]
fn test_right_division_explanation_clarity() {
    let step = MessageBuilder::new(
        MessageCategory::NoncommutativeAlgebra,
        MessageType::RightMultiplyInverse,
        1,
    )
    .with_substitution("X", "X")
    .with_substitution("A", "A")
    .with_substitution("B", "B")
    .with_substitution("A_inv", "A^(-1)")
    .build();

    assert!(step.is_some());
    let desc = step.unwrap().description;

    assert!(
        desc.contains("RIGHT") || desc.contains("right"),
        "Explanation should mention RIGHT"
    );
    assert!(
        desc.contains("LEFT") || desc.contains("left"),
        "Explanation should mention position context"
    );
}

#[test]
fn test_explanation_includes_left_or_right() {
    let left_step = MessageBuilder::new(
        MessageCategory::NoncommutativeAlgebra,
        MessageType::LeftMultiplyInverse,
        0,
    )
    .build()
    .unwrap();

    let right_step = MessageBuilder::new(
        MessageCategory::NoncommutativeAlgebra,
        MessageType::RightMultiplyInverse,
        0,
    )
    .build()
    .unwrap();

    assert!(
        left_step.description.contains("LEFT"),
        "Left step must explicitly say LEFT"
    );
    assert!(
        right_step.description.contains("RIGHT"),
        "Right step must explicitly say RIGHT"
    );
}

#[test]
fn test_explanation_explains_why_order_matters() {
    let educational_step = MessageBuilder::new(
        MessageCategory::NoncommutativeAlgebra,
        MessageType::LeftMultiplyInverse,
        3,
    )
    .with_substitution("A", "A")
    .with_substitution("X", "X")
    .with_substitution("B", "B")
    .with_substitution("A_inv", "A^(-1)")
    .build();

    assert!(educational_step.is_some());
    let desc = educational_step.unwrap().description;

    assert!(
        desc.contains("Why") || desc.contains("why"),
        "Should explain WHY"
    );
    assert!(
        desc.contains("noncommutative") || desc.contains("NOT equal"),
        "Should explain noncommutativity"
    );
}

#[test]
fn test_explanation_for_matrix_equations() {
    let matrix_warning = MessageBuilder::new(
        MessageCategory::NoncommutativeAlgebra,
        MessageType::NoncommutativeWarning,
        1,
    )
    .build();

    assert!(matrix_warning.is_some());
    let desc = matrix_warning.unwrap().description;

    assert!(desc.contains("Matri") || desc.contains("matri"));
    assert!(desc.contains("NOT equal") || desc.contains("not equal"));
}

#[test]
fn test_explanation_for_operator_equations() {
    let operator_warning = MessageBuilder::new(
        MessageCategory::NoncommutativeAlgebra,
        MessageType::NoncommutativeWarning,
        2,
    )
    .build();

    assert!(operator_warning.is_some());
    let desc = operator_warning.unwrap().description;

    assert!(desc.contains("operator") || desc.contains("Operator"));
    assert!(desc.contains("commut"));
}

#[test]
fn test_explanation_for_quaternion_equations() {
    let quaternion_warning = MessageBuilder::new(
        MessageCategory::NoncommutativeAlgebra,
        MessageType::NoncommutativeWarning,
        3,
    )
    .build();

    assert!(quaternion_warning.is_some());
    let desc = quaternion_warning.unwrap().description;

    assert!(desc.contains("Quaternion") || desc.contains("quaternion"));
    assert!(desc.contains("ij") || desc.contains("i*j"));
}

#[test]
fn test_explanation_for_commutative_scalar_equations_unchanged() {
    let linear_step = MessageBuilder::new(
        MessageCategory::LinearEquation,
        MessageType::Strategy,
        0,
    )
    .with_substitution("variable", "x")
    .build();

    assert!(linear_step.is_some());
    let desc = linear_step.unwrap().description;

    assert!(
        !desc.contains("LEFT") && !desc.contains("RIGHT"),
        "Scalar equations should not mention left/right"
    );
}

#[test]
fn test_full_step_by_step_output_for_a_x_equals_b() {
    let steps = vec![
        MessageBuilder::new(
            MessageCategory::NoncommutativeAlgebra,
            MessageType::LeftMultiplyInverse,
            1,
        )
        .with_substitution("A", "A")
        .with_substitution("X", "X")
        .with_substitution("B", "B")
        .with_substitution("A_inv", "A^(-1)")
        .build()
        .unwrap(),
        MessageBuilder::new(
            MessageCategory::NoncommutativeAlgebra,
            MessageType::LeftMultiplyInverse,
            2,
        )
        .with_substitution("A", "A")
        .with_substitution("X", "X")
        .with_substitution("B", "B")
        .with_substitution("A_inv", "A^(-1)")
        .build()
        .unwrap(),
        MessageBuilder::new(
            MessageCategory::NoncommutativeAlgebra,
            MessageType::LeftMultiplyInverse,
            3,
        )
        .with_substitution("A", "A")
        .with_substitution("X", "X")
        .with_substitution("B", "B")
        .with_substitution("A_inv", "A^(-1)")
        .build()
        .unwrap(),
    ];

    assert_eq!(steps.len(), 3);

    assert!(steps[0].description.contains("LEFT"));
    assert!(steps[1].description.contains("associativity"));
    assert!(steps[2].description.contains("Why") || steps[2].description.contains("why"));
}

#[test]
fn test_order_matters_message() {
    let step = MessageBuilder::new(
        MessageCategory::NoncommutativeAlgebra,
        MessageType::OrderMatters,
        0,
    )
    .with_substitution("symbol", "A")
    .with_substitution("symbol_type", "Matrix")
    .with_substitution("A", "A")
    .with_substitution("B", "B")
    .build();

    assert!(step.is_some());
    let desc = step.unwrap().description;
    assert!(desc.contains("order"));
    assert!(desc.contains("matters"));
}

#[test]
fn test_associativity_still_valid_message() {
    let step = MessageBuilder::new(
        MessageCategory::NoncommutativeAlgebra,
        MessageType::OrderMatters,
        2,
    )
    .with_substitution("A", "A")
    .with_substitution("B", "B")
    .with_substitution("C", "C")
    .build();

    assert!(step.is_some());
    let desc = step.unwrap().description;
    assert!(desc.contains("associativity") || desc.contains("Associativity"));
}

#[test]
fn test_common_errors_message() {
    let step = MessageBuilder::new(
        MessageCategory::NoncommutativeAlgebra,
        MessageType::OrderMatters,
        3,
    )
    .with_substitution("A", "A")
    .with_substitution("B", "B")
    .with_substitution("C", "C")
    .with_substitution("X", "X")
    .with_substitution("Y", "Y")
    .build();

    assert!(step.is_some());
    let desc = step.unwrap().description;
    assert!(desc.contains("mistake") || desc.contains("error") || desc.contains("Common"));
}
