//! Core optimization benchmarks for MathHook performance measurement

use criterion::{criterion_group, criterion_main, Criterion};
use mathhook::prelude::*;
use std::hint::black_box;

/// Benchmark basic arithmetic operations
fn bench_arithmetic_operations(c: &mut Criterion) {
    let x = symbol!(x);

    c.bench_function("addition_benchmark", |b| {
        b.iter(|| {
            let expr = Expression::integer(black_box(42)) + Expression::integer(black_box(58));
            black_box(expr.simplify())
        })
    });

    c.bench_function("multiplication_benchmark", |b| {
        b.iter(|| {
            let expr = Expression::integer(black_box(6)) * Expression::integer(black_box(7));
            black_box(expr.simplify())
        })
    });

    c.bench_function("symbolic_operations", |b| {
        b.iter(|| {
            let expr = Expression::symbol(x.clone()) + Expression::integer(black_box(1));
            black_box(expr.simplify())
        })
    });
}

/// Benchmark simplification performance
fn bench_simplification_performance(c: &mut Criterion) {
    let x = symbol!(x);

    c.bench_function("simple_simplification", |b| {
        b.iter(|| {
            let expr = Expression::add(vec![
                Expression::integer(black_box(2)),
                Expression::integer(black_box(3)),
                Expression::symbol(x.clone()),
            ]);
            black_box(expr.simplify())
        })
    });

    c.bench_function("complex_simplification", |b| {
        b.iter(|| {
            let expr = Expression::mul(vec![
                Expression::add(vec![
                    Expression::symbol(x.clone()),
                    Expression::integer(black_box(1)),
                ]),
                Expression::integer(black_box(2)),
            ]);
            black_box(expr.simplify())
        })
    });

    c.bench_function("power_simplification", |b| {
        b.iter(|| {
            let expr = Expression::pow(
                Expression::symbol(x.clone()),
                Expression::integer(black_box(1)),
            );
            black_box(expr.simplify())
        })
    });
}

/// Benchmark GCD operations
fn bench_gcd_operations(c: &mut Criterion) {
    c.bench_function("integer_gcd", |b| {
        b.iter(|| {
            let a = Expression::integer(black_box(48));
            let b = Expression::integer(black_box(18));
            black_box(a.gcd(&b))
        })
    });

    c.bench_function("polynomial_gcd", |b| {
        let x = symbol!(x);
        b.iter(|| {
            let poly1 = Expression::mul(vec![
                Expression::integer(black_box(6)),
                Expression::symbol(x.clone()),
            ]);
            let poly2 = Expression::mul(vec![
                Expression::integer(black_box(9)),
                Expression::symbol(x.clone()),
            ]);
            black_box(poly1.gcd(&poly2))
        })
    });
}

/// Benchmark memory optimization
fn bench_memory_optimization(c: &mut Criterion) {
    c.bench_function("number_creation", |b| {
        b.iter(|| {
            let num = Number::SmallInt(black_box(42));
            black_box(Expression::number(num))
        })
    });

    c.bench_function("expression_creation", |b| {
        let x = symbol!(x);
        b.iter(|| {
            let expr = Expression::add(vec![
                Expression::symbol(x.clone()),
                Expression::integer(black_box(5)),
            ]);
            black_box(expr)
        })
    });

    c.bench_function("bulk_operations", |b| {
        let x = symbol!(x);
        b.iter(|| {
            let mut result = Expression::integer(0);
            for i in 0..black_box(100) {
                let term =
                    Expression::add(vec![Expression::symbol(x.clone()), Expression::integer(i)]);
                result = Expression::add(vec![result, term]);
            }
            black_box(result)
        })
    });
}

criterion_group!(
    benches,
    bench_arithmetic_operations,
    bench_simplification_performance,
    bench_gcd_operations,
    bench_memory_optimization
);
criterion_main!(benches);
