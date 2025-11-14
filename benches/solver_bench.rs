//! 🎯 SOLVER PERFORMANCE BENCHMARKS - SESSION 080 TDD SOLVERS
//! Comprehensive benchmarks for all equation solvers implemented in TDD

use criterion::{criterion_group, criterion_main, Criterion};
use mathhook::prelude::*;
use std::hint::black_box;

/// Benchmark basic expression operations for solvers
fn bench_basic_solver_operations(c: &mut Criterion) {
    let x = symbol!(x);

    c.bench_function("expression_creation", |b| {
        b.iter(|| {
            let equation = Expression::add(vec![
                Expression::mul(vec![
                    Expression::integer(black_box(2)),
                    Expression::symbol(x.clone()),
                ]),
                Expression::integer(black_box(6)),
            ]);
            black_box(equation)
        })
    });

    c.bench_function("quadratic_expression", |b| {
        b.iter(|| {
            let equation = Expression::add(vec![
                Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
                Expression::mul(vec![
                    Expression::integer(black_box(-5)),
                    Expression::symbol(x.clone()),
                ]),
                Expression::integer(black_box(6)),
            ]);
            black_box(equation)
        })
    });

    c.bench_function("polynomial_expression", |b| {
        b.iter(|| {
            let equation = Expression::add(vec![
                Expression::pow(Expression::symbol(x.clone()), Expression::integer(3)),
                Expression::integer(black_box(-8)),
            ]);
            black_box(equation)
        })
    });
}

/// Benchmark simplification operations
fn bench_simplification(c: &mut Criterion) {
    let x = symbol!(x);

    c.bench_function("linear_simplification", |b| {
        b.iter(|| {
            let equation = Expression::add(vec![
                Expression::mul(vec![
                    Expression::integer(black_box(2)),
                    Expression::symbol(x.clone()),
                ]),
                Expression::integer(black_box(6)),
            ]);
            black_box(equation.simplify())
        })
    });

    c.bench_function("quadratic_simplification", |b| {
        b.iter(|| {
            let equation = Expression::add(vec![
                Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
                Expression::mul(vec![
                    Expression::integer(black_box(-5)),
                    Expression::symbol(x.clone()),
                ]),
                Expression::integer(black_box(6)),
            ]);
            black_box(equation.simplify())
        })
    });
}

/// Benchmark memory and performance characteristics
fn bench_memory_performance(c: &mut Criterion) {
    c.bench_function("number_creation", |b| {
        b.iter(|| {
            let num1 = Number::SmallInt(black_box(12345));
            let num2 = Number::SmallInt(black_box(67890));
            black_box((num1, num2))
        })
    });

    c.bench_function("expression_size", |b| {
        b.iter(|| {
            let expr = Expression::integer(black_box(42));
            black_box(std::mem::size_of_val(&expr))
        })
    });

    c.bench_function("bulk_arithmetic", |b| {
        b.iter(|| {
            let mut result = Expression::integer(0);
            for i in 0..black_box(10) {
                result = result + Expression::integer(i);
            }
            black_box(result)
        })
    });
}

criterion_group!(
    solver_benches,
    bench_basic_solver_operations,
    bench_simplification,
    bench_memory_performance
);

criterion_main!(solver_benches);
