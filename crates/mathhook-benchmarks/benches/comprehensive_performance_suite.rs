//! Comprehensive Performance Suite
//!
//! This benchmark suite tests all MathHook performance optimizations:
//! - SIMD operations
//! - GPU acceleration
//! - Parallel processing
//! - Memoization
//! - Adaptive thresholds
//! - Background precomputation

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use mathhook_core::{
    core::performance::{
        clear_cache, get_adaptive_thresholds, get_gpu_capabilities, get_performance_metrics,
        is_gpu_available, record_performance, submit_background_task, ComputePriority,
    },
    Expression, Number, Simplify,
};
use num_bigint::BigInt;
use num_rational::BigRational;
use std::hint::black_box;
use std::time::Instant;

/// Benchmark SIMD-optimized bulk operations
fn bench_simd_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("simd_operations");

    // Test different sizes to verify SIMD thresholds
    for size in [10, 50, 100, 500, 1000, 5000, 10000].iter() {
        group.throughput(Throughput::Elements(*size as u64));

        // SIMD bulk addition
        let integers: Vec<Expression> =
            (1..=*size).map(|i| Expression::integer(i as i64)).collect();

        group.bench_with_input(
            BenchmarkId::new("simd_bulk_addition", size),
            size,
            |b, _| b.iter(|| black_box(Expression::add(integers.clone()).simplify())),
        );

        // SIMD bulk multiplication
        let factors: Vec<Expression> = (1..=*size).map(|i| Expression::integer(i as i64)).collect();

        group.bench_with_input(
            BenchmarkId::new("simd_bulk_multiplication", size),
            size,
            |b, _| b.iter(|| black_box(Expression::mul(factors.clone()).simplify())),
        );

        // Mixed numeric operations (realistic workload)
        let mixed_terms: Vec<Expression> = (1..=*size)
            .map(|i| {
                if i % 3 == 0 {
                    Expression::Number(Number::rational(BigRational::new(
                        BigInt::from(i),
                        BigInt::from(i + 1),
                    )))
                } else if i % 2 == 0 {
                    Expression::Number(Number::Float(i as f64 * 0.5))
                } else {
                    Expression::integer(i as i64)
                }
            })
            .collect();

        group.bench_with_input(
            BenchmarkId::new("simd_mixed_numeric", size),
            size,
            |b, _| b.iter(|| black_box(Expression::add(mixed_terms.clone()).simplify())),
        );
    }

    group.finish();
}

/// Benchmark GPU acceleration (if available)
fn bench_gpu_operations(c: &mut Criterion) {
    if !is_gpu_available() {
        println!("⚠️ GPU not available, skipping GPU benchmarks");
        return;
    }

    let caps = get_gpu_capabilities().unwrap();
    println!(
        "🚀 GPU Available: {} ({:?})",
        caps.device_name, caps.backend
    );

    let mut group = c.benchmark_group("gpu_operations");

    // Test GPU thresholds for bulk operations
    for size in [1000, 5000, 10000, 50000, 100000].iter() {
        group.throughput(Throughput::Elements(*size as u64));

        // GPU bulk addition
        let large_integers: Vec<Expression> =
            (1..=*size).map(|i| Expression::integer(i as i64)).collect();

        group.bench_with_input(BenchmarkId::new("gpu_bulk_addition", size), size, |b, _| {
            b.iter(|| black_box(Expression::add(large_integers.clone()).simplify()))
        });

        // Large matrix operations (GPU candidates)
        if *size <= 1000 {
            // Reasonable matrix sizes
            let matrix_size = (*size as f64).sqrt() as usize;
            if matrix_size >= 10 {
                let matrix_rows: Vec<Vec<Expression>> = (0..matrix_size)
                    .map(|i| {
                        (0..matrix_size)
                            .map(|j| Expression::integer((i * matrix_size + j) as i64))
                            .collect()
                    })
                    .collect();

                let matrix_expr = Expression::matrix(matrix_rows);

                group.bench_with_input(
                    BenchmarkId::new("gpu_matrix_operations", matrix_size),
                    &matrix_size,
                    |b, _| b.iter(|| black_box(matrix_expr.clone().simplify())),
                );
            }
        }
    }

    group.finish();
}

/// Benchmark memoization effectiveness
fn bench_memoization(c: &mut Criterion) {
    let mut group = c.benchmark_group("memoization");

    // Clear cache for clean test
    clear_cache();

    // Create complex expressions that benefit from memoization
    let complex_expr = Expression::add(vec![
        Expression::mul(vec![
            Expression::symbol("x"),
            Expression::pow(Expression::symbol("y"), Expression::integer(2)),
        ]),
        Expression::mul(vec![Expression::integer(3), Expression::symbol("z")]),
        Expression::pow(Expression::symbol("x"), Expression::integer(3)),
    ]);

    // First run (cold cache)
    group.bench_function("memoization_cold_cache", |b| {
        b.iter(|| {
            clear_cache(); // Ensure cold cache
            black_box(complex_expr.clone().simplify())
        })
    });

    // Warm up cache
    for _ in 0..10 {
        complex_expr.clone().simplify();
    }

    // Second run (warm cache)
    group.bench_function("memoization_warm_cache", |b| {
        b.iter(|| black_box(complex_expr.clone().simplify()))
    });

    // Test cache effectiveness with repeated operations
    let repeated_exprs: Vec<Expression> = (0..100)
        .map(|i| {
            if i % 10 == 0 {
                // 10% unique expressions
                Expression::add(vec![
                    Expression::symbol(format!("x{}", i)),
                    Expression::integer(i as i64),
                ])
            } else {
                // 90% repeated expressions (should hit cache)
                complex_expr.clone()
            }
        })
        .collect();

    group.bench_function("memoization_cache_hit_rate", |b| {
        b.iter(|| {
            for expr in &repeated_exprs {
                black_box(expr.clone().simplify());
            }
        })
    });

    group.finish();
}

/// Benchmark adaptive thresholds
fn bench_adaptive_thresholds(c: &mut Criterion) {
    let mut group = c.benchmark_group("adaptive_thresholds");

    // Record some performance measurements to train adaptive thresholds
    for size in [10, 50, 100, 500, 1000].iter() {
        let start = Instant::now();
        let terms: Vec<Expression> = (1..=*size).map(|i| Expression::integer(i as i64)).collect();
        let _ = Expression::add(terms).simplify();
        let duration = start.elapsed();

        record_performance("adaptive_training", *size, duration);
    }

    // Get current adaptive thresholds
    let thresholds = get_adaptive_thresholds();
    println!(
        "📊 Adaptive Thresholds: SIMD={}, Parallel={}, Confidence={:.2}",
        thresholds.simd_threshold, thresholds.parallel_threshold, thresholds.confidence
    );

    // Test operations around threshold boundaries
    let sizes_around_threshold = [
        thresholds.simd_threshold / 2,
        thresholds.simd_threshold,
        thresholds.simd_threshold * 2,
    ];

    for size in sizes_around_threshold.iter() {
        if *size > 0 {
            group.throughput(Throughput::Elements(*size as u64));

            let terms: Vec<Expression> =
                (1..=*size).map(|i| Expression::integer(i as i64)).collect();

            group.bench_with_input(
                BenchmarkId::new("adaptive_threshold_test", size),
                size,
                |b, _| b.iter(|| black_box(Expression::add(terms.clone()).simplify())),
            );
        }
    }

    group.finish();
}

/// Benchmark background precomputation
fn bench_background_precomputation(c: &mut Criterion) {
    let mut group = c.benchmark_group("background_precomputation");

    // Submit common expressions for background computation
    let common_expressions = [
        Expression::add(vec![Expression::symbol("x"), Expression::integer(1)]),
        Expression::mul(vec![Expression::symbol("x"), Expression::integer(2)]),
        Expression::pow(Expression::symbol("x"), Expression::integer(2)),
    ];

    for (i, expr) in common_expressions.iter().enumerate() {
        submit_background_task(expr.clone(), ComputePriority::High, 0.9);

        group.bench_with_input(BenchmarkId::new("background_precompute", i), &i, |b, _| {
            b.iter(|| black_box(expr.clone().simplify()))
        });
    }

    // Test predictive precomputation
    let base_expr = Expression::mul(vec![Expression::symbol("a"), Expression::symbol("b")]);

    group.bench_function("predictive_precomputation", |b| {
        b.iter(|| {
            let result = black_box(base_expr.clone().simplify());
            // This should trigger predictive precomputation for likely next operations
            black_box(result)
        })
    });

    group.finish();
}

/// Benchmark parallel processing
fn bench_parallel_processing(c: &mut Criterion) {
    let mut group = c.benchmark_group("parallel_processing");

    // Test parallel thresholds
    for size in [100, 500, 1000, 5000, 10000].iter() {
        group.throughput(Throughput::Elements(*size as u64));

        // Large bulk operations that should use parallelism
        let large_terms: Vec<Expression> =
            (1..=*size).map(|i| Expression::integer(i as i64)).collect();

        group.bench_with_input(
            BenchmarkId::new("parallel_bulk_addition", size),
            size,
            |b, _| b.iter(|| black_box(Expression::add(large_terms.clone()).simplify())),
        );

        // Complex expressions that benefit from parallel processing
        let complex_terms: Vec<Expression> = (1..=*size)
            .map(|i| {
                Expression::mul(vec![
                    Expression::symbol(format!("x{}", i)),
                    Expression::integer(i as i64),
                    Expression::pow(Expression::symbol("y"), Expression::integer(2)),
                ])
            })
            .collect();

        group.bench_with_input(
            BenchmarkId::new("parallel_complex_operations", size),
            size,
            |b, _| b.iter(|| black_box(Expression::add(complex_terms.clone()).simplify())),
        );
    }

    group.finish();
}

/// Comprehensive performance metrics reporting
fn bench_performance_metrics(c: &mut Criterion) {
    let mut group = c.benchmark_group("performance_metrics");

    // Benchmark the performance monitoring itself
    group.bench_function("get_performance_metrics", |b| {
        b.iter(|| black_box(get_performance_metrics()))
    });

    // Test performance under monitoring
    let test_expr = Expression::add(vec![
        Expression::integer(1),
        Expression::integer(2),
        Expression::integer(3),
    ]);

    group.bench_function("operations_with_monitoring", |b| {
        b.iter(|| black_box(test_expr.clone().simplify()))
    });

    // Report final performance metrics
    let metrics = get_performance_metrics();
    println!("\n📊 Final Performance Metrics:");
    println!("   SIMD Enabled: {}", metrics.config.simd_enabled);
    println!("   Parallel Enabled: {}", metrics.config.parallel_enabled);
    println!(
        "   Memoization Enabled: {}",
        metrics.config.memoization_enabled
    );
    println!(
        "   Cache Size: {}/{}",
        metrics.cache.current_size, metrics.cache.max_size
    );
    println!(
        "   Cache Utilization: {:.1}%",
        metrics.cache.utilization_percent
    );

    group.finish();
}

/// Realistic workflow benchmarks
fn bench_realistic_workflows(c: &mut Criterion) {
    let mut group = c.benchmark_group("realistic_workflows");

    // Polynomial expansion and simplification
    let polynomial = Expression::pow(
        Expression::add(vec![
            Expression::symbol("x"),
            Expression::symbol("y"),
            Expression::integer(1),
        ]),
        Expression::integer(5),
    );

    group.bench_function("polynomial_workflow", |b| {
        b.iter(|| black_box(polynomial.clone().simplify()))
    });

    // Matrix operations workflow
    let matrix_a = Expression::matrix(vec![
        vec![Expression::integer(1), Expression::integer(2)],
        vec![Expression::integer(3), Expression::integer(4)],
    ]);

    let matrix_b = Expression::matrix(vec![
        vec![Expression::integer(5), Expression::integer(6)],
        vec![Expression::integer(7), Expression::integer(8)],
    ]);

    group.bench_function("matrix_workflow", |b| {
        b.iter(|| {
            let a_simplified = black_box(matrix_a.clone().simplify());
            let b_simplified = black_box(matrix_b.clone().simplify());
            black_box((a_simplified, b_simplified))
        })
    });

    // Mixed symbolic and numeric workflow
    let mixed_expr = Expression::add(vec![
        Expression::mul(vec![Expression::symbol("x"), Expression::integer(5)]),
        Expression::pow(Expression::symbol("y"), Expression::integer(2)),
        Expression::Number(Number::rational(BigRational::new(
            BigInt::from(3),
            BigInt::from(4),
        ))),
        Expression::Number(Number::Float(2.5)),
    ]);

    group.bench_function("mixed_workflow", |b| {
        b.iter(|| black_box(mixed_expr.clone().simplify()))
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_simd_operations,
    bench_gpu_operations,
    bench_memoization,
    bench_adaptive_thresholds,
    bench_background_precomputation,
    bench_parallel_processing,
    bench_performance_metrics,
    bench_realistic_workflows
);

criterion_main!(benches);
