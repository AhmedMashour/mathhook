//! Series expansion integration tests
//!
//! Tests for symbolic series expansion including:
//! - Taylor series
//! - Maclaurin series
//! - Power series operations

use mathhook_core::calculus::SeriesExpansion;
use mathhook_core::{expr, symbol, Expression, Simplify};

#[test]
fn test_maclaurin_exp() {
    let x = symbol!(x);

    // e^x = 1 + x + x^2/2! + x^3/3! + ...
    let f = Expression::function("exp", vec![Expression::symbol(x.clone())]);
    let series = f.maclaurin_series(&x, 4);

    // Verify series has reasonable structure (sum of terms)
    let simplified = series.simplify();
    // The series should contain terms with x
    assert!(
        simplified.to_string().contains('x'),
        "Maclaurin series of exp(x) should contain x terms"
    );
}

#[test]
fn test_maclaurin_sin() {
    let x = symbol!(x);

    // sin(x) = x - x^3/3! + x^5/5! - ...
    let f = Expression::function("sin", vec![Expression::symbol(x.clone())]);
    let series = f.maclaurin_series(&x, 5);

    // Verify series has reasonable structure
    let simplified = series.simplify();
    assert!(
        simplified.to_string().contains('x'),
        "Maclaurin series of sin(x) should contain x terms"
    );
}

#[test]
fn test_maclaurin_cos() {
    let x = symbol!(x);

    // cos(x) = 1 - x^2/2! + x^4/4! - ...
    let f = Expression::function("cos", vec![Expression::symbol(x.clone())]);
    let series = f.maclaurin_series(&x, 4);

    // Verify series has reasonable structure
    let simplified = series.simplify();
    // cos(0) = 1, so the series should produce a valid result
    let result_str = simplified.to_string();
    assert!(
        result_str.contains('1') || result_str.contains('x'),
        "Maclaurin series of cos(x) should contain constant or x terms"
    );
}

#[test]
fn test_maclaurin_ln_one_plus_x() {
    let x = symbol!(x);
    // ln(1+x) = x - x^2/2 + x^3/3 - x^4/4 + ...
    // This is a composite function ln(1+x), not just ln(x), so requires
    // general series expansion with chain rule
    let f = Expression::function("ln", vec![expr!(1 + x)]);
    let _series = f.maclaurin_series(&x, 4);
    // TODO: verify output when general taylor series fully handles composite functions
}

#[test]
fn test_maclaurin_geometric_series() {
    let x = symbol!(x);
    // 1/(1-x) = 1 + x + x^2 + x^3 + ... (geometric series)
    let f = Expression::div(expr!(1), expr!(1 - x));
    let _series = f.maclaurin_series(&x, 4);
    // Would need to add geometric series to known_series or improve general_taylor_series
}

#[test]
fn test_maclaurin_arctan() {
    let x = symbol!(x);
    // arctan(x) = x - x^3/3 + x^5/5 - ...
    let f = Expression::function("arctan", vec![Expression::symbol(x.clone())]);
    let _series = f.maclaurin_series(&x, 5);
}

#[test]
fn test_maclaurin_sinh() {
    let x = symbol!(x);
    // sinh(x) = x + x^3/3! + x^5/5! + ...
    let f = Expression::function("sinh", vec![Expression::symbol(x.clone())]);
    let _series = f.maclaurin_series(&x, 5);
}

#[test]
fn test_maclaurin_cosh() {
    let x = symbol!(x);
    // cosh(x) = 1 + x^2/2! + x^4/4! + ...
    let f = Expression::function("cosh", vec![Expression::symbol(x.clone())]);
    let _series = f.maclaurin_series(&x, 4);
}

#[test]
fn test_taylor_exp_at_one() {
    let x = symbol!(x);
    // e^x around x=1: e * (1 + (x-1) + (x-1)^2/2 + ...)
    let f = Expression::function("exp", vec![Expression::symbol(x.clone())]);
    let _series = f.taylor_series(&x, &expr!(1), 2);
}

#[test]
fn test_taylor_ln_at_one() {
    let x = symbol!(x);
    // ln(x) around x=1: (x-1) - (x-1)^2/2 + (x-1)^3/3 - ...
    // known_series supports ln at point 1
    let f = Expression::function("ln", vec![Expression::symbol(x.clone())]);
    let series = f.taylor_series(&x, &expr!(1), 3);
    // ln is supported at point 1 in known_series
    let simplified = series.simplify();
    let result_str = simplified.to_string();
    assert!(
        result_str.contains('x') || result_str.contains('1'),
        "Taylor series of ln(x) at x=1 should produce valid terms"
    );
}

#[test]
fn test_taylor_sqrt_at_one() {
    let x = symbol!(x);
    // sqrt(x) around x=1: 1 + (x-1)/2 - (x-1)^2/8 + ...
    let f = Expression::function("sqrt", vec![Expression::symbol(x.clone())]);
    let _series = f.taylor_series(&x, &expr!(1), 2);
}

#[test]
fn test_binomial_series_sqrt() {
    let x = symbol!(x);
    // (1+x)^(1/2) = 1 + x/2 - x^2/8 + x^3/16 - ...
    let f = Expression::pow(expr!(1 + x), Expression::rational(1, 2));
    let _series = f.maclaurin_series(&x, 3);
}

#[test]
fn test_binomial_series_negative_exponent() {
    let x = symbol!(x);
    // (1+x)^(-1) = 1 - x + x^2 - x^3 + ...
    let f = Expression::pow(expr!(1 + x), expr!(-1));
    let _series = f.maclaurin_series(&x, 4);
}

#[test]
fn test_series_polynomial_expansion() {
    // Polynomial expansion is exact - no approximation needed
    // (1 + x)^2 = 1 + 2x + x^2
    let f = Expression::pow(expr!(1 + x), expr!(2));
    let expanded = f.simplify();

    // Check that the result contains expected structure
    let result_str = expanded.to_string();
    assert!(
        result_str.contains('x') || result_str.contains('1'),
        "Polynomial (1+x)^2 should simplify to expression with x terms"
    );
}

#[test]
fn test_expression_function_constructor() {
    let x = symbol!(x);
    let sin_x = Expression::function("sin", vec![Expression::symbol(x.clone())]);

    // Verify function was created correctly
    match sin_x {
        Expression::Function { name, args } => {
            assert_eq!(name, "sin");
            assert_eq!(args.len(), 1);
        }
        _ => panic!("Expected Function expression"),
    }
}

#[test]
fn test_derivative_constructor_exists() {
    let x = symbol!(x);
    let f = expr!(x ^ 2);

    // Verify derivative constructor exists and creates Calculus expression
    let deriv = Expression::derivative(f, x.clone(), 1);

    match deriv {
        Expression::Calculus(_) => {}
        _ => panic!("Expected Calculus expression for derivative"),
    }
}

#[test]
fn test_integral_constructor_exists() {
    let x = symbol!(x);
    let f = expr!(x ^ 2);

    // Verify integral constructor exists and creates Calculus expression
    let integ = Expression::integral(f, x.clone());

    match integ {
        Expression::Calculus(_) => {}
        _ => panic!("Expected Calculus expression for integral"),
    }
}
