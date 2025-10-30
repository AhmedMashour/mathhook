pub mod data;
#[cfg(test)]
mod tests;

use crate::core::expression::Expression;
use crate::core::number::Number;
use crate::error::MathError;
use crate::core::functions::gamma::lanczos_gamma;
use self::data::beta_special_value;

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
/// use mathhook_core::core::expression::Expression;
/// use mathhook_core::core::functions::beta::beta;
///
/// let a = Expression::integer(2);
/// let b = Expression::integer(3);
/// let result = beta(&a, &b).unwrap();
///
/// let a_float = Expression::float(2.5);
/// let b_float = Expression::float(3.7);
/// let result_num = beta(&a_float, &b_float).unwrap();
///
/// let err = beta(&Expression::integer(0), &Expression::integer(1));
/// assert!(err.is_err());
/// ```
pub fn beta(a: &Expression, b: &Expression) -> Result<Expression, MathError> {
    if let Some(special) = beta_special_value(a, b) {
        return Ok(special);
    }

    match (a, b) {
        (Expression::Number(Number::Float(x)), Expression::Number(Number::Float(y))) => {
            if *x <= 0.0 || *y <= 0.0 {
                return Err(MathError::DomainError {
                    operation: "beta".to_string(),
                    value: Expression::function("beta", vec![a.clone(), b.clone()]),
                    reason: "Beta function requires positive arguments".to_string(),
                });
            }
            let result = beta_numerical(*x, *y);
            Ok(Expression::float(result))
        }
        (Expression::Number(Number::Float(x)), Expression::Number(Number::Integer(n))) => {
            if *x <= 0.0 || *n <= 0 {
                return Err(MathError::DomainError {
                    operation: "beta".to_string(),
                    value: Expression::function("beta", vec![a.clone(), b.clone()]),
                    reason: "Beta function requires positive arguments".to_string(),
                });
            }
            let result = beta_numerical(*x, *n as f64);
            Ok(Expression::float(result))
        }
        (Expression::Number(Number::Integer(n)), Expression::Number(Number::Float(y))) => {
            if *n <= 0 || *y <= 0.0 {
                return Err(MathError::DomainError {
                    operation: "beta".to_string(),
                    value: Expression::function("beta", vec![a.clone(), b.clone()]),
                    reason: "Beta function requires positive arguments".to_string(),
                });
            }
            let result = beta_numerical(*n as f64, *y);
            Ok(Expression::float(result))
        }
        _ => {
            use crate::core::functions::gamma::gamma;
            let gamma_a = gamma(a)?;
            let gamma_b = gamma(b)?;
            let sum = Expression::add(vec![a.clone(), b.clone()]);
            let gamma_sum = gamma(&sum)?;
            Ok(Expression::mul(vec![
                gamma_a,
                gamma_b,
                Expression::pow(gamma_sum, Expression::integer(-1)),
            ]))
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
/// use mathhook_core::core::functions::beta::beta_numerical;
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
