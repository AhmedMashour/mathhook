//! Arithmetic utility methods for expressions
//!
//! This module provides GCD, LCM, factorization, and related utilities
//! for mathematical expressions.
use super::super::Expression;
use crate::core::polynomial::IntPoly;
use crate::core::Number;
use crate::expr;
use std::collections::HashSet;

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
    /// For integer expressions, computes the mathematical GCD.
    /// For polynomial expressions with integer coefficients, uses fast IntPoly algorithm.
    /// For other expressions, returns 1.
    ///
    /// # Arguments
    ///
    /// * `other` - The expression to compute GCD with
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::expr;
    ///
    /// let a = expr!(12);
    /// let b = expr!(8);
    /// let gcd = a.gcd(&b);
    /// assert_eq!(gcd, expr!(4));
    /// ```
    ///
    /// ```rust
    /// use mathhook_core::expr;
    ///
    /// // GCD of zero with any number is that number
    /// let a = expr!(0);
    /// let b = expr!(15);
    /// let gcd = a.gcd(&b);
    /// assert_eq!(gcd, expr!(15));
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
                _ => expr!(1),
            },
            _ => {
                // IntPoly fast-path for univariate integer polynomials
                let vars = self.find_variables();
                if vars.len() == 1 {
                    let var = &vars[0];
                    if IntPoly::can_convert(self, var) && IntPoly::can_convert(other, var) {
                        if let (Some(poly1), Some(poly2)) = (
                            IntPoly::try_from_expression(self, var),
                            IntPoly::try_from_expression(other, var),
                        ) {
                            // Use gcd_i64 which normalizes the result (positive leading coeff)
                            if let Ok(gcd_poly) = poly1.gcd_i64(&poly2) {
                                return gcd_poly.to_expression(var);
                            }
                        }
                    }
                }
                expr!(1)
            }
        }
    }

    /// Compute the least common multiple of two expressions
    ///
    /// For integer expressions, computes the mathematical LCM.
    /// For other expressions, returns the first expression.
    ///
    /// # Arguments
    ///
    /// * `other` - The expression to compute LCM with
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::expr;
    ///
    /// let a = expr!(12);
    /// let b = expr!(8);
    /// let lcm = a.lcm(&b);
    /// assert_eq!(lcm, expr!(24));
    /// ```
    ///
    /// ```rust
    /// use mathhook_core::expr;
    ///
    /// // LCM with zero is zero
    /// let a = expr!(0);
    /// let b = expr!(15);
    /// let lcm = a.lcm(&b);
    /// assert_eq!(lcm, expr!(0));
    /// ```
    pub fn lcm(&self, other: &Expression) -> Expression {
        match (self, other) {
            (Expression::Number(num1), Expression::Number(num2)) => match (num1, num2) {
                (Number::Integer(a), Number::Integer(b)) => {
                    if *a == 0 || *b == 0 {
                        expr!(0)
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
    /// Currently a stub that returns the original expression.
    /// Will be implemented to extract common factors.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{expr, Expression};
    ///
    /// let expr = Expression::add(vec![
    ///     expr!(6*x),
    ///     expr!(9),
    /// ]);
    /// let factored = expr.factor_gcd();
    /// ```
    pub fn factor_gcd(&self) -> Expression {
        self.clone()
    }

    /// Compute GCD and cofactors
    ///
    /// Returns a tuple of (gcd, cofactor_a, cofactor_b) where:
    /// - gcd is the greatest common divisor
    /// - cofactor_a = a / gcd
    /// - cofactor_b = b / gcd
    ///
    /// # Arguments
    ///
    /// * `other` - The expression to compute cofactors with
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::expr;
    ///
    /// let a = expr!(12);
    /// let b = expr!(8);
    /// let (gcd, cofactor_a, cofactor_b) = a.cofactors(&b);
    /// assert_eq!(gcd, expr!(4));
    /// assert_eq!(cofactor_a, expr!(3));
    /// assert_eq!(cofactor_b, expr!(2));
    /// ```
    pub fn cofactors(&self, other: &Expression) -> (Expression, Expression, Expression) {
        let gcd = self.gcd(other);

        // IntPoly fast-path for polynomial cofactors
        let vars = self.find_variables();
        if vars.len() == 1 {
            let var = &vars[0];
            let other_vars = other.find_variables();
            if other_vars.len() == 1
                && &other_vars[0] == var
                && IntPoly::can_convert(self, var)
                && IntPoly::can_convert(other, var)
                && IntPoly::can_convert(&gcd, var)
            {
                if let (Some(p_self), Some(p_other), Some(p_gcd)) = (
                    IntPoly::try_from_expression(self, var),
                    IntPoly::try_from_expression(other, var),
                    IntPoly::try_from_expression(&gcd, var),
                ) {
                    if let (Ok((cofactor_self, rem1)), Ok((cofactor_other, rem2))) =
                        (p_self.div_rem(&p_gcd), p_other.div_rem(&p_gcd))
                    {
                        if rem1.is_zero() && rem2.is_zero() {
                            return (
                                gcd,
                                cofactor_self.to_expression(var),
                                cofactor_other.to_expression(var),
                            );
                        }
                    }
                }
            }
        }

        match (&gcd, self, other) {
            (
                Expression::Number(Number::Integer(g)),
                Expression::Number(Number::Integer(a)),
                Expression::Number(Number::Integer(b)),
            ) if *g != 0 => {
                let cofactor_a = Expression::integer(a / g);
                let cofactor_b = Expression::integer(b / g);
                (gcd, cofactor_a, cofactor_b)
            }
            _ => {
                let cofactor_a =
                    Expression::mul(vec![self.clone(), Expression::pow(gcd.clone(), expr!(-1))]);
                let cofactor_b =
                    Expression::mul(vec![other.clone(), Expression::pow(gcd.clone(), expr!(-1))]);
                (gcd, cofactor_a, cofactor_b)
            }
        }
    }

    /// Find all variables in expression
    ///
    /// Returns a vector of all unique Symbol nodes found in the expression tree.
    /// This is used by GCD algorithms to detect univariate polynomials.
    pub fn find_variables(&self) -> Vec<crate::Symbol> {
        fn collect_symbols(expr: &Expression, symbols: &mut HashSet<crate::Symbol>) {
            match expr {
                Expression::Symbol(s) => {
                    symbols.insert(s.clone());
                }
                Expression::Add(terms) | Expression::Mul(terms) => {
                    for term in terms.iter() {
                        collect_symbols(term, symbols);
                    }
                }
                Expression::Pow(base, exp) => {
                    collect_symbols(base, symbols);
                    collect_symbols(exp, symbols);
                }
                Expression::Function { args, .. } => {
                    for arg in args.iter() {
                        collect_symbols(arg, symbols);
                    }
                }
                _ => {}
            }
        }

        let mut symbols = HashSet::new();
        collect_symbols(self, &mut symbols);
        symbols.into_iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gcd_basic() {
        let a = expr!(12);
        let b = expr!(8);
        assert_eq!(a.gcd(&b), expr!(4));
    }

    #[test]
    fn test_gcd_with_zero() {
        let a = expr!(0);
        let b = expr!(15);
        assert_eq!(a.gcd(&b), expr!(15));
        assert_eq!(b.gcd(&a), expr!(15));
    }

    #[test]
    fn test_lcm_basic() {
        let a = expr!(12);
        let b = expr!(8);
        assert_eq!(a.lcm(&b), expr!(24));
    }

    #[test]
    fn test_lcm_with_zero() {
        let a = expr!(0);
        let b = expr!(15);
        assert_eq!(a.lcm(&b), expr!(0));
    }

    #[test]
    fn test_cofactors() {
        let a = expr!(12);
        let b = expr!(8);
        let (gcd, cof_a, cof_b) = a.cofactors(&b);
        assert_eq!(gcd, expr!(4));
        assert_eq!(cof_a, expr!(3));
        assert_eq!(cof_b, expr!(2));
    }
}
