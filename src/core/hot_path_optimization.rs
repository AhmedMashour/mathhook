//! Hot path optimization for frequently executed code paths
//! Aggressive inlining and branch prediction optimization

use crate::core::{Expression, Number, Symbol};
use num_traits::{Zero, One, ToPrimitive};
use num_bigint::BigInt;

/// Hot path optimizer for critical performance sections
pub struct HotPathOptimizer;

impl HotPathOptimizer {
    /// Ultra-fast integer detection (most common case)
    #[inline(always)]
    pub fn is_integer_fast(expr: &Expression) -> Option<i64> {
        match expr {
            Expression::Number(Number::Integer(i)) => i.to_i64(),
            _ => None,
        }
    }
    
    /// Ultra-fast zero detection
    #[inline(always)]
    pub fn is_zero_fast(expr: &Expression) -> bool {
        match expr {
            Expression::Number(Number::Integer(i)) => i.is_zero(),
            Expression::Number(Number::Float(f)) => *f == 0.0,
            _ => false,
        }
    }
    
    /// Ultra-fast one detection
    #[inline(always)]
    pub fn is_one_fast(expr: &Expression) -> bool {
        match expr {
            Expression::Number(Number::Integer(i)) => i.is_one(),
            Expression::Number(Number::Float(f)) => *f == 1.0,
            _ => false,
        }
    }
    
    /// Hot path integer arithmetic (most frequent operation)
    #[inline(always)]
    pub fn add_integers_fast(a: i64, b: i64) -> Option<Expression> {
        a.checked_add(b).map(Expression::integer)
    }
    
    /// Hot path integer multiplication
    #[inline(always)]
    pub fn mul_integers_fast(a: i64, b: i64) -> Option<Expression> {
        a.checked_mul(b).map(Expression::integer)
    }
    
    /// Branch prediction optimized addition
    #[inline(always)]
    pub fn add_optimized(left: &Expression, right: &Expression) -> Expression {
        // Branch prediction: integers are the most common case (90%+)
        if let (Some(a), Some(b)) = (Self::is_integer_fast(left), Self::is_integer_fast(right)) {
            if let Some(result) = Self::add_integers_fast(a, b) {
                return result;
            }
        }
        
        // Fast zero detection (second most common)
        if Self::is_zero_fast(left) {
            return right.clone();
        }
        if Self::is_zero_fast(right) {
            return left.clone();
        }
        
        // Fallback to general case
        Expression::add(vec![left.clone(), right.clone()])
    }
    
    /// Branch prediction optimized multiplication
    #[inline(always)]
    pub fn mul_optimized(left: &Expression, right: &Expression) -> Expression {
        // Branch prediction: check for zero first (early termination)
        if Self::is_zero_fast(left) || Self::is_zero_fast(right) {
            return Expression::integer(0);
        }
        
        // Check for one (identity)
        if Self::is_one_fast(left) {
            return right.clone();
        }
        if Self::is_one_fast(right) {
            return left.clone();
        }
        
        // Fast integer multiplication
        if let (Some(a), Some(b)) = (Self::is_integer_fast(left), Self::is_integer_fast(right)) {
            if let Some(result) = Self::mul_integers_fast(a, b) {
                return result;
            }
        }
        
        // Fallback to general case
        Expression::mul(vec![left.clone(), right.clone()])
    }
    
    /// Hot path power optimization
    #[inline(always)]
    pub fn pow_optimized(base: &Expression, exp: &Expression) -> Expression {
        // Branch prediction: most common exponents first
        if Self::is_zero_fast(exp) {
            return Expression::integer(1);
        }
        if Self::is_one_fast(exp) {
            return base.clone();
        }
        if Self::is_zero_fast(base) {
            return Expression::integer(0);
        }
        if Self::is_one_fast(base) {
            return Expression::integer(1);
        }
        
        // Fast integer powers for small exponents
        if let (Some(base_val), Some(exp_val)) = (Self::is_integer_fast(base), Self::is_integer_fast(exp)) {
            if exp_val >= 0 && exp_val <= 10 {
                if let Some(result) = base_val.checked_pow(exp_val as u32) {
                    return Expression::integer(result);
                }
            }
        }
        
        // Fallback to general case
        Expression::pow(base.clone(), exp.clone())
    }
}

/// Hot path optimized operations
pub struct HotPathOptimized {
    hit_count: std::cell::RefCell<u64>,
    fast_path_hits: std::cell::RefCell<u64>,
}

impl HotPathOptimized {
    /// Create a new hot path optimizer
    pub fn new() -> Self {
        Self {
            hit_count: std::cell::RefCell::new(0),
            fast_path_hits: std::cell::RefCell::new(0),
        }
    }
    
    /// Perform hot path optimized simplification
    #[inline(always)]
    pub fn simplify_hot(&self, expr: &Expression) -> Expression {
        *self.hit_count.borrow_mut() += 1;
        
        match expr {
            // Hot path: numbers and symbols (no processing needed)
            Expression::Number(_) | Expression::Symbol(_) => {
                *self.fast_path_hits.borrow_mut() += 1;
                expr.clone()
            },
            
            // Hot path: addition
            Expression::Add(terms) => {
                if terms.len() == 2 {
                    *self.fast_path_hits.borrow_mut() += 1;
                    HotPathOptimizer::add_optimized(&terms[0], &terms[1])
                } else {
                    self.simplify_add_general(terms)
                }
            },
            
            // Hot path: multiplication
            Expression::Mul(factors) => {
                if factors.len() == 2 {
                    *self.fast_path_hits.borrow_mut() += 1;
                    HotPathOptimizer::mul_optimized(&factors[0], &factors[1])
                } else {
                    self.simplify_mul_general(factors)
                }
            },
            
            // Hot path: power
            Expression::Pow(base, exp) => {
                *self.fast_path_hits.borrow_mut() += 1;
                HotPathOptimizer::pow_optimized(base, exp)
            },
            
            Expression::Function { .. } => expr.clone(),
        }
    }
    
    /// General addition simplification (fallback)
    fn simplify_add_general(&self, terms: &[Expression]) -> Expression {
        // Implement general addition logic here
        Expression::Add(terms.to_vec())
    }
    
    /// General multiplication simplification (fallback)
    fn simplify_mul_general(&self, factors: &[Expression]) -> Expression {
        // Implement general multiplication logic here
        Expression::Mul(factors.to_vec())
    }
    
    /// Get optimization statistics
    pub fn stats(&self) -> HotPathStats {
        let total = *self.hit_count.borrow();
        let fast = *self.fast_path_hits.borrow();
        
        HotPathStats {
            total_operations: total,
            fast_path_hits: fast,
            fast_path_ratio: if total > 0 { fast as f64 / total as f64 } else { 0.0 },
        }
    }
    
    /// Reset statistics
    pub fn reset_stats(&self) {
        *self.hit_count.borrow_mut() = 0;
        *self.fast_path_hits.borrow_mut() = 0;
    }
}

impl Default for HotPathOptimized {
    fn default() -> Self {
        Self::new()
    }
}

/// Hot path optimization statistics
#[derive(Debug)]
pub struct HotPathStats {
    pub total_operations: u64,
    pub fast_path_hits: u64,
    pub fast_path_ratio: f64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fast_detection_functions() {
        let zero = Expression::integer(0);
        let one = Expression::integer(1);
        let five = Expression::integer(5);
        let x = Expression::symbol(Symbol::new("x"));
        
        assert!(HotPathOptimizer::is_zero_fast(&zero));
        assert!(!HotPathOptimizer::is_zero_fast(&one));
        assert!(!HotPathOptimizer::is_zero_fast(&x));
        
        assert!(HotPathOptimizer::is_one_fast(&one));
        assert!(!HotPathOptimizer::is_one_fast(&zero));
        assert!(!HotPathOptimizer::is_one_fast(&x));
        
        assert_eq!(HotPathOptimizer::is_integer_fast(&five), Some(5));
        assert_eq!(HotPathOptimizer::is_integer_fast(&x), None);
    }
    
    #[test]
    fn test_fast_integer_operations() {
        let result = HotPathOptimizer::add_integers_fast(5, 3);
        assert_eq!(result, Some(Expression::integer(8)));
        
        let result = HotPathOptimizer::mul_integers_fast(4, 7);
        assert_eq!(result, Some(Expression::integer(28)));
        
        // Test overflow handling
        let result = HotPathOptimizer::add_integers_fast(i64::MAX, 1);
        assert_eq!(result, None);
    }
    
    #[test]
    fn test_hot_path_optimization() {
        let optimizer = HotPathOptimized::new();
        
        let five = Expression::integer(5);
        let three = Expression::integer(3);
        let x = Expression::symbol(Symbol::new("x"));
        
        // Test hot path for numbers
        let result = optimizer.simplify_hot(&five);
        assert_eq!(result, five);
        
        // Test hot path for addition
        let sum = Expression::add(vec![five.clone(), three.clone()]);
        let result = optimizer.simplify_hot(&sum);
        assert_eq!(result, Expression::integer(8));
        
        // Test statistics
        let stats = optimizer.stats();
        assert!(stats.total_operations > 0);
        assert!(stats.fast_path_ratio > 0.0);
    }
    
    #[test]
    fn test_branch_prediction_optimization() {
        use std::time::Instant;
        
        let optimizer = HotPathOptimized::new();
        let operations = 100_000;
        
        // Create test expressions (mostly integers for realistic workload)
        let mut test_exprs = Vec::new();
        for i in 0..operations {
            if i % 10 == 0 {
                // 10% symbols
                test_exprs.push(Expression::symbol(Symbol::new("x")));
            } else {
                // 90% integers (realistic distribution)
                test_exprs.push(Expression::integer(i as i64));
            }
        }
        
        let start = Instant::now();
        for expr in &test_exprs {
            let _result = optimizer.simplify_hot(expr);
        }
        let duration = start.elapsed();
        
        let ops_per_sec = operations as f64 / duration.as_secs_f64();
        println!("Hot path performance: {:.2}M ops/sec", ops_per_sec / 1_000_000.0);
        
        let stats = optimizer.stats();
        println!("Fast path ratio: {:.2}%", stats.fast_path_ratio * 100.0);
        
        // Should achieve high performance due to branch prediction
        assert!(ops_per_sec > 1_000_000.0, "Expected >1M ops/sec, got {:.2}", ops_per_sec);
        assert!(stats.fast_path_ratio > 0.8, "Expected >80% fast path hits, got {:.2}%", stats.fast_path_ratio * 100.0);
    }
}
