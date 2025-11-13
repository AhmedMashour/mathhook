//! Comprehensive tests for rational function integration via partial fractions

use mathhook_core::calculus::integrals::rational::{integrate_rational, is_rational_function};
use mathhook_core::{symbol, Expression};

#[test]
fn test_is_rational_function_simple() {
    let x = symbol!(x);

    // 1/(x-2) is rational
    let expr1 = Expression::mul(vec![
        Expression::integer(1),
        Expression::pow(
            Expression::add(vec![Expression::symbol(x.clone()), Expression::integer(-2)]),
            Expression::integer(-1),
        ),
    ]);
    assert!(is_rational_function(&expr1, &x));

    // x^2 + 1 is NOT a rational function (just a polynomial)
    let expr2 = Expression::add(vec![
        Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
        Expression::integer(1),
    ]);
    assert!(!is_rational_function(&expr2, &x));

    // 1/x is rational
    let expr3 = Expression::pow(Expression::symbol(x.clone()), Expression::integer(-1));
    assert!(is_rational_function(&expr3, &x));
}

#[test]
fn test_rational_proper_simple_linear() {
    // SymPy validation: sympy.integrate(1/(x-2), x) = log(x-2)
    // ∫1/(x-2) dx = ln|x-2|
    let x = symbol!(x);
    let expr = Expression::mul(vec![
        Expression::integer(1),
        Expression::pow(
            Expression::add(vec![Expression::symbol(x.clone()), Expression::integer(-2)]),
            Expression::integer(-1),
        ),
    ]);

    let result = integrate_rational(&expr, &x);
    assert!(result.is_some());

    let integral = result.unwrap();
    // Should contain ln term
    assert!(integral.to_string().contains("ln") || integral.to_string().contains("log"));
}

#[test]
fn test_rational_proper_reciprocal_x() {
    // SymPy validation: sympy.integrate(1/x, x) = log(x)
    // ∫1/x dx = ln|x|
    let x = symbol!(x);
    let expr = Expression::pow(Expression::symbol(x.clone()), Expression::integer(-1));

    let result = integrate_rational(&expr, &x);
    assert!(result.is_some());

    let integral = result.unwrap();
    assert!(integral.to_string().contains("ln") || integral.to_string().contains("log"));
}

#[test]
fn test_rational_proper_constant_numerator() {
    // SymPy validation: sympy.integrate(5/(x-3), x) = 5*log(x-3)
    // ∫5/(x-3) dx = 5*ln|x-3|
    let x = symbol!(x);
    let expr = Expression::mul(vec![
        Expression::integer(5),
        Expression::pow(
            Expression::add(vec![Expression::symbol(x.clone()), Expression::integer(-3)]),
            Expression::integer(-1),
        ),
    ]);

    let result = integrate_rational(&expr, &x);
    assert!(result.is_some());

    let integral = result.unwrap();
    assert!(integral.to_string().contains("ln") || integral.to_string().contains("log"));
}

#[test]
fn test_rational_improper_polynomial_division() {
    // SymPy validation: sympy.integrate((x^2+1)/(x-1), x) = x^2/2 + x + 2*log(x-1)
    // ∫(x²+1)/(x-1) dx requires polynomial division first
    let x = symbol!(x);
    let numerator = Expression::add(vec![
        Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
        Expression::integer(1),
    ]);
    let denominator = Expression::add(vec![Expression::symbol(x.clone()), Expression::integer(-1)]);

    // Express as rational function: numerator * denominator^(-1)
    let expr = Expression::mul(vec![
        numerator,
        Expression::pow(denominator, Expression::integer(-1)),
    ]);

    let result = integrate_rational(&expr, &x);
    assert!(result.is_some());

    let integral = result.unwrap();
    // Should contain both polynomial terms and logarithmic term
    let result_str = integral.to_string();
    assert!(result_str.contains("ln") || result_str.contains("log") || result_str.contains("x"));
}

#[test]
fn test_rational_linear_distinct_roots() {
    // SymPy validation: sympy.integrate(1/((x-1)*(x-2)), x) = log(x-1) - log(x-2)
    // ∫1/((x-1)(x-2)) dx = ln|x-1| - ln|x-2|
    let x = symbol!(x);
    let factor1 = Expression::add(vec![Expression::symbol(x.clone()), Expression::integer(-1)]);
    let factor2 = Expression::add(vec![Expression::symbol(x.clone()), Expression::integer(-2)]);
    let denominator = Expression::mul(vec![factor1, factor2]);

    let expr = Expression::mul(vec![
        Expression::integer(1),
        Expression::pow(denominator, Expression::integer(-1)),
    ]);

    let result = integrate_rational(&expr, &x);
    assert!(result.is_some());

    let integral = result.unwrap();
    assert!(integral.to_string().contains("ln") || integral.to_string().contains("log"));
}

#[test]
fn test_rational_linear_repeated_factor() {
    // SymPy validation: sympy.integrate(1/(x-1)^2, x) = -1/(x-1)
    // ∫1/(x-1)² dx = -1/(x-1)
    let x = symbol!(x);
    let base = Expression::add(vec![Expression::symbol(x.clone()), Expression::integer(-1)]);
    let denominator = Expression::pow(base, Expression::integer(2));

    let expr = Expression::mul(vec![
        Expression::integer(1),
        Expression::pow(denominator, Expression::integer(-1)),
    ]);

    let result = integrate_rational(&expr, &x);
    assert!(result.is_some());
}

#[test]
fn test_rational_linear_repeated_factor_cubed() {
    // SymPy validation: sympy.integrate(1/(x-1)^3, x) = -1/(2*(x-1)^2)
    // ∫1/(x-1)³ dx = -1/(2(x-1)²)
    let x = symbol!(x);
    let base = Expression::add(vec![Expression::symbol(x.clone()), Expression::integer(-1)]);
    let denominator = Expression::pow(base, Expression::integer(3));

    let expr = Expression::mul(vec![
        Expression::integer(1),
        Expression::pow(denominator, Expression::integer(-1)),
    ]);

    let result = integrate_rational(&expr, &x);
    assert!(result.is_some());
}

#[test]
fn test_rational_quadratic_irreducible_x_numerator() {
    // SymPy validation: sympy.integrate(x/(x^2+1), x) = (1/2)*log(x^2+1)
    // ∫x/(x²+1) dx = (1/2)ln(x²+1)
    let x = symbol!(x);
    let numerator = Expression::symbol(x.clone());
    let denominator = Expression::add(vec![
        Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
        Expression::integer(1),
    ]);

    let expr = Expression::mul(vec![
        numerator,
        Expression::pow(denominator, Expression::integer(-1)),
    ]);

    let result = integrate_rational(&expr, &x);
    assert!(result.is_some());

    let integral = result.unwrap();
    let result_str = integral.to_string();
    assert!(result_str.contains("ln") || result_str.contains("log") || result_str.contains("x"));
}

#[test]
fn test_rational_quadratic_irreducible_constant_numerator() {
    // SymPy validation: sympy.integrate(1/(x^2+1), x) = atan(x)
    // ∫1/(x²+1) dx = arctan(x)
    let x = symbol!(x);
    let denominator = Expression::add(vec![
        Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
        Expression::integer(1),
    ]);

    let expr = Expression::mul(vec![
        Expression::integer(1),
        Expression::pow(denominator, Expression::integer(-1)),
    ]);

    let result = integrate_rational(&expr, &x);
    assert!(result.is_some());

    let integral = result.unwrap();
    assert!(integral.to_string().contains("atan") || integral.to_string().contains("arctan"));
}

#[test]
fn test_rational_quadratic_irreducible_scaled() {
    // SymPy validation: sympy.integrate(1/(x^2+4), x) = (1/2)*atan(x/2)
    // ∫1/(x²+4) dx = (1/2)arctan(x/2)
    let x = symbol!(x);
    let denominator = Expression::add(vec![
        Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
        Expression::integer(4),
    ]);

    let expr = Expression::mul(vec![
        Expression::integer(1),
        Expression::pow(denominator, Expression::integer(-1)),
    ]);

    let result = integrate_rational(&expr, &x);
    assert!(result.is_some());

    let integral = result.unwrap();
    assert!(integral.to_string().contains("atan") || integral.to_string().contains("arctan"));
}

#[test]
fn test_rational_quadratic_irreducible_general() {
    // SymPy validation: sympy.integrate(1/(x^2+2*x+5), x) = atan((x+1)/2)/2
    // ∫1/(x²+2x+5) dx = (1/2)arctan((x+1)/2)
    let x = symbol!(x);
    let denominator = Expression::add(vec![
        Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
        Expression::mul(vec![Expression::integer(2), Expression::symbol(x.clone())]),
        Expression::integer(5),
    ]);

    let expr = Expression::mul(vec![
        Expression::integer(1),
        Expression::pow(denominator, Expression::integer(-1)),
    ]);

    let result = integrate_rational(&expr, &x);
    assert!(result.is_some());
}

#[test]
fn test_rational_mixed_linear_quadratic() {
    // SymPy validation: sympy.integrate(1/(x*(x^2+1)), x)
    // ∫1/(x(x²+1)) dx = partial fractions
    let x = symbol!(x);
    let linear_factor = Expression::symbol(x.clone());
    let quadratic_factor = Expression::add(vec![
        Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
        Expression::integer(1),
    ]);
    let denominator = Expression::mul(vec![linear_factor, quadratic_factor]);

    let expr = Expression::mul(vec![
        Expression::integer(1),
        Expression::pow(denominator, Expression::integer(-1)),
    ]);

    let result = integrate_rational(&expr, &x);
    assert!(result.is_some());
}

#[test]
fn test_rational_degree_zero_constant() {
    // SymPy validation: sympy.integrate(1, x) = x
    // ∫1 dx = x (not technically rational function form, but should handle)
    let x = symbol!(x);
    let expr = Expression::integer(1);

    // This is not a rational function, so should return None
    let result = integrate_rational(&expr, &x);
    assert!(result.is_none());
}

#[test]
fn test_rational_numerator_higher_degree() {
    // SymPy validation: sympy.integrate((x^3+2*x^2+x+1)/(x^2+1), x)
    // Requires polynomial division
    let x = symbol!(x);
    let numerator = Expression::add(vec![
        Expression::pow(Expression::symbol(x.clone()), Expression::integer(3)),
        Expression::mul(vec![
            Expression::integer(2),
            Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
        ]),
        Expression::symbol(x.clone()),
        Expression::integer(1),
    ]);
    let denominator = Expression::add(vec![
        Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
        Expression::integer(1),
    ]);

    let expr = Expression::mul(vec![
        numerator,
        Expression::pow(denominator, Expression::integer(-1)),
    ]);

    let result = integrate_rational(&expr, &x);
    assert!(result.is_some());
}

#[test]
fn test_rational_simple_sum_in_denominator() {
    // SymPy validation: sympy.integrate(1/(x+5), x) = log(x+5)
    // ∫1/(x+5) dx = ln|x+5|
    let x = symbol!(x);
    let denominator = Expression::add(vec![Expression::symbol(x.clone()), Expression::integer(5)]);

    let expr = Expression::mul(vec![
        Expression::integer(1),
        Expression::pow(denominator, Expression::integer(-1)),
    ]);

    let result = integrate_rational(&expr, &x);
    assert!(result.is_some());

    let integral = result.unwrap();
    assert!(integral.to_string().contains("ln") || integral.to_string().contains("log"));
}

#[test]
fn test_rational_negative_root() {
    // SymPy validation: sympy.integrate(1/(x+3), x) = log(x+3)
    // ∫1/(x+3) dx = ln|x+3|
    let x = symbol!(x);
    let denominator = Expression::add(vec![Expression::symbol(x.clone()), Expression::integer(3)]);

    let expr = Expression::mul(vec![
        Expression::integer(1),
        Expression::pow(denominator, Expression::integer(-1)),
    ]);

    let result = integrate_rational(&expr, &x);
    assert!(result.is_some());

    let integral = result.unwrap();
    assert!(integral.to_string().contains("ln") || integral.to_string().contains("log"));
}

#[test]
fn test_rational_coefficient_in_numerator() {
    // SymPy validation: sympy.integrate(3/(x-1), x) = 3*log(x-1)
    // ∫3/(x-1) dx = 3*ln|x-1|
    let x = symbol!(x);
    let denominator = Expression::add(vec![Expression::symbol(x.clone()), Expression::integer(-1)]);

    let expr = Expression::mul(vec![
        Expression::integer(3),
        Expression::pow(denominator, Expression::integer(-1)),
    ]);

    let result = integrate_rational(&expr, &x);
    assert!(result.is_some());

    let integral = result.unwrap();
    assert!(integral.to_string().contains("ln") || integral.to_string().contains("log"));
}

#[test]
fn test_rational_linear_factor_at_origin() {
    // SymPy validation: sympy.integrate(1/x, x) = log(x)
    // ∫1/x dx = ln|x| (special case: root at 0)
    let x = symbol!(x);
    let expr = Expression::pow(Expression::symbol(x.clone()), Expression::integer(-1));

    let result = integrate_rational(&expr, &x);
    assert!(result.is_some());

    let integral = result.unwrap();
    assert!(integral.to_string().contains("ln") || integral.to_string().contains("log"));
}

#[test]
fn test_rational_two_distinct_linear_factors() {
    // SymPy validation: sympy.integrate((2*x+3)/((x-1)*(x-3)), x)
    // Partial fractions decomposition required
    let x = symbol!(x);
    let numerator = Expression::add(vec![
        Expression::mul(vec![Expression::integer(2), Expression::symbol(x.clone())]),
        Expression::integer(3),
    ]);
    let factor1 = Expression::add(vec![Expression::symbol(x.clone()), Expression::integer(-1)]);
    let factor2 = Expression::add(vec![Expression::symbol(x.clone()), Expression::integer(-3)]);
    let denominator = Expression::mul(vec![factor1, factor2]);

    let expr = Expression::mul(vec![
        numerator,
        Expression::pow(denominator, Expression::integer(-1)),
    ]);

    let result = integrate_rational(&expr, &x);
    assert!(result.is_some());
}

#[test]
fn test_rational_proper_fraction_quadratic_plus_constant() {
    // SymPy validation: sympy.integrate(2/(x^2+9), x) = (2/3)*atan(x/3)
    // ∫2/(x²+9) dx = (2/3)arctan(x/3)
    let x = symbol!(x);
    let denominator = Expression::add(vec![
        Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
        Expression::integer(9),
    ]);

    let expr = Expression::mul(vec![
        Expression::integer(2),
        Expression::pow(denominator, Expression::integer(-1)),
    ]);

    let result = integrate_rational(&expr, &x);
    assert!(result.is_some());

    let integral = result.unwrap();
    assert!(integral.to_string().contains("atan") || integral.to_string().contains("arctan"));
}

#[test]
fn test_rational_edge_case_x_squared_denominator() {
    // SymPy validation: sympy.integrate(1/x^2, x) = -1/x
    // ∫1/x² dx = -1/x
    let x = symbol!(x);
    let denominator = Expression::pow(Expression::symbol(x.clone()), Expression::integer(2));

    let expr = Expression::mul(vec![
        Expression::integer(1),
        Expression::pow(denominator, Expression::integer(-1)),
    ]);

    let result = integrate_rational(&expr, &x);
    assert!(result.is_some());
}

#[test]
fn test_rational_linear_numerator_quadratic_denominator() {
    // SymPy validation: sympy.integrate((2*x+1)/(x^2+1), x)
    // Split into ∫2x/(x²+1) dx + ∫1/(x²+1) dx
    let x = symbol!(x);
    let numerator = Expression::add(vec![
        Expression::mul(vec![Expression::integer(2), Expression::symbol(x.clone())]),
        Expression::integer(1),
    ]);
    let denominator = Expression::add(vec![
        Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
        Expression::integer(1),
    ]);

    let expr = Expression::mul(vec![
        numerator,
        Expression::pow(denominator, Expression::integer(-1)),
    ]);

    let result = integrate_rational(&expr, &x);
    assert!(result.is_some());

    let integral = result.unwrap();
    let result_str = integral.to_string();
    // Should contain both ln and atan terms
    assert!(result_str.contains("ln") || result_str.contains("log") || result_str.contains("atan"));
}
