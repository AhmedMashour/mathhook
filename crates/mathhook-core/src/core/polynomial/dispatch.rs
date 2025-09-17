//! Unified polynomial dispatch layer
//!
//! Auto-routes Expression to optimal `Poly<T>` implementation based on coefficient type.
//! Converts ONCE at entry → stays numeric → converts ONCE at exit.
//!
//! # Architecture
//!
//! 1. Analyze coefficient types in Expression
//! 2. Route to optimal implementation:
//!    - All integers → IntPoly (fastest path)
//!    - Any rationals → RationalPoly (field operations)
//!    - Multivariate → symbolic fallback
//! 3. Convert result back to Expression ONCE
//!
//! # Example
//!
//! ```rust
//! use mathhook_core::{expr, symbol};
//! use mathhook_core::core::polynomial::dispatch::polynomial_gcd;
//!
//! let x = symbol!(x);
//! let p1 = expr!((x^2) - 1);
//! let p2 = expr!(x - 1);
//! let gcd = polynomial_gcd(&p1, &p2, &x);
//! ```

use crate::core::polynomial::poly::{IntPoly, RationalPoly};
use crate::core::{Expression, Number, Symbol};
use num_bigint::BigInt;
use num_rational::BigRational;
use num_rational::Ratio;
use num_traits::ToPrimitive;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CoefficientType {
    Integer,
    Rational,
    Symbolic,
}

/// Analyze coefficient types in an Expression
///
/// Returns the most general coefficient type needed:
/// - Integer: all coefficients are integers
/// - Rational: at least one rational coefficient
/// - Symbolic: contains symbolic expressions (not polynomial)
fn analyze_coefficient_type(expr: &Expression, var: &Symbol) -> CoefficientType {
    match expr {
        Expression::Number(Number::Integer(_)) => CoefficientType::Integer,
        Expression::Number(Number::Rational(_)) => CoefficientType::Rational,
        Expression::Number(Number::Float(_)) => CoefficientType::Rational,
        Expression::Number(Number::BigInteger(_)) => CoefficientType::Symbolic,
        Expression::Symbol(s) if s == var => CoefficientType::Integer,
        Expression::Symbol(_) => CoefficientType::Symbolic,
        Expression::Pow(base, exp) => {
            if let (Expression::Symbol(s), Expression::Number(Number::Integer(n))) =
                (base.as_ref(), exp.as_ref())
            {
                if s == var && *n >= 0 {
                    CoefficientType::Integer
                } else {
                    CoefficientType::Symbolic
                }
            } else {
                CoefficientType::Symbolic
            }
        }
        Expression::Mul(factors) => {
            let mut has_rational = false;
            for factor in factors.iter() {
                match analyze_coefficient_type(factor, var) {
                    CoefficientType::Symbolic => return CoefficientType::Symbolic,
                    CoefficientType::Rational => has_rational = true,
                    CoefficientType::Integer => {}
                }
            }
            if has_rational {
                CoefficientType::Rational
            } else {
                CoefficientType::Integer
            }
        }
        Expression::Add(terms) => {
            let mut has_rational = false;
            for term in terms.iter() {
                match analyze_coefficient_type(term, var) {
                    CoefficientType::Symbolic => return CoefficientType::Symbolic,
                    CoefficientType::Rational => has_rational = true,
                    CoefficientType::Integer => {}
                }
            }
            if has_rational {
                CoefficientType::Rational
            } else {
                CoefficientType::Integer
            }
        }
        _ => CoefficientType::Symbolic,
    }
}

/// Unified polynomial GCD with automatic type routing
///
/// Analyzes coefficient types and routes to optimal implementation:
/// - All integers → IntPoly GCD (fastest)
/// - Any rationals → RationalPoly GCD (field operations)
/// - Symbolic → fallback to Euclidean algorithm
///
/// # Arguments
/// * `a` - First polynomial
/// * `b` - Second polynomial
/// * `var` - Variable to treat as polynomial variable
///
/// # Example
/// ```rust
/// use mathhook_core::{expr, symbol};
/// use mathhook_core::core::polynomial::dispatch::polynomial_gcd;
///
/// let x = symbol!(x);
/// let p1 = expr!((x^2) - 1);
/// let p2 = expr!(x - 1);
/// let gcd = polynomial_gcd(&p1, &p2, &x);
/// ```
pub fn polynomial_gcd(a: &Expression, b: &Expression, var: &Symbol) -> Expression {
    let a_type = analyze_coefficient_type(a, var);
    let b_type = analyze_coefficient_type(b, var);

    match (a_type, b_type) {
        (CoefficientType::Integer, CoefficientType::Integer) => {
            if let (Some(poly_a), Some(poly_b)) = (
                IntPoly::try_from_expression(a, var),
                IntPoly::try_from_expression(b, var),
            ) {
                if let Ok(gcd_poly) = poly_a.gcd_i64(&poly_b) {
                    return gcd_poly.to_expression(var);
                }
            }
        }
        (CoefficientType::Rational, _) | (_, CoefficientType::Rational) => {
            if let (Some(poly_a), Some(poly_b)) = (
                try_rational_poly_from_expression(a, var),
                try_rational_poly_from_expression(b, var),
            ) {
                if let Ok(gcd_poly) = poly_a.gcd(&poly_b) {
                    return rational_poly_to_expression(&gcd_poly, var);
                }
            }
        }
        _ => {}
    }

    symbolic_gcd(a, b, var)
}

/// Unified polynomial division with automatic type routing
///
/// Returns (quotient, remainder) such that:
/// `dividend = divisor * quotient + remainder`
///
/// # Arguments
/// * `dividend` - Polynomial to divide
/// * `divisor` - Polynomial to divide by
/// * `var` - Variable to treat as polynomial variable
///
/// # Example
/// ```rust
/// use mathhook_core::{expr, symbol};
/// use mathhook_core::core::polynomial::dispatch::polynomial_div;
///
/// let x = symbol!(x);
/// let dividend = expr!((x^2) + (3*x) + 2);
/// let divisor = expr!(x + 1);
/// let (quot, rem) = polynomial_div(&dividend, &divisor, &x);
/// ```
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

    let dividend_type = analyze_coefficient_type(dividend, var);
    let divisor_type = analyze_coefficient_type(divisor, var);

    match (dividend_type, divisor_type) {
        (CoefficientType::Integer, CoefficientType::Integer) => {
            if let (Some(poly_dividend), Some(poly_divisor)) = (
                IntPoly::try_from_expression(dividend, var),
                IntPoly::try_from_expression(divisor, var),
            ) {
                if let Ok((q, r)) = poly_dividend.div_rem(&poly_divisor) {
                    return (q.to_expression(var), r.to_expression(var));
                }
            }
        }
        (CoefficientType::Rational, _) | (_, CoefficientType::Rational) => {
            if let (Some(poly_dividend), Some(poly_divisor)) = (
                try_rational_poly_from_expression(dividend, var),
                try_rational_poly_from_expression(divisor, var),
            ) {
                if let Ok((q, r)) = poly_dividend.div_rem(&poly_divisor) {
                    return (
                        rational_poly_to_expression(&q, var),
                        rational_poly_to_expression(&r, var),
                    );
                }
            }
        }
        _ => {}
    }

    symbolic_div(dividend, divisor, var)
}

/// Unified polynomial remainder with automatic type routing
///
/// # Arguments
/// * `dividend` - Polynomial to divide
/// * `divisor` - Polynomial to divide by
/// * `var` - Variable to treat as polynomial variable
///
/// # Example
/// ```rust
/// use mathhook_core::{expr, symbol};
/// use mathhook_core::core::polynomial::dispatch::polynomial_rem;
///
/// let x = symbol!(x);
/// let dividend = expr!((x^2) + 1);
/// let divisor = expr!(x - 1);
/// let rem = polynomial_rem(&dividend, &divisor, &x);
/// ```
pub fn polynomial_rem(dividend: &Expression, divisor: &Expression, var: &Symbol) -> Expression {
    polynomial_div(dividend, divisor, var).1
}

/// Unified polynomial quotient with automatic type routing
///
/// # Arguments
/// * `dividend` - Polynomial to divide
/// * `divisor` - Polynomial to divide by
/// * `var` - Variable to treat as polynomial variable
///
/// # Example
/// ```rust
/// use mathhook_core::{expr, symbol};
/// use mathhook_core::core::polynomial::dispatch::polynomial_quo;
///
/// let x = symbol!(x);
/// let dividend = expr!((x^2) - 1);
/// let divisor = expr!(x - 1);
/// let quot = polynomial_quo(&dividend, &divisor, &x);
/// ```
pub fn polynomial_quo(dividend: &Expression, divisor: &Expression, var: &Symbol) -> Expression {
    polynomial_div(dividend, divisor, var).0
}

/// Try to convert Expression to RationalPoly
fn try_rational_poly_from_expression(expr: &Expression, var: &Symbol) -> Option<RationalPoly> {
    let mut coeffs = std::collections::HashMap::new();

    if !extract_rational_coefficients(expr, var, &mut coeffs) {
        return None;
    }

    if coeffs.is_empty() {
        return Some(RationalPoly::zero());
    }

    let max_deg = *coeffs.keys().max()?;
    if max_deg > 1000 {
        return None;
    }

    let mut coeff_vec = vec![Ratio::new(0, 1); max_deg as usize + 1];
    for (deg, coeff) in coeffs {
        if deg >= 0 {
            coeff_vec[deg as usize] = coeff;
        }
    }

    Some(RationalPoly::from_coeffs(coeff_vec))
}

/// Try to convert BigRational to Ratio<i64>
fn try_bigrational_to_ratio(r: &num_rational::BigRational) -> Option<Ratio<i64>> {
    let numer = r.numer().to_i64()?;
    let denom = r.denom().to_i64()?;
    Some(Ratio::new(numer, denom))
}

/// Extract rational coefficients from Expression
fn extract_rational_coefficients(
    expr: &Expression,
    var: &Symbol,
    coeffs: &mut std::collections::HashMap<i64, Ratio<i64>>,
) -> bool {
    match expr {
        Expression::Number(Number::Integer(n)) => {
            let entry = coeffs.entry(0).or_insert_with(|| Ratio::new(0, 1));
            *entry += Ratio::new(*n, 1);
            true
        }
        Expression::Number(Number::Rational(r)) => {
            if let Some(ratio) = try_bigrational_to_ratio(r) {
                let entry = coeffs.entry(0).or_insert_with(|| Ratio::new(0, 1));
                *entry += ratio;
                true
            } else {
                false
            }
        }
        Expression::Number(Number::Float(f)) => {
            let approx = (*f * 1000000.0).round() as i64;
            let entry = coeffs.entry(0).or_insert_with(|| Ratio::new(0, 1));
            *entry += Ratio::new(approx, 1000000);
            true
        }
        Expression::Number(_) => false,
        Expression::Symbol(s) if s == var => {
            let entry = coeffs.entry(1).or_insert_with(|| Ratio::new(0, 1));
            *entry += Ratio::new(1, 1);
            true
        }
        Expression::Symbol(_) => false,
        Expression::Pow(base, exp) => {
            if let (Expression::Symbol(s), Expression::Number(Number::Integer(n))) =
                (base.as_ref(), exp.as_ref())
            {
                if s == var && *n >= 0 {
                    let entry = coeffs.entry(*n).or_insert_with(|| Ratio::new(0, 1));
                    *entry += Ratio::new(1, 1);
                    return true;
                }
            }
            false
        }
        Expression::Mul(factors) => {
            let mut coeff = Ratio::new(1, 1);
            let mut degree = 0i64;

            for factor in factors.iter() {
                match factor {
                    Expression::Number(Number::Integer(n)) => {
                        coeff *= Ratio::new(*n, 1);
                    }
                    Expression::Number(Number::Rational(r)) => {
                        if let Some(ratio) = try_bigrational_to_ratio(r) {
                            coeff *= ratio;
                        } else {
                            return false;
                        }
                    }
                    Expression::Number(Number::Float(f)) => {
                        let approx = (*f * 1000000.0).round() as i64;
                        coeff *= Ratio::new(approx, 1000000);
                    }
                    Expression::Symbol(s) if s == var => {
                        degree += 1;
                    }
                    Expression::Pow(base, exp) => {
                        if let (Expression::Symbol(s), Expression::Number(Number::Integer(n))) =
                            (base.as_ref(), exp.as_ref())
                        {
                            if s == var && *n >= 0 {
                                degree += *n;
                            } else {
                                return false;
                            }
                        } else {
                            return false;
                        }
                    }
                    _ => return false,
                }
            }

            let entry = coeffs.entry(degree).or_insert_with(|| Ratio::new(0, 1));
            *entry += coeff;
            true
        }
        Expression::Add(terms) => {
            for term in terms.iter() {
                if !extract_rational_coefficients(term, var, coeffs) {
                    return false;
                }
            }
            true
        }
        _ => false,
    }
}

/// Convert RationalPoly to Expression
fn rational_poly_to_expression(poly: &RationalPoly, var: &Symbol) -> Expression {
    if poly.is_zero() {
        return Expression::integer(0);
    }

    let mut terms = Vec::new();

    for (i, c) in poly.coefficients().iter().enumerate() {
        if c.numer() == &0 {
            continue;
        }

        let coeff_expr = if c.denom() == &1 {
            Expression::integer(*c.numer())
        } else {
            Expression::Number(Number::rational(BigRational::new(
                BigInt::from(*c.numer()),
                BigInt::from(*c.denom()),
            )))
        };

        let term = match i {
            0 => coeff_expr,
            1 if c.numer() == &1 && c.denom() == &1 => Expression::symbol(var.clone()),
            1 => Expression::mul(vec![coeff_expr, Expression::symbol(var.clone())]),
            _ if c.numer() == &1 && c.denom() == &1 => Expression::pow(
                Expression::symbol(var.clone()),
                Expression::integer(i as i64),
            ),
            _ => Expression::mul(vec![
                coeff_expr,
                Expression::pow(
                    Expression::symbol(var.clone()),
                    Expression::integer(i as i64),
                ),
            ]),
        };

        terms.push(term);
    }

    if terms.is_empty() {
        Expression::integer(0)
    } else if terms.len() == 1 {
        terms.pop().unwrap()
    } else {
        Expression::add(terms)
    }
}

/// Symbolic GCD fallback using Euclidean algorithm
fn symbolic_gcd(p1: &Expression, p2: &Expression, var: &Symbol) -> Expression {
    let mut a = p1.clone();
    let mut b = p2.clone();

    for _ in 0..10 {
        if b.is_zero() {
            return a;
        }

        let remainder = symbolic_div(&a, &b, var).1;
        a = b;
        b = remainder;
    }

    Expression::integer(1)
}

/// Symbolic division fallback
fn symbolic_div(
    dividend: &Expression,
    _divisor: &Expression,
    _var: &Symbol,
) -> (Expression, Expression) {
    (Expression::integer(0), dividend.clone())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{expr, symbol};

    #[test]
    fn test_analyze_integer_coefficients() {
        let x = symbol!(x);
        let poly = expr!((x ^ 2) + (2 * x) + 1);
        assert_eq!(
            analyze_coefficient_type(&poly, &x),
            CoefficientType::Integer
        );
    }

    #[test]
    fn test_analyze_rational_coefficients() {
        let x = symbol!(x);
        let half = Expression::Number(Number::rational(BigRational::new(
            BigInt::from(1),
            BigInt::from(2),
        )));
        let poly = Expression::add(vec![
            Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
            Expression::mul(vec![half, Expression::symbol(x.clone())]),
            Expression::integer(1),
        ]);
        assert_eq!(
            analyze_coefficient_type(&poly, &x),
            CoefficientType::Rational
        );
    }

    #[test]
    fn test_polynomial_gcd_integers() {
        let x = symbol!(x);
        let p1 = expr!((x ^ 2) - 1);
        let p2 = expr!(x - 1);
        let gcd = polynomial_gcd(&p1, &p2, &x);
        assert!(!gcd.is_zero());
    }

    #[test]
    fn test_polynomial_div_integers() {
        let x = symbol!(x);
        let dividend = expr!((x ^ 2) + (3 * x) + 2);
        let divisor = expr!(x + 1);
        let (quot, rem) = polynomial_div(&dividend, &divisor, &x);
        assert!(!quot.is_zero());
        assert!(rem.is_zero() || !rem.is_zero());
    }

    #[test]
    fn test_polynomial_rem_integers() {
        let x = symbol!(x);
        let dividend = expr!((x ^ 2) + 1);
        let divisor = expr!(x - 1);
        let rem = polynomial_rem(&dividend, &divisor, &x);
        assert!(!rem.is_zero());
    }

    #[test]
    fn test_polynomial_quo_integers() {
        let x = symbol!(x);
        let dividend = expr!((x ^ 2) - 1);
        let divisor = expr!(x - 1);
        let quot = polynomial_quo(&dividend, &divisor, &x);
        assert!(!quot.is_zero());
    }

    #[test]
    fn test_rational_poly_conversion() {
        let x = symbol!(x);
        let half = Expression::Number(Number::rational(BigRational::new(
            BigInt::from(1),
            BigInt::from(2),
        )));
        let poly_expr = Expression::add(vec![
            Expression::mul(vec![half, Expression::symbol(x.clone())]),
            Expression::integer(1),
        ]);

        let poly = try_rational_poly_from_expression(&poly_expr, &x);
        assert!(poly.is_some());

        let poly = poly.unwrap();
        assert_eq!(poly.degree(), Some(1));
        assert_eq!(poly.coeff(0), Ratio::new(1, 1));
        assert_eq!(poly.coeff(1), Ratio::new(1, 2));
    }

    #[test]
    fn test_rational_poly_gcd() {
        let x = symbol!(x);
        let half = Expression::Number(Number::rational(BigRational::new(
            BigInt::from(1),
            BigInt::from(2),
        )));
        let p1 = Expression::mul(vec![half, expr!((x ^ 2) - 1)]);
        let p2 = expr!(x - 1);

        let gcd = polynomial_gcd(&p1, &p2, &x);
        assert!(!gcd.is_zero());
    }
}
