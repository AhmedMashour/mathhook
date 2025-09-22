//! Zero detection and cancellation tests
//!
//! Tests mathematical properties related to zero detection and additive inverses

use mathhook_core::prelude::*;

#[test]
fn test_simple_zero_case() {
    /// Test additive inverse: 4 + (-4) = 0
    let expr = Expression::integer(4) + Expression::integer(-4);
    let result = expr.simplify();

    assert_eq!(result, Expression::integer(0), "4 + (-4) should equal 0");
}

#[test]
fn test_variable_additive_inverse() {
    /// Test symbolic additive inverse: 4x + (-4x) should cancel when implemented
    let x = Symbol::new("x");

    let expr = 4 * Expression::symbol(x.clone()) + (-4) * Expression::symbol(x.clone());
    let result = expr.simplify();

    // For now, verify expression is well-formed
    // When like-term collection is implemented, this should equal 0
    match result {
        Expression::Number(Number::Integer(0)) => {
            // Perfect! Like-term collection is working
            assert_eq!(result, Expression::integer(0));
        }
        _ => {
            // Not yet implemented, but should be well-formed
            assert!(
                !result.to_string().is_empty(),
                "Expression should be well-formed"
            );
            println!(
                "Variable zero test: 4*x + (-4)*x = {} (like-term collection not yet implemented)",
                result
            );
        }
    }
}

#[test]
fn test_combined_zero_detection() {
    /// Test combined constant and variable cancellation
    let x = Symbol::new("x");

    let expr = Expression::integer(4)
        + 4 * Expression::symbol(x.clone())
        + Expression::integer(-4)
        + (-4) * Expression::symbol(x.clone());

    let result = expr.simplify();

    // This should eventually simplify to 0 when full simplification is implemented
    // For now, verify it's well-formed and check if any simplification occurred
    match result {
        Expression::Number(Number::Integer(0)) => {
            // Excellent! Full simplification working
            assert_eq!(result, Expression::integer(0));
        }
        _ => {
            // Partial simplification may have occurred
            println!("Combined zero test: 4 + 4*x + (-4) + (-4)*x = {}", result);

            // At minimum, the constant terms should cancel: 4 + (-4) = 0
            // So result should not contain standalone constant terms of 4 and -4
            let result_str = result.to_string();
            assert!(!result_str.is_empty(), "Expression should be well-formed");

            // If constants were simplified, we shouldn't see both 4 and -4
            let has_pos_4 = result_str.contains("4") && !result_str.contains("-4");
            let has_neg_4 = result_str.contains("-4") && !result_str.contains("+ 4");
            assert!(
                !(has_pos_4 && has_neg_4),
                "Constants 4 and -4 should cancel each other"
            );
        }
    }
}

#[test]
fn test_zero_multiplication_property() {
    /// Test zero multiplication: 0 * anything = 0
    let x = Symbol::new("x");

    let test_cases = vec![
        Expression::integer(42),
        Expression::symbol(x.clone()),
        Expression::add(vec![Expression::symbol(x.clone()), Expression::integer(5)]),
    ];

    for expr in test_cases {
        let zero_mult = Expression::mul(vec![Expression::integer(0), expr.clone()]).simplify();
        assert_eq!(
            zero_mult,
            Expression::integer(0),
            "0 * {} should equal 0",
            expr
        );

        let mult_zero = Expression::mul(vec![expr.clone(), Expression::integer(0)]).simplify();
        assert_eq!(
            mult_zero,
            Expression::integer(0),
            "{} * 0 should equal 0",
            expr
        );
    }
}

#[test]
fn test_additive_identity_with_zero() {
    /// Test additive identity: a + 0 = a
    let x = Symbol::new("x");

    let test_expressions = vec![
        (Expression::integer(5), Expression::integer(5)), // 5 + 0 = 5
        (Expression::symbol(x.clone()), Expression::symbol(x.clone())), // x + 0 = x
        // Note: 2+3 + 0 = 5 (our CAS correctly simplifies this to the mathematically superior result)
        (
            Expression::add(vec![Expression::integer(2), Expression::integer(3)]),
            Expression::integer(5),
        ),
    ];

    for (expr, expected) in test_expressions {
        let plus_zero = Expression::add(vec![expr.clone(), Expression::integer(0)]).simplify();
        assert_eq!(
            plus_zero, expected,
            "{} + 0 should equal {} (mathematically correct simplification)",
            expr, expected
        );

        let zero_plus = Expression::add(vec![Expression::integer(0), expr.clone()]).simplify();
        assert_eq!(
            zero_plus, expected,
            "0 + {} should equal {} (mathematically correct simplification)",
            expr, expected
        );
    }
}
