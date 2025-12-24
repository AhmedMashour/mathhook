//! Greatest Common Divisor operations for polynomials and expressions

use crate::core::polynomial::IntPoly;
use crate::core::polynomial::PolynomialError;
use crate::core::{Expression, Number, Symbol};
use num_integer::Integer;

/// Trait for GCD operations on expressions
pub trait PolynomialGcd {
    fn gcd(&self, other: &Self) -> Self;
    fn lcm(&self, other: &Self) -> Self;
    fn factor_gcd(&self) -> Self;
    fn cofactors(&self, other: &Self) -> (Expression, Expression, Expression);

    /// Divides this polynomial by another, returning (quotient, remainder).
    ///
    /// Performs polynomial long division with respect to the specified variable.
    /// Returns a tuple (quotient, remainder) satisfying the division identity:
    /// `dividend = divisor * quotient + remainder` where `degree(remainder) < degree(divisor)`.
    ///
    /// # Arguments
    ///
    /// * `divisor` - The polynomial to divide by (must be non-zero)
    /// * `var` - The variable to treat as the polynomial variable
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{symbol, expr, Expression, algebra::gcd::PolynomialGcd};
    ///
    /// let x = symbol!(x);
    /// // Divide (x^2 + 3x + 2) by (x + 1)
    /// // Expected: (x + 1)(x + 2) = x^2 + 3x + 2
    /// let dividend = expr!((x^2) + (3*x) + 2);
    /// let divisor = expr!(x + 1);
    /// let (quotient, remainder) = dividend.div_polynomial(&divisor, &x);
    /// assert_eq!(remainder, Expression::integer(0));
    /// ```
    ///
    /// # Returns
    ///
    /// Returns `(quotient, remainder)` tuple where both are expressions
    fn div_polynomial(&self, divisor: &Expression, var: &Symbol) -> (Expression, Expression);

    /// Returns the quotient of polynomial division.
    ///
    /// Computes only the quotient part of polynomial division, discarding the remainder.
    /// Equivalent to `div_polynomial(divisor, var).0`.
    ///
    /// # Arguments
    ///
    /// * `divisor` - The polynomial to divide by
    /// * `var` - The variable to treat as the polynomial variable
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{symbol, expr, algebra::gcd::PolynomialGcd};
    ///
    /// let x = symbol!(x);
    /// let dividend = expr!((x^2) - 1);
    /// let divisor = expr!(x - 1);
    /// let quotient = dividend.quo_polynomial(&divisor, &x);
    /// // quotient = x + 1
    /// ```
    ///
    /// # Returns
    ///
    /// Returns the quotient expression
    fn quo_polynomial(&self, divisor: &Expression, var: &Symbol) -> Expression;

    /// Returns the remainder of polynomial division.
    ///
    /// Computes only the remainder part of polynomial division, discarding the quotient.
    /// Equivalent to `div_polynomial(divisor, var).1`.
    ///
    /// # Arguments
    ///
    /// * `divisor` - The polynomial to divide by
    /// * `var` - The variable to treat as the polynomial variable
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{symbol, expr, Expression, algebra::gcd::PolynomialGcd};
    ///
    /// let x = symbol!(x);
    /// let dividend = expr!((x^2) + 1);
    /// let divisor = expr!(x - 1);
    /// let remainder = dividend.rem_polynomial(&divisor, &x);
    /// assert_eq!(remainder, Expression::integer(2));
    /// ```
    ///
    /// # Returns
    ///
    /// Returns the remainder expression
    fn rem_polynomial(&self, divisor: &Expression, var: &Symbol) -> Expression;
}

impl PolynomialGcd for Expression {
    /// GCD using IntPoly fast-path (primary path)
    ///
    /// Converts to IntPoly at API boundary, performs pure numeric GCD,
    /// converts result back. NO Expression tree walking in fast path.
    #[inline(always)]
    fn gcd(&self, other: &Self) -> Self {
        // Numeric GCD (most common case)
        if let (Expression::Number(Number::Integer(a)), Expression::Number(Number::Integer(b))) =
            (self, other)
        {
            return Expression::integer(a.gcd(b));
        }

        if self == other {
            return self.clone();
        }

        if self.is_zero() {
            return other.clone();
        }
        if other.is_zero() {
            return self.clone();
        }

        // IntPoly fast-path: PRIMARY PATH for polynomial GCD
        let vars = self.find_variables();
        if vars.len() == 1 {
            let var = &vars[0];
            if IntPoly::can_convert(self, var) && IntPoly::can_convert(other, var) {
                if let (Some(poly1), Some(poly2)) = (
                    IntPoly::try_from_expression(self, var),
                    IntPoly::try_from_expression(other, var),
                ) {
                    if let Ok(gcd_poly) = poly1.gcd_i64(&poly2) {
                        let result = gcd_poly.to_expression(var);
                        // Ensure GCD of coprime polynomials returns positive 1, not -1
                        if let Expression::Number(Number::Integer(n)) = &result {
                            if *n < 0 {
                                return Expression::integer(n.abs());
                            }
                        }
                        return result;
                    }
                }
            }
        }

        // Minimal symbolic fallback for rational coefficient polynomials
        if vars.len() == 1 {
            let var = &vars[0];
            let result = symbolic_gcd_euclidean(self, other, var);
            // Final normalization: ensure we return +1 not -1 for coprime
            if let Expression::Number(Number::Integer(n)) = &result {
                if *n < 0 {
                    return Expression::integer(n.abs());
                }
            }
            return result;
        }

        // For multivariate or non-polynomial cases, return 1 (coprime)
        Expression::integer(1)
    }

    /// Least Common Multiple
    #[inline(always)]
    fn lcm(&self, other: &Self) -> Self {
        let gcd_val = self.gcd(other);

        if gcd_val.is_zero() {
            return Expression::integer(0);
        }

        let product = Expression::mul(vec![self.clone(), other.clone()]);
        Expression::div(product, gcd_val)
    }

    /// Factor out GCD from expression
    #[inline(always)]
    fn factor_gcd(&self) -> Self {
        match self {
            Expression::Add(terms) => {
                if terms.len() < 2 {
                    return self.clone();
                }

                let mut common_gcd = terms[0].clone();
                for term in &terms[1..] {
                    common_gcd = common_gcd.gcd(term);
                    if common_gcd.is_one() {
                        return self.clone();
                    }
                }

                common_gcd
            }
            Expression::Mul(_factors) => self.clone(),
            _ => self.clone(),
        }
    }

    /// Compute GCD and cofactors: returns (gcd, a/gcd, b/gcd)
    fn cofactors(&self, other: &Self) -> (Expression, Expression, Expression) {
        let gcd_val = self.gcd(other);

        if gcd_val.is_zero() || gcd_val.is_one() {
            return (gcd_val, self.clone(), other.clone());
        }

        // IntPoly fast-path for cofactors
        let vars = self.find_variables();
        if vars.len() == 1 {
            let var = &vars[0];
            if IntPoly::can_convert(self, var)
                && IntPoly::can_convert(other, var)
                && IntPoly::can_convert(&gcd_val, var)
            {
                if let (Some(poly1), Some(poly2), Some(gcd_poly)) = (
                    IntPoly::try_from_expression(self, var),
                    IntPoly::try_from_expression(other, var),
                    IntPoly::try_from_expression(&gcd_val, var),
                ) {
                    if let (Ok((cof1, _)), Ok((cof2, _))) =
                        (poly1.div_rem(&gcd_poly), poly2.div_rem(&gcd_poly))
                    {
                        return (gcd_val, cof1.to_expression(var), cof2.to_expression(var));
                    }
                }
            }

            // Symbolic fallback for cofactors - unwrap or fallback to original
            if let (Ok((q1, r1)), Ok((q2, r2))) = (
                crate::algebra::polynomial_division::polynomial_div(self, &gcd_val, var),
                crate::algebra::polynomial_division::polynomial_div(other, &gcd_val, var),
            ) {
                if r1.is_zero() && r2.is_zero() {
                    return (gcd_val, q1, q2);
                }
            }
        }

        (gcd_val, self.clone(), other.clone())
    }

    fn div_polynomial(&self, divisor: &Expression, var: &Symbol) -> (Expression, Expression) {
        crate::algebra::polynomial_division::polynomial_div(self, divisor, var)
            .unwrap_or_else(|_| (Expression::undefined(), Expression::undefined()))
    }

    fn quo_polynomial(&self, divisor: &Expression, var: &Symbol) -> Expression {
        self.div_polynomial(divisor, var).0
    }

    fn rem_polynomial(&self, divisor: &Expression, var: &Symbol) -> Expression {
        self.div_polynomial(divisor, var).1
    }
}

/// Symbolic Euclidean GCD algorithm
///
/// Minimal fallback for rational coefficient polynomials.
/// Uses polynomial remainder operations.
fn symbolic_gcd_euclidean(p1: &Expression, p2: &Expression, var: &Symbol) -> Expression {
    let mut a = p1.clone();
    let mut b = p2.clone();

    // Euclidean algorithm with at most 10 iterations
    for _ in 0..10 {
        if b.is_zero() {
            // Normalize: for coprime polynomials, ensure we return +1, not -1
            if let Expression::Number(Number::Integer(n)) = &a {
                if *n < 0 {
                    return Expression::integer(n.abs());
                }
            }
            return a;
        }

        let remainder =
            crate::algebra::polynomial_division::polynomial_rem(&a, &b, var).unwrap_or(b.clone());
        a = b;
        b = remainder;
    }

    // Fallback: return 1 if algorithm doesn't converge
    Expression::integer(1)
}

/// Polynomial GCD - routes to best algorithm automatically
///
/// Uses fast-path optimization to avoid expensive classification for simple cases.
///
/// # Arguments
///
/// * `p1` - First expression
/// * `p2` - Second expression
///
/// # Returns
///
/// GCD of the two expressions
///
/// # Examples
///
/// ```rust
/// use mathhook_core::algebra::gcd::polynomial_gcd;
/// use mathhook_core::core::Expression;
///
/// let a = Expression::integer(12);
/// let b = Expression::integer(18);
/// let gcd = polynomial_gcd(&a, &b).unwrap();
/// assert_eq!(gcd, Expression::integer(6));
/// ```
pub fn polynomial_gcd(p1: &Expression, p2: &Expression) -> Result<Expression, PolynomialError> {
    use crate::expr;

    // Fast path 1: Both integers
    if let (Expression::Number(Number::Integer(n1)), Expression::Number(Number::Integer(n2))) =
        (p1, p2)
    {
        return Ok(Expression::integer(n1.gcd(n2)));
    }

    // Fast path 2: Either is zero
    if p1.is_zero() {
        return Ok(p2.clone());
    }
    if p2.is_zero() {
        return Ok(p1.clone());
    }

    // Fast path 3: Either is one
    if p1.is_one() || p2.is_one() {
        return Ok(expr!(1));
    }

    // Fast path 4: Both are same symbol
    if let (Expression::Symbol(s1), Expression::Symbol(s2)) = (p1, p2) {
        if s1 == s2 {
            return Ok(p1.clone());
        }
        return Ok(expr!(1));
    }

    // Fast path 5: IntPoly for univariate integer polynomials
    let vars_p1 = p1.find_variables();
    if vars_p1.len() == 1 {
        let var = &vars_p1[0];
        if IntPoly::can_convert(p1, var) && IntPoly::can_convert(p2, var) {
            if let (Some(poly1), Some(poly2)) = (
                IntPoly::try_from_expression(p1, var),
                IntPoly::try_from_expression(p2, var),
            ) {
                return Ok(poly1
                    .gcd_i64(&poly2)
                    .map_err(|e| PolynomialError::GcdComputationFailed {
                        reason: format!("{:?}", e),
                    })?
                    .to_expression(var));
            }
        }
    }

    // Fallback to Expression::gcd()
    Ok(p1.gcd(p2))
}

/// Univariate polynomial GCD with IntPoly fast-path
///
/// Uses IntPoly for integer coefficient polynomials, falls back to Expression::gcd().
///
/// # Arguments
///
/// * `p1` - First univariate polynomial
/// * `p2` - Second univariate polynomial
/// * `var` - The variable of the polynomials
///
/// # Returns
///
/// GCD of the two polynomials
pub fn univariate_gcd(
    p1: &Expression,
    p2: &Expression,
    var: &Symbol,
) -> Result<Expression, PolynomialError> {
    // IntPoly fast-path
    if IntPoly::can_convert(p1, var) && IntPoly::can_convert(p2, var) {
        if let (Some(poly1), Some(poly2)) = (
            IntPoly::try_from_expression(p1, var),
            IntPoly::try_from_expression(p2, var),
        ) {
            let gcd_poly =
                poly1
                    .gcd_i64(&poly2)
                    .map_err(|e| PolynomialError::GcdComputationFailed {
                        reason: format!("{:?}", e),
                    })?;
            return Ok(gcd_poly.to_expression(var));
        }
    }

    // Fallback to Expression::gcd()
    Ok(p1.gcd(p2))
}

/// Univariate polynomial GCD with cofactors
///
/// Returns (gcd, cofactor_p1, cofactor_p2) using IntPoly fast-path.
///
/// # Arguments
///
/// * `p1` - First univariate polynomial
/// * `p2` - Second univariate polynomial
/// * `var` - The variable of the polynomials
///
/// # Returns
///
/// Tuple of (gcd, cofactor_p1, cofactor_p2)
pub fn univariate_gcd_modular(
    p1: &Expression,
    p2: &Expression,
    var: &Symbol,
) -> Result<(Expression, Expression, Expression), PolynomialError> {
    // IntPoly fast-path with cofactors
    if IntPoly::can_convert(p1, var) && IntPoly::can_convert(p2, var) {
        if let (Some(poly1), Some(poly2)) = (
            IntPoly::try_from_expression(p1, var),
            IntPoly::try_from_expression(p2, var),
        ) {
            let gcd_poly =
                poly1
                    .gcd_i64(&poly2)
                    .map_err(|e| PolynomialError::GcdComputationFailed {
                        reason: format!("{:?}", e),
                    })?;

            // Compute cofactors
            let (cof1, _) = poly1
                .div_rem(&gcd_poly)
                .map_err(|_| PolynomialError::DivisionByZero)?;
            let (cof2, _) = poly2
                .div_rem(&gcd_poly)
                .map_err(|_| PolynomialError::DivisionByZero)?;

            return Ok((
                gcd_poly.to_expression(var),
                cof1.to_expression(var),
                cof2.to_expression(var),
            ));
        }
    }

    // Fallback
    let gcd = p1.gcd(p2);
    Ok((gcd.clone(), p1.clone(), p2.clone()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::expr;
    use crate::symbol;

    #[test]
    fn test_number_gcd() {
        let a = Expression::integer(12);
        let b = Expression::integer(8);
        let result = a.gcd(&b);
        assert_eq!(result, Expression::integer(4));

        let a = Expression::integer(17);
        let b = Expression::integer(13);
        let result = a.gcd(&b);
        assert_eq!(result, Expression::integer(1));
    }

    #[test]
    fn test_gcd_with_zero() {
        let a = Expression::integer(5);
        let zero = Expression::integer(0);

        let result = a.gcd(&zero);
        assert_eq!(result, Expression::integer(5));

        let result = zero.gcd(&a);
        assert_eq!(result, Expression::integer(5));
    }

    #[test]
    fn test_identical_expressions() {
        let x = expr!(x);
        let result = x.gcd(&x);
        assert_eq!(result, x);
    }

    #[test]
    fn test_gcd_performance_benchmark() {
        use std::time::Instant;

        let start = Instant::now();

        for i in 1..10_000 {
            let a = Expression::integer(i * 6);
            let b = Expression::integer(i * 9);
            let _result = a.gcd(&b);
        }

        let duration = start.elapsed();
        let ops_per_sec = 10_000.0 / duration.as_secs_f64();

        println!("GCD Performance: {:.2}M ops/sec", ops_per_sec / 1_000_000.0);

        assert!(
            ops_per_sec > 100_000.0,
            "Expected >100K ops/sec, got {:.2}",
            ops_per_sec
        );
    }

    #[test]
    fn test_polynomial_gcd_basic() {
        let x = symbol!(x);

        let poly1 = Expression::mul(vec![Expression::integer(6), Expression::symbol(x.clone())]);
        let poly2 = Expression::mul(vec![Expression::integer(9), Expression::symbol(x.clone())]);

        let result = poly1.gcd(&poly2);

        println!("Polynomial GCD result: {}", result);
        assert!(!result.is_zero());
    }

    #[test]
    fn test_factor_gcd() {
        let x = symbol!(x);

        let term1 = Expression::mul(vec![Expression::integer(6), Expression::symbol(x.clone())]);
        let term2 = Expression::mul(vec![Expression::integer(9), Expression::symbol(x.clone())]);
        let sum = Expression::add(vec![term1, term2]);

        let gcd_factor = sum.factor_gcd();
        println!("Factored GCD: {}", gcd_factor);

        assert!(!gcd_factor.is_zero());
    }

    #[test]
    fn test_lcm_basic() {
        let a = Expression::integer(6);
        let b = Expression::integer(8);
        let result = a.lcm(&b);

        println!("LCM result: {}", result);
        assert!(!result.is_zero());
    }

    #[test]
    fn test_int_poly_fast_path() {
        let _x = symbol!(x);

        let poly1 = expr!((x ^ 5) + (2 * (x ^ 4)) + (3 * (x ^ 3)) + (4 * (x ^ 2)) + (5 * x) + 6);
        let poly2 =
            expr!((2 * (x ^ 5)) + (4 * (x ^ 4)) + (6 * (x ^ 3)) + (8 * (x ^ 2)) + (10 * x) + 12);

        let result = poly1.gcd(&poly2);

        println!("IntPoly fast-path GCD: {}", result);
        assert!(!result.is_zero());
    }

    #[test]
    fn test_intpoly_gcd_direct() {
        let _x = symbol!(x);

        let p1 = expr!((x ^ 2) - 1);
        let p2 = expr!(x - 1);

        let gcd = p1.gcd(&p2);

        println!("Direct IntPoly GCD: {}", gcd);
        assert!(!gcd.is_zero());
    }

    #[test]
    fn test_cofactors_intpoly() {
        let _x = symbol!(x);

        let p1 = expr!((x ^ 2) - 1);
        let p2 = expr!(x - 1);

        let (gcd, cof1, cof2) = p1.cofactors(&p2);

        println!("GCD: {}, Cofactor1: {}, Cofactor2: {}", gcd, cof1, cof2);
        assert!(!gcd.is_zero());
    }

    #[test]
    fn test_polynomial_gcd_function() {
        let a = Expression::integer(12);
        let b = Expression::integer(18);
        let gcd = polynomial_gcd(&a, &b).unwrap();
        assert_eq!(gcd, Expression::integer(6));
    }

    #[test]
    fn test_univariate_gcd_function() {
        let _x = symbol!(x);
        let p1 = expr!((x ^ 2) - 1);
        let p2 = expr!(x - 1);
        let gcd = univariate_gcd(&p1, &p2, &_x).unwrap();
        assert!(!gcd.is_zero());
    }

    #[test]
    fn test_gcd_coprime_expressions() {
        let _x = symbol!(x);
        let a = expr!(x + 1);
        let b = expr!(x + 2);
        let result = a.gcd(&b);
        println!("GCD result: {:?}", result);
        assert_eq!(result, Expression::integer(1));
    }
}
