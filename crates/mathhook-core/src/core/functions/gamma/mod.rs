pub mod data;
#[cfg(test)]
mod tests;

use crate::core::expression::Expression;
use crate::core::number::Number;
use crate::error::MathError;
use self::data::gamma_special_value;
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
/// use mathhook_core::core::expression::Expression;
/// use mathhook_core::core::functions::gamma::gamma;
///
/// let result = gamma(&Expression::integer(5)).unwrap();
/// assert_eq!(result, Expression::integer(24));
///
/// let half = gamma(&Expression::rational(1, 2)).unwrap();
/// assert_eq!(half, Expression::sqrt(Expression::pi()));
///
/// let err = gamma(&Expression::integer(0));
/// assert!(err.is_err());
/// ```
pub fn gamma(z: &Expression) -> Result<Expression, MathError> {
    if let Some(special) = gamma_special_value(z) {
        return Ok(special);
    }

    if let Expression::Number(Number::Integer(n)) = z {
        if *n <= 0 {
            return Err(MathError::Pole {
                function: "gamma".to_string(),
                at: z.clone(),
            });
        }
    }

    if let Expression::Number(Number::Float(x)) = z {
        if *x <= 0.0 && (*x - x.round()).abs() < 1e-10 {
            return Err(MathError::Pole {
                function: "gamma".to_string(),
                at: z.clone(),
            });
        }
        let result = lanczos_gamma(*x);
        return Ok(Expression::float(result));
    }

    Ok(Expression::function("gamma", vec![z.clone()]))
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
/// use mathhook_core::core::functions::gamma::lanczos_gamma;
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
