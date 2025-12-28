//! Pattern matching integration tests
//!
//! Tests for symbolic pattern matching and rewrite rules including:
//! - Basic pattern matching
//! - Wildcard patterns
//! - Conditional patterns
//! - Rewrite rules
//! - Pattern-based simplification

use mathhook_core::core::polynomial::{coefficient_at, coefficients_list};
use mathhook_core::pattern::{Matchable, Pattern, Substitutable};
use mathhook_core::{expr, symbol, Expression, Number, Simplify};

#[test]
fn test_match_literal() {
    let expr = expr!(2 + 3);
    let pattern = Pattern::Exact(expr!(2 + 3));
    assert!(expr.matches(&pattern).is_some());
}

#[test]
fn test_match_with_wildcard() {
    let a = symbol!(a);
    let expr = Expression::add(vec![Expression::symbol(a.clone()), Expression::integer(3)]);
    let pattern = Pattern::Add(vec![
        Pattern::wildcard("x"),
        Pattern::Exact(Expression::integer(3)),
    ]);
    let matches = expr.matches(&pattern);
    assert!(matches.is_some());
    if let Some(bindings) = matches {
        assert_eq!(bindings.get("x"), Some(&Expression::symbol(a)));
    }
}

#[test]
fn test_match_named_wildcard() {
    let expr = expr!(sin(5));
    let pattern = Pattern::Function {
        name: "sin".to_string(),
        args: vec![Pattern::wildcard("n")],
    };
    let matches = expr.matches(&pattern);
    assert!(matches.is_some());
    if let Some(bindings) = matches {
        assert_eq!(bindings.get("n"), Some(&Expression::integer(5)));
    }
}

#[test]
fn test_match_nested_pattern() {
    let x = symbol!(x);
    let expr = expr!(sin(x + 1));
    let pattern = Pattern::Function {
        name: "sin".to_string(),
        args: vec![Pattern::Add(vec![
            Pattern::Exact(Expression::symbol(x.clone())),
            Pattern::wildcard("c"),
        ])],
    };
    let matches = expr.matches(&pattern);
    assert!(matches.is_some());
    if let Some(bindings) = matches {
        assert_eq!(bindings.get("c"), Some(&Expression::integer(1)));
    }
}

#[test]
fn test_no_match() {
    let expr = expr!(2 + 3);
    let pattern = Pattern::Mul(vec![
        Pattern::Exact(Expression::integer(2)),
        Pattern::Exact(Expression::integer(3)),
    ]);
    assert!(expr.matches(&pattern).is_none());
}

#[test]
fn test_wildcard_matches_any() {
    let pattern = Pattern::wildcard("x");
    assert!(Expression::integer(42).matches(&pattern).is_some());
    let x = symbol!(x);
    assert!(Expression::symbol(x).matches(&pattern).is_some());
    assert!(expr!(sin(5)).matches(&pattern).is_some());
}

#[test]
fn test_typed_wildcard_integer() {
    fn is_integer(expr: &Expression) -> bool {
        matches!(expr, Expression::Number(_))
    }

    let pattern = Pattern::wildcard_with_properties("n", vec![is_integer]);
    assert!(Expression::integer(42).matches(&pattern).is_some());
    let x = symbol!(x);
    assert!(Expression::symbol(x).matches(&pattern).is_none());
}

#[test]
fn test_typed_wildcard_symbol() {
    fn is_symbol(expr: &Expression) -> bool {
        matches!(expr, Expression::Symbol(_))
    }

    let pattern = Pattern::wildcard_with_properties("s", vec![is_symbol]);
    let x = symbol!(x);
    assert!(Expression::symbol(x).matches(&pattern).is_some());
    assert!(Expression::integer(42).matches(&pattern).is_none());
}

#[test]
fn test_conditional_pattern_positive() {
    fn is_positive(expr: &Expression) -> bool {
        match expr {
            Expression::Number(Number::Integer(i)) => *i > 0,
            Expression::Number(Number::Float(f)) => *f > 0.0,
            Expression::Number(Number::BigInteger(bi)) => {
                use num_traits::sign::Signed;
                bi.is_positive()
            }
            Expression::Number(Number::Rational(r)) => {
                use num_traits::sign::Signed;
                r.is_positive()
            }
            _ => false,
        }
    }

    let pattern = Pattern::wildcard_with_properties("n", vec![is_positive]);
    assert!(Expression::integer(5).matches(&pattern).is_some());
    assert!(Expression::integer(-5).matches(&pattern).is_none());
}

#[test]
fn test_conditional_pattern_even() {
    fn is_even(expr: &Expression) -> bool {
        match expr {
            Expression::Number(Number::Integer(i)) => i % 2 == 0,
            _ => false,
        }
    }

    let pattern = Pattern::wildcard_with_properties("n", vec![is_even]);
    assert!(Expression::integer(4).matches(&pattern).is_some());
    assert!(Expression::integer(5).matches(&pattern).is_none());
}

#[test]
fn test_simple_rewrite() {
    let x = symbol!(x);
    let expr = expr!(x + 0);
    let pattern = Pattern::Add(vec![
        Pattern::wildcard("x"),
        Pattern::Exact(Expression::integer(0)),
    ]);
    let replacement = Pattern::wildcard("x");
    let result = expr.replace(&pattern, &replacement);
    assert_eq!(result, Expression::symbol(x));
}

#[test]
fn test_rewrite_with_transformation() {
    let x = symbol!(x);
    let sin_x = Expression::function("sin".to_string(), vec![Expression::symbol(x.clone())]);
    let cos_x = Expression::function("cos".to_string(), vec![Expression::symbol(x.clone())]);
    let expr = Expression::add(vec![
        Expression::pow(sin_x.clone(), Expression::integer(2)),
        Expression::pow(cos_x.clone(), Expression::integer(2)),
    ]);

    let pattern = Pattern::Add(vec![
        Pattern::Pow(
            Box::new(Pattern::Function {
                name: "sin".to_string(),
                args: vec![Pattern::wildcard("a")],
            }),
            Box::new(Pattern::Exact(Expression::integer(2))),
        ),
        Pattern::Pow(
            Box::new(Pattern::Function {
                name: "cos".to_string(),
                args: vec![Pattern::wildcard("a")],
            }),
            Box::new(Pattern::Exact(Expression::integer(2))),
        ),
    ]);

    let replacement = Pattern::Exact(Expression::integer(1));
    let result = expr.replace(&pattern, &replacement);
    assert_eq!(result, Expression::integer(1));
}

#[test]
fn test_rewrite_power_of_power() {
    let x = symbol!(x);
    let expr = Expression::pow(
        Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
        Expression::integer(3),
    );

    let pattern = Pattern::Pow(
        Box::new(Pattern::Pow(
            Box::new(Pattern::wildcard("x")),
            Box::new(Pattern::wildcard("a")),
        )),
        Box::new(Pattern::wildcard("b")),
    );

    let replacement = Pattern::Pow(
        Box::new(Pattern::wildcard("x")),
        Box::new(Pattern::Mul(vec![
            Pattern::wildcard("a"),
            Pattern::wildcard("b"),
        ])),
    );

    let result = expr.replace(&pattern, &replacement);
    let expected = Expression::pow(Expression::symbol(x), Expression::integer(6));
    assert_eq!(result.simplify(), expected);
}

#[test]
fn test_rewrite_log_product() {
    let a = symbol!(a);
    let b = symbol!(b);
    let expr = Expression::function(
        "log".to_string(),
        vec![Expression::mul(vec![
            Expression::symbol(a.clone()),
            Expression::symbol(b.clone()),
        ])],
    );

    let _inner = Expression::mul(vec![
        Expression::symbol(a.clone()),
        Expression::symbol(b.clone()),
    ]);
    let pattern = Pattern::Function {
        name: "log".to_string(),
        args: vec![Pattern::Mul(vec![
            Pattern::wildcard("a"),
            Pattern::wildcard("b"),
        ])],
    };

    let matches = expr.matches(&pattern);
    assert!(matches.is_some());

    let simplified = expr.simplify();
    assert!(matches!(simplified, Expression::Function { .. }));
}

#[test]
fn test_repeated_rule_application() {
    let y = symbol!(y);
    let expr = Expression::add(vec![
        Expression::add(vec![
            Expression::add(vec![Expression::symbol(y.clone()), Expression::integer(0)]),
            Expression::integer(0),
        ]),
        Expression::integer(0),
    ]);

    let pattern = Pattern::Add(vec![
        Pattern::wildcard("x"),
        Pattern::Exact(Expression::integer(0)),
    ]);
    let replacement = Pattern::wildcard("x");

    let result1 = expr.replace(&pattern, &replacement);
    let result2 = result1.replace(&pattern, &replacement);
    let result3 = result2.replace(&pattern, &replacement);

    assert_eq!(result3, Expression::symbol(y));
}

#[test]
fn test_match_add_structure() {
    let _a = symbol!(a);
    let _b = symbol!(b);
    let _c = symbol!(c);
    let expr = expr!(a + b + c);
    let pattern = Pattern::Add(vec![
        Pattern::wildcard("x"),
        Pattern::wildcard("y"),
        Pattern::wildcard("z"),
    ]);
    assert!(expr.matches(&pattern).is_some());
}

#[test]
fn test_match_function_structure() {
    let _x = symbol!(x);
    let expr = Expression::function("sin".to_string(), vec![Expression::symbol(_x)]);
    let pattern = Pattern::Function {
        name: "sin".to_string(),
        args: vec![Pattern::wildcard("arg")],
    };
    assert!(expr.matches(&pattern).is_some());
}

#[test]
fn test_match_power_structure() {
    let _x = symbol!(x);
    let expr = expr!(x ^ 2);
    let pattern = Pattern::Pow(
        Box::new(Pattern::wildcard("base")),
        Box::new(Pattern::wildcard("exp")),
    );
    assert!(expr.matches(&pattern).is_some());
}

#[test]
fn test_commutative_match() {
    let x = symbol!(x);
    let y = symbol!(y);
    let expr = expr!(x + y);
    let pattern = Pattern::Add(vec![
        Pattern::Exact(Expression::symbol(y.clone())),
        Pattern::Exact(Expression::symbol(x.clone())),
    ]);
    assert!(expr.matches(&pattern).is_some());
}

#[test]
fn test_ac_match_complex() {
    let a = symbol!(a);
    let b = symbol!(b);
    let c = symbol!(c);
    let expr = expr!(a + b + c);
    let pattern = Pattern::Add(vec![
        Pattern::Exact(Expression::symbol(c.clone())),
        Pattern::Exact(Expression::symbol(a.clone())),
        Pattern::Exact(Expression::symbol(b.clone())),
    ]);
    assert!(expr.matches(&pattern).is_some());
}

#[test]
fn test_substitute_single() {
    let x = symbol!(x);
    let expr = expr!(x + 1);
    let result = expr.subs(&Expression::symbol(x), &Expression::integer(5));
    assert_eq!(result, Expression::integer(6));
}

#[test]
fn test_substitute_multiple() {
    let x = symbol!(x);
    let y = symbol!(y);
    let expr = expr!((x ^ 2) + (y ^ 2));
    let result = expr.subs_multiple(&[
        (Expression::symbol(x), Expression::integer(3)),
        (Expression::symbol(y), Expression::integer(4)),
    ]);
    assert_eq!(result, Expression::integer(25));
}

#[test]
fn test_substitute_expression() {
    let x = symbol!(x);
    let y = symbol!(y);
    let sin_x = Expression::function("sin".to_string(), vec![Expression::symbol(x.clone())]);
    let cos_x = Expression::function("cos".to_string(), vec![Expression::symbol(x)]);
    let expr = Expression::add(vec![sin_x.clone(), cos_x]);
    let result = expr.subs(&sin_x, &Expression::symbol(y.clone()));
    assert!(matches!(
        result,
        Expression::Add(_) if result.to_string().contains("y")
    ));
}

#[test]
fn test_collect_symbols() {
    let x = symbol!(x);
    let y = symbol!(y);
    let expr = expr!((x ^ 2) + (2 * x * y) + (y ^ 2));
    let symbols = expr.find_variables();
    assert_eq!(symbols.len(), 2);
    assert!(symbols.contains(&x));
    assert!(symbols.contains(&y));
}

#[test]
fn test_collect_by_pattern() {
    let x = symbol!(x);
    let expr = expr!((2 * x) + (3 * (x ^ 2)) + (5 * (x ^ 3)));

    let c0 = coefficient_at(&expr, &x, 0);
    let c1 = coefficient_at(&expr, &x, 1);
    let c2 = coefficient_at(&expr, &x, 2);
    let c3 = coefficient_at(&expr, &x, 3);

    assert_eq!(c0, Expression::integer(0));
    assert_eq!(c1, Expression::integer(2));
    assert_eq!(c2, Expression::integer(3));
    assert_eq!(c3, Expression::integer(5));

    let coeffs = coefficients_list(&expr, &x);
    assert!(coeffs.len() >= 3);
}

#[test]
fn test_expression_equality_basic() {
    let expr1 = expr!(2 + 3);
    let expr2 = expr!(2 + 3);
    assert_eq!(expr1, expr2);
}

#[test]
fn test_expression_equality_different() {
    let expr1 = expr!(2 + 3);
    let expr2 = expr!(3 + 2);
    let simplified1 = expr1.simplify();
    let simplified2 = expr2.simplify();
    assert_eq!(simplified1, simplified2);
}

#[test]
fn test_function_expression_creation() {
    let x = symbol!(x);
    let sin_x = Expression::function("sin".to_string(), vec![Expression::symbol(x)]);
    match sin_x {
        Expression::Function { name, args } => {
            assert_eq!(name.as_ref(), "sin");
            assert_eq!(args.len(), 1);
        }
        _ => panic!("Expected Function expression"),
    }
}

#[test]
fn test_wildcard_excluding() {
    let x = symbol!(x);
    let y = symbol!(y);
    let pattern = Pattern::wildcard_excluding("a", vec![Expression::symbol(x.clone())]);

    assert!(Expression::symbol(x).matches(&pattern).is_none());
    assert!(Expression::symbol(y).matches(&pattern).is_some());
}

#[test]
fn test_pattern_replace_recursive() {
    let x = symbol!(x);
    let expr = Expression::add(vec![
        Expression::add(vec![Expression::symbol(x.clone()), Expression::integer(0)]),
        Expression::mul(vec![Expression::symbol(x.clone()), Expression::integer(1)]),
    ]);

    let pattern_add_zero = Pattern::Add(vec![
        Pattern::wildcard("x"),
        Pattern::Exact(Expression::integer(0)),
    ]);
    let replacement_x = Pattern::wildcard("x");

    let pattern_mul_one = Pattern::Mul(vec![
        Pattern::wildcard("x"),
        Pattern::Exact(Expression::integer(1)),
    ]);

    let step1 = expr.replace(&pattern_add_zero, &replacement_x);
    let step2 = step1.replace(&pattern_mul_one, &replacement_x);

    assert_eq!(
        step2,
        Expression::add(vec![Expression::symbol(x.clone()), Expression::symbol(x)])
    );
}
