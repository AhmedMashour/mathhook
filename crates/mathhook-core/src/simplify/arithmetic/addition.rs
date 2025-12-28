//! Addition simplification operations

use super::helpers::{expression_order, extract_arithmetic_coefficient_and_base};
use super::multiplication::simplify_multiplication;
use super::power::simplify_power;
use super::Simplify;
use crate::core::commutativity::Commutativity;
use crate::core::constants::EPSILON;
use crate::core::{Expression, Number};
use num_bigint::BigInt;
use num_rational::BigRational;
use num_traits::{ToPrimitive, Zero};
use std::collections::VecDeque;
use std::sync::Arc;

fn extract_trig_squared(expr: &Expression, func: &str) -> Option<Expression> {
    if let Expression::Pow(base, exp) = expr {
        if let Expression::Number(Number::Integer(2)) = exp.as_ref() {
            if let Expression::Function { name, args } = base.as_ref() {
                if name.as_ref() == func && args.len() == 1 {
                    return Some(args[0].clone());
                }
            }
        }
    }
    None
}

fn check_pythagorean(terms: &[Expression]) -> Option<Vec<Expression>> {
    for (i, t1) in terms.iter().enumerate() {
        for (j, t2) in terms.iter().enumerate() {
            if i >= j {
                continue;
            }
            if let (Some(arg1), Some(arg2)) = (
                extract_trig_squared(t1, "sin"),
                extract_trig_squared(t2, "cos"),
            ) {
                if arg1 == arg2 {
                    let mut remaining: Vec<_> = terms
                        .iter()
                        .enumerate()
                        .filter(|(k, _)| *k != i && *k != j)
                        .map(|(_, e)| e.clone())
                        .collect();
                    remaining.push(Expression::integer(1));
                    return Some(remaining);
                }
            }
            if let (Some(arg1), Some(arg2)) = (
                extract_trig_squared(t1, "cos"),
                extract_trig_squared(t2, "sin"),
            ) {
                if arg1 == arg2 {
                    let mut remaining: Vec<_> = terms
                        .iter()
                        .enumerate()
                        .filter(|(k, _)| *k != i && *k != j)
                        .map(|(_, e)| e.clone())
                        .collect();
                    remaining.push(Expression::integer(1));
                    return Some(remaining);
                }
            }
        }
    }
    None
}

/// Simplify addition expressions with minimal overhead
pub fn simplify_addition(terms: &[Expression]) -> Expression {
    if terms.is_empty() {
        return Expression::integer(0);
    }

    let mut flattened_terms: Vec<Expression> = Vec::new();
    let mut to_process: VecDeque<&Expression> = terms.iter().collect();

    while let Some(term) = to_process.pop_front() {
        match term {
            Expression::Add(nested_terms) => {
                for nested_term in nested_terms.iter().rev() {
                    to_process.push_front(nested_term);
                }
            }
            Expression::Mul(factors) if factors.len() == 2 => {
                if let (Expression::Number(coeff), Expression::Add(add_terms)) =
                    (&factors[0], &factors[1])
                {
                    for add_term in add_terms.iter() {
                        let distributed = Expression::mul(vec![
                            Expression::Number(coeff.clone()),
                            add_term.clone(),
                        ]);
                        flattened_terms.push(distributed);
                    }
                } else if let (Expression::Add(add_terms), Expression::Number(coeff)) =
                    (&factors[0], &factors[1])
                {
                    for add_term in add_terms.iter() {
                        let distributed = Expression::mul(vec![
                            Expression::Number(coeff.clone()),
                            add_term.clone(),
                        ]);
                        flattened_terms.push(distributed);
                    }
                } else {
                    flattened_terms.push(term.clone());
                }
            }
            _ => flattened_terms.push(term.clone()),
        }
    }

    let terms = &flattened_terms;

    if terms.len() == 2 {
        if let Some(Ok(result)) = super::matrix_ops::try_matrix_add(&terms[0], &terms[1]) {
            return result;
        }
    }

    let mut int_sum = 0i64;
    let mut float_sum = 0.0;
    let mut has_float = false;
    let mut rational_sum: Option<BigRational> = None;
    let mut non_numeric_count = 0;
    let mut first_non_numeric: Option<Expression> = None;
    let mut numeric_result = None;

    for term in terms {
        let simplified_term = match term {
            Expression::Add(_) => term.clone(),
            Expression::Mul(factors) => simplify_multiplication(factors),
            Expression::Pow(base, exp) => simplify_power(base.as_ref(), exp.as_ref()),
            _ => term.simplify(),
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

    if let Some(rational) = rational_sum {
        let mut final_rational = rational;
        if int_sum != 0 {
            final_rational += BigRational::from(BigInt::from(int_sum));
        }
        if has_float {
            let float_val = final_rational.to_f64().unwrap_or(0.0) + float_sum;
            if float_val.abs() >= EPSILON {
                numeric_result = Some(Expression::Number(Number::float(float_val)));
            }
        } else if !final_rational.is_zero() {
            numeric_result = Some(Expression::Number(Number::rational(final_rational)));
        }
    } else if has_float {
        let total = int_sum as f64 + float_sum;
        if total.abs() >= EPSILON {
            numeric_result = Some(Expression::Number(Number::float(total)));
        }
    } else if int_sum != 0 {
        numeric_result = Some(Expression::integer(int_sum));
    }

    match (numeric_result.as_ref(), non_numeric_count) {
        (None, 0) => Expression::integer(0),
        (Some(num), 0) => num.clone(),
        (None, 1) => {
            first_non_numeric.expect("BUG: non_numeric_count is 1 but first_non_numeric is None")
        }
        (Some(num), 1) => {
            let simplified_non_numeric = first_non_numeric
                .expect("BUG: non_numeric_count is 1 but first_non_numeric is None");
            match num {
                Expression::Number(Number::Integer(0)) => simplified_non_numeric,
                Expression::Number(Number::Float(f)) if f.abs() < EPSILON => simplified_non_numeric,
                _ => Expression::Add(Arc::new(vec![num.clone(), simplified_non_numeric])),
            }
        }
        _ => {
            let mut result_terms = Vec::with_capacity(non_numeric_count + 1);
            if let Some(num) = numeric_result {
                match num {
                    Expression::Number(Number::Integer(0)) => {}
                    Expression::Number(Number::Float(0.0)) => {}
                    _ => result_terms.push(num),
                }
            }

            let mut like_terms: Vec<(String, Expression, Vec<Expression>)> = Vec::new();

            for term in terms {
                if !matches!(term, Expression::Number(_)) {
                    let simplified_term = match term {
                        Expression::Add(_) => term.clone(),
                        Expression::Mul(factors) => simplify_multiplication(factors),
                        Expression::Pow(base, exp) => simplify_power(base.as_ref(), exp.as_ref()),
                        _ => term.simplify(),
                    };
                    match simplified_term {
                        Expression::Number(Number::Integer(0)) => {}
                        Expression::Number(Number::Float(0.0)) => {}
                        _ => {
                            let (coeff, base) =
                                extract_arithmetic_coefficient_and_base(&simplified_term);

                            let base_key = format!("{:?}", base);

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

            for (_, base, coeffs) in like_terms {
                if coeffs.len() == 1 {
                    let coeff = &coeffs[0];
                    match coeff {
                        Expression::Number(Number::Integer(1)) => {
                            result_terms.push(base);
                        }
                        _ => {
                            result_terms.push(Expression::Mul(Arc::new(vec![coeff.clone(), base])));
                        }
                    }
                } else {
                    let coeff_sum = simplify_addition(&coeffs);
                    match coeff_sum {
                        Expression::Number(Number::Integer(0)) => {}
                        Expression::Number(Number::Float(0.0)) => {}
                        Expression::Number(Number::Integer(1)) => {
                            result_terms.push(base);
                        }
                        _ => {
                            result_terms.push(Expression::Mul(Arc::new(vec![coeff_sum, base])));
                        }
                    }
                }
            }

            if let Some(pythagorean_terms) = check_pythagorean(&result_terms) {
                return simplify_addition(&pythagorean_terms);
            }

            match result_terms.len() {
                0 => Expression::integer(0),
                1 => result_terms
                    .into_iter()
                    .next()
                    .expect("BUG: result_terms has length 1 but iterator is empty"),
                _ => {
                    let commutativity =
                        Commutativity::combine(result_terms.iter().map(|t| t.commutativity()));

                    if commutativity.can_sort() {
                        result_terms.sort_by(expression_order);
                    }

                    Expression::Add(Arc::new(result_terms))
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::simplify::Simplify;
    use crate::{expr, symbol, Expression};

    #[test]
    fn test_addition_simplification() {
        let expr = simplify_addition(&[Expression::integer(2), Expression::integer(3)]);
        assert_eq!(expr, Expression::integer(5));

        let expr = simplify_addition(&[Expression::integer(5), Expression::integer(0)]);
        assert_eq!(expr, Expression::integer(5));

        let x = symbol!(x);
        let expr = simplify_addition(&[Expression::integer(2), Expression::symbol(x.clone())]);
        assert_eq!(
            expr,
            Expression::add(vec![Expression::integer(2), Expression::symbol(x)])
        );
    }

    #[test]
    fn test_scalar_terms_combine() {
        let x = symbol!(x);
        let y = symbol!(y);

        let xy = Expression::mul(vec![
            Expression::symbol(x.clone()),
            Expression::symbol(y.clone()),
        ]);
        let yx = Expression::mul(vec![
            Expression::symbol(y.clone()),
            Expression::symbol(x.clone()),
        ]);
        let expr = Expression::add(vec![xy.clone(), yx.clone()]);

        let simplified = expr.simplify();

        match simplified {
            Expression::Mul(factors) => {
                assert_eq!(factors.len(), 3);
                assert_eq!(factors[0], Expression::integer(2));
            }
            _ => panic!("Expected Mul, got {:?}", simplified),
        }
    }

    #[test]
    fn test_matrix_terms_not_combined() {
        let mat_a = symbol!(A; matrix);
        let mat_b = symbol!(B; matrix);

        let ab = Expression::mul(vec![
            Expression::symbol(mat_a.clone()),
            Expression::symbol(mat_b.clone()),
        ]);
        let ba = Expression::mul(vec![
            Expression::symbol(mat_b.clone()),
            Expression::symbol(mat_a.clone()),
        ]);
        let expr = Expression::add(vec![ab.clone(), ba.clone()]);

        let simplified = expr.simplify();

        match simplified {
            Expression::Add(terms) => {
                assert_eq!(terms.len(), 2);
            }
            _ => panic!("Expected Add with 2 terms, got {:?}", simplified),
        }
    }

    #[test]
    fn test_identical_matrix_terms_combine() {
        let mat_a = symbol!(A; matrix);
        let mat_b = symbol!(B; matrix);

        let ab1 = Expression::mul(vec![
            Expression::symbol(mat_a.clone()),
            Expression::symbol(mat_b.clone()),
        ]);
        let ab2 = Expression::mul(vec![
            Expression::symbol(mat_a.clone()),
            Expression::symbol(mat_b.clone()),
        ]);
        let expr = Expression::add(vec![ab1, ab2]);

        let simplified = expr.simplify();

        match simplified {
            Expression::Mul(factors) => {
                assert_eq!(factors.len(), 3);
                assert_eq!(factors[0], Expression::integer(2));
            }
            _ => panic!("Expected Mul, got {:?}", simplified),
        }
    }

    #[test]
    fn test_operator_terms_not_combined() {
        let operator_p = symbol!(P; operator);
        let operator_q = symbol!(Q; operator);

        let pq = Expression::mul(vec![
            Expression::symbol(operator_p.clone()),
            Expression::symbol(operator_q.clone()),
        ]);
        let qp = Expression::mul(vec![
            Expression::symbol(operator_q.clone()),
            Expression::symbol(operator_p.clone()),
        ]);
        let expr = Expression::add(vec![pq, qp]);

        let simplified = expr.simplify();

        match simplified {
            Expression::Add(terms) => {
                assert_eq!(terms.len(), 2);
            }
            _ => panic!("Expected Add with 2 terms, got {:?}", simplified),
        }
    }

    #[test]
    fn test_quaternion_terms_not_combined() {
        let i = symbol!(i; quaternion);
        let j = symbol!(j; quaternion);

        let ij = Expression::mul(vec![
            Expression::symbol(i.clone()),
            Expression::symbol(j.clone()),
        ]);
        let ji = Expression::mul(vec![
            Expression::symbol(j.clone()),
            Expression::symbol(i.clone()),
        ]);
        let expr = Expression::add(vec![ij, ji]);

        let simplified = expr.simplify();

        match simplified {
            Expression::Add(terms) => {
                assert_eq!(terms.len(), 2);
            }
            _ => panic!("Expected Add with 2 terms, got {:?}", simplified),
        }
    }

    #[test]
    fn test_scalar_addition_sorts() {
        let y = symbol!(y);
        let x = symbol!(x);
        let expr = Expression::add(vec![
            Expression::symbol(y.clone()),
            Expression::symbol(x.clone()),
        ]);
        let simplified = expr.simplify();

        match simplified {
            Expression::Add(terms) => {
                assert_eq!(terms.len(), 2);
                assert_eq!(terms[0], Expression::symbol(symbol!(x)));
                assert_eq!(terms[1], Expression::symbol(symbol!(y)));
            }
            _ => panic!("Expected Add, got {:?}", simplified),
        }
    }

    #[test]
    fn test_matrix_addition_preserves_order() {
        let mat_b = symbol!(B; matrix);
        let mat_a = symbol!(A; matrix);
        let expr = Expression::add(vec![
            Expression::symbol(mat_b.clone()),
            Expression::symbol(mat_a.clone()),
        ]);
        let simplified = expr.simplify();

        match simplified {
            Expression::Add(terms) => {
                assert_eq!(terms.len(), 2);
                assert_eq!(terms[0], Expression::symbol(symbol!(B; matrix)));
                assert_eq!(terms[1], Expression::symbol(symbol!(A; matrix)));
            }
            _ => panic!("Expected Add, got {:?}", simplified),
        }
    }

    #[test]
    fn test_mixed_scalar_matrix_addition_preserves_order() {
        let x = symbol!(x);
        let mat_a = symbol!(A; matrix);
        let expr = Expression::add(vec![
            Expression::symbol(x.clone()),
            Expression::symbol(mat_a.clone()),
        ]);
        let simplified = expr.simplify();

        match simplified {
            Expression::Add(terms) => {
                assert_eq!(terms.len(), 2);
                assert_eq!(terms[0], expr!(x));
                assert_eq!(terms[1], Expression::symbol(symbol!(A; matrix)));
            }
            _ => panic!("Expected Add, got {:?}", simplified),
        }
    }

    #[test]
    fn test_three_scalar_like_terms_combine() {
        let x = symbol!(x);
        let expr = Expression::add(vec![
            Expression::symbol(x.clone()),
            Expression::symbol(x.clone()),
            Expression::symbol(x.clone()),
        ]);
        let simplified = expr.simplify();

        match simplified {
            Expression::Mul(factors) => {
                assert_eq!(factors.len(), 2);
                assert_eq!(factors[0], Expression::integer(3));
                assert_eq!(factors[1], expr!(x));
            }
            _ => panic!("Expected Mul, got {:?}", simplified),
        }
    }

    #[test]
    fn test_three_matrix_like_terms_combine() {
        let mat_a = symbol!(A; matrix);
        let expr = Expression::add(vec![
            Expression::symbol(mat_a.clone()),
            Expression::symbol(mat_a.clone()),
            Expression::symbol(mat_a.clone()),
        ]);
        let simplified = expr.simplify();

        match simplified {
            Expression::Mul(factors) => {
                assert_eq!(factors.len(), 2);
                assert_eq!(factors[0], Expression::integer(3));
                assert_eq!(factors[1], Expression::symbol(symbol!(A; matrix)));
            }
            _ => panic!("Expected Mul, got {:?}", simplified),
        }
    }

    #[test]
    fn test_incompatible_matrix_addition_during_simplification() {
        let a = Expression::matrix(vec![vec![expr!(1), expr!(2)], vec![expr!(3), expr!(4)]]);
        let b = Expression::matrix(vec![vec![expr!(5), expr!(6), expr!(7)]]);

        let expr = Expression::add(vec![a.clone(), b.clone()]);
        let simplified = expr.simplify();

        match simplified {
            Expression::Add(terms) => {
                assert_eq!(terms.len(), 2);
            }
            _ => panic!(
                "Expected Add with 2 terms for incompatible matrices during simplification, got {:?}",
                simplified
            ),
        }
    }

    #[test]
    fn test_pythagorean_identity_sin_cos() {
        let x = symbol!(x);
        let sin_x = Expression::function("sin", vec![Expression::symbol(x.clone())]);
        let cos_x = Expression::function("cos", vec![Expression::symbol(x.clone())]);
        let sin_squared = Expression::pow(sin_x, Expression::integer(2));
        let cos_squared = Expression::pow(cos_x, Expression::integer(2));

        let expr = Expression::add(vec![sin_squared, cos_squared]);
        let simplified = expr.simplify();

        assert_eq!(simplified, Expression::integer(1));
    }

    #[test]
    fn test_pythagorean_identity_cos_sin() {
        let x = symbol!(x);
        let sin_x = Expression::function("sin", vec![Expression::symbol(x.clone())]);
        let cos_x = Expression::function("cos", vec![Expression::symbol(x.clone())]);
        let sin_squared = Expression::pow(sin_x, Expression::integer(2));
        let cos_squared = Expression::pow(cos_x, Expression::integer(2));

        let expr = Expression::add(vec![cos_squared, sin_squared]);
        let simplified = expr.simplify();

        assert_eq!(simplified, Expression::integer(1));
    }

    #[test]
    fn test_pythagorean_identity_different_args() {
        let x = symbol!(x);
        let y = symbol!(y);
        let sin_x = Expression::function("sin", vec![Expression::symbol(x.clone())]);
        let cos_y = Expression::function("cos", vec![Expression::symbol(y.clone())]);
        let sin_squared = Expression::pow(sin_x, Expression::integer(2));
        let cos_squared = Expression::pow(cos_y, Expression::integer(2));

        let expr = Expression::add(vec![sin_squared, cos_squared]);
        let simplified = expr.simplify();

        match simplified {
            Expression::Add(_) => {}
            _ => panic!("Expected Add (unchanged), got {:?}", simplified),
        }
    }

    #[test]
    fn test_pythagorean_identity_with_additional_terms() {
        let x = symbol!(x);
        let y = symbol!(y);
        let sin_x = Expression::function("sin", vec![Expression::symbol(x.clone())]);
        let cos_x = Expression::function("cos", vec![Expression::symbol(x.clone())]);
        let sin_squared = Expression::pow(sin_x, Expression::integer(2));
        let cos_squared = Expression::pow(cos_x, Expression::integer(2));

        let expr = Expression::add(vec![
            sin_squared,
            cos_squared,
            Expression::symbol(y.clone()),
        ]);
        let simplified = expr.simplify();

        assert_eq!(
            simplified,
            Expression::add(vec![Expression::integer(1), Expression::symbol(y)])
        );
    }

    #[test]
    fn test_pythagorean_identity_not_squared() {
        let x = symbol!(x);
        let sin_x = Expression::function("sin", vec![Expression::symbol(x.clone())]);
        let cos_x = Expression::function("cos", vec![Expression::symbol(x.clone())]);

        let expr = Expression::add(vec![sin_x, cos_x]);
        let simplified = expr.simplify();

        match simplified {
            Expression::Add(_) => {}
            _ => panic!("Expected Add (unchanged), got {:?}", simplified),
        }
    }

    #[test]
    fn test_distribute_numeric_over_addition() {
        let x = symbol!(x);

        let expr = Expression::add(vec![Expression::mul(vec![
            Expression::integer(-1),
            Expression::add(vec![Expression::symbol(x.clone()), Expression::integer(1)]),
        ])]);

        let simplified = expr.simplify();

        match &simplified {
            Expression::Add(terms) => {
                assert_eq!(terms.len(), 2);
                let has_neg_one = terms
                    .iter()
                    .any(|t| matches!(t, Expression::Number(Number::Integer(-1))));
                let has_neg_x = terms.iter().any(|t| {
                    matches!(t, Expression::Mul(factors)
                        if factors.len() == 2
                        && matches!(factors[0], Expression::Number(Number::Integer(-1)))
                    )
                });
                assert!(
                    has_neg_one || has_neg_x,
                    "Expected distributed terms, got {:?}",
                    simplified
                );
            }
            _ => panic!("Expected Add with distributed terms, got {:?}", simplified),
        }
    }
}
