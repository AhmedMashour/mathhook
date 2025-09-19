//! Core optimization benchmarks for MathHook performance measurement

use criterion::{criterion_group, criterion_main, Criterion};
use mathhook_benchmarks::BenchmarkData;
use mathhook_core::{Expression, MathSolver, Simplify, Symbol};
use std::hint::black_box;

/// Benchmark basic arithmetic operations
fn bench_arithmetic_operations(c: &mut Criterion) {
    let x = Symbol::new("x");

    c.bench_function("expression_creation", |b| {
        b.iter(|| {
            black_box(Expression::add(vec![
                Expression::symbol(x.clone()),
                Expression::integer(42),
            ]))
        })
    });

    c.bench_function("simplification", |b| {
        let expr = Expression::add(vec![
            Expression::integer(2),
            Expression::integer(3),
            Expression::integer(5),
        ]);
        b.iter(|| black_box(expr.clone().simplify()))
    });
}

/// Benchmark solver operations
fn bench_solver_operations(c: &mut Criterion) {
    let mut solver = MathSolver::new();
    let equation = Expression::equation(Expression::symbol("x"), Expression::integer(42));
    let variable = Symbol::new("x");

    c.bench_function("basic_solving", |b| {
        b.iter(|| black_box(solver.solve(&equation, &variable)))
    });
}

/// Benchmark polynomial operations
fn bench_polynomial_operations(c: &mut Criterion) {
    c.bench_function("polynomial_creation", |b| {
        b.iter(|| black_box(BenchmarkData::complex_polynomial("x", 10)))
    });

    let poly = BenchmarkData::quadratic_expression("x", 1, -5, 6);
    c.bench_function("polynomial_simplification", |b| {
        b.iter(|| black_box(poly.clone().simplify()))
    });
}

/// Benchmark memory-optimized Expression size
fn bench_memory_efficiency(c: &mut Criterion) {
    c.bench_function("expression_size_verification", |b| {
        b.iter(|| {
            let size = std::mem::size_of::<Expression>();
            black_box(size)
        })
    });
}

criterion_group!(
    benches,
    bench_arithmetic_operations,
    bench_solver_operations,
    bench_polynomial_operations,
    bench_memory_efficiency
);
criterion_main!(benches);
