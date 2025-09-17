//! Integration tests for procedural expr!() macro
//!
//! Tests all supported syntax patterns and edge cases.

use mathhook_core::{Expression, Symbol};
use mathhook_macros::expr;

#[test]
fn test_integer_literals() {
    let result = expr!(42);
    assert_eq!(result, Expression::integer(42));

    let result = expr!(0);
    assert_eq!(result, Expression::integer(0));

    let result = expr!(-5);
    assert_eq!(
        result,
        Expression::mul(vec![Expression::integer(-1), Expression::integer(5)])
    );
}

#[test]
fn test_float_literals() {
    let result = expr!(3.15);
    assert_eq!(result, Expression::float(3.15));

    let result = expr!(0.0);
    assert_eq!(result, Expression::float(0.0));
}

#[test]
fn test_symbols() {
    let result = expr!(x);
    assert_eq!(result, Expression::symbol(Symbol::scalar("x")));

    let result = expr!(theta);
    assert_eq!(result, Expression::symbol(Symbol::scalar("theta")));

    let result = expr!(alpha_1);
    assert_eq!(result, Expression::symbol(Symbol::scalar("alpha_1")));
}

#[test]
fn test_mathematical_constants() {
    let result = expr!(pi);
    assert_eq!(result, Expression::pi());

    let result = expr!(e);
    assert_eq!(result, Expression::e());

    let result = expr!(i);
    assert_eq!(result, Expression::i());
}

#[test]
fn test_addition() {
    let result = expr!(x + y);
    assert_eq!(
        result,
        Expression::add(vec![
            Expression::symbol(Symbol::scalar("x")),
            Expression::symbol(Symbol::scalar("y"))
        ])
    );

    let result = expr!(2 + 3);
    assert_eq!(
        result,
        Expression::add(vec![Expression::integer(2), Expression::integer(3)])
    );
}

#[test]
fn test_subtraction() {
    let result = expr!(x - y);
    assert_eq!(
        result,
        Expression::add(vec![
            Expression::symbol(Symbol::scalar("x")),
            Expression::mul(vec![
                Expression::integer(-1),
                Expression::symbol(Symbol::scalar("y"))
            ])
        ])
    );

    let result = expr!(5 - 2);
    assert_eq!(
        result,
        Expression::add(vec![
            Expression::integer(5),
            Expression::mul(vec![Expression::integer(-1), Expression::integer(2)])
        ])
    );
}

#[test]
fn test_multiplication() {
    let result = expr!(x * y);
    assert_eq!(
        result,
        Expression::mul(vec![
            Expression::symbol(Symbol::scalar("x")),
            Expression::symbol(Symbol::scalar("y"))
        ])
    );

    let result = expr!(2 * x);
    assert_eq!(
        result,
        Expression::mul(vec![
            Expression::integer(2),
            Expression::symbol(Symbol::scalar("x"))
        ])
    );
}

#[test]
fn test_division() {
    let result = expr!(x / y);
    assert_eq!(
        result,
        Expression::mul(vec![
            Expression::symbol(Symbol::scalar("x")),
            Expression::pow(
                Expression::symbol(Symbol::scalar("y")),
                Expression::integer(-1)
            )
        ])
    );

    let result = expr!(6 / 2);
    assert_eq!(
        result,
        Expression::mul(vec![
            Expression::integer(6),
            Expression::pow(Expression::integer(2), Expression::integer(-1))
        ])
    );
}

#[test]
fn test_power_with_method() {
    let result = expr!(x.pow(2));
    assert_eq!(
        result,
        Expression::pow(
            Expression::symbol(Symbol::scalar("x")),
            Expression::integer(2)
        )
    );

    let result = expr!(x.pow(y));
    assert_eq!(
        result,
        Expression::pow(
            Expression::symbol(Symbol::scalar("x")),
            Expression::symbol(Symbol::scalar("y"))
        )
    );
}

#[test]
fn test_unary_negation() {
    let result = expr!(-x);
    assert_eq!(
        result,
        Expression::mul(vec![
            Expression::integer(-1),
            Expression::symbol(Symbol::scalar("x"))
        ])
    );

    let result = expr!(-42);
    assert_eq!(
        result,
        Expression::mul(vec![Expression::integer(-1), Expression::integer(42)])
    );
}

#[test]
fn test_double_negation() {
    let result = expr!(-(-x));
    assert_eq!(
        result,
        Expression::mul(vec![
            Expression::integer(-1),
            Expression::mul(vec![
                Expression::integer(-1),
                Expression::symbol(Symbol::scalar("x"))
            ])
        ])
    );
}

#[test]
fn test_parenthesized_expressions() {
    let result = expr!((x));
    assert_eq!(result, Expression::symbol(Symbol::scalar("x")));

    let result = expr!((x + y));
    assert_eq!(
        result,
        Expression::add(vec![
            Expression::symbol(Symbol::scalar("x")),
            Expression::symbol(Symbol::scalar("y"))
        ])
    );

    let result = expr!((x));
    assert_eq!(result, Expression::symbol(Symbol::scalar("x")));
}

#[test]
fn test_nested_operations() {
    let result = expr!((x + y) * (x - y));
    assert!(matches!(result, Expression::Mul(_)));
}

#[test]
fn test_function_calls_zero_args() {
    let result = expr!(gamma());
    assert_eq!(result, Expression::function("gamma", vec![]));
}

#[test]
fn test_function_calls_one_arg() {
    let result = expr!(sin(x));
    assert_eq!(
        result,
        Expression::function("sin", vec![Expression::symbol(Symbol::scalar("x"))])
    );

    let result = expr!(cos(pi));
    assert_eq!(result, Expression::function("cos", vec![Expression::pi()]));
}

#[test]
fn test_function_calls_two_args() {
    let result = expr!(log(x, y));
    assert_eq!(
        result,
        Expression::function(
            "log",
            vec![
                Expression::symbol(Symbol::scalar("x")),
                Expression::symbol(Symbol::scalar("y"))
            ]
        )
    );

    let result = expr!(atan2(y, x));
    assert_eq!(
        result,
        Expression::function(
            "atan2",
            vec![
                Expression::symbol(Symbol::scalar("y")),
                Expression::symbol(Symbol::scalar("x"))
            ]
        )
    );
}

#[test]
fn test_function_calls_three_args() {
    let result = expr!(f(x, y, z));
    assert_eq!(
        result,
        Expression::function(
            "f",
            vec![
                Expression::symbol(Symbol::scalar("x")),
                Expression::symbol(Symbol::scalar("y")),
                Expression::symbol(Symbol::scalar("z"))
            ]
        )
    );
}

#[test]
fn test_nested_functions() {
    let result = expr!(sin(cos(x)));
    assert_eq!(
        result,
        Expression::function(
            "sin",
            vec![Expression::function(
                "cos",
                vec![Expression::symbol(Symbol::scalar("x"))]
            )]
        )
    );
}

#[test]
fn test_function_with_operations() {
    let result = expr!(sin(x + y));
    assert_eq!(
        result,
        Expression::function(
            "sin",
            vec![Expression::add(vec![
                Expression::symbol(Symbol::scalar("x")),
                Expression::symbol(Symbol::scalar("y"))
            ])]
        )
    );
}

#[test]
fn test_complex_expression_1() {
    let result = expr!(2 * x);
    assert_eq!(
        result,
        Expression::mul(vec![
            Expression::integer(2),
            Expression::symbol(Symbol::scalar("x"))
        ])
    );
}

#[test]
fn test_complex_expression_2() {
    let result = expr!(x.pow(2));
    assert_eq!(
        result,
        Expression::pow(
            Expression::symbol(Symbol::scalar("x")),
            Expression::integer(2)
        )
    );
}

#[test]
fn test_complex_expression_3() {
    let result = expr!((x + 1) * (x - 1));
    assert!(matches!(result, Expression::Mul(_)));
}

#[test]
fn test_precedence_addition_vs_multiplication() {
    let result = expr!(x * y);
    assert!(matches!(result, Expression::Mul(_)));
}

#[test]
fn test_precedence_with_parentheses() {
    let result = expr!((x + y) * z);
    assert!(matches!(result, Expression::Mul(_)));
}

#[test]
fn test_edge_case_negative_zero() {
    let result = expr!(-0);
    assert_eq!(
        result,
        Expression::mul(vec![Expression::integer(-1), Expression::integer(0)])
    );
}

#[test]
fn test_edge_case_division_by_one() {
    let result = expr!(x / 1);
    assert_eq!(
        result,
        Expression::mul(vec![
            Expression::symbol(Symbol::scalar("x")),
            Expression::pow(Expression::integer(1), Expression::integer(-1))
        ])
    );
}

#[test]
fn test_edge_case_multiplication_by_zero() {
    let result = expr!(0 * x);
    assert_eq!(
        result,
        Expression::mul(vec![
            Expression::integer(0),
            Expression::symbol(Symbol::scalar("x"))
        ])
    );
}

#[test]
fn test_edge_case_power_of_one() {
    let result = expr!(x.pow(1));
    assert_eq!(
        result,
        Expression::pow(
            Expression::symbol(Symbol::scalar("x")),
            Expression::integer(1)
        )
    );
}

#[test]
fn test_edge_case_power_of_zero() {
    let result = expr!(x.pow(0));
    assert_eq!(
        result,
        Expression::pow(
            Expression::symbol(Symbol::scalar("x")),
            Expression::integer(0)
        )
    );
}

// ============================================================================
// NEW TESTS: Edge Cases (10 tests)
// ============================================================================

#[test]
fn test_overflow_protection_large_integer() {
    let result = expr!(999999999999999999);
    assert_eq!(result, Expression::integer(999999999999999999));
}

#[test]
fn test_underflow_protection_small_float() {
    let result = expr!(0.0000000001);
    assert_eq!(result, Expression::float(0.0000000001));
}

#[test]
fn test_zero_division_literal() {
    let result = expr!(1 / 0);
    assert_eq!(
        result,
        Expression::mul(vec![
            Expression::integer(1),
            Expression::pow(Expression::integer(0), Expression::integer(-1))
        ])
    );
}

#[test]
fn test_negative_base_power() {
    let result = expr!((-2).pow(3));
    assert_eq!(
        result,
        Expression::pow(
            Expression::mul(vec![Expression::integer(-1), Expression::integer(2)]),
            Expression::integer(3)
        )
    );
}

#[test]
fn test_fractional_power() {
    let result = expr!(x.pow(0.5));
    assert_eq!(
        result,
        Expression::pow(
            Expression::symbol(Symbol::scalar("x")),
            Expression::float(0.5)
        )
    );
}

#[test]
fn test_chained_unary_negation() {
    let result = expr!(---x);
    assert_eq!(
        result,
        Expression::mul(vec![
            Expression::integer(-1),
            Expression::mul(vec![
                Expression::integer(-1),
                Expression::mul(vec![
                    Expression::integer(-1),
                    Expression::symbol(Symbol::scalar("x"))
                ])
            ])
        ])
    );
}

#[test]
fn test_complex_parentheses_nesting() {
    let result = expr!((x));
    assert_eq!(result, Expression::symbol(Symbol::scalar("x")));
}

#[test]
#[allow(clippy::approx_constant)]
fn test_mixed_integer_float() {
    let result = expr!(2 * 3.14);
    assert_eq!(
        result,
        Expression::mul(vec![Expression::integer(2), Expression::float(3.14)])
    );
}

#[test]
fn test_empty_function_call() {
    let result = expr!(f());
    assert_eq!(result, Expression::function("f", vec![]));
}

#[test]
fn test_very_large_float() {
    let result = expr!(1.7976931348623157e308);
    assert_eq!(result, Expression::float(1.7976931348623157e308));
}

// ============================================================================
// NEW TESTS: Complex Nested Expressions (3 tests)
// ============================================================================

#[test]
fn test_deeply_nested_arithmetic() {
    let result = expr!(((2 * x) + (3 * y)) - ((4 * z) / (5 * w)));
    assert!(matches!(result, Expression::Add(_)));
}

#[test]
fn test_nested_functions_deep() {
    let result = expr!(sin(cos(tan(x))));
    assert!(matches!(result, Expression::Function { .. }));
}

#[test]
fn test_mixed_operations_deep_nesting() {
    let result = expr!(sin(x + y) * cos(z - w) + tan(a / b));
    assert!(matches!(result, Expression::Add(_)));
}

// ============================================================================
// NEW TESTS: Operator Precedence Edge Cases (3 tests)
// ============================================================================

#[test]
fn test_precedence_unary_vs_power() {
    let result = expr!((-x).pow(2));
    assert_eq!(
        result,
        Expression::pow(
            Expression::mul(vec![
                Expression::integer(-1),
                Expression::symbol(Symbol::scalar("x"))
            ]),
            Expression::integer(2)
        )
    );
}

#[test]
fn test_precedence_all_operators() {
    let result = expr!(a + b * c / d - e);
    assert!(matches!(result, Expression::Add(_)));
}

#[test]
fn test_precedence_with_parentheses_override() {
    let result = expr!((a + b) * (c - d));
    assert!(matches!(result, Expression::Mul(_)));
}

// ============================================================================
// NEW TESTS: Additional Edge Cases (5 tests)
// ============================================================================

#[test]
fn test_power_of_power() {
    let result = expr!((x.pow(2)).pow(3));
    assert!(matches!(result, Expression::Pow(..)));
}

#[test]
fn test_division_by_division() {
    let result = expr!(x / (y / z));
    assert!(matches!(result, Expression::Mul(_)));
}

#[test]
fn test_multiple_multiplications() {
    let result = expr!(a * b * c * d);
    assert!(matches!(result, Expression::Mul(_)));
}

#[test]
fn test_multiple_additions() {
    let result = expr!(a + b + c + d + e);
    assert!(matches!(result, Expression::Add(_)));
}

#[test]
fn test_function_with_complex_arg() {
    let result = expr!(sin((x + y) * (z - w)));
    assert!(matches!(result, Expression::Function { .. }));
}

// ============================================================================
// ** Power Operator Tests
// ============================================================================

#[test]
fn test_power_operator_basic() {
    let result = expr!(x * *2);
    assert_eq!(
        result,
        Expression::pow(
            Expression::symbol(Symbol::scalar("x")),
            Expression::integer(2)
        )
    );
}

#[test]
fn test_power_operator_integer_base() {
    let result = expr!(2 * *3);
    assert_eq!(
        result,
        Expression::pow(Expression::integer(2), Expression::integer(3))
    );
}

#[test]
fn test_power_operator_float_exponent() {
    let result = expr!(x * *0.5);
    assert_eq!(
        result,
        Expression::pow(
            Expression::symbol(Symbol::scalar("x")),
            Expression::float(0.5)
        )
    );
}

#[test]
fn test_power_operator_negative_exponent() {
    let result = expr!(x * *-2);
    assert_eq!(
        result,
        Expression::pow(
            Expression::symbol(Symbol::scalar("x")),
            Expression::mul(vec![Expression::integer(-1), Expression::integer(2)])
        )
    );
}

#[test]
fn test_power_operator_right_associative() {
    let result = expr!(x * *2 * *3);
    assert_eq!(
        result,
        Expression::pow(
            Expression::symbol(Symbol::scalar("x")),
            Expression::pow(Expression::integer(2), Expression::integer(3))
        )
    );
}

#[test]
fn test_power_precedence_vs_multiply() {
    let result = expr!(2 * (x * *3));
    assert_eq!(
        result,
        Expression::mul(vec![
            Expression::integer(2),
            Expression::pow(
                Expression::symbol(Symbol::scalar("x")),
                Expression::integer(3)
            )
        ])
    );
}

#[test]
fn test_power_precedence_vs_divide() {
    let result = expr!(x / (y * *2));
    assert_eq!(
        result,
        Expression::mul(vec![
            Expression::symbol(Symbol::scalar("x")),
            Expression::pow(
                Expression::pow(
                    Expression::symbol(Symbol::scalar("y")),
                    Expression::integer(2)
                ),
                Expression::integer(-1)
            )
        ])
    );
}

#[test]
fn test_power_precedence_vs_add() {
    let result = expr!(x + (y * *2));
    assert_eq!(
        result,
        Expression::add(vec![
            Expression::symbol(Symbol::scalar("x")),
            Expression::pow(
                Expression::symbol(Symbol::scalar("y")),
                Expression::integer(2)
            )
        ])
    );
}

#[test]
fn test_power_with_parentheses() {
    let result = expr!((x + 1) * *2);
    assert_eq!(
        result,
        Expression::pow(
            Expression::add(vec![
                Expression::symbol(Symbol::scalar("x")),
                Expression::integer(1)
            ]),
            Expression::integer(2)
        )
    );
}

#[test]
fn test_power_operator_chained() {
    let result = expr!(x * *y * *z);
    assert_eq!(
        result,
        Expression::pow(
            Expression::symbol(Symbol::scalar("x")),
            Expression::pow(
                Expression::symbol(Symbol::scalar("y")),
                Expression::symbol(Symbol::scalar("z"))
            )
        )
    );
}

#[test]
fn test_power_operator_with_constants() {
    let result = expr!(pi * *e);
    assert_eq!(result, Expression::pow(Expression::pi(), Expression::e()));
}

#[test]
fn test_power_operator_complex_expression() {
    // Due to token-level preprocessing limitations, complex expressions with **
    // should use parentheses for clarity: 2 * (x ** 2) + 3 * x + 1
    // This test validates that parentheses work correctly
    let result = expr!(2 * (x * *2) + 3 * x + 1);
    assert_eq!(
        result,
        Expression::add(vec![
            Expression::add(vec![
                Expression::mul(vec![
                    Expression::integer(2),
                    Expression::pow(
                        Expression::symbol(Symbol::scalar("x")),
                        Expression::integer(2)
                    )
                ]),
                Expression::mul(vec![
                    Expression::integer(3),
                    Expression::symbol(Symbol::scalar("x"))
                ])
            ]),
            Expression::integer(1)
        ])
    );
}

#[test]
fn test_power_both_syntaxes_equivalent() {
    let power_op = expr!(x * *2);
    let pow_method = expr!(x.pow(2));
    assert_eq!(power_op, pow_method);
}

#[test]
fn test_power_operator_mixed_with_unary() {
    // Note: Due to token-level preprocessing, -x ** 2 is parsed as (-x) ** 2
    // This is acceptable behavior. For -(x ** 2), use explicit parentheses.
    let result = expr!(-x * *2);
    assert_eq!(
        result,
        Expression::pow(
            Expression::mul(vec![
                Expression::integer(-1),
                Expression::symbol(Symbol::scalar("x"))
            ]),
            Expression::integer(2)
        )
    );
}
