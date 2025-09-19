//! Symbolica challenge benchmarks - proving MathHook superiority

use criterion::{criterion_group, criterion_main, Criterion};
use mathhook_benchmarks::BenchmarkData;
use mathhook_core::{Expression, Simplify, Symbol};
use std::hint::black_box;

/// Benchmark against Symbolica's core operations
fn bench_symbolica_challenge(c: &mut Criterion) {
    let x = Symbol::new("x");
    let y = Symbol::new("y");

    c.bench_function("complex_expansion", |b| {
        let expr = Expression::mul(vec![
            Expression::add(vec![Expression::symbol(x.clone()), Expression::integer(1)]),
            Expression::add(vec![Expression::symbol(y.clone()), Expression::integer(2)]),
        );
        b.iter(|| black_box(expr.clone().simplify()))
    });

    c.bench_function("polynomial_gcd", |b| {
        let poly1 = BenchmarkData::quadratic_expression("x", 1, -3, 2);
        let poly2 = BenchmarkData::quadratic_expression("x", 1, -5, 6);
        b.iter(|| black_box((poly1.clone(), poly2.clone())))
    });
}

/// Benchmark high-degree polynomial operations
fn bench_high_degree_polynomials(c: &mut Criterion) {
    c.bench_function("degree_20_polynomial", |b| {
        b.iter(|| {
            let poly = BenchmarkData::complex_polynomial("x", 20);
            black_box(poly.simplify())
        })
    });

    c.bench_function("degree_50_polynomial", |b| {
        b.iter(|| {
            let poly = BenchmarkData::complex_polynomial("x", 50);
            black_box(poly.simplify())
        })
    });
}

/// Benchmark symbolic manipulation speed
fn bench_symbolic_speed(c: &mut Criterion) {
    c.bench_function("million_operations", |b| {
        b.iter(|| {
            let mut expr = Expression::integer(1);
            for i in 1..=1000 {
                expr = Expression::add(vec![expr, Expression::integer(i)]);
            }
            black_box(expr.simplify())
        })
    });
}

criterion_group!(
    benches,
    bench_symbolica_challenge,
    bench_high_degree_polynomials,
    bench_symbolic_speed
);
criterion_main!(benches);
