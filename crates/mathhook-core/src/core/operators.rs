//! Operator overloading for expressions to enable natural mathematical syntax

use crate::core::Expression;
use std::ops::{Add, Sub, Mul, Neg};

// Addition operators
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

// Integer addition
impl Add<i32> for Expression {
    type Output = Expression;
    
    fn add(self, rhs: i32) -> Expression {
        Expression::add(vec![self, Expression::integer(rhs)])
    }
}

impl Add<Expression> for i32 {
    type Output = Expression;
    
    fn add(self, rhs: Expression) -> Expression {
        Expression::add(vec![Expression::integer(self), rhs])
    }
}

impl Add<i32> for &Expression {
    type Output = Expression;
    
    fn add(self, rhs: i32) -> Expression {
        Expression::add(vec![self.clone(), Expression::integer(rhs)])
    }
}

impl Add<&Expression> for i32 {
    type Output = Expression;
    
    fn add(self, rhs: &Expression) -> Expression {
        Expression::add(vec![Expression::integer(self), rhs.clone()])
    }
}

// Subtraction operators
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

impl Sub<&Expression> for Expression {
    type Output = Expression;
    
    fn sub(self, rhs: &Expression) -> Expression {
        Expression::add(vec![self, -rhs.clone()])
    }
}

impl Sub<Expression> for &Expression {
    type Output = Expression;
    
    fn sub(self, rhs: Expression) -> Expression {
        Expression::add(vec![self.clone(), -rhs])
    }
}

// Integer subtraction
impl Sub<i32> for Expression {
    type Output = Expression;
    
    fn sub(self, rhs: i32) -> Expression {
        Expression::add(vec![self, Expression::integer(-rhs)])
    }
}

impl Sub<Expression> for i32 {
    type Output = Expression;
    
    fn sub(self, rhs: Expression) -> Expression {
        Expression::add(vec![Expression::integer(self), -rhs])
    }
}

// Multiplication operators
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

// Integer multiplication
impl Mul<i32> for Expression {
    type Output = Expression;
    
    fn mul(self, rhs: i32) -> Expression {
        Expression::mul(vec![self, Expression::integer(rhs)])
    }
}

impl Mul<Expression> for i32 {
    type Output = Expression;
    
    fn mul(self, rhs: Expression) -> Expression {
        Expression::mul(vec![Expression::integer(self), rhs])
    }
}

impl Mul<i32> for &Expression {
    type Output = Expression;
    
    fn mul(self, rhs: i32) -> Expression {
        Expression::mul(vec![self.clone(), Expression::integer(rhs)])
    }
}

impl Mul<&Expression> for i32 {
    type Output = Expression;
    
    fn mul(self, rhs: &Expression) -> Expression {
        Expression::mul(vec![Expression::integer(self), rhs.clone()])
    }
}

// Negation operator
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::Symbol;

    #[test]
    fn test_addition() {
        let x = Expression::symbol(Symbol::new("x"));
        let y = Expression::symbol(Symbol::new("y"));
        let sum = &x + &y;
        
        assert!(matches!(sum, Expression::Add(_)));
    }
    
    #[test]
    fn test_multiplication() {
        let x = Expression::symbol(Symbol::new("x"));
        let two = Expression::integer(2);
        let product = &x * &two;
        
        assert!(matches!(product, Expression::Mul(_)));
    }
    
    #[test]
    fn test_integer_ops() {
        let x = Expression::symbol(Symbol::new("x"));
        let sum = &x + 5;
        let product = 3 * &x;
        
        assert!(matches!(sum, Expression::Add(_)));
        assert!(matches!(product, Expression::Mul(_)));
    }
    
    #[test]
    fn test_negation() {
        let x = Expression::symbol(Symbol::new("x"));
        let neg_x = -&x;
        
        assert!(matches!(neg_x, Expression::Mul(_)));
    }
}
