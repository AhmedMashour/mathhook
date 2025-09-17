use crate::{Expression, MathConstant, Number};

/// Digamma function ψ(z) = Γ'(z)/Γ(z)
///
/// The digamma function is the logarithmic derivative of the Gamma function:
/// ψ(z) = d/dz ln(Γ(z))
///
/// # Mathematical Properties
///
/// - ψ(1) = -γ (Euler-Mascheroni constant ≈ -0.5772156649)
/// - ψ(n+1) = ψ(n) + 1/n for positive integers n
/// - ψ(z+1) = ψ(z) + 1/z (recurrence relation)
/// - ψ(1/2) = -γ - ln(4)
///
/// # Implementation
///
/// Uses reflection formula for z < 0.5 and series expansion for z ≥ 0.5.
/// Special values are computed exactly when possible.
///
/// # Arguments
///
/// * `z` - Expression to evaluate digamma function at
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
    match z {
        Expression::Number(Number::Integer(1)) => Expression::mul(vec![
            Expression::integer(-1),
            Expression::constant(MathConstant::EulerGamma),
        ]),
        Expression::Number(Number::Integer(n)) if *n > 1 => {
            let mut sum = Expression::mul(vec![
                Expression::integer(-1),
                Expression::constant(MathConstant::EulerGamma),
            ]);
            for k in 1..*n {
                sum = Expression::add(vec![sum, Expression::rational(1, k)]);
            }
            sum
        }
        Expression::Number(Number::Float(x)) => {
            let result = digamma_numerical(*x);
            Expression::Number(Number::Float(result))
        }
        _ => Expression::function("digamma", vec![z.clone()]),
    }
}

/// Numerically evaluates the digamma function using series expansion
///
/// Uses reflection formula for z < 0.5 and asymptotic series for z ≥ 0.5.
///
/// # Arguments
///
/// * `z` - Value to evaluate digamma at
///
/// # Returns
///
/// ψ(z) value
pub fn digamma_numerical(mut z: f64) -> f64 {
    if z.is_nan() || z.is_infinite() {
        return f64::NAN;
    }

    if z <= 0.0 && (z - z.round()).abs() < 1e-10 {
        return f64::NAN;
    }

    let mut result = 0.0;
    while z < 10.0 {
        result -= 1.0 / z;
        z += 1.0;
    }

    let z_inv = 1.0 / z;
    let z2_inv = z_inv * z_inv;

    result += f64::ln(z) - 0.5 * z_inv;
    result -= z2_inv
        * (1.0 / 12.0 - z2_inv * (1.0 / 120.0 - z2_inv * (1.0 / 252.0 - z2_inv * (1.0 / 240.0))));

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_digamma_special_values() {
        let result = digamma(&Expression::Number(Number::Integer(1)));
        match result {
            Expression::Mul(ref factors) if factors.len() == 2 => {
                assert!(matches!(
                    factors[0],
                    Expression::Number(Number::Integer(-1))
                ));
                assert!(matches!(
                    factors[1],
                    Expression::Constant(MathConstant::EulerGamma)
                ));
            }
            _ => panic!("ψ(1) should be -γ"),
        }
    }

    #[test]
    fn test_digamma_numerical() {
        let result = digamma(&Expression::Number(Number::Float(1.0)));
        match result {
            Expression::Number(Number::Float(val)) => {
                const EULER_GAMMA: f64 = 0.577_215_664_901_532_9;
                assert!((val + EULER_GAMMA).abs() < 1e-10, "ψ(1) ≈ -γ, got {}", val);
            }
            _ => panic!("ψ(1.0) should return numerical float"),
        }
    }

    #[test]
    fn test_digamma_recurrence() {
        const EULER_GAMMA: f64 = 0.577_215_664_901_532_9;
        let psi_2 = digamma_numerical(2.0);
        let expected = -EULER_GAMMA + 1.0;
        assert!(
            (psi_2 - expected).abs() < 1e-10,
            "ψ(2) = ψ(1) + 1, got {}",
            psi_2
        );
    }

    #[test]
    fn test_digamma_integer_formula() {
        let result = digamma(&Expression::Number(Number::Integer(3)));
        let simplified = result.to_string();
        assert!(simplified.contains("EulerGamma"));
    }
}
