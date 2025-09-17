//! Comprehensive tests for function!() macro

use mathhook_core::{Expression, Symbol};
use mathhook_macros::function;

// ============================================================================
// Zero-Argument Functions (5 tests)
// ============================================================================

#[test]
fn test_function_zero_args_simple() {
    let result = function!(gamma);
    assert_eq!(result, Expression::function("gamma", vec![]));
}

#[test]
fn test_function_zero_args_custom() {
    let result = function!(myfunc);
    assert_eq!(result, Expression::function("myfunc", vec![]));
}

#[test]
fn test_function_zero_args_uppercase() {
    let result = function!(GAMMA);
    assert_eq!(result, Expression::function("GAMMA", vec![]));
}

#[test]
fn test_function_zero_args_with_underscore() {
    let result = function!(my_func);
    assert_eq!(result, Expression::function("my_func", vec![]));
}

#[test]
fn test_function_zero_args_long_name() {
    let result = function!(calculate_special_value);
    assert_eq!(
        result,
        Expression::function("calculate_special_value", vec![])
    );
}

// ============================================================================
// Single-Argument Functions (10 tests)
// ============================================================================

#[test]
fn test_function_one_arg_symbol() {
    let x = Expression::symbol(Symbol::scalar("x"));
    let result = function!(sin, x.clone());
    assert_eq!(result, Expression::function("sin", vec![x]));
}

#[test]
fn test_function_one_arg_integer() {
    let n = Expression::integer(5);
    let result = function!(factorial, n.clone());
    assert_eq!(result, Expression::function("factorial", vec![n]));
}

#[test]
fn test_function_one_arg_float() {
    let val = Expression::float(2.728);
    let result = function!(round, val.clone());
    assert_eq!(result, Expression::function("round", vec![val]));
}

#[test]
fn test_function_one_arg_pi() {
    let pi = Expression::pi();
    let result = function!(cos, pi.clone());
    assert_eq!(result, Expression::function("cos", vec![pi]));
}

#[test]
fn test_function_one_arg_expression() {
    let expr = Expression::add(vec![
        Expression::symbol(Symbol::scalar("x")),
        Expression::integer(1),
    ]);
    let result = function!(sin, expr.clone());
    assert_eq!(result, Expression::function("sin", vec![expr]));
}

#[test]
fn test_function_one_arg_nested() {
    let inner = Expression::function("sin", vec![Expression::symbol(Symbol::scalar("x"))]);
    let result = function!(cos, inner.clone());
    assert_eq!(result, Expression::function("cos", vec![inner]));
}

#[test]
fn test_function_one_arg_power() {
    let expr = Expression::pow(
        Expression::symbol(Symbol::scalar("x")),
        Expression::integer(2),
    );
    let result = function!(sqrt, expr.clone());
    assert_eq!(result, Expression::function("sqrt", vec![expr]));
}

#[test]
fn test_function_one_arg_product() {
    let expr = Expression::mul(vec![
        Expression::integer(2),
        Expression::symbol(Symbol::scalar("x")),
    ]);
    let result = function!(sin, expr.clone());
    assert_eq!(result, Expression::function("sin", vec![expr]));
}

#[test]
fn test_function_one_arg_negative() {
    let expr = Expression::mul(vec![
        Expression::integer(-1),
        Expression::symbol(Symbol::scalar("x")),
    ]);
    let result = function!(abs, expr.clone());
    assert_eq!(result, Expression::function("abs", vec![expr]));
}

#[test]
fn test_function_one_arg_complex() {
    let expr = Expression::add(vec![
        Expression::mul(vec![
            Expression::integer(2),
            Expression::pow(
                Expression::symbol(Symbol::scalar("x")),
                Expression::integer(2),
            ),
        ]),
        Expression::integer(1),
    ]);
    let result = function!(simplify, expr.clone());
    assert_eq!(result, Expression::function("simplify", vec![expr]));
}

// ============================================================================
// Two-Argument Functions (10 tests)
// ============================================================================

#[test]
fn test_function_two_args_symbols() {
    let x = Expression::symbol(Symbol::scalar("x"));
    let y = Expression::symbol(Symbol::scalar("y"));
    let result = function!(log, x.clone(), y.clone());
    assert_eq!(result, Expression::function("log", vec![x, y]));
}

#[test]
fn test_function_two_args_integers() {
    let a = Expression::integer(10);
    let b = Expression::integer(2);
    let result = function!(pow, a.clone(), b.clone());
    assert_eq!(result, Expression::function("pow", vec![a, b]));
}

#[test]
fn test_function_two_args_mixed() {
    let x = Expression::symbol(Symbol::scalar("x"));
    let n = Expression::integer(2);
    let result = function!(pow, x.clone(), n.clone());
    assert_eq!(result, Expression::function("pow", vec![x, n]));
}

#[test]
fn test_function_two_args_expressions() {
    let expr1 = Expression::add(vec![
        Expression::symbol(Symbol::scalar("x")),
        Expression::integer(1),
    ]);
    let expr2 = Expression::add(vec![
        Expression::symbol(Symbol::scalar("y")),
        Expression::integer(1),
    ]);
    let result = function!(atan2, expr1.clone(), expr2.clone());
    assert_eq!(result, Expression::function("atan2", vec![expr1, expr2]));
}

#[test]
fn test_function_two_args_nested_functions() {
    let sin_x = Expression::function("sin", vec![Expression::symbol(Symbol::scalar("x"))]);
    let cos_y = Expression::function("cos", vec![Expression::symbol(Symbol::scalar("y"))]);
    let result = function!(max, sin_x.clone(), cos_y.clone());
    assert_eq!(result, Expression::function("max", vec![sin_x, cos_y]));
}

#[test]
fn test_function_two_args_constants() {
    let pi = Expression::pi();
    let e = Expression::e();
    let result = function!(log, pi.clone(), e.clone());
    assert_eq!(result, Expression::function("log", vec![pi, e]));
}

#[test]
fn test_function_two_args_same_expr() {
    let x = Expression::symbol(Symbol::scalar("x"));
    let result = function!(gcd, x.clone(), x.clone());
    assert_eq!(result, Expression::function("gcd", vec![x.clone(), x]));
}

#[test]
fn test_function_two_args_power_base_exp() {
    let base = Expression::symbol(Symbol::scalar("x"));
    let exp = Expression::integer(2);
    let result = function!(power, base.clone(), exp.clone());
    assert_eq!(result, Expression::function("power", vec![base, exp]));
}

#[test]
fn test_function_two_args_division() {
    let num = Expression::symbol(Symbol::scalar("a"));
    let den = Expression::symbol(Symbol::scalar("b"));
    let result = function!(div, num.clone(), den.clone());
    assert_eq!(result, Expression::function("div", vec![num, den]));
}

#[test]
fn test_function_two_args_modulo() {
    let a = Expression::integer(17);
    let b = Expression::integer(5);
    let result = function!(mod_, a.clone(), b.clone());
    assert_eq!(result, Expression::function("mod_", vec![a, b]));
}

// ============================================================================
// Three+ Argument Functions (10 tests)
// ============================================================================

#[test]
fn test_function_three_args() {
    let x = Expression::symbol(Symbol::scalar("x"));
    let y = Expression::symbol(Symbol::scalar("y"));
    let z = Expression::symbol(Symbol::scalar("z"));
    let result = function!(f, x.clone(), y.clone(), z.clone());
    assert_eq!(result, Expression::function("f", vec![x, y, z]));
}

#[test]
fn test_function_four_args() {
    let a = Expression::integer(1);
    let b = Expression::integer(2);
    let c = Expression::integer(3);
    let d = Expression::integer(4);
    let result = function!(sum, a.clone(), b.clone(), c.clone(), d.clone());
    assert_eq!(result, Expression::function("sum", vec![a, b, c, d]));
}

#[test]
fn test_function_five_args() {
    let args: Vec<Expression> = (1..=5).map(Expression::integer).collect();
    let result = function!(
        product,
        args[0].clone(),
        args[1].clone(),
        args[2].clone(),
        args[3].clone(),
        args[4].clone()
    );
    assert_eq!(result, Expression::function("product", args.clone()));
}

#[test]
fn test_function_six_args() {
    let args: Vec<Expression> = (1..=6).map(Expression::integer).collect();
    let result = function!(
        hypergeometric,
        args[0].clone(),
        args[1].clone(),
        args[2].clone(),
        args[3].clone(),
        args[4].clone(),
        args[5].clone()
    );
    assert_eq!(result, Expression::function("hypergeometric", args));
}

#[test]
fn test_function_mixed_types_three_args() {
    let x = Expression::symbol(Symbol::scalar("x"));
    let n = Expression::integer(5);
    let pi = Expression::pi();
    let result = function!(custom, x.clone(), n.clone(), pi.clone());
    assert_eq!(result, Expression::function("custom", vec![x, n, pi]));
}

#[test]
fn test_function_nested_as_arg() {
    let inner1 = Expression::function("sin", vec![Expression::symbol(Symbol::scalar("x"))]);
    let inner2 = Expression::function("cos", vec![Expression::symbol(Symbol::scalar("y"))]);
    let inner3 = Expression::function("tan", vec![Expression::symbol(Symbol::scalar("z"))]);
    let result = function!(combine, inner1.clone(), inner2.clone(), inner3.clone());
    assert_eq!(
        result,
        Expression::function("combine", vec![inner1, inner2, inner3])
    );
}

#[test]
fn test_function_expressions_three_args() {
    let expr1 = Expression::add(vec![
        Expression::symbol(Symbol::scalar("x")),
        Expression::integer(1),
    ]);
    let expr2 = Expression::mul(vec![
        Expression::integer(2),
        Expression::symbol(Symbol::scalar("y")),
    ]);
    let expr3 = Expression::pow(
        Expression::symbol(Symbol::scalar("z")),
        Expression::integer(2),
    );
    let result = function!(blend, expr1.clone(), expr2.clone(), expr3.clone());
    assert_eq!(
        result,
        Expression::function("blend", vec![expr1, expr2, expr3])
    );
}

#[test]
fn test_function_all_constants() {
    let pi = Expression::pi();
    let e = Expression::e();
    let i = Expression::i();
    let result = function!(combine_constants, pi.clone(), e.clone(), i.clone());
    assert_eq!(
        result,
        Expression::function("combine_constants", vec![pi, e, i])
    );
}

#[test]
fn test_function_repeated_args() {
    let x = Expression::symbol(Symbol::scalar("x"));
    let result = function!(triple, x.clone(), x.clone(), x.clone());
    assert_eq!(
        result,
        Expression::function("triple", vec![x.clone(), x.clone(), x])
    );
}

#[test]
fn test_function_many_integers() {
    let args: Vec<Expression> = vec![1, 2, 3, 4, 5, 6, 7, 8]
        .into_iter()
        .map(Expression::integer)
        .collect();
    let result = function!(
        octuple,
        args[0].clone(),
        args[1].clone(),
        args[2].clone(),
        args[3].clone(),
        args[4].clone(),
        args[5].clone(),
        args[6].clone(),
        args[7].clone()
    );
    assert_eq!(result, Expression::function("octuple", args));
}
