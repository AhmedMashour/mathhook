use mathhook_core::calculus::derivatives::Derivative;
use mathhook_core::{symbol, Expression};
/// Quick benchmark to verify fast derivative() performance claims
use std::time::Instant;

fn main() {
    let x = symbol!(x);

    println!("=== Fast Derivative Performance Verification ===\n");

    // Simple power rule: d/dx(x^2)
    let expr = Expression::pow(Expression::symbol(x.clone()), Expression::integer(2));

    // Warmup
    for _ in 0..1000 {
        let _ = expr.derivative(x.clone());
    }

    // Measure
    let iterations = 1_000_000;
    let start = Instant::now();
    for _ in 0..iterations {
        let _ = expr.derivative(x.clone());
    }
    let duration = start.elapsed();

    let avg_ns = duration.as_nanos() / iterations;
    println!("Power Rule (d/dx x^2):");
    println!("  Average: {}ns per operation", avg_ns);
    println!("  Total for {} iterations: {:?}", iterations, duration);
    println!("  Target: ~400ns");
    println!(
        "  Status: {}",
        if avg_ns <= 500 {
            "✓ PASS"
        } else {
            "✗ SLOWER THAN TARGET"
        }
    );

    // Complex derivative: d/dx(sin(x^2) * e^x)
    let complex_expr = Expression::mul(vec![
        Expression::function(
            "sin",
            vec![Expression::pow(
                Expression::symbol(x.clone()),
                Expression::integer(2),
            )],
        ),
        Expression::function("exp", vec![Expression::symbol(x.clone())]),
    ]);

    let iterations_complex = 100_000;
    let start = Instant::now();
    for _ in 0..iterations_complex {
        let _ = complex_expr.derivative(x.clone());
    }
    let duration_complex = start.elapsed();

    let avg_us = duration_complex.as_micros() / iterations_complex;
    println!("\nComplex Derivative (d/dx sin(x^2) * e^x):");
    println!("  Average: {}µs per operation", avg_us);
    println!(
        "  Total for {} iterations: {:?}",
        iterations_complex, duration_complex
    );
    println!("  Target: <5µs");
    println!(
        "  Status: {}",
        if avg_us < 5 {
            "✓ PASS"
        } else {
            "✗ SLOWER THAN TARGET"
        }
    );

    // Polynomial: d/dx(3x^3 - 2x^2 + 5x - 1)
    let poly_expr = Expression::add(vec![
        Expression::mul(vec![
            Expression::integer(3),
            Expression::pow(Expression::symbol(x.clone()), Expression::integer(3)),
        ]),
        Expression::mul(vec![
            Expression::integer(-2),
            Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
        ]),
        Expression::mul(vec![Expression::integer(5), Expression::symbol(x.clone())]),
        Expression::integer(-1),
    ]);

    let iterations_poly = 100_000;
    let start = Instant::now();
    for _ in 0..iterations_poly {
        let _ = poly_expr.derivative(x.clone());
    }
    let duration_poly = start.elapsed();

    let avg_poly_us = duration_poly.as_micros() / iterations_poly;
    println!("\nPolynomial (d/dx 3x^3 - 2x^2 + 5x - 1):");
    println!("  Average: {}µs per operation", avg_poly_us);
    println!(
        "  Total for {} iterations: {:?}",
        iterations_poly, duration_poly
    );

    println!("\n=== Summary ===");
    println!("Fast derivative() mode is optimized for production use.");
    println!("For educational step-by-step explanations, use derivative_with_steps().");
}
