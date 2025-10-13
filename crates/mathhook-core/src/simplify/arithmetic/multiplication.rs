//! Multiplication simplification operations

use super::addition::simplify_addition;
use super::helpers::expression_order;
use super::power::simplify_power;
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

    // Iteratively flatten nested multiplications to avoid stack overflow
    let mut flattened_factors = Vec::new();
    let mut to_process: Vec<&Expression> = factors.iter().collect();

    while let Some(factor) = to_process.pop() {
        match factor {
            Expression::Mul(nested_factors) => {
                // Add nested factors to the processing queue
                to_process.extend(nested_factors.iter());
            }
            _ => {
                // Simplify non-Mul expressions during flattening (same pattern as addition)
                let simplified = match factor {
                    Expression::Add(terms) => simplify_addition(terms),
                    Expression::Pow(base, exp) => simplify_power(base, exp),
                    _ => factor.clone(),
                };
                flattened_factors.push(simplified);
            }
        }
    }

    // Use flattened factors for the rest of the function
    let factors = &flattened_factors;

    // Handle simple 2-factor numeric multiplication directly
    if factors.len() == 2 {
        match (&factors[0], &factors[1]) {
            (Expression::Number(Number::Integer(a)), Expression::Number(Number::Integer(b))) => {
                return Expression::integer(a * b);
            }
            // Handle rational with denominator 1 as integer
            (Expression::Number(Number::Rational(r)), Expression::Number(Number::Integer(b))) => {
                if r.denom() == &BigInt::from(1) {
                    if let Some(a) = r.numer().to_i64() {
                        return Expression::integer(a * b);
                    }
                }
            }
            (Expression::Number(Number::Integer(a)), Expression::Number(Number::Rational(r))) => {
                if r.denom() == &BigInt::from(1) {
                    if let Some(b) = r.numer().to_i64() {
                        return Expression::integer(a * b);
                    }
                }
            }
            // Handle two rationals with denominator 1 as integers
            (
                Expression::Number(Number::Rational(r1)),
                Expression::Number(Number::Rational(r2)),
            ) => {
                if r1.denom() == &BigInt::from(1) && r2.denom() == &BigInt::from(1) {
                    if let (Some(a), Some(b)) = (r1.numer().to_i64(), r2.numer().to_i64()) {
                        return Expression::integer(a * b);
                    }
                }
            }
            (Expression::Number(Number::Float(a)), Expression::Number(Number::Float(b))) => {
                return Expression::Number(Number::float(a * b));
            }
            // Handle rational conversion: a * b^(-1) = a/b
            (Expression::Number(Number::Integer(a)), Expression::Pow(base, exp)) => {
                if let (
                    Expression::Number(Number::Integer(b)),
                    Expression::Number(Number::Integer(-1)),
                ) = (base.as_ref(), exp.as_ref())
                {
                    return Expression::Number(Number::rational(BigRational::new(
                        BigInt::from(*a),
                        BigInt::from(*b),
                    )));
                }
            }
            (Expression::Pow(base, exp), Expression::Number(Number::Integer(a))) => {
                if let (
                    Expression::Number(Number::Integer(b)),
                    Expression::Number(Number::Integer(-1)),
                ) = (base.as_ref(), exp.as_ref())
                {
                    return Expression::Number(Number::rational(BigRational::new(
                        BigInt::from(*a),
                        BigInt::from(*b),
                    )));
                }
            }
            // Handle Rational * Pow(Rational, -1) = Rational * (1/Rational) = Rational / Rational
            (Expression::Number(Number::Rational(r1)), Expression::Pow(base, exp)) => {
                if let (
                    Expression::Number(Number::Rational(r2)),
                    Expression::Number(Number::Integer(-1)),
                ) = (base.as_ref(), exp.as_ref())
                {
                    // r1 * (r2)^(-1) = r1 * (1/r2) = r1 / r2
                    let result = r1.as_ref() / r2.as_ref();
                    return Expression::Number(Number::rational(result));
                }
            }
            (Expression::Pow(base, exp), Expression::Number(Number::Rational(r1))) => {
                if let (
                    Expression::Number(Number::Rational(r2)),
                    Expression::Number(Number::Integer(-1)),
                ) = (base.as_ref(), exp.as_ref())
                {
                    // (r2)^(-1) * r1 = (1/r2) * r1 = r1 / r2
                    let result = r1.as_ref() / r2.as_ref();
                    return Expression::Number(Number::rational(result));
                }
            }
            // Handle special case: if one factor is Add, try to simplify it first
            (a, Expression::Add(terms)) => {
                let simplified_add = simplify_addition(terms);
                if !matches!(simplified_add, Expression::Add(_)) {
                    // The addition simplified to something else, try multiplication again
                    return simplify_multiplication(&[a.clone(), simplified_add]);
                }
            }
            (Expression::Add(terms), b) => {
                let simplified_add = simplify_addition(terms);
                if !matches!(simplified_add, Expression::Add(_)) {
                    // The addition simplified to something else, try multiplication again
                    return simplify_multiplication(&[simplified_add, b.clone()]);
                }
            }
            _ => {} // Fall through to general case
        }
    }

    // Handle multiple integer multiplication
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

    // General case with ultra-fast numeric handling
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
                if float_product == 0.0 && !has_undefined {
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

    // Determine numeric result
    if let Some(rational) = rational_product {
        // Combine rational with integer and float products
        let mut final_rational = rational;
        if int_product != 1 {
            final_rational *= BigRational::from(BigInt::from(int_product));
        }
        if has_float {
            // Convert to float if we have float factors
            let float_val = final_rational.to_f64().unwrap_or(0.0) * float_product;
            if float_val != 1.0 {
                numeric_result = Some(Expression::Number(Number::float(float_val)));
            }
        } else {
            // Check if rational has denominator 1 and convert to integer
            if final_rational.denom() == &BigInt::from(1) {
                if let Some(int_val) = final_rational.numer().to_i64() {
                    if int_val != 1 {
                        numeric_result = Some(Expression::integer(int_val));
                    }
                } else {
                    // Large integer - keep as rational
                    if !final_rational.is_one() {
                        numeric_result = Some(Expression::Number(Number::rational(final_rational)));
                    }
                }
            } else {
                // Keep as rational if it's not 1
                if !final_rational.is_one() {
                    numeric_result = Some(Expression::Number(Number::rational(final_rational)));
                }
            }
        }
    } else if has_float {
        let total = int_product as f64 * float_product;
        if total != 1.0 {
            numeric_result = Some(Expression::Number(Number::float(total)));
        }
    } else if int_product != 1 {
        numeric_result = Some(Expression::integer(int_product));
    }

    match (numeric_result.as_ref(), non_numeric_count) {
        (None, 0) => Expression::integer(1),
        (Some(num), 0) => num.clone(),
        (None, 1) => {
            // Simplify the single non-numeric factor without recursion
            let factor = first_non_numeric
                .expect("BUG: non_numeric_count is 1 but first_non_numeric is None");
            match factor {
                Expression::Add(terms) => simplify_addition(terms),
                Expression::Pow(base, exp) => simplify_power(base, exp),
                _ => factor.clone(),
            }
        }
        (Some(num), 1) => {
            // Only multiply if the numeric factor isn't 1
            let factor = first_non_numeric
                .expect("BUG: non_numeric_count is 1 but first_non_numeric is None");
            let simplified_non_numeric = match factor {
                Expression::Add(terms) => simplify_addition(terms),
                Expression::Pow(base, exp) => simplify_power(base, exp),
                _ => factor.clone(),
            };
            match num {
                Expression::Number(Number::Integer(1)) => simplified_non_numeric,
                Expression::Number(Number::Float(f)) if *f == 1.0 => simplified_non_numeric,
                _ => Expression::Mul(Box::new(vec![num.clone(), simplified_non_numeric])),
            }
        }
        _ => {
            // Multiple factors - build result efficiently
            let mut result_factors = Vec::with_capacity(non_numeric_count + 1);
            if let Some(num) = numeric_result {
                // Only include numeric factor if it's not 1
                match num {
                    Expression::Number(Number::Integer(1)) => {}
                    Expression::Number(Number::Float(f)) if f == 1.0 => {}
                    _ => result_factors.push(num),
                }
            }
            for factor in factors {
                if !matches!(factor, Expression::Number(_)) {
                    let simplified_factor = match factor {
                        Expression::Add(terms) => simplify_addition(terms),
                        Expression::Pow(base, exp) => simplify_power(base, exp),
                        _ => factor.clone(),
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
                    // Sort factors for canonical ordering
                    result_factors.sort_by(expression_order);
                    Expression::Mul(Box::new(result_factors))
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Expression;

    #[test]
    fn test_multiplication_simplification() {
        // Simple integer multiplication
        let expr = simplify_multiplication(&[Expression::integer(2), Expression::integer(3)]);
        assert_eq!(expr, Expression::integer(6));

        // Multiplication with one
        let expr = simplify_multiplication(&[Expression::integer(5), Expression::integer(1)]);
        assert_eq!(expr, Expression::integer(5));

        // Multiplication with zero
        let expr = simplify_multiplication(&[Expression::integer(5), Expression::integer(0)]);
        assert_eq!(expr, Expression::integer(0));
    }

    #[test]
    fn test_nested_multiplication_flattening() {
        // Mul([2, Mul([3, 4])]) should become Mul([2, 3, 4]) = 24
        let nested = Expression::mul(vec![Expression::integer(3), Expression::integer(4)]);
        let expr = simplify_multiplication(&[Expression::integer(2), nested]);
        assert_eq!(expr, Expression::integer(24));
    }
}
