/// Educational derivative mode benchmarks (step-by-step explanations)
///
/// Tests the `derivative_with_steps()` API that generates complete
/// educational explanations with LaTeX formatting.
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use mathhook_core::calculus::derivatives::DerivativeWithSteps;
use mathhook_core::{symbol, Expression};
use std::time::Duration;

fn bench_educational_power_rule(c: &mut Criterion) {
    let mut group = c.benchmark_group("derivatives_educational/power_rule");
    let x = symbol!(x);

    for power in [2, 5, 10].iter() {
        let expr = Expression::pow(Expression::symbol(x.clone()), Expression::integer(*power));
        group.bench_with_input(BenchmarkId::from_parameter(power), power, |b, _| {
            b.iter(|| expr.derivative_with_steps(&x, 1))
        });
    }

    group.finish();
}

fn bench_educational_chain_rule(c: &mut Criterion) {
    let mut group = c.benchmark_group("derivatives_educational/chain_rule");
    let x = symbol!(x);

    // sin(x^2)
    let expr = Expression::function(
        "sin",
        vec![Expression::pow(
            Expression::symbol(x.clone()),
            Expression::integer(2),
        )],
    );

    group.bench_function("sin_x2", |b| b.iter(|| expr.derivative_with_steps(&x, 1)));

    group.finish();
}

fn bench_educational_product_rule(c: &mut Criterion) {
    let mut group = c.benchmark_group("derivatives_educational/product_rule");
    let x = symbol!(x);

    // x^2 * sin(x)
    let expr = Expression::mul(vec![
        Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
        Expression::function("sin", vec![Expression::symbol(x.clone())]),
    ]);

    group.bench_function("x2_sin_x", |b| b.iter(|| expr.derivative_with_steps(&x, 1)));

    group.finish();
}

fn bench_educational_complex(c: &mut Criterion) {
    let mut group = c.benchmark_group("derivatives_educational/complex");
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
        b.iter(|| complex_expr.derivative_with_steps(&x, 1))
    });

    // Trigonometric derivative
    let trig_expr = Expression::function(
        "sin",
        vec![Expression::mul(vec![
            Expression::integer(2),
            Expression::symbol(x.clone()),
        ])],
    );

    group.bench_function("sin_2x", |b| {
        b.iter(|| trig_expr.derivative_with_steps(&x, 1))
    });

    group.finish();
}

fn bench_educational_quotient_rule(c: &mut Criterion) {
    let mut group = c.benchmark_group("derivatives_educational/quotient_rule");
    let x = symbol!(x);

    // sin(x) / cos(x)
    let expr = Expression::mul(vec![
        Expression::function("sin", vec![Expression::symbol(x.clone())]),
        Expression::pow(
            Expression::function("cos", vec![Expression::symbol(x.clone())]),
            Expression::integer(-1),
        ),
    ]);

    group.bench_function("sin_x_div_cos_x", |b| {
        b.iter(|| expr.derivative_with_steps(&x, 1))
    });

    group.finish();
}

criterion_group!(
    name = educational_mode_benchmarks;
    config = Criterion::default()
        .measurement_time(Duration::from_secs(10))
        .sample_size(50);  // Fewer samples since these are slower
    targets =
        bench_educational_power_rule,
        bench_educational_chain_rule,
        bench_educational_product_rule,
        bench_educational_complex,
        bench_educational_quotient_rule
);

criterion_main!(educational_mode_benchmarks);
