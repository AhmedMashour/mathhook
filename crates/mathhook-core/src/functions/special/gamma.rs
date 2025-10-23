//! Gamma and Beta special functions with high-precision numerical evaluation.
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

/// Beta function B(a, b)
///
/// The Beta function is defined as:
/// B(a, b) = Γ(a)·Γ(b) / Γ(a+b)
///
/// # Mathematical Properties
///
/// - B(a, b) = B(b, a) (symmetric)
/// - B(a, b) = ∫₀¹ t^(a-1)·(1-t)^(b-1) dt
///
/// # Numerical Evaluation
///
/// Float inputs are evaluated numerically using Lanczos gamma approximation.
/// Mixed Float/Integer inputs are converted to numerical evaluation.
///
/// # Arguments
///
/// * `a` - First parameter
/// * `b` - Second parameter
///
/// # Examples
///
/// ```rust
/// use mathhook_core::{Expression, Number};
/// use mathhook_core::functions::special::beta;
///
/// let a = Expression::Number(Number::Integer(2));
/// let b = Expression::Number(Number::Integer(3));
/// let result = beta(&a, &b);
///
/// let a_float = Expression::Number(Number::Float(2.5));
/// let b_float = Expression::Number(Number::Float(3.7));
/// let result_num = beta(&a_float, &b_float);
/// ```
pub fn beta(a: &Expression, b: &Expression) -> Expression {
    match (a, b) {
        (Expression::Number(Number::Float(x)), Expression::Number(Number::Float(y))) => {
            let result = beta_numerical(*x, *y);
            Expression::Number(Number::Float(result))
        }
        (Expression::Number(Number::Float(x)), Expression::Number(Number::Integer(n))) => {
            let result = beta_numerical(*x, *n as f64);
            Expression::Number(Number::Float(result))
        }
        (Expression::Number(Number::Integer(n)), Expression::Number(Number::Float(y))) => {
            let result = beta_numerical(*n as f64, *y);
            Expression::Number(Number::Float(result))
        }
        _ => {
            let gamma_a = gamma(a);
            let gamma_b = gamma(b);
            let sum = Expression::add(vec![a.clone(), b.clone()]);
            let gamma_sum = gamma(&sum);
            Expression::mul(vec![
                gamma_a,
                gamma_b,
                Expression::pow(gamma_sum, Expression::Number(Number::Integer(-1))),
            ])
        }
    }
}

/// Numerically evaluates the beta function B(a, b) = Γ(a)·Γ(b)/Γ(a+b)
///
/// Uses Lanczos gamma approximation for high accuracy.
///
/// # Arguments
///
/// * `a` - First parameter
/// * `b` - Second parameter
///
/// # Examples
///
/// ```rust
/// use mathhook_core::functions::special::beta_numerical;
///
/// let result = beta_numerical(2.0, 3.0);
/// assert!((result - 1.0/12.0).abs() < 1e-14);
/// ```
pub fn beta_numerical(a: f64, b: f64) -> f64 {
    if a.is_nan() || b.is_nan() || a.is_infinite() || b.is_infinite() {
        return f64::NAN;
    }
    let gamma_a = lanczos_gamma(a);
    let gamma_b = lanczos_gamma(b);
    let gamma_ab = lanczos_gamma(a + b);
    (gamma_a * gamma_b) / gamma_ab
}

/// Digamma function ψ(z) = Γ'(z)/Γ(z)
///
/// The digamma function is the logarithmic derivative of the Gamma function.
///
/// # Mathematical Properties
///
/// - ψ(1) = -γ (Euler-Mascheroni constant)
/// - ψ(n+1) = ψ(n) + 1/n for n > 0
/// - ψ(z+1) = ψ(z) + 1/z
///
/// # Examples
///
/// ```rust
/// use mathhook_core::{Expression, Number};
/// use mathhook_core::functions::special::digamma;
///
/// let z = Expression::Number(Number::Integer(1));
/// let result = digamma(&z);
/// ```
pub fn digamma(z: &Expression) -> Expression {
    Expression::function("digamma", vec![z.clone()])
}

/// Polygamma function ψ^(n)(z)
///
/// The polygamma function is the (n+1)-th derivative of ln(Γ(z)):
/// ψ^(n)(z) = d^(n+1)/dz^(n+1) ln(Γ(z))
///
/// Special cases:
/// - ψ^(0)(z) = ψ(z) (digamma)
/// - ψ^(1)(z) = trigamma
/// - ψ^(2)(z) = tetragamma
///
/// # Examples
///
/// ```rust
/// use mathhook_core::{Expression, Number};
/// use mathhook_core::functions::special::polygamma;
///
/// let result = polygamma(0, &Expression::Number(Number::Integer(1)));
/// let trigamma = polygamma(1, &Expression::Number(Number::Integer(1)));
/// ```
pub fn polygamma(n: u32, z: &Expression) -> Expression {
    if n == 0 {
        digamma(z)
    } else {
        Expression::function(
            "polygamma",
            vec![Expression::Number(Number::Integer(n as i64)), z.clone()],
        )
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
        0.99999999999980993,
        676.5203681218851,
        -1259.1392167224028,
        771.32342877765313,
        -176.61502916214059,
        12.507343278686905,
        -0.13857109526572012,
        9.9843695780195716e-6,
        1.5056327351493116e-7,
    ];
    if z < 0.5 {
        PI / (f64::sin(PI * z) * lanczos_gamma(1.0 - z))
    } else {
        let z = z - 1.0;
        let mut x = LANCZOS_COEFFS[0];
        for i in 1..9 {
            x += LANCZOS_COEFFS[i] / (z + i as f64);
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
        assert_eq!(gamma(&Expression::Number(Number::Integer(1))),
                   Expression::Number(Number::Integer(1)));
        assert_eq!(gamma(&Expression::Number(Number::Integer(2))),
                   Expression::Number(Number::Integer(1)));
        assert_eq!(gamma(&Expression::Number(Number::Integer(3))),
                   Expression::Number(Number::Integer(2)));
        assert_eq!(gamma(&Expression::Number(Number::Integer(4))),
                   Expression::Number(Number::Integer(6)));
        assert_eq!(gamma(&Expression::Number(Number::Integer(5))),
                   Expression::Number(Number::Integer(24)));
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
        assert!((result_half - expected_half).abs() < 1e-14,
                "Γ(1/2) accuracy: expected {}, got {}", expected_half, result_half);
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
        assert_eq!(result_1_5, expected_1_5, "Γ(3/2) should be √π/2 symbolically");
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
    fn test_beta_symmetry() {
        let result_ab = beta_numerical(2.5, 3.7);
        let result_ba = beta_numerical(3.7, 2.5);
        assert!((result_ab - result_ba).abs() < 1e-14, "Beta symmetry: B(a,b) = B(b,a)");
    }

    #[test]
    fn test_beta_numerical_evaluation() {
        let result = beta_numerical(2.0, 3.0);
        assert!((result - 1.0/12.0).abs() < 1e-14, "B(2,3) = 1/12");
        let result_2_5 = beta_numerical(2.0, 5.0);
        assert!((result_2_5 - 1.0/30.0).abs() < 1e-14, "B(2,5) = 1/30");
    }

    #[test]
    fn test_beta_float_evaluation() {
        let a = Expression::Number(Number::Float(2.5));
        let b = Expression::Number(Number::Float(3.7));
        let result = beta(&a, &b);
        match result {
            Expression::Number(Number::Float(_)) => {}
            _ => panic!("Beta with float inputs should return numerical result"),
        }
    }

    #[test]
    fn test_beta_mixed_evaluation() {
        let a = Expression::Number(Number::Float(2.5));
        let b = Expression::Number(Number::Integer(3));
        let result = beta(&a, &b);
        match result {
            Expression::Number(Number::Float(_)) => {}
            _ => panic!("Beta with mixed inputs should return numerical result"),
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

    #[test]
    fn test_polygamma_zero_is_digamma() {
        let z = Expression::Number(Number::Integer(2));
        let poly_0 = polygamma(0, &z);
        let dig = digamma(&z);
        assert_eq!(poly_0, dig);
    }
}
