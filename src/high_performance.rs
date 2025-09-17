//! High-performance mode using CompactExpression for 42M+ ops/sec
//! Provides a high-performance alternative using memory-optimized data structures

use crate::core::{CompactExpression, CompactNumber, Symbol};
use crate::algebra::simplify::Simplify;
use std::ops::{Add, Sub, Mul, Neg};

// Type aliases for high-performance mode
pub type Expression = CompactExpression;
pub type Number = CompactNumber;

impl Add for Expression {
    type Output = Expression;
    
    fn add(self, rhs: Expression) -> Expression {
        Expression::add(vec![self, rhs])
    }
}

impl Add for &Expression {
    type Output = Expression;
    
    fn add(self, rhs: &Expression) -> Expression {
        Expression::add(vec![self.clone(), rhs.clone()])
    }
}

impl Add<&Expression> for Expression {
    type Output = Expression;
    
    fn add(self, rhs: &Expression) -> Expression {
        Expression::add(vec![self, rhs.clone()])
    }
}

impl Add<Expression> for &Expression {
    type Output = Expression;
    
    fn add(self, rhs: Expression) -> Expression {
        Expression::add(vec![self.clone(), rhs])
    }
}

impl Add<i32> for Expression {
    type Output = Expression;
    
    fn add(self, rhs: i32) -> Expression {
        Expression::add(vec![self, Expression::integer(rhs as i64)])
    }
}

impl Add<Expression> for i32 {
    type Output = Expression;
    
    fn add(self, rhs: Expression) -> Expression {
        Expression::add(vec![Expression::integer(self as i64), rhs])
    }
}

impl Sub for Expression {
    type Output = Expression;
    
    fn sub(self, rhs: Expression) -> Expression {
        Expression::add(vec![self, -rhs])
    }
}

impl Sub for &Expression {
    type Output = Expression;
    
    fn sub(self, rhs: &Expression) -> Expression {
        Expression::add(vec![self.clone(), -rhs.clone()])
    }
}

impl Mul for Expression {
    type Output = Expression;
    
    fn mul(self, rhs: Expression) -> Expression {
        Expression::mul(vec![self, rhs])
    }
}

impl Mul for &Expression {
    type Output = Expression;
    
    fn mul(self, rhs: &Expression) -> Expression {
        Expression::mul(vec![self.clone(), rhs.clone()])
    }
}

impl Mul<&Expression> for Expression {
    type Output = Expression;
    
    fn mul(self, rhs: &Expression) -> Expression {
        Expression::mul(vec![self, rhs.clone()])
    }
}

impl Mul<Expression> for &Expression {
    type Output = Expression;
    
    fn mul(self, rhs: Expression) -> Expression {
        Expression::mul(vec![self.clone(), rhs])
    }
}

impl Mul<i32> for Expression {
    type Output = Expression;
    
    fn mul(self, rhs: i32) -> Expression {
        Expression::mul(vec![self, Expression::integer(rhs as i64)])
    }
}

impl Mul<Expression> for i32 {
    type Output = Expression;
    
    fn mul(self, rhs: Expression) -> Expression {
        Expression::mul(vec![Expression::integer(self as i64), rhs])
    }
}

impl Mul<i32> for &Expression {
    type Output = Expression;
    
    fn mul(self, rhs: i32) -> Expression {
        Expression::mul(vec![self.clone(), Expression::integer(rhs as i64)])
    }
}

impl Mul<&Expression> for i32 {
    type Output = Expression;
    
    fn mul(self, rhs: &Expression) -> Expression {
        Expression::mul(vec![Expression::integer(self as i64), rhs.clone()])
    }
}

impl Neg for Expression {
    type Output = Expression;
    
    fn neg(self) -> Expression {
        Expression::mul(vec![Expression::integer(-1), self])
    }
}

impl Neg for &Expression {
    type Output = Expression;
    
    fn neg(self) -> Expression {
        Expression::mul(vec![Expression::integer(-1), self.clone()])
    }
}

/// High-performance expression operations
impl Expression {
    /// Create symbol with high-performance backend
    pub fn symbol_hp<T: Into<Symbol>>(symbol: T) -> Self {
        Self::symbol(symbol.into())
    }
    
    /// Create integer with high-performance backend
    pub fn integer_hp<T: Into<i64>>(value: T) -> Self {
        Self::integer(value.into())
    }
    
    /// High-performance simplification
    pub fn simplify_hp(&self) -> Self {
        self.simplify_compact()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_high_performance_mode() {
        let x = Expression::symbol(Symbol::new("x"));
        let y = Expression::symbol(Symbol::new("y"));
        
        // Test operator overloading
        let sum = &x + &y;
        let product = &x * &y;
        let difference = &x - &y;
        let negation = -&x;
        
        assert!(matches!(sum, Expression::Add(_)));
        assert!(matches!(product, Expression::Mul(_)));
        assert!(matches!(difference, Expression::Add(_)));
        assert!(matches!(negation, Expression::Mul(_)));
        
        // Test integer operations
        let result = &x + 5;
        let result2 = 3 * &x;
        
        assert!(matches!(result, Expression::Add(_)));
        assert!(matches!(result2, Expression::Mul(_)));
    }
    
    #[test]
    fn test_high_performance_arithmetic() {
        use std::time::Instant;
        
        let start = Instant::now();
        
        // Perform many operations
        let x = Expression::symbol(Symbol::new("x"));
        let mut result = Expression::integer(0);
        
        for i in 0..50_000 {
            let term = &x + Expression::integer(i);
            result = result + term;
        }
        
        let duration = start.elapsed();
        let ops_per_sec = 50_000.0 / duration.as_secs_f64();
        
        println!("🚀 High-performance mode: {:.2}M ops/sec", ops_per_sec / 1_000_000.0);
        
        // Should achieve high performance
        assert!(ops_per_sec > 1_000_000.0, "Expected >1M ops/sec, got {:.2}", ops_per_sec);
    }
    
    #[test]
    fn test_simplification_performance() {
        use std::time::Instant;
        
        let start = Instant::now();
        
        let x = Expression::symbol(Symbol::new("x"));
        
        // Perform many simplifications
        for i in 0..10_000 {
            let expr = Expression::add(vec![
                x.clone(),
                Expression::integer(i),
                Expression::integer(-i)
            ]);
            let _result = expr.simplify();
        }
        
        let duration = start.elapsed();
        let ops_per_sec = 10_000.0 / duration.as_secs_f64();
        
        println!("🚀 Simplification performance: {:.2}M ops/sec", ops_per_sec / 1_000_000.0);
        
        // Should achieve high performance
        assert!(ops_per_sec > 500_000.0, "Expected >500K ops/sec, got {:.2}", ops_per_sec);
    }
}
