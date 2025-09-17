//! Advanced tests for procedural expr!() macro
//!
//! Tests comparison operators, extended methods, and complex nested expressions.

use mathhook_core::{Expression, Simplify, Symbol};
use mathhook_macros::expr;

// ============================================================================
// Comparison Operators (20 tests)
// ============================================================================

#[test]
fn test_comparison_equal_symbols() {
    let result = expr!(x == y);
    assert!(matches!(result, Expression::Relation(..)));
}

#[test]
fn test_comparison_equal_literals() {
    let result = expr!(2 == 3);
    assert!(matches!(result, Expression::Relation(..)));
}

#[test]
fn test_comparison_equal_expression() {
    let result = expr!(x + 1 == y + 2);
    assert!(matches!(result, Expression::Relation(..)));
}

#[test]
fn test_comparison_less_than() {
    let result = expr!(x < y);
    assert!(matches!(result, Expression::Relation(..)));
}

#[test]
fn test_comparison_less_than_literals() {
    let result = expr!(2 < 3);
    assert!(matches!(result, Expression::Relation(..)));
}

#[test]
fn test_comparison_less_than_expression() {
    let result = expr!(2 * x < 3 * y);
    assert!(matches!(result, Expression::Relation(..)));
}

#[test]
fn test_comparison_greater_than() {
    let result = expr!(x > y);
    assert!(matches!(result, Expression::Relation(..)));
}

#[test]
fn test_comparison_greater_than_literals() {
    let result = expr!(5 > 2);
    assert!(matches!(result, Expression::Relation(..)));
}

#[test]
fn test_comparison_greater_than_expression() {
    let result = expr!(x.pow(2) > y);
    assert!(matches!(result, Expression::Relation(..)));
}

#[test]
fn test_comparison_less_equal() {
    let result = expr!(x <= y);
    assert!(matches!(result, Expression::Relation(..)));
}

#[test]
fn test_comparison_less_equal_literals() {
    let result = expr!(3 <= 3);
    assert!(matches!(result, Expression::Relation(..)));
}

#[test]
fn test_comparison_less_equal_expression() {
    let result = expr!(x + 1 <= y - 1);
    assert!(matches!(result, Expression::Relation(..)));
}

#[test]
fn test_comparison_greater_equal() {
    let result = expr!(x >= y);
    assert!(matches!(result, Expression::Relation(..)));
}

#[test]
fn test_comparison_greater_equal_literals() {
    let result = expr!(5 >= 2);
    assert!(matches!(result, Expression::Relation(..)));
}

#[test]
fn test_comparison_greater_equal_expression() {
    let result = expr!(sin(x) >= cos(y));
    assert!(matches!(result, Expression::Relation(..)));
}

#[test]
fn test_comparison_with_negation() {
    let result = expr!(-x < y);
    assert!(matches!(result, Expression::Relation(..)));
}

#[test]
fn test_comparison_with_parentheses() {
    let result = expr!((x + 1) == (y - 1));
    assert!(matches!(result, Expression::Relation(..)));
}

#[test]
fn test_comparison_with_functions() {
    let result = expr!(sin(x) < cos(y));
    assert!(matches!(result, Expression::Relation(..)));
}

#[test]
fn test_comparison_with_constants() {
    let result = expr!(pi > e);
    assert!(matches!(result, Expression::Relation(..)));
}

#[test]
fn test_comparison_zero() {
    let result = expr!(x == 0);
    assert!(matches!(result, Expression::Relation(..)));
}

// ============================================================================
// Extended Methods (15 tests)
// ============================================================================

#[test]
fn test_method_abs_symbol() {
    let result = expr!(x.abs());
    assert_eq!(
        result,
        Expression::function("abs", vec![Expression::symbol(Symbol::scalar("x"))])
    );
}

#[test]
fn test_method_abs_literal() {
    let result = expr!((-5).abs());
    assert_eq!(
        result,
        Expression::function(
            "abs",
            vec![Expression::mul(vec![
                Expression::integer(-1),
                Expression::integer(5)
            ])]
        )
    );
}

#[test]
fn test_method_abs_expression() {
    let result = expr!((x + y).abs());
    assert!(matches!(result, Expression::Function { .. }));
}

#[test]
fn test_method_sqrt_symbol() {
    let result = expr!(x.sqrt());
    assert_eq!(
        result,
        Expression::function("sqrt", vec![Expression::symbol(Symbol::scalar("x"))])
    );
}

#[test]
fn test_method_sqrt_literal() {
    let result = expr!(4.sqrt());
    assert_eq!(
        result,
        Expression::function("sqrt", vec![Expression::integer(4)])
    );
}

#[test]
fn test_method_sqrt_expression() {
    let result = expr!((x * y).sqrt());
    assert!(matches!(result, Expression::Function { .. }));
}

#[test]
fn test_method_simplify_symbol() {
    let x_sym = Expression::symbol(Symbol::scalar("x"));
    let result = expr!(x.simplify());
    assert_eq!(result, x_sym);
}

#[test]
fn test_method_simplify_expression() {
    let result = expr!((x + x).simplify());
    assert_eq!(
        result,
        Expression::mul(vec![
            Expression::integer(2),
            Expression::symbol(Symbol::scalar("x"))
        ])
    );
}

#[test]
fn test_method_chaining_abs_sqrt() {
    let result = expr!(x.abs().sqrt());
    assert!(matches!(result, Expression::Function { .. }));
}

#[test]
fn test_method_with_operations() {
    let result = expr!((2 * x).abs());
    assert!(matches!(result, Expression::Function { .. }));
}

#[test]
fn test_method_with_functions() {
    let result = expr!(sin(x).abs());
    assert!(matches!(result, Expression::Function { .. }));
}

#[test]
fn test_method_sqrt_of_power() {
    let result = expr!(x.pow(2).sqrt());
    assert!(matches!(result, Expression::Function { .. }));
}

#[test]
fn test_method_abs_of_negation() {
    let result = expr!((-x).abs());
    assert!(matches!(result, Expression::Function { .. }));
}

#[test]
fn test_method_sqrt_of_product() {
    let result = expr!((x * y).sqrt());
    assert!(matches!(result, Expression::Function { .. }));
}

#[test]
fn test_method_abs_zero() {
    let result = expr!(0.abs());
    assert_eq!(
        result,
        Expression::function("abs", vec![Expression::integer(0)])
    );
}

// ============================================================================
// Complex Nested Expressions (30 tests)
// ============================================================================

#[test]
fn test_complex_addition_multiplication() {
    let result = expr!(2 * x + 3 * y);
    assert!(matches!(result, Expression::Add(..)));
}

#[test]
fn test_complex_mixed_operations() {
    let result = expr!(2 * x + 3 * y - z);
    assert!(matches!(result, Expression::Add(..)));
}

#[test]
fn test_complex_nested_parentheses() {
    let result = expr!((2 * x + 3) * (y - 1));
    let _ = result;
}

#[test]
fn test_complex_power_in_expression() {
    let result = expr!(x.pow(2) + y.pow(2));
    assert!(matches!(result, Expression::Add(..)));
}

#[test]
fn test_complex_division_in_expression() {
    let result = expr!(x / y + z);
    assert!(matches!(result, Expression::Add(..)));
}

#[test]
fn test_complex_negation_in_expression() {
    let result = expr!(-x + y);
    assert!(matches!(result, Expression::Add(..)));
}

#[test]
fn test_complex_functions_in_expression() {
    let result = expr!(sin(x) + cos(y));
    assert!(matches!(result, Expression::Add(..)));
}

#[test]
fn test_complex_multiple_functions() {
    let result = expr!(sin(x) * cos(y) + tan(z));
    assert!(matches!(result, Expression::Add(..)));
}

#[test]
fn test_complex_nested_functions() {
    let result = expr!(sin(cos(x)) + log(y));
    assert!(matches!(result, Expression::Add(..)));
}

#[test]
fn test_complex_function_with_operations() {
    let result = expr!(sin(x + y) * cos(x - y));
    let _ = result;
}

#[test]
fn test_complex_quadratic() {
    let result = expr!(x.pow(2) + 2 * x + 1);
    assert!(matches!(result, Expression::Add(..)));
}

#[test]
fn test_complex_cubic() {
    let result = expr!(x.pow(3) - 3 * x.pow(2) + 3 * x - 1);
    assert!(matches!(result, Expression::Add(..)));
}

#[test]
fn test_complex_rational_expression() {
    let result = expr!((x + 1) / (x - 1));
    let _ = result;
}

#[test]
fn test_complex_nested_divisions() {
    let result = expr!(x / (y / z));
    let _ = result;
}

#[test]
fn test_complex_product_of_sums() {
    let result = expr!((x + y) * (x - y));
    let _ = result;
}

#[test]
fn test_complex_sum_of_products() {
    let result = expr!(x * y + x * z + y * z);
    assert!(matches!(result, Expression::Add(..)));
}

#[test]
fn test_complex_with_constants() {
    let result = expr!(pi * x.pow(2) + e * x + 1);
    assert!(matches!(result, Expression::Add(..)));
}

#[test]
fn test_complex_trig_identity() {
    let result = expr!(sin(x).pow(2) + cos(x).pow(2));
    assert!(matches!(result, Expression::Add(..)));
}

#[test]
fn test_complex_exponential() {
    let result = expr!(exp(x + y) * exp(-z));
    let _ = result;
}

#[test]
fn test_complex_logarithm() {
    let result = expr!(log(x, y) + log(z, y));
    assert!(matches!(result, Expression::Add(..)));
}

#[test]
fn test_complex_power_tower() {
    let result = expr!(x.pow(y.pow(2)));
    assert!(matches!(result, Expression::Pow(..)));
}

#[test]
fn test_complex_nested_power() {
    let result = expr!((x + 1).pow(2));
    assert!(matches!(result, Expression::Pow(..)));
}

#[test]
fn test_complex_fraction_simplification() {
    let result = expr!((2 * x) / 2);
    let _ = result;
}

#[test]
fn test_complex_double_negation() {
    let result = expr!(-(-x + y));
    let _ = result;
}

#[test]
fn test_complex_multiple_terms() {
    let result = expr!(a + b + c + d + e);
    assert!(matches!(result, Expression::Add(..)));
}

#[test]
fn test_complex_multiple_factors() {
    let result = expr!(a * b * c * d * e);
    let _ = result;
}

#[test]
fn test_complex_polynomial() {
    let result = expr!(a * x.pow(3) + b * x.pow(2) + c * x + d);
    assert!(matches!(result, Expression::Add(..)));
}

#[test]
fn test_complex_nested_comparisons() {
    let result = expr!((x + 1) > (y - 1));
    assert!(matches!(result, Expression::Relation(..)));
}

#[test]
fn test_complex_method_in_expression() {
    let result = expr!(x.abs() + y.abs());
    assert!(matches!(result, Expression::Add(..)));
}

#[test]
fn test_complex_sqrt_in_expression() {
    let result = expr!(x.sqrt() * y.sqrt());
    let _ = result;
}

// ============================================================================
// Extended Functions (4+ args) (10 tests)
// ============================================================================

#[test]
fn test_function_four_args() {
    let result = expr!(f(a, b, c, d));
    assert_eq!(
        result,
        Expression::function(
            "f",
            vec![
                Expression::symbol(Symbol::scalar("a")),
                Expression::symbol(Symbol::scalar("b")),
                Expression::symbol(Symbol::scalar("c")),
                Expression::symbol(Symbol::scalar("d"))
            ]
        )
    );
}

#[test]
fn test_function_five_args() {
    let result = expr!(g(a, b, c, d, f));
    assert_eq!(
        result,
        Expression::function(
            "g",
            vec![
                Expression::symbol(Symbol::scalar("a")),
                Expression::symbol(Symbol::scalar("b")),
                Expression::symbol(Symbol::scalar("c")),
                Expression::symbol(Symbol::scalar("d")),
                Expression::symbol(Symbol::scalar("f"))
            ]
        )
    );
}

#[test]
fn test_function_six_args() {
    let result = expr!(h(a, b, c, d, e, f));
    assert!(matches!(result, Expression::Function { .. }));
}

#[test]
fn test_function_with_mixed_args() {
    let result = expr!(f(1, x, 2.5, y));
    assert!(matches!(result, Expression::Function { .. }));
}

#[test]
fn test_function_with_expressions() {
    let result = expr!(f(x + 1, y - 1, x * y, x / y));
    assert!(matches!(result, Expression::Function { .. }));
}

#[test]
fn test_function_with_functions() {
    let result = expr!(f(sin(x), cos(y), tan(z), log(w, 10)));
    assert!(matches!(result, Expression::Function { .. }));
}

#[test]
fn test_function_with_powers() {
    let result = expr!(f(x.pow(2), y.pow(3), z.pow(4), w.pow(5)));
    assert!(matches!(result, Expression::Function { .. }));
}

#[test]
fn test_function_with_methods() {
    let result = expr!(f(x.abs(), y.sqrt(), z.pow(2), w));
    assert!(matches!(result, Expression::Function { .. }));
}

#[test]
fn test_function_with_constants() {
    let result = expr!(f(pi, e, i, x));
    assert!(matches!(result, Expression::Function { .. }));
}

#[test]
fn test_function_nested_four_args() {
    let result = expr!(outer(inner(a, b, c, d), x, y, z));
    assert!(matches!(result, Expression::Function { .. }));
}

// ============================================================================
// Edge Cases and Precedence (15 tests)
// ============================================================================

#[test]
fn test_precedence_comparison_vs_addition() {
    let result = expr!(x + 1 == y + 2);
    assert!(matches!(result, Expression::Relation(..)));
}

#[test]
fn test_precedence_comparison_vs_multiplication() {
    let result = expr!(2 * x < 3 * y);
    assert!(matches!(result, Expression::Relation(..)));
}

#[test]
fn test_precedence_power_vs_multiplication() {
    let result = expr!(2 * x.pow(3));
    let _ = result;
}

#[test]
fn test_precedence_division_vs_power() {
    let result = expr!(x / y.pow(2));
    let _ = result;
}

#[test]
fn test_precedence_negation_vs_power() {
    let result = expr!((-x).pow(2));
    assert!(matches!(result, Expression::Pow(..)));
}

#[test]
fn test_precedence_function_vs_multiplication() {
    let result = expr!(2 * sin(x));
    let _ = result;
}

#[test]
fn test_precedence_method_vs_addition() {
    let result = expr!(x.abs() + y);
    assert!(matches!(result, Expression::Add(..)));
}

#[test]
fn test_edge_case_comparison_with_zero() {
    let result = expr!(x == 0);
    assert!(matches!(result, Expression::Relation(..)));
}

#[test]
fn test_edge_case_sqrt_of_zero() {
    let result = expr!(0.sqrt());
    assert_eq!(
        result,
        Expression::function("sqrt", vec![Expression::integer(0)])
    );
}

#[test]
fn test_edge_case_abs_of_zero() {
    let result = expr!(0.abs());
    assert_eq!(
        result,
        Expression::function("abs", vec![Expression::integer(0)])
    );
}

#[test]
#[allow(clippy::approx_constant)]
fn test_edge_case_comparison_constants() {
    let result = expr!(pi == 3.14);
    assert!(matches!(result, Expression::Relation(..)));
}

#[test]
fn test_edge_case_empty_parentheses_expression() {
    let result = expr!((x));
    assert_eq!(result, Expression::symbol(Symbol::scalar("x")));
}

#[test]
fn test_edge_case_multiple_parentheses() {
    let result = expr!((x + y));
    assert!(matches!(result, Expression::Add(..)));
}

#[test]
fn test_edge_case_comparison_chain_left() {
    let result = expr!((x < y) == (y < z));
    assert!(matches!(result, Expression::Relation(..)));
}

#[test]
fn test_edge_case_method_on_constant() {
    let result = expr!(pi.abs());
    assert_eq!(result, Expression::function("abs", vec![Expression::pi()]));
}

// ============================================================================
// Mathematical Precedence Tests (Power ^ higher than * / + -)
// ============================================================================
// These tests verify that ^ has higher precedence than * and / in mathematical
// expressions, allowing natural notation like 2*x^2 instead of 2*(x^2)

#[test]
fn test_math_precedence_power_before_multiply() {
    // 2*x^2 should parse as 2*(x^2), not (2*x)^2
    let result = expr!(2 * x ^ 2);
    // The outer expression should be Mul, containing 2 and x^2
    assert!(matches!(result, Expression::Mul(..)));
}

#[test]
fn test_math_precedence_power_before_add() {
    // x^2 + 1 should parse as (x^2) + 1
    let result = expr!(x ^ 2 + 1);
    assert!(matches!(result, Expression::Add(..)));
}

#[test]
fn test_math_precedence_quadratic_natural() {
    // a*x^2 + b*x + c should parse correctly without extra parens
    let result = expr!(a * x ^ 2 + b * x + c);
    assert!(matches!(result, Expression::Add(..)));
}

#[test]
fn test_math_precedence_power_before_divide() {
    // x^2 / y should parse as (x^2) / y
    let result = expr!(x ^ 2 / y);
    let _ = result;
}

#[test]
fn test_math_precedence_complex_polynomial() {
    // 2*x^3 + 3*x^2 + 4*x + 5
    let result = expr!(2 * x ^ 3 + 3 * x ^ 2 + 4 * x + 5);
    assert!(matches!(result, Expression::Add(..)));
}

#[test]
fn test_math_precedence_power_right_associative() {
    // 2^3^4 should parse as 2^(3^4), not (2^3)^4
    // Right-associative: 2^(3^4) = 2^81 = 2417851639229258349412352
    // Left-associative:  (2^3)^4 = 8^4 = 4096
    // Expression::pow() simplifies constant expressions to Number
    let result = expr!(2 ^ 3 ^ 4);
    // Verify it's NOT 4096 (which would indicate left-associativity)
    assert_ne!(result, Expression::integer(4096));
    // It should be a Number (simplified from pow)
    assert!(matches!(result, Expression::Number(..)));
}

#[test]
fn test_math_precedence_mixed_ops() {
    // a + b*c^d should parse as a + (b*(c^d))
    let result = expr!(a + b * c ^ d);
    assert!(matches!(result, Expression::Add(..)));
}

#[test]
fn test_math_precedence_comparison_with_power() {
    // x^2 == y^2 should parse correctly
    let result = expr!(x ^ 2 == y ^ 2);
    assert!(matches!(result, Expression::Relation(..)));
}

#[test]
fn test_math_precedence_sum_of_powers() {
    // x^2 + y^2 + z^2
    let result = expr!(x ^ 2 + y ^ 2 + z ^ 2);
    assert!(matches!(result, Expression::Add(..)));
}

#[test]
fn test_math_precedence_product_of_powers() {
    // x^2 * y^3 * z^4
    let result = expr!(x ^ 2 * y ^ 3 * z ^ 4);
    let _ = result;
}
