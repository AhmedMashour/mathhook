//! Multiplication simplification operations

mod binary_numeric;
mod power_combining;

pub use binary_numeric::try_simplify_binary;
pub use power_combining::combine_like_powers;

use super::addition::simplify_addition;
use super::helpers::expression_order;
use super::power::simplify_power;
use super::Simplify;
use crate::core::commutativity::Commutativity;
use crate::core::constants::EPSILON;
use crate::core::{Expression, Number};
use num_bigint::BigInt;
use num_rational::BigRational;
use num_traits::{One, ToPrimitive, Zero};

/// Simplify multiplication with minimal overhead and flattening
pub fn simplify_multiplication(factors: &[Expression]) -> Expression {
    if factors.is_empty() {
        return Expression::integer(1);
    }
    if factors.len() == 1 {
        return factors[0].clone();
    }

    let mut flattened_factors = Vec::new();
    let mut to_process: Vec<&Expression> = factors.iter().collect();

    while !to_process.is_empty() {
        let factor = to_process.remove(0);
        match factor {
            Expression::Mul(nested_factors) => {
                for (i, nested) in nested_factors.iter().enumerate() {
                    to_process.insert(i, nested);
                }
            }
            _ => {
                let simplified = match factor {
                    Expression::Add(terms) => simplify_addition(terms),
                    Expression::Pow(base, exp) => simplify_power(base, exp),
                    _ => factor.simplify(),
                };
                flattened_factors.push(simplified);
            }
        }
    }

    let factors = &flattened_factors;

    if factors.len() == 2 {
        if let Some(result) = try_simplify_binary(&factors[0], &factors[1]) {
            return result;
        }

        // Matrix fast-path: try direct matrix multiplication
        // Note: During simplification, we only apply if it succeeds.
        // Dimension errors will be caught during evaluation, not simplification.
        if let Some(Ok(result)) = super::matrix_ops::try_matrix_multiply(&factors[0], &factors[1]) {
            return result;
        }

        // Handle Add simplification special cases
        match (&factors[0], &factors[1]) {
            (a, Expression::Add(terms)) => {
                let simplified_add = simplify_addition(terms);
                if !matches!(simplified_add, Expression::Add(_)) {
                    return simplify_multiplication(&[a.clone(), simplified_add]);
                }
            }
            (Expression::Add(terms), b) => {
                let simplified_add = simplify_addition(terms);
                if !matches!(simplified_add, Expression::Add(_)) {
                    return simplify_multiplication(&[simplified_add, b.clone()]);
                }
            }
            _ => {}
        }
    }

    let mut all_integers = true;
    let mut integer_product = 1i64;
    for factor in factors {
        match factor {
            Expression::Number(Number::Integer(n)) => {
                integer_product = integer_product.saturating_mul(*n);
            }
            _ => {
                all_integers = false;
                break;
            }
        }
    }
    if all_integers && factors.len() > 2 {
        return Expression::integer(integer_product);
    }

    let mut int_product = 1i64;
    let mut float_product = 1.0;
    let mut has_float = false;
    let mut non_numeric_count = 0;
    let mut first_non_numeric = None;
    let mut numeric_result = None;

    let mut rational_product: Option<BigRational> = None;

    let has_undefined = factors
        .iter()
        .any(|f| matches!(f, Expression::Function { name, .. } if name == "undefined"));

    for factor in factors {
        match factor {
            Expression::Number(Number::Integer(n)) => {
                int_product = int_product.saturating_mul(*n);
                if int_product == 0 && !has_undefined {
                    return Expression::integer(0);
                }
            }
            Expression::Number(Number::Float(f)) => {
                float_product *= f;
                has_float = true;
                if float_product.abs() < EPSILON && !has_undefined {
                    return Expression::integer(0);
                }
            }
            Expression::Number(Number::Rational(r)) => {
                if let Some(ref mut current_rational) = rational_product {
                    *current_rational *= r.as_ref();
                } else {
                    rational_product = Some(r.as_ref().clone());
                }
                if rational_product
                    .as_ref()
                    .expect("BUG: rational_product should be Some at this point")
                    .is_zero()
                    && !has_undefined
                {
                    return Expression::integer(0);
                }
            }
            _ => {
                non_numeric_count += 1;
                if first_non_numeric.is_none() {
                    first_non_numeric = Some(factor);
                }
            }
        }
    }

    if let Some(rational) = rational_product {
        let mut final_rational = rational;
        if int_product != 1 {
            final_rational *= BigRational::from(BigInt::from(int_product));
        }
        if has_float {
            let float_val = final_rational.to_f64().unwrap_or(0.0) * float_product;
            if (float_val - 1.0).abs() >= EPSILON {
                numeric_result = Some(Expression::Number(Number::float(float_val)));
            }
        } else if final_rational.denom() == &BigInt::from(1) {
            if let Some(int_val) = final_rational.numer().to_i64() {
                if int_val != 1 {
                    numeric_result = Some(Expression::integer(int_val));
                }
            } else if !final_rational.is_one() {
                numeric_result = Some(Expression::Number(Number::rational(final_rational)));
            }
        } else if !final_rational.is_one() {
            numeric_result = Some(Expression::Number(Number::rational(final_rational)));
        }
    } else if has_float {
        let total = int_product as f64 * float_product;
        if (total - 1.0).abs() >= EPSILON {
            numeric_result = Some(Expression::Number(Number::float(total)));
        }
    } else if int_product != 1 {
        numeric_result = Some(Expression::integer(int_product));
    }

    match (numeric_result.as_ref(), non_numeric_count) {
        (None, 0) => Expression::integer(1),
        (Some(num), 0) => num.clone(),
        (None, 1) => {
            let factor = first_non_numeric
                .expect("BUG: non_numeric_count is 1 but first_non_numeric is None");
            match factor {
                Expression::Add(terms) => simplify_addition(terms),
                Expression::Pow(base, exp) => simplify_power(base, exp),
                _ => factor.simplify(),
            }
        }
        (Some(num), 1) => {
            let factor = first_non_numeric
                .expect("BUG: non_numeric_count is 1 but first_non_numeric is None");
            let simplified_non_numeric = match factor {
                Expression::Add(terms) => simplify_addition(terms),
                Expression::Pow(base, exp) => simplify_power(base, exp),
                _ => factor.simplify(),
            };
            match num {
                Expression::Number(Number::Integer(1)) => simplified_non_numeric,
                Expression::Number(Number::Float(f)) if (f - 1.0).abs() < EPSILON => {
                    simplified_non_numeric
                }
                _ => Expression::Mul(Box::new(vec![num.clone(), simplified_non_numeric])),
            }
        }
        _ => {
            let mut result_factors = Vec::with_capacity(non_numeric_count + 1);
            if let Some(num) = numeric_result {
                match num {
                    Expression::Number(Number::Integer(1)) => {}
                    Expression::Number(Number::Float(1.0)) => {}
                    _ => result_factors.push(num),
                }
            }
            for factor in factors {
                if !matches!(factor, Expression::Number(_)) {
                    let simplified_factor = match factor {
                        Expression::Add(terms) => simplify_addition(terms),
                        Expression::Pow(base, exp) => simplify_power(base, exp),
                        _ => factor.simplify(),
                    };
                    result_factors.push(simplified_factor);
                }
            }
            match result_factors.len() {
                0 => Expression::integer(1),
                1 => result_factors
                    .into_iter()
                    .next()
                    .expect("BUG: result_factors has length 1 but iterator is empty"),
                _ => {
                    let commutativity =
                        Commutativity::combine(result_factors.iter().map(|f| f.commutativity()));

                    if commutativity.can_sort() {
                        result_factors = combine_like_powers(result_factors);
                        result_factors.sort_by(expression_order);
                    }

                    match result_factors.len() {
                        0 => Expression::integer(1),
                        1 => result_factors.into_iter().next().unwrap(),
                        _ => Expression::Mul(Box::new(result_factors)),
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::simplify::Simplify;
    use crate::symbol;
    use crate::Expression;

    #[test]
    fn test_multiplication_simplification() {
        let expr = simplify_multiplication(&[Expression::integer(2), Expression::integer(3)]);
        assert_eq!(expr, Expression::integer(6));

        let expr = simplify_multiplication(&[Expression::integer(5), Expression::integer(1)]);
        assert_eq!(expr, Expression::integer(5));

        let expr = simplify_multiplication(&[Expression::integer(5), Expression::integer(0)]);
        assert_eq!(expr, Expression::integer(0));
    }

    #[test]
    fn test_nested_multiplication_flattening() {
        let nested = Expression::mul(vec![Expression::integer(3), Expression::integer(4)]);
        let expr = simplify_multiplication(&[Expression::integer(2), nested]);
        assert_eq!(expr, Expression::integer(24));
    }

    #[test]
    fn test_scalar_multiplication_sorts() {
        let y = symbol!(y);
        let x = symbol!(x);
        let expr = Expression::mul(vec![
            Expression::symbol(y.clone()),
            Expression::symbol(x.clone()),
        ]);
        let simplified = expr.simplify();

        match simplified {
            Expression::Mul(factors) => {
                assert_eq!(factors.len(), 2);
                assert_eq!(factors[0], Expression::symbol(symbol!(x)));
                assert_eq!(factors[1], Expression::symbol(symbol!(y)));
            }
            _ => panic!("Expected Mul, got {:?}", simplified),
        }
    }

    #[test]
    fn test_matrix_multiplication_preserves_order() {
        let mat_a = symbol!(A; matrix);
        let mat_b = symbol!(B; matrix);
        let expr = Expression::mul(vec![
            Expression::symbol(mat_b.clone()),
            Expression::symbol(mat_a.clone()),
        ]);
        let simplified = expr.simplify();

        match simplified {
            Expression::Mul(factors) => {
                assert_eq!(factors.len(), 2);
                assert_eq!(factors[0], Expression::symbol(symbol!(B; matrix)));
                assert_eq!(factors[1], Expression::symbol(symbol!(A; matrix)));
            }
            _ => panic!("Expected Mul, got {:?}", simplified),
        }
    }

    #[test]
    fn test_mixed_scalar_matrix_preserves_order() {
        let x = symbol!(x);
        let mat_a = symbol!(A; matrix);
        let expr = Expression::mul(vec![
            Expression::symbol(x.clone()),
            Expression::symbol(mat_a.clone()),
        ]);
        let simplified = expr.simplify();

        match simplified {
            Expression::Mul(factors) => {
                assert_eq!(factors.len(), 2);
                assert_eq!(factors[0], Expression::symbol(symbol!(x)));
                assert_eq!(factors[1], Expression::symbol(symbol!(A; matrix)));
            }
            _ => panic!("Expected Mul, got {:?}", simplified),
        }
    }

    #[test]
    fn test_operator_multiplication_preserves_order() {
        let mat_p = symbol!(P; operator);
        let mat_q = symbol!(Q; operator);
        let expr = Expression::mul(vec![
            Expression::symbol(mat_q.clone()),
            Expression::symbol(mat_p.clone()),
        ]);
        let simplified = expr.simplify();

        match simplified {
            Expression::Mul(factors) => {
                assert_eq!(factors.len(), 2);
                assert_eq!(factors[0], Expression::symbol(symbol!(Q; operator)));
                assert_eq!(factors[1], Expression::symbol(symbol!(P; operator)));
            }
            _ => panic!("Expected Mul, got {:?}", simplified),
        }
    }

    #[test]
    fn test_quaternion_multiplication_preserves_order() {
        let i = symbol!(i; quaternion);
        let j = symbol!(j; quaternion);
        let expr = Expression::mul(vec![
            Expression::symbol(j.clone()),
            Expression::symbol(i.clone()),
        ]);
        let simplified = expr.simplify();

        match simplified {
            Expression::Mul(factors) => {
                assert_eq!(factors.len(), 2);
                assert_eq!(factors[0], Expression::symbol(symbol!(j; quaternion)));
                assert_eq!(factors[1], Expression::symbol(symbol!(i; quaternion)));
            }
            _ => panic!("Expected Mul, got {:?}", simplified),
        }
    }

    #[test]
    fn test_three_scalar_factors_sort() {
        let z = symbol!(z);
        let x = symbol!(x);
        let y = symbol!(y);
        let expr = Expression::mul(vec![
            Expression::symbol(z.clone()),
            Expression::symbol(x.clone()),
            Expression::symbol(y.clone()),
        ]);
        let simplified = expr.simplify();

        match simplified {
            Expression::Mul(factors) => {
                assert_eq!(factors.len(), 3);
                assert_eq!(factors[0], Expression::symbol(symbol!(x)));
                assert_eq!(factors[1], Expression::symbol(symbol!(y)));
                assert_eq!(factors[2], Expression::symbol(symbol!(z)));
            }
            _ => panic!("Expected Mul, got {:?}", simplified),
        }
    }

    #[test]
    fn test_three_matrix_factors_preserve_order() {
        let mat_c = symbol!(C; matrix);
        let mat_a = symbol!(A; matrix);
        let mat_b = symbol!(B; matrix);
        let expr = Expression::mul(vec![
            Expression::symbol(mat_c.clone()),
            Expression::symbol(mat_a.clone()),
            Expression::symbol(mat_b.clone()),
        ]);
        let simplified = expr.simplify();

        match simplified {
            Expression::Mul(factors) => {
                assert_eq!(factors.len(), 3);
                assert_eq!(factors[0], Expression::symbol(symbol!(C; matrix)));
                assert_eq!(factors[1], Expression::symbol(symbol!(A; matrix)));
                assert_eq!(factors[2], Expression::symbol(symbol!(B; matrix)));
            }
            _ => panic!("Expected Mul, got {:?}", simplified),
        }
    }

    #[test]
    fn test_numeric_coefficient_with_scalars_sorts() {
        let y = symbol!(y);
        let x = symbol!(x);
        let expr = Expression::mul(vec![
            Expression::integer(2),
            Expression::symbol(y.clone()),
            Expression::symbol(x.clone()),
        ]);
        let simplified = expr.simplify();

        match simplified {
            Expression::Mul(factors) => {
                assert_eq!(factors.len(), 3);
                assert_eq!(factors[0], Expression::integer(2));
                assert_eq!(factors[1], Expression::symbol(symbol!(x)));
                assert_eq!(factors[2], Expression::symbol(symbol!(y)));
            }
            _ => panic!("Expected Mul, got {:?}", simplified),
        }
    }

    #[test]
    fn test_numeric_coefficient_with_matrices_preserves_order() {
        let mat_b = symbol!(B; matrix);
        let mat_a = symbol!(A; matrix);
        let expr = Expression::mul(vec![
            Expression::integer(2),
            Expression::symbol(mat_b.clone()),
            Expression::symbol(mat_a.clone()),
        ]);
        let simplified = expr.simplify();

        match simplified {
            Expression::Mul(factors) => {
                assert_eq!(factors.len(), 3);
                assert_eq!(factors[0], Expression::integer(2));
                assert_eq!(factors[1], Expression::symbol(symbol!(B; matrix)));
                assert_eq!(factors[2], Expression::symbol(symbol!(A; matrix)));
            }
            _ => panic!("Expected Mul, got {:?}", simplified),
        }
    }
}
