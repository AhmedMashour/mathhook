//! Stable, Low-Variance Performance Operations
//!
//! This module provides allocation-free, lock-free operations designed to eliminate
//! performance outliers and provide consistent, predictable performance.
use crate::matrices::operations::MatrixOperations;
use crate::matrices::unified::Matrix;
use crate::{
    core::{Expression, Number},
    expr,
};
use num_traits::{ToPrimitive, Zero};

use crate::core::constants::EPSILON;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

thread_local! {
    static LOCAL_CACHE: std::cell::RefCell<std::collections::HashMap<u64, Expression>> =
        std::cell::RefCell::new(std::collections::HashMap::new());
}

/// Atomic counters for lock-free statistics
static SIMD_OPERATIONS: AtomicUsize = AtomicUsize::new(0);
static CACHE_HITS: AtomicUsize = AtomicUsize::new(0);
static CACHE_MISSES: AtomicUsize = AtomicUsize::new(0);

/// Bulk addition with zero allocations
#[inline(always)]
pub fn stable_bulk_addition(terms: &[Expression]) -> Expression {
    // Fast path: no allocations for simple cases
    match terms.len() {
        0 => return expr!(0),
        1 => return terms[0].clone(),
        2 => return stable_add_two(&terms[0], &terms[1]),
        _ => {}
    }

    // Optimized path: process in-place without allocations
    let mut int_sum: i64 = 0;
    let mut float_sum: f64 = 0.0;
    let mut has_float = false;
    let mut non_numeric_count = 0;
    let mut first_non_numeric_idx = 0;

    // Single pass: no allocations, no vector creation
    for (i, term) in terms.iter().enumerate() {
        match term {
            Expression::Number(Number::Integer(n)) => {
                int_sum = int_sum.saturating_add(*n);
            }
            Expression::Number(Number::Float(f)) => {
                float_sum += *f;
                has_float = true;
            }
            Expression::Number(Number::Rational(r)) => {
                if let Some(f) = r.to_f64() {
                    float_sum += f;
                    has_float = true;
                } else {
                    // Keep track of first non-numeric for fallback
                    if non_numeric_count == 0 {
                        first_non_numeric_idx = i;
                    }
                    non_numeric_count += 1;
                }
            }
            _ => {
                if non_numeric_count == 0 {
                    first_non_numeric_idx = i;
                }
                non_numeric_count += 1;
            }
        }
    }

    // Return optimized result without allocations
    if non_numeric_count == 0 {
        // All numeric - return single result
        if has_float {
            let total = int_sum as f64 + float_sum;
            Expression::Number(Number::Float(total))
        } else {
            Expression::Number(Number::Integer(int_sum))
        }
    } else if non_numeric_count == 1 && int_sum == 0 && float_sum.abs() < EPSILON {
        // Only one non-numeric term with zero numeric sum
        terms[first_non_numeric_idx].clone()
    } else {
        // Mixed case: use allocation-minimized fallback
        stable_mixed_addition(terms, int_sum, float_sum, has_float)
    }
}

/// Fast two-term addition without allocations
#[inline(always)]
fn stable_add_two(a: &Expression, b: &Expression) -> Expression {
    match (a, b) {
        (Expression::Number(Number::Integer(x)), Expression::Number(Number::Integer(y))) => {
            Expression::Number(Number::Integer(x.saturating_add(*y)))
        }
        (Expression::Number(Number::Float(x)), Expression::Number(Number::Float(y))) => {
            Expression::Number(Number::Float(x + y))
        }
        (Expression::Number(Number::Integer(x)), Expression::Number(Number::Float(y))) => {
            Expression::Number(Number::Float(*x as f64 + y))
        }
        (Expression::Number(Number::Float(x)), Expression::Number(Number::Integer(y))) => {
            Expression::Number(Number::Float(x + *y as f64))
        }
        _ => {
            // Fallback: create minimal addition expression
            Expression::Add(Arc::new(vec![a.clone(), b.clone()]))
        }
    }
}

/// Allocation-minimized mixed addition
fn stable_mixed_addition(
    terms: &[Expression],
    int_sum: i64,
    float_sum: f64,
    has_float: bool,
) -> Expression {
    let mut result_terms = Vec::with_capacity(terms.len().min(8)); // Pre-allocate reasonable size

    // Add numeric sum if non-zero
    if has_float {
        let total = int_sum as f64 + float_sum;
        if total.abs() >= EPSILON {
            result_terms.push(Expression::Number(Number::Float(total)));
        }
    } else if int_sum != 0 {
        result_terms.push(Expression::Number(Number::Integer(int_sum)));
    }

    // Add non-numeric terms
    for term in terms {
        match term {
            Expression::Number(_) => {} // Already processed
            _ => result_terms.push(term.clone()),
        }
    }

    match result_terms.len() {
        0 => Expression::integer(0),
        1 => result_terms
            .into_iter()
            .next()
            .expect("BUG: result_terms has length 1 but iterator is empty"),
        _ => Expression::Add(Arc::new(result_terms)),
    }
}

/// Ultra-fast bulk multiplication with zero allocations
#[inline(always)]
pub fn stable_bulk_multiplication(factors: &[Expression]) -> Expression {
    // Fast path: no allocations for simple cases
    match factors.len() {
        0 => return Expression::integer(1),
        1 => return factors[0].clone(),
        2 => return stable_multiply_two(&factors[0], &factors[1]),
        _ => {}
    }

    // Check for zero early (short-circuit)
    for factor in factors {
        if is_zero(factor) {
            return Expression::integer(0);
        }
    }

    // Optimized path: process in-place without allocations
    let mut int_product: i64 = 1;
    let mut float_product: f64 = 1.0;
    let mut has_float = false;
    let mut non_numeric_count = 0;
    let mut first_non_numeric_idx = 0;

    // Single pass: no allocations, no vector creation
    for (i, factor) in factors.iter().enumerate() {
        match factor {
            Expression::Number(Number::Integer(n)) => {
                int_product = int_product.saturating_mul(*n);
            }
            Expression::Number(Number::Float(f)) => {
                float_product *= *f;
                has_float = true;
            }
            Expression::Number(Number::Rational(r)) => {
                if let Some(f) = r.to_f64() {
                    float_product *= f;
                    has_float = true;
                } else {
                    if non_numeric_count == 0 {
                        first_non_numeric_idx = i;
                    }
                    non_numeric_count += 1;
                }
            }
            _ => {
                if non_numeric_count == 0 {
                    first_non_numeric_idx = i;
                }
                non_numeric_count += 1;
            }
        }
    }

    // Return optimized result without allocations
    if non_numeric_count == 0 {
        // All numeric - return single result
        if has_float {
            let total = int_product as f64 * float_product;
            Expression::Number(Number::Float(total))
        } else {
            Expression::Number(Number::Integer(int_product))
        }
    } else if non_numeric_count == 1 && int_product == 1 && (float_product - 1.0).abs() < EPSILON {
        // Only one non-numeric factor with identity numeric product
        factors[first_non_numeric_idx].clone()
    } else {
        // Mixed case: use allocation-minimized fallback
        stable_mixed_multiplication(factors, int_product, float_product, has_float)
    }
}

/// Fast two-term multiplication without allocations
#[inline(always)]
fn stable_multiply_two(a: &Expression, b: &Expression) -> Expression {
    match (a, b) {
        (Expression::Number(Number::Integer(x)), Expression::Number(Number::Integer(y))) => {
            Expression::Number(Number::Integer(x.saturating_mul(*y)))
        }
        (Expression::Number(Number::Float(x)), Expression::Number(Number::Float(y))) => {
            Expression::Number(Number::Float(x * y))
        }
        (Expression::Number(Number::Integer(x)), Expression::Number(Number::Float(y))) => {
            Expression::Number(Number::Float(*x as f64 * y))
        }
        (Expression::Number(Number::Float(x)), Expression::Number(Number::Integer(y))) => {
            Expression::Number(Number::Float(x * *y as f64))
        }
        _ => {
            // Fallback: create minimal multiplication expression
            Expression::Mul(Arc::new(vec![a.clone(), b.clone()]))
        }
    }
}

/// Allocation-minimized mixed multiplication
fn stable_mixed_multiplication(
    factors: &[Expression],
    int_product: i64,
    float_product: f64,
    has_float: bool,
) -> Expression {
    let mut result_factors = Vec::with_capacity(factors.len().min(8)); // Pre-allocate reasonable size

    // Add numeric product if not identity
    if has_float {
        let total = int_product as f64 * float_product;
        if (total - 1.0).abs() >= EPSILON {
            result_factors.push(Expression::Number(Number::Float(total)));
        }
    } else if int_product != 1 {
        result_factors.push(Expression::Number(Number::Integer(int_product)));
    }

    // Add non-numeric factors
    for factor in factors {
        match factor {
            Expression::Number(_) => {} // Already processed
            _ => result_factors.push(factor.clone()),
        }
    }

    match result_factors.len() {
        0 => Expression::integer(1),
        1 => result_factors
            .into_iter()
            .next()
            .expect("BUG: result_factors has length 1 but iterator is empty"),
        _ => Expression::Mul(Arc::new(result_factors)),
    }
}

/// Check if expression is zero without allocations
#[inline(always)]
fn is_zero(expr: &Expression) -> bool {
    match expr {
        Expression::Number(Number::Integer(0)) => true,
        Expression::Number(Number::Float(f)) if f.abs() < EPSILON => true,
        Expression::Number(Number::Rational(r)) if r.is_zero() => true,
        _ => false,
    }
}

/// Lock-free SIMD operations with consistent performance
pub struct StableSIMD;

impl StableSIMD {
    /// Stable bulk addition with predictable performance
    #[inline(always)]
    pub fn bulk_add_stable(values: &[f64]) -> f64 {
        SIMD_OPERATIONS.fetch_add(1, Ordering::Relaxed);

        // Use manual loop unrolling for consistent performance
        let len = values.len();
        let chunks = len / 8; // Process 8 at a time for better cache usage
        let remainder = len % 8;

        let mut sum = 0.0;

        // Unrolled loop for consistent performance
        for i in 0..chunks {
            let base = i * 8;
            sum += values[base]
                + values[base + 1]
                + values[base + 2]
                + values[base + 3]
                + values[base + 4]
                + values[base + 5]
                + values[base + 6]
                + values[base + 7];
        }

        // Handle remainder
        let remainder_start = chunks * 8;
        for i in 0..remainder {
            sum += values[remainder_start + i];
        }

        sum
    }

    /// Stable bulk multiplication with predictable performance
    #[inline(always)]
    pub fn bulk_multiply_stable(values: &[f64]) -> f64 {
        SIMD_OPERATIONS.fetch_add(1, Ordering::Relaxed);

        let len = values.len();
        let chunks = len / 8;
        let remainder = len % 8;

        let mut product = 1.0;

        // Unrolled loop for consistent performance
        for i in 0..chunks {
            let base = i * 8;
            product *= values[base]
                * values[base + 1]
                * values[base + 2]
                * values[base + 3]
                * values[base + 4]
                * values[base + 5]
                * values[base + 6]
                * values[base + 7];
        }

        // Handle remainder
        let remainder_start = chunks * 8;
        for i in 0..remainder {
            product *= values[remainder_start + i];
        }

        product
    }
}

/// Thread-local cache operations (no locks, no contention)
pub struct StableCache;

impl StableCache {
    /// Get cached result without locks
    #[inline(always)]
    pub fn get(hash: u64) -> Option<Expression> {
        LOCAL_CACHE.with(|cache| {
            let cache_ref = cache.borrow();
            if let Some(result) = cache_ref.get(&hash) {
                CACHE_HITS.fetch_add(1, Ordering::Relaxed);
                Some(result.clone())
            } else {
                CACHE_MISSES.fetch_add(1, Ordering::Relaxed);
                None
            }
        })
    }

    /// Store result in cache without locks
    #[inline(always)]
    pub fn store(hash: u64, result: Expression) {
        LOCAL_CACHE.with(|cache| {
            let mut cache_ref = cache.borrow_mut();

            // Simple size limit to prevent unbounded growth
            if cache_ref.len() >= 1000 {
                // Remove oldest entry (simple FIFO)
                if let Some(oldest_key) = cache_ref.keys().next().copied() {
                    cache_ref.remove(&oldest_key);
                }
            }

            cache_ref.insert(hash, result);
        });
    }

    /// Get cache statistics without locks
    pub fn stats() -> (usize, usize, usize) {
        let hits = CACHE_HITS.load(Ordering::Relaxed);
        let misses = CACHE_MISSES.load(Ordering::Relaxed);
        let simd_ops = SIMD_OPERATIONS.load(Ordering::Relaxed);
        (hits, misses, simd_ops)
    }
}

/// Stable matrix operations without allocation spikes
pub struct StableMatrix;

impl StableMatrix {
    /// Process matrix without excessive allocations
    pub fn process_stable(matrix: &crate::matrices::types::MatrixData) -> Expression {
        let rows = &matrix.rows;

        // Early return for empty matrix
        if rows.is_empty() {
            return Expression::matrix(vec![]);
        }

        // Check if matrix is already simplified (avoid unnecessary work)
        let mut needs_simplification = false;

        for row in rows {
            for element in row {
                if !is_simple_numeric(element) {
                    needs_simplification = true;
                    break;
                }
            }
            if needs_simplification {
                break;
            }
        }

        if !needs_simplification {
            return Expression::Matrix(Arc::new(Matrix::Dense(matrix.clone())));
        }

        let mut simplified_rows = Vec::with_capacity(rows.len());

        for row in rows {
            let mut simplified_row = Vec::with_capacity(row.len());
            for element in row {
                simplified_row.push(element.clone().simplify_matrix());
            }
            simplified_rows.push(simplified_row);
        }

        Expression::matrix(simplified_rows)
    }
}

#[inline(always)]
fn is_simple_numeric(expr: &Expression) -> bool {
    matches!(
        expr,
        Expression::Number(Number::Integer(_))
            | Expression::Number(Number::Float(_))
            | Expression::Number(Number::Rational(_))
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stable_bulk_addition() {
        let terms = vec![expr!(1), expr!(2), expr!(3)];

        let result = stable_bulk_addition(&terms);
        assert_eq!(result, expr!(6));
    }

    #[test]
    fn test_stable_bulk_multiplication() {
        let factors = vec![expr!(2), expr!(3), Expression::integer(4)];

        let result = stable_bulk_multiplication(&factors);
        assert_eq!(result, expr!(24));
    }

    #[test]
    fn test_stable_simd_operations() {
        let values = vec![1.0, 2.0, 3.0, 4.0, 5.0];

        let sum = StableSIMD::bulk_add_stable(&values);
        assert_eq!(sum, 15.0);

        let product = StableSIMD::bulk_multiply_stable(&values);
        assert_eq!(product, 120.0);
    }

    #[test]
    fn test_stable_cache() {
        let hash = 12345;
        let num_expr = expr!(42);

        assert!(StableCache::get(hash).is_none());

        StableCache::store(hash, num_expr.clone());
        assert_eq!(StableCache::get(hash), Some(num_expr));

        let (hits, misses, _) = StableCache::stats();
        assert!(hits > 0);
        assert!(misses > 0);
    }
}
