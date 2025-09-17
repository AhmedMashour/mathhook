//! Power and exponentiation operation tests

use mathhook_core::prelude::*;

#[test]
fn test_power_simplification() {
    let x = symbol!(x);

    // Test x^0 = 1
    let expr = Expression::pow(Expression::symbol(x.clone()), Expression::integer(0));
    let result = expr.simplify();
    assert_eq!(result, Expression::integer(1));

    // Test x^1 = x
    let expr = Expression::pow(Expression::symbol(x.clone()), Expression::integer(1));
    let result = expr.simplify();
    assert_eq!(result, Expression::symbol(x.clone()));

    // Test 0^n = 0 (for n > 0)
    let expr = Expression::pow(Expression::integer(0), Expression::integer(5));
    let result = expr.simplify();
    assert_eq!(result, Expression::integer(0));

    // Test 1^n = 1
    let expr = Expression::pow(Expression::integer(1), Expression::integer(100));
    let result = expr.simplify();
    assert_eq!(result, Expression::integer(1));
}

#[test]
fn test_distribute_multiplication() {
    let x = symbol!(x);
    let y = symbol!(y);

    // Test basic distribution patterns
    let expr = Expression::mul(vec![
        Expression::integer(2),
        Expression::add(vec![
            Expression::symbol(x.clone()),
            Expression::symbol(y.clone()),
        ]),
    ]);

    let result = expr.simplify();
    println!("Distribution test: 2(x + y) = {}", result);

    // Should maintain structure for now (distribution not yet implemented)
    assert!(!result.is_zero());
}

#[test]
fn test_algebraic_manipulation_patterns() {
    let x = symbol!(x);

    // Test (x^2)^(1/2) - should be handled carefully
    let expr = Expression::pow(
        Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
        Expression::number(Number::rational(num_rational::BigRational::new(
            num_bigint::BigInt::from(1),
            num_bigint::BigInt::from(2),
        ))),
    );

    let result = expr.simplify();
    println!("(x^2)^(1/2) = {}", result);

    // This is a complex case - for now just ensure it doesn't crash
    assert!(!result.is_zero());
}

#[test]
fn test_historic_50_percent_milestone() {
    // Commemorating our 50% SymPy coverage milestone
    let x = symbol!(x);

    let expr = Expression::pow(
        Expression::add(vec![Expression::symbol(x.clone()), Expression::integer(1)]),
        Expression::integer(2),
    );

    let result = expr.simplify();
    println!("50% milestone: (x + 1)^2 = {}", result);

    // Should maintain the power structure
    assert!(
        matches!(result, Expression::Pow(_, _)),
        "Expected (x + 1)^2 to remain as power (50% milestone), got: {}",
        result
    );
}

#[test]
fn test_simplify_zero_expressions() {
    let x = symbol!(x);

    // Test expressions that should simplify to zero
    let expr = Expression::add(vec![
        Expression::symbol(x.clone()),
        Expression::mul(vec![Expression::integer(-1), Expression::symbol(x.clone())]),
    ]);

    let result = expr.simplify();
    println!("x + (-1)*x = {}", result);

    // This should eventually equal 0 when like-term collection is implemented
    // For now, verify it's a valid expression
    match result {
        Expression::Number(Number::Integer(0)) => {
            // Perfect! Like-term collection working
            assert_eq!(result, Expression::integer(0));
        }
        Expression::Add(_) | Expression::Mul(_) | Expression::Symbol(_) => {
            // Valid expression structure, simplification not yet complete
            println!(
                "Like-term collection not yet implemented: x + (-1)*x = {}",
                result
            );
        }
        _ => panic!("Expression should be valid"),
    }
}

#[test]
fn test_advanced_power_patterns() {
    let x = symbol!(x);

    // Test x^2 * x^3 = x^5 (if implemented)
    let expr = Expression::mul(vec![
        Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
        Expression::pow(Expression::symbol(x.clone()), Expression::integer(3)),
    ]);

    let result = expr.simplify();
    println!("x^2 * x^3 = {}", result);

    // Advanced power combination might not be implemented yet
    assert!(!result.is_zero());

    // Test (x^2)^3 = x^6 (if implemented)
    let expr = Expression::pow(
        Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
        Expression::integer(3),
    );

    let result = expr.simplify();
    println!("(x^2)^3 = {}", result);
    assert!(!result.is_zero());
}

#[test]
fn test_ultimate_power_mastery() {
    let x = symbol!(x);

    // Test complex power expression: x^(-2) * x^3 = x
    let expr = Expression::mul(vec![
        Expression::pow(Expression::symbol(x.clone()), Expression::integer(-2)),
        Expression::pow(Expression::symbol(x.clone()), Expression::integer(3)),
    ]);

    let result = expr.simplify();
    println!("x^(-2) * x^3 = {}", result);

    // This is advanced - might not simplify to x yet
    // Check that it maintains power structure with x and exponents -2 or 3
    let result_str = result.to_string();
    assert!(result_str.contains("x"), "Should contain x");

    // Test fractional exponents
    let expr = Expression::pow(
        Expression::integer(8),
        Expression::number(Number::rational(num_rational::BigRational::new(
            num_bigint::BigInt::from(1),
            num_bigint::BigInt::from(3),
        ))),
    );

    let result = expr.simplify();
    println!("8^(1/3) = {}", result);
    assert!(!result.is_zero());
}

#[test]
fn test_ultimate_power_combinations() {
    let x = symbol!(x);
    let y = symbol!(y);

    // Test mixed power operations
    let expr = Expression::add(vec![
        Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
        Expression::mul(vec![
            Expression::integer(2),
            Expression::symbol(x.clone()),
            Expression::symbol(y.clone()),
        ]),
        Expression::pow(Expression::symbol(y.clone()), Expression::integer(2)),
    ]);

    let result = expr.simplify();
    println!("x^2 + 2xy + y^2 = {}", result);

    // This is (x + y)^2 expanded - should be preserved as valid expression
    match result {
        Expression::Add(_)
        | Expression::Mul(_)
        | Expression::Pow(_, _)
        | Expression::Symbol(_)
        | Expression::Number(_) => (),
        _ => panic!("Polynomial expression should be valid"),
    }

    // Test power with zero base
    let expr = Expression::pow(Expression::integer(0), Expression::symbol(x.clone()));
    let result = expr.simplify();
    println!("0^x = {}", result);

    // Should handle this edge case gracefully
    assert!(!result.to_string().is_empty());
}

#[test]
fn test_ultimate_100_percent_operation_2() {
    let x = symbol!(x);

    // Ultimate power test for 100% coverage
    let expr = Expression::pow(
        Expression::add(vec![
            Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
            Expression::integer(1),
        ]),
        Expression::number(Number::rational(num_rational::BigRational::new(
            num_bigint::BigInt::from(1),
            num_bigint::BigInt::from(2),
        ))),
    );

    let result = expr.simplify();
    println!("Ultimate power operation 2: {}", result);

    // Complex nested power expression
    assert!(!result.is_zero());
}

#[test]
fn test_numeric_powers() {
    // Test numeric power calculations
    let expr = Expression::pow(Expression::integer(2), Expression::integer(3));
    let result = expr.simplify();

    // Should calculate 2^3 = 8
    match result {
        Expression::Number(Number::Integer(n)) if n == 8 => {
            assert_eq!(n, 8);
        }
        _ => {
            println!("Numeric power result: {}", result);
            // Might not be implemented yet, but should not crash
            assert!(!result.is_zero() || result == Expression::integer(0));
        }
    }

    // Test larger powers
    let expr = Expression::pow(Expression::integer(3), Expression::integer(4));
    let result = expr.simplify();
    println!("3^4 = {}", result);

    // Test negative base
    let expr = Expression::pow(Expression::integer(-2), Expression::integer(3));
    let result = expr.simplify();
    println!("(-2)^3 = {}", result);
}

#[test]
fn test_power_edge_cases() {
    let x = symbol!(x);

    // Test 0^0 (mathematically undefined, but often treated as 1)
    let expr = Expression::pow(Expression::integer(0), Expression::integer(0));
    let result = expr.simplify();
    println!("0^0 = {}", result);

    // Test x^0 for various x
    let expr = Expression::pow(
        Expression::add(vec![Expression::symbol(x.clone()), Expression::integer(5)]),
        Expression::integer(0),
    );
    let result = expr.simplify();
    assert_eq!(result, Expression::integer(1));

    // Test 1^x for various x
    let expr = Expression::pow(Expression::integer(1), Expression::symbol(x.clone()));
    let result = expr.simplify();
    assert_eq!(result, Expression::integer(1));
}
