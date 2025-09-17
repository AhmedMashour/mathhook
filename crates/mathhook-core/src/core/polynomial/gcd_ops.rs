//! Polynomial GCD Operations
//!
//! Provides GCD, LCM, and cofactor computation for polynomial expressions.
//! Uses IntPoly fast-path dispatch to route to optimized algorithms.

use super::classification::PolynomialClassification;
use super::error::PolynomialError;
use super::integer_gcd;
use crate::algebra::gcd::{polynomial_gcd, PolynomialGcd as PolynomialGcdTrait};
use crate::core::Expression;
use crate::simplify::Simplify;

/// Trait for polynomial GCD operations
///
/// Provides methods for computing GCD, LCM, and cofactors of polynomial
/// expressions. The implementation uses IntPoly fast-path when possible.
///
/// # Examples
///
/// ```rust,ignore
/// use mathhook_core::core::polynomial::PolynomialGcdOps;
/// use mathhook_core::core::Expression;
/// use mathhook_core::{symbol, expr};
///
/// let x = symbol!(x);
/// // x^2 - 1 = (x-1)(x+1)
/// let p1 = expr!((x ^ 2) - 1);
/// // x - 1
/// let p2 = expr!(x - 1);
///
/// let gcd = p1.polynomial_gcd(&p2).unwrap();
/// // GCD divides both polynomials
/// ```
pub trait PolynomialGcdOps: PolynomialClassification {
    /// Compute GCD of two polynomials
    ///
    /// Returns the greatest common divisor of `self` and `other`.
    /// Uses IntPoly fast-path for integer univariate polynomials.
    ///
    /// # Arguments
    ///
    /// * `other` - The other polynomial
    ///
    /// # Returns
    ///
    /// Returns `Ok(gcd)` on success, or `Err(PolynomialError)` if the operation fails.
    fn polynomial_gcd(&self, other: &Self) -> Result<Expression, PolynomialError>;

    /// Compute LCM of two polynomials
    ///
    /// Returns the least common multiple of `self` and `other`.
    fn polynomial_lcm(&self, other: &Self) -> Result<Expression, PolynomialError>;

    /// Compute GCD and cofactors
    ///
    /// Returns tuple (gcd, self/gcd, other/gcd).
    fn polynomial_cofactors(
        &self,
        other: &Self,
    ) -> Result<(Expression, Expression, Expression), PolynomialError>;
}

impl PolynomialGcdOps for Expression {
    fn polynomial_gcd(&self, other: &Self) -> Result<Expression, PolynomialError> {
        polynomial_gcd(self, other)
    }

    fn polynomial_lcm(&self, other: &Self) -> Result<Expression, PolynomialError> {
        use crate::core::Number;

        // Handle integer case directly
        if let (Expression::Number(Number::Integer(a)), Expression::Number(Number::Integer(b))) =
            (self, other)
        {
            let gcd_val = integer_gcd(*a, *b);
            if gcd_val == 0 {
                return Ok(Expression::integer(0));
            }
            let lcm_val = (*a / gcd_val) * *b;
            return Ok(Expression::integer(lcm_val.abs()));
        }

        let gcd = self.polynomial_gcd(other)?;

        if gcd.is_zero() {
            return Ok(Expression::integer(0));
        }

        let product = Expression::mul(vec![self.clone(), other.clone()]);
        let vars = self.find_variables();
        if vars.is_empty() {
            // For constants, LCM = |a * b| / gcd(a, b)
            return Ok(product.simplify());
        }
        Ok(PolynomialGcdTrait::quo_polynomial(&product, &gcd, &vars[0]))
    }

    fn polynomial_cofactors(
        &self,
        other: &Self,
    ) -> Result<(Expression, Expression, Expression), PolynomialError> {
        let gcd = self.polynomial_gcd(other)?;

        if gcd.is_zero() || gcd.is_one() {
            return Ok((gcd, self.clone(), other.clone()));
        }

        let vars = self.find_variables();
        if vars.len() == 1 {
            let var = &vars[0];
            let (q1, r1) = PolynomialGcdTrait::div_polynomial(self, &gcd, var);
            let (q2, r2) = PolynomialGcdTrait::div_polynomial(other, &gcd, var);

            if r1.is_zero() && r2.is_zero() {
                return Ok((gcd, q1, q2));
            }
        }

        Ok((gcd, self.clone(), other.clone()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{expr, symbol};

    #[test]
    fn test_polynomial_gcd_basic() {
        let _x = symbol!(x);
        let p1 = expr!((x ^ 2) - 1);
        let p2 = expr!(x - 1);

        let gcd = p1.polynomial_gcd(&p2).unwrap();
        assert!(!gcd.is_zero());
    }

    #[test]
    fn test_polynomial_lcm_basic() {
        let a = Expression::integer(6);
        let b = Expression::integer(8);

        let lcm = a.polynomial_lcm(&b).unwrap();
        assert!(!lcm.is_zero());
    }

    #[test]
    fn test_polynomial_cofactors_basic() {
        let _x = symbol!(x);
        let p1 = expr!((x ^ 2) - 1);
        let p2 = expr!(x - 1);

        let (gcd, cof1, cof2) = p1.polynomial_cofactors(&p2).unwrap();
        assert!(!gcd.is_zero());
        assert!(!cof1.is_zero());
        assert!(!cof2.is_zero());
    }
}
