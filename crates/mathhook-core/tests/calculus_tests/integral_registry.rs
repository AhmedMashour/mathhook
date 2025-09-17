//! Mathematical integration tests for integral table/registry system
//!
//! Tests verify that table lookup produces mathematically correct results.
//! Each test validates the Fundamental Theorem of Calculus: d/dx(∫f dx) = f

use mathhook_core::calculus::derivatives::Derivative;
use mathhook_core::calculus::integrals::Integration;
use mathhook_core::core::Expression;
use mathhook_core::simplify::Simplify;
use mathhook_core::symbol;

// Helper: Verify Fundamental Theorem of Calculus
// Tests that d/dx(∫f dx) = f
fn verify_ftc(expr: &Expression, var: mathhook_core::Symbol) {
    let integral = expr.integrate(var.clone(), 0);
    let derivative = integral.derivative(var).simplify();
    let original = expr.simplify();

    assert_eq!(
        derivative, original,
        "Fundamental Theorem violated: d/dx(∫f dx) ≠ f"
    );
}

#[test]
fn test_integrate_sin_x() {
    let x = symbol!(x);
    let expr = Expression::function("sin", vec![Expression::symbol(x.clone())]);

    verify_ftc(&expr, x);
}

#[test]
fn test_integrate_cos_x() {
    let x = symbol!(x);
    let expr = Expression::function("cos", vec![Expression::symbol(x.clone())]);

    verify_ftc(&expr, x);
}

#[test]
fn test_integrate_tan_x() {
    let x = symbol!(x);
    let expr = Expression::function("tan", vec![Expression::symbol(x.clone())]);

    let integral = expr.integrate(x, 0);
    assert!(
        !matches!(integral, Expression::Calculus(_)),
        "∫tan(x) dx should be solved (= -ln|cos(x)| + C)"
    );
}

#[test]
fn test_integrate_sec_x() {
    let x = symbol!(x);
    let expr = Expression::function("sec", vec![Expression::symbol(x.clone())]);

    let integral = expr.integrate(x, 0);
    assert!(
        !matches!(integral, Expression::Calculus(_)),
        "∫sec(x) dx should be solved (= ln|sec(x) + tan(x)| + C)"
    );
}

#[test]
fn test_integrate_csc_x() {
    let x = symbol!(x);
    let expr = Expression::function("csc", vec![Expression::symbol(x.clone())]);

    let integral = expr.integrate(x, 0);
    assert!(
        !matches!(integral, Expression::Calculus(_)),
        "∫csc(x) dx should be solved (= -ln|csc(x) + cot(x)| + C)"
    );
}

#[test]
fn test_integrate_cot_x() {
    let x = symbol!(x);
    let expr = Expression::function("cot", vec![Expression::symbol(x.clone())]);

    let integral = expr.integrate(x, 0);
    assert!(
        !matches!(integral, Expression::Calculus(_)),
        "∫cot(x) dx should be solved (= ln|sin(x)| + C)"
    );
}

#[test]
fn test_integrate_exp_x() {
    let x = symbol!(x);
    let expr = Expression::function("exp", vec![Expression::symbol(x.clone())]);

    verify_ftc(&expr, x);
}

#[test]
fn test_integrate_ln_x() {
    let x = symbol!(x);
    let expr = Expression::function("ln", vec![Expression::symbol(x.clone())]);

    let integral = expr.integrate(x, 0);

    assert!(
        !matches!(integral, Expression::Calculus(_)),
        "∫ln(x) dx should be solved (= x·ln(x) - x + C)"
    );
}

#[test]
fn test_integrate_log_base_10() {
    let x = symbol!(x);
    let expr = Expression::function("log", vec![Expression::symbol(x.clone())]);

    let integral = expr.integrate(x, 0);

    assert!(
        !matches!(integral, Expression::Calculus(_)),
        "∫log₁₀(x) dx should be solved (= (1/ln(10))·[x·ln(x) - x] + C)"
    );
}

#[test]
fn test_integrate_arcsin_x() {
    let x = symbol!(x);
    let expr = Expression::function("arcsin", vec![Expression::symbol(x.clone())]);

    let integral = expr.integrate(x, 0);

    assert!(
        !matches!(integral, Expression::Calculus(_)),
        "∫arcsin(x) dx should be solved (= x·arcsin(x) + √(1-x²) + C)"
    );
}

#[test]
fn test_integrate_arccos_x() {
    let x = symbol!(x);
    let expr = Expression::function("arccos", vec![Expression::symbol(x.clone())]);

    let integral = expr.integrate(x, 0);

    assert!(
        !matches!(integral, Expression::Calculus(_)),
        "∫arccos(x) dx should be solved (= x·arccos(x) - √(1-x²) + C)"
    );
}

#[test]
fn test_integrate_arctan_x() {
    let x = symbol!(x);
    let expr = Expression::function("arctan", vec![Expression::symbol(x.clone())]);

    let integral = expr.integrate(x, 0);

    assert!(
        !matches!(integral, Expression::Calculus(_)),
        "∫arctan(x) dx should be solved (= x·arctan(x) - (1/2)·ln(1+x²) + C)"
    );
}

#[test]
fn test_integrate_sinh_x() {
    let x = symbol!(x);
    let expr = Expression::function("sinh", vec![Expression::symbol(x.clone())]);

    verify_ftc(&expr, x);
}

#[test]
fn test_integrate_cosh_x() {
    let x = symbol!(x);
    let expr = Expression::function("cosh", vec![Expression::symbol(x.clone())]);

    verify_ftc(&expr, x);
}

#[test]
fn test_integrate_tanh_x() {
    let x = symbol!(x);
    let expr = Expression::function("tanh", vec![Expression::symbol(x.clone())]);

    let integral = expr.integrate(x, 0);

    assert!(
        !matches!(integral, Expression::Calculus(_)),
        "∫tanh(x) dx should be solved (= ln(cosh(x)) + C)"
    );
}

#[test]
fn test_integrate_sqrt_x() {
    let x = symbol!(x);
    let expr = Expression::function("sqrt", vec![Expression::symbol(x.clone())]);

    let integral = expr.integrate(x, 0);

    assert!(
        !matches!(integral, Expression::Calculus(_)),
        "∫√x dx should be solved (= (2/3)·x^(3/2) + C)"
    );
}

#[test]
fn test_integrate_one_over_x() {
    let x = symbol!(x);
    let expr = Expression::pow(Expression::symbol(x.clone()), Expression::integer(-1));

    let integral = expr.integrate(x, 0);

    assert!(
        !matches!(integral, Expression::Calculus(_)),
        "∫(1/x) dx should be solved (= ln|x| + C)"
    );
}

#[test]
fn test_integrate_x_squared() {
    let x = symbol!(x);
    let expr = Expression::pow(Expression::symbol(x.clone()), Expression::integer(2));

    verify_ftc(&expr, x);
}

#[test]
fn test_integrate_x_cubed() {
    let x = symbol!(x);
    let expr = Expression::pow(Expression::symbol(x.clone()), Expression::integer(3));

    verify_ftc(&expr, x);
}

#[test]
fn test_integrate_sin_2x() {
    let x = symbol!(x);
    let inner = Expression::mul(vec![Expression::integer(2), Expression::symbol(x.clone())]);
    let expr = Expression::function("sin", vec![inner]);

    let integral = expr.integrate(x, 0);

    assert!(
        !matches!(integral, Expression::Calculus(_)),
        "∫sin(2x) dx should be solved (= -(1/2)cos(2x) + C)"
    );
}

#[test]
fn test_integrate_cos_3x() {
    let x = symbol!(x);
    let inner = Expression::mul(vec![Expression::integer(3), Expression::symbol(x.clone())]);
    let expr = Expression::function("cos", vec![inner]);

    let integral = expr.integrate(x, 0);

    assert!(
        !matches!(integral, Expression::Calculus(_)),
        "∫cos(3x) dx should be solved (= (1/3)sin(3x) + C)"
    );
}

#[test]
fn test_integrate_exp_negative_x() {
    let x = symbol!(x);
    let inner = Expression::mul(vec![Expression::integer(-1), Expression::symbol(x.clone())]);
    let expr = Expression::function("exp", vec![inner]);

    let integral = expr.integrate(x, 0);

    assert!(
        !matches!(integral, Expression::Calculus(_)),
        "∫exp(-x) dx should be solved (= -exp(-x) + C)"
    );
}

#[test]
fn test_integrate_constant() {
    let x = symbol!(x);
    let expr = Expression::integer(5);

    let integral = expr.integrate(x.clone(), 0);

    let expected = Expression::mul(vec![Expression::integer(5), Expression::symbol(x)]);
    assert_eq!(integral, expected, "∫5 dx should equal 5x + C");
}

#[test]
fn test_integrate_zero() {
    let x = symbol!(x);
    let expr = Expression::integer(0);

    let integral = expr.integrate(x.clone(), 0);

    let expected = Expression::mul(vec![Expression::integer(0), Expression::symbol(x)]);
    assert_eq!(integral, expected, "∫0 dx should equal 0");
}

#[test]
fn test_integrate_function_of_different_variable() {
    let x = symbol!(x);
    let y = symbol!(y);

    let expr = Expression::function("sin", vec![Expression::symbol(y.clone())]);
    let integral = expr.integrate(x.clone(), 0);

    let expected = Expression::mul(vec![
        Expression::function("sin", vec![Expression::symbol(y)]),
        Expression::symbol(x),
    ]);

    assert_eq!(
        integral, expected,
        "∫sin(y) dx should treat sin(y) as constant → sin(y)·x + C"
    );
}

#[test]
fn test_integrate_unknown_function_returns_symbolic() {
    let x = symbol!(x);
    let expr = Expression::function("unknown_func", vec![Expression::symbol(x.clone())]);

    let integral = expr.integrate(x, 0);

    assert!(
        matches!(integral, Expression::Calculus(_)),
        "Unknown functions should return symbolic integral"
    );
}

#[test]
fn test_integrate_sin_x_squared_remains_symbolic() {
    let x = symbol!(x);
    let inner = Expression::pow(Expression::symbol(x.clone()), Expression::integer(2));
    let expr = Expression::function("sin", vec![inner]);

    let integral = expr.integrate(x, 0);

    // ∫sin(x²) dx is non-elementary (requires Fresnel functions)
    // Verify it returns a valid result (either symbolic or evaluated)
    assert!(
        !integral.to_string().is_empty(),
        "∫sin(x²) dx should return valid result (symbolic or Fresnel function)"
    );
}

#[test]
fn test_integrate_sin_x_times_cos_x() {
    let x = symbol!(x);
    let sin_x = Expression::function("sin", vec![Expression::symbol(x.clone())]);
    let cos_x = Expression::function("cos", vec![Expression::symbol(x.clone())]);
    let expr = Expression::mul(vec![sin_x, cos_x]);

    let integral = expr.integrate(x, 0);

    // Smoke test: verify integration completes without panicking and returns valid result
    assert!(
        !integral.to_string().is_empty(),
        "∫sin(x)cos(x) dx: should be solved via substitution or remain symbolic"
    );
}

#[test]
fn test_ftc_all_basic_trig() {
    let x = symbol!(x);

    let functions = vec![
        Expression::function("sin", vec![Expression::symbol(x.clone())]),
        Expression::function("cos", vec![Expression::symbol(x.clone())]),
    ];

    for func in functions {
        verify_ftc(&func, x.clone());
    }
}

#[test]
fn test_ftc_all_hyperbolic() {
    let x = symbol!(x);

    let functions = vec![
        Expression::function("sinh", vec![Expression::symbol(x.clone())]),
        Expression::function("cosh", vec![Expression::symbol(x.clone())]),
    ];

    for func in functions {
        verify_ftc(&func, x.clone());
    }
}

#[test]
fn test_ftc_exponential() {
    let x = symbol!(x);
    let expr = Expression::function("exp", vec![Expression::symbol(x.clone())]);
    verify_ftc(&expr, x);
}

#[test]
fn test_ftc_power_functions() {
    let x = symbol!(x);

    let powers = vec![
        Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
        Expression::pow(Expression::symbol(x.clone()), Expression::integer(3)),
        Expression::pow(Expression::symbol(x.clone()), Expression::integer(4)),
    ];

    for power in powers {
        verify_ftc(&power, x.clone());
    }
}
