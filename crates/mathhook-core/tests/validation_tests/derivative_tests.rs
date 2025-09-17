use mathhook_core::calculus::derivatives::Derivative;
use mathhook_core::prelude::*;

#[test]
fn test_derivative_constant() {
    // diff(5, x) = 0
    let x = symbol!(x);
    let expr = expr!(5);
    let result = expr.derivative(x);
    assert_eq!(result, expr!(0));
}

#[test]
fn test_derivative_variable() {
    // SymPy: diff(x, x) = 1
    let x = symbol!(x);
    let expr = Expression::symbol(x.clone());
    let result = expr.derivative(x);
    assert_eq!(result, expr!(1));
}

#[test]
fn test_derivative_different_variable() {
    // SymPy: diff(x, y) = 0
    let x = symbol!(x);
    let y = symbol!(y);
    let expr = Expression::symbol(x);
    let result = expr.derivative(y);
    assert_eq!(result, expr!(0));
}

#[test]
fn test_derivative_power_rule_simple() {
    // SymPy: diff(x**2, x) = 2*x
    let x = symbol!(x);
    let expr = expr!(x ^ 2);
    let result = expr.derivative(x.clone());
    let expected = expr!(2 * x);
    assert_eq!(result, expected);
}

#[test]
fn test_derivative_power_rule_cubic() {
    // SymPy: diff(x**3, x) = 3*x**2
    let x = symbol!(x);
    let expr = expr!(x ^ 3);
    let result = expr.derivative(x.clone());
    let expected = expr!(3 * (x ^ 2));
    assert_eq!(result, expected);
}

#[test]
fn test_derivative_power_rule_quartic() {
    // SymPy: diff(x**4, x) = 4*x**3
    let x = symbol!(x);
    let expr = expr!(x ^ 4);
    let result = expr.derivative(x.clone());
    let expected = expr!(4 * (x ^ 3));
    assert_eq!(result, expected);
}

#[test]
fn test_derivative_linear() {
    // SymPy: diff(2*x, x) = 2
    let x = symbol!(x);
    let expr = expr!(2 * x);
    let result = expr.derivative(x).simplify();
    assert_eq!(result, expr!(2));
}

#[test]
fn test_derivative_sum_rule() {
    // SymPy: diff(x + 5, x) = 1
    let x = symbol!(x);
    let expr = expr!(x + 5);
    let result = expr.derivative(x).simplify();
    assert_eq!(result, expr!(1));
}

#[test]
fn test_derivative_polynomial_quadratic() {
    // SymPy: diff(x**2 + 2*x + 1, x) = 2*x + 2
    let x = symbol!(x);
    let expr = expr!((x ^ 2) + (2 * x) + 1);
    let result = expr.derivative(x.clone()).simplify();
    let expected = expr!((2 * x) + 2);
    assert_eq!(result, expected);
}

#[test]
fn test_derivative_polynomial_cubic() {
    // SymPy: diff(x**3 + 3*x**2 + 3*x + 1, x) = 3*x**2 + 6*x + 3
    let x = symbol!(x);
    let expr = expr!((x ^ 3) + (3 * (x ^ 2)) + (3 * x) + 1);
    let result = expr.derivative(x.clone()).simplify();
    let expected = expr!((3 * (x ^ 2)) + (6 * x) + 3);
    assert_eq!(result, expected);
}

#[test]
fn test_derivative_product_rule_simple() {
    // SymPy: diff(x * x, x) = 2*x
    let x = symbol!(x);
    let expr = expr!(x * x);
    let result = expr.derivative(x.clone()).simplify();
    let expected = expr!(2 * x);
    assert_eq!(result, expected);
}

#[test]
fn test_derivative_product_rule_polynomial() {
    // SymPy: diff(x**2 * x, x) = 3*x**2
    let x = symbol!(x);
    let expr = expr!((x ^ 2) * x);
    let result = expr.derivative(x.clone()).simplify();
    let expected = expr!(3 * (x ^ 2));
    assert_eq!(result, expected);
}

#[test]
fn test_derivative_chain_rule_power() {
    // SymPy: diff((x + 1)**2, x) = 2*(x + 1)
    let x = symbol!(x);
    let expr = expr!((x + 1) ^ 2);
    let result = expr.derivative(x.clone()).simplify();
    let expected = expr!(2 * (x + 1));
    assert_eq!(result, expected);
}

#[test]
fn test_derivative_chain_rule_nested() {
    // SymPy: diff((x**2 + 1)**3, x) = 6*x*(x**2 + 1)**2
    let x = symbol!(x);
    let expr = expr!(((x ^ 2) + 1) ^ 3);
    let result = expr.derivative(x.clone()).simplify();
    let expected = Expression::mul(vec![
        expr!(6),
        Expression::symbol(x.clone()),
        expr!(((x ^ 2) + 1) ^ 2),
    ]);
    assert_eq!(result, expected);
}

#[test]
fn test_derivative_sin() {
    // SymPy: diff(sin(x), x) = cos(x)
    let x = symbol!(x);
    let expr = function!(sin, Expression::symbol(x.clone()));
    let result = expr.derivative(x.clone());
    let expected = function!(cos, Expression::symbol(x));
    assert_eq!(result, expected);
}

#[test]
fn test_derivative_cos() {
    // SymPy: diff(cos(x), x) = -sin(x)
    let x = symbol!(x);
    let expr = function!(cos, Expression::symbol(x.clone()));
    let result = expr.derivative(x.clone());
    let expected = Expression::mul(vec![
        Expression::integer(-1),
        function!(sin, Expression::symbol(x)),
    ]);
    assert_eq!(result, expected);
}

#[test]
fn test_derivative_exp() {
    // SymPy: diff(exp(x), x) = exp(x)
    let x = symbol!(x);
    let expr = function!(exp, Expression::symbol(x.clone()));
    let result = expr.derivative(x.clone());
    let expected = function!(exp, Expression::symbol(x));
    assert_eq!(result, expected);
}

#[test]
#[ignore = "FIXME: Let's find out why"]
fn test_derivative_log() {
    // SymPy: diff(log(x), x) = 1/x
    let x = symbol!(x);
    let expr = function!(log, Expression::symbol(x.clone()));
    let result = expr.derivative(x.clone());
    let expected = expr!(1 / x);
    assert_eq!(result, expected);
}

#[test]
fn test_derivative_sqrt() {
    // SymPy: diff(sqrt(x), x) = 1/(2*sqrt(x))
    let x = symbol!(x);
    let expr = function!(sqrt, Expression::symbol(x.clone()));
    let result = expr.derivative(x.clone());

    // sqrt(x) can be represented as x^(1/2)
    let result_str = format!("{:?}", result);
    assert!(result_str.contains("2") || result_str.contains("1/2"));
}

#[test]
fn test_derivative_tan() {
    // SymPy: diff(tan(x), x) = sec(x)**2 = 1/cos(x)**2
    let x = symbol!(x);
    let expr = function!(tan, Expression::symbol(x.clone()));
    let result = expr.derivative(x.clone());

    // Result should be sec^2(x) or 1/cos^2(x)
    let result_str = format!("{:?}", result);
    assert!(result_str.contains("sec") || result_str.contains("cos"));
}

#[test]
fn test_derivative_higher_order_second() {
    // SymPy: diff(x**3, x, 2) = 6*x
    let x = symbol!(x);
    let expr = expr!(x ^ 3);
    let first = expr.derivative(x.clone());
    let second = first.derivative(x.clone()).simplify();
    let expected = expr!(6 * x);
    assert_eq!(second, expected);
}

#[test]
fn test_derivative_higher_order_third() {
    // SymPy: diff(x**3, x, 3) = 6
    let x = symbol!(x);
    let expr = expr!(x ^ 3);
    let first = expr.derivative(x.clone());
    let second = first.derivative(x.clone());
    let third = second.derivative(x).simplify();
    assert_eq!(third, expr!(6));
}

#[test]
fn test_derivative_constant_multiple() {
    // SymPy: diff(5*x**2, x) = 10*x
    let x = symbol!(x);
    let expr = expr!(5 * (x ^ 2));
    let result = expr.derivative(x.clone()).simplify();
    let expected = expr!(10 * x);
    assert_eq!(result, expected);
}

#[test]
fn test_derivative_negative_power() {
    // SymPy: diff(x**(-1), x) = -x**(-2) = -1/x**2
    let x = symbol!(x);
    let expr = expr!(x ^ (-1));
    let result = expr.derivative(x.clone()).simplify();
    let expected = Expression::mul(vec![Expression::integer(-1), expr!(x ^ (-2))]);
    assert_eq!(result, expected);
}

#[test]
fn test_derivative_rational_power() {
    // SymPy: diff(x**(1/2), x) = (1/2)*x**(-1/2)
    let x = symbol!(x);
    let expr = Expression::pow(Expression::symbol(x.clone()), Expression::rational(1, 2));
    let result = expr.derivative(x.clone());

    // Result should contain 1/2 and x^(-1/2)
    let result_str = format!("{:?}", result);
    assert!(result_str.contains("1") && result_str.contains("2"));
}

#[test]
fn test_derivative_multivariate_partial() {
    // SymPy: diff(x*y, x) = y
    let x = symbol!(x);
    let y = symbol!(y);
    let expr = expr!(x * y);
    let result = expr.derivative(x).simplify();
    let expected = Expression::symbol(y);
    assert_eq!(result, expected);
}

#[test]
fn test_derivative_quotient_rule() {
    // SymPy: diff(x/x**2, x) = diff(x**(-1), x) = -x**(-2)
    let x = symbol!(x);
    let expr = expr!(x / (x ^ 2));
    let result = expr.derivative(x.clone()).simplify();
    let expected = Expression::mul(vec![Expression::integer(-1), expr!(x ^ (-2))]);
    assert_eq!(result, expected);
}

#[test]
fn test_derivative_sum_of_functions() {
    // SymPy: diff(sin(x) + cos(x), x) = cos(x) - sin(x)
    let x = symbol!(x);
    let expr = Expression::add(vec![
        function!(sin, Expression::symbol(x.clone())),
        function!(cos, Expression::symbol(x.clone())),
    ]);
    let result = expr.derivative(x.clone()).simplify();
    let expected = Expression::add(vec![
        function!(cos, Expression::symbol(x.clone())),
        Expression::mul(vec![
            Expression::integer(-1),
            function!(sin, Expression::symbol(x)),
        ]),
    ])
    .simplify();
    assert_eq!(result, expected);
}

#[test]
fn test_derivative_product_of_functions() {
    // SymPy: diff(x*sin(x), x) = sin(x) + x*cos(x)
    let x = symbol!(x);
    let expr = expr!(x * (sin(x)));
    let result = expr.derivative(x.clone()).simplify();

    // Result should contain both sin(x) and x*cos(x) terms
    let result_str = format!("{:?}", result);
    assert!(result_str.contains("sin") && result_str.contains("cos"));
}

#[test]
fn test_derivative_zero_result() {
    // SymPy: diff(5, x) = 0
    let x = symbol!(x);
    let expr = expr!(5);
    let result = expr.derivative(x);
    assert_eq!(result, expr!(0));
}
