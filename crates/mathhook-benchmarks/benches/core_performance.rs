//! Core optimization benchmarks for MathHook performance measurement

use criterion::{criterion_group, criterion_main, Criterion};
use mathhook_benchmarks::BenchmarkData;
use mathhook_core::{parse, symbol, Expression, MathSolver, Simplify, Symbol};
use std::hint::black_box;

/// Benchmark basic arithmetic operations
fn bench_arithmetic_operations(c: &mut Criterion) {
    let x = symbol!(x);

    c.bench_function("expression_creation", |b| {
        b.iter(|| {
            black_box(Expression::add(vec![
                Expression::symbol(x.clone()),
                Expression::integer(42),
            ]))
        })
    });

    c.bench_function("expression_creation_with_parsing", |b| {
        b.iter(|| {
            let expr = parse!("x + 42").unwrap();
            black_box(expr)
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

    c.bench_function("simplification_with_parsing", |b| {
        b.iter(|| {
            let expr = parse!("2 + 3 + 5").unwrap();
            black_box(expr.simplify())
        })
    });
}

/// Benchmark solver operations
fn bench_solver_operations(c: &mut Criterion) {
    let mut solver = MathSolver::new();
    let equation = Expression::equation(Expression::symbol("x"), Expression::integer(42));
    let variable = symbol!(x);

    c.bench_function("basic_solving", |b| {
        b.iter(|| black_box(solver.solve(&equation, &variable)))
    });

    c.bench_function("basic_solving_with_parsing", |b| {
        b.iter(|| {
            let equation = parse!("x = 42").unwrap();
            let x = Symbol::new("x");
            black_box(solver.solve(&equation, &x))
        })
    });
}

/// Benchmark polynomial operations
fn bench_polynomial_operations(c: &mut Criterion) {
    c.bench_function("polynomial_creation", |b| {
        b.iter(|| black_box(BenchmarkData::complex_polynomial("x", 10)))
    });

    c.bench_function("polynomial_creation_with_parsing", |b| {
        b.iter(|| {
            let expr = parse!(
                "x^10 + 9*x^9 + 8*x^8 + 7*x^7 + 6*x^6 + 5*x^5 + 4*x^4 + 3*x^3 + 2*x^2 + x + 1"
            )
            .unwrap();
            black_box(expr)
        })
    });

    let poly = BenchmarkData::quadratic_expression("x", 1, -5, 6);
    c.bench_function("polynomial_simplification", |b| {
        b.iter(|| black_box(poly.clone().simplify()))
    });

    c.bench_function("polynomial_simplification_with_parsing", |b| {
        b.iter(|| {
            let poly = parse!("x^2 - 5*x + 6").unwrap();
            black_box(poly.simplify())
        })
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
