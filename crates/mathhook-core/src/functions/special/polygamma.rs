use super::{digamma, digamma_numerical};
use crate::{Expression, Number};

/// Polygamma function ψ^(n)(z)
///
/// The polygamma function is the (n+1)-th derivative of ln(Γ(z)):
/// ψ^(n)(z) = d^(n+1)/dz^(n+1) ln(Γ(z))
///
/// # Special Cases
///
/// - ψ^(0)(z) = ψ(z) (digamma)
/// - ψ^(1)(z) = trigamma
/// - ψ^(2)(z) = tetragamma
///
/// # Mathematical Properties
///
/// - ψ^(1)(1) = π²/6 (trigamma at 1)
/// - ψ^(n)(z+1) = ψ^(n)(z) + (-1)^n · n! / z^(n+1)
///
/// # Arguments
///
/// * `n` - Order of derivative (0 = digamma, 1 = trigamma, etc.)
/// * `z` - Argument
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
pub fn polygamma(n: i32, z: &Expression) -> Expression {
    if n == 0 {
        return digamma(z);
    }

    if n == 1 {
        match z {
            Expression::Number(Number::Integer(1)) => Expression::div(
                Expression::pow(Expression::pi(), Expression::integer(2)),
                Expression::integer(6),
            ),
            Expression::Number(Number::Float(x)) => {
                let result = polygamma_numerical(1, *x);
                Expression::Number(Number::Float(result))
            }
            _ => Expression::function(
                "polygamma",
                vec![Expression::Number(Number::Integer(n as i64)), z.clone()],
            ),
        }
    } else {
        match z {
            Expression::Number(Number::Float(x)) => {
                let result = polygamma_numerical(n, *x);
                Expression::Number(Number::Float(result))
            }
            _ => Expression::function(
                "polygamma",
                vec![Expression::Number(Number::Integer(n as i64)), z.clone()],
            ),
        }
    }
}

/// Numerically evaluates the polygamma function
///
/// Uses the series formula: ψ^(n)(z) = (-1)^(n+1) n! ∑_{k=0}^∞ 1/(z+k)^(n+1)
///
/// # Arguments
///
/// * `n` - Order of derivative
/// * `z` - Value to evaluate at
///
/// # Returns
///
/// ψ^(n)(z) value
fn polygamma_numerical(n: i32, z: f64) -> f64 {
    if n < 0 {
        return f64::NAN;
    }

    if n == 0 {
        return digamma_numerical(z);
    }

    if z.is_nan() || z.is_infinite() {
        return f64::NAN;
    }

    if z <= 0.0 && (z - z.round()).abs() < 1e-10 {
        return f64::NAN;
    }

    let sign = if (n + 1) % 2 == 0 { 1.0 } else { -1.0 };

    let mut sum = 0.0;
    for k in 0..1000 {
        let term = 1.0 / f64::powi(z + k as f64, n + 1);
        sum += term;
        if term.abs() < 1e-15 * sum.abs() {
            break;
        }
    }

    let mut factorial = 1.0;
    for i in 1..=n {
        factorial *= i as f64;
    }

    sign * factorial * sum
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Expression, Number};

    #[test]
    fn test_polygamma_zero_is_digamma() {
        let z = Expression::Number(Number::Integer(2));
        let poly_0 = polygamma(0, &z);
        let dig = digamma(&z);
        assert_eq!(poly_0, dig);
    }

    #[test]
    fn test_polygamma_trigamma_special_value() {
        let result = polygamma(1, &Expression::Number(Number::Integer(1)));
        let result_str = result.to_string();
        assert!(
            result_str.contains("Pi") && (result_str.contains('6') || result_str.contains("Pow")),
            "ψ^(1)(1) = π²/6"
        );
    }
}
