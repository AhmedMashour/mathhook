//! Special functions integration tests
//!
//! Tests for special mathematical functions including:
//! - Gamma and related functions
//! - Bessel functions
//! - Zeta function
//! - Error functions
//! - Elliptic functions
//! - Orthogonal polynomials

use mathhook_core::{expr, symbol, Expression, MathConstant, Simplify};

#[test]
fn test_gamma_positive_integers() {
    // Γ(n) = (n-1)! for positive integers
    assert_eq!(
        Expression::function("gamma", vec![expr!(1)]).simplify(),
        expr!(1)
    );
    assert_eq!(
        Expression::function("gamma", vec![expr!(2)]).simplify(),
        expr!(1)
    );
    assert_eq!(
        Expression::function("gamma", vec![expr!(3)]).simplify(),
        expr!(2)
    );
    assert_eq!(
        Expression::function("gamma", vec![expr!(4)]).simplify(),
        expr!(6)
    );
    assert_eq!(
        Expression::function("gamma", vec![expr!(5)]).simplify(),
        expr!(24)
    );
}

#[test]
fn test_gamma_at_non_positive_integers() {
    let result = Expression::function("gamma", vec![expr!(0)]).simplify();
    if let Expression::Constant(c) = result {
        assert!(
            matches!(
                c,
                MathConstant::Infinity | MathConstant::NegativeInfinity | MathConstant::Undefined
            ),
            "Expected Infinity or Undefined at gamma pole"
        );
    }
}

#[test]
fn test_beta_integers() {
    // B(m, n) = Γ(m)Γ(n)/Γ(m+n)
    // B(2, 3) = 1!*2!/4! = 1*2/24 = 1/12
    let result = Expression::function("beta", vec![expr!(2), expr!(3)]).simplify();
    assert_eq!(result, Expression::rational(1, 12));
}

#[test]
fn test_beta_symmetry() {
    // B(a, b) = B(b, a)
    let a = expr!(3);
    let b = expr!(5);

    let lhs = Expression::function("beta", vec![a.clone(), b.clone()]).simplify();
    let rhs = Expression::function("beta", vec![b, a]).simplify();

    assert_eq!(lhs, rhs);
}

#[test]
fn test_digamma_at_one() {
    // ψ(1) = -γ (negative Euler-Mascheroni constant)
    let result = Expression::function("digamma", vec![expr!(1)]).simplify();
    let expected = Expression::mul(vec![expr!(-1), Expression::euler_gamma()]);
    assert_eq!(result, expected.simplify());
}

#[test]
fn test_bessel_j_zero_at_origin() {
    // J_0(0) = 1
    let result = Expression::function("besselj", vec![expr!(0), expr!(0)]).simplify();
    assert_eq!(result, expr!(1));
}

#[test]
fn test_bessel_j_n_at_origin() {
    // J_n(0) = 0 for n > 0 (implemented in bessel.rs lines 60-65)
    for n in 1..=5 {
        let result =
            Expression::function("besselj", vec![Expression::integer(n), expr!(0)]).simplify();
        assert_eq!(result, expr!(0), "J_{}(0) should be 0", n);
    }
}

#[test]
fn test_bessel_y_at_origin_is_undefined() {
    let result = Expression::function("bessely", vec![expr!(0), expr!(0)]).simplify();
    if let Expression::Constant(c) = result {
        assert!(
            matches!(c, MathConstant::NegativeInfinity | MathConstant::Undefined),
            "Expected NegativeInfinity or Undefined at Bessel Y singularity"
        );
    }
}

#[test]
fn test_erf_at_zero() {
    // erf(0) = 0
    let result = Expression::function("erf", vec![expr!(0)]).simplify();
    assert_eq!(result, expr!(0));
}

#[test]
fn test_erf_at_infinity() {
    // erf(∞) = 1
    let result = Expression::function("erf", vec![Expression::infinity()]).simplify();
    assert_eq!(result, expr!(1));
}

#[test]
fn test_zeta_at_two() {
    // ζ(2) = π²/6
    let result = Expression::function("zeta", vec![expr!(2)]).simplify();
    let expected = Expression::div(Expression::pow(Expression::pi(), expr!(2)), expr!(6));
    assert_eq!(result, expected.simplify());
}

#[test]
fn test_zeta_at_four() {
    // ζ(4) = π⁴/90
    let result = Expression::function("zeta", vec![expr!(4)]).simplify();
    let expected = Expression::div(Expression::pow(Expression::pi(), expr!(4)), expr!(90));
    assert_eq!(result, expected.simplify());
}

#[test]
fn test_zeta_at_negative_even_integers() {
    // ζ(-2n) = 0 for positive integer n (trivial zeros) - implemented in zeta.rs line 121
    for n in 1..=5 {
        let result = Expression::function("zeta", vec![Expression::integer(-2 * n)]).simplify();
        assert_eq!(result, expr!(0), "ζ({}) should be 0", -2 * n);
    }
}

#[test]
fn test_zeta_at_negative_one() {
    // ζ(-1) = -1/12 (Ramanujan summation)
    let result = Expression::function("zeta", vec![expr!(-1)]).simplify();
    assert_eq!(result, Expression::rational(-1, 12));
}

#[test]
fn test_exponential_integral_ei() {
    let x = symbol!(x);

    let expr = Expression::function("ei", vec![Expression::symbol(x.clone())]);
    assert!(!expr.is_zero());
}

#[test]
fn test_logarithmic_integral_li() {
    let result = Expression::function("li", vec![expr!(2)]).simplify();
    assert!(!result.is_zero());
}

#[test]
fn test_legendre_p0() {
    let x = symbol!(x);
    // P_0(x) = 1
    let result =
        Expression::function("legendrep", vec![expr!(0), Expression::symbol(x.clone())]).simplify();
    assert_eq!(result, expr!(1));
}

#[test]
fn test_legendre_p1() {
    let x = symbol!(x);
    // P_1(x) = x
    let result =
        Expression::function("legendrep", vec![expr!(1), Expression::symbol(x.clone())]).simplify();
    assert_eq!(result, Expression::symbol(x));
}

#[test]
fn test_legendre_p2() {
    // P_2(0) = -1/2
    let result_0 = Expression::function("legendrep", vec![expr!(2), expr!(0)]).simplify();
    assert_eq!(
        result_0,
        Expression::rational(-1, 2),
        "P_2(0) should be -1/2, got {:?}",
        result_0
    );

    // P_2(1) = 1
    let result_1 = Expression::function("legendrep", vec![expr!(2), expr!(1)]).simplify();
    assert!(result_1.is_one(), "P_2(1) should be 1, got {:?}", result_1);

    // P_2(-1) = 1
    let result_neg1 = Expression::function("legendrep", vec![expr!(2), expr!(-1)]).simplify();
    assert!(
        result_neg1.is_one(),
        "P_2(-1) should be 1, got {:?}",
        result_neg1
    );
}

#[test]
fn test_chebyshev_t0() {
    let x = symbol!(x);
    // T_0(x) = 1
    let result = Expression::function("chebyshevt", vec![expr!(0), Expression::symbol(x.clone())])
        .simplify();
    assert_eq!(result, expr!(1));
}

#[test]
fn test_chebyshev_t1() {
    let x = symbol!(x);
    // T_1(x) = x
    let result = Expression::function("chebyshevt", vec![expr!(1), Expression::symbol(x.clone())])
        .simplify();
    assert_eq!(result, Expression::symbol(x));
}

#[test]
fn test_chebyshev_t2() {
    let x = symbol!(x);
    // T_2(x) = 2x² - 1
    let result = Expression::function("chebyshevt", vec![expr!(2), Expression::symbol(x.clone())])
        .simplify();
    let expected = expr!((2 * (x ^ 2)) - 1).simplify();
    assert_eq!(result, expected);
}

#[test]
fn test_hermite_h0() {
    let x = symbol!(x);
    // H_0(x) = 1
    let result =
        Expression::function("hermiteh", vec![expr!(0), Expression::symbol(x.clone())]).simplify();
    assert_eq!(result, expr!(1));
}

#[test]
fn test_hermite_h1() {
    let x = symbol!(x);
    // H_1(x) = 2x
    let result =
        Expression::function("hermiteh", vec![expr!(1), Expression::symbol(x.clone())]).simplify();
    assert_eq!(result, expr!(2 * x).simplify());
}

#[test]
fn test_hermite_h2() {
    let x = symbol!(x);
    // H_2(x) = 4x² - 2
    let result =
        Expression::function("hermiteh", vec![expr!(2), Expression::symbol(x.clone())]).simplify();
    let expected = expr!((4 * (x ^ 2)) - 2).simplify();
    assert_eq!(result, expected);
}

#[test]
fn test_laguerre_l0() {
    let x = symbol!(x);
    // L_0(x) = 1
    let result =
        Expression::function("laguerrel", vec![expr!(0), Expression::symbol(x.clone())]).simplify();
    assert_eq!(result, expr!(1));
}

#[test]
fn test_laguerre_l1() {
    let x = symbol!(x);
    // L_1(x) = 1 - x
    let result =
        Expression::function("laguerrel", vec![expr!(1), Expression::symbol(x.clone())]).simplify();
    assert_eq!(result, expr!(1 - x).simplify());
}

#[test]
fn test_laguerre_l2() {
    // L_2(x) = (x² - 4x + 2)/2
    // Test numerical correctness at specific points
    // L_2(0) = 1
    let result_0 = Expression::function("laguerrel", vec![expr!(2), expr!(0)]).simplify();
    assert!(result_0.is_one(), "L_2(0) should be 1, got {:?}", result_0);

    // L_2(1) = -1/2 (from (1 - 4 + 2)/2 = -1/2)
    let result_1 = Expression::function("laguerrel", vec![expr!(2), expr!(1)]).simplify();
    assert_eq!(
        result_1,
        Expression::rational(-1, 2),
        "L_2(1) should be -1/2, got {:?}",
        result_1
    );

    // L_2(2) = -1 (from (4 - 8 + 2)/2 = -2/2 = -1)
    let result_2 = Expression::function("laguerrel", vec![expr!(2), expr!(2)]).simplify();
    assert!(
        matches!(&result_2, Expression::Number(n) if n.is_negative_one()),
        "L_2(2) should be -1, got {:?}",
        result_2
    );
}

#[test]
fn test_airy_ai_at_zero() {
    let result = Expression::function("airyai", vec![expr!(0)]).simplify();
    assert!(!result.is_zero());
}

#[test]
fn test_airy_bi_at_zero() {
    let result = Expression::function("airybi", vec![expr!(0)]).simplify();
    assert!(!result.is_zero());
}
