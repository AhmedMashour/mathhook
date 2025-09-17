//! Factorial function implementation

use crate::core::{Expression, Number};
use num_bigint::BigInt;
use num_traits::One;

/// Factorial function n!
///
/// # Mathematical Definition
///
/// n! = n × (n-1) × (n-2) × ... × 2 × 1
/// 0! = 1 by convention
///
/// # Arguments
///
/// * `arg` - Expression to compute factorial of
///
/// # Returns
///
/// Factorial expression
///
/// # Examples
///
/// ```
/// use mathhook_core::functions::special::factorial::factorial;
/// use mathhook_core::{expr, Expression};
///
/// // Factorial returns BigInteger, so we check it's a number
/// let result = factorial(&Expression::integer(0));
/// assert!(matches!(result, Expression::Number(_)));
///
/// let result2 = factorial(&Expression::integer(5));
/// assert!(matches!(result2, Expression::Number(_)));
/// ```
pub fn factorial(arg: &Expression) -> Expression {
    match arg {
        Expression::Number(Number::Integer(n)) if *n >= 0 => {
            Expression::Number(compute_factorial(*n))
        }
        Expression::Number(Number::Integer(n)) if *n < 0 => {
            Expression::function("factorial", vec![arg.clone()])
        }
        _ => Expression::function("factorial", vec![arg.clone()]),
    }
}

/// Compute factorial, returning Integer if it fits in i64, BigInteger otherwise
fn compute_factorial(n: i64) -> Number {
    if n == 0 || n == 1 {
        return Number::Integer(1);
    }

    // Try to compute as i64 first (checked arithmetic)
    let mut result: i64 = 1;
    for i in 2..=n {
        match result.checked_mul(i) {
            Some(new_result) => result = new_result,
            None => {
                // Overflow - fall back to BigInt computation
                return Number::BigInteger(Box::new(compute_factorial_bigint(n)));
            }
        }
    }
    Number::Integer(result)
}

/// Compute factorial as BigInt (for large values)
fn compute_factorial_bigint(n: i64) -> BigInt {
    if n == 0 || n == 1 {
        return BigInt::one();
    }

    let mut result = BigInt::one();
    for i in 2..=n {
        result *= BigInt::from(i);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_factorial_zero() {
        let result = factorial(&Expression::integer(0));
        assert_eq!(result, Expression::Number(Number::Integer(1)));
    }

    #[test]
    fn test_factorial_one() {
        let result = factorial(&Expression::integer(1));
        assert_eq!(result, Expression::Number(Number::Integer(1)));
    }

    #[test]
    fn test_factorial_five() {
        let result = factorial(&Expression::integer(5));
        assert_eq!(result, Expression::Number(Number::Integer(120)));
    }

    #[test]
    fn test_factorial_ten() {
        let result = factorial(&Expression::integer(10));
        assert_eq!(result, Expression::Number(Number::Integer(3628800)));
    }

    #[test]
    fn test_factorial_large() {
        // 21! doesn't fit in i64, should return BigInteger
        let result = factorial(&Expression::integer(21));
        assert!(matches!(result, Expression::Number(Number::BigInteger(_))));

        if let Expression::Number(Number::BigInteger(bi)) = result {
            // 21! = 51090942171709440000
            let expected: BigInt = "51090942171709440000".parse().unwrap();
            assert_eq!(*bi, expected);
        }
    }
}
