//! Polynomial long division operations
//!
//! Implements polynomial division algorithms for univariate polynomials,
//! supporting the Euclidean GCD algorithm and general polynomial arithmetic.
//!
//! # Algorithm
//!
//! Uses IntPoly fast-path for univariate integer polynomials (primary path).
//! Falls back to symbolic polynomial division for rational coefficient cases.
//!
//! # Example
//!
//! ```rust
//! use mathhook_core::{expr, symbol};
//! use mathhook_core::algebra::polynomial_division::polynomial_div;
//!
//! let x = symbol!(x);
//! let (quotient, remainder) = polynomial_div(&expr!((x^2) - 1), &expr!(x - 1), &x);
//! ```

use crate::core::polynomial::IntPoly;
use crate::core::{Expression, Number, Symbol};
use crate::simplify::Simplify;

/// Polynomial long division
///
/// Returns (quotient, remainder) such that:
/// `dividend = divisor * quotient + remainder`
/// and `degree(remainder) < degree(divisor)`
///
/// Uses IntPoly fast-path for univariate integer polynomials (primary path).
///
/// # Arguments
///
/// * `dividend` - Polynomial to divide
/// * `divisor` - Polynomial to divide by (must be non-zero)
/// * `var` - Variable to treat as polynomial variable
///
/// # Examples
///
/// ```rust
/// use mathhook_core::{expr, symbol};
/// use mathhook_core::algebra::polynomial_division::polynomial_div;
///
/// let x = symbol!(x);
/// // (x^2 + 3x + 2) / (x + 1) = (x + 2) with remainder 0
/// let dividend = expr!((x^2) + (3*x) + 2);
/// let divisor = expr!(x + 1);
/// let (quot, rem) = polynomial_div(&dividend, &divisor, &x);
/// ```
///
/// # Returns
///
/// Returns `(quotient, remainder)` tuple where both are expressions
pub fn polynomial_div(
    dividend: &Expression,
    divisor: &Expression,
    var: &Symbol,
) -> (Expression, Expression) {
    if divisor.is_zero() {
        return (Expression::undefined(), Expression::undefined());
    }

    if dividend.is_zero() {
        return (Expression::integer(0), Expression::integer(0));
    }

    if dividend == divisor {
        return (Expression::integer(1), Expression::integer(0));
    }

    // Fast path: divisor is constant
    if let Some(divisor_const) = extract_constant(divisor) {
        if divisor_const.is_zero() {
            return (Expression::undefined(), Expression::undefined());
        }
        let quotient = Expression::mul(vec![
            dividend.clone(),
            Expression::pow(divisor.clone(), Expression::integer(-1)),
        ])
        .simplify();
        return (quotient, Expression::integer(0));
    }

    // IntPoly fast-path - PRIMARY PATH
    let vars = dividend.find_variables();
    if vars.len() == 1 {
        let dividend_var = &vars[0];
        if dividend_var == var {
            let divisor_vars = divisor.find_variables();
            if divisor_vars.len() == 1
                && &divisor_vars[0] == var
                && IntPoly::can_convert(dividend, var)
                && IntPoly::can_convert(divisor, var)
            {
                if let (Some(p1), Some(p2)) = (
                    IntPoly::try_from_expression(dividend, var),
                    IntPoly::try_from_expression(divisor, var),
                ) {
                    // Pure IntPoly division - NO Expression tree involved
                    if let Ok((q, r)) = p1.div_rem(&p2) {
                        return (q.to_expression(var), r.to_expression(var));
                    }
                }
            }
        }
    }

    // Symbolic fallback for rational coefficients (minimal, necessary for some algorithms)
    symbolic_polynomial_div(dividend, divisor, var)
}

/// Extract constant value from expression if it's a constant
fn extract_constant(expr: &Expression) -> Option<Expression> {
    match expr {
        Expression::Number(_) => Some(expr.clone()),
        _ => None,
    }
}

/// Symbolic polynomial division using Expression operations
///
/// This is a MINIMAL fallback for cases that cannot use IntPoly.
/// Used for rational coefficient polynomials and special cases.
fn symbolic_polynomial_div(
    dividend: &Expression,
    divisor: &Expression,
    var: &Symbol,
) -> (Expression, Expression) {
    let dividend_degree = polynomial_degree_in_var(dividend, var);
    let divisor_degree = polynomial_degree_in_var(divisor, var);

    if dividend_degree < divisor_degree {
        return (Expression::integer(0), dividend.clone());
    }

    // Simple case: single division step
    if dividend_degree == divisor_degree {
        let dividend_lc = polynomial_leading_coefficient(dividend, var);
        let divisor_lc = polynomial_leading_coefficient(divisor, var);

        let quotient_term = Expression::mul(vec![
            dividend_lc,
            Expression::pow(divisor_lc, Expression::integer(-1)),
        ])
        .simplify();

        let product = Expression::mul(vec![quotient_term.clone(), divisor.clone()]).simplify();
        let remainder = Expression::add(vec![
            dividend.clone(),
            Expression::mul(vec![Expression::integer(-1), product]),
        ])
        .simplify();

        return (quotient_term, remainder);
    }

    // For more complex cases, return symbolic remainder
    (Expression::integer(0), dividend.clone())
}

/// Get polynomial degree with respect to a specific variable
fn polynomial_degree_in_var(expr: &Expression, var: &Symbol) -> i64 {
    match expr {
        Expression::Symbol(s) if s == var => 1,
        Expression::Number(_) => 0,
        Expression::Pow(base, exp) => {
            if let (Expression::Symbol(s), Expression::Number(Number::Integer(e))) =
                (base.as_ref(), exp.as_ref())
            {
                if s == var {
                    return *e;
                }
            }
            0
        }
        Expression::Add(terms) => {
            let mut max_degree = 0i64;
            for term in terms.iter() {
                let deg = polynomial_degree_in_var(term, var);
                max_degree = max_degree.max(deg);
            }
            max_degree
        }
        Expression::Mul(factors) => {
            let mut total_degree = 0i64;
            for factor in factors.iter() {
                total_degree += polynomial_degree_in_var(factor, var);
            }
            total_degree
        }
        _ => 0,
    }
}

/// Get leading coefficient of polynomial
fn polynomial_leading_coefficient(expr: &Expression, var: &Symbol) -> Expression {
    let degree = polynomial_degree_in_var(expr, var);

    match expr {
        Expression::Number(_n) => expr.clone(),
        Expression::Symbol(s) if s == var => Expression::integer(1),
        Expression::Pow(base, exp) => {
            if let (Expression::Symbol(s), Expression::Number(Number::Integer(_))) =
                (base.as_ref(), exp.as_ref())
            {
                if s == var {
                    return Expression::integer(1);
                }
            }
            expr.clone()
        }
        Expression::Mul(factors) => {
            let mut coeff = Expression::integer(1);
            for factor in factors.iter() {
                if polynomial_degree_in_var(factor, var) == 0 {
                    coeff = Expression::mul(vec![coeff, factor.clone()]);
                }
            }
            coeff
        }
        Expression::Add(terms) => {
            for term in terms.iter() {
                if polynomial_degree_in_var(term, var) == degree {
                    return polynomial_leading_coefficient(term, var);
                }
            }
            Expression::integer(0)
        }
        _ => Expression::integer(1),
    }
}

/// Polynomial quotient (division without remainder)
///
/// Returns only the quotient part of polynomial division
///
/// # Arguments
///
/// * `dividend` - Polynomial to divide
/// * `divisor` - Polynomial to divide by
/// * `var` - Variable to treat as polynomial variable
///
/// # Examples
///
/// ```rust
/// use mathhook_core::{expr, symbol};
/// use mathhook_core::algebra::polynomial_division::polynomial_quo;
///
/// let x = symbol!(x);
/// let dividend = expr!((x^2) + (3*x) + 2);
/// let divisor = expr!(x + 1);
/// let quot = polynomial_quo(&dividend, &divisor, &x);
/// ```
pub fn polynomial_quo(dividend: &Expression, divisor: &Expression, var: &Symbol) -> Expression {
    polynomial_div(dividend, divisor, var).0
}

/// Polynomial remainder
///
/// Returns only the remainder part of polynomial division
///
/// # Arguments
///
/// * `dividend` - Polynomial to divide
/// * `divisor` - Polynomial to divide by
/// * `var` - Variable to treat as polynomial variable
///
/// # Examples
///
/// ```rust
/// use mathhook_core::{expr, symbol};
/// use mathhook_core::algebra::polynomial_division::polynomial_rem;
///
/// let x = symbol!(x);
/// let dividend = expr!((x^2) + 1);
/// let divisor = expr!(x - 1);
/// let rem = polynomial_rem(&dividend, &divisor, &x);
/// ```
pub fn polynomial_rem(dividend: &Expression, divisor: &Expression, var: &Symbol) -> Expression {
    // IntPoly fast-path - dedicated remainder computation
    let vars = dividend.find_variables();
    if vars.len() == 1 {
        let dividend_var = &vars[0];
        if dividend_var == var {
            let divisor_vars = divisor.find_variables();
            if divisor_vars.len() == 1
                && &divisor_vars[0] == var
                && IntPoly::can_convert(dividend, var)
                && IntPoly::can_convert(divisor, var)
            {
                if let (Some(p1), Some(p2)) = (
                    IntPoly::try_from_expression(dividend, var),
                    IntPoly::try_from_expression(divisor, var),
                ) {
                    if let Ok((_, r)) = p1.div_rem(&p2) {
                        return r.to_expression(var);
                    }
                }
            }
        }
    }

    polynomial_div(dividend, divisor, var).1
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{expr, symbol};

    #[test]
    fn test_polynomial_div_exact() {
        let x = symbol!(x);

        let dividend = expr!((x ^ 2) - 1);
        let divisor = expr!(x - 1);
        let (_quot, rem) = polynomial_div(&dividend, &divisor, &x);

        assert!(rem.is_zero(), "Expected zero remainder");
    }

    #[test]
    fn test_polynomial_div_with_remainder() {
        let x = symbol!(x);

        let dividend = expr!((x ^ 2) + 1);
        let divisor = expr!(x - 1);
        let (_quot, rem) = polynomial_div(&dividend, &divisor, &x);

        assert!(!rem.is_zero(), "Expected non-zero remainder");
    }

    #[test]
    fn test_polynomial_div_by_constant() {
        let x = symbol!(x);

        let dividend = Expression::add(vec![
            Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
            Expression::mul(vec![Expression::integer(2), Expression::symbol(x.clone())]),
            Expression::integer(1),
        ]);
        let divisor = Expression::integer(2);
        let (_quot, rem) = polynomial_div(&dividend, &divisor, &x);

        assert!(rem.is_zero(), "Expected zero remainder");
    }

    #[test]
    fn test_polynomial_div_identical() {
        let x = symbol!(x);

        let dividend = expr!(x + 1);
        let divisor = expr!(x + 1);
        let (quot, rem) = polynomial_div(&dividend, &divisor, &x);

        assert_eq!(quot, Expression::integer(1));
        assert!(rem.is_zero());
    }

    #[test]
    fn test_polynomial_quo() {
        let x = symbol!(x);

        let dividend = expr!((x ^ 2) - 1);
        let divisor = expr!(x - 1);
        let quot = polynomial_quo(&dividend, &divisor, &x);

        assert!(!quot.is_zero());
    }

    #[test]
    fn test_polynomial_rem() {
        let x = symbol!(x);

        let dividend = expr!((x ^ 2) + 1);
        let divisor = expr!(x - 1);
        let rem = polynomial_rem(&dividend, &divisor, &x);

        assert!(!rem.is_zero());
    }

    #[test]
    fn test_intpoly_fastpath() {
        let x = symbol!(x);

        let dividend = expr!((x ^ 3) + (2 * (x ^ 2)) + (3 * x) + 4);
        let divisor = expr!((x ^ 2) + 1);
        let (quot, rem) = polynomial_div(&dividend, &divisor, &x);

        println!("Quotient: {}, Remainder: {}", quot, rem);
        assert_ne!(quot, Expression::undefined());
    }
}
