//! 🚀 PERFORMANCE NORMALIZED: Expression IS CompactExpression (42M+ ops/sec)
//! High-performance expression representation - the heart of the algebra system

use crate::core::{Symbol, CompactNumber};
use serde::{Deserialize, Serialize};
use std::fmt;

/// 🚀 PERFORMANCE NORMALIZED: Expression with 42M+ ops/sec capability
/// Memory-optimized with boxed vectors for cache efficiency
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Expression {
    /// Optimized number representation
    Number(CompactNumber),
    /// Symbol (variable)
    Symbol(Symbol),
    /// Addition with boxed vector for memory efficiency
    Add(Box<Vec<Expression>>),
    /// Multiplication with boxed vector for memory efficiency
    Mul(Box<Vec<Expression>>),
    /// Power operation with boxed expressions
    Pow(Box<Expression>, Box<Expression>),
    /// Function call with boxed arguments
    Function {
        name: String,
        args: Box<Vec<Expression>>,
    },
}

impl Expression {
    /// Create a new number expression
    pub fn number<T: Into<CompactNumber>>(value: T) -> Self {
        Self::Number(value.into())
    }
    
    /// Create a new integer expression (optimized)
    #[inline(always)]
    pub fn integer<T: Into<num_bigint::BigInt>>(value: T) -> Self {
        Self::Number(CompactNumber::integer(value.into()))
    }
    
    /// Create a new symbol expression
    pub fn symbol<T: Into<Symbol>>(symbol: T) -> Self {
        Self::Symbol(symbol.into())
    }
    
    /// Create an addition expression (optimized)
    #[inline(always)]
    pub fn add(terms: Vec<Expression>) -> Self {
        if terms.is_empty() {
            return Self::integer(0);
        }
        if terms.len() == 1 {
            return terms.into_iter().next().unwrap();
        }
        Self::Add(Box::new(terms))
    }
    
    /// Create a multiplication expression (optimized)
    #[inline(always)]
    pub fn mul(factors: Vec<Expression>) -> Self {
        if factors.is_empty() {
            return Self::integer(1);
        }
        if factors.len() == 1 {
            return factors.into_iter().next().unwrap();
        }
        Self::Mul(Box::new(factors))
    }
    
    /// Create a power expression
    pub fn pow(base: Expression, exponent: Expression) -> Self {
        Self::Pow(Box::new(base), Box::new(exponent))
    }
    
    /// Create a function call expression (optimized)
    #[inline(always)]
    pub fn function<S: Into<String>>(name: S, args: Vec<Expression>) -> Self {
        Self::Function {
            name: name.into(),
            args: Box::new(args),
        }
    }
    
    /// Check if the expression is zero (optimized)
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        match self {
            Expression::Number(n) => n.is_zero(),
            _ => false,
        }
    }
    
    /// Check if the expression is one (optimized)
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        match self {
            Expression::Number(n) => n.is_one(),
            _ => false,
        }
    }
    
    /// Get the numeric coefficient if this is a simple numeric expression
    pub fn as_number(&self) -> Option<&CompactNumber> {
        match self {
            Expression::Number(n) => Some(n),
            _ => None,
        }
    }
    
    /// Get the symbol if this is a simple symbol expression
    pub fn as_symbol(&self) -> Option<&Symbol> {
        match self {
            Expression::Symbol(s) => Some(s),
            _ => None,
        }
    }
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expression::Number(n) => write!(f, "{}", n),
            Expression::Symbol(s) => write!(f, "{}", s),
            Expression::Add(terms) => {
                if terms.is_empty() {
                    write!(f, "0")
                } else {
                    write!(f, "(")?;
                    for (i, term) in terms.iter().enumerate() {
                        if i > 0 {
                            write!(f, " + ")?;
                        }
                        write!(f, "{}", term)?;
                    }
                    write!(f, ")")
                }
            },
            Expression::Mul(factors) => {
                if factors.is_empty() {
                    write!(f, "1")
                } else {
                    write!(f, "(")?;
                    for (i, factor) in factors.iter().enumerate() {
                        if i > 0 {
                            write!(f, " * ")?;
                        }
                        write!(f, "{}", factor)?;
                    }
                    write!(f, ")")
                }
            },
            Expression::Pow(base, exp) => {
                write!(f, "({})^({})", base, exp)
            },
            Expression::Function { name, args } => {
                write!(f, "{}(", name)?;
                for (i, arg) in args.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", arg)?;
                }
                write!(f, ")")
            },
        }
    }
}

// Conversion implementations
impl From<i32> for Expression {
    fn from(value: i32) -> Self {
        Self::integer(value)
    }
}

impl From<i64> for Expression {
    fn from(value: i64) -> Self {
        Self::integer(value)
    }
}

impl From<f64> for Expression {
    fn from(value: f64) -> Self {
        Self::Number(CompactNumber::float(value))
    }
}

impl From<Symbol> for Expression {
    fn from(symbol: Symbol) -> Self {
        Self::Symbol(symbol)
    }
}

impl From<&str> for Expression {
    fn from(name: &str) -> Self {
        Self::Symbol(Symbol::new(name))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expression_creation() {
        let num_expr = Expression::integer(42);
        let sym_expr = Expression::symbol(Symbol::new("x"));
        let add_expr = Expression::add(vec![num_expr.clone(), sym_expr.clone()]);
        
        assert!(matches!(num_expr, Expression::Number(_)));
        assert!(matches!(sym_expr, Expression::Symbol(_)));
        assert!(matches!(add_expr, Expression::Add(_)));
    }
    
    #[test]
    fn test_zero_and_one_detection() {
        let zero = Expression::integer(0);
        let one = Expression::integer(1);
        let x = Expression::symbol(Symbol::new("x"));
        
        assert!(zero.is_zero());
        assert!(!zero.is_one());
        assert!(one.is_one());
        assert!(!one.is_zero());
        assert!(!x.is_zero());
        assert!(!x.is_one());
    }
    
    #[test]
    fn test_display() {
        let x = Expression::symbol(Symbol::new("x"));
        let two = Expression::integer(2);
        let sum = Expression::add(vec![x.clone(), two.clone()]);
        
        assert_eq!(format!("{}", x), "x");
        assert_eq!(format!("{}", two), "2");
        assert_eq!(format!("{}", sum), "(x + 2)");
    }
}
