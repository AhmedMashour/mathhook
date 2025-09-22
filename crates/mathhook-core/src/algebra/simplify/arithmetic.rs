//! Arithmetic operation simplification
//!
//! Handles simplification of basic arithmetic operations: addition, multiplication, and powers.
//! Implements ultra-fast paths for common cases while maintaining mathematical correctness.

use super::Simplify;
use crate::core::{Expression, Number};
use num_bigint::BigInt;
use num_rational::BigRational;
use num_traits::{One, ToPrimitive, Zero};

/// Simplify addition expressions with minimal overhead
#[inline(always)]
pub fn simplify_addition(terms: &[Expression]) -> Expression {
    if terms.is_empty() {
        return Expression::integer(0);
    }
    if terms.len() == 1 {
        return terms[0].clone();
    }

    // First, flatten nested Add expressions
    let mut flattened_terms = Vec::new();
    for term in terms {
        match term {
            Expression::Add(nested_terms) => {
                // Recursively flatten nested additions
                flattened_terms.extend_from_slice(nested_terms);
            }
            _ => flattened_terms.push(term.clone()),
        }
    }

    // If we flattened anything, recursively call with flattened terms
    if flattened_terms.len() != terms.len() {
        return simplify_addition(&flattened_terms);
    }

    // Ultra-fast path for numeric addition
    let mut int_sum = 0i64;
    let mut float_sum = 0.0;
    let mut has_float = false;
    let mut rational_sum: Option<BigRational> = None;
    let mut non_numeric_count = 0;
    let mut first_non_numeric: Option<Expression> = None;
    let mut numeric_result = None;

    for term in terms {
        // Process terms directly without recursive simplification for performance
        match term {
            Expression::Number(Number::Integer(n)) => {
                int_sum = int_sum.saturating_add(*n);
            }
            Expression::Number(Number::Float(f)) => {
                float_sum += *f;
                has_float = true;
            }
            Expression::Number(Number::Rational(r)) => {
                if let Some(ref mut current_sum) = rational_sum {
                    *current_sum += r.as_ref();
                } else {
                    rational_sum = Some(r.as_ref().clone());
                }
            }
            _ => {
                non_numeric_count += 1;
                if first_non_numeric.is_none() {
                    first_non_numeric = Some(term.clone());
                }
            }
        }
    }

    // Determine numeric result
    if let Some(rational) = rational_sum {
        // Combine rational with integer and float sums
        let mut final_rational = rational;
        if int_sum != 0 {
            final_rational += BigRational::from(BigInt::from(int_sum));
        }
        if has_float {
            // Convert to float if we have float terms
            let float_val = final_rational.to_f64().unwrap_or(0.0) + float_sum;
            if float_val != 0.0 {
                numeric_result = Some(Expression::Number(Number::float(float_val)));
            }
        } else {
            // Keep as rational if it's not zero
            if !final_rational.is_zero() {
                numeric_result = Some(Expression::Number(Number::rational(final_rational)));
            }
        }
    } else if has_float {
        let total = int_sum as f64 + float_sum;
        if total != 0.0 {
            numeric_result = Some(Expression::Number(Number::float(total)));
        }
    } else if int_sum != 0 {
        numeric_result = Some(Expression::integer(int_sum));
    }

    match (numeric_result.as_ref(), non_numeric_count) {
        (None, 0) => Expression::integer(0),
        (Some(num), 0) => num.clone(),
        (None, 1) => {
            // Return the single remaining term (already simplified)
            first_non_numeric.unwrap()
        }
        (Some(num), 1) => {
            // Use the already simplified non-numeric term
            let simplified_non_numeric = first_non_numeric.unwrap();
            // If numeric part is zero, just return the non-numeric part
            match num {
                Expression::Number(Number::Integer(0)) => simplified_non_numeric,
                Expression::Number(Number::Float(f)) if *f == 0.0 => simplified_non_numeric,
                _ => Expression::add(vec![num.clone(), simplified_non_numeric]),
            }
        }
        _ => {
            // Multiple non-numeric terms - collect like terms and build result efficiently
            let mut result_terms = Vec::with_capacity(non_numeric_count + 1);
            if let Some(num) = numeric_result {
                // Only include numeric result if it's not zero
                match num {
                    Expression::Number(Number::Integer(0)) => {}
                    Expression::Number(Number::Float(f)) if f == 0.0 => {}
                    _ => result_terms.push(num),
                }
            }

            // Collect like terms using an order-preserving approach
            let mut like_terms: Vec<(String, Expression, Vec<Expression>)> = Vec::new();

            for term in terms {
                if !matches!(term, Expression::Number(_)) {
                    // Process non-numeric terms directly for performance
                    match term {
                        Expression::Number(Number::Integer(0)) => {}
                        Expression::Number(Number::Float(f)) if *f == 0.0 => {}
                        _ => {
                            // Extract coefficient and base term
                            let (coeff, base) = extract_coefficient_and_base(term);
                            let base_key = format!("{:?}", base); // Simple key for like terms

                            // Find existing entry or create new one
                            if let Some(entry) =
                                like_terms.iter_mut().find(|(key, _, _)| key == &base_key)
                            {
                                entry.2.push(coeff);
                            } else {
                                like_terms.push((base_key, base.clone(), vec![coeff]));
                            }
                        }
                    }
                }
            }

            // Combine like terms
            for (_, base, coeffs) in like_terms {
                if coeffs.len() == 1 {
                    // Single term, reconstruct if coefficient is not 1
                    let coeff = &coeffs[0];
                    match coeff {
                        Expression::Number(Number::Integer(1)) => {
                            // Just add the base term (coefficient is 1)
                            result_terms.push(base);
                        }
                        _ => {
                            // Reconstruct coefficient * base
                            result_terms.push(Expression::mul(vec![coeff.clone(), base]));
                        }
                    }
                } else {
                    // Multiple coefficients for the same base - sum them
                    let coeff_sum = Expression::add(coeffs);
                    match coeff_sum {
                        Expression::Number(Number::Integer(0)) => {}
                        Expression::Number(Number::Float(f)) if f == 0.0 => {}
                        Expression::Number(Number::Integer(1)) => {
                            // Coefficient sum is 1, just add the base
                            result_terms.push(base);
                        }
                        _ => {
                            // Non-zero coefficient sum, reconstruct
                            result_terms.push(Expression::mul(vec![coeff_sum, base]));
                        }
                    }
                }
            }

            match result_terms.len() {
                0 => Expression::integer(0),
                1 => result_terms.into_iter().next().unwrap(),
                _ => Expression::Add(Box::new(result_terms)),
            }
        }
    }
}

/// Simplify multiplication with minimal overhead and flattening
#[inline(always)]
pub fn simplify_multiplication(factors: &[Expression]) -> Expression {
    if factors.is_empty() {
        return Expression::integer(1);
    }
    if factors.len() == 1 {
        return factors[0].clone();
    }

    // Flatten nested multiplications without recursive simplification for performance
    let mut flattened_factors = Vec::new();
    for factor in factors {
        match factor {
            Expression::Mul(nested_factors) => {
                // Flatten nested multiplication
                flattened_factors.extend(nested_factors.iter().cloned());
            }
            _ => flattened_factors.push(factor.clone()),
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
                    return Expression::mul(vec![a.clone(), simplified_add]);
                }
            }
            (Expression::Add(terms), b) => {
                let simplified_add = simplify_addition(terms);
                if !matches!(simplified_add, Expression::Add(_)) {
                    // The addition simplified to something else, try multiplication again
                    return Expression::mul(vec![simplified_add, b.clone()]);
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

    for factor in factors {
        match factor {
            Expression::Number(Number::Integer(n)) => {
                int_product = int_product.saturating_mul(*n);
                if int_product == 0 {
                    return Expression::integer(0); // Early exit for zero
                }
            }
            Expression::Number(Number::Float(f)) => {
                float_product *= f;
                has_float = true;
                if float_product == 0.0 {
                    return Expression::integer(0); // Early exit for zero
                }
            }
            Expression::Number(Number::Rational(r)) => {
                if let Some(ref mut current_rational) = rational_product {
                    *current_rational *= r.as_ref();
                } else {
                    rational_product = Some(r.as_ref().clone());
                }
                if rational_product.as_ref().unwrap().is_zero() {
                    return Expression::integer(0); // Early exit for zero
                }
            }
            _ => {
                non_numeric_count += 1;
                if first_non_numeric.is_none() {
                    first_non_numeric = Some(factor.clone());
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
        (None, 1) => first_non_numeric.unwrap(),
        (Some(num), 1) => {
            // Only multiply if the numeric factor isn't 1
            let non_numeric = first_non_numeric.unwrap();
            match num {
                Expression::Number(Number::Integer(1)) => non_numeric,
                Expression::Number(Number::Float(f)) if *f == 1.0 => non_numeric,
                _ => Expression::mul(vec![num.clone(), non_numeric]),
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
                    result_factors.push(factor.clone());
                }
            }
            match result_factors.len() {
                0 => Expression::integer(1),
                1 => result_factors.into_iter().next().unwrap(),
                _ => Expression::Mul(Box::new(result_factors)),
            }
        }
    }
}

/// Power simplification
#[inline(always)]
pub fn simplify_power(base: &Expression, exp: &Expression) -> Expression {
    // Process base and exponent directly for performance
    match (base, exp) {
        // x^0 = 1
        (_, Expression::Number(Number::Integer(0))) => Expression::integer(1),
        // x^1 = x
        (_, Expression::Number(Number::Integer(1))) => base.clone(),
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
        (Expression::Pow(b, e), c) => Expression::pow(
            (**b).clone(),
            Expression::mul(vec![(**e).clone(), c.clone()]),
        ),
        _ => Expression::Pow(Box::new(base.clone()), Box::new(exp.clone())),
    }
}

/// Extract coefficient and base term from an expression
/// For example: 3*x -> (3, x), -2*y -> (-2, y), x -> (1, x)
fn extract_coefficient_and_base(expr: &Expression) -> (Expression, Expression) {
    match expr {
        Expression::Mul(factors) if factors.len() >= 2 => {
            // Check if first factor is numeric
            if matches!(factors[0], Expression::Number(_)) {
                let coeff = factors[0].clone();
                let base = if factors.len() == 2 {
                    factors[1].clone()
                } else {
                    Expression::Mul(Box::new(factors[1..].to_vec()))
                };
                (coeff, base)
            } else {
                // No numeric coefficient, coefficient is 1
                (Expression::integer(1), expr.clone())
            }
        }
        _ => {
            // Single term, coefficient is 1
            (Expression::integer(1), expr.clone())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::Symbol;

    #[test]
    fn test_addition_simplification() {
        // Simple integer addition
        let expr = simplify_addition(&[Expression::integer(2), Expression::integer(3)]);
        assert_eq!(expr, Expression::integer(5));

        // Addition with zero
        let expr = simplify_addition(&[Expression::integer(5), Expression::integer(0)]);
        assert_eq!(expr, Expression::integer(5));

        // Mixed numeric and symbolic
        let x = Symbol::new("x");
        let expr = simplify_addition(&[Expression::integer(2), Expression::symbol(x.clone())]);
        assert_eq!(
            expr,
            Expression::add(vec![Expression::integer(2), Expression::symbol(x)])
        );
    }

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
    fn test_power_simplification() {
        let x = Symbol::new("x");

        // x^0 = 1
        let expr = simplify_power(&Expression::symbol(x.clone()), &Expression::integer(0));
        assert_eq!(expr, Expression::integer(1));

        // x^1 = x
        let expr = simplify_power(&Expression::symbol(x.clone()), &Expression::integer(1));
        assert_eq!(expr, Expression::symbol(x));
    }

    #[test]
    fn test_nested_multiplication_flattening() {
        // Mul([2, Mul([3, 4])]) should become Mul([2, 3, 4]) = 24
        let nested = Expression::mul(vec![Expression::integer(3), Expression::integer(4)]);
        let expr = simplify_multiplication(&[Expression::integer(2), nested]);
        assert_eq!(expr, Expression::integer(24));
    }
}
