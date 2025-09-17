//! Number theory function implementations

use crate::core::constants::EPSILON;
use crate::core::{Expression, Number};
use num_integer::Integer;

/// Least common multiple (LCM)
///
/// # Mathematical Definition
///
/// lcm(a, b) = |a × b| / gcd(a, b)
///
/// # Arguments
///
/// * `a` - First expression
/// * `b` - Second expression
///
/// # Returns
///
/// LCM expression
///
/// # Examples
///
/// ```
/// use mathhook_core::functions::number_theory_eval::lcm;
/// use mathhook_core::expr;
///
/// let result = lcm(&expr!(12), &expr!(18));
/// assert_eq!(result, expr!(36));
/// ```
pub fn lcm(a: &Expression, b: &Expression) -> Expression {
    match (a, b) {
        (
            Expression::Number(Number::Integer(a_val)),
            Expression::Number(Number::Integer(b_val)),
        ) => {
            if *a_val == 0 || *b_val == 0 {
                Expression::integer(0)
            } else {
                let gcd_val = a_val.gcd(b_val);
                Expression::integer((a_val * b_val).abs() / gcd_val)
            }
        }
        _ => Expression::function("lcm", vec![a.clone(), b.clone()]),
    }
}

/// Modulo operation
///
/// # Mathematical Definition
///
/// a mod b = remainder when a is divided by b
///
/// # Arguments
///
/// * `a` - Dividend expression
/// * `b` - Divisor expression
///
/// # Returns
///
/// Modulo expression
///
/// # Examples
///
/// ```
/// use mathhook_core::functions::number_theory_eval::modulo;
/// use mathhook_core::expr;
///
/// let result = modulo(&expr!(17), &expr!(5));
/// assert_eq!(result, expr!(2));
/// ```
pub fn modulo(a: &Expression, b: &Expression) -> Expression {
    match (a, b) {
        (
            Expression::Number(Number::Integer(a_val)),
            Expression::Number(Number::Integer(b_val)),
        ) if *b_val != 0 => Expression::integer(a_val % b_val),
        (Expression::Number(Number::Float(a_val)), Expression::Number(Number::Float(b_val)))
            if b_val.abs() >= EPSILON =>
        {
            Expression::float(a_val % b_val)
        }
        _ => Expression::function("mod", vec![a.clone(), b.clone()]),
    }
}

/// Primality test
///
/// # Mathematical Definition
///
/// isprime(n) = true if n is prime, false otherwise
///
/// # Arguments
///
/// * `arg` - Expression to test for primality
///
/// # Returns
///
/// Boolean expression (1 for true, 0 for false) or symbolic
///
/// # Examples
///
/// ```
/// use mathhook_core::functions::number_theory_eval::isprime;
/// use mathhook_core::expr;
///
/// assert_eq!(isprime(&expr!(2)), expr!(1));
/// assert_eq!(isprime(&expr!(17)), expr!(1));
/// assert_eq!(isprime(&expr!(4)), expr!(0));
/// ```
pub fn isprime(arg: &Expression) -> Expression {
    match arg {
        Expression::Number(Number::Integer(n)) if *n <= 1 => Expression::integer(0),
        Expression::Number(Number::Integer(2)) => Expression::integer(1),
        Expression::Number(Number::Integer(n)) if n % 2 == 0 => Expression::integer(0),
        Expression::Number(Number::Integer(n)) if *n > 0 => {
            Expression::integer(if is_prime_trial_division(*n) { 1 } else { 0 })
        }
        _ => Expression::function("isprime", vec![arg.clone()]),
    }
}

fn is_prime_trial_division(n: i64) -> bool {
    if n <= 1 {
        return false;
    }
    if n == 2 {
        return true;
    }
    if n % 2 == 0 {
        return false;
    }

    let sqrt_n = (n as f64).sqrt() as i64 + 1;
    for i in (3..=sqrt_n).step_by(2) {
        if n % i == 0 {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lcm() {
        assert_eq!(
            lcm(&Expression::integer(12), &Expression::integer(18)),
            Expression::integer(36)
        );
        assert_eq!(
            lcm(&Expression::integer(4), &Expression::integer(6)),
            Expression::integer(12)
        );
    }

    #[test]
    fn test_modulo() {
        assert_eq!(
            modulo(&Expression::integer(17), &Expression::integer(5)),
            Expression::integer(2)
        );
        assert_eq!(
            modulo(&Expression::integer(10), &Expression::integer(3)),
            Expression::integer(1)
        );
    }

    #[test]
    fn test_isprime() {
        assert_eq!(isprime(&Expression::integer(2)), Expression::integer(1));
        assert_eq!(isprime(&Expression::integer(17)), Expression::integer(1));
        assert_eq!(isprime(&Expression::integer(19)), Expression::integer(1));
        assert_eq!(isprime(&Expression::integer(4)), Expression::integer(0));
        assert_eq!(isprime(&Expression::integer(9)), Expression::integer(0));
        assert_eq!(isprime(&Expression::integer(1)), Expression::integer(0));
    }
}
