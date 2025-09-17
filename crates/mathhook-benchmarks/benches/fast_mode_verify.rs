use criterion::{criterion_group, criterion_main, Criterion};
use mathhook_core::calculus::derivatives::Derivative;
use mathhook_core::{symbol, Expression};
/// Verify fast derivative() performance for marketing claims
use std::hint::black_box;
use std::time::Duration;

fn bench_fast_derivatives(c: &mut Criterion) {
    let mut group = c.benchmark_group("derivatives/fast");

    let x = symbol!(x);

    // Simple power rule: d/dx(x^2) - Should be ~400ns
    let simple_expr = Expression::pow(Expression::symbol(x.clone()), Expression::integer(2));

    group.bench_function("power_rule_x2", |b| {
        b.iter(|| black_box(simple_expr.derivative(x.clone())))
    });

    // Complex derivative: d/dx(sin(x^2) * e^x) - Should be <5µs
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

    group.bench_function("complex_chain_product", |b| {
        b.iter(|| black_box(complex_expr.derivative(x.clone())))
    });

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

    group.bench_function("polynomial_degree_3", |b| {
        b.iter(|| black_box(poly_expr.derivative(x.clone())))
    });

    group.finish();
}

criterion_group!(
    name = fast_mode_benchmarks;
    config = Criterion::default()
        .measurement_time(Duration::from_secs(5))
        .sample_size(1000);  // More samples for faster operations
    targets = bench_fast_derivatives
);

criterion_main!(fast_mode_benchmarks);
