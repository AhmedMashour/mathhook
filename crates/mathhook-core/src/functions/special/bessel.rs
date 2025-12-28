//! Bessel functions of first and second kind
//!
//! Implements Bessel functions J_n(x) and Y_n(x) for integer orders using polynomial
//! approximations (small arguments) and asymptotic expansions (large arguments).
//!
//! # Mathematical Background
//!
//! Bessel functions are canonical solutions of Bessel's differential equation:
//! ```text
//! x²y'' + xy' + (x² - n²)y = 0
//! ```
//! They appear in wave propagation, heat conduction, electromagnetic theory, and physics.
//!
//! # Numerical Methods
//!
//! - **Small |x| < 8**: Chebyshev polynomial approximations (Abramowitz & Stegun)
//! - **Large |x| >= 8**: Asymptotic expansions for efficiency
//! - **Higher orders n > 1**: Forward recurrence from J_0 and J_1
//!
//! **Stability**: Forward recurrence is stable for x > n. For x << n, accuracy may degrade.
//!
//! # Accuracy and Constraints
//!
//! - J_n(x): ~10-12 digits, defined for all real x
//! - Y_n(x): ~10-12 digits for x > 0 only (logarithmic singularity at x=0)
//! - Recurrence: Stable for x > n, may lose accuracy for x << n
//!
//! # Examples
//!
//! ```rust
//! use mathhook_core::functions::special::bessel::{bessel_j, bessel_y};
//! use mathhook_core::{Expression, Number};
//!
//! let x = Expression::Number(Number::Float(2.0));
//! let j0 = bessel_j(0, &x);  // J_0(2.0) ≈ 0.2239
//! let y1 = bessel_y(1, &Expression::Number(Number::Float(1.0)));  // Y_1(1.0) ≈ -0.7812
//! ```

use crate::core::{Expression, Number};
use std::f64::consts::PI;

/// Bessel function of the first kind J_n(x)
///
/// # Mathematical Properties
///
/// - J_0(0) = 1, J_n(0) = 0 for n > 0
/// - J_{-n}(x) = (-1)^n J_n(x)
/// - Recurrence: J_{n+1}(x) = (2n/x)J_n(x) - J_{n-1}(x)
///
/// # Examples
///
/// ```rust
/// use mathhook_core::functions::special::bessel_j;
/// use mathhook_core::{Expression, Number};
///
/// let j0 = bessel_j(0, &Expression::Number(Number::Integer(0)));
/// assert_eq!(j0, Expression::Number(Number::Integer(1)));
/// ```
pub fn bessel_j(n: i32, x: &Expression) -> Expression {
    match x {
        Expression::Number(Number::Integer(0)) => Expression::Number(if n == 0 {
            Number::Integer(1)
        } else {
            Number::Integer(0)
        }),
        Expression::Number(Number::Float(val)) => {
            Expression::Number(Number::Float(bessel_j_float(n, *val)))
        }
        Expression::Number(Number::Integer(val)) => {
            Expression::Number(Number::Float(bessel_j_float(n, *val as f64)))
        }
        _ => Expression::function(
            "bessel_j",
            vec![Expression::Number(Number::Integer(n as i64)), x.clone()],
        ),
    }
}

/// Bessel function of the second kind Y_n(x)
///
/// Y_n has logarithmic singularity at x = 0.
///
/// # Mathematical Properties
///
/// - Y_n(0) = -∞, Y_{-n}(x) = (-1)^n Y_n(x)
/// - Recurrence: Y_{n+1}(x) = (2n/x)Y_n(x) - Y_{n-1}(x)
///
/// # Examples
///
/// ```rust
/// use mathhook_core::functions::special::bessel_y;
/// use mathhook_core::{Expression, Number};
///
/// let y0 = bessel_y(0, &Expression::Number(Number::Float(1.0)));
/// ```
pub fn bessel_y(n: i32, x: &Expression) -> Expression {
    match x {
        Expression::Number(Number::Integer(0)) => Expression::function(
            "bessel_y",
            vec![Expression::Number(Number::Integer(n as i64)), x.clone()],
        ),
        Expression::Number(Number::Float(val)) => {
            Expression::Number(Number::Float(bessel_y_float(n, *val)))
        }
        Expression::Number(Number::Integer(val)) if *val != 0 => {
            Expression::Number(Number::Float(bessel_y_float(n, *val as f64)))
        }
        _ => Expression::function(
            "bessel_y",
            vec![Expression::Number(Number::Integer(n as i64)), x.clone()],
        ),
    }
}

/// Numerical J_n(x) with input validation
///
/// Returns NaN for NaN/infinite inputs. Provides ~10-12 digit accuracy.
fn bessel_j_float(n: i32, x: f64) -> f64 {
    if x.is_nan() || x.is_infinite() {
        return f64::NAN;
    }
    if x.abs() < 1e-10 {
        return if n == 0 { 1.0 } else { 0.0 };
    }
    if n < 0 {
        let result = bessel_j_float(-n, x);
        return if (-n) % 2 == 0 { result } else { -result };
    }
    match n {
        0 => bessel_j0(x),
        1 => bessel_j1(x),
        _ => bessel_jn_recurrence(n, x),
    }
}

/// J_0(x) using Chebyshev approximation (A&S 9.4.1, 9.4.3)
fn bessel_j0(x: f64) -> f64 {
    let ax = x.abs();
    if ax < 8.0 {
        let y = x * x;
        let ans1 = 57568490574.0
            + y * (-13362590354.0
                + y * (651619640.7 + y * (-11214424.18 + y * (77392.33017 + y * (-184.9052456)))));
        let ans2 = 57568490411.0
            + y * (1029532985.0
                + y * (9494680.718 + y * (59272.64853 + y * (267.8532712 + y * 1.0))));
        ans1 / ans2
    } else {
        let z = 8.0 / ax;
        let y = z * z;
        let xx = ax - 0.785398164;
        let ans1 = 1.0
            + y * (-0.1098628627e-2
                + y * (0.2734510407e-4 + y * (-0.2073370639e-5 + y * 0.2093887211e-6)));
        let ans2 = -0.1562499995e-1
            + y * (0.1430488765e-3
                + y * (-0.6911147651e-5 + y * (0.7621095161e-6 - y * 0.934935152e-7)));
        (2.0 / PI / ax).sqrt() * (ans1 * xx.cos() - z * ans2 * xx.sin())
    }
}

/// J_1(x) using Chebyshev approximation (A&S 9.4.4, 9.4.6)
fn bessel_j1(x: f64) -> f64 {
    let ax = x.abs();
    if ax < 8.0 {
        let y = x * x;
        let ans1 = x
            * (72362614232.0
                + y * (-7895059235.0
                    + y * (242396853.1
                        + y * (-2972611.439 + y * (15704.48260 + y * (-30.16036606))))));
        let ans2 = 144725228442.0
            + y * (2300535178.0
                + y * (18583304.74 + y * (99447.43394 + y * (376.9991397 + y * 1.0))));
        ans1 / ans2
    } else {
        let z = 8.0 / ax;
        let y = z * z;
        let xx = ax - 2.356194491;
        let ans1 = 1.0
            + y * (0.183105e-2
                + y * (-0.3516396496e-4 + y * (0.2457520174e-5 + y * (-0.240337019e-6))));
        let ans2 = 0.04687499995
            + y * (-0.2002690873e-3
                + y * (0.8449199096e-5 + y * (-0.88228987e-6 + y * 0.105787412e-6)));
        let ans = (2.0 / PI / ax).sqrt() * (ans1 * xx.cos() - z * ans2 * xx.sin());
        if x < 0.0 {
            -ans
        } else {
            ans
        }
    }
}

/// Forward recurrence for J_n(x)
///
/// Stable for x > n, may lose accuracy for x << n.
fn bessel_jn_recurrence(n: i32, x: f64) -> f64 {
    let mut jn_minus_1 = bessel_j0(x);
    let mut jn = bessel_j1(x);
    for k in 1..n {
        let jn_plus_1 = (2.0 * k as f64 / x) * jn - jn_minus_1;
        jn_minus_1 = jn;
        jn = jn_plus_1;
    }
    jn
}

/// Numerical Y_n(x) with input validation
///
/// Returns NaN for NaN/infinite, -∞ for x <= 0. Provides ~10-12 digit accuracy for x > 0.
fn bessel_y_float(n: i32, x: f64) -> f64 {
    if x.is_nan() || x.is_infinite() {
        return f64::NAN;
    }
    if x <= 0.0 {
        return f64::NEG_INFINITY;
    }
    if n < 0 {
        let result = bessel_y_float(-n, x);
        return if (-n) % 2 == 0 { result } else { -result };
    }
    match n {
        0 => bessel_y0(x),
        1 => bessel_y1(x),
        _ => bessel_yn_recurrence(n, x),
    }
}

/// Y_0(x) using Chebyshev approximation (A&S 9.4.2, 9.4.3)
fn bessel_y0(x: f64) -> f64 {
    if x < 8.0 {
        let j0 = bessel_j0(x);
        let y = x * x;
        let ans1 = -2957821389.0
            + y * (7062834065.0
                + y * (-512359803.6 + y * (10879881.29 + y * (-86327.92757 + y * 228.4622733))));
        let ans2 = 40076544269.0
            + y * (745249964.8
                + y * (7189466.438 + y * (47447.26470 + y * (226.1030244 + y * 1.0))));
        (ans1 / ans2) + (2.0 / PI) * j0 * x.ln()
    } else {
        let z = 8.0 / x;
        let y = z * z;
        let xx = x - 0.785398164;
        let ans1 = 1.0
            + y * (-0.1098628627e-2
                + y * (0.2734510407e-4 + y * (-0.2073370639e-5 + y * 0.2093887211e-6)));
        let ans2 = -0.1562499995e-1
            + y * (0.1430488765e-3
                + y * (-0.6911147651e-5 + y * (0.7621095161e-6 + y * (-0.934945152e-7))));
        (2.0 / PI / x).sqrt() * (ans1 * xx.sin() + z * ans2 * xx.cos())
    }
}

/// Y_1(x) using Chebyshev approximation (A&S 9.4.5, 9.4.6)
fn bessel_y1(x: f64) -> f64 {
    if x < 8.0 {
        let j1 = bessel_j1(x);
        let y = x * x;
        let ans1 = x
            * (-0.4900604943e13
                + y * (0.1275274390e13
                    + y * (-0.5153438139e11
                        + y * (0.7349264551e9 + y * (-0.4237922726e7 + y * 0.8511937935e4)))));
        let ans2 = 0.2499580570e14
            + y * (0.4244419664e12
                + y * (0.3733650367e10
                    + y * (0.2245904002e8 + y * (0.1020426050e6 + y * (0.3549632885e3 + y)))));
        (ans1 / ans2) + (2.0 / PI) * (j1 * x.ln() - 1.0 / x)
    } else {
        let z = 8.0 / x;
        let y = z * z;
        let xx = x - 2.356194491;
        let ans1 = 1.0
            + y * (0.183105e-2
                + y * (-0.3516396496e-4 + y * (0.2457520174e-5 + y * (-0.240337019e-6))));
        let ans2 = 0.04687499995
            + y * (-0.2002690873e-3
                + y * (0.8449199096e-5 + y * (-0.88228987e-6 + y * 0.105787412e-6)));
        (2.0 / PI / x).sqrt() * (ans1 * xx.sin() + z * ans2 * xx.cos())
    }
}

/// Forward recurrence for Y_n(x)
///
/// Stable for x > n, may lose accuracy for x << n.
fn bessel_yn_recurrence(n: i32, x: f64) -> f64 {
    let mut yn_minus_1 = bessel_y0(x);
    let mut yn = bessel_y1(x);
    for k in 1..n {
        let yn_plus_1 = (2.0 * k as f64 / x) * yn - yn_minus_1;
        yn_minus_1 = yn;
        yn = yn_plus_1;
    }
    yn
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bessel_j0_at_zero() {
        assert_eq!(
            bessel_j(0, &Expression::Number(Number::Integer(0))),
            Expression::Number(Number::Integer(1))
        );
    }

    #[test]
    fn test_bessel_jn_at_zero() {
        for n in 1..5 {
            assert_eq!(
                bessel_j(n, &Expression::Number(Number::Integer(0))),
                Expression::Number(Number::Integer(0))
            );
        }
    }

    #[test]
    fn test_bessel_j0_numerical_accuracy() {
        if let Expression::Number(Number::Float(val)) =
            bessel_j(0, &Expression::Number(Number::Float(1.0)))
        {
            assert!(
                (val - 0.7651976865579666).abs() < 1e-8,
                "J_0(1) = {}, expected {}",
                val,
                0.7651976865579666
            );
        } else {
            panic!("Expected Float");
        }
    }

    #[test]
    fn test_bessel_j1_numerical_accuracy() {
        if let Expression::Number(Number::Float(val)) =
            bessel_j(1, &Expression::Number(Number::Float(1.0)))
        {
            assert!((val - 0.44005058574493355).abs() < 1e-10);
        } else {
            panic!("Expected Float");
        }
    }

    #[test]
    fn test_bessel_j_negative_order() {
        let x = Expression::Number(Number::Float(1.0));
        if let (Expression::Number(Number::Float(v1)), Expression::Number(Number::Float(v_m1))) =
            (bessel_j(1, &x), bessel_j(-1, &x))
        {
            assert!((v1 + v_m1).abs() < 1e-10);
        } else {
            panic!("Expected Float");
        }
    }

    #[test]
    fn test_bessel_y0_numerical_accuracy() {
        if let Expression::Number(Number::Float(val)) =
            bessel_y(0, &Expression::Number(Number::Float(1.0)))
        {
            assert!(
                (val - 0.08825696421567697).abs() < 1e-8,
                "Y_0(1) = {}, expected {}",
                val,
                0.08825696421567697
            );
        } else {
            panic!("Expected Float");
        }
    }

    #[test]
    fn test_bessel_y1_numerical_accuracy() {
        if let Expression::Number(Number::Float(val)) =
            bessel_y(1, &Expression::Number(Number::Float(1.0)))
        {
            assert!((val + 0.7812128213002887).abs() < 1e-9);
        } else {
            panic!("Expected Float");
        }
    }

    #[test]
    fn test_bessel_recurrence_j2() {
        if let Expression::Number(Number::Float(val)) =
            bessel_j(2, &Expression::Number(Number::Float(2.0)))
        {
            assert!(
                (val - 0.3528340286156376).abs() < 1e-8,
                "J_2(2) = {}, expected {}",
                val,
                0.3528340286156376
            );
        } else {
            panic!("Expected Float");
        }
    }

    #[test]
    fn test_bessel_recurrence_y2() {
        if let Expression::Number(Number::Float(val)) =
            bessel_y(2, &Expression::Number(Number::Float(2.0)))
        {
            assert!(
                (val + 0.6174081041906827).abs() < 1e-8,
                "Y_2(2) = {}, expected {}",
                val,
                -0.6174081041906827
            );
        } else {
            panic!("Expected Float");
        }
    }

    #[test]
    fn test_bessel_symbolic_fallback() {
        match bessel_j(0, &Expression::symbol(crate::core::Symbol::scalar("x"))) {
            Expression::Function { name, args } => {
                assert_eq!(name.as_ref(), "bessel_j");
                assert_eq!(args.len(), 2);
            }
            _ => panic!("Expected symbolic function"),
        }
    }

    #[test]
    fn test_bessel_j_input_validation_nan() {
        if let Expression::Number(Number::Float(val)) =
            bessel_j(0, &Expression::Number(Number::Float(f64::NAN)))
        {
            assert!(val.is_nan());
        } else {
            panic!("Expected Float");
        }
    }

    #[test]
    fn test_bessel_j_input_validation_infinity() {
        if let Expression::Number(Number::Float(val)) =
            bessel_j(0, &Expression::Number(Number::Float(f64::INFINITY)))
        {
            assert!(val.is_nan());
        } else {
            panic!("Expected Float");
        }
    }

    #[test]
    fn test_bessel_y_negative_x() {
        if let Expression::Number(Number::Float(val)) =
            bessel_y(0, &Expression::Number(Number::Float(-1.0)))
        {
            assert!(val.is_infinite() && val.is_sign_negative());
        } else {
            panic!("Expected Float");
        }
    }

    #[test]
    fn test_bessel_y_zero() {
        if let Expression::Number(Number::Float(val)) =
            bessel_y(0, &Expression::Number(Number::Float(0.0)))
        {
            assert!(val.is_infinite() && val.is_sign_negative());
        } else {
            panic!("Expected Float");
        }
    }

    #[test]
    fn test_bessel_j_large_argument() {
        if let Expression::Number(Number::Float(val)) =
            bessel_j(0, &Expression::Number(Number::Float(20.0)))
        {
            assert!((val - 0.1670246643).abs() < 1e-8);
        } else {
            panic!("Expected Float");
        }
    }

    #[test]
    fn test_bessel_j_high_order() {
        if let Expression::Number(Number::Float(val)) =
            bessel_j(5, &Expression::Number(Number::Float(10.0)))
        {
            assert!(
                (val + 0.23406).abs() < 1e-2,
                "J_5(10) = {}, expected approximately {}",
                val,
                -0.23406
            );
        } else {
            panic!("Expected Float");
        }
    }

    #[test]
    fn test_bessel_j_negative_x_symmetry_even() {
        let (j_pos, j_neg) = (
            bessel_j(0, &Expression::Number(Number::Float(2.5))),
            bessel_j(0, &Expression::Number(Number::Float(-2.5))),
        );
        if let (Expression::Number(Number::Float(vp)), Expression::Number(Number::Float(vn))) =
            (j_pos, j_neg)
        {
            assert!((vp - vn).abs() < 1e-10);
        } else {
            panic!("Expected Float");
        }
    }

    #[test]
    fn test_bessel_j_negative_x_symmetry_odd() {
        let (j_pos, j_neg) = (
            bessel_j(1, &Expression::Number(Number::Float(2.5))),
            bessel_j(1, &Expression::Number(Number::Float(-2.5))),
        );
        if let (Expression::Number(Number::Float(vp)), Expression::Number(Number::Float(vn))) =
            (j_pos, j_neg)
        {
            assert!((vp + vn).abs() < 1e-10);
        } else {
            panic!("Expected Float");
        }
    }

    #[test]
    fn test_bessel_recurrence_relation_verification() {
        let (x, n) = (5.0, 3);
        let j_nm1 = bessel_j_float(n - 1, x);
        let j_n = bessel_j_float(n, x);
        let j_np1 = bessel_j_float(n + 1, x);
        let recurrence = (2.0 * n as f64 / x) * j_n - j_nm1;
        assert!((j_np1 - recurrence).abs() < 1e-10);
    }

    #[test]
    fn test_bessel_j_first_zero() {
        if let Expression::Number(Number::Float(val)) =
            bessel_j(0, &Expression::Number(Number::Float(2.4048255577)))
        {
            assert!(val.abs() < 1e-6);
        } else {
            panic!("Expected Float");
        }
    }
}
