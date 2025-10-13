//! Addition simplification operations

use super::helpers::{expression_order, extract_arithmetic_coefficient_and_base};
use super::multiplication::simplify_multiplication;
use super::power::simplify_power;
use crate::core::{Expression, Number};
use num_bigint::BigInt;
use num_rational::BigRational;
use num_traits::{ToPrimitive, Zero};
use std::collections::VecDeque;

/// Simplify addition expressions with minimal overhead
pub fn simplify_addition(terms: &[Expression]) -> Expression {
    if terms.is_empty() {
        return Expression::integer(0);
    }
    if terms.len() == 1 {
        return terms[0].clone();
    }

    // Iteratively flatten nested Add expressions to avoid stack overflow
    let mut flattened_terms: Vec<Expression> = Vec::new();
    let mut to_process: VecDeque<&Expression> = terms.iter().collect();

    while let Some(term) = to_process.pop_front() {
        match term {
            Expression::Add(nested_terms) => {
                for nested_term in nested_terms.iter().rev() {
                    to_process.push_front(nested_term);
                }
            }
            _ => flattened_terms.push(term.clone()),
        }
    }

    // Use flattened terms for the rest of the function
    let terms = &flattened_terms;

    // Ultra-fast path for numeric addition
    let mut int_sum = 0i64;
    let mut float_sum = 0.0;
    let mut has_float = false;
    let mut rational_sum: Option<BigRational> = None;
    let mut non_numeric_count = 0;
    let mut first_non_numeric: Option<Expression> = None;
    let mut numeric_result = None;

    for term in terms {
        // Simplify the term, but avoid recursive calls for Add expressions (already flattened)
        let simplified_term = match term {
            Expression::Add(_) => {
                // Add expressions should already be flattened, so this shouldn't happen
                // But if it does, just use the term as-is to avoid recursion
                term.clone()
            }
            Expression::Mul(factors) => simplify_multiplication(factors),
            Expression::Pow(base, exp) => simplify_power(base, exp),
            _ => term.clone(),
        };
        match simplified_term {
            Expression::Number(Number::Integer(n)) => {
                int_sum = int_sum.saturating_add(n);
            }
            Expression::Number(Number::Float(f)) => {
                float_sum += f;
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
                    first_non_numeric = Some(simplified_term);
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
            first_non_numeric.expect("BUG: non_numeric_count is 1 but first_non_numeric is None")
        }
        (Some(num), 1) => {
            // Use the already simplified non-numeric term
            let simplified_non_numeric = first_non_numeric
                .expect("BUG: non_numeric_count is 1 but first_non_numeric is None");
            // If numeric part is zero, just return the non-numeric part
            match num {
                Expression::Number(Number::Integer(0)) => simplified_non_numeric,
                Expression::Number(Number::Float(f)) if *f == 0.0 => simplified_non_numeric,
                _ => Expression::Add(Box::new(vec![num.clone(), simplified_non_numeric])),
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
                    // Each non-numeric term - use controlled simplification to avoid recursion
                    let simplified_term = match term {
                        Expression::Add(_) => term.clone(), // Already flattened
                        Expression::Mul(factors) => simplify_multiplication(factors),
                        Expression::Pow(base, exp) => simplify_power(base, exp),
                        _ => term.clone(),
                    };
                    match simplified_term {
                        Expression::Number(Number::Integer(0)) => {}
                        Expression::Number(Number::Float(f)) if f == 0.0 => {}
                        _ => {
                            // Extract coefficient and base term
                            let (coeff, base) =
                                extract_arithmetic_coefficient_and_base(&simplified_term);
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
                            result_terms.push(Expression::Mul(Box::new(vec![coeff.clone(), base])));
                        }
                    }
                } else {
                    // Multiple coefficients for the same base - sum them
                    let coeff_sum = simplify_addition(&coeffs);
                    match coeff_sum {
                        Expression::Number(Number::Integer(0)) => {}
                        Expression::Number(Number::Float(f)) if f == 0.0 => {}
                        Expression::Number(Number::Integer(1)) => {
                            // Coefficient sum is 1, just add the base
                            result_terms.push(base);
                        }
                        _ => {
                            result_terms.push(Expression::Mul(Box::new(vec![coeff_sum, base])));
                        }
                    }
                }
            }

            match result_terms.len() {
                0 => Expression::integer(0),
                1 => result_terms
                    .into_iter()
                    .next()
                    .expect("BUG: result_terms has length 1 but iterator is empty"),
                _ => {
                    // Sort terms for canonical ordering
                    result_terms.sort_by(expression_order);
                    Expression::Add(Box::new(result_terms))
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{symbol, Expression};

    #[test]
    fn test_addition_simplification() {
        // Simple integer addition
        let expr = simplify_addition(&[Expression::integer(2), Expression::integer(3)]);
        assert_eq!(expr, Expression::integer(5));

        // Addition with zero
        let expr = simplify_addition(&[Expression::integer(5), Expression::integer(0)]);
        assert_eq!(expr, Expression::integer(5));

        // Mixed numeric and symbolic
        let x = symbol!(x);
        let expr = simplify_addition(&[Expression::integer(2), Expression::symbol(x.clone())]);
        assert_eq!(
            expr,
            Expression::add(vec![Expression::integer(2), Expression::symbol(x)])
        );
    }
}
