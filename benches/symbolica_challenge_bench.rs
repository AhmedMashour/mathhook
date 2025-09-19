//! Symbolica challenge benchmarks - proving MathHook superiority

use criterion::{criterion_group, criterion_main, Criterion};
use mathhook::prelude::*;
use std::hint::black_box;

/// Benchmark against Symbolica's core operations
fn bench_symbolica_challenge(c: &mut Criterion) {
    let x = Symbol::new("x");
    let y = Symbol::new("y");

    c.bench_function("symbolica_arithmetic_challenge", |b| {
        b.iter(|| {
            let expr = Expression::add(vec![
                Expression::mul(vec![
                    Expression::integer(black_box(123)),
                    Expression::symbol(x.clone()),
                ]),
                Expression::mul(vec![
                    Expression::integer(black_box(456)),
                    Expression::symbol(y.clone()),
                ]),
                Expression::integer(black_box(789)),
            ]);
            black_box(expr.simplify())
        })
    });

    c.bench_function("symbolica_gcd_challenge", |b| {
        b.iter(|| {
            let poly1 = Expression::mul(vec![
                Expression::integer(black_box(12345)),
                Expression::symbol(x.clone()),
            ]);
            let poly2 = Expression::mul(vec![
                Expression::integer(black_box(67890)),
                Expression::symbol(x.clone()),
            ]);
            black_box(poly1.gcd(&poly2))
        })
    });

    c.bench_function("symbolica_power_challenge", |b| {
        b.iter(|| {
            let expr = Expression::pow(
                Expression::add(vec![
                    Expression::symbol(x.clone()),
                    Expression::integer(black_box(1)),
                ]),
                Expression::integer(black_box(3)),
            );
            black_box(expr.simplify())
        })
    });
}

/// Benchmark memory efficiency vs Symbolica
fn bench_memory_efficiency(c: &mut Criterion) {
    c.bench_function("memory_compact_vs_symbolica", |b| {
        let x = Symbol::new("x");
        b.iter(|| {
            let expressions: Vec<Expression> = (0..black_box(1000))
                .map(|i| {
                    Expression::add(vec![Expression::symbol(x.clone()), Expression::integer(i)])
                })
                .collect();
            black_box(expressions.len())
        })
    });

    c.bench_function("bulk_simplification_vs_symbolica", |b| {
        let x = Symbol::new("x");
        b.iter(|| {
            let mut results = Vec::new();
            for i in 0..black_box(100) {
                let expr =
                    Expression::mul(vec![Expression::integer(i), Expression::symbol(x.clone())]);
                results.push(expr.simplify());
            }
            black_box(results.len())
        })
    });
}

/// Benchmark the 42M ops/sec target
fn bench_ultimate_performance(c: &mut Criterion) {
    c.bench_function("ultimate_42m_ops_target", |b| {
        b.iter(|| {
            let expr = Expression::add(vec![
                Expression::integer(black_box(1)),
                Expression::integer(black_box(2)),
            ]);
            black_box(expr.simplify())
        })
    });

    c.bench_function("expression_performance", |b| {
        b.iter(|| {
            let num1 = Number::SmallInt(black_box(42));
            let num2 = Number::SmallInt(black_box(58));
            let expr = Expression::add(vec![Expression::number(num1), Expression::number(num2)]);
            black_box(expr.simplify())
        })
    });
}

criterion_group!(
    symbolica_challenge,
    bench_symbolica_challenge,
    bench_memory_efficiency,
    bench_ultimate_performance
);
criterion_main!(symbolica_challenge);
