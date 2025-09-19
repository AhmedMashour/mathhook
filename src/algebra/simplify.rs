//! Minimal overhead, maximum performance implementation

use crate::core::{Expression, Number};

/// Trait for simplifying expressions
pub trait Simplify {
    fn simplify(&self) -> Self;
}

impl Simplify for Expression {
    #[inline(always)]
    fn simplify(&self) -> Self {
        match self {
            Expression::Number(_) | Expression::Symbol(_) => self.clone(),
            Expression::Add(terms) => self.simplify_addition(terms),
            Expression::Mul(factors) => self.simplify_multiplication(factors),
            Expression::Pow(base, exp) => self.simplify_power(base, exp),
            Expression::Function { .. } => self.clone(),
        }
    }
}

impl Expression {
    #[inline(always)]
    fn simplify_addition(&self, terms: &[Expression]) -> Self {
        if terms.is_empty() {
            return Expression::integer(0);
        }
        if terms.len() == 1 {
            return terms[0].clone();
        }

        let mut int_sum = 0i64;
        let mut float_sum = 0.0f64;
        let mut has_float = false;
        let mut non_numeric_count = 0;
        let mut first_non_numeric = None;

        // Single pass - count and accumulate
        for term in terms {
            match term {
                Expression::Number(Number::SmallInt(n)) => {
                    int_sum += n;
                }
                Expression::Number(Number::Float(f)) => {
                    float_sum += f;
                    has_float = true;
                }
                _ => {
                    non_numeric_count += 1;
                    if first_non_numeric.is_none() {
                        first_non_numeric = Some(term.clone());
                    }
                }
            }
        }

        let numeric_result = if has_float {
            let total = float_sum + int_sum as f64;
            if total != 0.0 {
                Some(Expression::Number(Number::float(total)))
            } else {
                None
            }
        } else if int_sum != 0 {
            Some(Expression::integer(int_sum))
        } else {
            None
        };

        match (numeric_result.as_ref(), non_numeric_count) {
            (None, 0) => Expression::integer(0),
            (Some(num), 0) => num.clone(),
            (None, 1) => {
                // Ensure single remaining term is fully simplified
                first_non_numeric.unwrap().simplify()
            }
            (Some(num), 1) => {
                // Ensure non-numeric term is simplified
                let simplified_non_numeric = first_non_numeric.unwrap().simplify();
                Expression::add(vec![num.clone(), simplified_non_numeric])
            }
            _ => {
                // Multiple non-numeric terms - build result efficiently with recursive simplification
                let mut result_terms = Vec::with_capacity(non_numeric_count + 1);
                if let Some(num) = numeric_result {
                    result_terms.push(num);
                }
                for term in terms {
                    if !matches!(term, Expression::Number(_)) {
                        // Each non-numeric term
                        result_terms.push(term.simplify());
                    }
                }
                Expression::Add(Box::new(result_terms))
            }
        }
    }

    /// Multiplication with minimal overhead
    #[inline(always)]
    fn simplify_multiplication(&self, factors: &[Expression]) -> Self {
        if factors.is_empty() {
            return Expression::integer(1);
        }
        if factors.len() == 1 {
            return factors[0].clone();
        }

        // Handle simple 2-factor numeric multiplication directly
        if factors.len() == 2 {
            match (&factors[0], &factors[1]) {
                (
                    Expression::Number(Number::SmallInt(a)),
                    Expression::Number(Number::SmallInt(b)),
                ) => {
                    return Expression::integer(a * b);
                }
                (Expression::Number(Number::Float(a)), Expression::Number(Number::Float(b))) => {
                    return Expression::Number(Number::float(a * b));
                }
                _ => {} // Fall through to general case
            }
        }

        // Zero detection first - early termination
        for factor in factors {
            if let Expression::Number(Number::SmallInt(0)) = factor {
                return Expression::integer(0);
            }
        }

        // Direct numeric combination
        let mut int_product = 1i64;
        let mut float_product = 1.0f64;
        let mut has_float = false;
        let mut non_numeric_count = 0;
        let mut first_non_numeric = None;

        for factor in factors {
            match factor {
                Expression::Number(Number::SmallInt(n)) => {
                    int_product *= n;
                }
                Expression::Number(Number::Float(f)) => {
                    float_product *= f;
                    has_float = true;
                }
                _ => {
                    non_numeric_count += 1;
                    if first_non_numeric.is_none() {
                        first_non_numeric = Some(factor.clone());
                    }
                }
            }
        }

        let numeric_result = if has_float {
            let total = float_product * int_product as f64;
            if total != 1.0 {
                Some(Expression::Number(Number::float(total)))
            } else {
                None
            }
        } else if int_product != 1 {
            Some(Expression::integer(int_product))
        } else {
            None
        };

        match (numeric_result.as_ref(), non_numeric_count) {
            (None, 0) => Expression::integer(1),
            (Some(num), 0) => num.clone(),
            (None, 1) => first_non_numeric.unwrap(),
            (Some(num), 1) => {
                // Only multiply if the numeric factor isn't 1
                match num {
                    Expression::Number(Number::SmallInt(1)) => first_non_numeric.unwrap(),
                    Expression::Number(Number::Float(f)) if *f == 1.0 => first_non_numeric.unwrap(),
                    _ => Expression::mul(vec![num.clone(), first_non_numeric.unwrap()]),
                }
            }
            _ => {
                // Multiple factors - build result efficiently
                let mut result_factors = Vec::with_capacity(non_numeric_count + 1);
                if let Some(num) = numeric_result {
                    // Only include numeric factor if it's not 1
                    match num {
                        Expression::Number(Number::SmallInt(1)) => {}
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
    fn simplify_power(&self, base: &Expression, exp: &Expression) -> Self {
        match (base, exp) {
            // x^0 = 1
            (_, Expression::Number(Number::SmallInt(0))) => Expression::integer(1),
            // x^1 = x
            (_, Expression::Number(Number::SmallInt(1))) => base.clone(),
            // 0^n = 0 (for n > 0)
            (Expression::Number(Number::SmallInt(0)), Expression::Number(Number::SmallInt(n)))
                if *n > 0 =>
            {
                Expression::integer(0)
            }
            // 1^n = 1
            (Expression::Number(Number::SmallInt(1)), _) => Expression::integer(1),
            // Direct numeric powers for small integers
            (
                Expression::Number(Number::SmallInt(base_val)),
                Expression::Number(Number::SmallInt(exp_val)),
            ) => {
                if *exp_val >= 0 && *exp_val <= 10 && base_val.abs() <= 100 {
                    // Safe to compute directly
                    let result = (*base_val as f64).powi(*exp_val as i32);
                    if result.fract() == 0.0 && result.abs() <= i64::MAX as f64 {
                        Expression::integer(result as i64)
                    } else {
                        Expression::Number(Number::float(result))
                    }
                } else {
                    Expression::pow(base.clone(), exp.clone())
                }
            }
            _ => Expression::pow(base.clone(), exp.clone()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ultra_fast_addition() {
        // Test direct numeric addition
        let expr = Expression::add(vec![Expression::integer(2), Expression::integer(3)]);
        let result = expr.simplify();
        assert_eq!(result, Expression::integer(5));
    }

    #[test]
    fn test_ultra_fast_multiplication() {
        // Test direct numeric multiplication
        let expr = Expression::mul(vec![Expression::integer(2), Expression::integer(3)]);
        let result = expr.simplify();
        assert_eq!(result, Expression::integer(6));
    }

    #[test]
    fn test_ultra_fast_power() {
        // Test direct power computation
        let expr = Expression::pow(Expression::integer(2), Expression::integer(3));
        let result = expr.simplify();
        assert_eq!(result, Expression::integer(8));
    }

    #[test]
    fn test_zero_detection() {
        // Test zero multiplication
        let expr = Expression::mul(vec![Expression::integer(0), Expression::integer(5)]);
        let result = expr.simplify();
        assert_eq!(result, Expression::integer(0));
    }
}
