//! Gamma, Beta, Digamma, and Polygamma special functions with high-precision numerical evaluation.
//!
//! Implements the Gamma function, Beta function, Digamma function, and Polygamma functions
//! with comprehensive numerical evaluation support.
//!
//! # Numerical Evaluation
//!
//! Float inputs are automatically evaluated numerically using the Lanczos approximation
//! (14-digit precision). Half-integer values are handled symbolically for exact results.
//!
//! # Half-Integer Special Cases
//!
//! The gamma function has exact symbolic forms for half-integers:
//! - Γ(1/2) = √π
//! - Γ(3/2) = √π/2
//! - Γ(5/2) = 3√π/4
//! - Γ(n+1/2) = (2n-1)!! · √π / 2^n
//!
//! # Beta Function
//!
//! The beta function B(a, b) = Γ(a)·Γ(b)/Γ(a+b) supports both symbolic and numerical
//! evaluation. Float inputs are evaluated numerically using Lanczos gamma.
//!
//! # Digamma Function
//!
//! The digamma function ψ(z) = d/dz ln(Γ(z)) = Γ'(z)/Γ(z) is the logarithmic derivative
//! of the gamma function. Special values are computed exactly.
//!
//! # Polygamma Function
//!
//! The polygamma function ψ^(n)(z) is the (n+1)-th derivative of ln(Γ(z)).
//! ψ^(0) = digamma, ψ^(1) = trigamma, etc.
//!
//! # Input Validation
//!
//! All numerical functions validate inputs for NaN, infinity, and mathematical poles.
//! Non-positive integers are poles for the gamma function and return infinity.

use crate::core::{Expression, Number};
use std::f64::consts::PI;

/// Gamma function Γ(z)
///
/// The Gamma function extends the factorial to complex numbers:
/// Γ(n) = (n-1)! for positive integers n
///
/// # Mathematical Properties
///
/// - Γ(n+1) = n·Γ(n) (functional equation)
/// - Γ(1) = 1
/// - Γ(1/2) = √π
/// - Pole at non-positive integers
///
/// # Numerical Evaluation
///
/// Float inputs are evaluated numerically using Lanczos approximation (14-digit precision).
/// Half-integers return exact symbolic forms (e.g., Γ(1/2) = √π).
///
/// # Input Validation
///
/// - NaN or infinity inputs return NaN
/// - Non-positive integers are poles (return symbolic or error)
///
/// # Arguments
///
/// * `z` - Expression to evaluate gamma function at
///
/// # Examples
///
/// ```rust
/// use mathhook_core::{Expression, Number};
/// use mathhook_core::functions::special::gamma;
///
/// let result = gamma(&Expression::Number(Number::Integer(5)));
/// assert_eq!(result, Expression::Number(Number::Integer(24)));
///
/// let half = gamma(&Expression::Number(Number::Float(0.5)));
/// ```
pub fn gamma(z: &Expression) -> Expression {
    match z {
        Expression::Number(Number::Integer(n)) if *n > 0 => {
            let val = *n;
            if val == 1 {
                Expression::Number(Number::Integer(1))
            } else {
                let mut result = 1i64;
                for i in 1..val {
                    result *= i;
                }
                Expression::Number(Number::Integer(result))
            }
        }
        Expression::Number(Number::Float(x)) => {
            let twice = x * 2.0;
            if (twice - twice.round()).abs() < 1e-10 {
                gamma_half_integer(*x)
            } else {
                let result = lanczos_gamma(*x);
                Expression::Number(Number::Float(result))
            }
        }
        _ => Expression::function("gamma", vec![z.clone()]),
    }
}

/// Computes gamma function for half-integers symbolically
///
/// Returns exact symbolic expressions using sqrt(pi):
/// - Γ(1/2) = √π
/// - Γ(3/2) = √π/2
/// - Γ(5/2) = 3√π/4
/// - Γ(n+1/2) = (2n-1)!! · √π / 2^n
fn gamma_half_integer(x: f64) -> Expression {
    let n = (x - 0.5).round() as i64;
    if (x - (n as f64 + 0.5)).abs() < 1e-10 && n >= 0 {
        let sqrt_pi = Expression::sqrt(Expression::pi());
        if n == 0 {
            return sqrt_pi;
        }
        let mut double_fact = Expression::integer(1);
        for k in 0..n {
            let term = Expression::integer(2 * k + 1);
            double_fact = Expression::mul(vec![double_fact, term]);
        }
        let numerator = Expression::mul(vec![double_fact, sqrt_pi]);
        let denominator = Expression::pow(Expression::integer(2), Expression::integer(n));
        Expression::div(numerator, denominator)
    } else {
        Expression::Number(Number::Float(lanczos_gamma(x)))
    }
}

/// Lanczos approximation for Gamma function (for numerical evaluation)
///
/// Provides accurate numerical evaluation using the Lanczos approximation
/// with 14-digit precision. This is used for non-special values.
///
/// # Input Validation
///
/// - NaN or infinity inputs return NaN
/// - Non-positive integers (poles) return infinity
///
/// # Examples
///
/// ```rust
/// use mathhook_core::functions::special::lanczos_gamma;
///
/// let result = lanczos_gamma(5.0);
/// assert!((result - 24.0).abs() < 1e-10);
///
/// let half = lanczos_gamma(0.5);
/// let sqrt_pi = std::f64::consts::PI.sqrt();
/// assert!((half - sqrt_pi).abs() < 1e-14);
/// ```
pub fn lanczos_gamma(z: f64) -> f64 {
    if z.is_nan() || z.is_infinite() {
        return f64::NAN;
    }
    if z <= 0.0 && (z - z.round()).abs() < 1e-10 {
        return f64::INFINITY;
    }
    const LANCZOS_G: f64 = 7.0;
    const LANCZOS_COEFFS: [f64; 9] = [
        0.999_999_999_999_809_9,
        676.5203681218851,
        -1259.1392167224028,
        771.323_428_777_653_1,
        -176.615_029_162_140_6,
        12.507343278686905,
        -0.13857109526572012,
        9.984_369_578_019_572e-6,
        1.5056327351493116e-7,
    ];
    if z < 0.5 {
        PI / (f64::sin(PI * z) * lanczos_gamma(1.0 - z))
    } else {
        let z = z - 1.0;
        let mut x = LANCZOS_COEFFS[0];
        for (i, coef) in LANCZOS_COEFFS.iter().enumerate().skip(1) {
            x += coef / (z + i as f64);
        }
        let t = z + LANCZOS_G + 0.5;
        f64::sqrt(2.0 * PI) * f64::powf(t, z + 0.5) * f64::exp(-t) * x
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gamma_positive_integers() {
        assert_eq!(
            gamma(&Expression::Number(Number::Integer(1))),
            Expression::Number(Number::Integer(1))
        );
        assert_eq!(
            gamma(&Expression::Number(Number::Integer(2))),
            Expression::Number(Number::Integer(1))
        );
        assert_eq!(
            gamma(&Expression::Number(Number::Integer(3))),
            Expression::Number(Number::Integer(2))
        );
        assert_eq!(
            gamma(&Expression::Number(Number::Integer(4))),
            Expression::Number(Number::Integer(6))
        );
        assert_eq!(
            gamma(&Expression::Number(Number::Integer(5))),
            Expression::Number(Number::Integer(24))
        );
    }

    #[test]
    fn test_lanczos_gamma_numerical() {
        let result = lanczos_gamma(5.0);
        assert!((result - 24.0).abs() < 1e-10);
    }

    #[test]
    fn test_lanczos_gamma_accuracy() {
        let result_half = lanczos_gamma(0.5);
        let expected_half = std::f64::consts::PI.sqrt();
        assert!(
            (result_half - expected_half).abs() < 1e-14,
            "Γ(1/2) accuracy: expected {}, got {}",
            expected_half,
            result_half
        );
        let result_one = lanczos_gamma(1.0);
        assert!((result_one - 1.0).abs() < 1e-14, "Γ(1) = 1");
        let result_two = lanczos_gamma(2.0);
        assert!((result_two - 1.0).abs() < 1e-14, "Γ(2) = 1");
        let result_three = lanczos_gamma(3.0);
        assert!((result_three - 2.0).abs() < 1e-14, "Γ(3) = 2");
    }

    #[test]
    fn test_gamma_half_integer_symbolic() {
        let result = gamma(&Expression::Number(Number::Float(0.5)));
        let expected = Expression::sqrt(Expression::pi());
        assert_eq!(result, expected, "Γ(1/2) should be √π symbolically");
        let result_1_5 = gamma(&Expression::Number(Number::Float(1.5)));
        let sqrt_pi = Expression::sqrt(Expression::pi());
        let expected_1_5 = Expression::div(sqrt_pi, Expression::integer(2));
        assert_eq!(
            result_1_5, expected_1_5,
            "Γ(3/2) should be √π/2 symbolically"
        );
    }

    #[test]
    fn test_gamma_float_numerical() {
        let result = gamma(&Expression::Number(Number::Float(3.7)));
        match result {
            Expression::Number(Number::Float(_)) => {}
            _ => panic!("Γ(3.7) should return numerical Float"),
        }
    }

    #[test]
    fn test_lanczos_gamma_input_validation() {
        assert!(lanczos_gamma(f64::NAN).is_nan());
        assert!(lanczos_gamma(f64::INFINITY).is_nan());
        assert!(lanczos_gamma(0.0).is_infinite());
        assert!(lanczos_gamma(-1.0).is_infinite());
        assert!(lanczos_gamma(-2.0).is_infinite());
    }
}
