/// Comprehensive calculus operation benchmarks
///
/// Tests derivatives, integrals, and limits with varying complexity levels.
/// Critical for measuring symbolic calculus performance.

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use mathhook_core::calculus::derivatives::DerivativeWithSteps;
use mathhook_core::calculus::integrals::strategy::integrate_with_strategy;
use mathhook_core::{symbol, Expression, Simplify};
use std::hint::black_box;
use std::time::Duration;

/// Benchmark derivative operations with varying complexity
fn bench_derivatives(c: &mut Criterion) {
    let mut group = c.benchmark_group("derivatives");

    let x = symbol!(x);

    // Power rule derivatives: d/dx(x^n)
    for power in [2, 5, 10, 20, 50].iter() {
        let expr = Expression::pow(
            Expression::symbol(x.clone()),
            Expression::integer(*power),
        );

        group.throughput(Throughput::Elements(1));
        group.bench_with_input(
            BenchmarkId::new("power_rule", power),
            power,
            |b, _| {
                b.iter(|| {
                    black_box(expr.derivative_with_steps(&x, 1))
                })
            },
        );
    }

    // Product rule: d/dx(f(x) * g(x))
    let product_expr = Expression::mul(vec![
        Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
        Expression::function("sin", vec![Expression::symbol(x.clone())]),
    ]);

    group.bench_function("product_rule", |b| {
        b.iter(|| black_box(product_expr.derivative_with_steps(&x, 1)))
    });

    // Chain rule: d/dx(sin(x^2))
    let chain_expr = Expression::function(
        "sin",
        vec![Expression::pow(
            Expression::symbol(x.clone()),
            Expression::integer(2),
        )],
    );

    group.bench_function("chain_rule", |b| {
        b.iter(|| black_box(chain_expr.derivative_with_steps(&x, 1)))
    });

    // Quotient rule: d/dx((x^2 + 1)/(x - 1))
    let quotient_expr = Expression::mul(vec![
        Expression::add(vec![
            Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
            Expression::integer(1),
        ]),
        Expression::pow(
            Expression::add(vec![
                Expression::symbol(x.clone()),
                Expression::integer(-1),
            ]),
            Expression::integer(-1),
        ),
    ]);

    group.bench_function("quotient_rule", |b| {
        b.iter(|| black_box(quotient_expr.derivative_with_steps(&x, 1)))
    });

    // Higher order derivatives
    for order in [1, 2, 3, 5].iter() {
        let expr = Expression::pow(
            Expression::symbol(x.clone()),
            Expression::integer(10),
        );

        group.bench_with_input(
            BenchmarkId::new("higher_order", order),
            order,
            |b, &ord| {
                b.iter(|| {
                    black_box(expr.derivative_with_steps(&x, ord))
                })
            },
        );
    }

    // Trigonometric derivatives
    let trig_expr = Expression::add(vec![
        Expression::function("sin", vec![Expression::symbol(x.clone())]),
        Expression::function("cos", vec![Expression::symbol(x.clone())]),
    ]);

    group.bench_function("trigonometric_derivative", |b| {
        b.iter(|| black_box(trig_expr.derivative_with_steps(&x, 1)))
    });

    // Exponential derivatives
    let exp_expr = Expression::function(
        "exp",
        vec![Expression::mul(vec![
            Expression::integer(2),
            Expression::symbol(x.clone()),
        ])],
    );

    group.bench_function("exponential_derivative", |b| {
        b.iter(|| black_box(exp_expr.derivative_with_steps(&x, 1)))
    });

    // Logarithmic derivatives
    let log_expr = Expression::function(
        "log",
        vec![Expression::pow(
            Expression::symbol(x.clone()),
            Expression::integer(2),
        )],
    );

    group.bench_function("logarithmic_derivative", |b| {
        b.iter(|| black_box(log_expr.derivative_with_steps(&x, 1)))
    });

    // Complex mixed expression
    let complex_expr = Expression::add(vec![
        Expression::mul(vec![
            Expression::integer(3),
            Expression::pow(Expression::symbol(x.clone()), Expression::integer(3)),
        ]),
        Expression::mul(vec![
            Expression::integer(-2),
            Expression::function("sin", vec![Expression::symbol(x.clone())]),
        ]),
        Expression::function(
            "exp",
            vec![Expression::symbol(x.clone())],
        ),
    ]);

    group.bench_function("complex_mixed_derivative", |b| {
        b.iter(|| black_box(complex_expr.derivative_with_steps(&x, 1)))
    });

    group.finish();
}

/// Benchmark integration operations
fn bench_integrals(c: &mut Criterion) {
    let mut group = c.benchmark_group("integrals");

    let x = symbol!(x);

    // Power rule integration: ∫x^n dx
    for power in [1, 2, 5, 10].iter() {
        let expr = Expression::pow(
            Expression::symbol(x.clone()),
            Expression::integer(*power),
        );

        group.throughput(Throughput::Elements(1));
        group.bench_with_input(
            BenchmarkId::new("power_rule", power),
            power,
            |b, _| {
                b.iter(|| {
                    let result = integrate_with_strategy(&expr, x.clone());
                    black_box(result)
                })
            },
        );
    }

    // Trigonometric integrals
    let sin_expr = Expression::function("sin", vec![Expression::symbol(x.clone())]);
    group.bench_function("trigonometric_integral_sin", |b| {
        b.iter(|| {
            let result = integrate_with_strategy(&sin_expr, x.clone());
            black_box(result)
        })
    });

    let cos_expr = Expression::function("cos", vec![Expression::symbol(x.clone())]);
    group.bench_function("trigonometric_integral_cos", |b| {
        b.iter(|| {
            let result = integrate_with_strategy(&cos_expr, x.clone());
            black_box(result)
        })
    });

    // Exponential integrals
    let exp_expr = Expression::function("exp", vec![Expression::symbol(x.clone())]);
    group.bench_function("exponential_integral", |b| {
        b.iter(|| {
            let result = integrate_with_strategy(&exp_expr, x.clone());
            black_box(result)
        })
    });

    // Rational function integration
    let rational_expr = Expression::mul(vec![
        Expression::integer(1),
        Expression::pow(
            Expression::add(vec![
                Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
                Expression::integer(1),
            ]),
            Expression::integer(-1),
        ),
    ]);

    group.bench_function("rational_integral", |b| {
        b.iter(|| {
            let result = integrate_with_strategy(&rational_expr, x.clone());
            black_box(result)
        })
    });

    // Polynomial integration
    for degree in [2, 5, 10].iter() {
        let terms: Vec<Expression> = (0..=*degree)
            .map(|i| {
                Expression::mul(vec![
                    Expression::integer((i + 1) as i64),
                    Expression::pow(
                        Expression::symbol(x.clone()),
                        Expression::integer(i as i64),
                    ),
                ])
            })
            .collect();

        let poly_expr = Expression::add(terms);

        group.bench_with_input(
            BenchmarkId::new("polynomial_integral", degree),
            degree,
            |b, _| {
                b.iter(|| {
                    let result = integrate_with_strategy(&poly_expr, x.clone());
                    black_box(result)
                })
            },
        );
    }

    // Substitution integration: ∫2x * e^(x^2) dx
    let substitution_expr = Expression::mul(vec![
        Expression::mul(vec![
            Expression::integer(2),
            Expression::symbol(x.clone()),
        ]),
        Expression::function(
            "exp",
            vec![Expression::pow(
                Expression::symbol(x.clone()),
                Expression::integer(2),
            )],
        ),
    ]);

    group.bench_function("substitution_integral", |b| {
        b.iter(|| {
            let result = integrate_with_strategy(&substitution_expr, x.clone());
            black_box(result)
        })
    });

    group.finish();
}

/// Benchmark derivative simplification (derivative + simplify)
fn bench_derivative_simplification(c: &mut Criterion) {
    let mut group = c.benchmark_group("derivative_simplification");

    let x = symbol!(x);

    // Derivative that simplifies significantly
    for degree in [3, 5, 10, 20].iter() {
        let expr = Expression::pow(
            Expression::add(vec![
                Expression::symbol(x.clone()),
                Expression::integer(1),
            ]),
            Expression::integer(*degree),
        );

        group.bench_with_input(
            BenchmarkId::new("polynomial_expansion", degree),
            degree,
            |b, _| {
                b.iter(|| {
                    let derivative = expr.derivative_with_steps(&x, 1);
                    black_box(derivative.final_expression.simplify())
                })
            },
        );
    }

    group.finish();
}

/// Benchmark multiple variable derivatives
fn bench_multivariable_derivatives(c: &mut Criterion) {
    let mut group = c.benchmark_group("multivariable_derivatives");

    let x = symbol!(x);
    let y = symbol!(y);

    // Partial derivatives: ∂/∂x(x^2 + y^2)
    let expr = Expression::add(vec![
        Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
        Expression::pow(Expression::symbol(y.clone()), Expression::integer(2)),
    ]);

    group.bench_function("partial_derivative_x", |b| {
        b.iter(|| black_box(expr.derivative_with_steps(&x, 1)))
    });

    group.bench_function("partial_derivative_y", |b| {
        b.iter(|| black_box(expr.derivative_with_steps(&y, 1)))
    });

    // Mixed expression: x*y^2 + sin(x)*cos(y)
    let mixed_expr = Expression::add(vec![
        Expression::mul(vec![
            Expression::symbol(x.clone()),
            Expression::pow(Expression::symbol(y.clone()), Expression::integer(2)),
        ]),
        Expression::mul(vec![
            Expression::function("sin", vec![Expression::symbol(x.clone())]),
            Expression::function("cos", vec![Expression::symbol(y.clone())]),
        ]),
    ]);

    group.bench_function("mixed_partial_x", |b| {
        b.iter(|| black_box(mixed_expr.derivative_with_steps(&x, 1)))
    });

    group.bench_function("mixed_partial_y", |b| {
        b.iter(|| black_box(mixed_expr.derivative_with_steps(&y, 1)))
    });

    group.finish();
}

criterion_group!(
    name = calculus_benchmarks;
    config = Criterion::default()
        .measurement_time(Duration::from_secs(10))
        .sample_size(100);
    targets =
        bench_derivatives,
        bench_integrals,
        bench_derivative_simplification,
        bench_multivariable_derivatives
);

criterion_main!(calculus_benchmarks);
