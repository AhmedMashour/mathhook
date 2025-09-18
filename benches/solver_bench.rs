//! Comprehensive benchmarks for all equation solvers implemented in TDD

use criterion::{criterion_group, criterion_main, Criterion};
use mathhook::algebra::equation_analyzer::EquationAnalyzer;
use mathhook::algebra::solvers::{LinearSolver, PolynomialSolver, QuadraticSolver};
use mathhook::core::{ExpressionArena, SimdOptimized};
use mathhook::prelude::*;
use std::hint::black_box;

/// Benchmark LinearSolver performance
fn bench_linear_solver(c: &mut Criterion) {
    let x = Symbol::new("x");
    let solver = LinearSolver::new();

    c.bench_function("linear_solver_simple", |b| {
        b.iter(|| {
            let equation = Expression::add(vec![
                Expression::mul(vec![
                    Expression::integer(black_box(2)),
                    Expression::symbol(x.clone()),
                ]),
                Expression::integer(black_box(6)),
            ]);
            black_box(solver.solve(&equation, &x))
        })
    });

    c.bench_function("linear_solver_fractional", |b| {
        b.iter(|| {
            let equation = Expression::add(vec![
                Expression::mul(vec![
                    Expression::Number(Number::float(black_box(1.5))),
                    Expression::symbol(x.clone()),
                ]),
                Expression::Number(Number::float(black_box(3.5))),
            ]);
            black_box(solver.solve(&equation, &x))
        })
    });

    c.bench_function("linear_solver_with_steps", |b| {
        b.iter(|| {
            let equation = Expression::add(vec![
                Expression::mul(vec![
                    Expression::integer(black_box(3)),
                    Expression::symbol(x.clone()),
                ]),
                Expression::integer(black_box(9)),
            ]);
            black_box(solver.solve_with_explanation(&equation, &x))
        })
    });
}

/// Benchmark QuadraticSolver performance
fn bench_quadratic_solver(c: &mut Criterion) {
    let x = Symbol::new("x");
    let solver = QuadraticSolver::new();

    c.bench_function("quadratic_solver_two_solutions", |b| {
        b.iter(|| {
            let equation = Expression::add(vec![
                Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
                Expression::mul(vec![
                    Expression::integer(black_box(-5)),
                    Expression::symbol(x.clone()),
                ]),
                Expression::integer(black_box(6)),
            ]);
            black_box(solver.solve(&equation, &x))
        })
    });

    c.bench_function("quadratic_solver_one_solution", |b| {
        b.iter(|| {
            let equation = Expression::add(vec![
                Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
                Expression::mul(vec![
                    Expression::integer(black_box(-6)),
                    Expression::symbol(x.clone()),
                ]),
                Expression::integer(black_box(9)),
            ]);
            black_box(solver.solve(&equation, &x))
        })
    });

    c.bench_function("quadratic_solver_complex", |b| {
        b.iter(|| {
            let equation = Expression::add(vec![
                Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
                Expression::integer(black_box(1)),
            ]);
            black_box(solver.solve(&equation, &x))
        })
    });
}

/// Benchmark PolynomialSolver performance
fn bench_polynomial_solver(c: &mut Criterion) {
    let x = Symbol::new("x");
    let solver = PolynomialSolver::new();

    c.bench_function("polynomial_solver_cubic", |b| {
        b.iter(|| {
            let equation = Expression::add(vec![
                Expression::pow(Expression::symbol(x.clone()), Expression::integer(3)),
                Expression::integer(black_box(-8)),
            ]);
            black_box(solver.solve(&equation, &x))
        })
    });

    c.bench_function("polynomial_solver_quartic", |b| {
        b.iter(|| {
            let equation = Expression::add(vec![
                Expression::pow(Expression::symbol(x.clone()), Expression::integer(4)),
                Expression::integer(black_box(-16)),
            ]);
            black_box(solver.solve(&equation, &x))
        })
    });
}

/// Benchmark Magic Bullets performance impact
fn bench_magic_bullets(c: &mut Criterion) {
    c.bench_function("magic_bullet_number", |b| {
        b.iter(|| {
            let num1 = Number::SmallInt(black_box(12345));
            let num2 = Number::SmallInt(black_box(67890));
            black_box((num1, num2))
        })
    });

    c.bench_function("magic_bullet_expression_size", |b| {
        b.iter(|| {
            let expr = Expression::integer(black_box(42));
            black_box(std::mem::size_of_val(&expr))
        })
    });

    c.bench_function("magic_bullet_simd_operations", |b| {
        b.iter(|| {
            let values = vec![
                black_box(1.0),
                black_box(2.0),
                black_box(3.0),
                black_box(4.0),
            ];
            black_box(SimdOptimized::bulk_add_numeric(&values))
        })
    });

    c.bench_function("magic_bullet_arena_allocation", |b| {
        b.iter(|| {
            let arena = ExpressionArena::new();
            black_box(arena)
        })
    });
}

/// Benchmark overall solver dispatch performance
fn bench_smart_dispatch(c: &mut Criterion) {
    c.bench_function("smart_equation_analysis", |b| {
        b.iter(|| {
            let x = Symbol::new("x");
            let equation = Expression::add(vec![
                Expression::pow(
                    Expression::symbol(x.clone()),
                    Expression::integer(black_box(2)),
                ),
                Expression::mul(vec![
                    Expression::integer(black_box(3)),
                    Expression::symbol(x.clone()),
                ]),
                Expression::integer(black_box(2)),
            ]);
            black_box(EquationAnalyzer::analyze(&equation, &x))
        })
    });
}

criterion_group!(
    solver_benches,
    bench_linear_solver,
    bench_quadratic_solver,
    bench_polynomial_solver,
    bench_magic_bullets,
    bench_smart_dispatch
);

criterion_main!(solver_benches);
