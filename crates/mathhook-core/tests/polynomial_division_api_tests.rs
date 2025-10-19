use mathhook_core::algebra::gcd::PolynomialGcd;
use mathhook_core::{expr, symbol, Expression};

#[test]
fn test_div_polynomial_simple() {
    let x = symbol!(x);

    let dividend = expr!((x^2) - 1);
    let divisor = expr!(x - 1);
    let (_quotient, remainder) = dividend.div_polynomial(&divisor, &x);

    assert_eq!(remainder, Expression::integer(0));
}

#[test]
fn test_div_polynomial_with_remainder() {
    let x = symbol!(x);

    let dividend = expr!((x^2) + 1);
    let divisor = expr!(x - 1);
    let (_quotient, remainder) = dividend.div_polynomial(&divisor, &x);

    assert_eq!(remainder, Expression::integer(2));
}

#[test]
fn test_div_polynomial_exact() {
    let x = symbol!(x);

    let dividend = expr!(add: (x^2), (3*x), 2);
    let divisor = expr!(x + 1);
    let (_quotient, remainder) = dividend.div_polynomial(&divisor, &x);

    assert_eq!(remainder, Expression::integer(0));
}

#[test]
fn test_quo_polynomial() {
    let x = symbol!(x);

    let dividend = expr!((x^2) - 1);
    let divisor = expr!(x - 1);
    let quotient = dividend.quo_polynomial(&divisor, &x);

    assert!(!quotient.is_zero());
}

#[test]
fn test_rem_polynomial() {
    let x = symbol!(x);

    let dividend = expr!((x^2) + 1);
    let divisor = expr!(x - 1);
    let remainder = dividend.rem_polynomial(&divisor, &x);

    assert_eq!(remainder, Expression::integer(2));
}

#[test]
fn test_div_polynomial_higher_degree() {
    let x = symbol!(x);

    let dividend = Expression::add(vec![
        Expression::pow(Expression::symbol(x.clone()), Expression::integer(3)),
        Expression::mul(vec![Expression::integer(2), Expression::pow(Expression::symbol(x.clone()), Expression::integer(2))]),
        Expression::mul(vec![Expression::integer(-5), Expression::symbol(x.clone())]),
        Expression::integer(-6),
    ]);
    let divisor = expr!(x - 2);
    let (_quotient, _remainder) = dividend.div_polynomial(&divisor, &x);

    assert!(!_quotient.is_zero());
}

#[test]
fn test_div_polynomial_equal_degree() {
    let x = symbol!(x);

    let dividend = expr!(add: (x^2), (3*x), 2);
    let divisor = expr!((x^2) + 1);
    let (_quotient, _remainder) = dividend.div_polynomial(&divisor, &x);

    assert!(!_quotient.is_zero() || !_remainder.is_zero());
}

#[test]
fn test_div_polynomial_constant_divisor() {
    let x = symbol!(x);

    let dividend = expr!(add: (2*(x^2)), (4*x), 6);
    let divisor = Expression::integer(2);
    let (_quotient, remainder) = dividend.div_polynomial(&divisor, &x);

    assert_eq!(remainder, Expression::integer(0));
}

#[test]
fn test_div_polynomial_linear_divisor() {
    let x = symbol!(x);

    let dividend = expr!((x^3) - 1);
    let divisor = expr!(x - 1);
    let (_quotient, remainder) = dividend.div_polynomial(&divisor, &x);

    assert_eq!(remainder, Expression::integer(0));
}

#[test]
fn test_div_polynomial_quadratic_divisor() {
    let x = symbol!(x);

    let dividend = expr!((x^4) - 1);
    let divisor = expr!((x^2) - 1);
    let (_quotient, remainder) = dividend.div_polynomial(&divisor, &x);

    assert_eq!(remainder, Expression::integer(0));
}

#[test]
fn test_div_polynomial_zero_remainder() {
    let x = symbol!(x);

    let dividend = expr!(add: (x^2), (5*x), 6);
    let divisor = expr!(x + 2);
    let (_quotient, remainder) = dividend.div_polynomial(&divisor, &x);

    assert_eq!(remainder, Expression::integer(0));
}

#[test]
fn test_div_polynomial_identity() {
    let x = symbol!(x);

    let polynomial = expr!(add: (x^2), (5*x), 3);
    let (quotient, remainder) = polynomial.div_polynomial(&polynomial, &x);

    assert_eq!(quotient, Expression::integer(1));
    assert_eq!(remainder, Expression::integer(0));
}
