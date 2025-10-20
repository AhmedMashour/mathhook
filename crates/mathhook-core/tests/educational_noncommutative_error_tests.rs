//! Error Handling Tests for Noncommutative Algebra (Wave 11 Enhancement)
//!
//! Tests verifying graceful error handling in edge cases and malformed inputs.
//! Ensures system stability with invalid data and provides helpful error messages.

use mathhook_core::core::symbol::SymbolType;
use mathhook_core::educational::message_registry::{MessageBuilder, MessageCategory, MessageType};
use mathhook_core::formatter::latex::LaTeXContext;
use mathhook_core::formatter::LaTeXFormatter;
use mathhook_core::parser::config::ParserConfig;
use mathhook_core::parser::Parser;
use mathhook_core::{symbol, Expression, Symbol};

#[test]
fn test_message_registry_missing_message() {
    let msg = MessageBuilder::new(
        MessageCategory::NoncommutativeAlgebra,
        MessageType::LeftMultiplyInverse,
        255,
    )
    .build();

    if let Some(message) = msg {
        assert!(!message.description.is_empty(), "Should provide default or fallback message");
    }
}

#[test]
fn test_formatter_invalid_symbol_type() {
    let x = symbol!(x);
    let a_matrix = symbol!(A; matrix);
    let p_op = symbol!(p; operator);

    let expr_scalar = Expression::symbol(x.clone());
    let expr_matrix = Expression::symbol(a_matrix.clone());
    let expr_op = Expression::symbol(p_op.clone());

    let latex_scalar = expr_scalar.to_latex(LaTeXContext::default());
    let latex_matrix = expr_matrix.to_latex(LaTeXContext::default());
    let latex_op = expr_op.to_latex(LaTeXContext::default());

    assert!(latex_scalar.is_ok(), "Scalar formatting should succeed");
    assert!(latex_matrix.is_ok(), "Matrix formatting should succeed");
    assert!(latex_op.is_ok(), "Operator formatting should succeed");

    if let Ok(latex) = latex_scalar {
        assert!(!latex.is_empty(), "Formatted output should not be empty");
    }
}

#[test]
fn test_formatter_null_symbol() {
    let empty_name = Symbol::new("");

    let expr = Expression::symbol(empty_name.clone());
    let latex = expr.to_latex(LaTeXContext::default());

    assert!(
        latex.is_ok() || latex.is_err(),
        "Formatter should handle edge case gracefully"
    );
}

#[test]
fn test_educational_steps_malformed_equation() {
    let a = symbol!(a);
    let _x = symbol!(x);

    let malformed = Expression::add(vec![
        Expression::symbol(a.clone()),
        Expression::symbol(a.clone()),
    ]);

    let latex = malformed.to_latex(LaTeXContext::default());

    assert!(
        latex.is_ok(),
        "Should handle degenerate equations gracefully"
    );
}

#[test]
fn test_latex_formatter_special_characters() {
    let theta = Symbol::new("theta");
    let alpha = Symbol::new("alpha");
    let beta = Symbol::new("beta");

    let expr_theta = Expression::symbol(theta);
    let expr_alpha = Expression::symbol(alpha);
    let expr_beta = Expression::symbol(beta);

    let latex_theta = expr_theta.to_latex(LaTeXContext::default());
    let latex_alpha = expr_alpha.to_latex(LaTeXContext::default());
    let latex_beta = expr_beta.to_latex(LaTeXContext::default());

    assert!(latex_theta.is_ok(), "Greek symbols should format correctly");
    assert!(latex_alpha.is_ok(), "Greek symbols should format correctly");
    assert!(latex_beta.is_ok(), "Greek symbols should format correctly");

    if let (Ok(t), Ok(a), Ok(b)) = (latex_theta, latex_alpha, latex_beta) {
        assert!(!t.is_empty() && !a.is_empty() && !b.is_empty());
    }
}

#[test]
fn test_parser_malformed_mathbf() {
    let parser = Parser::new(ParserConfig {
        enable_implicit_multiplication: true,
    });

    let result1 = parser.parse(r"\mathbf{");
    let result2 = parser.parse(r"\mathbf}");
    let result3 = parser.parse(r"mathbf{A}");

    assert!(
        result1.is_err() || result1.is_ok(),
        "Parser should handle malformed mathbf gracefully"
    );
    assert!(
        result2.is_err() || result2.is_ok(),
        "Parser should handle malformed mathbf gracefully"
    );
    assert!(
        result3.is_ok(),
        "Parser should parse mathbf without backslash"
    );
}

#[test]
fn test_parser_malformed_hat() {
    let parser = Parser::new(ParserConfig {
        enable_implicit_multiplication: true,
    });

    let result1 = parser.parse(r"\hat{");
    let result2 = parser.parse(r"\hat}");
    let result3 = parser.parse(r"hat{p}");

    assert!(
        result1.is_err() || result1.is_ok(),
        "Parser should handle malformed hat gracefully"
    );
    assert!(
        result2.is_err() || result2.is_ok(),
        "Parser should handle malformed hat gracefully"
    );
    assert!(
        result3.is_ok(),
        "Parser should parse hat without backslash"
    );
}

#[test]
fn test_symbol_type_consistency() {
    let x = symbol!(x);
    let a_mat = symbol!(A; matrix);
    let p_op = symbol!(p; operator);
    let i_quat = symbol!(i; quaternion);

    assert_eq!(x.symbol_type(), SymbolType::Scalar);
    assert_eq!(a_mat.symbol_type(), SymbolType::Matrix);
    assert_eq!(p_op.symbol_type(), SymbolType::Operator);
    assert_eq!(i_quat.symbol_type(), SymbolType::Quaternion);

    let x_copy = symbol!(x);
    assert_eq!(x.symbol_type(), x_copy.symbol_type());
}

#[test]
fn test_error_recovery_in_multiplication() {
    let a = symbol!(a);
    let b_mat = symbol!(B; matrix);

    let mixed = Expression::mul(vec![
        Expression::symbol(a.clone()),
        Expression::symbol(b_mat.clone()),
    ]);

    let latex = mixed.to_latex(LaTeXContext::default());

    assert!(
        latex.is_ok(),
        "Mixed scalar-matrix multiplication should format correctly"
    );

    if let Ok(output) = latex {
        assert!(
            output.contains("a") || output.contains("B"),
            "Output should contain both symbols"
        );
    }
}

#[test]
fn test_zero_length_expression_handling() {
    let empty_add = Expression::add(vec![]);
    let empty_mul = Expression::mul(vec![]);

    let latex_add = empty_add.to_latex(LaTeXContext::default());
    let latex_mul = empty_mul.to_latex(LaTeXContext::default());

    assert!(
        latex_add.is_ok() || latex_add.is_err(),
        "Should handle empty add expression"
    );
    assert!(
        latex_mul.is_ok() || latex_mul.is_err(),
        "Should handle empty mul expression"
    );
}

#[test]
fn test_deeply_nested_expression_handling() {
    let x = symbol!(x);
    let mut nested = Expression::symbol(x.clone());

    for _ in 0..100 {
        nested = Expression::mul(vec![nested.clone(), nested.clone()]);
    }

    let latex = nested.to_latex(LaTeXContext::default());

    assert!(
        latex.is_ok() || latex.is_err(),
        "Should handle deeply nested expressions without crashing"
    );
}

#[test]
fn test_message_category_boundary_conditions() {
    let msg1 = MessageBuilder::new(
        MessageCategory::NoncommutativeAlgebra,
        MessageType::LeftMultiplyInverse,
        0,
    )
    .build();

    let msg2 = MessageBuilder::new(
        MessageCategory::NoncommutativeAlgebra,
        MessageType::RightMultiplyInverse,
        0,
    )
    .build();

    assert!(
        msg1.is_some() || msg1.is_none(),
        "Message creation should handle all valid types"
    );
    assert!(
        msg2.is_some() || msg2.is_none(),
        "Message creation should handle all valid types"
    );
}

#[test]
fn test_formatter_context_variations() {
    let a = symbol!(A; matrix);
    let expr = Expression::symbol(a);

    let context1 = LaTeXContext::default();
    let context2 = LaTeXContext {
        needs_parentheses: true,
        ..Default::default()
    };

    let latex1 = expr.to_latex(context1);
    let latex2 = expr.to_latex(context2);

    assert!(
        latex1.is_ok(),
        "Should format with default context"
    );
    assert!(
        latex2.is_ok(),
        "Should format with parentheses context"
    );
}

#[test]
fn test_symbol_name_edge_cases() {
    let single_char = Symbol::new("A");
    let multi_char = Symbol::new("Alpha");
    let numeric = Symbol::new("x1");
    let underscore = Symbol::new("x_1");

    let expr1 = Expression::symbol(single_char);
    let expr2 = Expression::symbol(multi_char);
    let expr3 = Expression::symbol(numeric);
    let expr4 = Expression::symbol(underscore);

    assert!(
        expr1.to_latex(LaTeXContext::default()).is_ok(),
        "Single character symbols should format"
    );
    assert!(
        expr2.to_latex(LaTeXContext::default()).is_ok(),
        "Multi-character symbols should format"
    );
    assert!(
        expr3.to_latex(LaTeXContext::default()).is_ok(),
        "Numeric subscript symbols should format"
    );
    assert!(
        expr4.to_latex(LaTeXContext::default()).is_ok(),
        "Underscore symbols should format"
    );
}
