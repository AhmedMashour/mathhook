//! Minimal overhead, maximum performance implementation

use crate::core::{Expression, Number};
use num_traits::ToPrimitive;
use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

mod arithmetic;
mod complex;
mod constants;
mod functions;
mod matrix;

pub use arithmetic::*;
pub use complex::*;
pub use constants::*;
pub use functions::*;
pub use matrix::*;

/// Simple memoization cache for expensive simplifications
static SIMPLIFY_CACHE: OnceLock<Mutex<HashMap<String, Expression>>> = OnceLock::new();

fn get_cache() -> &'static Mutex<HashMap<String, Expression>> {
    SIMPLIFY_CACHE.get_or_init(|| Mutex::new(HashMap::new()))
}

fn cache_key(expr: &Expression) -> String {
    format!("{:?}", expr)
}

/// Trait for simplifying expressions
pub trait Simplify {
    fn simplify(&self) -> Self;
}

impl Simplify for Expression {
    #[inline(always)]
    fn simplify(&self) -> Self {
        // Simple expressions don't need caching
        match self {
            Expression::Number(_) | Expression::Symbol(_) | Expression::Constant(_) => {
                return self.clone()
            }
            _ => {}
        }

        // For complex expressions, check cache first
        let key = cache_key(self);
        if let Ok(cache) = get_cache().lock() {
            if let Some(cached) = cache.get(&key) {
                return cached.clone();
            }
        }

        let result = match self {
            Expression::Add(terms) => self.simplify_addition(terms),
            Expression::Mul(factors) => self.simplify_multiplication(factors),
            Expression::Pow(base, exp) => self.simplify_power(base, exp),
            Expression::Function { name, args } => simplify_function(name, args),
            Expression::Complex(_) => simplify_complex(self),
            Expression::Matrix(_) => simplify_matrix(self),
            Expression::Constant(constant) => simplify_constant(constant),
            Expression::Relation(_) => self.clone(),
            Expression::Piecewise(_) => self.clone(),
            Expression::Set(_) => self.clone(),
            Expression::Interval(_) => self.clone(),
            Expression::Calculus(_) => self.clone(),
            _ => self.clone(),
        };

        // Cache the result for complex expressions only
        if matches!(
            self,
            Expression::Add(_)
                | Expression::Mul(_)
                | Expression::Pow(_, _)
                | Expression::Function { .. }
                | Expression::Complex(_)
                | Expression::Matrix(_)
        ) {
            if let Ok(mut cache) = get_cache().lock() {
                // Limit cache size to prevent memory bloat
                if cache.len() < 1000 {
                    cache.insert(key, result.clone());
                }
            }
        }

        result
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

        // Single pass - count and accumulate with SIMD optimization for large numeric arrays
        if terms.len() > 100 && terms.iter().all(|t| matches!(t, Expression::Number(_))) {
            // SIMD-optimized path for large arrays of pure numbers
            let mut int_values = Vec::new();
            let mut float_values = Vec::new();

            for term in terms {
                match term {
                    Expression::Number(Number::Integer(n)) => int_values.push(*n as f64),
                    Expression::Number(Number::Float(f)) => float_values.push(*f),
                    Expression::Number(Number::Rational(r)) => {
                        if let Some(f) = r.to_f64() {
                            float_values.push(f);
                        } else {
                            // Fallback for very large rationals
                            return Expression::Add(Box::new(terms.to_vec()));
                        }
                    }
                    Expression::Number(Number::BigInteger(bi)) => {
                        if let Some(f) = bi.to_f64() {
                            float_values.push(f);
                        } else {
                            // Fallback for very large big integers
                            return Expression::Add(Box::new(terms.to_vec()));
                        }
                    }
                    _ => {
                        // This shouldn't happen given our check, but handle gracefully
                        return Expression::Add(Box::new(terms.to_vec()));
                    }
                }
            }

            // Use iterator sum for potential SIMD optimization
            let total_sum: f64 = int_values.iter().sum::<f64>() + float_values.iter().sum::<f64>();

            if total_sum.fract() == 0.0 && total_sum.abs() <= i64::MAX as f64 {
                return Expression::integer(total_sum as i64);
            } else {
                return Expression::Number(Number::float(total_sum));
            }
        }

        // Standard path for mixed or small arrays
        for term in terms {
            match term {
                Expression::Number(Number::Integer(n)) => {
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
                    Expression::Number(Number::Integer(a)),
                    Expression::Number(Number::Integer(b)),
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
            if let Expression::Number(Number::Integer(0)) = factor {
                return Expression::integer(0);
            }
        }

        // SIMD-optimized path for large arrays of pure numbers
        if factors.len() > 50 && factors.iter().all(|f| matches!(f, Expression::Number(_))) {
            let mut int_values = Vec::new();
            let mut float_values = Vec::new();

            for factor in factors {
                match factor {
                    Expression::Number(Number::Integer(n)) => int_values.push(*n as f64),
                    Expression::Number(Number::Float(f)) => float_values.push(*f),
                    Expression::Number(Number::Rational(r)) => {
                        if let Some(f) = r.to_f64() {
                            float_values.push(f);
                        } else {
                            // Fallback for very large rationals
                            return Expression::Mul(Box::new(factors.to_vec()));
                        }
                    }
                    Expression::Number(Number::BigInteger(bi)) => {
                        if let Some(f) = bi.to_f64() {
                            float_values.push(f);
                        } else {
                            // Fallback for very large big integers
                            return Expression::Mul(Box::new(factors.to_vec()));
                        }
                    }
                    _ => {
                        // This shouldn't happen given our check, but handle gracefully
                        return Expression::Mul(Box::new(factors.to_vec()));
                    }
                }
            }

            // Use iterator product for potential SIMD optimization
            let total_product: f64 =
                int_values.iter().product::<f64>() * float_values.iter().product::<f64>();

            if total_product.fract() == 0.0 && total_product.abs() <= i64::MAX as f64 {
                return Expression::integer(total_product as i64);
            } else {
                return Expression::Number(Number::float(total_product));
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
                Expression::Number(Number::Integer(n)) => {
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
                // Convert back to integer if possible
                if total.fract() == 0.0 && total.abs() <= i64::MAX as f64 {
                    Some(Expression::integer(total as i64))
                } else {
                    Some(Expression::Number(Number::float(total)))
                }
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
            (None, 1) => first_non_numeric.unwrap().simplify(),
            (Some(num), 1) => {
                // Only multiply if the numeric factor isn't 1
                match num {
                    Expression::Number(Number::Integer(1)) => first_non_numeric.unwrap().simplify(),
                    Expression::Number(Number::Float(f)) if *f == 1.0 => {
                        first_non_numeric.unwrap().simplify()
                    }
                    _ => Expression::mul(vec![num.clone(), first_non_numeric.unwrap().simplify()]),
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
                        // Recursively simplify each factor
                        result_factors.push(factor.simplify());
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
            (_, Expression::Number(Number::Integer(0))) => Expression::integer(1),
            (_, Expression::Number(Number::Float(f))) if *f == 0.0 => Expression::integer(1),

            // x^1 = x
            (_, Expression::Number(Number::Integer(1))) => base.clone(),
            (_, Expression::Number(Number::Float(f))) if *f == 1.0 => base.clone(),

            // 0^n = 0 (for n > 0)
            (Expression::Number(Number::Integer(0)), Expression::Number(Number::Integer(n)))
                if *n > 0 =>
            {
                Expression::integer(0)
            }
            (Expression::Number(Number::Float(f)), _) if *f == 0.0 => Expression::integer(0),

            // 1^n = 1
            (Expression::Number(Number::Integer(1)), _) => Expression::integer(1),
            (Expression::Number(Number::Float(f)), _) if *f == 1.0 => Expression::integer(1),

            // Direct numeric powers for small integers
            (
                Expression::Number(Number::Integer(base_val)),
                Expression::Number(Number::Integer(exp_val)),
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

            // Float powers
            (Expression::Number(Number::Float(b)), Expression::Number(Number::Integer(e))) => {
                if *e >= -10 && *e <= 10 {
                    let result = b.powi(*e as i32);
                    Expression::Number(Number::float(result))
                } else {
                    Expression::pow(base.clone(), exp.clone())
                }
            }

            (Expression::Number(Number::Float(b)), Expression::Number(Number::Float(e))) => {
                let result = b.powf(*e);
                if result.is_finite() {
                    Expression::Number(Number::float(result))
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
