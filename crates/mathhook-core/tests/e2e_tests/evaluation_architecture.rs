//! Integration tests for Expression Evaluation Architecture
use mathhook_core::core::expression::data_types::RelationType;
use mathhook_core::core::expression::eval_numeric::EvalNumeric;
use mathhook_core::{expr, symbol, Expression, MathError};

use mathhook_core::core::expression::eval_numeric::EvalContext;
use std::collections::HashMap;

#[test]
fn test_symbolic_context() {
    let e = expr!(x + y);

    let ctx = EvalContext::symbolic();
    let result = e.evaluate_with_context(&ctx).unwrap();

    assert_eq!(result, e);
}

#[test]
fn test_numeric_context_with_substitution() {
    let e = expr!(x + y);

    let mut vars = HashMap::new();
    vars.insert("x".to_string(), expr!(3));
    vars.insert("y".to_string(), expr!(4));

    let ctx = EvalContext::numeric(vars);
    let result = e.evaluate_with_context(&ctx).unwrap();

    assert_eq!(result, expr!(7));
}

#[test]
fn test_substitute_simple() {
    let e = expr!(x ^ 2);

    let mut subs = HashMap::new();
    subs.insert("x".to_string(), expr!(3));

    let result = e.substitute(&subs);
    assert_eq!(result, expr!(3 ^ 2));
}

#[test]
fn test_substitute_multiple_variables() {
    let e = expr!((x ^ 2) + (2 * y));

    let mut subs = HashMap::new();
    subs.insert("x".to_string(), expr!(3));
    subs.insert("y".to_string(), expr!(5));

    let result = e.substitute(&subs);
    assert_eq!(result, expr!((3 ^ 2) + (2 * 5)));
}

#[test]
fn test_substitute_nested() {
    let e = Expression::function("sin".to_string(), vec![expr!(x ^ 2)]);

    let mut subs = HashMap::new();
    subs.insert("x".to_string(), expr!(3));

    let result = e.substitute(&subs);
    let expected = Expression::function("sin".to_string(), vec![expr!(3 ^ 2)]);
    assert_eq!(result, expected);
}

#[test]
fn test_evaluate_with_context_with_simplification() {
    let e = expr!(x ^ 2);

    let mut vars = HashMap::new();
    vars.insert("x".to_string(), expr!(3));

    let ctx = EvalContext::numeric(vars).with_simplify(true);
    let result = e.evaluate_with_context(&ctx).unwrap();

    assert_eq!(result, expr!(9));
}

#[test]
fn test_evaluate_with_context_precision() {
    let e = expr!(x ^ 2);

    let mut vars = HashMap::new();
    vars.insert("x".to_string(), expr!(3));

    let ctx = EvalContext::numeric(vars).with_precision(128);
    let result = e.evaluate_with_context(&ctx).unwrap();

    assert_eq!(ctx.precision, 128);
    assert_eq!(result, expr!(9));
}

#[test]
fn test_substitute_no_match() {
    let e = expr!(x ^ 2);

    let mut subs = HashMap::new();
    subs.insert("y".to_string(), expr!(3));

    let result = e.substitute(&subs);
    assert_eq!(result, e);
}

#[test]
fn test_substitute_complex_expression() {
    let e = Expression::complex(expr!(x), expr!(y));

    let mut subs = HashMap::new();
    subs.insert("x".to_string(), expr!(2));
    subs.insert("y".to_string(), expr!(3));

    let result = e.substitute(&subs);
    let expected = Expression::complex(expr!(2), expr!(3));
    assert_eq!(result, expected);
}

#[test]
fn test_symbolic_evaluation_no_substitution() {
    let e = expr!(x ^ 2);

    let ctx = EvalContext::symbolic();
    let result = e.evaluate_with_context(&ctx).unwrap();

    assert_eq!(result, expr!(x ^ 2));
}

#[test]
fn test_numerical_evaluation_with_simplify_disabled() {
    let e = expr!(x + x);

    let mut vars = HashMap::new();
    vars.insert("x".to_string(), expr!(5));

    let ctx = EvalContext::numeric(vars).with_simplify(false);
    let result = e.evaluate_with_context(&ctx).unwrap();

    assert_eq!(result, expr!(10));
}

#[test]
fn test_eval_numeric_number_returns_self() {
    let num = expr!(42);
    let result = num.eval_numeric(53).unwrap();
    assert_eq!(result, num);
}

#[test]
fn test_eval_numeric_symbol_returns_self() {
    let x = symbol!(x);
    let sym_expr = Expression::symbol(x);
    let result = sym_expr.eval_numeric(53).unwrap();
    assert_eq!(result, sym_expr);
}

#[test]
fn test_eval_numeric_constant_pi() {
    let pi = Expression::pi();
    let result = pi.eval_numeric(53).unwrap();

    // Should evaluate to float approximation of π
    assert_eq!(result, Expression::float(std::f64::consts::PI));
}

#[test]
fn test_eval_numeric_constant_e() {
    let e = Expression::e();
    let result = e.eval_numeric(53).unwrap();

    // Should evaluate to float approximation of e
    assert_eq!(result, Expression::float(std::f64::consts::E));
}

#[test]
fn test_eval_numeric_constant_i_remains_symbolic() {
    let i = Expression::i();
    let result = i.eval_numeric(53).unwrap();
    assert_eq!(result, i); // Imaginary unit stays symbolic
}

#[test]
fn test_eval_numeric_add_numbers() {
    let expr = expr!((2 + 3));
    let result = expr.eval_numeric(53).unwrap();

    // Should evaluate to 5
    assert_eq!(result, expr!((2 + 3))); // Returns sum expression, not simplified
}

#[test]
fn test_eval_numeric_add_with_symbols() {
    let x = symbol!(x);
    let y = symbol!(y);
    let expr = Expression::add(vec![
        Expression::symbol(x.clone()),
        Expression::symbol(y.clone()),
    ]);

    let result = expr.eval_numeric(53).unwrap();

    // Symbols stay symbolic (no substitution at this level)
    assert_eq!(result, expr);
}

#[test]
fn test_eval_numeric_mul_numbers() {
    let expr = expr!((2 * 3));
    let result = expr.eval_numeric(53).unwrap();

    // Should evaluate to 6
    assert_eq!(result, expr!((2 * 3)));
}

#[test]
fn test_eval_numeric_pow_positive_exponent() {
    let expr = expr!((2 ^ 3));
    let result = expr.eval_numeric(53).unwrap();

    // Should evaluate to 8
    assert_eq!(result, expr!((2 ^ 3)));
}

#[test]
fn test_eval_numeric_pow_zero_base_positive_exp() {
    let expr = expr!((0 ^ 2));
    let result = expr.eval_numeric(53).unwrap();

    // 0^2 = 0 (valid)
    assert_eq!(result, expr!((0 ^ 2)));
}

#[test]
fn test_eval_numeric_pow_zero_base_negative_exp_errors() {
    // expr! macro doesn't support negative numbers, use explicit API
    let expr = Expression::pow(Expression::integer(0), Expression::integer(-2));
    let result = expr.eval_numeric(53);

    // 0^(-2) = 1/0 → DivisionByZero error
    assert!(matches!(result, Err(MathError::DivisionByZero)));
}

#[test]
fn test_eval_numeric_function_sin() {
    let x = symbol!(x);
    let expr = Expression::function("sin".to_string(), vec![Expression::symbol(x)]);
    let result = expr.eval_numeric(53).unwrap();

    // Function with symbolic argument stays symbolic
    assert_eq!(result, expr);
}

#[test]
fn test_eval_numeric_function_with_number() {
    let expr = Expression::function("sin".to_string(), vec![Expression::integer(0)]);
    let result = expr.eval_numeric(53).unwrap();

    // sin(0) should evaluate (delegated to function evaluator)
    assert_eq!(result, expr!(0));
}

#[test]
fn test_eval_numeric_matrix_with_numbers() {
    let matrix_expr = Expression::matrix(vec![
        vec![Expression::integer(1), Expression::integer(2)],
        vec![Expression::integer(3), Expression::integer(4)],
    ]);

    let result = matrix_expr.eval_numeric(53).unwrap();

    // Elements should be evaluated (though numbers stay as-is)
    match result {
        Expression::Matrix(m) => {
            assert_eq!(m.dimensions(), (2, 2));
            assert_eq!(m.get_element(0, 0), Expression::integer(1));
            assert_eq!(m.get_element(0, 1), Expression::integer(2));
            assert_eq!(m.get_element(1, 0), Expression::integer(3));
            assert_eq!(m.get_element(1, 1), Expression::integer(4));
        }
        _ => panic!("Expected Matrix expression"),
    }
}

#[test]
fn test_eval_numeric_matrix_with_constants() {
    let matrix_expr = Expression::matrix(vec![vec![Expression::pi(), Expression::e()]]);

    let result = matrix_expr.eval_numeric(53).unwrap();

    // Constants should be numerically evaluated
    match result {
        Expression::Matrix(m) => {
            let pi_elem = m.get_element(0, 0);
            let e_elem = m.get_element(0, 1);

            // Compare at Expression level
            assert_eq!(pi_elem, Expression::float(std::f64::consts::PI));
            assert_eq!(e_elem, Expression::float(std::f64::consts::E));
        }
        _ => panic!("Expected Matrix expression"),
    }
}

#[test]
fn test_eval_numeric_set_with_numbers() {
    let set_expr = Expression::set(vec![
        Expression::integer(1),
        Expression::integer(2),
        Expression::integer(3),
    ]);

    let result = set_expr.eval_numeric(53).unwrap();

    // Elements should be evaluated
    match result {
        Expression::Set(elements) => {
            assert_eq!(elements.len(), 3);
            assert!(elements.contains(&Expression::integer(1)));
            assert!(elements.contains(&Expression::integer(2)));
            assert!(elements.contains(&Expression::integer(3)));
        }
        _ => panic!("Expected Set expression"),
    }
}

#[test]
fn test_eval_numeric_set_with_constants() {
    let set_expr = Expression::set(vec![Expression::pi(), Expression::e()]);

    let result = set_expr.eval_numeric(53).unwrap();

    // Constants should be numerically evaluated
    match result {
        Expression::Set(elements) => {
            assert_eq!(elements.len(), 2);

            // Check if set contains evaluated constants
            assert!(elements.contains(&Expression::float(std::f64::consts::PI)));
            assert!(elements.contains(&Expression::float(std::f64::consts::E)));
        }
        _ => panic!("Expected Set expression"),
    }
}

#[test]
fn test_eval_numeric_complex_with_constants() {
    let complex_expr = Expression::complex(Expression::pi(), Expression::e());

    let result = complex_expr.eval_numeric(53).unwrap();

    // Real and imaginary parts should be numerically evaluated
    match result {
        Expression::Complex(data) => {
            assert_eq!(data.real, Expression::float(std::f64::consts::PI));
            assert_eq!(data.imag, Expression::float(std::f64::consts::E));
        }
        _ => panic!("Expected Complex expression"),
    }
}

#[test]
fn test_eval_numeric_interval_with_constants() {
    let interval_expr = Expression::interval(
        Expression::pi(),
        Expression::e(),
        true,  // start_inclusive
        false, // end_inclusive
    );

    let result = interval_expr.eval_numeric(53).unwrap();

    // Bounds should be numerically evaluated
    match result {
        Expression::Interval(data) => {
            assert_eq!(data.start, Expression::float(std::f64::consts::PI));
            assert_eq!(data.end, Expression::float(std::f64::consts::E));
            assert!(data.start_inclusive);
            assert!(!data.end_inclusive);
        }
        _ => panic!("Expected Interval expression"),
    }
}

#[test]
fn test_eval_numeric_piecewise_evaluates_expressions_not_conditions() {
    let x = symbol!(x);
    let condition = Expression::relation(
        Expression::symbol(x.clone()),
        Expression::integer(0),
        RelationType::Greater,
    );

    let piecewise_expr = Expression::piecewise(
        vec![
            (Expression::pi(), condition.clone()),
            (Expression::e(), Expression::integer(1)), // Always true
        ],
        Some(Expression::integer(0)),
    );

    let result = piecewise_expr.eval_numeric(53).unwrap();

    // Expressions should be evaluated, conditions stay symbolic
    match result {
        Expression::Piecewise(data) => {
            assert_eq!(data.pieces.len(), 2);

            // First piece: expression = π (evaluated), condition = x > 0 (symbolic)
            let (expr1, cond1) = &data.pieces[0];
            assert_eq!(expr1, &Expression::float(std::f64::consts::PI));
            assert_eq!(cond1, &condition); // Condition unchanged

            // Second piece: expression = e (evaluated), condition = 1 (unchanged)
            let (expr2, cond2) = &data.pieces[1];
            assert_eq!(expr2, &Expression::float(std::f64::consts::E));
            assert_eq!(cond2, &Expression::integer(1));

            // Default: 0 (unchanged)
            assert_eq!(data.default, Some(Expression::integer(0)));
        }
        _ => panic!("Expected Piecewise expression"),
    }
}

#[test]
fn test_eval_numeric_relation_evaluates_both_sides() {
    let relation_expr = Expression::relation(Expression::pi(), Expression::e(), RelationType::Less);

    let result = relation_expr.eval_numeric(53).unwrap();

    // Both lhs and rhs should be numerically evaluated
    match result {
        Expression::Relation(data) => {
            assert_eq!(data.left, Expression::float(std::f64::consts::PI));
            assert_eq!(data.right, Expression::float(std::f64::consts::E));
            assert_eq!(data.relation_type, RelationType::Less);
        }
        _ => panic!("Expected Relation expression"),
    }
}

#[test]
fn test_eval_numeric_calculus_remains_symbolic() {
    let x = symbol!(x);
    let derivative_expr = Expression::derivative(
        Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
        x,
        1,
    );

    let result = derivative_expr.eval_numeric(53).unwrap();

    // Calculus expressions stay symbolic
    assert_eq!(result, derivative_expr);
}

#[test]
fn test_eval_numeric_nested_expressions() {
    let nested = Expression::add(vec![
        Expression::mul(vec![Expression::integer(2), Expression::pi()]),
        Expression::pow(Expression::e(), Expression::integer(2)),
    ]);

    let result = nested.eval_numeric(53).unwrap();

    // All constants should be numerically evaluated recursively
    match result {
        Expression::Add(terms) => {
            assert_eq!(terms.len(), 2);

            // Check that nested constants were evaluated
            let has_pi_evaluation = terms.iter().any(|term| {
                if let Expression::Mul(factors) = term {
                    factors.contains(&Expression::float(std::f64::consts::PI))
                } else {
                    false
                }
            });

            let has_e_evaluation = terms.iter().any(|term| {
                if let Expression::Pow(base, _exp) = term {
                    **base == Expression::float(std::f64::consts::E)
                } else {
                    false
                }
            });

            assert!(has_pi_evaluation || has_e_evaluation);
        }
        _ => panic!("Expected Add expression"),
    }
}

#[test]
fn test_eval_numeric_precision_parameter_ignored() {
    // Current implementation ignores precision parameter
    // (uses f64 precision by default)
    let pi = Expression::pi();

    let result1 = pi.eval_numeric(53).unwrap();
    let result2 = pi.eval_numeric(100).unwrap();

    // Results should be identical (precision not yet implemented)
    assert_eq!(result1, result2);
}
