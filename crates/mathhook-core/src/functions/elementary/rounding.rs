//! Rounding function implementations

use crate::core::{Expression, Number};
use num_traits::Signed;
use num_traits::ToPrimitive;

/// Sign function
///
/// # Mathematical Definition
///
/// sign(x) = { -1  if x < 0
///           {  0  if x = 0
///           {  1  if x > 0
///
/// # Arguments
///
/// * `arg` - Expression to compute sign of
///
/// # Returns
///
/// Sign expression (-1, 0, or 1)
///
/// # Examples
///
/// ```
/// use mathhook_core::functions::elementary::rounding::sign;
/// use mathhook_core::{expr, Expression};
///
/// assert_eq!(sign(&expr!(-5)), expr!(-1));
/// assert_eq!(sign(&expr!(0)), expr!(0));
/// assert_eq!(sign(&expr!(5)), expr!(1));
/// ```
pub fn sign(arg: &Expression) -> Expression {
    match arg {
        Expression::Number(n) => evaluate_sign_number(n),
        _ => Expression::function("sign", vec![arg.clone()]),
    }
}

fn evaluate_sign_number(n: &Number) -> Expression {
    match n {
        Number::Integer(i) => Expression::integer(i.signum()),
        Number::Float(f) => {
            if *f > 0.0 {
                Expression::integer(1)
            } else if *f < 0.0 {
                Expression::integer(-1)
            } else {
                Expression::integer(0)
            }
        }
        Number::BigInteger(bi) => {
            use num_bigint::Sign;
            match bi.sign() {
                Sign::Plus => Expression::integer(1),
                Sign::Minus => Expression::integer(-1),
                Sign::NoSign => Expression::integer(0),
            }
        }
        Number::Rational(r) => {
            if r.is_positive() {
                Expression::integer(1)
            } else if r.is_negative() {
                Expression::integer(-1)
            } else {
                Expression::integer(0)
            }
        }
    }
}

/// Floor function (round down to nearest integer)
///
/// # Mathematical Definition
///
/// floor(x) = ⌊x⌋ = greatest integer ≤ x
///
/// # Arguments
///
/// * `arg` - Expression to floor
///
/// # Returns
///
/// Floor expression
///
/// # Examples
///
/// ```
/// use mathhook_core::functions::elementary::rounding::floor;
/// use mathhook_core::{expr, Expression};
///
/// assert_eq!(floor(&Expression::float(3.7)), Expression::integer(3));
/// assert_eq!(floor(&Expression::float(-2.3)), Expression::integer(-3));
/// ```
pub fn floor(arg: &Expression) -> Expression {
    match arg {
        Expression::Number(Number::Integer(i)) => Expression::integer(*i),
        Expression::Number(Number::Float(f)) => Expression::integer(f.floor() as i64),
        Expression::Number(Number::BigInteger(_)) => arg.clone(),
        Expression::Number(Number::Rational(r)) => {
            Expression::integer(r.to_f64().unwrap_or(0.0).floor() as i64)
        }
        _ => Expression::function("floor", vec![arg.clone()]),
    }
}

/// Ceiling function (round up to nearest integer)
///
/// # Mathematical Definition
///
/// ceil(x) = ⌈x⌉ = smallest integer ≥ x
///
/// # Arguments
///
/// * `arg` - Expression to ceil
///
/// # Returns
///
/// Ceiling expression
///
/// # Examples
///
/// ```
/// use mathhook_core::functions::elementary::rounding::ceil;
/// use mathhook_core::{expr, Expression};
///
/// assert_eq!(ceil(&Expression::float(3.2)), Expression::integer(4));
/// assert_eq!(ceil(&Expression::float(-2.7)), Expression::integer(-2));
/// ```
pub fn ceil(arg: &Expression) -> Expression {
    match arg {
        Expression::Number(Number::Integer(i)) => Expression::integer(*i),
        Expression::Number(Number::Float(f)) => Expression::integer(f.ceil() as i64),
        Expression::Number(Number::BigInteger(_)) => arg.clone(),
        Expression::Number(Number::Rational(r)) => {
            Expression::integer(r.to_f64().unwrap_or(0.0).ceil() as i64)
        }
        _ => Expression::function("ceil", vec![arg.clone()]),
    }
}

/// Round function (round to nearest integer)
///
/// # Mathematical Definition
///
/// round(x) rounds to nearest integer, with ties rounding away from zero
///
/// # Arguments
///
/// * `arg` - Expression to round
///
/// # Returns
///
/// Rounded expression
///
/// # Examples
///
/// ```
/// use mathhook_core::functions::elementary::rounding::round;
/// use mathhook_core::{expr, Expression};
///
/// assert_eq!(round(&Expression::float(3.4)), Expression::integer(3));
/// assert_eq!(round(&Expression::float(3.6)), Expression::integer(4));
/// assert_eq!(round(&Expression::float(3.5)), Expression::integer(4));
/// ```
pub fn round(arg: &Expression) -> Expression {
    match arg {
        Expression::Number(Number::Integer(i)) => Expression::integer(*i),
        Expression::Number(Number::Float(f)) => Expression::integer(f.round() as i64),
        Expression::Number(Number::BigInteger(_)) => arg.clone(),
        Expression::Number(Number::Rational(r)) => {
            Expression::integer(r.to_f64().unwrap_or(0.0).round() as i64)
        }
        _ => Expression::function("round", vec![arg.clone()]),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sign() {
        assert_eq!(sign(&Expression::integer(-5)), Expression::integer(-1));
        assert_eq!(sign(&Expression::integer(0)), Expression::integer(0));
        assert_eq!(sign(&Expression::integer(5)), Expression::integer(1));
    }

    #[test]
    fn test_floor() {
        assert_eq!(floor(&Expression::float(3.7)), Expression::integer(3));
        assert_eq!(floor(&Expression::float(-2.3)), Expression::integer(-3));
        assert_eq!(floor(&Expression::integer(5)), Expression::integer(5));
    }

    #[test]
    fn test_ceil() {
        assert_eq!(ceil(&Expression::float(3.2)), Expression::integer(4));
        assert_eq!(ceil(&Expression::float(-2.7)), Expression::integer(-2));
        assert_eq!(ceil(&Expression::integer(5)), Expression::integer(5));
    }

    #[test]
    fn test_round() {
        assert_eq!(round(&Expression::float(3.4)), Expression::integer(3));
        assert_eq!(round(&Expression::float(3.6)), Expression::integer(4));
        assert_eq!(round(&Expression::float(3.5)), Expression::integer(4));
        assert_eq!(round(&Expression::integer(5)), Expression::integer(5));
    }
}
