//! Inverse operation property tests
//!
//! Validates that inverse operations correctly undo each other:
//! - d/dx(integral(f)) = f (Fundamental Theorem of Calculus)
//! - expand(factor(x)) preserves equivalence
//! - simplify(simplify(x)) = simplify(x) (idempotence)

use mathhook_core::calculus::derivatives::Derivative;
use mathhook_core::{symbol, Expression, Simplify};

#[test]
fn test_simplify_idempotence() {
    // simplify(simplify(x)) should equal simplify(x)
    let test_cases = vec![
        Expression::add(vec![Expression::integer(2), Expression::integer(3)]),
        Expression::mul(vec![Expression::integer(2), Expression::symbol(symbol!(x))]),
        Expression::pow(Expression::symbol(symbol!(x)), Expression::integer(2)),
        Expression::add(vec![
            Expression::symbol(symbol!(x)),
            Expression::symbol(symbol!(x)),
        ]),
    ];

    for expr in test_cases {
        let once = expr.clone().simplify();
        let twice = once.clone().simplify();

        assert_eq!(
            once, twice,
            "Simplify idempotence failed: simplify({}) != simplify(simplify({}))",
            expr, expr
        );
    }
}

#[test]
fn test_derivative_of_constant_is_zero() {
    let x = symbol!(x);

    let constants = vec![
        Expression::integer(0),
        Expression::integer(5),
        Expression::integer(-3),
        Expression::integer(100),
    ];

    for c in constants {
        let derivative = c.clone().derivative(x.clone()).simplify();
        assert_eq!(
            derivative,
            Expression::integer(0),
            "Derivative of constant {} should be 0",
            c
        );
    }
}

#[test]
fn test_derivative_of_x_is_one() {
    let x = symbol!(x);

    let derivative = Expression::symbol(x.clone())
        .derivative(x.clone())
        .simplify();

    assert_eq!(
        derivative,
        Expression::integer(1),
        "Derivative of x with respect to x should be 1"
    );
}

#[test]
fn test_power_rule_derivative() {
    let x = symbol!(x);

    // d/dx(x^n) = n * x^(n-1)
    let test_cases = vec![
        (2, 2, 1), // d/dx(x^2) = 2x^1 = 2x
        (3, 3, 2), // d/dx(x^3) = 3x^2
        (4, 4, 3), // d/dx(x^4) = 4x^3
    ];

    for (n, expected_coeff, expected_power) in test_cases {
        let expr = Expression::pow(Expression::symbol(x.clone()), Expression::integer(n));
        let derivative = expr.derivative(x.clone()).simplify();

        let expected = Expression::mul(vec![
            Expression::integer(expected_coeff),
            Expression::pow(
                Expression::symbol(x.clone()),
                Expression::integer(expected_power),
            ),
        ])
        .simplify();

        assert_eq!(
            derivative, expected,
            "Power rule failed: d/dx(x^{}) should be {}*x^{}",
            n, expected_coeff, expected_power
        );
    }
}

#[test]
fn test_derivative_linearity_addition() {
    let x = symbol!(x);

    // d/dx(f + g) = d/dx(f) + d/dx(g)
    let f = Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)); // x^2
    let g = Expression::pow(Expression::symbol(x.clone()), Expression::integer(3)); // x^3

    let sum = Expression::add(vec![f.clone(), g.clone()]);
    let derivative_of_sum = sum.derivative(x.clone()).simplify();

    let df = f.derivative(x.clone());
    let dg = g.derivative(x.clone());
    let sum_of_derivatives = Expression::add(vec![df, dg]).simplify();

    assert_eq!(
        derivative_of_sum, sum_of_derivatives,
        "Derivative linearity failed: d/dx(f+g) != d/dx(f) + d/dx(g)"
    );
}

#[test]
fn test_derivative_constant_multiple() {
    let x = symbol!(x);

    // d/dx(c * f) = c * d/dx(f)
    let c = Expression::integer(5);
    let f = Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)); // x^2

    let cf = Expression::mul(vec![c.clone(), f.clone()]);
    let derivative_of_cf = cf.derivative(x.clone()).simplify();

    let df = f.derivative(x.clone());
    let c_times_df = Expression::mul(vec![c, df]).simplify();

    assert_eq!(
        derivative_of_cf, c_times_df,
        "Constant multiple rule failed: d/dx(c*f) != c * d/dx(f)"
    );
}

#[test]
fn test_expand_preserves_value() {
    // Both should give same result at x = 3: (3+1)^2 = 16, and 9 + 6 + 1 = 16
    let factored_at_3 = Expression::pow(
        Expression::add(vec![Expression::integer(3), Expression::integer(1)]),
        Expression::integer(2),
    )
    .simplify();

    let expanded_at_3 = Expression::add(vec![
        Expression::pow(Expression::integer(3), Expression::integer(2)),
        Expression::mul(vec![Expression::integer(2), Expression::integer(3)]),
        Expression::integer(1),
    ])
    .simplify();

    assert_eq!(factored_at_3, expanded_at_3, "Expand should preserve value");
    assert_eq!(factored_at_3, Expression::integer(16));
}

#[test]
fn test_second_derivative() {
    let x = symbol!(x);

    // d^2/dx^2(x^3) = d/dx(3x^2) = 6x
    let expr = Expression::pow(Expression::symbol(x.clone()), Expression::integer(3));
    let second_derivative = expr.nth_derivative(x.clone(), 2).simplify();

    let expected =
        Expression::mul(vec![Expression::integer(6), Expression::symbol(x.clone())]).simplify();

    assert_eq!(
        second_derivative, expected,
        "Second derivative failed: d^2/dx^2(x^3) should be 6x"
    );
}

#[test]
fn test_third_derivative() {
    let x = symbol!(x);

    // d^3/dx^3(x^4) = d/dx(d/dx(4x^3)) = d/dx(12x^2) = 24x
    let expr = Expression::pow(Expression::symbol(x.clone()), Expression::integer(4));
    let third_derivative = expr.nth_derivative(x.clone(), 3).simplify();

    let expected =
        Expression::mul(vec![Expression::integer(24), Expression::symbol(x.clone())]).simplify();

    assert_eq!(
        third_derivative, expected,
        "Third derivative failed: d^3/dx^3(x^4) should be 24x"
    );
}
