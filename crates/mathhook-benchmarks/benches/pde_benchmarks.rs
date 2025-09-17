//! Performance benchmarks for PDE module
//!
//! Benchmarks critical PDE operations to establish performance baselines
//! and detect regressions.

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use mathhook_core::calculus::pde::method_of_characteristics::{
    method_of_characteristics, solve_characteristic_odes, PdeCoefficients,
};
use mathhook_core::calculus::pde::types::Pde;
use mathhook_core::{expr, symbol, Expression};
use std::hint::black_box;

/// Benchmark coefficient extraction from PDE
fn bench_coefficient_extraction(c: &mut Criterion) {
    let u = symbol!(u);
    let t = symbol!(t);
    let x = symbol!(x);
    let equation = expr!(u);
    let pde = Pde::new(equation, u, vec![t, x]);

    c.bench_function("pde_coefficient_extraction", |b| {
        b.iter(|| {
            // Extract coefficients (a, b, c) from PDE
            // Currently returns constants, but benchmarks the validation path
            let result = method_of_characteristics(black_box(&pde));
            black_box(result)
        });
    });
}

/// Benchmark ODE system construction from coefficients
fn bench_ode_system_construction(c: &mut Criterion) {
    let coeffs = PdeCoefficients {
        a: Expression::integer(1),
        b: Expression::integer(1),
        c: Expression::integer(0),
    };

    c.bench_function("pde_ode_system_construction", |b| {
        b.iter(|| {
            // Construct characteristic equations: dx/ds, dy/ds, du/ds
            let char_eqs = vec![
                black_box(coeffs.a.clone()),
                black_box(coeffs.b.clone()),
                black_box(coeffs.c.clone()),
            ];
            black_box(char_eqs)
        });
    });
}

/// Benchmark full transport equation solve
fn bench_transport_equation_solve(c: &mut Criterion) {
    let u = symbol!(u);
    let t = symbol!(t);
    let x = symbol!(x);
    let equation = expr!(u);
    let pde = Pde::new(equation, u, vec![t, x]);

    c.bench_function("pde_transport_equation_full_solve", |b| {
        b.iter(|| {
            let result = method_of_characteristics(black_box(&pde));
            black_box(result)
        });
    });
}

/// Benchmark characteristic ODE numerical integration
fn bench_characteristic_odes_numerical(c: &mut Criterion) {
    let char_eqs = vec![
        Expression::integer(1),
        Expression::integer(1),
        Expression::integer(0),
    ];

    let initial_conditions = vec![0.0, 0.0, 1.0];

    let mut group = c.benchmark_group("pde_characteristic_odes_numerical");

    for step_size in [0.1, 0.05, 0.01].iter() {
        group.bench_with_input(
            BenchmarkId::from_parameter(format!("step_{}", step_size)),
            step_size,
            |b, &step_size| {
                b.iter(|| {
                    let result = solve_characteristic_odes(
                        black_box(&char_eqs),
                        black_box(&initial_conditions),
                        black_box(1.0),
                        black_box(step_size),
                    );
                    black_box(result)
                });
            },
        );
    }

    group.finish();
}

/// Benchmark PDE classification overhead
fn bench_pde_classification(c: &mut Criterion) {
    let u = symbol!(u);
    let t = symbol!(t);
    let x = symbol!(x);
    let equation = expr!(u);
    let pde = Pde::new(equation, u, vec![t, x]);

    c.bench_function("pde_classification", |b| {
        b.iter(|| {
            use mathhook_core::calculus::pde::classification::classify_pde;
            let result = classify_pde(black_box(&pde));
            black_box(result)
        });
    });
}

/// Benchmark PDE order detection
fn bench_pde_order_detection(c: &mut Criterion) {
    let u = symbol!(u);
    let t = symbol!(t);
    let x = symbol!(x);
    let equation = expr!(u);
    let pde = Pde::new(equation, u, vec![t, x]);

    c.bench_function("pde_order_detection", |b| {
        b.iter(|| {
            let order = black_box(&pde).order();
            black_box(order)
        });
    });
}

/// Benchmark solution construction overhead
fn bench_solution_construction(c: &mut Criterion) {
    let t = symbol!(t);
    let x = symbol!(x);

    let coeffs = PdeCoefficients {
        a: Expression::integer(1),
        b: Expression::integer(1),
        c: Expression::integer(0),
    };

    c.bench_function("pde_solution_construction", |b| {
        b.iter(|| {
            // Construct general solution F(x - (a/b)*y)
            let x_expr = Expression::symbol(black_box(t.clone()));
            let y_expr = Expression::symbol(black_box(x.clone()));

            let ratio = Expression::mul(vec![
                black_box(coeffs.a.clone()),
                Expression::pow(black_box(coeffs.b.clone()), Expression::integer(-1)),
            ]);

            let arg = Expression::add(vec![
                x_expr,
                Expression::mul(vec![Expression::integer(-1), ratio, y_expr]),
            ]);

            let solution = Expression::function("F", vec![arg]);
            black_box(solution)
        });
    });
}

/// Benchmark memory allocations in PDE solving
fn bench_pde_memory_allocations(c: &mut Criterion) {
    let u = symbol!(u);
    let t = symbol!(t);
    let x = symbol!(x);

    c.bench_function("pde_memory_allocations", |b| {
        b.iter(|| {
            // Measure allocation overhead
            let equation = black_box(expr!(u));
            let pde = black_box(Pde::new(equation, u.clone(), vec![t.clone(), x.clone()]));
            let result = method_of_characteristics(&pde);
            black_box(result)
        });
    });
}

criterion_group!(
    pde_benches,
    bench_coefficient_extraction,
    bench_ode_system_construction,
    bench_transport_equation_solve,
    bench_characteristic_odes_numerical,
    bench_pde_classification,
    bench_pde_order_detection,
    bench_solution_construction,
    bench_pde_memory_allocations,
);

criterion_main!(pde_benches);
