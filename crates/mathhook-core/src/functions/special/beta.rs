use super::{gamma, lanczos_gamma};
use crate::{Expression, Number};
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
/// use mathhook_core::functions::special::beta::beta_numerical;
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

#[cfg(test)]
mod tests {
    use super::*;

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
    fn test_beta_symmetry() {
        let result_ab = beta_numerical(2.5, 3.7);
        let result_ba = beta_numerical(3.7, 2.5);
        assert!(
            (result_ab - result_ba).abs() < 1e-14,
            "Beta symmetry: B(a,b) = B(b,a)"
        );
    }

    #[test]
    fn test_beta_numerical_evaluation() {
        let result = beta_numerical(2.0, 3.0);
        assert!((result - 1.0 / 12.0).abs() < 1e-14, "B(2,3) = 1/12");
        let result_2_5 = beta_numerical(2.0, 5.0);
        assert!((result_2_5 - 1.0 / 30.0).abs() < 1e-14, "B(2,5) = 1/30");
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
}
