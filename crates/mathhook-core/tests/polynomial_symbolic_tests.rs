//! Comprehensive Tests for Symbolic Polynomial Expansion
//!
//! Tests symbolic polynomial expansion for all 5 families (Legendre, Hermite,
//! Laguerre, Chebyshev T & U) with validation against numerical evaluation
//! and verification of mathematical correctness.

use mathhook_core::core::{Expression, Symbol};
use mathhook_core::functions::polynomials::evaluation::*;
use mathhook_core::functions::polynomials::symbolic::*;

/// Helper function to evaluate symbolic expression at a given point
fn evaluate_at(expr: &Expression, x_val: f64) -> f64 {
    let x = Symbol::new("x");
    match expr {
        Expression::Number(num) => match num {
            mathhook_core::core::Number::Integer(i) => *i as f64,
            mathhook_core::core::Number::Rational(r) => {
                use num_traits::ToPrimitive;
                r.to_f64().unwrap_or(0.0)
            }
            mathhook_core::core::Number::Float(f) => *f,
            _ => 0.0,
        },
        Expression::Symbol(sym) if sym == &x => x_val,
        Expression::Add(terms) => terms.iter().map(|t| evaluate_at(t, x_val)).sum(),
        Expression::Mul(factors) => factors.iter().map(|f| evaluate_at(f, x_val)).product(),
        Expression::Pow(base, exp) => {
            let base_val = evaluate_at(base, x_val);
            let exp_val = evaluate_at(exp, x_val);
            base_val.powf(exp_val)
        }
        _ => 0.0,
    }
}

#[test]
fn test_legendre_p0_symbolic_exact() {
    let p0 = expand_legendre_symbolic(0);
    assert_eq!(p0, Expression::integer(1));
}

#[test]
fn test_legendre_p1_symbolic_exact() {
    let p1 = expand_legendre_symbolic(1);
    assert_eq!(p1, Expression::symbol("x"));
}

#[test]
fn test_legendre_p2_symbolic_vs_numerical() {
    let p2 = expand_legendre_symbolic(2);

    for &x_val in &[-1.0, -0.5, 0.0, 0.5, 1.0] {
        let symbolic_result = evaluate_at(&p2, x_val);
        let numerical_result = evaluate_legendre_numerical(&[2.0, x_val])[0];
        assert!(
            (symbolic_result - numerical_result).abs() < 1e-10,
            "P_2({}) mismatch: symbolic={}, numerical={}",
            x_val,
            symbolic_result,
            numerical_result
        );
    }
}

#[test]
fn test_legendre_p3_symbolic_vs_numerical() {
    let p3 = expand_legendre_symbolic(3);

    for &x_val in &[-1.0, -0.5, 0.0, 0.5, 1.0] {
        let symbolic_result = evaluate_at(&p3, x_val);
        let numerical_result = evaluate_legendre_numerical(&[3.0, x_val])[0];
        assert!(
            (symbolic_result - numerical_result).abs() < 1e-10,
            "P_3({}) mismatch: symbolic={}, numerical={}",
            x_val,
            symbolic_result,
            numerical_result
        );
    }
}

#[test]
fn test_legendre_p5_symbolic_vs_numerical() {
    let p5 = expand_legendre_symbolic(5);

    for &x_val in &[-1.0, -0.5, 0.0, 0.5, 1.0] {
        let symbolic_result = evaluate_at(&p5, x_val);
        let numerical_result = evaluate_legendre_numerical(&[5.0, x_val])[0];
        assert!(
            (symbolic_result - numerical_result).abs() < 1e-9,
            "P_5({}) mismatch: symbolic={}, numerical={}",
            x_val,
            symbolic_result,
            numerical_result
        );
    }
}

#[test]
fn test_legendre_special_value_p_n_at_1() {
    for n in 0..6 {
        let p_n = expand_legendre_symbolic(n);
        let result = evaluate_at(&p_n, 1.0);
        assert!(
            (result - 1.0).abs() < 1e-10,
            "P_{}(1) = {} (expected 1)",
            n,
            result
        );
    }
}

#[test]
fn test_hermite_h0_h1_symbolic_exact() {
    let h0 = expand_hermite_symbolic(0);
    let h1 = expand_hermite_symbolic(1);

    assert_eq!(h0, Expression::integer(1));
    assert_eq!(
        h1,
        Expression::mul(vec![Expression::integer(2), Expression::symbol("x")])
    );
}

#[test]
fn test_hermite_h2_symbolic_vs_numerical() {
    let h2 = expand_hermite_symbolic(2);

    for &x_val in &[-2.0, -1.0, 0.0, 1.0, 2.0] {
        let symbolic_result = evaluate_at(&h2, x_val);
        let numerical_result = evaluate_hermite_numerical(&[2.0, x_val])[0];
        assert!(
            (symbolic_result - numerical_result).abs() < 1e-10,
            "H_2({}) mismatch: symbolic={}, numerical={}",
            x_val,
            symbolic_result,
            numerical_result
        );
    }
}

#[test]
fn test_hermite_h3_symbolic_vs_numerical() {
    let h3 = expand_hermite_symbolic(3);

    for &x_val in &[-2.0, -1.0, 0.0, 1.0, 2.0] {
        let symbolic_result = evaluate_at(&h3, x_val);
        let numerical_result = evaluate_hermite_numerical(&[3.0, x_val])[0];
        assert!(
            (symbolic_result - numerical_result).abs() < 1e-10,
            "H_3({}) mismatch: symbolic={}, numerical={}",
            x_val,
            symbolic_result,
            numerical_result
        );
    }
}

#[test]
fn test_hermite_h5_symbolic_vs_numerical() {
    let h5 = expand_hermite_symbolic(5);

    for &x_val in &[-2.0, -1.0, 0.0, 1.0, 2.0] {
        let symbolic_result = evaluate_at(&h5, x_val);
        let numerical_result = evaluate_hermite_numerical(&[5.0, x_val])[0];
        assert!(
            (symbolic_result - numerical_result).abs() < 1e-8,
            "H_5({}) mismatch: symbolic={}, numerical={}",
            x_val,
            symbolic_result,
            numerical_result
        );
    }
}

#[test]
fn test_laguerre_l0_l1_symbolic_exact() {
    let l0 = expand_laguerre_symbolic(0);
    let l1 = expand_laguerre_symbolic(1);

    assert_eq!(l0, Expression::integer(1));

    let l1_at_0 = evaluate_at(&l1, 0.0);
    assert!((l1_at_0 - 1.0).abs() < 1e-10);
}

#[test]
fn test_laguerre_l2_symbolic_vs_numerical() {
    let l2 = expand_laguerre_symbolic(2);

    for &x_val in &[0.0, 0.5, 1.0, 2.0, 3.0] {
        let symbolic_result = evaluate_at(&l2, x_val);
        let numerical_result = evaluate_laguerre_numerical(&[2.0, x_val])[0];
        assert!(
            (symbolic_result - numerical_result).abs() < 1e-10,
            "L_2({}) mismatch: symbolic={}, numerical={}",
            x_val,
            symbolic_result,
            numerical_result
        );
    }
}

#[test]
fn test_laguerre_l3_symbolic_vs_numerical() {
    let l3 = expand_laguerre_symbolic(3);

    for &x_val in &[0.0, 0.5, 1.0, 2.0, 3.0] {
        let symbolic_result = evaluate_at(&l3, x_val);
        let numerical_result = evaluate_laguerre_numerical(&[3.0, x_val])[0];
        assert!(
            (symbolic_result - numerical_result).abs() < 1e-10,
            "L_3({}) mismatch: symbolic={}, numerical={}",
            x_val,
            symbolic_result,
            numerical_result
        );
    }
}

#[test]
fn test_laguerre_l5_symbolic_vs_numerical() {
    let l5 = expand_laguerre_symbolic(5);

    for &x_val in &[0.0, 0.5, 1.0, 2.0, 3.0] {
        let symbolic_result = evaluate_at(&l5, x_val);
        let numerical_result = evaluate_laguerre_numerical(&[5.0, x_val])[0];
        assert!(
            (symbolic_result - numerical_result).abs() < 1e-9,
            "L_5({}) mismatch: symbolic={}, numerical={}",
            x_val,
            symbolic_result,
            numerical_result
        );
    }
}

#[test]
fn test_chebyshev_first_t0_t1_symbolic_exact() {
    let t0 = expand_chebyshev_first_symbolic(0);
    let t1 = expand_chebyshev_first_symbolic(1);

    assert_eq!(t0, Expression::integer(1));
    assert_eq!(t1, Expression::symbol("x"));
}

#[test]
fn test_chebyshev_first_t2_symbolic_vs_numerical() {
    let t2 = expand_chebyshev_first_symbolic(2);

    for &x_val in &[-1.0, -0.5, 0.0, 0.5, 1.0] {
        let symbolic_result = evaluate_at(&t2, x_val);
        let numerical_result = evaluate_chebyshev_first_numerical(&[2.0, x_val])[0];
        assert!(
            (symbolic_result - numerical_result).abs() < 1e-10,
            "T_2({}) mismatch: symbolic={}, numerical={}",
            x_val,
            symbolic_result,
            numerical_result
        );
    }
}

#[test]
fn test_chebyshev_first_t3_symbolic_vs_numerical() {
    let t3 = expand_chebyshev_first_symbolic(3);

    for &x_val in &[-1.0, -0.5, 0.0, 0.5, 1.0] {
        let symbolic_result = evaluate_at(&t3, x_val);
        let numerical_result = evaluate_chebyshev_first_numerical(&[3.0, x_val])[0];
        assert!(
            (symbolic_result - numerical_result).abs() < 1e-10,
            "T_3({}) mismatch: symbolic={}, numerical={}",
            x_val,
            symbolic_result,
            numerical_result
        );
    }
}

#[test]
fn test_chebyshev_first_t5_symbolic_vs_numerical() {
    let t5 = expand_chebyshev_first_symbolic(5);

    for &x_val in &[-1.0, -0.5, 0.0, 0.5, 1.0] {
        let symbolic_result = evaluate_at(&t5, x_val);
        let numerical_result = evaluate_chebyshev_first_numerical(&[5.0, x_val])[0];
        assert!(
            (symbolic_result - numerical_result).abs() < 1e-9,
            "T_5({}) mismatch: symbolic={}, numerical={}",
            x_val,
            symbolic_result,
            numerical_result
        );
    }
}

#[test]
fn test_chebyshev_second_u0_u1_symbolic_exact() {
    let u0 = expand_chebyshev_second_symbolic(0);
    let u1 = expand_chebyshev_second_symbolic(1);

    assert_eq!(u0, Expression::integer(1));
    assert_eq!(
        u1,
        Expression::mul(vec![Expression::integer(2), Expression::symbol("x")])
    );
}

#[test]
fn test_chebyshev_second_u2_symbolic_vs_numerical() {
    let u2 = expand_chebyshev_second_symbolic(2);

    for &x_val in &[-1.0, -0.5, 0.0, 0.5, 1.0] {
        let symbolic_result = evaluate_at(&u2, x_val);
        let numerical_result = evaluate_chebyshev_second_numerical(&[2.0, x_val])[0];
        assert!(
            (symbolic_result - numerical_result).abs() < 1e-10,
            "U_2({}) mismatch: symbolic={}, numerical={}",
            x_val,
            symbolic_result,
            numerical_result
        );
    }
}

#[test]
fn test_chebyshev_second_u3_symbolic_vs_numerical() {
    let u3 = expand_chebyshev_second_symbolic(3);

    for &x_val in &[-1.0, -0.5, 0.0, 0.5, 1.0] {
        let symbolic_result = evaluate_at(&u3, x_val);
        let numerical_result = evaluate_chebyshev_second_numerical(&[3.0, x_val])[0];
        assert!(
            (symbolic_result - numerical_result).abs() < 1e-10,
            "U_3({}) mismatch: symbolic={}, numerical={}",
            x_val,
            symbolic_result,
            numerical_result
        );
    }
}

#[test]
fn test_chebyshev_second_u5_symbolic_vs_numerical() {
    let u5 = expand_chebyshev_second_symbolic(5);

    for &x_val in &[-1.0, -0.5, 0.0, 0.5, 1.0] {
        let symbolic_result = evaluate_at(&u5, x_val);
        let numerical_result = evaluate_chebyshev_second_numerical(&[5.0, x_val])[0];
        assert!(
            (symbolic_result - numerical_result).abs() < 1e-9,
            "U_5({}) mismatch: symbolic={}, numerical={}",
            x_val,
            symbolic_result,
            numerical_result
        );
    }
}

#[test]
fn test_legendre_p3_explicit_coefficients() {
    let p3 = expand_legendre_symbolic(3);

    let at_0 = evaluate_at(&p3, 0.0);
    assert!((at_0 - 0.0).abs() < 1e-10, "P_3(0) should be 0");

    let at_1 = evaluate_at(&p3, 1.0);
    assert!((at_1 - 1.0).abs() < 1e-10, "P_3(1) should be 1");

    let at_minus_1 = evaluate_at(&p3, -1.0);
    assert!((at_minus_1 - (-1.0)).abs() < 1e-10, "P_3(-1) should be -1");
}

#[test]
fn test_hermite_h3_explicit_coefficients() {
    let h3 = expand_hermite_symbolic(3);

    let at_0 = evaluate_at(&h3, 0.0);
    assert!((at_0 - 0.0).abs() < 1e-10, "H_3(0) should be 0");

    let at_1 = evaluate_at(&h3, 1.0);
    assert!((at_1 - (-4.0)).abs() < 1e-10, "H_3(1) should be -4");
}

#[test]
fn test_laguerre_l2_explicit_coefficients() {
    let l2 = expand_laguerre_symbolic(2);

    let at_0 = evaluate_at(&l2, 0.0);
    assert!((at_0 - 1.0).abs() < 1e-10, "L_2(0) should be 1");

    let at_2 = evaluate_at(&l2, 2.0);
    assert!((at_2 - (-1.0)).abs() < 1e-10, "L_2(2) should be -1");
}

#[test]
fn test_chebyshev_first_t2_explicit_coefficients() {
    let t2 = expand_chebyshev_first_symbolic(2);

    let at_0 = evaluate_at(&t2, 0.0);
    assert!((at_0 - (-1.0)).abs() < 1e-10, "T_2(0) should be -1");

    let at_1 = evaluate_at(&t2, 1.0);
    assert!((at_1 - 1.0).abs() < 1e-10, "T_2(1) should be 1");
}

#[test]
fn test_chebyshev_second_u2_explicit_coefficients() {
    let u2 = expand_chebyshev_second_symbolic(2);

    let at_0 = evaluate_at(&u2, 0.0);
    assert!((at_0 - (-1.0)).abs() < 1e-10, "U_2(0) should be -1");

    let at_1 = evaluate_at(&u2, 1.0);
    assert!((at_1 - 3.0).abs() < 1e-10, "U_2(1) should be 3");
}

#[test]
fn test_all_families_low_order_consistency() {
    let test_points = vec![-1.0, -0.5, 0.0, 0.5, 1.0];

    for n in 0..4 {
        for &x_val in &test_points {
            let p_n = expand_legendre_symbolic(n);
            let p_sym = evaluate_at(&p_n, x_val);
            let p_num = evaluate_legendre_numerical(&[n as f64, x_val])[0];
            assert!(
                (p_sym - p_num).abs() < 1e-10,
                "Legendre P_{}({}) failed consistency",
                n,
                x_val
            );

            let t_n = expand_chebyshev_first_symbolic(n);
            let t_sym = evaluate_at(&t_n, x_val);
            let t_num = evaluate_chebyshev_first_numerical(&[n as f64, x_val])[0];
            assert!(
                (t_sym - t_num).abs() < 1e-10,
                "Chebyshev T_{}({}) failed consistency",
                n,
                x_val
            );

            let u_n = expand_chebyshev_second_symbolic(n);
            let u_sym = evaluate_at(&u_n, x_val);
            let u_num = evaluate_chebyshev_second_numerical(&[n as f64, x_val])[0];
            assert!(
                (u_sym - u_num).abs() < 1e-10,
                "Chebyshev U_{}({}) failed consistency",
                n,
                x_val
            );
        }
    }

    for n in 0..4 {
        for &x_val in &[0.0, 0.5, 1.0, 2.0] {
            let h_n = expand_hermite_symbolic(n);
            let h_sym = evaluate_at(&h_n, x_val);
            let h_num = evaluate_hermite_numerical(&[n as f64, x_val])[0];
            assert!(
                (h_sym - h_num).abs() < 1e-10,
                "Hermite H_{}({}) failed consistency",
                n,
                x_val
            );

            let l_n = expand_laguerre_symbolic(n);
            let l_sym = evaluate_at(&l_n, x_val);
            let l_num = evaluate_laguerre_numerical(&[n as f64, x_val])[0];
            assert!(
                (l_sym - l_num).abs() < 1e-10,
                "Laguerre L_{}({}) failed consistency",
                n,
                x_val
            );
        }
    }
}
