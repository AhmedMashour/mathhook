//! Power simplification operations

use super::multiplication::simplify_multiplication;
use super::Simplify;
use crate::core::{Expression, Number};
use num_bigint::BigInt;
use num_rational::BigRational;

/// Power simplification
pub fn simplify_power(base: &Expression, exp: &Expression) -> Expression {
    // First, simplify both base and exponent for better pattern matching
    let simplified_base = base.simplify();
    let simplified_exp = exp.simplify();

    match (&simplified_base, &simplified_exp) {
        // x^0 = 1
        (_, Expression::Number(Number::Integer(0))) => Expression::integer(1),
        // x^1 = x (use already simplified base)
        (_, Expression::Number(Number::Integer(1))) => simplified_base,
        // 1^x = 1
        (Expression::Number(Number::Integer(1)), _) => Expression::integer(1),
        // 0^x = 0 (for x > 0)
        (Expression::Number(Number::Integer(0)), Expression::Number(Number::Integer(n)))
            if *n > 0 =>
        {
            Expression::integer(0)
        }
        // 0^(-1) = undefined (division by zero)
        (Expression::Number(Number::Integer(0)), Expression::Number(Number::Integer(-1))) => {
            Expression::function("undefined".to_string(), vec![])
        }
        // a^n = a^n for positive integers a and n (compute the power)
        (Expression::Number(Number::Integer(a)), Expression::Number(Number::Integer(n)))
            if *n > 0 && *a != 0 =>
        {
            let result = (*a as i64).pow(*n as u32);
            Expression::integer(result)
        }
        // a^(-1) = 1/a (convert to rational for integers)
        (Expression::Number(Number::Integer(a)), Expression::Number(Number::Integer(-1)))
            if *a != 0 =>
        {
            Expression::Number(Number::rational(BigRational::new(
                BigInt::from(1),
                BigInt::from(*a),
            )))
        }
        // (a/b)^(-1) = b/a (reciprocal of rational)
        (Expression::Number(Number::Rational(r)), Expression::Number(Number::Integer(-1))) => {
            Expression::Number(Number::rational(BigRational::new(
                r.denom().clone(),
                r.numer().clone(),
            )))
        }
        // (a/b)^n = a^n/b^n for positive integers n
        (Expression::Number(Number::Rational(r)), Expression::Number(Number::Integer(n)))
            if *n > 0 =>
        {
            let exp = *n as u32;
            let numerator = r.numer().pow(exp);
            let denominator = r.denom().pow(exp);
            Expression::Number(Number::rational(BigRational::new(numerator, denominator)))
        }
        // a^(-n) = 1/(a^n) for positive integers a and n
        (Expression::Number(Number::Integer(a)), Expression::Number(Number::Integer(n)))
            if *n < 0 && *a != 0 =>
        {
            let positive_exp = (-n) as u32;
            let numerator = BigInt::from(1);
            let denominator = BigInt::from(*a).pow(positive_exp);
            Expression::Number(Number::rational(BigRational::new(numerator, denominator)))
        }
        // (a^b)^c = a^(b*c)
        (Expression::Pow(b, e), c) => {
            let new_exp = simplify_multiplication(&[(**e).clone(), c.clone()]);
            Expression::Pow(Box::new((**b).clone()), Box::new(new_exp))
        }
        _ => Expression::Pow(Box::new(simplified_base), Box::new(simplified_exp)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{symbol, Expression};

    #[test]
    fn test_power_simplification() {
        let x = symbol!(x);

        // x^0 = 1
        let expr = simplify_power(&Expression::symbol(x.clone()), &Expression::integer(0));
        assert_eq!(expr, Expression::integer(1));

        // x^1 = x
        let expr = simplify_power(&Expression::symbol(x.clone()), &Expression::integer(1));
        assert_eq!(expr, Expression::symbol(x));
    }
}
