/// Comprehensive educational feature benchmarks
///
/// Tests step-by-step explanation generation for derivatives and solving.
/// Focuses on performance of educational content creation.

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use mathhook_core::calculus::derivatives::DerivativeWithSteps;
use mathhook_core::{symbol, Expression};
use std::hint::black_box;
use std::time::Duration;

/// Benchmark derivative step-by-step generation
fn bench_derivative_explanations(c: &mut Criterion) {
    let mut group = c.benchmark_group("derivative_explanations");

    let x = symbol!(x);

    // Power rule explanation: d/dx(x^3)
    group.bench_function("power_rule_explanation", |b| {
        let expr = Expression::pow(
            Expression::symbol(x.clone()),
            Expression::integer(3),
        );
        b.iter(|| black_box(expr.derivative_with_steps(&x, 1)))
    });

    // Product rule explanation: d/dx(x^2 * sin(x))
    group.bench_function("product_rule_explanation", |b| {
        let expr = Expression::mul(vec![
            Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
            Expression::function("sin", vec![Expression::symbol(x.clone())]),
        ]);
        b.iter(|| black_box(expr.derivative_with_steps(&x, 1)))
    });

    // Chain rule explanation: d/dx(sin(x^2))
    group.bench_function("chain_rule_explanation", |b| {
        let expr = Expression::function(
            "sin",
            vec![Expression::pow(
                Expression::symbol(x.clone()),
                Expression::integer(2),
            )],
        );
        b.iter(|| black_box(expr.derivative_with_steps(&x, 1)))
    });

    // Quotient rule explanation: d/dx((x^2 + 1)/(x - 1))
    group.bench_function("quotient_rule_explanation", |b| {
        let expr = Expression::mul(vec![
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
        b.iter(|| black_box(expr.derivative_with_steps(&x, 1)))
    });

    // Complex expression explanation
    group.bench_function("complex_derivative_explanation", |b| {
        let expr = Expression::add(vec![
            Expression::mul(vec![
                Expression::integer(3),
                Expression::pow(Expression::symbol(x.clone()), Expression::integer(4)),
            ]),
            Expression::mul(vec![
                Expression::integer(-2),
                Expression::function("sin", vec![Expression::symbol(x.clone())]),
            ]),
            Expression::function("exp", vec![Expression::symbol(x.clone())]),
        ]);
        b.iter(|| black_box(expr.derivative_with_steps(&x, 1)))
    });

    // Higher order derivative explanation
    for order in [1, 2, 3].iter() {
        group.bench_with_input(
            BenchmarkId::new("higher_order_explanation", order),
            order,
            |b, &ord| {
                let expr = Expression::pow(
                    Expression::symbol(x.clone()),
                    Expression::integer(5),
                );
                b.iter(|| black_box(expr.derivative_with_steps(&x, ord)))
            },
        );
    }

    group.finish();
}

/// Benchmark step explanation complexity
fn bench_step_complexity(c: &mut Criterion) {
    let mut group = c.benchmark_group("step_complexity");

    let x = symbol!(x);

    // Varying expression complexity
    for num_terms in [2, 5, 10, 15].iter() {
        let terms: Vec<Expression> = (1..=*num_terms)
            .map(|i| {
                Expression::mul(vec![
                    Expression::integer(i as i64),
                    Expression::pow(
                        Expression::symbol(x.clone()),
                        Expression::integer(i as i64),
                    ),
                ])
            })
            .collect();

        let poly = Expression::add(terms);

        group.throughput(Throughput::Elements(*num_terms as u64));
        group.bench_with_input(
            BenchmarkId::new("polynomial_explanation", num_terms),
            num_terms,
            |b, _| b.iter(|| black_box(poly.derivative_with_steps(&x, 1))),
        );
    }

    group.finish();
}

/// Benchmark trigonometric explanations
fn bench_trig_explanations(c: &mut Criterion) {
    let mut group = c.benchmark_group("trigonometric_explanations");

    let x = symbol!(x);

    // Basic trig derivatives
    let trig_funcs = ["sin", "cos", "tan"];
    for func in trig_funcs.iter() {
        group.bench_function(&format!("{}_derivative_explanation", func), |b| {
            let expr = Expression::function(*func, vec![Expression::symbol(x.clone())]);
            b.iter(|| black_box(expr.derivative_with_steps(&x, 1)))
        });
    }

    // Trig with chain rule: d/dx(sin(2x))
    group.bench_function("trig_chain_rule_explanation", |b| {
        let expr = Expression::function(
            "sin",
            vec![Expression::mul(vec![
                Expression::integer(2),
                Expression::symbol(x.clone()),
            ])],
        );
        b.iter(|| black_box(expr.derivative_with_steps(&x, 1)))
    });

    // Trig product: d/dx(sin(x)*cos(x))
    group.bench_function("trig_product_explanation", |b| {
        let expr = Expression::mul(vec![
            Expression::function("sin", vec![Expression::symbol(x.clone())]),
            Expression::function("cos", vec![Expression::symbol(x.clone())]),
        ]);
        b.iter(|| black_box(expr.derivative_with_steps(&x, 1)))
    });

    group.finish();
}

/// Benchmark exponential and logarithm explanations
fn bench_exp_log_explanations(c: &mut Criterion) {
    let mut group = c.benchmark_group("exp_log_explanations");

    let x = symbol!(x);

    // Exponential derivative: d/dx(exp(x))
    group.bench_function("exp_derivative_explanation", |b| {
        let expr = Expression::function("exp", vec![Expression::symbol(x.clone())]);
        b.iter(|| black_box(expr.derivative_with_steps(&x, 1)))
    });

    // Log derivative: d/dx(log(x))
    group.bench_function("log_derivative_explanation", |b| {
        let expr = Expression::function("log", vec![Expression::symbol(x.clone())]);
        b.iter(|| black_box(expr.derivative_with_steps(&x, 1)))
    });

    // exp with chain rule: d/dx(exp(x^2))
    group.bench_function("exp_chain_rule_explanation", |b| {
        let expr = Expression::function(
            "exp",
            vec![Expression::pow(
                Expression::symbol(x.clone()),
                Expression::integer(2),
            )],
        );
        b.iter(|| black_box(expr.derivative_with_steps(&x, 1)))
    });

    // log with chain rule: d/dx(log(x^2))
    group.bench_function("log_chain_rule_explanation", |b| {
        let expr = Expression::function(
            "log",
            vec![Expression::pow(
                Expression::symbol(x.clone()),
                Expression::integer(2),
            )],
        );
        b.iter(|| black_box(expr.derivative_with_steps(&x, 1)))
    });

    group.finish();
}

/// Benchmark rational function explanations
fn bench_rational_explanations(c: &mut Criterion) {
    let mut group = c.benchmark_group("rational_explanations");

    let x = symbol!(x);

    // Simple rational: d/dx(1/x)
    group.bench_function("reciprocal_explanation", |b| {
        let expr = Expression::pow(
            Expression::symbol(x.clone()),
            Expression::integer(-1),
        );
        b.iter(|| black_box(expr.derivative_with_steps(&x, 1)))
    });

    // Rational function: d/dx((x^2 + 1)/(x + 1))
    group.bench_function("rational_quotient_explanation", |b| {
        let expr = Expression::mul(vec![
            Expression::add(vec![
                Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
                Expression::integer(1),
            ]),
            Expression::pow(
                Expression::add(vec![
                    Expression::symbol(x.clone()),
                    Expression::integer(1),
                ]),
                Expression::integer(-1),
            ),
        ]);
        b.iter(|| black_box(expr.derivative_with_steps(&x, 1)))
    });

    // Complex rational
    group.bench_function("complex_rational_explanation", |b| {
        let expr = Expression::mul(vec![
            Expression::add(vec![
                Expression::pow(Expression::symbol(x.clone()), Expression::integer(3)),
                Expression::mul(vec![
                    Expression::integer(2),
                    Expression::symbol(x.clone()),
                ]),
            ]),
            Expression::pow(
                Expression::add(vec![
                    Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
                    Expression::integer(1),
                ]),
                Expression::integer(-1),
            ),
        ]);
        b.iter(|| black_box(expr.derivative_with_steps(&x, 1)))
    });

    group.finish();
}

/// Benchmark multivariable explanations
fn bench_multivariable_explanations(c: &mut Criterion) {
    let mut group = c.benchmark_group("multivariable_explanations");

    let x = symbol!(x);
    let y = symbol!(y);

    // Partial derivative: ∂/∂x(x^2 + y^2)
    group.bench_function("partial_derivative_x_explanation", |b| {
        let expr = Expression::add(vec![
            Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
            Expression::pow(Expression::symbol(y.clone()), Expression::integer(2)),
        ]);
        b.iter(|| black_box(expr.derivative_with_steps(&x, 1)))
    });

    group.bench_function("partial_derivative_y_explanation", |b| {
        let expr = Expression::add(vec![
            Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
            Expression::pow(Expression::symbol(y.clone()), Expression::integer(2)),
        ]);
        b.iter(|| black_box(expr.derivative_with_steps(&y, 1)))
    });

    // Mixed expression: ∂/∂x(x*y^2)
    group.bench_function("mixed_partial_explanation", |b| {
        let expr = Expression::mul(vec![
            Expression::symbol(x.clone()),
            Expression::pow(Expression::symbol(y.clone()), Expression::integer(2)),
        ]);
        b.iter(|| black_box(expr.derivative_with_steps(&x, 1)))
    });

    group.finish();
}

/// Benchmark nested function explanations
fn bench_nested_explanations(c: &mut Criterion) {
    let mut group = c.benchmark_group("nested_explanations");

    let x = symbol!(x);

    // Double nesting: d/dx(sin(exp(x)))
    group.bench_function("double_nested_explanation", |b| {
        let expr = Expression::function(
            "sin",
            vec![Expression::function(
                "exp",
                vec![Expression::symbol(x.clone())],
            )],
        );
        b.iter(|| black_box(expr.derivative_with_steps(&x, 1)))
    });

    // Triple nesting: d/dx(log(sin(exp(x))))
    group.bench_function("triple_nested_explanation", |b| {
        let expr = Expression::function(
            "log",
            vec![Expression::function(
                "sin",
                vec![Expression::function(
                    "exp",
                    vec![Expression::symbol(x.clone())],
                )],
            )],
        );
        b.iter(|| black_box(expr.derivative_with_steps(&x, 1)))
    });

    // Nested with polynomial: d/dx(sin((x^2 + 1)^3))
    group.bench_function("nested_polynomial_explanation", |b| {
        let expr = Expression::function(
            "sin",
            vec![Expression::pow(
                Expression::add(vec![
                    Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
                    Expression::integer(1),
                ]),
                Expression::integer(3),
            )],
        );
        b.iter(|| black_box(expr.derivative_with_steps(&x, 1)))
    });

    group.finish();
}

criterion_group!(
    name = educational_benchmarks;
    config = Criterion::default()
        .measurement_time(Duration::from_secs(10))
        .sample_size(100);
    targets =
        bench_derivative_explanations,
        bench_step_complexity,
        bench_trig_explanations,
        bench_exp_log_explanations,
        bench_rational_explanations,
        bench_multivariable_explanations,
        bench_nested_explanations
);

criterion_main!(educational_benchmarks);
