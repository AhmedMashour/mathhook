//! SIMD Performance Analysis Benchmarks
//!
//! These benchmarks measure the performance impact of SIMD operations
//! and help determine optimal thresholds for SIMD activation.

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use mathhook_core::core::Expression;
use mathhook_core::simplify::Simplify;
use num_traits::ToPrimitive;
use std::hint::black_box;
use std::time::Duration;

/// Benchmark optimized vs scalar operations for different data sizes
fn bench_optimized_vs_scalar(c: &mut Criterion) {
    let mut group = c.benchmark_group("optimized_vs_scalar");

    // Test different sizes to find optimization crossover point
    for size in [10, 50, 100, 500, 1000, 5000].iter() {
        group.throughput(Throughput::Elements(*size as u64));

        let a: Vec<f64> = (0..*size).map(|i| i as f64).collect();
        let b: Vec<f64> = (0..*size).map(|i| (i * 2) as f64).collect();

        // Optimized addition (using iterator methods)
        group.bench_with_input(
            BenchmarkId::new("optimized_addition", size),
            size,
            |bench, _| {
                bench.iter(|| {
                    let result: Vec<f64> = a.iter().zip(b.iter()).map(|(x, y)| x + y).collect();
                    black_box(result)
                })
            },
        );

        // Scalar addition
        group.bench_with_input(
            BenchmarkId::new("scalar_addition", size),
            size,
            |bench, _| {
                bench.iter(|| {
                    let mut result = vec![0.0; *size];
                    for i in 0..*size {
                        result[i] = a[i] + b[i];
                    }
                    black_box(result)
                })
            },
        );

        // Optimized multiplication
        group.bench_with_input(
            BenchmarkId::new("optimized_multiplication", size),
            size,
            |bench, _| {
                bench.iter(|| {
                    let result: Vec<f64> = a.iter().zip(b.iter()).map(|(x, y)| x * y).collect();
                    black_box(result)
                })
            },
        );

        // Scalar multiplication
        group.bench_with_input(
            BenchmarkId::new("scalar_multiplication", size),
            size,
            |bench, _| {
                bench.iter(|| {
                    let mut result = vec![0.0; *size];
                    for i in 0..*size {
                        result[i] = a[i] * b[i];
                    }
                    black_box(result)
                })
            },
        );
    }

    group.finish();
}

/// Benchmark optimized dot product vs scalar implementation
fn bench_dot_product_optimized(c: &mut Criterion) {
    let mut group = c.benchmark_group("dot_product_comparison");

    for size in [10, 100, 1000, 10000].iter() {
        group.throughput(Throughput::Elements(*size as u64));

        let a: Vec<f64> = (0..*size).map(|i| i as f64).collect();
        let b: Vec<f64> = (0..*size).map(|i| (i * 2) as f64).collect();

        // Optimized dot product (using iterator methods)
        group.bench_with_input(
            BenchmarkId::new("optimized_dot_product", size),
            size,
            |bench, _| {
                bench.iter(|| black_box(a.iter().zip(b.iter()).map(|(x, y)| x * y).sum::<f64>()))
            },
        );

        // Scalar dot product
        group.bench_with_input(
            BenchmarkId::new("scalar_dot_product", size),
            size,
            |bench, _| {
                bench.iter(|| {
                    let mut sum = 0.0;
                    for i in 0..*size {
                        sum += a[i] * b[i];
                    }
                    black_box(sum)
                })
            },
        );
    }

    group.finish();
}

/// Benchmark polynomial evaluation: optimized vs scalar Horner's method
fn bench_polynomial_evaluation_optimized(c: &mut Criterion) {
    let mut group = c.benchmark_group("polynomial_evaluation");

    for degree in [5, 10, 20, 50, 100].iter() {
        group.throughput(Throughput::Elements(*degree as u64));

        let coefficients: Vec<f64> = (1..=*degree).map(|i| i as f64).collect();
        let x = 2.0;

        // Optimized polynomial evaluation (using fold)
        group.bench_with_input(
            BenchmarkId::new("optimized_horner", degree),
            degree,
            |bench, _| {
                bench.iter(|| {
                    let result = coefficients
                        .iter()
                        .rev()
                        .fold(0.0, |acc, &coeff| acc * x + coeff);
                    black_box(result)
                })
            },
        );

        // Scalar Horner's method
        group.bench_with_input(
            BenchmarkId::new("scalar_horner", degree),
            degree,
            |bench, _| {
                bench.iter(|| {
                    let mut result = coefficients[coefficients.len() - 1];
                    for &coeff in coefficients.iter().rev().skip(1) {
                        result = result * x + coeff;
                    }
                    black_box(result)
                })
            },
        );
    }

    group.finish();
}

/// Benchmark bulk numeric operations in Expression simplification
fn bench_bulk_expression_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("bulk_expression_operations");

    for size in [10, 50, 100, 500, 1000].iter() {
        group.throughput(Throughput::Elements(*size as u64));

        // Test bulk integer addition
        let integers: Vec<Expression> = (0..*size).map(|i| Expression::integer(i as i64)).collect();
        group.bench_with_input(
            BenchmarkId::new("bulk_integer_addition", size),
            size,
            |bench, _| bench.iter(|| black_box(Expression::add(integers.clone()).simplify())),
        );

        // Test bulk float addition
        let floats: Vec<Expression> = (0..*size).map(|i| Expression::float(i as f64)).collect();
        group.bench_with_input(
            BenchmarkId::new("bulk_float_addition", size),
            size,
            |bench, _| bench.iter(|| black_box(Expression::add(floats.clone()).simplify())),
        );

        // Test mixed operations (symbolic + numeric)
        let mut mixed = Vec::with_capacity(*size);
        for i in 0..*size {
            if i % 2 == 0 {
                mixed.push(Expression::integer(i as i64));
            } else {
                mixed.push(Expression::symbol(format!("x{}", i)));
            }
        }
        group.bench_with_input(
            BenchmarkId::new("mixed_expression_simplification", size),
            size,
            |bench, _| bench.iter(|| black_box(Expression::add(mixed.clone()).simplify())),
        );
    }

    group.finish();
}

/// Benchmark optimization suitability detection
fn bench_optimization_detection(c: &mut Criterion) {
    let mut group = c.benchmark_group("optimization_detection");

    // Test different expression compositions
    let all_numeric: Vec<Expression> = (0..100).map(|i| Expression::integer(i as i64)).collect();
    let mixed_50_50: Vec<Expression> = (0..100)
        .map(|i| {
            if i < 50 {
                Expression::integer(i as i64)
            } else {
                Expression::symbol(format!("x{}", i))
            }
        })
        .collect();
    let mostly_symbolic: Vec<Expression> = (0..100)
        .map(|i| Expression::symbol(format!("x{}", i)))
        .collect();

    let test_cases = vec![
        ("all_numeric", all_numeric),
        ("mixed_50_50", mixed_50_50),
        ("mostly_symbolic", mostly_symbolic),
    ];

    for (name, expressions) in test_cases {
        group.bench_function(name, |bench| {
            bench.iter(|| {
                // Simple optimization detection: count numeric vs symbolic
                let numeric_count = expressions
                    .iter()
                    .filter(|e| matches!(e, Expression::Number(_)))
                    .count();
                black_box(numeric_count > expressions.len() / 2)
            })
        });

        group.bench_function(&format!("{}_extract_values", name), |bench| {
            bench.iter(|| {
                let values: Vec<f64> = expressions
                    .iter()
                    .filter_map(|e| match e {
                        Expression::Number(n) => match n {
                            mathhook_core::core::Number::Integer(i) => Some(*i as f64),
                            mathhook_core::core::Number::Float(f) => Some(*f),
                            mathhook_core::core::Number::Rational(r) => r.to_f64(),
                            mathhook_core::core::Number::BigInteger(bi) => bi.to_f64(),
                        },
                        _ => None,
                    })
                    .collect();
                black_box(values)
            })
        });
    }

    group.finish();
}

/// Benchmark optimized bulk summation
fn bench_optimized_bulk_summation(c: &mut Criterion) {
    let mut group = c.benchmark_group("optimized_bulk_summation");

    for size in [100, 500, 1000, 5000].iter() {
        group.throughput(Throughput::Elements(*size as u64));

        let values: Vec<f64> = (1..=*size).map(|i| i as f64).collect();

        // Optimized summation (using iterator sum)
        group.bench_with_input(BenchmarkId::new("optimized_sum", size), size, |bench, _| {
            bench.iter(|| black_box(values.iter().sum::<f64>()))
        });

        // Manual loop summation
        group.bench_with_input(BenchmarkId::new("manual_sum", size), size, |bench, _| {
            bench.iter(|| {
                let mut sum = 0.0;
                for &value in &values {
                    sum += value;
                }
                black_box(sum)
            })
        });

        // Iterator fold summation
        group.bench_with_input(
            BenchmarkId::new("iterator_fold_sum", size),
            size,
            |bench, _| bench.iter(|| black_box(values.iter().fold(0.0, |acc, &x| acc + x))),
        );
    }

    group.finish();
}

criterion_group!(
    name = optimization_benchmarks;
    config = Criterion::default()
        .measurement_time(Duration::from_secs(5))
        .sample_size(50);
    targets =
        bench_optimized_vs_scalar,
        bench_dot_product_optimized,
        bench_polynomial_evaluation_optimized,
        bench_bulk_expression_operations,
        bench_optimization_detection,
        bench_optimized_bulk_summation
);

criterion_main!(optimization_benchmarks);
