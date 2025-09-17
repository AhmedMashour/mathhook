/// Comprehensive simplification benchmarks
///
/// Tests algebraic, trigonometric, logarithmic, and rational simplification
/// with varying complexity levels. Critical for symbolic simplification performance.
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use mathhook_core::{parse, symbol, Expression, Simplify};
use std::hint::black_box;
use std::time::Duration;

/// Benchmark polynomial simplification
fn bench_polynomial_simplification(c: &mut Criterion) {
    let mut group = c.benchmark_group("polynomial_simplification");

    let x = symbol!(x);

    // Collect like terms: 3x + 2x + x
    let like_terms = Expression::add(vec![
        Expression::mul(vec![Expression::integer(3), Expression::symbol(x.clone())]),
        Expression::mul(vec![Expression::integer(2), Expression::symbol(x.clone())]),
        Expression::symbol(x.clone()),
    ]);

    group.bench_function("collect_like_terms", |b| {
        b.iter(|| black_box(like_terms.clone().simplify()))
    });

    group.bench_function("collect_like_terms_with_parsing", |b| {
        b.iter(|| {
            let expr = parse!("3*x + 2*x + x").unwrap();
            black_box(expr.simplify())
        })
    });

    // Expand and simplify: (x + 1)(x + 2)
    let product = Expression::mul(vec![
        Expression::add(vec![Expression::symbol(x.clone()), Expression::integer(1)]),
        Expression::add(vec![Expression::symbol(x.clone()), Expression::integer(2)]),
    ]);

    group.bench_function("expand_product", |b| {
        b.iter(|| black_box(product.clone().simplify()))
    });

    // Simplify powers: x^2 * x^3
    let power_product = Expression::mul(vec![
        Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
        Expression::pow(Expression::symbol(x.clone()), Expression::integer(3)),
    ]);

    group.bench_function("combine_powers", |b| {
        b.iter(|| black_box(power_product.clone().simplify()))
    });

    // Complex polynomial: expand (x + 1)^n
    for power in [2, 5, 10, 15].iter() {
        let binomial = Expression::pow(
            Expression::add(vec![Expression::symbol(x.clone()), Expression::integer(1)]),
            Expression::integer(*power),
        );

        group.bench_with_input(
            BenchmarkId::new("binomial_expansion", power),
            power,
            |b, _| b.iter(|| black_box(binomial.clone().simplify())),
        );
    }

    // Multinomial simplification with multiple variables
    let y = symbol!(y);
    let multinomial = Expression::add(vec![
        Expression::mul(vec![
            Expression::integer(2),
            Expression::symbol(x.clone()),
            Expression::symbol(y.clone()),
        ]),
        Expression::mul(vec![
            Expression::integer(3),
            Expression::symbol(x.clone()),
            Expression::symbol(y.clone()),
        ]),
        Expression::mul(vec![
            Expression::integer(-1),
            Expression::symbol(x.clone()),
            Expression::symbol(y.clone()),
        ]),
    ]);

    group.bench_function("multinomial_simplification", |b| {
        b.iter(|| black_box(multinomial.clone().simplify()))
    });

    group.finish();
}

/// Benchmark trigonometric simplification
fn bench_trigonometric_simplification(c: &mut Criterion) {
    let mut group = c.benchmark_group("trigonometric_simplification");

    let x = symbol!(x);

    // Pythagorean identity: sin^2(x) + cos^2(x) → 1
    let pythagorean = Expression::add(vec![
        Expression::pow(
            Expression::function("sin", vec![Expression::symbol(x.clone())]),
            Expression::integer(2),
        ),
        Expression::pow(
            Expression::function("cos", vec![Expression::symbol(x.clone())]),
            Expression::integer(2),
        ),
    ]);

    group.bench_function("pythagorean_identity", |b| {
        b.iter(|| black_box(pythagorean.clone().simplify()))
    });

    // Double angle: 2*sin(x)*cos(x) → sin(2x)
    let double_angle = Expression::mul(vec![
        Expression::integer(2),
        Expression::function("sin", vec![Expression::symbol(x.clone())]),
        Expression::function("cos", vec![Expression::symbol(x.clone())]),
    ]);

    group.bench_function("double_angle", |b| {
        b.iter(|| black_box(double_angle.clone().simplify()))
    });

    // Reciprocal: 1/sin(x) → csc(x)
    let reciprocal = Expression::mul(vec![
        Expression::integer(1),
        Expression::pow(
            Expression::function("sin", vec![Expression::symbol(x.clone())]),
            Expression::integer(-1),
        ),
    ]);

    group.bench_function("trig_reciprocal", |b| {
        b.iter(|| black_box(reciprocal.clone().simplify()))
    });

    // Quotient: sin(x)/cos(x) → tan(x)
    let quotient = Expression::mul(vec![
        Expression::function("sin", vec![Expression::symbol(x.clone())]),
        Expression::pow(
            Expression::function("cos", vec![Expression::symbol(x.clone())]),
            Expression::integer(-1),
        ),
    ]);

    group.bench_function("trig_quotient", |b| {
        b.iter(|| black_box(quotient.clone().simplify()))
    });

    // Complex trig expression
    let complex_trig = Expression::add(vec![
        Expression::mul(vec![
            Expression::integer(3),
            Expression::pow(
                Expression::function("sin", vec![Expression::symbol(x.clone())]),
                Expression::integer(2),
            ),
        ]),
        Expression::mul(vec![
            Expression::integer(3),
            Expression::pow(
                Expression::function("cos", vec![Expression::symbol(x.clone())]),
                Expression::integer(2),
            ),
        ]),
        Expression::function("sin", vec![Expression::symbol(x.clone())]),
    ]);

    group.bench_function("complex_trig_simplification", |b| {
        b.iter(|| black_box(complex_trig.clone().simplify()))
    });

    group.finish();
}

/// Benchmark logarithmic simplification
fn bench_logarithmic_simplification(c: &mut Criterion) {
    let mut group = c.benchmark_group("logarithmic_simplification");

    let x = symbol!(x);
    let y = symbol!(y);

    // Product rule: log(x) + log(y) → log(x*y)
    let log_product = Expression::add(vec![
        Expression::function("log", vec![Expression::symbol(x.clone())]),
        Expression::function("log", vec![Expression::symbol(y.clone())]),
    ]);

    group.bench_function("log_product_rule", |b| {
        b.iter(|| black_box(log_product.clone().simplify()))
    });

    // Quotient rule: log(x) - log(y) → log(x/y)
    let log_quotient = Expression::add(vec![
        Expression::function("log", vec![Expression::symbol(x.clone())]),
        Expression::mul(vec![
            Expression::integer(-1),
            Expression::function("log", vec![Expression::symbol(y.clone())]),
        ]),
    ]);

    group.bench_function("log_quotient_rule", |b| {
        b.iter(|| black_box(log_quotient.clone().simplify()))
    });

    // Power rule: n*log(x) → log(x^n)
    let log_power = Expression::mul(vec![
        Expression::integer(3),
        Expression::function("log", vec![Expression::symbol(x.clone())]),
    ]);

    group.bench_function("log_power_rule", |b| {
        b.iter(|| black_box(log_power.clone().simplify()))
    });

    // Combined log rules
    let combined = Expression::add(vec![
        Expression::mul(vec![
            Expression::integer(2),
            Expression::function("log", vec![Expression::symbol(x.clone())]),
        ]),
        Expression::function("log", vec![Expression::symbol(y.clone())]),
        Expression::mul(vec![
            Expression::integer(-1),
            Expression::function("log", vec![Expression::integer(4)]),
        ]),
    ]);

    group.bench_function("combined_log_rules", |b| {
        b.iter(|| black_box(combined.clone().simplify()))
    });

    group.finish();
}

/// Benchmark rational expression simplification
fn bench_rational_simplification(c: &mut Criterion) {
    let mut group = c.benchmark_group("rational_simplification");

    let x = symbol!(x);

    // Simple rational: (x^2 - 1)/(x - 1) → x + 1
    let simple_rational = Expression::mul(vec![
        Expression::add(vec![
            Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
            Expression::integer(-1),
        ]),
        Expression::pow(
            Expression::add(vec![Expression::symbol(x.clone()), Expression::integer(-1)]),
            Expression::integer(-1),
        ),
    ]);

    group.bench_function("factor_rational", |b| {
        b.iter(|| black_box(simple_rational.clone().simplify()))
    });

    // Complex rational: (x^3 - 8)/(x - 2)
    let complex_rational = Expression::mul(vec![
        Expression::add(vec![
            Expression::pow(Expression::symbol(x.clone()), Expression::integer(3)),
            Expression::integer(-8),
        ]),
        Expression::pow(
            Expression::add(vec![Expression::symbol(x.clone()), Expression::integer(-2)]),
            Expression::integer(-1),
        ),
    ]);

    group.bench_function("complex_rational_factoring", |b| {
        b.iter(|| black_box(complex_rational.clone().simplify()))
    });

    // Partial fraction decomposition candidate
    let partial_fraction = Expression::mul(vec![
        Expression::add(vec![
            Expression::mul(vec![Expression::integer(2), Expression::symbol(x.clone())]),
            Expression::integer(1),
        ]),
        Expression::pow(
            Expression::mul(vec![
                Expression::add(vec![Expression::symbol(x.clone()), Expression::integer(-1)]),
                Expression::add(vec![Expression::symbol(x.clone()), Expression::integer(1)]),
            ]),
            Expression::integer(-1),
        ),
    ]);

    group.bench_function("partial_fraction_candidate", |b| {
        b.iter(|| black_box(partial_fraction.clone().simplify()))
    });

    // Nested rational expressions
    let nested = Expression::mul(vec![
        Expression::mul(vec![
            Expression::integer(1),
            Expression::pow(
                Expression::add(vec![
                    Expression::integer(1),
                    Expression::mul(vec![
                        Expression::integer(1),
                        Expression::pow(Expression::symbol(x.clone()), Expression::integer(-1)),
                    ]),
                ]),
                Expression::integer(-1),
            ),
        ]),
        Expression::pow(Expression::integer(2), Expression::integer(-1)),
    ]);

    group.bench_function("nested_rational", |b| {
        b.iter(|| black_box(nested.clone().simplify()))
    });

    group.finish();
}

/// Benchmark zero detection and special cases
fn bench_zero_detection(c: &mut Criterion) {
    let mut group = c.benchmark_group("zero_detection");

    let x = symbol!(x);

    // Obvious zero: x - x
    let obvious = Expression::add(vec![
        Expression::symbol(x.clone()),
        Expression::mul(vec![Expression::integer(-1), Expression::symbol(x.clone())]),
    ]);

    group.bench_function("obvious_zero", |b| {
        b.iter(|| black_box(obvious.clone().simplify()))
    });

    // Non-obvious zero: (x - 1)(x + 1) - (x^2 - 1)
    let non_obvious = Expression::add(vec![
        Expression::mul(vec![
            Expression::add(vec![Expression::symbol(x.clone()), Expression::integer(-1)]),
            Expression::add(vec![Expression::symbol(x.clone()), Expression::integer(1)]),
        ]),
        Expression::mul(vec![
            Expression::integer(-1),
            Expression::add(vec![
                Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
                Expression::integer(-1),
            ]),
        ]),
    ]);

    group.bench_function("non_obvious_zero", |b| {
        b.iter(|| black_box(non_obvious.clone().simplify()))
    });

    // Identity simplification: x * 1
    let identity = Expression::mul(vec![Expression::symbol(x.clone()), Expression::integer(1)]);

    group.bench_function("identity_simplification", |b| {
        b.iter(|| black_box(identity.clone().simplify()))
    });

    // Additive identity: x + 0
    let additive = Expression::add(vec![Expression::symbol(x.clone()), Expression::integer(0)]);

    group.bench_function("additive_identity", |b| {
        b.iter(|| black_box(additive.clone().simplify()))
    });

    group.finish();
}

/// Benchmark large expression simplification
fn bench_large_expressions(c: &mut Criterion) {
    let mut group = c.benchmark_group("large_expression_simplification");

    let x = symbol!(x);

    // Many-term polynomial
    for num_terms in [10, 25, 50, 100].iter() {
        let terms: Vec<Expression> = (0..*num_terms)
            .map(|i| {
                Expression::mul(vec![
                    Expression::integer((i % 5 + 1) as i64),
                    Expression::pow(
                        Expression::symbol(x.clone()),
                        Expression::integer((i % 10) as i64),
                    ),
                ])
            })
            .collect();

        let large_poly = Expression::add(terms);

        group.throughput(Throughput::Elements(*num_terms as u64));
        group.bench_with_input(
            BenchmarkId::new("many_term_polynomial", num_terms),
            num_terms,
            |b, _| b.iter(|| black_box(large_poly.clone().simplify())),
        );
    }

    group.finish();
}

criterion_group!(
    name = simplification_benchmarks;
    config = Criterion::default()
        .measurement_time(Duration::from_secs(10))
        .sample_size(100);
    targets =
        bench_polynomial_simplification,
        bench_trigonometric_simplification,
        bench_logarithmic_simplification,
        bench_rational_simplification,
        bench_zero_detection,
        bench_large_expressions
);

criterion_main!(simplification_benchmarks);
