/// Comprehensive function evaluation benchmarks
///
/// Tests elementary functions (sin, cos, exp, log) and special functions
/// with both symbolic and numerical evaluation paths.
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use mathhook_core::{parse, symbol, Expression, Simplify};
use std::hint::black_box;
use std::time::Duration;

/// Benchmark elementary trigonometric functions
fn bench_elementary_trigonometric(c: &mut Criterion) {
    let mut group = c.benchmark_group("elementary_trigonometric");

    let x = symbol!(x);

    // Symbolic evaluation
    group.bench_function("sin_symbolic", |b| {
        let expr = Expression::function("sin", vec![Expression::symbol(x.clone())]);
        b.iter(|| black_box(expr.clone().simplify()))
    });

    group.bench_function("sin_symbolic_with_parsing", |b| {
        b.iter(|| {
            let expr = parse!("sin(x)").unwrap();
            black_box(expr.simplify())
        })
    });

    group.bench_function("cos_symbolic", |b| {
        let expr = Expression::function("cos", vec![Expression::symbol(x.clone())]);
        b.iter(|| black_box(expr.clone().simplify()))
    });

    group.bench_function("tan_symbolic", |b| {
        let expr = Expression::function("tan", vec![Expression::symbol(x.clone())]);
        b.iter(|| black_box(expr.clone().simplify()))
    });

    // Special value evaluation: sin(0), sin(pi/2), etc.
    let special_values = [
        ("sin_zero", "sin", 0),
        ("cos_zero", "cos", 0),
        ("sin_pi_over_2", "sin", 90), // Using degrees as proxy
        ("cos_pi", "cos", 180),
    ];

    for (name, func, value) in special_values.iter() {
        group.bench_function(*name, |b| {
            let expr = Expression::function(*func, vec![Expression::integer(*value)]);
            b.iter(|| black_box(expr.clone().simplify()))
        });
    }

    // Nested trigonometric: sin(cos(x))
    group.bench_function("nested_trig", |b| {
        let expr = Expression::function(
            "sin",
            vec![Expression::function(
                "cos",
                vec![Expression::symbol(x.clone())],
            )],
        );
        b.iter(|| black_box(expr.clone().simplify()))
    });

    // Inverse trigonometric functions
    group.bench_function("arcsin_symbolic", |b| {
        let expr = Expression::function("arcsin", vec![Expression::symbol(x.clone())]);
        b.iter(|| black_box(expr.clone().simplify()))
    });

    group.bench_function("arctan_symbolic", |b| {
        let expr = Expression::function("arctan", vec![Expression::symbol(x.clone())]);
        b.iter(|| black_box(expr.clone().simplify()))
    });

    group.finish();
}

/// Benchmark hyperbolic functions
fn bench_hyperbolic_functions(c: &mut Criterion) {
    let mut group = c.benchmark_group("hyperbolic_functions");

    let x = symbol!(x);

    group.bench_function("sinh_symbolic", |b| {
        let expr = Expression::function("sinh", vec![Expression::symbol(x.clone())]);
        b.iter(|| black_box(expr.clone().simplify()))
    });

    group.bench_function("cosh_symbolic", |b| {
        let expr = Expression::function("cosh", vec![Expression::symbol(x.clone())]);
        b.iter(|| black_box(expr.clone().simplify()))
    });

    group.bench_function("tanh_symbolic", |b| {
        let expr = Expression::function("tanh", vec![Expression::symbol(x.clone())]);
        b.iter(|| black_box(expr.clone().simplify()))
    });

    // Hyperbolic identity: cosh^2(x) - sinh^2(x) = 1
    group.bench_function("hyperbolic_identity", |b| {
        let expr = Expression::add(vec![
            Expression::pow(
                Expression::function("cosh", vec![Expression::symbol(x.clone())]),
                Expression::integer(2),
            ),
            Expression::mul(vec![
                Expression::integer(-1),
                Expression::pow(
                    Expression::function("sinh", vec![Expression::symbol(x.clone())]),
                    Expression::integer(2),
                ),
            ]),
        ]);
        b.iter(|| black_box(expr.clone().simplify()))
    });

    group.finish();
}

/// Benchmark exponential and logarithmic functions
fn bench_exponential_logarithmic(c: &mut Criterion) {
    let mut group = c.benchmark_group("exponential_logarithmic");

    let x = symbol!(x);

    // Exponential evaluation
    group.bench_function("exp_symbolic", |b| {
        let expr = Expression::function("exp", vec![Expression::symbol(x.clone())]);
        b.iter(|| black_box(expr.clone().simplify()))
    });

    // Natural logarithm
    group.bench_function("log_symbolic", |b| {
        let expr = Expression::function("log", vec![Expression::symbol(x.clone())]);
        b.iter(|| black_box(expr.clone().simplify()))
    });

    // Logarithm with base
    group.bench_function("log_base_10", |b| {
        let expr = Expression::function(
            "log",
            vec![Expression::symbol(x.clone()), Expression::integer(10)],
        );
        b.iter(|| black_box(expr.clone().simplify()))
    });

    // exp(log(x)) identity
    group.bench_function("exp_log_identity", |b| {
        let expr = Expression::function(
            "exp",
            vec![Expression::function(
                "log",
                vec![Expression::symbol(x.clone())],
            )],
        );
        b.iter(|| black_box(expr.clone().simplify()))
    });

    // log(exp(x)) identity
    group.bench_function("log_exp_identity", |b| {
        let expr = Expression::function(
            "log",
            vec![Expression::function(
                "exp",
                vec![Expression::symbol(x.clone())],
            )],
        );
        b.iter(|| black_box(expr.clone().simplify()))
    });

    // Nested exponentials: exp(exp(x))
    group.bench_function("nested_exp", |b| {
        let expr = Expression::function(
            "exp",
            vec![Expression::function(
                "exp",
                vec![Expression::symbol(x.clone())],
            )],
        );
        b.iter(|| black_box(expr.clone().simplify()))
    });

    group.finish();
}

/// Benchmark power and root functions
fn bench_power_root_functions(c: &mut Criterion) {
    let mut group = c.benchmark_group("power_root_functions");

    let x = symbol!(x);

    // Square root
    group.bench_function("sqrt_symbolic", |b| {
        let expr = Expression::function("sqrt", vec![Expression::symbol(x.clone())]);
        b.iter(|| black_box(expr.clone().simplify()))
    });

    // Nth root for various n
    for n in [2, 3, 4, 5].iter() {
        group.bench_with_input(BenchmarkId::new("nth_root", n), n, |b, &root| {
            let expr = Expression::pow(
                Expression::symbol(x.clone()),
                Expression::mul(vec![
                    Expression::integer(1),
                    Expression::pow(Expression::integer(root), Expression::integer(-1)),
                ]),
            );
            b.iter(|| black_box(expr.clone().simplify()))
        });
    }

    // sqrt(x^2) simplification
    group.bench_function("sqrt_square_simplification", |b| {
        let expr = Expression::function(
            "sqrt",
            vec![Expression::pow(
                Expression::symbol(x.clone()),
                Expression::integer(2),
            )],
        );
        b.iter(|| black_box(expr.clone().simplify()))
    });

    // Rational exponents: x^(1/2) vs sqrt(x)
    group.bench_function("rational_exponent", |b| {
        let expr = Expression::pow(
            Expression::symbol(x.clone()),
            Expression::Number(mathhook_core::Number::rational(
                num_rational::BigRational::new(
                    num_bigint::BigInt::from(1),
                    num_bigint::BigInt::from(2),
                ),
            )),
        );
        b.iter(|| black_box(expr.clone().simplify()))
    });

    group.finish();
}

/// Benchmark absolute value and sign functions
fn bench_absolute_sign_functions(c: &mut Criterion) {
    let mut group = c.benchmark_group("absolute_sign_functions");

    let x = symbol!(x);

    // Absolute value
    group.bench_function("abs_symbolic", |b| {
        let expr = Expression::function("abs", vec![Expression::symbol(x.clone())]);
        b.iter(|| black_box(expr.clone().simplify()))
    });

    // abs(abs(x)) = abs(x)
    group.bench_function("nested_abs", |b| {
        let expr = Expression::function(
            "abs",
            vec![Expression::function(
                "abs",
                vec![Expression::symbol(x.clone())],
            )],
        );
        b.iter(|| black_box(expr.clone().simplify()))
    });

    // abs(x * y) properties
    let y = symbol!(y);
    group.bench_function("abs_product", |b| {
        let expr = Expression::function(
            "abs",
            vec![Expression::mul(vec![
                Expression::symbol(x.clone()),
                Expression::symbol(y.clone()),
            ])],
        );
        b.iter(|| black_box(expr.clone().simplify()))
    });

    group.finish();
}

/// Benchmark factorial and combinatorial functions
fn bench_factorial_combinatorial(c: &mut Criterion) {
    let mut group = c.benchmark_group("factorial_combinatorial");

    // Factorial evaluation for small numbers
    for n in [1, 5, 10, 15, 20].iter() {
        group.bench_with_input(BenchmarkId::new("factorial", n), n, |b, &num| {
            let expr = Expression::function("factorial", vec![Expression::integer(num)]);
            b.iter(|| black_box(expr.clone().simplify()))
        });
    }

    // Symbolic factorial
    let n = symbol!(n);
    group.bench_function("factorial_symbolic", |b| {
        let expr = Expression::function("factorial", vec![Expression::symbol(n.clone())]);
        b.iter(|| black_box(expr.clone().simplify()))
    });

    // Binomial coefficient: C(n, k)
    group.bench_function("binomial_coefficient", |b| {
        let expr = Expression::function(
            "binomial",
            vec![Expression::integer(10), Expression::integer(5)],
        );
        b.iter(|| black_box(expr.clone().simplify()))
    });

    group.finish();
}

/// Benchmark special functions (gamma, bessel, etc.)
fn bench_special_functions(c: &mut Criterion) {
    let mut group = c.benchmark_group("special_functions");

    let x = symbol!(x);

    // Gamma function
    group.bench_function("gamma_symbolic", |b| {
        let expr = Expression::function("gamma", vec![Expression::symbol(x.clone())]);
        b.iter(|| black_box(expr.clone().simplify()))
    });

    // Gamma at integer values: gamma(n) = (n-1)!
    for n in [1, 2, 3, 4, 5].iter() {
        group.bench_with_input(BenchmarkId::new("gamma_integer", n), n, |b, &num| {
            let expr = Expression::function("gamma", vec![Expression::integer(num)]);
            b.iter(|| black_box(expr.clone().simplify()))
        });
    }

    // Bessel functions (if implemented)
    group.bench_function("bessel_j_symbolic", |b| {
        let expr = Expression::function(
            "bessel_j",
            vec![Expression::integer(0), Expression::symbol(x.clone())],
        );
        b.iter(|| black_box(expr.clone().simplify()))
    });

    // Zeta function (if implemented)
    group.bench_function("zeta_symbolic", |b| {
        let expr = Expression::function("zeta", vec![Expression::symbol(x.clone())]);
        b.iter(|| black_box(expr.clone().simplify()))
    });

    group.finish();
}

/// Benchmark mixed function composition
fn bench_function_composition(c: &mut Criterion) {
    let mut group = c.benchmark_group("function_composition");

    let x = symbol!(x);

    // sin(exp(x))
    group.bench_function("sin_exp", |b| {
        let expr = Expression::function(
            "sin",
            vec![Expression::function(
                "exp",
                vec![Expression::symbol(x.clone())],
            )],
        );
        b.iter(|| black_box(expr.clone().simplify()))
    });

    // log(sin(x) + cos(x))
    group.bench_function("log_trig_sum", |b| {
        let expr = Expression::function(
            "log",
            vec![Expression::add(vec![
                Expression::function("sin", vec![Expression::symbol(x.clone())]),
                Expression::function("cos", vec![Expression::symbol(x.clone())]),
            ])],
        );
        b.iter(|| black_box(expr.clone().simplify()))
    });

    // exp(log(sqrt(x)))
    group.bench_function("exp_log_sqrt", |b| {
        let expr = Expression::function(
            "exp",
            vec![Expression::function(
                "log",
                vec![Expression::function(
                    "sqrt",
                    vec![Expression::symbol(x.clone())],
                )],
            )],
        );
        b.iter(|| black_box(expr.clone().simplify()))
    });

    // Deeply nested: f(g(h(i(x))))
    group.bench_function("deeply_nested", |b| {
        let expr = Expression::function(
            "sin",
            vec![Expression::function(
                "cos",
                vec![Expression::function(
                    "exp",
                    vec![Expression::function(
                        "log",
                        vec![Expression::symbol(x.clone())],
                    )],
                )],
            )],
        );
        b.iter(|| black_box(expr.clone().simplify()))
    });

    group.finish();
}

criterion_group!(
    name = function_evaluation_benchmarks;
    config = Criterion::default()
        .measurement_time(Duration::from_secs(10))
        .sample_size(100);
    targets =
        bench_elementary_trigonometric,
        bench_hyperbolic_functions,
        bench_exponential_logarithmic,
        bench_power_root_functions,
        bench_absolute_sign_functions,
        bench_factorial_combinatorial,
        bench_special_functions,
        bench_function_composition
);

criterion_main!(function_evaluation_benchmarks);
