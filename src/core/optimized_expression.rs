//! Memory-optimized Expression representation with inlined small values
//! Reduces heap allocations for common cases like small integers

use crate::core::{Expression, Number, Symbol};
use serde::{Deserialize, Serialize};
use std::fmt;
use num_traits::ToPrimitive;

/// Memory-optimized expression that inlines small integers
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum OptimizedExpression {
    /// Small integers stored inline (-128 to 127)
    SmallInteger(i8),
    /// Large expressions stored normally
    Number(Number),
    Symbol(Symbol),
    Add(Vec<OptimizedExpression>),
    Mul(Vec<OptimizedExpression>),
    Pow(Box<OptimizedExpression>, Box<OptimizedExpression>),
    Function {
        name: String,
        args: Vec<OptimizedExpression>,
    },
}

impl OptimizedExpression {
    /// Convert from regular Expression
    pub fn from_expression(expr: &Expression) -> Self {
        match expr {
            Expression::Number(Number::Integer(i)) => {
                if let Some(small_int) = i.to_i64() {
                    if small_int >= -128 && small_int <= 127 {
                        return Self::SmallInteger(small_int as i8);
                    }
                }
                Self::Number(Number::Integer(i.clone()))
            },
            Expression::Number(n) => Self::Number(n.clone()),
            Expression::Symbol(s) => Self::Symbol(s.clone()),
            Expression::Add(terms) => {
                let opt_terms: Vec<OptimizedExpression> = terms.iter()
                    .map(|t| Self::from_expression(t))
                    .collect();
                Self::Add(opt_terms)
            },
            Expression::Mul(factors) => {
                let opt_factors: Vec<OptimizedExpression> = factors.iter()
                    .map(|f| Self::from_expression(f))
                    .collect();
                Self::Mul(opt_factors)
            },
            Expression::Pow(base, exp) => {
                Self::Pow(
                    Box::new(Self::from_expression(base)),
                    Box::new(Self::from_expression(exp))
                )
            },
            Expression::Function { name, args } => {
                let opt_args: Vec<OptimizedExpression> = args.iter()
                    .map(|a| Self::from_expression(a))
                    .collect();
                Self::Function {
                    name: name.clone(),
                    args: opt_args,
                }
            }
        }
    }
    
    /// Convert to regular Expression
    pub fn to_expression(&self) -> Expression {
        match self {
            OptimizedExpression::SmallInteger(i) => Expression::integer(*i as i64),
            OptimizedExpression::Number(n) => Expression::Number(n.clone()),
            OptimizedExpression::Symbol(s) => Expression::Symbol(s.clone()),
            OptimizedExpression::Add(terms) => {
                let expr_terms: Vec<Expression> = terms.iter()
                    .map(|t| t.to_expression())
                    .collect();
                Expression::add(expr_terms)
            },
            OptimizedExpression::Mul(factors) => {
                let expr_factors: Vec<Expression> = factors.iter()
                    .map(|f| f.to_expression())
                    .collect();
                Expression::mul(expr_factors)
            },
            OptimizedExpression::Pow(base, exp) => {
                Expression::pow(base.to_expression(), exp.to_expression())
            },
            OptimizedExpression::Function { name, args } => {
                let expr_args: Vec<Expression> = args.iter()
                    .map(|a| a.to_expression())
                    .collect();
                Expression::function(name.clone(), expr_args)
            }
        }
    }
    
    /// Fast integer value extraction
    #[inline(always)]
    pub fn to_i64(&self) -> Option<i64> {
        match self {
            OptimizedExpression::SmallInteger(i) => Some(*i as i64),
            OptimizedExpression::Number(Number::Integer(n)) => n.to_i64(),
            _ => None,
        }
    }
    
    /// Fast float value extraction
    #[inline(always)]
    pub fn to_f64(&self) -> Option<f64> {
        match self {
            OptimizedExpression::SmallInteger(i) => Some(*i as f64),
            OptimizedExpression::Number(n) => n.to_f64(),
            _ => None,
        }
    }
    
    /// Check if optimized
    pub fn is_optimized(&self) -> bool {
        matches!(self, OptimizedExpression::SmallInteger(_))
    }
}

/// Trait for performance-optimized operations
pub trait PerformanceOptimized {
    fn optimize(&self) -> OptimizedExpression;
    fn performance_simplify(&self) -> Self;
}

impl PerformanceOptimized for Expression {
    fn optimize(&self) -> OptimizedExpression {
        OptimizedExpression::from_expression(self)
    }
    
    fn performance_simplify(&self) -> Self {
        // Convert to optimized form, simplify, then convert back
        let optimized = self.optimize();
        let simplified_opt = optimized.to_expression(); // Would have optimized simplification
        simplified_opt
    }
}

impl fmt::Display for OptimizedExpression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OptimizedExpression::SmallInteger(i) => write!(f, "{}", i),
            OptimizedExpression::Number(n) => write!(f, "{}", n),
            OptimizedExpression::Symbol(s) => write!(f, "{}", s),
            OptimizedExpression::Add(terms) => {
                write!(f, "(")?;
                for (i, term) in terms.iter().enumerate() {
                    if i > 0 { write!(f, " + ")?; }
                    write!(f, "{}", term)?;
                }
                write!(f, ")")
            },
            OptimizedExpression::Mul(factors) => {
                write!(f, "(")?;
                for (i, factor) in factors.iter().enumerate() {
                    if i > 0 { write!(f, " * ")?; }
                    write!(f, "{}", factor)?;
                }
                write!(f, ")")
            },
            OptimizedExpression::Pow(base, exp) => {
                write!(f, "({})^({})", base, exp)
            },
            OptimizedExpression::Function { name, args } => {
                write!(f, "{}(", name)?;
                for (i, arg) in args.iter().enumerate() {
                    if i > 0 { write!(f, ", ")?; }
                    write!(f, "{}", arg)?;
                }
                write!(f, ")")
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_small_integer_optimization() {
        let expr = Expression::integer(42);
        let optimized = OptimizedExpression::from_expression(&expr);
        
        assert!(matches!(optimized, OptimizedExpression::SmallInteger(42)));
        assert!(optimized.is_optimized());
    }
    
    #[test]
    fn test_large_integer_fallback() {
        let expr = Expression::integer(1000);
        let optimized = OptimizedExpression::from_expression(&expr);
        
        assert!(matches!(optimized, OptimizedExpression::Number(_)));
        assert!(!optimized.is_optimized());
    }
    
    #[test]
    fn test_roundtrip_conversion() {
        let x = Symbol::new("x");
        let original = Expression::add(vec![
            Expression::symbol(x.clone()),
            Expression::integer(5)
        ]);
        
        let optimized = OptimizedExpression::from_expression(&original);
        let converted_back = optimized.to_expression();
        
        assert_eq!(original, converted_back);
    }
    
    #[test]
    fn test_performance_optimization() {
        use std::time::Instant;
        
        let start = Instant::now();
        
        // Create many small integer expressions
        for i in -100..100 {
            let expr = Expression::integer(i);
            let _optimized = OptimizedExpression::from_expression(&expr);
        }
        
        let duration = start.elapsed();
        println!("Optimization time: {:?}", duration);
        
        // Should be fast
        assert!(duration.as_millis() < 100);
    }
}
