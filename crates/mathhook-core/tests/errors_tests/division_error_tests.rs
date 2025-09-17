//! Comprehensive tests for division by zero error handling
//!
//! Tests the integration of `Result<Expression, MathError>` for division operations.

use mathhook_core::{expr, symbol, Expression, MathError};

#[test]
fn test_div_constructor_symbolic() {
    let x = expr!(x);
    let y = expr!(y);

    let result = Expression::div(x.clone(), y.clone());

    assert!(matches!(result, Expression::Mul(_)));
}

#[test]
fn test_div_constructor_with_zero_denominator_symbolic() {
    let x = expr!(x);

    let result = Expression::div(x.clone(), Expression::integer(0));

    assert!(!result.is_zero());
}

#[test]
fn test_div_checked_valid_division() {
    let result = Expression::div_checked(Expression::integer(10), Expression::integer(2));

    assert!(result.is_ok());
    let expr_result = result.unwrap();
    assert!(!expr_result.is_zero());
}

#[test]
fn test_div_checked_zero_denominator() {
    let result = Expression::div_checked(Expression::integer(1), Expression::integer(0));

    assert!(matches!(result, Err(MathError::DivisionByZero)));
}

#[test]
fn test_div_checked_symbolic_nonzero() {
    let x = expr!(x);
    let y = expr!(y);

    let result = Expression::div_checked(x.clone(), y.clone());

    assert!(result.is_ok());
}

#[test]
fn test_div_checked_symbolic_zero_denominator() {
    let x = expr!(x);

    let result = Expression::div_checked(x.clone(), Expression::integer(0));

    assert!(matches!(result, Err(MathError::DivisionByZero)));
}

#[test]
fn test_pow_negative_zero_base() {
    let expr = Expression::pow(Expression::integer(0), Expression::integer(-2));

    let result = expr.evaluate();

    assert!(matches!(result, Err(MathError::DivisionByZero)));
}

#[test]
fn test_pow_positive_zero_base() {
    let expr = Expression::pow(Expression::integer(0), Expression::integer(2));

    let result = expr.evaluate();

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Expression::integer(0));
}

#[test]
fn test_rational_number_division_by_zero() {
    use mathhook_core::Number;

    let result = Number::integer(5) / Number::integer(0);

    assert!(matches!(result, Err(MathError::DivisionByZero)));
}

#[test]
fn test_rational_number_valid_division() {
    use mathhook_core::Number;

    let result = Number::integer(10) / Number::integer(2);

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Number::integer(5));
}

#[test]
fn test_rational_number_nonexact_division() {
    use mathhook_core::Number;

    let result = Number::integer(10) / Number::integer(3);

    assert!(result.is_ok());
    let num = result.unwrap();
    assert!(matches!(num, Number::Rational(_)));
}

#[test]
fn test_quadratic_solver_with_valid_coefficients() {
    use mathhook_core::algebra::solvers::{EquationSolver, QuadraticSolver, SolverResult};

    let x = symbol!(x);

    let equation = Expression::add(vec![
        Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
        Expression::mul(vec![Expression::integer(-5), Expression::symbol(x.clone())]),
        Expression::integer(6),
    ]);

    let solver = QuadraticSolver::new();
    let result = solver.solve(&equation, &x);

    assert!(matches!(result, SolverResult::Multiple(_)));
}

#[test]
fn test_expression_div_usage() {
    let x = symbol!(x);

    let expr = Expression::div(Expression::integer(2), Expression::symbol(x.clone()));

    assert!(matches!(expr, Expression::Mul(_)));
}

#[test]
fn test_expression_div_checked_usage() {
    let result = Expression::div_checked(Expression::integer(10), Expression::integer(5));

    assert!(result.is_ok());
    let expr_result = result.unwrap();

    assert!(!expr_result.is_zero());
}

#[test]
fn test_multiple_divisions_no_zero() {
    let x = expr!(x);
    let y = expr!(y);

    let expr1 = Expression::div_checked(x.clone(), y.clone());
    assert!(expr1.is_ok());

    let expr2 = Expression::div_checked(Expression::integer(10), Expression::integer(2));
    assert!(expr2.is_ok());

    let expr3 = Expression::div_checked(x.clone(), y.clone());
    assert!(expr3.is_ok());
}

#[test]
fn test_division_in_complex_expression() {
    let x = expr!(x);

    let numerator = Expression::add(vec![x.clone(), Expression::integer(1)]);

    let denominator = Expression::add(vec![x.clone(), Expression::integer(2)]);

    let result_expr = Expression::div(numerator, denominator);

    assert!(!result_expr.is_zero());
}

#[test]
fn test_zero_divided_by_nonzero() {
    let result = Expression::div_checked(Expression::integer(0), Expression::integer(5));

    assert!(result.is_ok());
}

#[test]
fn test_div_checked_error_message() {
    let result = Expression::div_checked(Expression::integer(1), Expression::integer(0));

    match result {
        Err(MathError::DivisionByZero) => {
            let error_msg = format!("{}", MathError::DivisionByZero);
            assert_eq!(error_msg, "Division by zero");
        }
        _ => panic!("Expected DivisionByZero error"),
    }
}
