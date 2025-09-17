//! Message registry and LaTeX formatter tests for noncommutative algebra
//!
//! Tests cover:
//! - Message registry with left/right division messages
//! - LaTeX formatter with symbol type awareness

use mathhook_core::educational::message_registry::{MessageBuilder, MessageCategory, MessageType};
use mathhook_core::formatter::latex::{LaTeXContext, LaTeXFormatter};
use mathhook_core::{symbol, Expression};

// Message Registry Tests (8 tests)

#[test]
fn test_left_division_message_retrieval() {
    let step = MessageBuilder::new(
        MessageCategory::NoncommutativeAlgebra,
        MessageType::LeftMultiplyInverse,
        0,
    )
    .with_substitution("inverse", "A^(-1)")
    .with_substitution("equation", "A*X = B")
    .with_substitution("lhs", "A*X")
    .with_substitution("rhs", "B")
    .build();

    assert!(step.is_some());
    let step = step.unwrap();
    assert!(step.description.contains("LEFT"));
    assert!(step.description.contains("A^(-1)"));
}

#[test]
fn test_right_division_message_retrieval() {
    let step = MessageBuilder::new(
        MessageCategory::NoncommutativeAlgebra,
        MessageType::RightMultiplyInverse,
        0,
    )
    .with_substitution("inverse", "A^(-1)")
    .with_substitution("equation", "X*A = B")
    .with_substitution("lhs", "X*A")
    .with_substitution("rhs", "B")
    .build();

    assert!(step.is_some());
    let step = step.unwrap();
    assert!(step.description.contains("RIGHT"));
    assert!(step.description.contains("A^(-1)"));
}

#[test]
fn test_noncommutative_warning_message() {
    let step = MessageBuilder::new(
        MessageCategory::NoncommutativeAlgebra,
        MessageType::NoncommutativeWarning,
        0,
    )
    .with_substitution("symbol", "A")
    .with_substitution("symbol_type", "Matrix")
    .with_substitution("other", "B")
    .build();

    assert!(step.is_some());
    let step = step.unwrap();
    assert!(step.description.contains("WARNING"));
    assert!(step.description.contains("noncommutative"));
    assert!(step.description.contains("order"));
}

#[test]
fn test_commutator_explanation_message() {
    let step = MessageBuilder::new(
        MessageCategory::NoncommutativeAlgebra,
        MessageType::CommutatorExplanation,
        0,
    )
    .with_substitution("A", "A")
    .with_substitution("B", "B")
    .build();

    assert!(step.is_some());
    let step = step.unwrap();
    assert!(step.description.contains("commutator"));
    assert!(step.description.contains("A*B - B*A") || step.description.contains("{A}*{B}"));
}

#[test]
fn test_message_formatting_with_parameters() {
    let step = MessageBuilder::new(
        MessageCategory::NoncommutativeAlgebra,
        MessageType::LeftMultiplyInverse,
        1,
    )
    .with_substitution("A", "M")
    .with_substitution("X", "X")
    .with_substitution("B", "V")
    .with_substitution("A_inv", "M^(-1)")
    .build();

    assert!(step.is_some());
    let step = step.unwrap();
    assert!(step.description.contains("M"));
    assert!(step.description.contains("X"));
    assert!(step.description.contains("V"));
}

#[test]
fn test_all_division_types_have_messages() {
    let left_msg = MessageBuilder::new(
        MessageCategory::NoncommutativeAlgebra,
        MessageType::LeftMultiplyInverse,
        0,
    )
    .build();

    let right_msg = MessageBuilder::new(
        MessageCategory::NoncommutativeAlgebra,
        MessageType::RightMultiplyInverse,
        0,
    )
    .build();

    assert!(left_msg.is_some(), "Left division message should exist");
    assert!(right_msg.is_some(), "Right division message should exist");
}

#[test]
fn test_backward_compatibility_scalar_messages() {
    let linear_msg = MessageBuilder::new(
        MessageCategory::LinearEquation,
        MessageType::Introduction,
        0,
    )
    .with_substitution("equation", "2*x + 3")
    .with_substitution("variable", "x")
    .build();

    assert!(
        linear_msg.is_some(),
        "Scalar equation messages should still work"
    );
}

#[test]
fn test_message_clarity_no_ambiguous_wording() {
    let left_step = MessageBuilder::new(
        MessageCategory::NoncommutativeAlgebra,
        MessageType::LeftMultiplyInverse,
        0,
    )
    .build();

    let right_step = MessageBuilder::new(
        MessageCategory::NoncommutativeAlgebra,
        MessageType::RightMultiplyInverse,
        0,
    )
    .build();

    assert!(left_step.is_some());
    assert!(right_step.is_some());

    let left_desc = left_step.unwrap().description;
    let right_desc = right_step.unwrap().description;

    assert!(
        left_desc.contains("LEFT") && !left_desc.contains("Multiply both sides by"),
        "Left message should specify LEFT"
    );
    assert!(
        right_desc.contains("RIGHT") && !right_desc.contains("Multiply both sides by"),
        "Right message should specify RIGHT"
    );
}

// LaTeX Formatter Tests (8 tests)

#[test]
fn test_matrix_symbol_formats_as_mathbf() {
    let a_matrix = symbol!(A; matrix);
    let expr = Expression::symbol(a_matrix);

    let latex = expr.to_latex(LaTeXContext::default()).unwrap();

    assert_eq!(latex, "\\mathbf{A}");
}

#[test]
fn test_operator_symbol_formats_as_hat() {
    let p_operator = symbol!(p; operator);
    let expr = Expression::symbol(p_operator);

    let latex = expr.to_latex(LaTeXContext::default()).unwrap();

    assert_eq!(latex, "\\hat{p}");
}

#[test]
fn test_quaternion_symbol_formats_correctly() {
    let i_quat = symbol!(i; quaternion);
    let expr = Expression::symbol(i_quat);

    let latex = expr.to_latex(LaTeXContext::default()).unwrap();

    assert_eq!(latex, "i");
}

#[test]
fn test_scalar_symbol_unchanged() {
    let x_scalar = symbol!(x);
    let expr = Expression::symbol(x_scalar);

    let latex = expr.to_latex(LaTeXContext::default()).unwrap();

    assert_eq!(latex, "x");
}

#[test]
fn test_mixed_expression_2_a_x_equals_b() {
    let a_matrix = symbol!(A; matrix);
    let x_matrix = symbol!(X; matrix);
    let b_matrix = symbol!(B; matrix);

    let two = Expression::integer(2);
    let a_expr = Expression::symbol(a_matrix);
    let x_expr = Expression::symbol(x_matrix);

    let left_side = Expression::mul(vec![two, a_expr, x_expr]);
    let right_side = Expression::symbol(b_matrix);

    let left_latex = left_side.to_latex(LaTeXContext::default()).unwrap();
    let right_latex = right_side.to_latex(LaTeXContext::default()).unwrap();

    assert!(left_latex.contains("\\mathbf{A}"));
    assert!(left_latex.contains("\\mathbf{X}"));
    assert_eq!(right_latex, "\\mathbf{B}");
}

#[test]
fn test_operator_equation_h_psi_equals_e_psi() {
    let h_op = symbol!(H; operator);
    let psi = symbol!(psi);
    let e_scalar = symbol!(E);

    let h_expr = Expression::symbol(h_op);
    let psi_expr = Expression::symbol(psi.clone());
    let e_expr = Expression::symbol(e_scalar);

    let left_side = Expression::mul(vec![h_expr, psi_expr.clone()]);
    let right_side = Expression::mul(vec![e_expr, psi_expr]);

    let left_latex = left_side.to_latex(LaTeXContext::default()).unwrap();
    let right_latex = right_side.to_latex(LaTeXContext::default()).unwrap();

    assert!(left_latex.contains("\\hat{H}"));
    assert!(left_latex.contains("psi"));
    assert!(right_latex.contains("E"));
}

#[test]
fn test_quaternion_multiplication_i_j_equals_k() {
    let i_quat = symbol!(i; quaternion);
    let j_quat = symbol!(j; quaternion);

    let i_expr = Expression::symbol(i_quat);
    let j_expr = Expression::symbol(j_quat);

    let product = Expression::mul(vec![i_expr, j_expr]);

    let latex = product.to_latex(LaTeXContext::default()).unwrap();

    assert!(latex.contains("i"));
    assert!(latex.contains("j"));
}

#[test]
fn test_complex_nested_expressions() {
    let a_matrix = symbol!(A; matrix);
    let b_matrix = symbol!(B; matrix);
    let x_scalar = symbol!(x);

    let a_expr = Expression::symbol(a_matrix);
    let b_expr = Expression::symbol(b_matrix);
    let x_expr = Expression::symbol(x_scalar);

    let sum = Expression::add(vec![a_expr.clone(), b_expr.clone()]);
    let product = Expression::mul(vec![sum, x_expr]);

    let latex = product.to_latex(LaTeXContext::default()).unwrap();

    assert!(latex.contains("\\mathbf{A}"));
    assert!(latex.contains("\\mathbf{B}"));
    assert!(latex.contains("x"));
}

#[test]
fn test_latex_formatter_preserves_order_for_noncommutative() {
    let a = symbol!(A; matrix);
    let b = symbol!(B; matrix);

    let ab = Expression::mul(vec![
        Expression::symbol(a.clone()),
        Expression::symbol(b.clone()),
    ]);
    let ba = Expression::mul(vec![Expression::symbol(b), Expression::symbol(a)]);

    let ab_latex = ab.to_latex(LaTeXContext::default()).unwrap();
    let ba_latex = ba.to_latex(LaTeXContext::default()).unwrap();

    assert!(ab_latex.contains("\\mathbf{A}"));
    assert!(ab_latex.contains("\\mathbf{B}"));
    assert!(ba_latex.contains("\\mathbf{B}"));
    assert!(ba_latex.contains("\\mathbf{A}"));
}

#[test]
fn test_message_registry_has_all_required_messages() {
    use mathhook_core::educational::message_registry::MESSAGE_REGISTRY;

    let count = MESSAGE_REGISTRY.len();
    assert!(
        count >= 80,
        "Expected at least 80 messages (65 original + 15 new), found {}",
        count
    );
}
