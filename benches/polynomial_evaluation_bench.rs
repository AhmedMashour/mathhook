//! Performance Benchmarks for Polynomial Evaluation
//!
//! Measures performance of all 5 polynomial families using recurrence relations.
//! Target: <1ms for n≤100

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use mathhook_core::functions::polynomials::evaluation::*;

fn benchmark_legendre(c: &mut Criterion) {
    c.bench_function("legendre_p10", |b| {
        b.iter(|| evaluate_legendre_numerical(black_box(&[10.0, 0.5])))
    });

    c.bench_function("legendre_p50", |b| {
        b.iter(|| evaluate_legendre_numerical(black_box(&[50.0, 0.5])))
    });

    c.bench_function("legendre_p100", |b| {
        b.iter(|| evaluate_legendre_numerical(black_box(&[100.0, 0.5])))
    });
}

fn benchmark_hermite(c: &mut Criterion) {
    c.bench_function("hermite_h10", |b| {
        b.iter(|| evaluate_hermite_numerical(black_box(&[10.0, 1.5])))
    });

    c.bench_function("hermite_h50", |b| {
        b.iter(|| evaluate_hermite_numerical(black_box(&[50.0, 1.5])))
    });

    c.bench_function("hermite_h100", |b| {
        b.iter(|| evaluate_hermite_numerical(black_box(&[100.0, 1.5])))
    });
}

fn benchmark_laguerre(c: &mut Criterion) {
    c.bench_function("laguerre_l10", |b| {
        b.iter(|| evaluate_laguerre_numerical(black_box(&[10.0, 1.0])))
    });

    c.bench_function("laguerre_l50", |b| {
        b.iter(|| evaluate_laguerre_numerical(black_box(&[50.0, 1.0])))
    });

    c.bench_function("laguerre_l100", |b| {
        b.iter(|| evaluate_laguerre_numerical(black_box(&[100.0, 1.0])))
    });
}

fn benchmark_chebyshev_first(c: &mut Criterion) {
    c.bench_function("chebyshev_t10", |b| {
        b.iter(|| evaluate_chebyshev_first_numerical(black_box(&[10.0, 0.7])))
    });

    c.bench_function("chebyshev_t50", |b| {
        b.iter(|| evaluate_chebyshev_first_numerical(black_box(&[50.0, 0.7])))
    });

    c.bench_function("chebyshev_t100", |b| {
        b.iter(|| evaluate_chebyshev_first_numerical(black_box(&[100.0, 0.7])))
    });
}

fn benchmark_chebyshev_second(c: &mut Criterion) {
    c.bench_function("chebyshev_u10", |b| {
        b.iter(|| evaluate_chebyshev_second_numerical(black_box(&[10.0, 0.6])))
    });

    c.bench_function("chebyshev_u50", |b| {
        b.iter(|| evaluate_chebyshev_second_numerical(black_box(&[50.0, 0.6])))
    });

    c.bench_function("chebyshev_u100", |b| {
        b.iter(|| evaluate_chebyshev_second_numerical(black_box(&[100.0, 0.6])))
    });
}

criterion_group!(
    benches,
    benchmark_legendre,
    benchmark_hermite,
    benchmark_laguerre,
    benchmark_chebyshev_first,
    benchmark_chebyshev_second
);
criterion_main!(benches);
