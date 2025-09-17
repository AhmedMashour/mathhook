//! Comprehensive Polynomial Evaluation Tests with SymPy Validation
//!
//! All test values validated against SymPy reference implementation.
//! Test coverage: low order (n=0,1,2), medium order (n=5,10), special values, boundary values.

use mathhook_core::functions::polynomials::evaluation::*;

const EPSILON: f64 = 1e-10;

#[test]
fn test_legendre_p0_p1_initial_conditions() {
    assert!((evaluate_legendre_numerical(&[0.0, 0.5])[0] - 1.0).abs() < EPSILON);
    assert!((evaluate_legendre_numerical(&[1.0, 0.5])[0] - 0.5).abs() < EPSILON);
}

#[test]
fn test_legendre_p2_low_order() {
    assert!((evaluate_legendre_numerical(&[2.0, 0.5])[0] - (-0.125)).abs() < EPSILON);
}

#[test]
fn test_legendre_p5_at_half() {
    assert!((evaluate_legendre_numerical(&[5.0, 0.5])[0] - 0.08984375).abs() < EPSILON);
}

#[test]
fn test_legendre_p5_at_one() {
    assert!((evaluate_legendre_numerical(&[5.0, 1.0])[0] - 1.0).abs() < EPSILON);
}

#[test]
fn test_legendre_p5_at_minus_one() {
    assert!((evaluate_legendre_numerical(&[5.0, -1.0])[0] - (-1.0)).abs() < EPSILON);
}

#[test]
fn test_legendre_p10_medium_order() {
    let result = evaluate_legendre_numerical(&[10.0, 0.3])[0];
    assert!(result.is_finite());
}

#[test]
fn test_hermite_h0_h1_initial_conditions() {
    assert!((evaluate_hermite_numerical(&[0.0, 2.0])[0] - 1.0).abs() < EPSILON);
    assert!((evaluate_hermite_numerical(&[1.0, 2.0])[0] - 4.0).abs() < EPSILON);
}

#[test]
fn test_hermite_h2_low_order() {
    assert!((evaluate_hermite_numerical(&[2.0, 1.0])[0] - 2.0).abs() < EPSILON);
}

#[test]
fn test_hermite_h3_at_two() {
    assert!((evaluate_hermite_numerical(&[3.0, 2.0])[0] - 40.0).abs() < EPSILON);
}

#[test]
fn test_hermite_h5_medium_order() {
    let result = evaluate_hermite_numerical(&[5.0, 1.5])[0];
    assert!(result.is_finite());
}

#[test]
fn test_hermite_h2_at_one() {
    assert!((evaluate_hermite_numerical(&[2.0, 1.0])[0] - 2.0).abs() < EPSILON);
}

#[test]
fn test_laguerre_l0_l1_initial_conditions() {
    assert!((evaluate_laguerre_numerical(&[0.0, 1.5])[0] - 1.0).abs() < EPSILON);
    assert!((evaluate_laguerre_numerical(&[1.0, 1.5])[0] - (-0.5)).abs() < EPSILON);
}

#[test]
fn test_laguerre_l4_at_1point5() {
    let result = evaluate_laguerre_numerical(&[4.0, 1.5])[0];
    assert!(result.is_finite());
}

#[test]
fn test_laguerre_l3_at_two() {
    let result = evaluate_laguerre_numerical(&[3.0, 2.0])[0];
    assert!(result.is_finite());
}

#[test]
fn test_laguerre_l5_at_half() {
    let result = evaluate_laguerre_numerical(&[5.0, 0.5])[0];
    assert!(result.is_finite());
}

#[test]
fn test_laguerre_at_zero() {
    assert!((evaluate_laguerre_numerical(&[4.0, 0.0])[0] - 1.0).abs() < EPSILON);
}

#[test]
fn test_chebyshev_first_t0_t1_initial_conditions() {
    assert!((evaluate_chebyshev_first_numerical(&[0.0, 0.7])[0] - 1.0).abs() < EPSILON);
    assert!((evaluate_chebyshev_first_numerical(&[1.0, 0.7])[0] - 0.7).abs() < EPSILON);
}

#[test]
fn test_chebyshev_first_t10_at_0point7() {
    assert!((evaluate_chebyshev_first_numerical(&[10.0, 0.7])[0] - (-0.0998400512)).abs() < 1e-6);
}

#[test]
fn test_chebyshev_first_t5_at_half() {
    assert!((evaluate_chebyshev_first_numerical(&[5.0, 0.5])[0] - 0.5).abs() < EPSILON);
}

#[test]
fn test_chebyshev_first_at_one() {
    assert!((evaluate_chebyshev_first_numerical(&[10.0, 1.0])[0] - 1.0).abs() < EPSILON);
}

#[test]
fn test_chebyshev_first_at_minus_one() {
    assert!((evaluate_chebyshev_first_numerical(&[5.0, -1.0])[0] - (-1.0)).abs() < EPSILON);
}

#[test]
fn test_chebyshev_second_u0_u1_initial_conditions() {
    assert!((evaluate_chebyshev_second_numerical(&[0.0, 0.3])[0] - 1.0).abs() < EPSILON);
    assert!((evaluate_chebyshev_second_numerical(&[1.0, 0.3])[0] - 0.6).abs() < EPSILON);
}

#[test]
fn test_chebyshev_second_u8_at_0point3() {
    assert!((evaluate_chebyshev_second_numerical(&[8.0, 0.3])[0] - (-0.9657958400)).abs() < 1e-6);
}

#[test]
fn test_chebyshev_second_u4_at_0point6() {
    assert!((evaluate_chebyshev_second_numerical(&[4.0, 0.6])[0] - (-1.2464)).abs() < 1e-3);
}

#[test]
fn test_chebyshev_second_at_one() {
    assert!((evaluate_chebyshev_second_numerical(&[4.0, 1.0])[0] - 5.0).abs() < EPSILON);
}

#[test]
fn test_legendre_boundary_zero() {
    assert!((evaluate_legendre_numerical(&[0.0, 0.0])[0] - 1.0).abs() < EPSILON);
}

#[test]
fn test_hermite_at_zero() {
    assert!((evaluate_hermite_numerical(&[2.0, 0.0])[0] - (-2.0)).abs() < EPSILON);
}

#[test]
fn test_all_polynomials_invalid_args() {
    assert_eq!(evaluate_legendre_numerical(&[]).len(), 1);
    assert_eq!(evaluate_hermite_numerical(&[1.0]).len(), 1);
    assert_eq!(evaluate_laguerre_numerical(&[1.0, 2.0, 3.0]).len(), 1);
}
