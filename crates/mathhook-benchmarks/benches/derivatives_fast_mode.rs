/// Fast derivative mode benchmarks (production performance)
///
/// Tests the fast `derivative()` API without step generation.
/// This is the performance-critical path for production use.
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use mathhook_core::calculus::derivatives::Derivative;
use mathhook_core::{symbol, Expression};
use std::time::Duration;

fn bench_fast_power_rule(c: &mut Criterion) {
    let mut group = c.benchmark_group("derivatives_fast/power_rule");
    let x = symbol!(x);

    for power in [2, 5, 10, 20, 50].iter() {
        let expr = Expression::pow(Expression::symbol(x.clone()), Expression::integer(*power));
        group.bench_with_input(BenchmarkId::from_parameter(power), power, |b, _| {
            b.iter(|| expr.derivative(x.clone()))
        });
    }

    group.finish();
}

fn bench_fast_chain_rule(c: &mut Criterion) {
    let mut group = c.benchmark_group("derivatives_fast/chain_rule");
    let x = symbol!(x);

    // sin(x^2)
    let expr = Expression::function(
        "sin",
        vec![Expression::pow(
            Expression::symbol(x.clone()),
            Expression::integer(2),
        )],
    );

    group.bench_function("sin_x2", |b| b.iter(|| expr.derivative(x.clone())));

    group.finish();
}

fn bench_fast_product_rule(c: &mut Criterion) {
    let mut group = c.benchmark_group("derivatives_fast/product_rule");
    let x = symbol!(x);

    // x^2 * sin(x)
    let expr = Expression::mul(vec![
        Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
        Expression::function("sin", vec![Expression::symbol(x.clone())]),
    ]);

    group.bench_function("x2_sin_x", |b| b.iter(|| expr.derivative(x.clone())));

    group.finish();
}

fn bench_fast_complex(c: &mut Criterion) {
    let mut group = c.benchmark_group("derivatives_fast/complex");
    let x = symbol!(x);

    // sin(x^2) * e^x
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

    group.bench_function("sin_x2_exp_x", |b| {
        b.iter(|| complex_expr.derivative(x.clone()))
    });

    // Trigonometric derivative
    let trig_expr = Expression::function(
        "sin",
        vec![Expression::mul(vec![
            Expression::integer(2),
            Expression::symbol(x.clone()),
        ])],
    );

    group.bench_function("sin_2x", |b| b.iter(|| trig_expr.derivative(x.clone())));

    // Exponential derivative
    let exp_expr = Expression::function(
        "exp",
        vec![Expression::mul(vec![
            Expression::integer(3),
            Expression::symbol(x.clone()),
        ])],
    );

    group.bench_function("exp_3x", |b| b.iter(|| exp_expr.derivative(x.clone())));

    // Logarithmic derivative
    let log_expr = Expression::function(
        "ln",
        vec![Expression::pow(
            Expression::symbol(x.clone()),
            Expression::integer(2),
        )],
    );

    group.bench_function("ln_x2", |b| b.iter(|| log_expr.derivative(x.clone())));

    group.finish();
}

fn bench_fast_polynomial(c: &mut Criterion) {
    let mut group = c.benchmark_group("derivatives_fast/polynomial");
    let x = symbol!(x);

    // 3x^3 - 2x^2 + 5x - 1
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

    group.bench_function("degree_3", |b| b.iter(|| poly_expr.derivative(x.clone())));

    group.finish();
}

criterion_group!(
    name = fast_mode_benchmarks;
    config = Criterion::default()
        .measurement_time(Duration::from_secs(5))
        .sample_size(100);
    targets =
        bench_fast_power_rule,
        bench_fast_chain_rule,
        bench_fast_product_rule,
        bench_fast_complex,
        bench_fast_polynomial
);

criterion_main!(fast_mode_benchmarks);
