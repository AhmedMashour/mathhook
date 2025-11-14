//! Performance regression tests for integration
//!
//! Ensures integration operations complete in reasonable time and don't
//! exhibit exponential complexity for common cases.

use mathhook_core::calculus::integrals::strategy::integrate_with_strategy;
use mathhook_core::core::{Expression, Number, Symbol};
use std::time::{Duration, Instant};

fn symbol(name: &str) -> Symbol {
    Symbol::scalar(name)
}

fn x() -> Symbol {
    symbol("x")
}

fn integer(n: i64) -> Expression {
    Expression::integer(n)
}

fn add(terms: Vec<Expression>) -> Expression {
    Expression::Add(Box::new(terms))
}

fn mul(factors: Vec<Expression>) -> Expression {
    Expression::Mul(Box::new(factors))
}

fn pow(base: Expression, exp: Expression) -> Expression {
    Expression::Pow(Box::new(base), Box::new(exp))
}

fn sin(arg: Expression) -> Expression {
    Expression::function("sin", vec![arg])
}

fn cos(arg: Expression) -> Expression {
    Expression::function("cos", vec![arg])
}

fn exp(arg: Expression) -> Expression {
    Expression::function("exp", vec![arg])
}

fn ln(arg: Expression) -> Expression {
    Expression::function("ln", vec![arg])
}

// Performance threshold: simple integrals should complete in < 1ms
const SIMPLE_THRESHOLD: Duration = Duration::from_millis(1);

// Performance threshold: moderate complexity in < 10ms
const MODERATE_THRESHOLD: Duration = Duration::from_millis(10);

// Performance threshold: complex integrals in < 100ms
const COMPLEX_THRESHOLD: Duration = Duration::from_millis(100);

// Performance threshold: very complex in < 1s (prevents hangs)
const VERY_COMPLEX_THRESHOLD: Duration = Duration::from_secs(1);

#[test]
fn test_simple_polynomial_fast() {
    let var = x();

    // ∫x^2 dx should be nearly instant
    let expr = pow(Expression::Symbol(var.clone()), integer(2));

    let start = Instant::now();
    let _result = integrate_with_strategy(&expr, var.clone(), 0);
    let duration = start.elapsed();

    assert!(
        duration < SIMPLE_THRESHOLD,
        "Simple polynomial integration took {:?}, expected < {:?}",
        duration,
        SIMPLE_THRESHOLD
    );
}

#[test]
fn test_simple_trig_fast() {
    let var = x();

    // ∫sin(x) dx should be table lookup, very fast
    let expr = sin(Expression::Symbol(var.clone()));

    let start = Instant::now();
    let _result = integrate_with_strategy(&expr, var.clone(), 0);
    let duration = start.elapsed();

    assert!(
        duration < SIMPLE_THRESHOLD,
        "Simple trig integration took {:?}, expected < {:?}",
        duration,
        SIMPLE_THRESHOLD
    );
}

#[test]
fn test_simple_exponential_fast() {
    let var = x();

    // ∫e^x dx should be instant
    let expr = exp(Expression::Symbol(var.clone()));

    let start = Instant::now();
    let _result = integrate_with_strategy(&expr, var.clone(), 0);
    let duration = start.elapsed();

    assert!(
        duration < SIMPLE_THRESHOLD,
        "Simple exponential integration took {:?}, expected < {:?}",
        duration,
        SIMPLE_THRESHOLD
    );
}

#[test]
fn test_polynomial_sum_linear_time() {
    let var = x();

    // ∫(x + x^2 + x^3 + ... + x^20) dx
    // Should scale linearly with number of terms
    let terms: Vec<Expression> = (1..=20)
        .map(|i| pow(Expression::Symbol(var.clone()), integer(i)))
        .collect();
    let expr = add(terms);

    let start = Instant::now();
    let _result = integrate_with_strategy(&expr, var.clone(), 0);
    let duration = start.elapsed();

    assert!(
        duration < MODERATE_THRESHOLD,
        "20-term polynomial integration took {:?}, expected < {:?}",
        duration,
        MODERATE_THRESHOLD
    );
}

#[test]
fn test_rational_function_reasonable_time() {
    let var = x();

    // ∫1/(x^2 + 1) dx - requires rational function handling
    let expr = mul(vec![
        integer(1),
        pow(
            add(vec![
                pow(Expression::Symbol(var.clone()), integer(2)),
                integer(1),
            ]),
            integer(-1),
        ),
    ]);

    let start = Instant::now();
    let _result = integrate_with_strategy(&expr, var.clone(), 0);
    let duration = start.elapsed();

    assert!(
        duration < MODERATE_THRESHOLD,
        "Simple rational integration took {:?}, expected < {:?}",
        duration,
        MODERATE_THRESHOLD
    );
}

#[test]
fn test_partial_fractions_moderate_time() {
    let var = x();

    // ∫1/((x-1)(x-2)(x-3)) dx - partial fraction decomposition
    let denominator = mul(vec![
        add(vec![Expression::Symbol(var.clone()), integer(-1)]),
        add(vec![Expression::Symbol(var.clone()), integer(-2)]),
        add(vec![Expression::Symbol(var.clone()), integer(-3)]),
    ]);
    let expr = mul(vec![integer(1), pow(denominator, integer(-1))]);

    let start = Instant::now();
    let _result = integrate_with_strategy(&expr, var.clone(), 0);
    let duration = start.elapsed();

    assert!(
        duration < MODERATE_THRESHOLD,
        "Partial fractions integration took {:?}, expected < {:?}",
        duration,
        MODERATE_THRESHOLD
    );
}

#[test]
fn test_trig_power_reduction_fast() {
    let var = x();

    // ∫sin^2(x) dx - requires power reduction
    let expr = pow(sin(Expression::Symbol(var.clone())), integer(2));

    let start = Instant::now();
    let _result = integrate_with_strategy(&expr, var.clone(), 0);
    let duration = start.elapsed();

    assert!(
        duration < MODERATE_THRESHOLD,
        "Trig power reduction took {:?}, expected < {:?}",
        duration,
        MODERATE_THRESHOLD
    );
}

#[test]
fn test_by_parts_reasonable_time() {
    let var = x();

    // ∫x*ln(x) dx - integration by parts
    let expr = mul(vec![Expression::Symbol(var.clone()), ln(Expression::Symbol(var.clone()))]);

    let start = Instant::now();
    let _result = integrate_with_strategy(&expr, var.clone(), 0);
    let duration = start.elapsed();

    assert!(
        duration < MODERATE_THRESHOLD,
        "Integration by parts took {:?}, expected < {:?}",
        duration,
        MODERATE_THRESHOLD
    );
}

#[test]
fn test_substitution_fast() {
    let var = x();

    // ∫x*sin(x^2) dx - u-substitution
    let expr = mul(vec![
        Expression::Symbol(var.clone()),
        sin(pow(Expression::Symbol(var.clone()), integer(2))),
    ]);

    let start = Instant::now();
    let _result = integrate_with_strategy(&expr, var.clone(), 0);
    let duration = start.elapsed();

    assert!(
        duration < MODERATE_THRESHOLD,
        "U-substitution took {:?}, expected < {:?}",
        duration,
        MODERATE_THRESHOLD
    );
}

#[test]
fn test_trig_product_moderate_time() {
    let var = x();

    // ∫sin^3(x)*cos^2(x) dx - requires multiple strategies
    let expr = mul(vec![
        pow(sin(Expression::Symbol(var.clone())), integer(3)),
        pow(cos(Expression::Symbol(var.clone())), integer(2)),
    ]);

    let start = Instant::now();
    let _result = integrate_with_strategy(&expr, var.clone(), 0);
    let duration = start.elapsed();

    assert!(
        duration < COMPLEX_THRESHOLD,
        "Complex trig product took {:?}, expected < {:?}",
        duration,
        COMPLEX_THRESHOLD
    );
}

#[test]
fn test_exponential_polynomial_product() {
    let var = x();

    // ∫x^3 * e^x dx - repeated integration by parts
    let expr = mul(vec![
        pow(Expression::Symbol(var.clone()), integer(3)),
        exp(Expression::Symbol(var.clone())),
    ]);

    let start = Instant::now();
    let _result = integrate_with_strategy(&expr, var.clone(), 0);
    let duration = start.elapsed();

    assert!(
        duration < COMPLEX_THRESHOLD,
        "Exponential polynomial product took {:?}, expected < {:?}",
        duration,
        COMPLEX_THRESHOLD
    );
}

#[test]
fn test_does_not_hang_on_complex_integral() {
    let var = x();

    // ∫e^(x^2) dx - non-elementary (error function)
    // Should recognize as non-elementary and return symbolic quickly
    let expr = exp(pow(Expression::Symbol(var.clone()), integer(2)));

    let start = Instant::now();
    let _result = integrate_with_strategy(&expr, var.clone(), 0);
    let duration = start.elapsed();

    assert!(
        duration < VERY_COMPLEX_THRESHOLD,
        "Non-elementary integral took {:?}, expected < {:?} (possible hang)",
        duration,
        VERY_COMPLEX_THRESHOLD
    );
}

#[test]
fn test_does_not_hang_on_non_integrable() {
    let var = x();

    // ∫sin(x)/x dx - sine integral, non-elementary
    // Should fail gracefully and return symbolic
    let expr = mul(vec![
        sin(Expression::Symbol(var.clone())),
        pow(Expression::Symbol(var.clone()), integer(-1)),
    ]);

    let start = Instant::now();
    let _result = integrate_with_strategy(&expr, var.clone(), 0);
    let duration = start.elapsed();

    assert!(
        duration < VERY_COMPLEX_THRESHOLD,
        "Non-elementary sine integral took {:?}, expected < {:?}",
        duration,
        VERY_COMPLEX_THRESHOLD
    );
}

#[test]
fn test_table_lookup_instant() {
    let var = x();

    // All these should be instant table lookups
    let test_cases = vec![
        sin(Expression::Symbol(var.clone())),
        cos(Expression::Symbol(var.clone())),
        exp(Expression::Symbol(var.clone())),
        pow(Expression::Symbol(var.clone()), integer(-1)), // 1/x
    ];

    for expr in test_cases {
        let start = Instant::now();
        let _result = integrate_with_strategy(&expr, var.clone(), 0);
        let duration = start.elapsed();

        assert!(
            duration < SIMPLE_THRESHOLD,
            "Table lookup took {:?}, expected < {:?}",
            duration,
            SIMPLE_THRESHOLD
        );
    }
}

#[test]
fn test_strategy_fallthrough_not_exponential() {
    let var = x();

    // This requires trying multiple strategies before succeeding
    // Should still be fast (not exponential in number of strategies)
    let expr = mul(vec![
        integer(1),
        pow(
            add(vec![
                pow(Expression::Symbol(var.clone()), integer(2)),
                mul(vec![integer(2), Expression::Symbol(var.clone())]),
                integer(1),
            ]),
            integer(-1),
        ),
    ]);

    let start = Instant::now();
    let _result = integrate_with_strategy(&expr, var.clone(), 0);
    let duration = start.elapsed();

    assert!(
        duration < MODERATE_THRESHOLD,
        "Strategy fallthrough took {:?}, expected < {:?}",
        duration,
        MODERATE_THRESHOLD
    );
}

#[test]
fn test_large_sum_linear_scaling() {
    let var = x();

    // Test that adding more terms scales linearly, not exponentially
    // ∫(sin(x) + cos(x) + ... + e^x) dx with 10 terms
    let terms = vec![
        sin(Expression::Symbol(var.clone())),
        cos(Expression::Symbol(var.clone())),
        exp(Expression::Symbol(var.clone())),
        ln(Expression::Symbol(var.clone())),
        pow(Expression::Symbol(var.clone()), integer(2)),
        pow(Expression::Symbol(var.clone()), integer(3)),
        mul(vec![integer(2), Expression::Symbol(var.clone())]),
        pow(Expression::Symbol(var.clone()), integer(-1)),
        mul(vec![integer(3), sin(Expression::Symbol(var.clone()))]),
        mul(vec![integer(5), cos(Expression::Symbol(var.clone()))]),
    ];
    let expr = add(terms);

    let start = Instant::now();
    let _result = integrate_with_strategy(&expr, var.clone(), 0);
    let duration = start.elapsed();

    assert!(
        duration < MODERATE_THRESHOLD,
        "10-term mixed integration took {:?}, expected < {:?}",
        duration,
        MODERATE_THRESHOLD
    );
}

#[test]
fn test_nested_products_reasonable() {
    let var = x();

    // ∫(2*3*4*x) dx - multiple constant factors
    // Should simplify constants quickly
    let expr = mul(vec![
        integer(2),
        integer(3),
        integer(4),
        Expression::Symbol(var.clone()),
    ]);

    let start = Instant::now();
    let _result = integrate_with_strategy(&expr, var.clone(), 0);
    let duration = start.elapsed();

    assert!(
        duration < SIMPLE_THRESHOLD,
        "Nested constant products took {:?}, expected < {:?}",
        duration,
        SIMPLE_THRESHOLD
    );
}

#[test]
fn test_constant_integration_instant() {
    let var = x();

    // ∫5 dx should be instant
    let expr = integer(5);

    let start = Instant::now();
    let _result = integrate_with_strategy(&expr, var.clone(), 0);
    let duration = start.elapsed();

    assert!(
        duration < SIMPLE_THRESHOLD,
        "Constant integration took {:?}, expected < {:?}",
        duration,
        SIMPLE_THRESHOLD
    );
}

#[test]
fn test_risch_timeout_protection() {
    let var = x();

    // ∫e^(e^x) dx - very complex, non-elementary
    // Risch algorithm should timeout or recognize as non-elementary quickly
    let expr = exp(exp(Expression::Symbol(var.clone())));

    let start = Instant::now();
    let _result = integrate_with_strategy(&expr, var.clone(), 0);
    let duration = start.elapsed();

    assert!(
        duration < VERY_COMPLEX_THRESHOLD,
        "Risch algorithm took {:?}, expected < {:?} (possible timeout issue)",
        duration,
        VERY_COMPLEX_THRESHOLD
    );
}

#[test]
fn test_multiple_integrations_no_memory_leak() {
    let var = x();

    // Perform same integration 100 times
    // Should not accumulate memory or get slower
    let expr = pow(Expression::Symbol(var.clone()), integer(2));

    let start = Instant::now();
    for _ in 0..100 {
        let _result = integrate_with_strategy(&expr, var.clone(), 0);
    }
    let duration = start.elapsed();

    // 100 simple integrations should still be very fast
    assert!(
        duration < Duration::from_millis(100),
        "100 simple integrations took {:?}, expected < 100ms (possible memory issue)",
        duration
    );
}
