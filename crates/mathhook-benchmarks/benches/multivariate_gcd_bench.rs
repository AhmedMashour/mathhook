//! Performance benchmarks for multivariate polynomial GCD
//!
//! Measures performance characteristics of multivariate GCD algorithms
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use mathhook_core::algebra::multivariate_gcd::multivariate_gcd;
use mathhook_core::{expr, symbol, Expression};
use std::hint::black_box;

fn bench_bivariate_simple(c: &mut Criterion) {
    let mut group = c.benchmark_group("bivariate_gcd_simple");

    let x = symbol!(x);
    let y = symbol!(y);
    let vars = vec![x.clone(), y.clone()];

    // Benchmark: gcd(xy, xy)
    group.bench_function("identical", |b| {
        let p1 = Expression::mul(vec![expr!(x), expr!(y)]);
        let p2 = Expression::mul(vec![expr!(x), expr!(y)]);
        b.iter(|| multivariate_gcd(black_box(&p1), black_box(&p2), black_box(&vars)))
    });

    // Benchmark: gcd(x²y, xy)
    group.bench_function("different_degrees", |b| {
        let p1 = Expression::mul(vec![expr!(x ^ 2), expr!(y)]);
        let p2 = Expression::mul(vec![expr!(x), expr!(y)]);
        b.iter(|| multivariate_gcd(black_box(&p1), black_box(&p2), black_box(&vars)))
    });

    group.finish();
}

fn bench_bivariate_complex(c: &mut Criterion) {
    let mut group = c.benchmark_group("bivariate_gcd_complex");

    let x = symbol!(x);
    let y = symbol!(y);
    let vars = vec![x.clone(), y.clone()];

    // Benchmark: gcd(x² - y², x - y)
    group.bench_function("difference_of_squares", |b| {
        let p1 = expr!((x ^ 2) - (y ^ 2));
        let p2 = expr!(x - y);
        b.iter(|| multivariate_gcd(black_box(&p1), black_box(&p2), black_box(&vars)))
    });

    // Benchmark: gcd(x²y + xy², xy)
    group.bench_function("sum_with_common_factor", |b| {
        let p1 = Expression::add(vec![
            Expression::mul(vec![expr!(x ^ 2), expr!(y)]),
            Expression::mul(vec![expr!(x), expr!(y ^ 2)]),
        ]);
        let p2 = Expression::mul(vec![expr!(x), expr!(y)]);
        b.iter(|| multivariate_gcd(black_box(&p1), black_box(&p2), black_box(&vars)))
    });

    group.finish();
}

fn bench_trivariate(c: &mut Criterion) {
    let mut group = c.benchmark_group("trivariate_gcd");

    let x = symbol!(x);
    let y = symbol!(y);
    let z = symbol!(z);
    let vars = vec![x.clone(), y.clone(), z.clone()];

    // Benchmark: gcd(xyz, xy)
    group.bench_function("three_variables", |b| {
        let p1 = Expression::mul(vec![expr!(x), expr!(y), expr!(z)]);
        let p2 = Expression::mul(vec![expr!(x), expr!(y)]);
        b.iter(|| multivariate_gcd(black_box(&p1), black_box(&p2), black_box(&vars)))
    });

    // Benchmark: gcd(x²yz, xy²z)
    group.bench_function("three_variables_complex", |b| {
        let p1 = Expression::mul(vec![expr!(x ^ 2), expr!(y), expr!(z)]);
        let p2 = Expression::mul(vec![expr!(x), expr!(y ^ 2), expr!(z)]);
        b.iter(|| multivariate_gcd(black_box(&p1), black_box(&p2), black_box(&vars)))
    });

    group.finish();
}

fn bench_numeric_content(c: &mut Criterion) {
    let mut group = c.benchmark_group("numeric_content");

    let x = symbol!(x);
    let y = symbol!(y);
    let vars = vec![x.clone(), y.clone()];

    // Benchmark different coefficient sizes
    for coeff in [6, 60, 600, 6000].iter() {
        group.bench_with_input(
            BenchmarkId::new("content_extraction", coeff),
            coeff,
            |b, &c| {
                let p1 = Expression::mul(vec![Expression::integer(c), expr!(x), expr!(y)]);
                let p2 = Expression::mul(vec![Expression::integer(c + 3), expr!(x), expr!(y)]);
                b.iter(|| multivariate_gcd(black_box(&p1), black_box(&p2), black_box(&vars)))
            },
        );
    }

    group.finish();
}

fn bench_edge_cases(c: &mut Criterion) {
    let mut group = c.benchmark_group("edge_cases");

    let x = symbol!(x);
    let y = symbol!(y);
    let vars = vec![x.clone(), y.clone()];

    // Benchmark: gcd(p, 0)
    group.bench_function("with_zero", |b| {
        let p = Expression::mul(vec![expr!(x), expr!(y)]);
        let zero = Expression::integer(0);
        b.iter(|| multivariate_gcd(black_box(&p), black_box(&zero), black_box(&vars)))
    });

    // Benchmark: gcd(p, 1)
    group.bench_function("with_one", |b| {
        let p = Expression::mul(vec![expr!(x), expr!(y)]);
        let one = Expression::integer(1);
        b.iter(|| multivariate_gcd(black_box(&p), black_box(&one), black_box(&vars)))
    });

    // Benchmark: gcd(p, p)
    group.bench_function("identical_polynomial", |b| {
        let p = expr!((x ^ 2) + (y ^ 2));
        b.iter(|| multivariate_gcd(black_box(&p), black_box(&p), black_box(&vars)))
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_bivariate_simple,
    bench_bivariate_complex,
    bench_trivariate,
    bench_numeric_content,
    bench_edge_cases
);
criterion_main!(benches);
