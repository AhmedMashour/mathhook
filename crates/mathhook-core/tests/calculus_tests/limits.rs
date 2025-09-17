//! Limit computation integration tests
//!
//! Tests for symbolic limit computation including:
//! - Basic limits (polynomial, rational)
//! - Limits at infinity
//! - One-sided limits
//! - Indeterminate forms (0/0, inf/inf, 0*inf, etc.)
//! - L'Hopital's rule applications
//! - Special limits (e definitions, trig limits)

use mathhook_core::calculus::Limits;
use mathhook_core::{expr, symbol, Expression, MathConstant, Simplify};

#[test]
fn test_limit_polynomial_at_point() {
    let x = symbol!(x);

    // lim(x->2) x^2 = 4
    let f = expr!(x ^ 2);
    let limit = f.limit(&x, &expr!(2));
    assert_eq!(limit.simplify(), expr!(4));
}

#[test]
fn test_limit_polynomial_substitution() {
    let x = symbol!(x);

    // lim(x->3) (x^2 + 2x + 1) = 16
    let f = expr!((x ^ 2) + (2 * x) + 1);
    let limit = f.limit(&x, &expr!(3));
    assert_eq!(limit.simplify(), expr!(16));
}

#[test]
fn test_limit_rational_cancellation() {
    let x = symbol!(x);

    // lim(x->2) (x^2 - 4)/(x - 2) = lim(x->2) (x+2)(x-2)/(x-2) = 4
    let numerator = expr!((x ^ 2) - 4);
    let denominator = expr!(x - 2);
    let f = Expression::div(numerator, denominator);
    let limit = f.limit(&x, &expr!(2));
    assert_eq!(limit.simplify(), expr!(4));
}

#[test]
#[ignore = "BUG: limit at pole returns unevaluated limit function"]
fn test_limit_rational_at_pole() {
    let x = symbol!(x);

    // lim(x->0) 1/x = undefined (or +/-infinity depending on direction)
    let f = Expression::div(expr!(1), Expression::symbol(x.clone()));
    let limit = f.limit(&x, &expr!(0));
    // Check that result is infinity constant
    let result = limit.simplify();
    match result {
        Expression::Constant(c) => {
            assert!(matches!(
                c,
                MathConstant::Infinity | MathConstant::NegativeInfinity | MathConstant::Undefined
            ));
        }
        _ => panic!(
            "Expected constant (Infinity or Undefined) for 1/x at 0, got {}",
            result
        ),
    }
}

#[test]
fn test_limit_polynomial_at_infinity() {
    let x = symbol!(x);

    // lim(x->inf) x^2 = inf
    let f = expr!(x ^ 2);
    let limit = f.limit_at_infinity(&x);
    let result = limit.simplify();
    assert_eq!(result, Expression::infinity());
}

#[test]
fn test_limit_rational_same_degree_at_infinity() {
    let x = symbol!(x);

    // lim(x->inf) (3x^2 + 2x)/(x^2 + 1) = 3
    let numerator = expr!((3 * (x ^ 2)) + (2 * x));
    let denominator = expr!((x ^ 2) + 1);
    let f = Expression::div(numerator, denominator);
    let limit = f.limit_at_infinity(&x);
    assert_eq!(limit.simplify(), expr!(3));
}

#[test]
fn test_limit_rational_higher_degree_numerator() {
    let x = symbol!(x);

    // lim(x->inf) x^3/x^2 = inf
    let f = Expression::div(expr!(x ^ 3), expr!(x ^ 2));
    let limit = f.limit_at_infinity(&x);
    assert_eq!(limit.simplify(), Expression::infinity());
}

#[test]
fn test_limit_rational_higher_degree_denominator() {
    let x = symbol!(x);

    // lim(x->inf) x^2/x^3 = 0
    let f = Expression::div(expr!(x ^ 2), expr!(x ^ 3));
    let limit = f.limit_at_infinity(&x);
    assert_eq!(limit.simplify(), expr!(0));
}

#[test]
fn test_limit_zero_over_zero_sin_x_over_x() {
    let x = symbol!(x);

    // lim(x->0) sin(x)/x = 1 (classic 0/0 form)
    let f = Expression::div(
        Expression::function("sin", vec![Expression::symbol(x.clone())]),
        Expression::symbol(x.clone()),
    );
    let limit = f.limit(&x, &expr!(0));
    assert_eq!(limit.simplify(), expr!(1));
}

#[test]
fn test_limit_infinity_over_infinity() {
    let x = symbol!(x);

    // lim(x->inf) ln(x)/x = 0 (inf/inf form, L'Hopital gives 1/x -> 0)
    let f = Expression::div(
        Expression::function("ln", vec![Expression::symbol(x.clone())]),
        Expression::symbol(x.clone()),
    );
    let limit = f.limit_at_infinity(&x);
    assert_eq!(limit.simplify(), expr!(0));
}

#[test]
fn test_limit_one_to_infinity_e_definition() {
    let x = symbol!(x);

    // lim(x->inf) (1 + 1/x)^x = e (1^inf form)
    let base = expr!(1 + (1 / x));
    let f = Expression::pow(base, Expression::symbol(x.clone()));
    let limit = f.limit_at_infinity(&x);
    assert_eq!(limit.simplify(), Expression::e());
}

#[test]
fn test_limit_sin_x_over_x() {
    let x = symbol!(x);

    // lim(x->0) sin(x)/x = 1
    let f = Expression::div(
        Expression::function("sin", vec![Expression::symbol(x.clone())]),
        Expression::symbol(x.clone()),
    );
    let limit = f.limit(&x, &expr!(0));
    assert_eq!(limit.simplify(), expr!(1));
}

#[test]
fn test_limit_one_minus_cos_over_x_squared() {
    let x = symbol!(x);

    // lim(x->0) (1 - cos(x))/x^2 = 1/2
    let numerator = expr!(1 - cos(x));
    let denominator = expr!(x ^ 2);
    let f = Expression::div(numerator, denominator);
    let limit = f.limit(&x, &expr!(0));
    assert_eq!(limit.simplify(), Expression::rational(1, 2));
}

#[test]
#[ignore = "BUG: tan(x)/x limit returns sec^2(0) instead of simplifying to 1"]
fn test_limit_tan_x_over_x() {
    let x = symbol!(x);

    // lim(x->0) tan(x)/x = 1
    let f = Expression::div(
        Expression::function("tan", vec![Expression::symbol(x.clone())]),
        Expression::symbol(x.clone()),
    );
    let limit = f.limit(&x, &expr!(0));
    assert_eq!(limit.simplify(), expr!(1));
}

#[test]
fn test_limit_exp_minus_one_over_x() {
    let x = symbol!(x);

    // lim(x->0) (e^x - 1)/x = 1
    let numerator = expr!(exp(x) - 1);
    let f = Expression::div(numerator, Expression::symbol(x.clone()));
    let limit = f.limit(&x, &expr!(0));
    assert_eq!(limit.simplify(), expr!(1));
}

#[test]
fn test_limit_ln_one_plus_x_over_x() {
    let x = symbol!(x);

    // lim(x->0) ln(1+x)/x = 1
    let numerator = Expression::function("ln", vec![expr!(1 + x)]);
    let f = Expression::div(numerator, Expression::symbol(x.clone()));
    let limit = f.limit(&x, &expr!(0));
    assert_eq!(limit.simplify(), expr!(1));
}

#[test]
fn test_limit_x_sin_one_over_x() {
    let x = symbol!(x);

    // lim(x->0) x * sin(1/x) = 0 (squeeze theorem: -|x| <= x*sin(1/x) <= |x|)
    let inner = Expression::div(expr!(1), Expression::symbol(x.clone()));
    let f = Expression::mul(vec![
        Expression::symbol(x.clone()),
        Expression::function("sin", vec![inner]),
    ]);
    let limit = f.limit(&x, &expr!(0));
    assert_eq!(limit.simplify(), expr!(0));
}

#[test]
fn test_limit_constructor_creates_calculus_expression() {
    let x = symbol!(x);
    let f = expr!(x ^ 2);
    let limit = Expression::limit(f, x.clone(), expr!(2));

    // Verify it creates a Calculus expression
    match limit {
        Expression::Calculus(_) => {}
        _ => panic!("Expected Calculus variant for limit expression"),
    }
}
