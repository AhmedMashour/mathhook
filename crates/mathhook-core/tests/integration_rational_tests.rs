// Integration tests for rational function integrals

use mathhook_core::calculus::integrals::Integration;
use mathhook_core::calculus::integrals::rational::is_rational_function;
use mathhook_core::core::Number;
use mathhook_core::{symbol, Expression};
use num_bigint::BigInt;
use num_rational::BigRational;

#[test]
fn test_basic_polynomial() {
    let x = symbol!(x);

    // ∫x^2 dx
    let expr = Expression::pow(Expression::symbol(x.clone()), Expression::integer(2));

    assert!(is_rational_function(&expr, &x));

    let result = expr.integrate(x, 0);

    // Should be x^3/3
    assert!(!matches!(result, Expression::Calculus(_)));
}

#[test]
fn test_simple_rational_function() {
    let x = symbol!(x);

    // ∫1/x dx
    let expr = Expression::mul(vec![
        Expression::integer(1),
        Expression::pow(Expression::symbol(x.clone()), Expression::integer(-1)),
    ]);

    assert!(is_rational_function(&expr, &x));

    let result = expr.integrate(x, 0);

    // Should be ln|x|
    assert!(!matches!(result, Expression::Calculus(_)));
}

#[test]
fn test_rational_with_linear_denominator() {
    let x = symbol!(x);

    // ∫1/(x+1) dx
    let expr = Expression::mul(vec![
        Expression::integer(1),
        Expression::pow(
            Expression::add(vec![Expression::symbol(x.clone()), Expression::integer(1)]),
            Expression::integer(-1),
        ),
    ]);

    assert!(is_rational_function(&expr, &x));

    let result = expr.integrate(x, 0);

    // Should be ln|x+1|
    assert!(!matches!(result, Expression::Calculus(_)));
}

#[test]
fn test_proper_fraction() {
    let x = symbol!(x);

    // ∫1/(x^2+1) dx
    let expr = Expression::mul(vec![
        Expression::integer(1),
        Expression::pow(
            Expression::add(vec![
                Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
                Expression::integer(1),
            ]),
            Expression::integer(-1),
        ),
    ]);

    assert!(is_rational_function(&expr, &x));

    let result = expr.integrate(x, 0);

    // Should be atan(x)
    assert!(!matches!(result, Expression::Calculus(_)));
}

#[test]
fn test_improper_fraction() {
    let x = symbol!(x);

    // ∫(x^2+2x+3)/(x+1) dx = ∫(x+1+2/(x+1)) dx
    let numerator = Expression::add(vec![
        Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
        Expression::mul(vec![Expression::integer(2), Expression::symbol(x.clone())]),
        Expression::integer(3),
    ]);

    let denominator =
        Expression::add(vec![Expression::symbol(x.clone()), Expression::integer(1)]);

    let expr = Expression::mul(vec![numerator, Expression::pow(denominator, Expression::integer(-1))]);

    assert!(is_rational_function(&expr, &x));

    let result = expr.integrate(x, 0);

    // Should be integrable
    assert!(!matches!(result, Expression::Calculus(_)));
}

#[test]
fn test_partial_fraction_decomposition() {
    let x = symbol!(x);

    // ∫1/((x-1)(x-2)) dx
    let expr = Expression::mul(vec![
        Expression::integer(1),
        Expression::pow(
            Expression::mul(vec![
                Expression::add(vec![Expression::symbol(x.clone()), Expression::integer(-1)]),
                Expression::add(vec![Expression::symbol(x.clone()), Expression::integer(-2)]),
            ]),
            Expression::integer(-1),
        ),
    ]);

    assert!(is_rational_function(&expr, &x));

    let result = expr.integrate(x, 0);

    // Should be decomposable and integrable
    assert!(!matches!(result, Expression::Calculus(_)));
}

#[test]
fn test_rational_with_quadratic_factors() {
    let x = symbol!(x);

    // ∫1/(x^2-1) dx = ∫1/((x-1)(x+1)) dx
    let expr = Expression::mul(vec![
        Expression::integer(1),
        Expression::pow(
            Expression::add(vec![
                Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
                Expression::integer(-1),
            ]),
            Expression::integer(-1),
        ),
    ]);

    assert!(is_rational_function(&expr, &x));

    let result = expr.integrate(x, 0);

    // Should be decomposable
    assert!(!matches!(result, Expression::Calculus(_)));
}

#[test]
fn test_rational_zero_numerator() {
    let x = symbol!(x);

    // ∫0/x dx
    let expr = Expression::mul(vec![
        Expression::integer(0),
        Expression::pow(Expression::symbol(x.clone()), Expression::integer(-1)),
    ]);

    assert!(is_rational_function(&expr, &x));

    let result = expr.integrate(x, 0);

    // Should be 0
    assert!(!matches!(result, Expression::Calculus(_)));
}

#[test]
fn test_rational_constant_numerator() {
    let x = symbol!(x);

    // ∫5/(x+3) dx
    let expr = Expression::mul(vec![
        Expression::integer(5),
        Expression::pow(
            Expression::add(vec![Expression::symbol(x.clone()), Expression::integer(3)]),
            Expression::integer(-1),
        ),
    ]);

    assert!(is_rational_function(&expr, &x));

    let result = expr.integrate(x, 0);

    // Should be 5*ln|x+3|
    assert!(!matches!(result, Expression::Calculus(_)));
}

#[test]
fn test_complex_rational_function() {
    let x = symbol!(x);

    // ∫(2x+3)/(x^2+5x+6) dx
    let numerator = Expression::add(vec![
        Expression::mul(vec![Expression::integer(2), Expression::symbol(x.clone())]),
        Expression::integer(3),
    ]);

    let denominator = Expression::add(vec![
        Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
        Expression::mul(vec![Expression::integer(5), Expression::symbol(x.clone())]),
        Expression::integer(6),
    ]);

    let expr = Expression::mul(vec![numerator, Expression::pow(denominator, Expression::integer(-1))]);

    assert!(is_rational_function(&expr, &x));

    let result = expr.integrate(x, 0);

    // Should be integrable using partial fractions
    assert!(!matches!(result, Expression::Calculus(_)));
}

#[test]
fn test_non_rational_sqrt() {
    let x = symbol!(x);

    // √x is not a rational function
    let expr = Expression::pow(
        Expression::symbol(x.clone()),
        Expression::rational(1, 2),
    );

    assert!(!is_rational_function(&expr, &x));
}

#[test]
fn test_non_rational_trig() {
    let x = symbol!(x);

    // sin(x) is not a rational function
    let expr = Expression::function("sin", vec![Expression::symbol(x.clone())]);

    assert!(!is_rational_function(&expr, &x));
}

#[test]
fn test_non_rational_exp() {
    let x = symbol!(x);

    // e^x is not a rational function
    let expr = Expression::function("exp", vec![Expression::symbol(x.clone())]);

    assert!(!is_rational_function(&expr, &x));
}

#[test]
fn test_rational_polynomial_over_polynomial() {
    let x = symbol!(x);

    // ∫(x^3-3x^2+2x-5)/(x^2-4) dx
    let numerator = Expression::add(vec![
        Expression::pow(Expression::symbol(x.clone()), Expression::integer(3)),
        Expression::mul(vec![Expression::integer(-3), Expression::pow(Expression::symbol(x.clone()), Expression::integer(2))]),
        Expression::mul(vec![Expression::integer(2), Expression::symbol(x.clone())]),
        Expression::integer(-5),
    ]);

    let denominator = Expression::add(vec![
        Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
        Expression::integer(-4),
    ]);

    let expr = Expression::mul(vec![numerator, Expression::pow(denominator, Expression::integer(-1))]);

    assert!(is_rational_function(&expr, &x));

    let result = expr.integrate(x, 0);

    // Should be integrable
    assert!(!matches!(result, Expression::Calculus(_)));
}

#[test]
fn test_rational_simple_substitution() {
    let x = symbol!(x);

    // ∫x/(x^2+1) dx (requires simple substitution u = x^2+1)
    let expr = Expression::mul(vec![
        Expression::symbol(x.clone()),
        Expression::pow(
            Expression::add(vec![
                Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
                Expression::integer(1),
            ]),
            Expression::integer(-1),
        ),
    ]);

    assert!(is_rational_function(&expr, &x));

    let result = expr.integrate(x, 0);

    // Should be (1/2)*ln(x^2+1)
    assert!(!matches!(result, Expression::Calculus(_)));
}

#[test]
fn test_rational_degree_one_numerator() {
    let x = symbol!(x);

    // ∫x/(x-1) dx
    let expr = Expression::mul(vec![
        Expression::symbol(x.clone()),
        Expression::pow(
            Expression::add(vec![Expression::symbol(x.clone()), Expression::integer(-1)]),
            Expression::integer(-1),
        ),
    ]);

    assert!(is_rational_function(&expr, &x));

    let result = expr.integrate(x, 0);

    // Should be x + ln|x-1|
    assert!(!matches!(result, Expression::Calculus(_)));
}

#[test]
fn test_rational_repeated_linear_factor() {
    let x = symbol!(x);

    // ∫1/(x-1)^2 dx
    let expr = Expression::mul(vec![
        Expression::integer(1),
        Expression::pow(
            Expression::add(vec![Expression::symbol(x.clone()), Expression::integer(-1)]),
            Expression::integer(-2),
        ),
    ]);

    assert!(is_rational_function(&expr, &x));

    let result = expr.integrate(x, 0);

    // Should be -1/(x-1)
    assert!(!matches!(result, Expression::Calculus(_)));
}

#[test]
fn test_rational_with_irreducible_quadratic() {
    let x = symbol!(x);

    // ∫1/(x^2+x+1) dx
    let expr = Expression::mul(vec![
        Expression::integer(1),
        Expression::pow(
            Expression::add(vec![
                Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
                Expression::symbol(x.clone()),
                Expression::integer(1),
            ]),
            Expression::integer(-1),
        ),
    ]);

    assert!(is_rational_function(&expr, &x));

    let result = expr.integrate(x, 0);

    // Should be integrable
    assert!(!matches!(result, Expression::Calculus(_)));
}

#[test]
fn test_rational_numerator_equals_denominator_derivative() {
    let x = symbol!(x);

    // ∫(2x+1)/(x^2+x+1) dx
    let numerator = Expression::add(vec![
        Expression::mul(vec![Expression::integer(2), Expression::symbol(x.clone())]),
        Expression::integer(1),
    ]);

    let denominator = Expression::add(vec![
        Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
        Expression::symbol(x.clone()),
        Expression::integer(1),
    ]);

    let expr = Expression::mul(vec![numerator, Expression::pow(denominator, Expression::integer(-1))]);

    assert!(is_rational_function(&expr, &x));

    let result = expr.integrate(x, 0);

    // Should be ln|x^2+x+1|
    assert!(!matches!(result, Expression::Calculus(_)));
}

#[test]
fn test_rational_sum_integration() {
    let x = symbol!(x);

    // ∫(1/x + 1/(x+1)) dx
    let expr = Expression::add(vec![
        Expression::mul(vec![
            Expression::integer(1),
            Expression::pow(Expression::symbol(x.clone()), Expression::integer(-1)),
        ]),
        Expression::mul(vec![
            Expression::integer(1),
            Expression::pow(
                Expression::add(vec![Expression::symbol(x.clone()), Expression::integer(1)]),
                Expression::integer(-1),
            ),
        ]),
    ]);

    assert!(is_rational_function(&expr, &x));

    let result = expr.integrate(x, 0);

    // Should be ln|x| + ln|x+1|
    assert!(!matches!(result, Expression::Calculus(_)));
}

#[test]
fn test_rational_mixed_terms() {
    let x = symbol!(x);

    // ∫(x^2 + 1/x) dx
    let expr = Expression::add(vec![
        Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
        Expression::mul(vec![
            Expression::integer(1),
            Expression::pow(Expression::symbol(x.clone()), Expression::integer(-1)),
        ]),
    ]);

    assert!(is_rational_function(&expr, &x));

    let result = expr.integrate(x, 0);

    // Should be x^3/3 + ln|x|
    assert!(!matches!(result, Expression::Calculus(_)));
}

#[test]
fn test_rational_cubic_denominator() {
    let x = symbol!(x);

    // ∫1/(x^3-1) dx
    let expr = Expression::mul(vec![
        Expression::integer(1),
        Expression::pow(
            Expression::add(vec![
                Expression::pow(Expression::symbol(x.clone()), Expression::integer(3)),
                Expression::integer(-1),
            ]),
            Expression::integer(-1),
        ),
    ]);

    assert!(is_rational_function(&expr, &x));

    let result = expr.integrate(x, 0);

    // Should be decomposable
    assert!(!matches!(result, Expression::Calculus(_)));
}

#[test]
fn test_rational_fourth_degree_denominator() {
    let x = symbol!(x);

    // ∫1/(x^4-1) dx
    let expr = Expression::mul(vec![
        Expression::integer(1),
        Expression::pow(
            Expression::add(vec![
                Expression::pow(Expression::symbol(x.clone()), Expression::integer(4)),
                Expression::integer(-1),
            ]),
            Expression::integer(-1),
        ),
    ]);

    assert!(is_rational_function(&expr, &x));

    let result = expr.integrate(x, 0);

    // Should be decomposable
    assert!(!matches!(result, Expression::Calculus(_)));
}

#[test]
fn test_rational_complex_numerator_denominator() {
    let x = symbol!(x);

    // ∫(x^2+3x+2)/(x^3+x^2-2x) dx
    let numerator = Expression::add(vec![
        Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
        Expression::mul(vec![Expression::integer(3), Expression::symbol(x.clone())]),
        Expression::integer(2),
    ]);

    let denominator = Expression::add(vec![
        Expression::pow(Expression::symbol(x.clone()), Expression::integer(3)),
        Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
        Expression::mul(vec![
            Expression::integer(-2),
            Expression::symbol(x.clone()),
        ]),
    ]);

    let expr = Expression::mul(vec![numerator, Expression::pow(denominator, Expression::integer(-1))]);

    assert!(is_rational_function(&expr, &x));

    let result = expr.integrate(x, 0);

    // Should be integrable via partial fractions
    assert!(!matches!(result, Expression::Calculus(_)));
}

#[test]
fn test_rational_with_irreducible_quadratic_in_partial_fraction() {
    let x = symbol!(x);

    // ∫1/(x*(x^2+1)) dx
    let expr = Expression::mul(vec![
        Expression::integer(1),
        Expression::pow(
            Expression::mul(vec![
                Expression::symbol(x.clone()),
                Expression::add(vec![
                    Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
                    Expression::integer(1),
                ]),
            ]),
            Expression::integer(-1),
        ),
    ]);

    assert!(is_rational_function(&expr, &x));

    let integral = expr.integrate(x, 0);
    let result_str = integral.to_string();
    // Should contain both ln and atan terms
    assert!(result_str.contains("ln") || result_str.contains("log") || result_str.contains("atan"));
}

#[test]
fn test_sqrt_x_integration_full() {
    let x = symbol!(x);

    // ∫√x dx = ∫x^(1/2) dx
    let expr = Expression::pow(
        Expression::symbol(x.clone()),
        Expression::Number(Number::rational(BigRational::new(
            BigInt::from(1),
            BigInt::from(2),
        ))),
    );

    let result = expr.integrate(x, 0);

    // Should be (2/3)*x^(3/2)
    assert!(!matches!(result, Expression::Calculus(_)));
}
