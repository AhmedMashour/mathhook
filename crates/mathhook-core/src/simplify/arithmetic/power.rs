//! Power simplification operations

use super::multiplication::simplify_multiplication;
use super::Simplify;
use crate::core::commutativity::Commutativity;
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
            Expression::function("undefined".to_owned(), vec![])
        }
        // a^n = a^n for positive integers a and n (compute the power)
        (Expression::Number(Number::Integer(a)), Expression::Number(Number::Integer(n)))
            if *n > 0 && *a != 0 =>
        {
            // Use checked_pow to prevent overflow, promote to BigInt on overflow
            if let Some(result) = (*a).checked_pow(*n as u32) {
                Expression::integer(result)
            } else {
                // Overflow - use BigInt for arbitrary precision
                let base_big = BigInt::from(*a);
                let result_big = base_big.pow(*n as u32);
                Expression::Number(Number::rational(BigRational::new(
                    result_big,
                    BigInt::from(1),
                )))
            }
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
        // sqrt(x)^2 = x (inverse function)
        (Expression::Function { name, args }, Expression::Number(Number::Integer(2)))
            if name == "sqrt" && args.len() == 1 =>
        {
            args[0].clone()
        }
        // (a^b)^c = a^(b*c)
        (Expression::Pow(b, e), c) => {
            let new_exp = simplify_multiplication(&[(**e).clone(), c.clone()]);
            Expression::Pow(Box::new((**b).clone()), Box::new(new_exp))
        }
        // (a*b)^n = a^n * b^n ONLY if commutative
        (Expression::Mul(factors), Expression::Number(Number::Integer(n))) if *n > 0 => {
            let commutativity = Commutativity::combine(factors.iter().map(|f| f.commutativity()));

            if commutativity.can_sort() {
                // Safe to distribute - all factors commutative
                let powered_factors: Vec<Expression> = factors
                    .iter()
                    .map(|f| Expression::pow(f.clone(), simplified_exp.clone()))
                    .collect();
                simplify_multiplication(&powered_factors)
            } else {
                // Noncommutative - keep as (a*b)^n
                Expression::Pow(Box::new(simplified_base), Box::new(simplified_exp))
            }
        }
        _ => Expression::Pow(Box::new(simplified_base), Box::new(simplified_exp)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::simplify::Simplify;
    use crate::symbol;
    use crate::Expression;

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

    #[test]
    fn test_scalar_power_distributed() {
        let x = symbol!(x);
        let y = symbol!(y);
        let xy = Expression::mul(vec![
            Expression::symbol(x.clone()),
            Expression::symbol(y.clone()),
        ]);
        let expr = Expression::pow(xy, Expression::integer(2));

        let simplified = expr.simplify();

        match simplified {
            Expression::Mul(factors) => {
                assert_eq!(factors.len(), 2);
                let has_x_squared = factors.iter().any(|f| {
                    matches!(f, Expression::Pow(base, exp) if
                        **base == Expression::symbol(symbol!(x)) &&
                        **exp == Expression::integer(2))
                });
                let has_y_squared = factors.iter().any(|f| {
                    matches!(f, Expression::Pow(base, exp) if
                        **base == Expression::symbol(symbol!(y)) &&
                        **exp == Expression::integer(2))
                });
                assert!(has_x_squared, "Expected x^2 in factors");
                assert!(has_y_squared, "Expected y^2 in factors");
            }
            _ => panic!("Expected Mul, got {:?}", simplified),
        }
    }

    #[test]
    fn test_matrix_power_not_distributed() {
        let matrix_a = symbol!(A; matrix);
        let matrix_b = symbol!(B; matrix);
        let ab = Expression::mul(vec![
            Expression::symbol(matrix_a.clone()),
            Expression::symbol(matrix_b.clone()),
        ]);
        let expr = Expression::pow(ab.clone(), Expression::integer(2));

        let simplified = expr.simplify();

        match simplified {
            Expression::Pow(base, exp) => {
                assert_eq!(*exp, Expression::integer(2));
                match *base {
                    Expression::Mul(factors) => {
                        assert_eq!(factors.len(), 2);
                        assert!(factors.iter().all(|f| matches!(f, Expression::Symbol(s) if s.symbol_type() == crate::core::symbol::SymbolType::Matrix)));
                    }
                    _ => panic!("Expected Mul base, got {:?}", base),
                }
            }
            _ => panic!("Expected Pow, got {:?}", simplified),
        }
    }

    #[test]
    fn test_operator_power_not_distributed() {
        let matrix_p = symbol!(P; operator);
        let matrix_q = symbol!(Q; operator);
        let pq = Expression::mul(vec![
            Expression::symbol(matrix_p.clone()),
            Expression::symbol(matrix_q.clone()),
        ]);
        let expr = Expression::pow(pq, Expression::integer(2));

        let simplified = expr.simplify();

        match simplified {
            Expression::Pow(base, exp) => {
                assert_eq!(*exp, Expression::integer(2));
                match *base {
                    Expression::Mul(factors) => {
                        assert_eq!(factors.len(), 2);
                    }
                    _ => panic!("Expected Mul base, got {:?}", base),
                }
            }
            _ => panic!("Expected Pow, got {:?}", simplified),
        }
    }

    #[test]
    fn test_quaternion_power_not_distributed() {
        let i = symbol!(i; quaternion);
        let j = symbol!(j; quaternion);
        let ij = Expression::mul(vec![
            Expression::symbol(i.clone()),
            Expression::symbol(j.clone()),
        ]);
        let expr = Expression::pow(ij, Expression::integer(2));

        let simplified = expr.simplify();

        match simplified {
            Expression::Pow(_, exp) => {
                assert_eq!(*exp, Expression::integer(2));
            }
            _ => panic!("Expected Pow, got {:?}", simplified),
        }
    }

    #[test]
    fn test_three_scalar_factors_power_distributed() {
        let x = symbol!(x);
        let y = symbol!(y);
        let z = symbol!(z);
        let xyz = Expression::mul(vec![
            Expression::symbol(x.clone()),
            Expression::symbol(y.clone()),
            Expression::symbol(z.clone()),
        ]);
        let expr = Expression::pow(xyz, Expression::integer(3));

        let simplified = expr.simplify();

        match simplified {
            Expression::Mul(factors) => {
                assert_eq!(factors.len(), 3);
            }
            _ => panic!("Expected Mul, got {:?}", simplified),
        }
    }

    #[test]
    fn test_mixed_scalar_matrix_power_not_distributed() {
        let x = symbol!(x);
        let matrix_a = symbol!(A; matrix);
        let xa = Expression::mul(vec![
            Expression::symbol(x.clone()),
            Expression::symbol(matrix_a.clone()),
        ]);
        let expr = Expression::pow(xa, Expression::integer(2));

        let simplified = expr.simplify();

        match simplified {
            Expression::Pow(_, exp) => {
                assert_eq!(*exp, Expression::integer(2));
            }
            _ => panic!("Expected Pow, got {:?}", simplified),
        }
    }

    #[test]
    fn test_numeric_power_distributed() {
        let expr = Expression::pow(
            Expression::mul(vec![Expression::integer(2), Expression::integer(3)]),
            Expression::integer(2),
        );

        let simplified = expr.simplify();

        assert_eq!(simplified, Expression::integer(36));
    }
}
