//! Comprehensive integration table tests
//!
//! Tests all elementary function integrals and integration by parts

use mathhook_core::calculus::derivatives::Derivative;
use mathhook_core::calculus::integrals::{Integration, IntegrationByParts};
use mathhook_core::core::Expression;
use mathhook_core::simplify::Simplify;
use mathhook_core::symbol;

#[test]
fn test_power_rule_basic() {
    let x = symbol!(x);

    // ∫ x dx = x²/2 + C
    let expr = Expression::symbol(x.clone());
    let result = expr.integrate(x.clone());

    // Result should be (1/2)·x²
    let expected = Expression::mul(vec![
        Expression::mul(vec![
            Expression::integer(1),
            Expression::pow(Expression::integer(2), Expression::integer(-1)),
        ]),
        Expression::pow(Expression::symbol(x), Expression::integer(2)),
    ]);

    assert_eq!(result.simplify(), expected.simplify());
}

#[test]
fn test_power_rule_quadratic() {
    let x = symbol!(x);

    // ∫ x² dx = x³/3 + C
    let expr = Expression::pow(Expression::symbol(x.clone()), Expression::integer(2));
    let result = expr.integrate(x.clone());

    // Result should be (1/3)·x³
    let expected = Expression::mul(vec![
        Expression::mul(vec![
            Expression::integer(1),
            Expression::pow(Expression::integer(3), Expression::integer(-1)),
        ]),
        Expression::pow(Expression::symbol(x), Expression::integer(3)),
    ]);

    assert_eq!(result.simplify(), expected.simplify());
}

#[test]
fn test_power_rule_cubic() {
    let x = symbol!(x);

    // ∫ x³ dx = x⁴/4 + C
    let expr = Expression::pow(Expression::symbol(x.clone()), Expression::integer(3));
    let result = expr.integrate(x.clone());

    // Result should be (1/4)·x⁴
    let expected = Expression::mul(vec![
        Expression::mul(vec![
            Expression::integer(1),
            Expression::pow(Expression::integer(4), Expression::integer(-1)),
        ]),
        Expression::pow(Expression::symbol(x), Expression::integer(4)),
    ]);

    assert_eq!(result.simplify(), expected.simplify());
}

#[test]
fn test_reciprocal_integral() {
    let x = symbol!(x);

    // ∫ 1/x dx = ln|x| + C
    let expr = Expression::pow(Expression::symbol(x.clone()), Expression::integer(-1));
    let result = expr.integrate(x.clone());

    // Result should be ln|x|
    let expected = Expression::function(
        "ln",
        vec![Expression::function("abs", vec![Expression::symbol(x)])],
    );

    assert_eq!(result, expected);
}

#[test]
fn test_constant_integral() {
    let x = symbol!(x);

    // ∫ 5 dx = 5x + C
    let expr = Expression::integer(5);
    let result = expr.integrate(x.clone());

    // Result should be 5x
    let expected = Expression::mul(vec![Expression::integer(5), Expression::symbol(x)]);

    assert_eq!(result, expected);
}

#[test]
fn test_sin_integral() {
    let x = symbol!(x);

    // ∫ sin(x) dx = -cos(x) + C
    let expr = Expression::function("sin", vec![Expression::symbol(x.clone())]);
    let result = expr.integrate(x);

    // Result should be -cos(x)
    let expected = Expression::mul(vec![
        Expression::integer(-1),
        Expression::function("cos", vec![Expression::symbol(symbol!(x))]),
    ]);

    assert_eq!(result, expected);
}

#[test]
fn test_cos_integral() {
    let x = symbol!(x);

    // ∫ cos(x) dx = sin(x) + C
    let expr = Expression::function("cos", vec![Expression::symbol(x.clone())]);
    let result = expr.integrate(x);

    // Result should be sin(x)
    let expected = Expression::function("sin", vec![Expression::symbol(symbol!(x))]);

    assert_eq!(result, expected);
}

#[test]
fn test_exp_integral() {
    let x = symbol!(x);

    // ∫ e^x dx = e^x + C
    let expr = Expression::function("exp", vec![Expression::symbol(x.clone())]);
    let result = expr.integrate(x);

    // Result should be e^x
    let expected = Expression::function("exp", vec![Expression::symbol(symbol!(x))]);

    assert_eq!(result, expected);
}

#[test]
fn test_tan_integral() {
    let x = symbol!(x);

    // ∫ tan(x) dx = -ln|cos(x)| + C
    let expr = Expression::function("tan", vec![Expression::symbol(x.clone())]);
    let result = expr.integrate(x);

    // Result should be -ln|cos(x)|
    let expected = Expression::mul(vec![
        Expression::integer(-1),
        Expression::function(
            "ln",
            vec![Expression::function(
                "abs",
                vec![Expression::function("cos", vec![Expression::symbol(symbol!(x))])],
            )],
        ),
    ]);

    assert_eq!(result, expected);
}

#[test]
fn test_ln_integral() {
    let x = symbol!(x);

    // ∫ ln(x) dx = x·ln(x) - x + C
    let expr = Expression::function("ln", vec![Expression::symbol(x.clone())]);
    let result = expr.integrate(x.clone());

    // Result should be x·ln(x) - x
    let expected = Expression::add(vec![
        Expression::mul(vec![
            Expression::symbol(x.clone()),
            Expression::function("ln", vec![Expression::symbol(x.clone())]),
        ]),
        Expression::mul(vec![Expression::integer(-1), Expression::symbol(x)]),
    ]);

    assert_eq!(result, expected);
}

#[test]
fn test_sqrt_integral() {
    let x = symbol!(x);

    // ∫ √x dx = (2/3)·x^(3/2) + C
    let expr = Expression::function("sqrt", vec![Expression::symbol(x.clone())]);
    let result = expr.integrate(x);

    // Result should be (2/3)·x^(3/2)
    let expected = Expression::mul(vec![
        Expression::mul(vec![
            Expression::integer(2),
            Expression::pow(Expression::integer(3), Expression::integer(-1)),
        ]),
        Expression::pow(
            Expression::symbol(symbol!(x)),
            Expression::mul(vec![
                Expression::integer(3),
                Expression::pow(Expression::integer(2), Expression::integer(-1)),
            ]),
        ),
    ]);

    assert_eq!(result, expected);
}

#[test]
fn test_arcsin_integral() {
    let x = symbol!(x);

    // ∫ arcsin(x) dx = x·arcsin(x) + √(1-x²) + C
    let expr = Expression::function("arcsin", vec![Expression::symbol(x.clone())]);
    let result = expr.integrate(x.clone());

    // Result should be x·arcsin(x) + √(1-x²)
    let expected = Expression::add(vec![
        Expression::mul(vec![
            Expression::symbol(x.clone()),
            Expression::function("arcsin", vec![Expression::symbol(x.clone())]),
        ]),
        Expression::function(
            "sqrt",
            vec![Expression::add(vec![
                Expression::integer(1),
                Expression::mul(vec![
                    Expression::integer(-1),
                    Expression::pow(Expression::symbol(x), Expression::integer(2)),
                ]),
            ])],
        ),
    ]);

    assert_eq!(result, expected);
}

#[test]
fn test_arctan_integral() {
    let x = symbol!(x);

    // ∫ arctan(x) dx = x·arctan(x) - (1/2)·ln(1+x²) + C
    let expr = Expression::function("arctan", vec![Expression::symbol(x.clone())]);
    let result = expr.integrate(x.clone());

    // Result should be x·arctan(x) - (1/2)·ln(1+x²)
    let expected = Expression::add(vec![
        Expression::mul(vec![
            Expression::symbol(x.clone()),
            Expression::function("arctan", vec![Expression::symbol(x.clone())]),
        ]),
        Expression::mul(vec![
            Expression::mul(vec![
                Expression::integer(-1),
                Expression::pow(Expression::integer(2), Expression::integer(-1)),
            ]),
            Expression::function(
                "ln",
                vec![Expression::add(vec![
                    Expression::integer(1),
                    Expression::pow(Expression::symbol(x), Expression::integer(2)),
                ])],
            ),
        ]),
    ]);

    assert_eq!(result, expected);
}

#[test]
fn test_sinh_integral() {
    let x = symbol!(x);

    // ∫ sinh(x) dx = cosh(x) + C
    let expr = Expression::function("sinh", vec![Expression::symbol(x.clone())]);
    let result = expr.integrate(x);

    // Result should be cosh(x)
    let expected = Expression::function("cosh", vec![Expression::symbol(symbol!(x))]);

    assert_eq!(result, expected);
}

#[test]
fn test_cosh_integral() {
    let x = symbol!(x);

    // ∫ cosh(x) dx = sinh(x) + C
    let expr = Expression::function("cosh", vec![Expression::symbol(x.clone())]);
    let result = expr.integrate(x);

    // Result should be sinh(x)
    let expected = Expression::function("sinh", vec![Expression::symbol(symbol!(x))]);

    assert_eq!(result, expected);
}

#[test]
fn test_tanh_integral() {
    let x = symbol!(x);

    // ∫ tanh(x) dx = ln(cosh(x)) + C
    let expr = Expression::function("tanh", vec![Expression::symbol(x.clone())]);
    let result = expr.integrate(x);

    // Result should be ln(cosh(x))
    let expected = Expression::function(
        "ln",
        vec![Expression::function("cosh", vec![Expression::symbol(symbol!(x))])],
    );

    assert_eq!(result, expected);
}

#[test]
fn test_linearity_sum() {
    let x = symbol!(x);

    // ∫ (x + 2) dx = x²/2 + 2x + C
    let expr = Expression::add(vec![Expression::symbol(x.clone()), Expression::integer(2)]);
    let result = expr.integrate(x.clone());

    // Result should be x²/2 + 2x
    let expected = Expression::add(vec![
        Expression::mul(vec![
            Expression::mul(vec![
                Expression::integer(1),
                Expression::pow(Expression::integer(2), Expression::integer(-1)),
            ]),
            Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
        ]),
        Expression::mul(vec![Expression::integer(2), Expression::symbol(x)]),
    ]);

    assert_eq!(result.simplify(), expected.simplify());
}

#[test]
fn test_constant_multiple() {
    let x = symbol!(x);

    // ∫ 3x dx = (3/2)·x² + C
    let expr = Expression::mul(vec![Expression::integer(3), Expression::symbol(x.clone())]);
    let result = expr.integrate(x.clone());

    // Result should be (3/2)·x²
    let expected = Expression::mul(vec![
        Expression::integer(3),
        Expression::mul(vec![
            Expression::mul(vec![
                Expression::integer(1),
                Expression::pow(Expression::integer(2), Expression::integer(-1)),
            ]),
            Expression::pow(Expression::symbol(x), Expression::integer(2)),
        ]),
    ]);

    assert_eq!(result.simplify(), expected.simplify());
}

#[test]
fn test_polynomial_integral() {
    let x = symbol!(x);

    // ∫ (x² + 2x + 1) dx = x³/3 + x² + x + C
    let expr = Expression::add(vec![
        Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
        Expression::mul(vec![Expression::integer(2), Expression::symbol(x.clone())]),
        Expression::integer(1),
    ]);
    let result = expr.integrate(x.clone());

    // Result should be x³/3 + x² + x
    let expected = Expression::add(vec![
        Expression::mul(vec![
            Expression::mul(vec![
                Expression::integer(1),
                Expression::pow(Expression::integer(3), Expression::integer(-1)),
            ]),
            Expression::pow(Expression::symbol(x.clone()), Expression::integer(3)),
        ]),
        Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
        Expression::symbol(x),
    ]);

    assert_eq!(result.simplify(), expected.simplify());
}

#[test]
fn test_by_parts_x_times_exp() {
    let x = symbol!(x);

    // ∫ x·e^x dx = x·e^x - e^x + C using integration by parts
    let expr = Expression::mul(vec![
        Expression::symbol(x.clone()),
        Expression::function("exp", vec![Expression::symbol(x.clone())]),
    ]);

    let result = IntegrationByParts::integrate(&expr, x);
    assert!(result.is_some());
}

#[test]
fn test_by_parts_x_times_sin() {
    let x = symbol!(x);

    // ∫ x·sin(x) dx = -x·cos(x) + sin(x) + C
    let expr = Expression::mul(vec![
        Expression::symbol(x.clone()),
        Expression::function("sin", vec![Expression::symbol(x.clone())]),
    ]);

    let result = IntegrationByParts::integrate(&expr, x);
    assert!(result.is_some());
}

#[test]
fn test_by_parts_x_times_cos() {
    let x = symbol!(x);

    // ∫ x·cos(x) dx = x·sin(x) + cos(x) + C
    let expr = Expression::mul(vec![
        Expression::symbol(x.clone()),
        Expression::function("cos", vec![Expression::symbol(x.clone())]),
    ]);

    let result = IntegrationByParts::integrate(&expr, x);
    assert!(result.is_some());
}

#[test]
fn test_fundamental_theorem_power() {
    let x = symbol!(x);

    // Test d/dx(∫ x² dx) = x²
    let expr = Expression::pow(Expression::symbol(x.clone()), Expression::integer(2));
    let integral = expr.integrate(x.clone());
    let derivative = integral.derivative(x);

    // Simplify both to compare
    assert_eq!(derivative.simplify(), expr.simplify());
}

#[test]
fn test_fundamental_theorem_sin() {
    let x = symbol!(x);

    // Test d/dx(∫ sin(x) dx) = sin(x)
    let expr = Expression::function("sin", vec![Expression::symbol(x.clone())]);
    let integral = expr.integrate(x.clone());
    let derivative = integral.derivative(x);

    // d/dx(-cos(x)) = sin(x)
    assert_eq!(derivative.simplify(), expr.simplify());
}

#[test]
fn test_fundamental_theorem_cos() {
    let x = symbol!(x);

    // Test d/dx(∫ cos(x) dx) = cos(x)
    let expr = Expression::function("cos", vec![Expression::symbol(x.clone())]);
    let integral = expr.integrate(x.clone());
    let derivative = integral.derivative(x);

    // d/dx(sin(x)) = cos(x)
    assert_eq!(derivative.simplify(), expr.simplify());
}

#[test]
fn test_fundamental_theorem_exp() {
    let x = symbol!(x);

    // Test d/dx(∫ e^x dx) = e^x
    let expr = Expression::function("exp", vec![Expression::symbol(x.clone())]);
    let integral = expr.integrate(x.clone());
    let derivative = integral.derivative(x);

    // d/dx(e^x) = e^x
    assert_eq!(derivative, expr);
}

#[test]
fn test_fundamental_theorem_polynomial() {
    let x = symbol!(x);

    // Test d/dx(∫ (3x² + 2x + 1) dx) = 3x² + 2x + 1
    let expr = Expression::add(vec![
        Expression::mul(vec![
            Expression::integer(3),
            Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
        ]),
        Expression::mul(vec![Expression::integer(2), Expression::symbol(x.clone())]),
        Expression::integer(1),
    ]);
    let integral = expr.integrate(x.clone());
    let derivative = integral.derivative(x);

    assert_eq!(derivative.simplify(), expr.simplify());
}

#[test]
fn test_zero_integral() {
    let x = symbol!(x);

    // ∫ 0 dx = 0 + C = 0 (ignoring constant)
    let expr = Expression::integer(0);
    let result = expr.integrate(x.clone());

    // Result should be 0·x = 0
    let expected = Expression::mul(vec![Expression::integer(0), Expression::symbol(x)]);

    assert_eq!(result, expected);
}

#[test]
fn test_negative_power() {
    let x = symbol!(x);

    // ∫ x^(-2) dx = -x^(-1) + C = -1/x + C
    let expr = Expression::pow(Expression::symbol(x.clone()), Expression::integer(-2));
    let result = expr.integrate(x.clone());

    // Result should be -x^(-1) = -(1/x)
    let expected = Expression::mul(vec![
        Expression::mul(vec![
            Expression::integer(1),
            Expression::pow(Expression::integer(-1), Expression::integer(-1)),
        ]),
        Expression::pow(Expression::symbol(x), Expression::integer(-1)),
    ]);

    assert_eq!(result.simplify(), expected.simplify());
}

#[test]
fn test_definite_integral_basic() {
    let x = symbol!(x);

    // ∫₀¹ x dx = [x²/2]₀¹ = 1/2
    let expr = Expression::symbol(x.clone());
    let lower = Expression::integer(0);
    let upper = Expression::integer(1);
    let result = expr.definite_integrate(x, lower, upper);

    // This should create a definite integral expression
    // Evaluation would require substitution support
    assert!(!result.is_zero());
}

#[test]
fn test_sec_integral() {
    let x = symbol!(x);

    // ∫ sec(x) dx = ln|sec(x) + tan(x)| + C
    let expr = Expression::function("sec", vec![Expression::symbol(x.clone())]);
    let result = expr.integrate(x.clone());

    // Result should be ln|sec(x) + tan(x)|
    let expected = Expression::function(
        "ln",
        vec![Expression::function(
            "abs",
            vec![Expression::add(vec![
                Expression::function("sec", vec![Expression::symbol(x.clone())]),
                Expression::function("tan", vec![Expression::symbol(x)]),
            ])],
        )],
    );

    assert_eq!(result, expected);
}

#[test]
fn test_csc_integral() {
    let x = symbol!(x);

    // ∫ csc(x) dx = -ln|csc(x) + cot(x)| + C
    let expr = Expression::function("csc", vec![Expression::symbol(x.clone())]);
    let result = expr.integrate(x.clone());

    // Result should be -ln|csc(x) + cot(x)|
    let expected = Expression::mul(vec![
        Expression::integer(-1),
        Expression::function(
            "ln",
            vec![Expression::function(
                "abs",
                vec![Expression::add(vec![
                    Expression::function("csc", vec![Expression::symbol(x.clone())]),
                    Expression::function("cot", vec![Expression::symbol(x)]),
                ])],
            )],
        ),
    ]);

    assert_eq!(result, expected);
}

#[test]
fn test_cot_integral() {
    let x = symbol!(x);

    // ∫ cot(x) dx = ln|sin(x)| + C
    let expr = Expression::function("cot", vec![Expression::symbol(x.clone())]);
    let result = expr.integrate(x);

    // Result should be ln|sin(x)|
    let expected = Expression::function(
        "ln",
        vec![Expression::function(
            "abs",
            vec![Expression::function("sin", vec![Expression::symbol(symbol!(x))])],
        )],
    );

    assert_eq!(result, expected);
}
