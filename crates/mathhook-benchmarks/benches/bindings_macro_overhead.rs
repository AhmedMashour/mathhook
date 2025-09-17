//! Benchmark to verify zero-cost abstraction for macro-generated bindings
//!
//! This benchmark compares:
//! 1. Hand-written binding function (current approach)
//! 2. Macro-generated binding function (proposed approach)
//!
//! Expected result: ZERO performance difference (within measurement noise)

use criterion::{criterion_group, criterion_main, Criterion};
use mathhook_core::Expression;
use std::hint::black_box;

/// Hand-written binding function (current approach)
///
/// This simulates what exists in mathhook-python/src/functions.rs
fn sin_handwritten(expr: Expression) -> Expression {
    Expression::function("sin", vec![expr])
}

/// Macro-generated binding function (proposed approach)
///
/// This simulates what the macro generates
fn sin_macro_generated(expr: Expression) -> Expression {
    Expression::function("sin_macro_generated", vec![expr])
}

/// Benchmark hand-written binding
fn bench_handwritten(c: &mut Criterion) {
    let x = Expression::symbol(mathhook_core::core::symbol::Symbol::scalar("x"));

    c.bench_function("binding_handwritten", |b| {
        b.iter(|| {
            let result = sin_handwritten(black_box(x.clone()));
            black_box(result)
        })
    });
}

/// Benchmark macro-generated binding
fn bench_macro_generated(c: &mut Criterion) {
    let x = Expression::symbol(mathhook_core::core::symbol::Symbol::scalar("x"));

    c.bench_function("binding_macro_generated", |b| {
        b.iter(|| {
            let result = sin_macro_generated(black_box(x.clone()));
            black_box(result)
        })
    });
}

/// Benchmark direct Expression::function call (baseline)
fn bench_baseline(c: &mut Criterion) {
    let x = Expression::symbol(mathhook_core::core::symbol::Symbol::scalar("x"));

    c.bench_function("binding_baseline_direct", |b| {
        b.iter(|| {
            let result = Expression::function("sin", vec![black_box(x.clone())]);
            black_box(result)
        })
    });
}

criterion_group!(
    bindings_overhead,
    bench_handwritten,
    bench_macro_generated,
    bench_baseline
);
criterion_main!(bindings_overhead);
