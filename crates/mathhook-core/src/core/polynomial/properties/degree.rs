//! Polynomial degree computation and leading coefficient extraction
//!
//! This module provides methods for computing polynomial degrees and
//! extracting leading coefficients with respect to specific variables.

use crate::core::{Expression, Number, Symbol};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use super::super::cache::with_cache;

/// Compute hash for caching using pointer address
///
/// Since Expression doesn't implement Hash, we use a combination of
/// the pointer address (for heap-allocated) and discriminant for caching.
/// This provides a fast but imperfect hash - collisions are acceptable
/// since the cache is just an optimization.
pub(crate) fn compute_hash(expr: &Expression) -> u64 {
    let mut hasher = DefaultHasher::new();

    let ptr = expr as *const Expression as usize;
    ptr.hash(&mut hasher);

    std::mem::discriminant(expr).hash(&mut hasher);

    hasher.finish()
}

/// Extract integer from expression
pub(crate) fn extract_integer(expr: &Expression) -> Option<i64> {
    match expr {
        Expression::Number(Number::Integer(n)) => Some(*n),
        _ => None,
    }
}

/// Compute degree of polynomial with respect to a variable (cached)
pub(crate) fn degree_cached(expr: &Expression, var: &Symbol) -> Option<i64> {
    let expr_hash = compute_hash(expr);
    let var_name = var.name();

    if let Some(deg) = with_cache(|cache| cache.get_degree(expr_hash, var_name)) {
        return Some(deg);
    }

    let deg = compute_degree_impl(expr, var)?;
    with_cache(|cache| cache.set_degree(expr_hash, var_name, deg));
    Some(deg)
}

/// Compute degree of polynomial with respect to a variable
pub(crate) fn compute_degree_impl(expr: &Expression, var: &Symbol) -> Option<i64> {
    match expr {
        Expression::Number(_) => Some(0),
        Expression::Symbol(s) => {
            if s == var {
                Some(1)
            } else {
                Some(0)
            }
        }
        Expression::Add(terms) => terms
            .iter()
            .filter_map(|t| compute_degree_impl(t, var))
            .max(),
        Expression::Mul(terms) => {
            let degrees: Option<Vec<i64>> =
                terms.iter().map(|t| compute_degree_impl(t, var)).collect();
            degrees.map(|ds| ds.into_iter().sum())
        }
        Expression::Pow(base, exp) => {
            let base_deg = compute_degree_impl(base, var)?;
            let exp_val = extract_integer(exp)?;
            Some(base_deg * exp_val)
        }
        _ => None,
    }
}

/// Compute total degree (maximum sum of degrees across all terms)
pub(crate) fn compute_total_degree_impl(expr: &Expression, vars: &[Symbol]) -> Option<i64> {
    match expr {
        Expression::Number(_) => Some(0),
        Expression::Symbol(s) => {
            if vars.contains(s) {
                Some(1)
            } else {
                Some(0)
            }
        }
        Expression::Add(terms) => {
            // Total degree of sum is max of total degrees
            terms
                .iter()
                .filter_map(|t| compute_total_degree_impl(t, vars))
                .max()
        }
        Expression::Mul(terms) => {
            // Total degree of product is sum of total degrees
            let degrees: Option<Vec<i64>> = terms
                .iter()
                .map(|t| compute_total_degree_impl(t, vars))
                .collect();
            degrees.map(|ds| ds.into_iter().sum())
        }
        Expression::Pow(base, exp) => {
            let base_deg = compute_total_degree_impl(base, vars)?;
            let exp_val = extract_integer(exp)?;
            Some(base_deg * exp_val)
        }
        _ => None,
    }
}

/// Compute leading coefficient with respect to a variable
pub(crate) fn compute_leading_coefficient_impl(expr: &Expression, var: &Symbol) -> Expression {
    match expr {
        Expression::Number(_) => expr.clone(),
        Expression::Symbol(s) => {
            if s == var {
                Expression::integer(1)
            } else {
                expr.clone()
            }
        }
        Expression::Pow(base, _exp) => {
            if let Expression::Symbol(s) = base.as_ref() {
                if s == var {
                    return Expression::integer(1);
                }
            }
            expr.clone()
        }
        Expression::Mul(factors) => {
            // Extract non-variable factors as coefficient
            let mut coefficient_factors = Vec::new();
            for factor in factors.iter() {
                if !contains_variable(factor, var) {
                    coefficient_factors.push(factor.clone());
                } else if let Expression::Mul(inner_factors) = factor {
                    // Handle nested multiplications
                    for inner in inner_factors.iter() {
                        if !contains_variable(inner, var) {
                            coefficient_factors.push(inner.clone());
                        }
                    }
                }
            }
            if coefficient_factors.is_empty() {
                Expression::integer(1)
            } else if coefficient_factors.len() == 1 {
                coefficient_factors.into_iter().next().unwrap()
            } else {
                Expression::mul(coefficient_factors)
            }
        }
        Expression::Add(terms) => {
            // Find term with highest degree and extract its coefficient
            let mut max_degree = -1i64;
            let mut leading_term = Expression::integer(0);

            for term in terms.iter() {
                if let Some(deg) = compute_degree_impl(term, var) {
                    if deg > max_degree {
                        max_degree = deg;
                        leading_term = term.clone();
                    }
                }
            }

            compute_leading_coefficient_impl(&leading_term, var)
        }
        _ => Expression::integer(1),
    }
}

/// Check if expression contains a specific variable
pub(crate) fn contains_variable(expr: &Expression, var: &Symbol) -> bool {
    match expr {
        Expression::Symbol(s) => s == var,
        Expression::Add(terms) | Expression::Mul(terms) => {
            terms.iter().any(|t| contains_variable(t, var))
        }
        Expression::Pow(base, exp) => contains_variable(base, var) || contains_variable(exp, var),
        _ => false,
    }
}
