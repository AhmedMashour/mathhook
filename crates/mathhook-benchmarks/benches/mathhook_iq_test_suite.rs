//! MathHook IQ Test Suite - Comprehensive performance benchmarks

use criterion::{criterion_group, criterion_main, Criterion};
use mathhook_core::prelude::*;
use std::hint::black_box;

/// Benchmark equation solving performance
fn bench_equation_solver(c: &mut Criterion) {
    let mut solver = MathSolver::new();

    c.bench_function("solve_linear_2x_plus_5_equals_11", |b| {
        let equation = Expression::equation(
            Expression::add(vec![
                Expression::mul(vec![Expression::integer(2), Expression::symbol("x")]),
                Expression::integer(5),
            ]),
            Expression::integer(11),
        );
        let variable = Symbol::new("x");

        b.iter(|| black_box(solver.solve(&equation, &variable)))
    });

    c.bench_function("solve_linear_3x_minus_2_equals_7", |b| {
        let equation = Expression::equation(
            Expression::add(vec![
                Expression::mul(vec![Expression::integer(3), Expression::symbol("x")]),
                Expression::integer(-2),
            ]),
            Expression::integer(7),
        );
        let variable = Symbol::new("x");

        b.iter(|| black_box(solver.solve(&equation, &variable)))
    });

    c.bench_function("solve_quadratic_x2_plus_4x_minus_8_equals_0", |b| {
        let equation = Expression::equation(
            Expression::add(vec![
                Expression::pow(Expression::symbol("x"), Expression::integer(2)),
                Expression::mul(vec![Expression::integer(4), Expression::symbol("x")]),
                Expression::integer(-8),
            ]),
            Expression::integer(0),
        );
        let variable = Symbol::new("x");

        b.iter(|| black_box(solver.solve(&equation, &variable)))
    });
}

/// Benchmark simplification performance
fn bench_simplification(c: &mut Criterion) {
    c.bench_function("simplify_x_plus_2_squared_plus_x_minus_2_squared", |b| {
        let x_plus_2 = Expression::add(vec![Expression::symbol("x"), Expression::integer(2)]);
        let x_minus_2 = Expression::add(vec![Expression::symbol("x"), Expression::integer(-2)]);
        let expr = Expression::add(vec![
            Expression::pow(x_plus_2, Expression::integer(2)),
            Expression::pow(x_minus_2, Expression::integer(2)),
        ]);

        b.iter(|| black_box(expr.clone().simplify()))
    });

    c.bench_function("simplify_2_times_x_plus_3_plus_5_times_x_minus_1", |b| {
        let x_plus_3 = Expression::add(vec![Expression::symbol("x"), Expression::integer(3)]);
        let x_minus_1 = Expression::add(vec![Expression::symbol("x"), Expression::integer(-1)]);
        let expr = Expression::add(vec![
            Expression::mul(vec![Expression::integer(2), x_plus_3]),
            Expression::mul(vec![Expression::integer(5), x_minus_1]),
        ]);

        b.iter(|| black_box(expr.clone().simplify()))
    });

    c.bench_function("simplify_rational_x2_plus_4x_plus_4_div_x_plus_2", |b| {
        let numerator = Expression::add(vec![
            Expression::pow(Expression::symbol("x"), Expression::integer(2)),
            Expression::mul(vec![Expression::integer(4), Expression::symbol("x")]),
            Expression::integer(4),
        ]);
        let denominator = Expression::add(vec![Expression::symbol("x"), Expression::integer(2)]);
        let expr = Expression::function("div", vec![numerator, denominator]);

        b.iter(|| black_box(expr.clone().simplify()))
    });
}

/// Benchmark mixed operations combining solving and simplification
fn bench_mixed_operations(c: &mut Criterion) {
    c.bench_function("solve_then_simplify_workflow", |b| {
        let mut solver = MathSolver::new();

        b.iter(|| {
            let equation = Expression::equation(
                Expression::add(vec![
                    Expression::mul(vec![Expression::integer(2), Expression::symbol("x")]),
                    Expression::integer(5),
                ]),
                Expression::integer(11),
            );
            let solution = solver.solve(&equation, &Symbol::new("x"));

            let expr = Expression::add(vec![Expression::symbol("x"), Expression::integer(1)]);
            let simplified = expr.simplify();

            black_box((solution, simplified))
        })
    });

    c.bench_function("complex_algebraic_manipulation", |b| {
        b.iter(|| {
            let base_expr = Expression::add(vec![
                Expression::pow(Expression::symbol("x"), Expression::integer(2)),
                Expression::mul(vec![Expression::integer(2), Expression::symbol("x")]),
                Expression::integer(1),
            ]);

            let factored = base_expr.clone().factor();
            let expanded = factored.expand();
            let simplified = expanded.simplify();

            black_box(simplified)
        })
    });
}

/// Benchmark expression construction performance
fn bench_expression_construction(c: &mut Criterion) {
    c.bench_function("construct_linear_equations", |b| {
        b.iter(|| {
            let equations = (0..100)
                .map(|i| {
                    Expression::equation(
                        Expression::add(vec![
                            Expression::mul(vec![Expression::integer(i), Expression::symbol("x")]),
                            Expression::integer(i + 1),
                        ]),
                        Expression::integer(i * 2),
                    )
                })
                .collect::<Vec<_>>();

            black_box(equations)
        })
    });

    c.bench_function("construct_quadratic_expressions", |b| {
        b.iter(|| {
            let expressions = (1..50)
                .map(|i| {
                    Expression::add(vec![
                        Expression::pow(Expression::symbol("x"), Expression::integer(2)),
                        Expression::mul(vec![Expression::integer(i), Expression::symbol("x")]),
                        Expression::integer(i * i),
                    ])
                })
                .collect::<Vec<_>>();

            black_box(expressions)
        })
    });
}

/// Benchmark memory efficiency of Expression enum
fn bench_memory_efficiency(c: &mut Criterion) {
    c.bench_function("expression_size_verification", |b| {
        b.iter(|| {
            let size = std::mem::size_of::<Expression>();
            black_box(size)
        })
    });

    c.bench_function("hot_path_allocation_performance", |b| {
        // Pre-allocate the symbol to avoid repeated Arc allocations
        let x_symbol = Expression::symbol("x");

        b.iter(|| {
            // Pre-allocate the vector to avoid reallocations
            let mut expressions = Vec::with_capacity(1000);

            for i in 0..1000 {
                expressions.push(Expression::add(vec![
                    x_symbol.clone(),
                    Expression::integer(i),
                ]));
            }

            black_box(expressions)
        })
    });

    c.bench_function("cold_path_allocation_performance", |b| {
        b.iter(|| {
            let expressions = (0..100)
                .map(|i| {
                    Expression::matrix(vec![
                        vec![Expression::integer(i), Expression::integer(i + 1)],
                        vec![Expression::integer(i + 2), Expression::integer(i + 3)],
                    ])
                })
                .collect::<Vec<_>>();

            black_box(expressions)
        })
    });
}

criterion_group!(
    benches,
    bench_equation_solver,
    bench_simplification,
    bench_mixed_operations,
    bench_expression_construction,
    bench_memory_efficiency
);
criterion_main!(benches);
