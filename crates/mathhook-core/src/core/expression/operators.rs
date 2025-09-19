//! Operator overloading for Expression

use super::Expression;
use std::ops::{Add, Mul};

impl Add for Expression {
    type Output = Expression;

    fn add(self, rhs: Expression) -> Expression {
        Expression::add(vec![self, rhs])
    }
}

impl Mul for Expression {
    type Output = Expression;

    fn mul(self, rhs: Expression) -> Expression {
        Expression::multiply(self, rhs)
    }
}

impl Mul<Expression> for i64 {
    type Output = Expression;

    fn mul(self, rhs: Expression) -> Expression {
        Expression::multiply(Expression::integer(self), rhs)
    }
}

impl Mul<i64> for Expression {
    type Output = Expression;

    fn mul(self, rhs: i64) -> Expression {
        Expression::multiply(self, Expression::integer(rhs))
    }
}
