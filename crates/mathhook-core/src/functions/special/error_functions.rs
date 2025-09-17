//! Error function implementations

use crate::core::{Expression, MathConstant, Number};

/// Error function erf(x)
///
/// # Mathematical Definition
///
/// erf(x) = (2/√π) ∫₀ˣ e^(-t²) dt
///
/// # Arguments
///
/// * `arg` - Expression to compute error function of
///
/// # Returns
///
/// Error function expression
///
/// # Examples
///
/// ```
/// use mathhook_core::functions::special::error_functions::erf;
/// use mathhook_core::expr;
///
/// let result = erf(&expr!(0));
/// assert_eq!(result, expr!(0));
/// ```
pub fn erf(arg: &Expression) -> Expression {
    match arg {
        Expression::Number(Number::Integer(0)) => Expression::integer(0),
        Expression::Constant(MathConstant::Infinity) => Expression::integer(1),
        Expression::Constant(MathConstant::NegativeInfinity) => Expression::integer(-1),
        Expression::Number(Number::Float(f)) => Expression::float(erf_approx(*f)),
        Expression::Number(Number::Integer(i)) => Expression::float(erf_approx(*i as f64)),
        _ => Expression::function("erf", vec![arg.clone()]),
    }
}

/// Complementary error function erfc(x)
///
/// # Mathematical Definition
///
/// erfc(x) = 1 - erf(x) = (2/√π) ∫ₓ^∞ e^(-t²) dt
///
/// # Arguments
///
/// * `arg` - Expression to compute complementary error function of
///
/// # Returns
///
/// Complementary error function expression
///
/// # Examples
///
/// ```
/// use mathhook_core::functions::special::error_functions::erfc;
/// use mathhook_core::expr;
///
/// let result = erfc(&expr!(0));
/// assert_eq!(result, expr!(1));
/// ```
pub fn erfc(arg: &Expression) -> Expression {
    match arg {
        Expression::Number(Number::Integer(0)) => Expression::integer(1),
        Expression::Constant(MathConstant::Infinity) => Expression::integer(0),
        Expression::Constant(MathConstant::NegativeInfinity) => Expression::integer(2),
        Expression::Number(Number::Float(f)) => Expression::float(erfc_approx(*f)),
        Expression::Number(Number::Integer(i)) => Expression::float(erfc_approx(*i as f64)),
        _ => Expression::function("erfc", vec![arg.clone()]),
    }
}

fn erf_approx(x: f64) -> f64 {
    if x.is_infinite() {
        return if x.is_sign_positive() { 1.0 } else { -1.0 };
    }

    let a1 = 0.254829592;
    let a2 = -0.284496736;
    let a3 = 1.421413741;
    let a4 = -1.453152027;
    let a5 = 1.061405429;
    let p = 0.3275911;

    let sign = if x < 0.0 { -1.0 } else { 1.0 };
    let x = x.abs();

    let t = 1.0 / (1.0 + p * x);
    let y = 1.0 - (((((a5 * t + a4) * t) + a3) * t + a2) * t + a1) * t * (-x * x).exp();

    sign * y
}

fn erfc_approx(x: f64) -> f64 {
    1.0 - erf_approx(x)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_erf_zero() {
        assert_eq!(erf(&Expression::integer(0)), Expression::integer(0));
    }

    #[test]
    fn test_erfc_zero() {
        assert_eq!(erfc(&Expression::integer(0)), Expression::integer(1));
    }

    #[test]
    fn test_erf_infinity() {
        assert_eq!(
            erf(&Expression::constant(MathConstant::Infinity)),
            Expression::integer(1)
        );
    }

    #[test]
    fn test_erf_negative_infinity() {
        assert_eq!(
            erf(&Expression::constant(MathConstant::NegativeInfinity)),
            Expression::integer(-1)
        );
    }

    #[test]
    fn test_erfc_infinity() {
        assert_eq!(
            erfc(&Expression::constant(MathConstant::Infinity)),
            Expression::integer(0)
        );
    }
}
