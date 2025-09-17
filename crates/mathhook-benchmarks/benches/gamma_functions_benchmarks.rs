/// Performance benchmarks for gamma, digamma, and polygamma functions
///
/// Measures both symbolic and numerical evaluation performance
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use mathhook_core::{symbol, Expression, Number, Simplify};
use std::hint::black_box;
use std::time::Duration;

fn bench_gamma_functions(c: &mut Criterion) {
    let mut group = c.benchmark_group("gamma_functions");

    group.bench_function("gamma_integer_5", |b| {
        let expr = Expression::function("gamma", vec![Expression::integer(5)]);
        b.iter(|| black_box(expr.clone().simplify()))
    });

    group.bench_function("gamma_integer_10", |b| {
        let expr = Expression::function("gamma", vec![Expression::integer(10)]);
        b.iter(|| black_box(expr.clone().simplify()))
    });

    group.bench_function("gamma_integer_20", |b| {
        let expr = Expression::function("gamma", vec![Expression::integer(20)]);
        b.iter(|| black_box(expr.clone().simplify()))
    });

    group.bench_function("gamma_half_integer", |b| {
        let expr = Expression::function("gamma", vec![Expression::Number(Number::Float(0.5))]);
        b.iter(|| black_box(expr.clone().simplify()))
    });

    group.bench_function("gamma_float_3_7", |b| {
        let expr = Expression::function("gamma", vec![Expression::Number(Number::Float(3.7))]);
        b.iter(|| black_box(expr.clone().simplify()))
    });

    let x = symbol!(x);
    group.bench_function("gamma_symbolic", |b| {
        let expr = Expression::function("gamma", vec![Expression::symbol(x.clone())]);
        b.iter(|| black_box(expr.clone().simplify()))
    });

    group.finish();
}

fn bench_digamma_functions(c: &mut Criterion) {
    let mut group = c.benchmark_group("digamma_functions");

    group.bench_function("digamma_integer_1", |b| {
        let expr = Expression::function("digamma", vec![Expression::integer(1)]);
        b.iter(|| black_box(expr.clone().simplify()))
    });

    group.bench_function("digamma_integer_5", |b| {
        let expr = Expression::function("digamma", vec![Expression::integer(5)]);
        b.iter(|| black_box(expr.clone().simplify()))
    });

    group.bench_function("digamma_integer_10", |b| {
        let expr = Expression::function("digamma", vec![Expression::integer(10)]);
        b.iter(|| black_box(expr.clone().simplify()))
    });

    group.bench_function("digamma_integer_50", |b| {
        let expr = Expression::function("digamma", vec![Expression::integer(50)]);
        b.iter(|| black_box(expr.clone().simplify()))
    });

    group.bench_function("digamma_integer_100", |b| {
        let expr = Expression::function("digamma", vec![Expression::integer(100)]);
        b.iter(|| black_box(expr.clone().simplify()))
    });

    group.bench_function("digamma_float_1_0", |b| {
        let expr = Expression::function("digamma", vec![Expression::Number(Number::Float(1.0))]);
        b.iter(|| black_box(expr.clone().simplify()))
    });

    group.bench_function("digamma_float_5_0", |b| {
        let expr = Expression::function("digamma", vec![Expression::Number(Number::Float(5.0))]);
        b.iter(|| black_box(expr.clone().simplify()))
    });

    group.bench_function("digamma_float_100_0", |b| {
        let expr = Expression::function("digamma", vec![Expression::Number(Number::Float(100.0))]);
        b.iter(|| black_box(expr.clone().simplify()))
    });

    let x = symbol!(x);
    group.bench_function("digamma_symbolic", |b| {
        let expr = Expression::function("digamma", vec![Expression::symbol(x.clone())]);
        b.iter(|| black_box(expr.clone().simplify()))
    });

    group.finish();
}

fn bench_polygamma_functions(c: &mut Criterion) {
    let mut group = c.benchmark_group("polygamma_functions");

    group.bench_function("polygamma_0_1", |b| {
        let expr = Expression::function(
            "polygamma",
            vec![Expression::integer(0), Expression::integer(1)],
        );
        b.iter(|| black_box(expr.clone().simplify()))
    });

    group.bench_function("polygamma_1_1", |b| {
        let expr = Expression::function(
            "polygamma",
            vec![Expression::integer(1), Expression::integer(1)],
        );
        b.iter(|| black_box(expr.clone().simplify()))
    });

    group.bench_function("polygamma_2_1", |b| {
        let expr = Expression::function(
            "polygamma",
            vec![Expression::integer(2), Expression::integer(1)],
        );
        b.iter(|| black_box(expr.clone().simplify()))
    });

    group.bench_function("polygamma_0_5", |b| {
        let expr = Expression::function(
            "polygamma",
            vec![Expression::integer(0), Expression::integer(5)],
        );
        b.iter(|| black_box(expr.clone().simplify()))
    });

    group.bench_function("polygamma_1_5", |b| {
        let expr = Expression::function(
            "polygamma",
            vec![Expression::integer(1), Expression::integer(5)],
        );
        b.iter(|| black_box(expr.clone().simplify()))
    });

    group.bench_function("polygamma_2_5", |b| {
        let expr = Expression::function(
            "polygamma",
            vec![Expression::integer(2), Expression::integer(5)],
        );
        b.iter(|| black_box(expr.clone().simplify()))
    });

    group.bench_function("polygamma_1_float_1_0", |b| {
        let expr = Expression::function(
            "polygamma",
            vec![
                Expression::integer(1),
                Expression::Number(Number::Float(1.0)),
            ],
        );
        b.iter(|| black_box(expr.clone().simplify()))
    });

    group.bench_function("polygamma_1_float_5_0", |b| {
        let expr = Expression::function(
            "polygamma",
            vec![
                Expression::integer(1),
                Expression::Number(Number::Float(5.0)),
            ],
        );
        b.iter(|| black_box(expr.clone().simplify()))
    });

    group.bench_function("polygamma_2_float_2_0", |b| {
        let expr = Expression::function(
            "polygamma",
            vec![
                Expression::integer(2),
                Expression::Number(Number::Float(2.0)),
            ],
        );
        b.iter(|| black_box(expr.clone().simplify()))
    });

    let x = symbol!(x);
    group.bench_function("polygamma_1_symbolic", |b| {
        let expr = Expression::function(
            "polygamma",
            vec![Expression::integer(1), Expression::symbol(x.clone())],
        );
        b.iter(|| black_box(expr.clone().simplify()))
    });

    group.finish();
}

fn bench_gamma_recurrence(c: &mut Criterion) {
    let mut group = c.benchmark_group("gamma_recurrence");

    for n in [5, 10, 15, 20].iter() {
        group.bench_with_input(BenchmarkId::new("gamma_factorial", n), n, |b, &num| {
            let expr = Expression::function("gamma", vec![Expression::integer(num)]);
            b.iter(|| black_box(expr.clone().simplify()))
        });
    }

    group.finish();
}

fn bench_digamma_series(c: &mut Criterion) {
    let mut group = c.benchmark_group("digamma_series");

    for n in [10, 50, 100, 500].iter() {
        group.bench_with_input(BenchmarkId::new("digamma_large_n", n), n, |b, &num| {
            let expr = Expression::function("digamma", vec![Expression::integer(num)]);
            b.iter(|| black_box(expr.clone().simplify()))
        });
    }

    group.finish();
}

fn bench_polygamma_orders(c: &mut Criterion) {
    let mut group = c.benchmark_group("polygamma_orders");

    for order in 0..=5 {
        group.bench_with_input(
            BenchmarkId::new("polygamma_order", order),
            &order,
            |b, &n| {
                let expr = Expression::function(
                    "polygamma",
                    vec![
                        Expression::integer(n),
                        Expression::Number(Number::Float(2.0)),
                    ],
                );
                b.iter(|| black_box(expr.clone().simplify()))
            },
        );
    }

    group.finish();
}

criterion_group!(
    name = gamma_benchmarks;
    config = Criterion::default()
        .measurement_time(Duration::from_secs(10))
        .sample_size(100);
    targets =
        bench_gamma_functions,
        bench_digamma_functions,
        bench_polygamma_functions,
        bench_gamma_recurrence,
        bench_digamma_series,
        bench_polygamma_orders
);

criterion_main!(gamma_benchmarks);
