//! Realistic CAS benchmarks that reflect actual mathematical workflows
//! These benchmarks guide SIMD integration by measuring real-world performance patterns

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use mathhook_core::{symbol, Expression, Number, Simplify, Symbol};
use num_bigint::BigInt;
use num_rational::BigRational;
use std::hint::black_box;
use std::time::Duration;

/// Benchmark bulk numeric operations (prime SIMD candidates)
fn bench_bulk_numeric_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("bulk_numeric_operations");

    // Test different sizes to find SIMD thresholds
    for size in [10, 50, 100, 500, 1000].iter() {
        group.throughput(Throughput::Elements(*size as u64));

        // Bulk integer addition (common in polynomial expansion)
        let integers: Vec<Expression> =
            (1..=*size).map(|i| Expression::integer(i as i64)).collect();
        group.bench_with_input(
            BenchmarkId::new("bulk_integer_addition", size),
            size,
            |b, _| b.iter(|| black_box(Expression::add(integers.clone()).simplify())),
        );

        // Bulk rational addition (common in fraction arithmetic)
        let rationals: Vec<Expression> = (1..=*size)
            .map(|i| {
                Expression::Number(Number::rational(BigRational::new(
                    BigInt::from(i),
                    BigInt::from(i + 1),
                )))
            })
            .collect();
        group.bench_with_input(
            BenchmarkId::new("bulk_rational_addition", size),
            size,
            |b, _| b.iter(|| black_box(Expression::add(rationals.clone()).simplify())),
        );

        // Bulk multiplication (matrix-like operations)
        let factors: Vec<Expression> = (1..=*size).map(|i| Expression::integer(i as i64)).collect();
        group.bench_with_input(
            BenchmarkId::new("bulk_multiplication", size),
            size,
            |b, _| b.iter(|| black_box(Expression::mul(factors.clone()).simplify())),
        );
    }

    group.finish();
}

/// Benchmark matrix operations (perfect for SIMD)
fn bench_matrix_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("matrix_operations");

    // Test different matrix sizes: small to large
    for size in [2, 3, 4, 8, 16, 32].iter() {
        group.throughput(Throughput::Elements((size * size) as u64));

        // Create test matrices
        let matrix_a = create_test_matrix(*size, |i, j| (i * size + j + 1) as i64);
        let matrix_b = create_test_matrix(*size, |i, j| ((i + j) * 2 + 1) as i64);

        group.bench_with_input(BenchmarkId::new("matrix_addition", size), size, |b, _| {
            b.iter(|| {
                black_box(Expression::add(vec![matrix_a.clone(), matrix_b.clone()]).simplify())
            })
        });

        group.bench_with_input(
            BenchmarkId::new("matrix_multiplication", size),
            size,
            |b, _| {
                b.iter(|| {
                    black_box(Expression::mul(vec![matrix_a.clone(), matrix_b.clone()]).simplify())
                })
            },
        );
    }

    group.finish();
}

/// Comprehensive matrix benchmarks per profiling doc recommendations
fn bench_matrix_comprehensive(c: &mut Criterion) {
    use mathhook_core::matrices::CoreMatrixOps;

    let mut group = c.benchmark_group("matrix_comprehensive");

    // Sizes per profiling doc: 2, 10, 50, 100
    for size in [2, 4, 8, 16, 32, 50].iter() {
        let n = *size;
        group.throughput(Throughput::Elements((n * n) as u64));

        // Create dense test matrix
        let matrix = create_dense_matrix(n);
        let matrix2 = create_dense_matrix_offset(n, 100);

        // Matrix transpose
        group.bench_with_input(BenchmarkId::new("transpose", n), &n, |b, _| {
            b.iter(|| black_box(matrix.transpose()))
        });

        // Matrix trace
        group.bench_with_input(BenchmarkId::new("trace", n), &n, |b, _| {
            b.iter(|| black_box(matrix.trace()))
        });

        // Matrix determinant (now O(n³) via LU!)
        if n <= 32 {
            group.bench_with_input(BenchmarkId::new("determinant", n), &n, |b, _| {
                b.iter(|| black_box(matrix.determinant()))
            });
        }

        // Direct CoreMatrixOps (bypassing Expression wrapper)
        group.bench_with_input(BenchmarkId::new("core_add", n), &n, |b, _| {
            b.iter(|| black_box(matrix.add(&matrix2)))
        });

        group.bench_with_input(BenchmarkId::new("core_multiply", n), &n, |b, _| {
            b.iter(|| black_box(matrix.multiply(&matrix2)))
        });

        // Matrix inverse (for smaller matrices)
        if n <= 16 {
            group.bench_with_input(BenchmarkId::new("inverse", n), &n, |b, _| {
                b.iter(|| black_box(matrix.inverse()))
            });
        }
    }

    group.finish();
}

/// Benchmark matrix decompositions
fn bench_matrix_decompositions(c: &mut Criterion) {
    let mut group = c.benchmark_group("matrix_decompositions");

    // Decompositions for smaller matrices (expensive operations)
    for size in [2, 3, 4, 8, 16].iter() {
        let n = *size;
        let matrix = create_dense_matrix(n);

        // LU decomposition
        group.bench_with_input(BenchmarkId::new("lu_decomposition", n), &n, |b, _| {
            b.iter(|| black_box(matrix.lu_decomposition()))
        });

        // QR decomposition
        group.bench_with_input(BenchmarkId::new("qr_decomposition", n), &n, |b, _| {
            b.iter(|| black_box(matrix.qr_decomposition()))
        });

        // Cholesky (for positive definite matrices)
        let pd_matrix = create_positive_definite_matrix(n);
        group.bench_with_input(BenchmarkId::new("cholesky_decomposition", n), &n, |b, _| {
            b.iter(|| black_box(pd_matrix.cholesky_decomposition()))
        });

        // SVD (expensive)
        if n <= 8 {
            group.bench_with_input(BenchmarkId::new("svd_decomposition", n), &n, |b, _| {
                b.iter(|| black_box(matrix.svd_decomposition()))
            });
        }
    }

    group.finish();
}

/// Benchmark special matrix types (should be O(1) or O(n))
fn bench_matrix_special_types(c: &mut Criterion) {
    use mathhook_core::matrices::{CoreMatrixOps, Matrix};

    let mut group = c.benchmark_group("matrix_special_types");

    for size in [10, 100, 1000].iter() {
        let n = *size;

        // Identity matrix operations (should be O(1))
        let identity = Matrix::identity(n);
        let identity2 = Matrix::identity(n);

        group.bench_with_input(BenchmarkId::new("identity_multiply", n), &n, |b, _| {
            b.iter(|| black_box(identity.multiply(&identity2)))
        });

        group.bench_with_input(BenchmarkId::new("identity_determinant", n), &n, |b, _| {
            b.iter(|| black_box(identity.determinant()))
        });

        group.bench_with_input(BenchmarkId::new("identity_inverse", n), &n, |b, _| {
            b.iter(|| black_box(identity.inverse()))
        });

        // Diagonal matrix operations (should be O(n))
        let diag_elements: Vec<Expression> =
            (1..=n).map(|i| Expression::integer(i as i64)).collect();
        let diagonal = Matrix::diagonal(diag_elements.clone());
        let diagonal2 = Matrix::diagonal(diag_elements);

        group.bench_with_input(BenchmarkId::new("diagonal_multiply", n), &n, |b, _| {
            b.iter(|| black_box(diagonal.multiply(&diagonal2)))
        });

        group.bench_with_input(BenchmarkId::new("diagonal_determinant", n), &n, |b, _| {
            b.iter(|| black_box(diagonal.determinant()))
        });

        // Scalar matrix operations (should be O(1))
        let scalar = Matrix::scalar(n, Expression::integer(5));

        group.bench_with_input(BenchmarkId::new("scalar_determinant", n), &n, |b, _| {
            b.iter(|| black_box(scalar.determinant()))
        });
    }

    group.finish();
}

// Helper functions for matrix benchmark data creation

fn create_dense_matrix(size: usize) -> mathhook_core::matrices::Matrix {
    use mathhook_core::matrices::Matrix;
    let rows: Vec<Vec<Expression>> = (0..size)
        .map(|i| {
            (0..size)
                .map(|j| Expression::integer((i * size + j + 1) as i64))
                .collect()
        })
        .collect();
    Matrix::dense(rows)
}

fn create_dense_matrix_offset(size: usize, offset: i64) -> mathhook_core::matrices::Matrix {
    use mathhook_core::matrices::Matrix;
    let rows: Vec<Vec<Expression>> = (0..size)
        .map(|i| {
            (0..size)
                .map(|j| Expression::integer((i * size + j) as i64 + offset))
                .collect()
        })
        .collect();
    Matrix::dense(rows)
}

fn create_positive_definite_matrix(size: usize) -> mathhook_core::matrices::Matrix {
    use mathhook_core::matrices::Matrix;
    // Create A^T * A which is always positive semi-definite
    // Add identity to make it strictly positive definite
    let rows: Vec<Vec<Expression>> = (0..size)
        .map(|i| {
            (0..size)
                .map(|j| {
                    if i == j {
                        Expression::integer((size + 1) as i64)
                    } else {
                        Expression::integer(1)
                    }
                })
                .collect()
        })
        .collect();
    Matrix::dense(rows)
}

/// Benchmark polynomial operations (Horner's method candidates)
fn bench_polynomial_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("polynomial_operations");

    let x = symbol!(x);

    // Test different polynomial degrees
    for degree in [5, 10, 20, 50, 100].iter() {
        group.throughput(Throughput::Elements(*degree as u64));

        // Dense polynomial: sum of x^i terms
        let dense_poly = create_dense_polynomial(&x, *degree);
        group.bench_with_input(
            BenchmarkId::new("dense_polynomial_simplification", degree),
            degree,
            |b, _| b.iter(|| black_box(dense_poly.clone().simplify())),
        );

        // Polynomial evaluation at a point (Horner's method)
        let eval_point = Expression::integer(2);
        group.bench_with_input(
            BenchmarkId::new("polynomial_evaluation", degree),
            degree,
            |b, _| {
                b.iter(|| {
                    // Substitute x = 2 and simplify
                    let substituted = substitute_symbol(&dense_poly, &x, &eval_point);
                    black_box(substituted.simplify())
                })
            },
        );

        // Polynomial multiplication (convolution-like)
        let poly_a = create_dense_polynomial(&x, *degree / 2);
        let poly_b = create_dense_polynomial(&x, *degree / 2);
        group.bench_with_input(
            BenchmarkId::new("polynomial_multiplication", degree),
            degree,
            |b, _| {
                b.iter(|| {
                    black_box(Expression::mul(vec![poly_a.clone(), poly_b.clone()]).simplify())
                })
            },
        );
    }

    group.finish();
}

/// Benchmark mixed symbolic-numeric operations (realistic CAS usage)
fn bench_mixed_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("mixed_symbolic_numeric");

    let x = symbol!(x);
    let y = symbol!(y);

    // Realistic expression: (2x + 3y + 5)^2 expansion
    let expr = Expression::pow(
        Expression::add(vec![
            Expression::mul(vec![Expression::integer(2), Expression::symbol(x.clone())]),
            Expression::mul(vec![Expression::integer(3), Expression::symbol(y.clone())]),
            Expression::integer(5),
        ]),
        Expression::integer(2),
    );

    group.bench_function("quadratic_expansion", |b| {
        b.iter(|| black_box(expr.clone().simplify()))
    });

    // Rational function simplification: (x^2 - 1)/(x - 1)
    let rational_expr = Expression::mul(vec![
        Expression::add(vec![
            Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
            Expression::integer(-1),
        ]),
        Expression::pow(
            Expression::add(vec![Expression::symbol(x.clone()), Expression::integer(-1)]),
            Expression::integer(-1),
        ),
    ]);

    group.bench_function("rational_simplification", |b| {
        b.iter(|| black_box(rational_expr.clone().simplify()))
    });

    // Large expression with many terms (stress test)
    let large_expr = create_large_mixed_expression(&x, &y, 50);
    group.bench_function("large_mixed_expression", |b| {
        b.iter(|| black_box(large_expr.clone().simplify()))
    });

    group.finish();
}

/// Benchmark expression construction overhead
fn bench_expression_construction(c: &mut Criterion) {
    let mut group = c.benchmark_group("expression_construction");

    // Measure pure construction cost (no simplification)
    for size in [10, 50, 100, 500].iter() {
        group.throughput(Throughput::Elements(*size as u64));

        group.bench_with_input(
            BenchmarkId::new("addition_construction", size),
            size,
            |b, _| {
                b.iter(|| {
                    let terms: Vec<Expression> =
                        (1..=*size).map(|i| Expression::integer(i as i64)).collect();
                    black_box(Expression::add(terms))
                })
            },
        );

        group.bench_with_input(
            BenchmarkId::new("multiplication_construction", size),
            size,
            |b, _| {
                b.iter(|| {
                    let factors: Vec<Expression> =
                        (1..=*size).map(|i| Expression::integer(i as i64)).collect();
                    black_box(Expression::mul(factors))
                })
            },
        );
    }

    group.finish();
}

/// Benchmark memory efficiency patterns
fn bench_memory_patterns(c: &mut Criterion) {
    let mut group = c.benchmark_group("memory_patterns");

    // Expression cloning cost
    let complex_expr = create_large_mixed_expression(&symbol!(x), &symbol!(y), 100);
    group.bench_function("expression_cloning", |b| {
        b.iter(|| black_box(complex_expr.clone()))
    });

    // Memory size verification
    group.bench_function("expression_size", |b| {
        b.iter(|| {
            let size = std::mem::size_of::<Expression>();
            black_box(size)
        })
    });

    group.finish();
}

// Helper functions for benchmark data creation

fn create_test_matrix<F>(size: usize, value_fn: F) -> Expression
where
    F: Fn(usize, usize) -> i64,
{
    let rows: Vec<Vec<Expression>> = (0..size)
        .map(|i| {
            (0..size)
                .map(|j| Expression::integer(value_fn(i, j)))
                .collect()
        })
        .collect();
    Expression::matrix(rows)
}

fn create_dense_polynomial(var: &Symbol, degree: usize) -> Expression {
    let terms: Vec<Expression> = (0..=degree)
        .map(|i| {
            let coeff = Expression::integer((i + 1) as i64);
            if i == 0 {
                coeff
            } else {
                Expression::mul(vec![
                    coeff,
                    Expression::pow(
                        Expression::symbol(var.clone()),
                        Expression::integer(i as i64),
                    ),
                ])
            }
        })
        .collect();
    Expression::add(terms)
}

fn create_large_mixed_expression(x: &Symbol, y: &Symbol, num_terms: usize) -> Expression {
    let terms: Vec<Expression> = (1..=num_terms)
        .map(|i| match i % 4 {
            0 => Expression::integer(i as i64),
            1 => Expression::mul(vec![
                Expression::integer(i as i64),
                Expression::symbol(x.clone()),
            ]),
            2 => Expression::mul(vec![
                Expression::integer(i as i64),
                Expression::symbol(y.clone()),
            ]),
            _ => Expression::mul(vec![
                Expression::integer(i as i64),
                Expression::symbol(x.clone()),
                Expression::symbol(y.clone()),
            ]),
        })
        .collect();
    Expression::add(terms)
}

fn substitute_symbol(expr: &Expression, symbol: &Symbol, value: &Expression) -> Expression {
    // Simple substitution for benchmarking (would use proper substitution in real code)
    match expr {
        Expression::Symbol(s) if s == symbol => value.clone(),
        Expression::Add(terms) => Expression::add(
            terms
                .iter()
                .map(|t| substitute_symbol(t, symbol, value))
                .collect(),
        ),
        Expression::Mul(factors) => Expression::mul(
            factors
                .iter()
                .map(|f| substitute_symbol(f, symbol, value))
                .collect(),
        ),
        Expression::Pow(base, exp) => Expression::pow(
            substitute_symbol(base, symbol, value),
            substitute_symbol(exp, symbol, value),
        ),
        _ => expr.clone(),
    }
}

criterion_group!(
    name = realistic_benchmarks;
    config = Criterion::default()
        .measurement_time(Duration::from_secs(10))
        .sample_size(100);
    targets =
        bench_bulk_numeric_operations,
        bench_matrix_operations,
        bench_matrix_comprehensive,
        bench_matrix_decompositions,
        bench_matrix_special_types,
        bench_polynomial_operations,
        bench_mixed_operations,
        bench_expression_construction,
        bench_memory_patterns
);

criterion_main!(realistic_benchmarks);
