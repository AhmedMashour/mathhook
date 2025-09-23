//! Performance Consistency Benchmark
//!
//! This benchmark tests for performance outliers and variance to ensure
//! our optimized operations provide consistent, predictable performance.
//! All operations are now optimized by default.

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use mathhook_core::core::Expression;
use mathhook_core::simplify::Simplify;
use std::hint::black_box;
use std::time::Duration;

/// Test optimized addition for consistent performance
fn bench_optimized_addition(c: &mut Criterion) {
    let mut group = c.benchmark_group("optimized_addition");

    // Test different sizes to check for consistent scaling
    for size in [10, 50, 100, 500, 1000].iter() {
        group.throughput(Throughput::Elements(*size as u64));

        let terms: Vec<Expression> = (0..*size).map(|i| Expression::integer(i as i64)).collect();

        group.bench_with_input(
            BenchmarkId::new("optimized_simplify", size),
            size,
            |b, _| {
                b.iter(|| {
                    let expr = Expression::add(terms.clone());
                    black_box(expr.simplify())
                })
            },
        );

        // Test direct expression creation (no simplification)
        group.bench_with_input(
            BenchmarkId::new("expression_creation", size),
            size,
            |b, _| b.iter(|| black_box(Expression::add(terms.clone()))),
        );
    }

    group.finish();
}

/// Test optimized multiplication for consistent performance
fn bench_optimized_multiplication(c: &mut Criterion) {
    let mut group = c.benchmark_group("optimized_multiplication");

    for size in [10, 50, 100, 500, 1000].iter() {
        group.throughput(Throughput::Elements(*size as u64));

        let factors: Vec<Expression> = (1..=*size).map(|i| Expression::integer(i as i64)).collect();

        group.bench_with_input(
            BenchmarkId::new("optimized_simplify", size),
            size,
            |b, _| {
                b.iter(|| {
                    let expr = Expression::mul(factors.clone());
                    black_box(expr.simplify())
                })
            },
        );

        group.bench_with_input(
            BenchmarkId::new("expression_creation", size),
            size,
            |b, _| b.iter(|| black_box(Expression::mul(factors.clone()))),
        );
    }

    group.finish();
}

/// Test matrix operations for consistent performance
fn bench_optimized_matrix(c: &mut Criterion) {
    let mut group = c.benchmark_group("optimized_matrix");

    for size in [5, 10, 20, 50].iter() {
        group.throughput(Throughput::Elements((size * size) as u64));

        // Create a matrix with numeric elements
        let rows: Vec<Vec<Expression>> = (0..*size)
            .map(|i| {
                (0..*size)
                    .map(|j| Expression::integer((i * size + j) as i64))
                    .collect()
            })
            .collect();

        let matrix_expr = Expression::matrix(rows);

        group.bench_with_input(
            BenchmarkId::new("optimized_matrix_simplify", size),
            size,
            |b, _| b.iter(|| black_box(matrix_expr.simplify())),
        );
    }

    group.finish();
}

/// Variance test - measures consistency across multiple runs
fn bench_variance_test(c: &mut Criterion) {
    let mut group = c.benchmark_group("variance_test");

    // Configure for more samples to detect outliers
    group.sample_size(1000);
    group.measurement_time(Duration::from_secs(30));

    // Test a medium-sized operation that should be very consistent
    let terms: Vec<Expression> = (0..100).map(|i| Expression::integer(i as i64)).collect();

    group.bench_function("addition_variance", |b| {
        b.iter(|| {
            let expr = Expression::add(terms.clone());
            black_box(expr.simplify())
        })
    });

    // Test multiplication consistency
    let factors: Vec<Expression> = (1..=50).map(|i| Expression::integer(i as i64)).collect();

    group.bench_function("multiplication_variance", |b| {
        b.iter(|| {
            let expr = Expression::mul(factors.clone());
            black_box(expr.simplify())
        })
    });

    group.finish();
}

/// Memory allocation test - should show minimal variance
fn bench_allocation_stability(c: &mut Criterion) {
    let mut group = c.benchmark_group("allocation_stability");

    group.bench_function("no_allocation_spikes", |b| {
        b.iter(|| {
            // This should not cause allocation spikes
            let terms: Vec<Expression> = (0..50).map(|i| Expression::integer(i as i64)).collect();

            let expr = Expression::add(terms);
            let result = expr.simplify();
            black_box(result);
        })
    });

    group.bench_function("matrix_no_allocation_spikes", |b| {
        b.iter(|| {
            // This should not cause allocation spikes
            let rows: Vec<Vec<Expression>> = (0..10)
                .map(|i| {
                    (0..10)
                        .map(|j| Expression::integer((i * 10 + j) as i64))
                        .collect()
                })
                .collect();

            let matrix_expr = Expression::matrix(rows);
            let result = matrix_expr.simplify();
            black_box(result);
        })
    });

    group.finish();
}

/// Performance scaling test
fn bench_performance_scaling(c: &mut Criterion) {
    let mut group = c.benchmark_group("performance_scaling");

    // Test how performance scales with input size
    for size in [10, 100, 1000, 10000].iter() {
        group.throughput(Throughput::Elements(*size as u64));

        let terms: Vec<Expression> = (0..*size).map(|i| Expression::integer(i as i64)).collect();

        group.bench_with_input(BenchmarkId::new("addition_scaling", size), size, |b, _| {
            b.iter(|| {
                let expr = Expression::add(terms.clone());
                black_box(expr.simplify())
            })
        });
    }

    group.finish();
}

criterion_group!(
    benches,
    bench_optimized_addition,
    bench_optimized_multiplication,
    bench_optimized_matrix,
    bench_variance_test,
    bench_allocation_stability,
    bench_performance_scaling
);

criterion_main!(benches);
