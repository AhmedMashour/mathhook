pub mod data;
#[cfg(test)]
mod tests;

use crate::core::expression::Expression;
use crate::core::number::Number;
use crate::error::MathError;
use self::data::digamma_special_value;

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
/// use mathhook_core::core::expression::Expression;
/// use mathhook_core::core::functions::digamma::digamma;
///
/// let z = Expression::integer(1);
/// let result = digamma(&z).unwrap();
///
/// let err = digamma(&Expression::integer(0));
/// assert!(err.is_err());
/// ```
pub fn digamma(z: &Expression) -> Result<Expression, MathError> {
    if let Some(special) = digamma_special_value(z) {
        return Ok(special);
    }

    if let Expression::Number(Number::Integer(n)) = z {
        if *n <= 0 {
            return Err(MathError::Pole {
                function: "digamma".to_string(),
                at: z.clone(),
            });
        }
    }

    if let Expression::Number(Number::Float(x)) = z {
        if *x <= 0.0 && (*x - x.round()).abs() < 1e-10 {
            return Err(MathError::Pole {
                function: "digamma".to_string(),
                at: z.clone(),
            });
        }
    }

    Ok(Expression::function("digamma", vec![z.clone()]))
}
