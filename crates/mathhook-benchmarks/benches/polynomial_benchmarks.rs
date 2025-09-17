//! Comprehensive Polynomial Module Performance Benchmarks
//!
//! Baselines performance for all polynomial operations including:
//! - GCD algorithms (univariate, multivariate, Zippel modular)
//! - Division and factorization
//! - Groebner basis computation
//! - Special polynomial families (Legendre, Chebyshev, Hermite, Laguerre)
//! - Finite field arithmetic
//!
//! Last Updated: 2025-12-07T1130
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use mathhook_core::algebra::multivariate_gcd::multivariate_gcd;
use mathhook_core::algebra::polynomial_advanced::AdvancedPolynomial;
use mathhook_core::algebra::polynomial_division::polynomial_div;
use mathhook_core::core::polynomial::algorithms::polynomial_gcd;
use mathhook_core::core::polynomial::algorithms::zippel_gcd::{
    multivariate_gcd_zippel, MultivariateConfig,
};
use mathhook_core::core::polynomial::finite_field::PolyZp;
use mathhook_core::core::polynomial::groebner::{GroebnerBasis, MonomialOrder};
use mathhook_core::core::polynomial::special_families::{
    evaluate_chebyshev_first_numerical, evaluate_hermite_numerical, evaluate_laguerre_numerical,
    evaluate_legendre_numerical, expand_chebyshev_first_symbolic, expand_hermite_symbolic,
    expand_laguerre_symbolic, expand_legendre_symbolic,
};
use mathhook_core::{expr, parse, symbol, Expression};
use std::collections::HashMap;
use std::hint::black_box as bb;

fn bench_gcd_algorithms(c: &mut Criterion) {
    let mut group = c.benchmark_group("gcd_algorithms");
    group.sample_size(50);
    let x = symbol!(x);
    group.bench_function("univariate_simple", |b| {
        let f = expr!((x ^ 2) - 1);
        let g = expr!(x - 1);
        b.iter(|| polynomial_gcd(bb(&f), bb(&g)))
    });
    group.bench_function("univariate_simple_with_parsing", |b| {
        b.iter(|| {
            let f = parse!("x^2 - 1").unwrap();
            let g = parse!("x - 1").unwrap();
            polynomial_gcd(bb(&f), bb(&g))
        })
    });
    group.bench_function("univariate_degree_10", |b| {
        let mut f_terms = vec![Expression::integer(-1)];
        for i in 1..=10 {
            f_terms.push(Expression::mul(vec![
                Expression::integer(i),
                Expression::pow(Expression::symbol(x.clone()), Expression::integer(i)),
            ]));
        }
        let f = Expression::add(f_terms);
        let g = expr!((x ^ 5) - 1);
        b.iter(|| polynomial_gcd(bb(&f), bb(&g)))
    });
    group.bench_function("univariate_large_coeffs", |b| {
        let f = Expression::add(vec![
            Expression::mul(vec![
                Expression::integer(1234567890),
                Expression::pow(Expression::symbol(x.clone()), Expression::integer(3)),
            ]),
            Expression::mul(vec![
                Expression::integer(987654321),
                Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
            ]),
            Expression::integer(1111111111),
        ]);
        let g = Expression::add(vec![
            Expression::mul(vec![
                Expression::integer(12345),
                Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
            ]),
            Expression::integer(67890),
        ]);
        b.iter(|| polynomial_gcd(bb(&f), bb(&g)))
    });
    group.finish();
}

fn bench_multivariate_gcd(c: &mut Criterion) {
    let mut group = c.benchmark_group("multivariate_gcd");
    group.sample_size(30);
    let x = symbol!(x);
    let y = symbol!(y);
    let z = symbol!(z);
    group.bench_function("bivariate_simple", |b| {
        let f = Expression::mul(vec![expr!(x), expr!(y)]);
        let g = Expression::mul(vec![expr!(x), expr!(y + 1)]);
        let vars = vec![x.clone(), y.clone()];
        b.iter(|| multivariate_gcd(bb(&f), bb(&g), bb(&vars)))
    });
    group.bench_function("bivariate_diff_squares", |b| {
        let f = expr!((x ^ 2) - (y ^ 2));
        let g = expr!(x - y);
        let vars = vec![x.clone(), y.clone()];
        b.iter(|| multivariate_gcd(bb(&f), bb(&g), bb(&vars)))
    });
    group.bench_function("trivariate", |b| {
        let f = Expression::mul(vec![expr!(x), expr!(y), expr!(z)]);
        let g = Expression::mul(vec![expr!(x), expr!(y)]);
        let vars = vec![x.clone(), y.clone(), z.clone()];
        b.iter(|| multivariate_gcd(bb(&f), bb(&g), bb(&vars)))
    });
    group.bench_function("zippel_bivariate", |b| {
        // f = xy (pure numeric: HashMap<Vec<usize>, i64>)
        let mut f: HashMap<Vec<usize>, i64> = HashMap::new();
        f.insert(vec![1, 1], 1); // xy
                                 // g = xy + x
        let mut g: HashMap<Vec<usize>, i64> = HashMap::new();
        g.insert(vec![1, 1], 1); // xy
        g.insert(vec![1, 0], 1); // x
        let config = MultivariateConfig::default();
        b.iter(|| multivariate_gcd_zippel(bb(&f), bb(&g), 2, &config))
    });
    group.finish();
}

fn bench_polynomial_division(c: &mut Criterion) {
    let mut group = c.benchmark_group("polynomial_division");
    group.sample_size(50);
    let x = symbol!(x);
    group.bench_function("division_simple", |b| {
        let dividend = expr!((x ^ 2) - 1);
        let divisor = expr!(x - 1);
        b.iter(|| polynomial_div(bb(&dividend), bb(&divisor), bb(&x)))
    });
    group.bench_function("division_degree_8", |b| {
        let dividend = Expression::add(vec![
            Expression::pow(Expression::symbol(x.clone()), Expression::integer(8)),
            Expression::integer(-1),
        ]);
        let divisor = expr!((x ^ 2) - 1);
        b.iter(|| polynomial_div(bb(&dividend), bb(&divisor), bb(&x)))
    });
    group.finish();
}

fn bench_numeric_content(c: &mut Criterion) {
    use mathhook_core::core::polynomial::algorithms::zippel_gcd::{
        analyze_sparsity, is_sparse, primitive_part,
    };

    let mut group = c.benchmark_group("numeric_content");
    group.sample_size(100);

    group.bench_function("content_small", |b| {
        let coeffs = vec![6, 12, 18];
        b.iter(|| primitive_part(bb(&coeffs)))
    });
    group.bench_function("content_large", |b| {
        let coeffs: Vec<i64> = (0..100).map(|i| (i * 6) % 1000 + 6).collect();
        b.iter(|| primitive_part(bb(&coeffs)))
    });
    group.bench_function("sparsity_analysis_sparse", |b| {
        let mut coeffs = vec![0i64; 100];
        coeffs[0] = 1;
        coeffs[50] = 5;
        coeffs[99] = 3;
        b.iter(|| analyze_sparsity(bb(&coeffs)))
    });
    group.bench_function("sparsity_analysis_dense", |b| {
        let coeffs: Vec<i64> = (1..=100).collect();
        b.iter(|| analyze_sparsity(bb(&coeffs)))
    });
    group.bench_function("is_sparse_check", |b| {
        let mut coeffs = vec![0i64; 100];
        coeffs[0] = 1;
        coeffs[99] = 1;
        b.iter(|| is_sparse(bb(&coeffs)))
    });
    group.finish();
}

fn bench_resultant(c: &mut Criterion) {
    let mut group = c.benchmark_group("resultant");
    group.sample_size(30);
    let x = symbol!(x);
    group.bench_function("resultant_quadratic", |b| {
        let f = expr!((x ^ 2) + x + 1);
        let g = expr!((x ^ 2) - 1);
        b.iter(|| f.polynomial_resultant(bb(&g), bb(&x)))
    });
    group.bench_function("resultant_degree_4", |b| {
        let f = Expression::add(vec![
            Expression::pow(Expression::symbol(x.clone()), Expression::integer(4)),
            Expression::integer(-1),
        ]);
        let g = Expression::add(vec![
            Expression::pow(Expression::symbol(x.clone()), Expression::integer(3)),
            Expression::integer(-1),
        ]);
        b.iter(|| f.polynomial_resultant(bb(&g), bb(&x)))
    });
    group.finish();
}

fn bench_groebner(c: &mut Criterion) {
    let mut group = c.benchmark_group("groebner_basis");
    group.sample_size(20);
    let x = symbol!(x);
    let y = symbol!(y);
    group.bench_function("simple_2var", |b| {
        let f1 = Expression::add(vec![
            Expression::symbol(x.clone()),
            Expression::mul(vec![Expression::integer(-1), Expression::symbol(y.clone())]),
        ]);
        let f2 = Expression::add(vec![
            Expression::pow(Expression::symbol(y.clone()), Expression::integer(2)),
            Expression::integer(-1),
        ]);
        b.iter(|| {
            let mut gb = GroebnerBasis::new(
                vec![f1.clone(), f2.clone()],
                vec![x.clone(), y.clone()],
                MonomialOrder::Lex,
            );
            gb.compute();
            bb(gb.basis.len())
        })
    });
    for order in [
        MonomialOrder::Lex,
        MonomialOrder::Grlex,
        MonomialOrder::Grevlex,
    ] {
        let order_name = match order {
            MonomialOrder::Lex => "lex",
            MonomialOrder::Grlex => "grlex",
            MonomialOrder::Grevlex => "grevlex",
        };
        group.bench_function(BenchmarkId::new("monomial_order", order_name), |b| {
            let f1 = Expression::add(vec![
                Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
                Expression::pow(Expression::symbol(y.clone()), Expression::integer(2)),
                Expression::integer(-1),
            ]);
            let f2 = Expression::add(vec![
                Expression::symbol(x.clone()),
                Expression::symbol(y.clone()),
            ]);
            b.iter(|| {
                let mut gb = GroebnerBasis::new(
                    vec![f1.clone(), f2.clone()],
                    vec![x.clone(), y.clone()],
                    order,
                );
                gb.compute();
                bb(gb.basis.len())
            })
        });
    }
    group.finish();
}

fn bench_special_polynomials_symbolic(c: &mut Criterion) {
    let mut group = c.benchmark_group("special_polynomials_symbolic");
    group.sample_size(50);
    for n in [5, 10, 15, 20] {
        group.bench_function(BenchmarkId::new("legendre", n), |b| {
            b.iter(|| expand_legendre_symbolic(bb(n)))
        });
    }
    for n in [5, 10, 15, 20] {
        group.bench_function(BenchmarkId::new("chebyshev_T", n), |b| {
            b.iter(|| expand_chebyshev_first_symbolic(bb(n)))
        });
    }
    for n in [5, 10, 15] {
        group.bench_function(BenchmarkId::new("hermite", n), |b| {
            b.iter(|| expand_hermite_symbolic(bb(n)))
        });
    }
    for n in [5, 10, 15] {
        group.bench_function(BenchmarkId::new("laguerre", n), |b| {
            b.iter(|| expand_laguerre_symbolic(bb(n)))
        });
    }
    group.finish();
}

fn bench_special_polynomials_numerical(c: &mut Criterion) {
    let mut group = c.benchmark_group("special_polynomials_numerical");
    group.sample_size(100);
    let test_points: Vec<f64> = vec![0.0, 0.5, 0.7, 0.9];
    for n in [10, 20, 50, 100] {
        group.bench_function(BenchmarkId::new("legendre_eval", n), |b| {
            b.iter(|| {
                for &x in &test_points {
                    bb(evaluate_legendre_numerical(&[n as f64, x]));
                }
            })
        });
    }
    for n in [10, 20, 50, 100] {
        group.bench_function(BenchmarkId::new("chebyshev_T_eval", n), |b| {
            b.iter(|| {
                for &x in &test_points {
                    bb(evaluate_chebyshev_first_numerical(&[n as f64, x]));
                }
            })
        });
    }
    for n in [10, 20, 50] {
        group.bench_function(BenchmarkId::new("hermite_eval", n), |b| {
            b.iter(|| {
                for &x in &test_points {
                    bb(evaluate_hermite_numerical(&[n as f64, x]));
                }
            })
        });
    }
    for n in [10, 20, 50] {
        group.bench_function(BenchmarkId::new("laguerre_eval", n), |b| {
            b.iter(|| {
                for &x in &test_points {
                    bb(evaluate_laguerre_numerical(&[n as f64, x]));
                }
            })
        });
    }
    group.finish();
}

fn bench_finite_field(c: &mut Criterion) {
    let mut group = c.benchmark_group("finite_field");
    group.sample_size(100);
    let prime = 2147483647u64;
    group.bench_function("poly_creation_degree_10", |b| {
        let coeffs: Vec<u64> = (0..11).map(|i| (i * 17) % prime).collect();
        b.iter(|| PolyZp::from_coeffs(bb(coeffs.clone()), bb(prime)))
    });
    group.bench_function("poly_mul_degree_5", |b| {
        let f_coeffs: Vec<u64> = (0..6).map(|i| (i * 17) % prime).collect();
        let g_coeffs: Vec<u64> = (0..6).map(|i| (i * 23) % prime).collect();
        let f = PolyZp::from_coeffs(f_coeffs, prime);
        let g = PolyZp::from_coeffs(g_coeffs, prime);
        b.iter(|| f.mul(bb(&g)))
    });
    group.bench_function("poly_gcd_degree_10", |b| {
        let f_coeffs: Vec<u64> = (0..11).map(|i| (i * 17 + 1) % prime).collect();
        let g_coeffs: Vec<u64> = (0..8).map(|i| (i * 23 + 1) % prime).collect();
        let f = PolyZp::from_coeffs(f_coeffs, prime);
        let g = PolyZp::from_coeffs(g_coeffs, prime);
        b.iter(|| f.gcd(bb(&g)))
    });
    group.bench_function("poly_div_degree_10", |b| {
        let f_coeffs: Vec<u64> = (0..11).map(|i| (i * 17 + 1) % prime).collect();
        let g_coeffs: Vec<u64> = (0..4).map(|i| (i * 23 + 1) % prime).collect();
        let f = PolyZp::from_coeffs(f_coeffs, prime);
        let g = PolyZp::from_coeffs(g_coeffs, prime);
        b.iter(|| f.div_rem(bb(&g)))
    });
    group.finish();
}

fn bench_trial_division(c: &mut Criterion) {
    use mathhook_core::core::polynomial::algorithms::zippel_gcd::{
        trial_divide, verify_gcd_candidate,
    };

    let mut group = c.benchmark_group("trial_division");
    group.sample_size(100);
    group.bench_function("trial_divide_exact", |b| {
        let dividend = vec![-1, 0, 1];
        let divisor = vec![-1, 1];
        b.iter(|| trial_divide(bb(&dividend), bb(&divisor)))
    });
    group.bench_function("trial_divide_non_exact", |b| {
        let dividend = vec![1, 2, 3, 4, 5];
        let divisor = vec![1, 1];
        b.iter(|| trial_divide(bb(&dividend), bb(&divisor)))
    });
    group.bench_function("verify_gcd_candidate", |b| {
        let f = vec![-1, 0, 1];
        let g = vec![-1, 1];
        let h = vec![-1, 1];
        b.iter(|| verify_gcd_candidate(bb(&f), bb(&g), bb(&h)))
    });
    group.finish();
}

criterion_group!(gcd_benches, bench_gcd_algorithms, bench_multivariate_gcd,);
criterion_group!(
    division_benches,
    bench_polynomial_division,
    bench_numeric_content,
    bench_resultant,
);
criterion_group!(groebner_benches, bench_groebner,);
criterion_group!(
    special_poly_benches,
    bench_special_polynomials_symbolic,
    bench_special_polynomials_numerical,
);
criterion_group!(
    finite_field_benches,
    bench_finite_field,
    bench_trial_division,
);
criterion_main!(
    gcd_benches,
    division_benches,
    groebner_benches,
    special_poly_benches,
    finite_field_benches
);
