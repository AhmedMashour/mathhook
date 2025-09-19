use criterion::{criterion_group, criterion_main, Criterion};
use mathhook::prelude::*;
use std::hint::black_box;

fn bench_industry_gcd(c: &mut Criterion) {
    c.bench_function("industry_gcd_benchmark", |b| {
        b.iter(|| {
            for i in 1..1000 {
                let a = black_box(Expression::integer(i * 12));
                let b = black_box(Expression::integer(i * 18));
                black_box(a.gcd(&b));
            }
        })
    });
}

fn bench_isolated_gcd(c: &mut Criterion) {
    // Create the Expressions once, outside of the benchmarked loop
    let a = black_box(Expression::integer(12000));
    let b = black_box(Expression::integer(18000));

    c.bench_function("isolated_gcd_benchmark", |d| {
        d.iter(|| {
            // Only measure the GCD operation
            black_box(a.gcd(&b));
        })
    });
}

fn bench_integer_creation(c: &mut Criterion) {
    c.bench_function("integer_creation_benchmark", |b| {
        b.iter(|| {
            black_box(Expression::integer(12000));
        })
    });
}

criterion_group!(
    benches,
    bench_industry_gcd,
    bench_isolated_gcd,
    bench_integer_creation
);
criterion_main!(benches);
