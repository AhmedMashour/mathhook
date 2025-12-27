/// Comprehensive equation solving benchmarks
///
/// Tests linear, quadratic, polynomial, system, and matrix equation solving
/// with varying complexity levels. Critical for solver performance measurement.
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use mathhook_core::algebra::solvers::linear::LinearSolver;
use mathhook_core::algebra::solvers::matrix_equations::MatrixEquationSolver;
use mathhook_core::algebra::solvers::polynomial::PolynomialSolver;
use mathhook_core::algebra::solvers::quadratic::QuadraticSolver;
use mathhook_core::algebra::solvers::systems::SystemSolver;
use mathhook_core::algebra::solvers::{EquationSolver, SystemEquationSolver};
use mathhook_core::calculus::ode::{LinearFirstOrderSolver, SeparableODESolver};
use mathhook_core::calculus::pde::EducationalPDESolver;
use mathhook_core::{parse, symbol, Expression, MathSolver, Symbol};
use std::hint::black_box;
use std::time::Duration;

/// Benchmark linear equation solving: ax + b = 0
fn bench_linear_solving(c: &mut Criterion) {
    let mut group = c.benchmark_group("linear_solving");

    let x = symbol!(x);
    let solver = LinearSolver::new();

    // Simple linear: 2x + 3 = 0
    let simple_eq = Expression::add(vec![
        Expression::mul(vec![Expression::integer(2), Expression::symbol(x.clone())]),
        Expression::integer(3),
    ]);

    group.bench_function("simple_linear", |b| {
        b.iter(|| black_box(solver.solve(&simple_eq, &x)))
    });

    group.bench_function("simple_linear_with_parsing", |b| {
        b.iter(|| {
            let eq = parse!("2*x + 3").unwrap();
            let x = Symbol::new("x");
            black_box(solver.solve(&eq, &x))
        })
    });

    // Linear with larger coefficients
    for coeff in [10, 100, 1000, 10000].iter() {
        let eq = Expression::add(vec![
            Expression::mul(vec![
                Expression::integer(*coeff),
                Expression::symbol(x.clone()),
            ]),
            Expression::integer(*coeff / 2),
        ]);

        group.bench_with_input(
            BenchmarkId::new("large_coefficient", coeff),
            coeff,
            |b, _| b.iter(|| black_box(solver.solve(&eq, &x))),
        );
    }

    // Rational linear equations
    let rational_eq = Expression::add(vec![
        Expression::mul(vec![
            Expression::Number(mathhook_core::Number::rational(
                num_rational::BigRational::new(
                    num_bigint::BigInt::from(3),
                    num_bigint::BigInt::from(4),
                ),
            )),
            Expression::symbol(x.clone()),
        ]),
        Expression::Number(mathhook_core::Number::rational(
            num_rational::BigRational::new(
                num_bigint::BigInt::from(5),
                num_bigint::BigInt::from(6),
            ),
        )),
    ]);

    group.bench_function("rational_linear", |b| {
        b.iter(|| black_box(solver.solve(&rational_eq, &x)))
    });

    group.finish();
}

/// Benchmark quadratic equation solving: ax^2 + bx + c = 0
fn bench_quadratic_solving(c: &mut Criterion) {
    let mut group = c.benchmark_group("quadratic_solving");

    let x = symbol!(x);
    let solver = QuadraticSolver;

    // Simple quadratic: x^2 - 4 = 0 (real roots)
    let simple_quad = Expression::add(vec![
        Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
        Expression::integer(-4),
    ]);

    group.bench_function("simple_quadratic_real_roots", |b| {
        b.iter(|| black_box(solver.solve(&simple_quad, &x)))
    });

    // Quadratic with complex roots: x^2 + 1 = 0
    let complex_quad = Expression::add(vec![
        Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
        Expression::integer(1),
    ]);

    group.bench_function("quadratic_complex_roots", |b| {
        b.iter(|| black_box(solver.solve(&complex_quad, &x)))
    });

    // General quadratic: ax^2 + bx + c = 0
    for (a, b, c) in [(1, 0, -4), (2, 3, -5), (3, -7, 2), (5, 12, -8)].iter() {
        let eq = Expression::add(vec![
            Expression::mul(vec![
                Expression::integer(*a),
                Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
            ]),
            Expression::mul(vec![Expression::integer(*b), Expression::symbol(x.clone())]),
            Expression::integer(*c),
        ]);

        group.bench_with_input(
            BenchmarkId::new("general_quadratic", format!("{}x²+{}x+{}", a, b, c)),
            &(*a, *b, *c),
            |b, _| b.iter(|| black_box(solver.solve(&eq, &x))),
        );
    }

    // Perfect square quadratic: (x - 2)^2 = 0 (repeated root)
    let perfect_square = Expression::pow(
        Expression::add(vec![Expression::symbol(x.clone()), Expression::integer(-2)]),
        Expression::integer(2),
    );

    group.bench_function("perfect_square", |b| {
        b.iter(|| black_box(solver.solve(&perfect_square, &x)))
    });

    group.finish();
}

/// Benchmark polynomial equation solving
fn bench_polynomial_solving(c: &mut Criterion) {
    let mut group = c.benchmark_group("polynomial_solving");

    let x = symbol!(x);
    let solver = PolynomialSolver;

    // Cubic equations: x^3 - 6x^2 + 11x - 6 = 0
    let cubic = Expression::add(vec![
        Expression::pow(Expression::symbol(x.clone()), Expression::integer(3)),
        Expression::mul(vec![
            Expression::integer(-6),
            Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
        ]),
        Expression::mul(vec![Expression::integer(11), Expression::symbol(x.clone())]),
        Expression::integer(-6),
    ]);

    group.bench_function("cubic_equation", |b| {
        b.iter(|| black_box(solver.solve(&cubic, &x)))
    });

    // Quartic equation: x^4 - 5x^2 + 4 = 0
    let quartic = Expression::add(vec![
        Expression::pow(Expression::symbol(x.clone()), Expression::integer(4)),
        Expression::mul(vec![
            Expression::integer(-5),
            Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
        ]),
        Expression::integer(4),
    ]);

    group.bench_function("quartic_equation", |b| {
        b.iter(|| black_box(solver.solve(&quartic, &x)))
    });

    // Higher degree polynomials
    for degree in [5, 7, 10].iter() {
        // Construct polynomial: x^n - 1 = 0
        let poly = Expression::add(vec![
            Expression::pow(Expression::symbol(x.clone()), Expression::integer(*degree)),
            Expression::integer(-1),
        ]);

        group.bench_with_input(BenchmarkId::new("higher_degree", degree), degree, |b, _| {
            b.iter(|| black_box(solver.solve(&poly, &x)))
        });
    }

    group.finish();
}

/// Benchmark system of equations solving
fn bench_system_solving(c: &mut Criterion) {
    let mut group = c.benchmark_group("system_solving");

    let x = symbol!(x);
    let y = symbol!(y);
    let solver = SystemSolver::new();

    // 2x2 linear system:
    // x + y = 3
    // 2x - y = 0
    let eq1 = Expression::add(vec![
        Expression::symbol(x.clone()),
        Expression::symbol(y.clone()),
        Expression::integer(-3),
    ]);
    let eq2 = Expression::add(vec![
        Expression::mul(vec![Expression::integer(2), Expression::symbol(x.clone())]),
        Expression::mul(vec![Expression::integer(-1), Expression::symbol(y.clone())]),
    ]);

    group.bench_function("2x2_linear_system", |b| {
        b.iter(|| {
            black_box(solver.solve_system(&[eq1.clone(), eq2.clone()], &[x.clone(), y.clone()]))
        })
    });

    // 3x3 linear system
    let z = symbol!(z);
    let eq3_1 = Expression::add(vec![
        Expression::symbol(x.clone()),
        Expression::symbol(y.clone()),
        Expression::symbol(z.clone()),
        Expression::integer(-6),
    ]);
    let eq3_2 = Expression::add(vec![
        Expression::mul(vec![Expression::integer(2), Expression::symbol(x.clone())]),
        Expression::symbol(y.clone()),
        Expression::integer(-3),
    ]);
    let eq3_3 = Expression::add(vec![
        Expression::symbol(x.clone()),
        Expression::mul(vec![Expression::integer(-1), Expression::symbol(y.clone())]),
        Expression::mul(vec![Expression::integer(2), Expression::symbol(z.clone())]),
        Expression::integer(-1),
    ]);

    group.bench_function("3x3_linear_system", |b| {
        b.iter(|| {
            black_box(solver.solve_system(
                &[eq3_1.clone(), eq3_2.clone(), eq3_3.clone()],
                &[x.clone(), y.clone(), z.clone()],
            ))
        })
    });

    // Nonlinear system: x^2 + y^2 = 5, x - y = 1
    let nonlinear1 = Expression::add(vec![
        Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
        Expression::pow(Expression::symbol(y.clone()), Expression::integer(2)),
        Expression::integer(-5),
    ]);
    let nonlinear2 = Expression::add(vec![
        Expression::symbol(x.clone()),
        Expression::mul(vec![Expression::integer(-1), Expression::symbol(y.clone())]),
        Expression::integer(-1),
    ]);

    group.bench_function("nonlinear_2x2_system", |b| {
        b.iter(|| {
            black_box(solver.solve_system(
                &[nonlinear1.clone(), nonlinear2.clone()],
                &[x.clone(), y.clone()],
            ))
        })
    });

    group.finish();
}

/// Benchmark matrix equation solving: AX = B
fn bench_matrix_equation_solving(c: &mut Criterion) {
    let mut group = c.benchmark_group("matrix_equation_solving");

    let matrix_x = symbol!(X; matrix);
    let solver = MatrixEquationSolver::new();

    // 2x2 matrix equation: [[1, 2], [3, 4]] * X = [[5], [11]]
    let matrix_a_2x2 = Expression::matrix(vec![
        vec![Expression::integer(1), Expression::integer(2)],
        vec![Expression::integer(3), Expression::integer(4)],
    ]);
    let matrix_b_2x2 = Expression::matrix(vec![
        vec![Expression::integer(5)],
        vec![Expression::integer(11)],
    ]);
    let eq_2x2 = Expression::add(vec![
        Expression::mul(vec![matrix_a_2x2, Expression::symbol(matrix_x.clone())]),
        Expression::mul(vec![Expression::integer(-1), matrix_b_2x2]),
    ]);

    group.bench_function("2x2_matrix_equation", |b| {
        b.iter(|| black_box(solver.solve(&eq_2x2, &matrix_x)))
    });

    // 3x3 matrix equation
    let matrix_a_3x3 = Expression::matrix(vec![
        vec![
            Expression::integer(2),
            Expression::integer(-1),
            Expression::integer(0),
        ],
        vec![
            Expression::integer(-1),
            Expression::integer(2),
            Expression::integer(-1),
        ],
        vec![
            Expression::integer(0),
            Expression::integer(-1),
            Expression::integer(2),
        ],
    ]);
    let matrix_b_3x3 = Expression::matrix(vec![
        vec![Expression::integer(1)],
        vec![Expression::integer(2)],
        vec![Expression::integer(3)],
    ]);
    let eq_3x3 = Expression::add(vec![
        Expression::mul(vec![matrix_a_3x3, Expression::symbol(matrix_x.clone())]),
        Expression::mul(vec![Expression::integer(-1), matrix_b_3x3]),
    ]);

    group.bench_function("3x3_matrix_equation", |b| {
        b.iter(|| black_box(solver.solve(&eq_3x3, &matrix_x)))
    });

    group.finish();
}

/// Benchmark unified MathSolver interface
fn bench_math_solver_interface(c: &mut Criterion) {
    let mut group = c.benchmark_group("math_solver_interface");

    let x = symbol!(x);
    let solver = MathSolver::new();

    // Linear equation through MathSolver
    let linear = Expression::add(vec![
        Expression::mul(vec![Expression::integer(3), Expression::symbol(x.clone())]),
        Expression::integer(-6),
    ]);

    group.bench_function("mathsolver_linear", |b| {
        b.iter(|| black_box(solver.solve(&linear, &x)))
    });

    // Quadratic through MathSolver
    let quadratic = Expression::add(vec![
        Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
        Expression::mul(vec![Expression::integer(-5), Expression::symbol(x.clone())]),
        Expression::integer(6),
    ]);

    group.bench_function("mathsolver_quadratic", |b| {
        b.iter(|| black_box(solver.solve(&quadratic, &x)))
    });

    // Polynomial through MathSolver
    let polynomial = Expression::add(vec![
        Expression::pow(Expression::symbol(x.clone()), Expression::integer(3)),
        Expression::mul(vec![
            Expression::integer(-3),
            Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
        ]),
        Expression::mul(vec![Expression::integer(3), Expression::symbol(x.clone())]),
        Expression::integer(-1),
    ]);

    group.bench_function("mathsolver_polynomial", |b| {
        b.iter(|| black_box(solver.solve(&polynomial, &x)))
    });

    group.finish();
}

/// Benchmark ODE (Ordinary Differential Equation) solving
fn bench_ode_solving(c: &mut Criterion) {
    let mut group = c.benchmark_group("ode_solving");

    let x = symbol!(x);
    let y = symbol!(y);
    let separable_solver = SeparableODESolver::new();
    let linear_solver = LinearFirstOrderSolver;

    // Separable ODE: dy/dx = xy
    let separable_rhs = Expression::mul(vec![
        Expression::symbol(x.clone()),
        Expression::symbol(y.clone()),
    ]);

    group.bench_function("separable_ode_xy", |b| {
        b.iter(|| black_box(separable_solver.solve(&separable_rhs, &y, &x, None)))
    });

    // Simple separable: dy/dx = x
    let simple_separable = Expression::symbol(x.clone());

    group.bench_function("separable_ode_simple", |b| {
        b.iter(|| black_box(separable_solver.solve(&simple_separable, &y, &x, None)))
    });

    // Linear first-order: dy/dx + y = x (in standard form dy/dx = -y + x)
    // For LinearFirstOrderSolver, we need p(x) and q(x) where dy/dx + p(x)y = q(x)
    let p_x = Expression::integer(1); // coefficient of y
    let q_x = Expression::symbol(x.clone()); // right-hand side

    group.bench_function("linear_first_order_ode", |b| {
        b.iter(|| {
            black_box(LinearFirstOrderSolver::solve(
                &linear_solver,
                &p_x,
                &q_x,
                &y,
                &x,
                None,
            ))
        })
    });

    // Varying complexity for separable ODEs
    for degree in [2, 3, 4].iter() {
        let rhs = Expression::mul(vec![
            Expression::pow(Expression::symbol(x.clone()), Expression::integer(*degree)),
            Expression::symbol(y.clone()),
        ]);

        group.bench_with_input(
            BenchmarkId::new("separable_degree", degree),
            degree,
            |b, _| b.iter(|| black_box(separable_solver.solve(&rhs, &y, &x, None))),
        );
    }

    group.finish();
}

/// Benchmark PDE (Partial Differential Equation) solving
fn bench_pde_solving(c: &mut Criterion) {
    let mut group = c.benchmark_group("pde_solving");

    let u = symbol!(u);
    let x = symbol!(x);
    let solver = EducationalPDESolver::new();

    // Simple placeholder PDE (PDE solving is not fully implemented yet)
    let pde = Expression::add(vec![
        Expression::symbol(u.clone()),
        Expression::symbol(x.clone()),
    ]);

    group.bench_function("pde_placeholder", |b| {
        b.iter(|| black_box(solver.solve(&pde, &u)))
    });

    group.finish();
}

criterion_group!(
    name = solving_benchmarks;
    config = Criterion::default()
        .measurement_time(Duration::from_secs(10))
        .sample_size(100);
    targets =
        bench_linear_solving,
        bench_quadratic_solving,
        bench_polynomial_solving,
        bench_system_solving,
        bench_matrix_equation_solving,
        bench_math_solver_interface,
        bench_ode_solving,
        bench_pde_solving
);

criterion_main!(solving_benchmarks);
