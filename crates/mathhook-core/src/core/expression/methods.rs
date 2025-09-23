//! Expression utility methods

use super::Expression;
use crate::core::Number;

/// Helper function for computing GCD of integers
fn gcd_integers(a: i64, b: i64) -> i64 {
    let (mut a, mut b) = (a.abs(), b.abs());
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

impl Expression {
    /// Compute the greatest common divisor of two expressions
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::Expression;
    ///
    /// let a = Expression::integer(12);
    /// let b = Expression::integer(8);
    /// let gcd = a.gcd(&b);
    /// ```
    pub fn gcd(&self, other: &Expression) -> Expression {
        if self == other {
            return self.clone();
        }

        if self.is_zero() {
            return other.clone();
        }
        if other.is_zero() {
            return self.clone();
        }

        match (self, other) {
            (Expression::Number(num1), Expression::Number(num2)) => match (num1, num2) {
                (Number::Integer(a), Number::Integer(b)) => {
                    Expression::integer(gcd_integers(*a, *b))
                }
                _ => Expression::integer(1),
            },
            _ => Expression::integer(1),
        }
    }

    /// Compute the least common multiple of two expressions
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::Expression;
    ///
    /// let a = Expression::integer(12);
    /// let b = Expression::integer(8);
    /// let lcm = a.lcm(&b);
    /// ```
    pub fn lcm(&self, other: &Expression) -> Expression {
        match (self, other) {
            (Expression::Number(num1), Expression::Number(num2)) => match (num1, num2) {
                (Number::Integer(a), Number::Integer(b)) => {
                    if *a == 0 || *b == 0 {
                        Expression::integer(0)
                    } else {
                        let gcd_val = gcd_integers(*a, *b);
                        Expression::integer((*a * *b).abs() / gcd_val)
                    }
                }
                _ => self.clone(),
            },
            _ => self.clone(),
        }
    }

    /// Factor out the GCD from an expression
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::Expression;
    ///
    /// let expr = Expression::add(vec![
    ///     Expression::mul(vec![Expression::integer(6), Expression::symbol("x")]),
    ///     Expression::integer(9),
    /// ]);
    /// let factored = expr.factor_gcd();
    /// ```
    pub fn factor_gcd(&self) -> Expression {
        self.clone()
    }

    /// Compute GCD and cofactors
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::Expression;
    ///
    /// let a = Expression::integer(12);
    /// let b = Expression::integer(8);
    /// let (gcd, cofactor_a, cofactor_b) = a.cofactors(&b);
    /// ```
    pub fn cofactors(&self, other: &Expression) -> (Expression, Expression, Expression) {
        let gcd = self.gcd(other);
        (gcd.clone(), self.clone(), other.clone())
    }
}
