// Integration tests for function evaluation dispatch flow
//
// These tests verify that the evaluation system correctly handles:
// - Composite expressions with mixed symbolic/numeric parts
// - Evaluation dispatch from Expression → Function implementations
// - Special value recognition in complex expressions
// - Recursive evaluation in nested structures

use mathhook_core::core::expression::evaluation::dispatch::evaluate_function_dispatch;
use mathhook_core::core::expression::Expression;
use mathhook_core::core::symbol::Symbol;
use mathhook_core::expr;
use mathhook_core::functions::elementary::abs_eval::abs;
use mathhook_core::functions::elementary::exp_eval::exp;
use mathhook_core::functions::elementary::log_eval::ln;
use mathhook_core::functions::elementary::sqrt_eval::sqrt;
use mathhook_core::functions::elementary::trigonometric::{cos, sin};
use mathhook_core::functions::special::gamma::gamma;
use mathhook_core::simplify::Simplify;

// Direct function call tests
// These verify that direct function calls work correctly
#[test]
fn test_direct_function_call_sin_zero() {
    let result = sin(&Expression::integer(0));
    assert_eq!(result, Expression::integer(0));
}

#[test]
fn test_direct_function_call_cos_zero() {
    let result = cos(&Expression::integer(0));
    assert_eq!(result, Expression::integer(1));
}

#[test]
fn test_direct_function_call_exp_zero() {
    let result = exp(&Expression::integer(0));
    assert_eq!(result, Expression::integer(1));
}

#[test]
fn test_direct_function_call_ln_one() {
    let result = ln(&Expression::integer(1));
    assert_eq!(result, Expression::integer(0));
}

#[test]
fn test_direct_function_call_sqrt_four() {
    let result = sqrt(&Expression::integer(4));
    assert_eq!(result, Expression::integer(2));
}

#[test]
fn test_direct_function_call_gamma_five() {
    let result = gamma(&Expression::integer(5));
    assert_eq!(result, Expression::integer(24));
}

#[test]
fn test_nested_function_composition_symbolic() {
    let x = Symbol::scalar("x");

    // sin(cos(x)) - nested composition preserves structure
    let nested = Expression::function(
        "sin",
        vec![Expression::function(
            "cos",
            vec![Expression::symbol(x.clone())],
        )],
    );

    let result = nested.simplify();

    // Should remain symbolic (correct behavior)
    assert!(result.to_string().contains("sin"));
    assert!(result.to_string().contains("cos"));
}

#[test]
fn test_identity_simplification_exp_log() {
    // exp(log(x)) should simplify to x
    let expr = Expression::function("exp", vec![Expression::function("log", vec![expr!(x)])]);

    let result = expr.simplify();

    // Identity simplification works
    assert_eq!(result, expr!(x));
}

#[test]
fn test_direct_dispatch_works() {
    // Direct dispatch through evaluate_function_dispatch
    let result = evaluate_function_dispatch("sin", &[Expression::integer(0)]);
    assert!(result.is_some());
    assert_eq!(result.unwrap(), Expression::integer(0));

    // Gamma through dispatch
    let result = evaluate_function_dispatch("gamma", &[Expression::integer(5)]);
    assert!(result.is_some());
    assert_eq!(result.unwrap(), Expression::integer(24));
}

#[test]
fn test_function_in_addition_symbolic() {
    let x = Symbol::scalar("x");

    // sin(x) + 3 should preserve both parts
    let expr = Expression::add(vec![
        Expression::function("sin", vec![Expression::symbol(x.clone())]),
        Expression::integer(3),
    ]);

    let result = expr.simplify();

    // Should remain as sin(x) + 3 (correct)
    assert!(result.to_string().contains("sin"));
    assert!(result.to_string().contains("3"));
}

#[test]
fn test_abs_evaluation_direct() {
    // abs(-5) using direct call
    let expr = Expression::mul(vec![Expression::integer(-1), Expression::integer(5)]);

    let result = abs(&expr);
    assert_eq!(result, Expression::integer(5));
}

// Integration tests for Expression::function + simplify() evaluation
// These tests verify that special values are correctly evaluated during simplification

#[test]
fn test_composite_expression_mixed_evaluation() {
    let x = Symbol::scalar("x");
    let y = Symbol::scalar("y");

    // Composite expression: sin(x^2 + 1) * cos(y) - sqrt(4)
    let composite = Expression::add(vec![
        Expression::mul(vec![
            Expression::function(
                "sin",
                vec![Expression::add(vec![
                    Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
                    Expression::integer(1),
                ])],
            ),
            Expression::function("cos", vec![Expression::symbol(y.clone())]),
        ]),
        Expression::mul(vec![
            Expression::integer(-1),
            Expression::function("sqrt", vec![Expression::integer(4)]),
        ]),
    ]);

    let result = composite.simplify();

    // sqrt(4) should evaluate to 2, symbolic parts preserved
    assert!(result.to_string().contains("sin"));
    assert!(result.to_string().contains("cos"));
    assert!(!result.to_string().contains("sqrt"));
    assert!(result.to_string().contains("2"));
}

#[test]
fn test_evaluation_propagation_in_multiplication() {
    let x = Symbol::scalar("x");

    // Expression: 2 * sin(0) * exp(x) * sqrt(16)
    let expr = Expression::mul(vec![
        Expression::integer(2),
        Expression::function("sin", vec![Expression::integer(0)]),
        Expression::function("exp", vec![Expression::symbol(x.clone())]),
        Expression::function("sqrt", vec![Expression::integer(16)]),
    ]);

    let result = expr.simplify();

    // sin(0) = 0, so entire expression should be 0
    assert_eq!(result, Expression::integer(0));
}

#[test]
fn test_special_values_in_nested_expressions() {
    // sin(π/2) * cos(0) + sqrt(9)
    let expr = Expression::add(vec![
        Expression::mul(vec![
            Expression::function(
                "sin",
                vec![Expression::div(Expression::pi(), Expression::integer(2))],
            ),
            Expression::function("cos", vec![Expression::integer(0)]),
        ]),
        Expression::function("sqrt", vec![Expression::integer(9)]),
    ]);

    let result = expr.simplify();

    // sin(π/2)=1, cos(0)=1, sqrt(9)=3 → 1*1+3=4
    assert_eq!(result, Expression::integer(4));
}

#[test]
fn test_partial_evaluation_preserves_symbolic() {
    let x = Symbol::scalar("x");
    let y = Symbol::scalar("y");

    // sin(x) + cos(y) + sqrt(4)
    let expr = Expression::add(vec![
        Expression::function("sin", vec![Expression::symbol(x.clone())]),
        Expression::function("cos", vec![Expression::symbol(y.clone())]),
        Expression::function("sqrt", vec![Expression::integer(4)]),
    ]);

    let result = expr.simplify();

    // sqrt(4) → 2, symbolic parts stay
    assert!(result.to_string().contains("sin"));
    assert!(result.to_string().contains("cos"));
    assert!(!result.to_string().contains("sqrt"));
}

#[test]
fn test_gamma_function_evaluation_through_expression() {
    // gamma(5) through Expression::function
    let expr = Expression::function("gamma", vec![Expression::integer(5)]);
    let result = expr.simplify();

    // Γ(5) = 4! = 24
    assert_eq!(result, Expression::integer(24));
}

#[test]
fn test_multiple_special_values() {
    // sin(0) + cos(0) + exp(0) + log(1)
    let expr = Expression::add(vec![
        Expression::function("sin", vec![Expression::integer(0)]),
        Expression::function("cos", vec![Expression::integer(0)]),
        Expression::function("exp", vec![Expression::integer(0)]),
        Expression::function("log", vec![Expression::integer(1)]),
    ]);

    let result = expr.simplify();

    // 0 + 1 + 1 + 0 = 2
    assert_eq!(result, Expression::integer(2));
}

#[test]
fn test_nested_sqrt_evaluation() {
    // sqrt(sqrt(16))
    let expr = Expression::function(
        "sqrt",
        vec![Expression::function("sqrt", vec![Expression::integer(16)])],
    );

    let result = expr.simplify();

    // sqrt(16)=4, sqrt(4)=2
    assert_eq!(result, Expression::integer(2));
}

#[test]
fn test_function_in_subtraction() {
    let x = Symbol::scalar("x");

    // cos(x) - sqrt(9)
    let expr = Expression::add(vec![
        Expression::function("cos", vec![Expression::symbol(x.clone())]),
        Expression::mul(vec![
            Expression::integer(-1),
            Expression::function("sqrt", vec![Expression::integer(9)]),
        ]),
    ]);

    let result = expr.simplify();

    // sqrt(9)=3, result is cos(x)-3
    assert!(result.to_string().contains("cos"));
    assert!(result.to_string().contains("3"));
    assert!(!result.to_string().contains("sqrt"));
}

#[test]
fn test_composite_with_powers() {
    let x = Symbol::scalar("x");

    // sin(x^2) * sqrt(4)
    let expr = Expression::mul(vec![
        Expression::function(
            "sin",
            vec![Expression::pow(
                Expression::symbol(x.clone()),
                Expression::integer(2),
            )],
        ),
        Expression::function("sqrt", vec![Expression::integer(4)]),
    ]);

    let result = expr.simplify();

    // sqrt(4)=2, result is 2*sin(x^2)
    assert!(result.to_string().contains("sin"));
    assert!(result.to_string().contains("2"));
    assert!(!result.to_string().contains("sqrt"));
}

#[test]
fn test_symbolic_preservation_in_complex_expr() {
    let x = Symbol::scalar("x");
    let y = Symbol::scalar("y");

    // (sin(x) + cos(y)) * sqrt(4) - exp(0)
    let expr = Expression::add(vec![
        Expression::mul(vec![
            Expression::add(vec![
                Expression::function("sin", vec![Expression::symbol(x.clone())]),
                Expression::function("cos", vec![Expression::symbol(y.clone())]),
            ]),
            Expression::function("sqrt", vec![Expression::integer(4)]),
        ]),
        Expression::mul(vec![
            Expression::integer(-1),
            Expression::function("exp", vec![Expression::integer(0)]),
        ]),
    ]);

    let result = expr.simplify();

    // sqrt(4)=2, exp(0)=1, result is 2*(sin(x)+cos(y))-1
    assert!(result.to_string().contains("sin"));
    assert!(result.to_string().contains("cos"));
    assert!(!result.to_string().contains("sqrt"));
    assert!(!result.to_string().contains("exp"));
}

#[test]
fn test_evaluation_flow_numeric_propagation() {
    // (2 + 3) * sin(0)
    let expr = Expression::mul(vec![
        Expression::add(vec![Expression::integer(2), Expression::integer(3)]),
        Expression::function("sin", vec![Expression::integer(0)]),
    ]);

    let result = expr.simplify();

    // (2+3)=5, sin(0)=0, 5*0=0
    assert_eq!(result, Expression::integer(0));
}

#[test]
fn test_function_composition_with_constants() {
    // sin(cos(0))
    let expr = Expression::function(
        "sin",
        vec![Expression::function("cos", vec![Expression::integer(0)])],
    );

    let result = expr.simplify();

    // cos(0)=1, sin(1) stays symbolic
    assert!(result.to_string().contains("sin"));
    assert!(result.to_string().contains("1"));
}

// Test that documents the current evaluation dispatch mechanism
#[test]
fn test_evaluation_dispatch_current_state() {
    // Direct function calls work
    assert_eq!(sin(&Expression::integer(0)), Expression::integer(0));
    assert_eq!(exp(&Expression::integer(0)), Expression::integer(1));
    assert_eq!(sqrt(&Expression::integer(4)), Expression::integer(2));

    // Expression::function DOES evaluate special values during simplify()
    let sin_expr = Expression::function("sin", vec![Expression::integer(0)]);
    let simplified = sin_expr.simplify();

    // Special values ARE evaluated
    assert_eq!(simplified, Expression::integer(0));

    // This documents that Expression::function + simplify() DOES dispatch
    // to special value evaluation for known special values
}

#[test]
fn test_partial_debug() {
    use mathhook_core::core::symbol::Symbol;
    use mathhook_core::core::Expression;
    use mathhook_core::simplify::Simplify;

    let x = Symbol::scalar("x");
    let y = Symbol::scalar("y");

    let expr = Expression::add(vec![
        Expression::function("sin", vec![Expression::symbol(x.clone())]),
        Expression::function("cos", vec![Expression::symbol(y.clone())]),
        Expression::function("sqrt", vec![Expression::integer(4)]),
    ]);

    let result = expr.simplify();
    println!("Result: {:?}", result);
    println!("String: {}", result);
    println!("Contains 'sin': {}", result.to_string().contains("sin"));
    println!("Contains 'cos': {}", result.to_string().contains("cos"));
    println!("Contains 'sqrt': {}", result.to_string().contains("sqrt"));
}
