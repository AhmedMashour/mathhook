//! Bridge between finite field polynomials and Expression types
//!
//! Provides conversion between PolyZp and Expression for integration
//! with the main CAS. This enables modular GCD algorithms to work
//! with the standard Expression-based polynomial representation.
use super::element::Zp;
use super::poly::PolyZp;
use crate::core::constants::EPSILON;
use crate::core::{Expression, Number, Symbol};
use crate::simplify::Simplify;
use num_traits::ToPrimitive;
use std::collections::HashMap;
impl PolyZp {
    /// Convert a univariate Expression polynomial to PolyZp
    ///
    /// Extracts integer coefficients from an Expression and reduces them modulo the prime.
    /// Supports polynomials with integer coefficients in a single variable.
    ///
    /// # Arguments
    ///
    /// * `expr` - The Expression to convert (must be polynomial in var)
    /// * `var` - The variable of the polynomial
    /// * `prime` - The prime modulus
    ///
    /// # Returns
    ///
    /// A PolyZp with coefficients reduced mod prime.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::algebra::PolyZp;
    /// use mathhook_core::{expr, symbol};
    ///
    /// let x = symbol!(x);
    /// let poly = expr!((x^2) + (2*x) + 3);  // x^2 + 2x + 3
    /// let poly_zp = PolyZp::from_expression(&poly, &x, 7);
    /// assert_eq!(poly_zp.coefficients(), &[3, 2, 1]);
    /// ```
    pub fn from_expression(expr: &Expression, var: &Symbol, prime: u64) -> Self {
        let simplified = expr.clone().simplify();
        let coefficients = extract_polynomial_coefficients(&simplified, var);
        let unsigned_coeffs: Vec<u64> = coefficients
            .iter()
            .map(|&c| {
                let m = prime as i64;
                (((c % m) + m) % m) as u64
            })
            .collect();
        Self::from_coeffs(unsigned_coeffs, prime)
    }
    /// Convert PolyZp back to an Expression
    ///
    /// Creates a polynomial Expression from the finite field polynomial.
    /// Uses symmetric representation for coefficients (values > p/2 become negative).
    ///
    /// # Arguments
    ///
    /// * `var` - The variable to use in the Expression
    ///
    /// # Returns
    ///
    /// An Expression representing the polynomial.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::algebra::PolyZp;
    /// use mathhook_core::symbol;
    ///
    /// let poly = PolyZp::from_coeffs(vec![3, 2, 1], 7);  // x^2 + 2x + 3
    /// let x = symbol!(x);
    /// let expr = poly.to_expression(&x);
    /// // expr represents x^2 + 2*x + 3
    /// ```
    pub fn to_expression(&self, var: &Symbol) -> Expression {
        if self.is_zero() {
            return Expression::integer(0);
        }
        let mut terms: Vec<Expression> = Vec::new();
        for (i, &coeff) in self.coefficients().iter().enumerate() {
            if coeff == 0 {
                continue;
            }
            let coeff_symmetric = Zp::new(coeff, self.modulus()).to_symmetric();
            let coeff_expr = Expression::integer(coeff_symmetric);
            let term = if i == 0 {
                coeff_expr
            } else if i == 1 {
                if coeff_symmetric == 1 {
                    Expression::symbol(var.clone())
                } else if coeff_symmetric == -1 {
                    Expression::mul(vec![
                        Expression::integer(-1),
                        Expression::symbol(var.clone()),
                    ])
                } else {
                    Expression::mul(vec![coeff_expr, Expression::symbol(var.clone())])
                }
            } else {
                let power = Expression::pow(
                    Expression::symbol(var.clone()),
                    Expression::integer(i as i64),
                );
                if coeff_symmetric == 1 {
                    power
                } else if coeff_symmetric == -1 {
                    Expression::mul(vec![Expression::integer(-1), power])
                } else {
                    Expression::mul(vec![coeff_expr, power])
                }
            };
            terms.push(term);
        }
        if terms.is_empty() {
            Expression::integer(0)
        } else if terms.len() == 1 {
            terms.into_iter().next().unwrap()
        } else {
            Expression::add(terms)
        }
    }
    /// Convert PolyZp to Expression using standard (non-symmetric) representation
    ///
    /// Similar to `to_expression`, but coefficients stay in [0, p-1] range.
    ///
    /// # Arguments
    ///
    /// * `var` - The variable to use in the Expression
    pub fn to_expression_unsigned(&self, var: &Symbol) -> Expression {
        if self.is_zero() {
            return Expression::integer(0);
        }
        let mut terms: Vec<Expression> = Vec::new();
        for (i, &coeff) in self.coefficients().iter().enumerate() {
            if coeff == 0 {
                continue;
            }
            let coeff_expr = Expression::integer(coeff as i64);
            let term = if i == 0 {
                coeff_expr
            } else if i == 1 {
                if coeff == 1 {
                    Expression::symbol(var.clone())
                } else {
                    Expression::mul(vec![coeff_expr, Expression::symbol(var.clone())])
                }
            } else {
                let power = Expression::pow(
                    Expression::symbol(var.clone()),
                    Expression::integer(i as i64),
                );
                if coeff == 1 {
                    power
                } else {
                    Expression::mul(vec![coeff_expr, power])
                }
            };
            terms.push(term);
        }
        if terms.is_empty() {
            Expression::integer(0)
        } else if terms.len() == 1 {
            terms.into_iter().next().unwrap()
        } else {
            Expression::add(terms)
        }
    }
}
/// Extract polynomial coefficients from an Expression
///
/// Returns coefficients as i64 values where index i is coefficient of var^i.
/// Returns empty vector for zero polynomial.
fn extract_polynomial_coefficients(expr: &Expression, var: &Symbol) -> Vec<i64> {
    let mut coeffs: HashMap<usize, i64> = HashMap::new();
    extract_terms_recursive(expr, var, 1, &mut coeffs);
    if coeffs.is_empty() {
        return vec![];
    }
    let max_degree = *coeffs.keys().max().unwrap_or(&0);
    let mut result = vec![0i64; max_degree + 1];
    for (deg, coeff) in coeffs {
        result[deg] = coeff;
    }
    while result.last() == Some(&0) && result.len() > 1 {
        result.pop();
    }
    if result == vec![0] {
        return vec![];
    }
    result
}
/// Recursively extract terms from an expression
fn extract_terms_recursive(
    expr: &Expression,
    var: &Symbol,
    multiplier: i64,
    coeffs: &mut HashMap<usize, i64>,
) {
    match expr {
        Expression::Number(num) => {
            if let Some(n) = number_to_i64(num) {
                *coeffs.entry(0).or_insert(0) += n * multiplier;
            }
        }
        Expression::Symbol(s) => {
            if s == var {
                *coeffs.entry(1).or_insert(0) += multiplier;
            } else {
                *coeffs.entry(0).or_insert(0) += multiplier;
            }
        }
        Expression::Add(terms) => {
            for term in terms.iter() {
                extract_terms_recursive(term, var, multiplier, coeffs);
            }
        }
        Expression::Mul(factors) => {
            let (coeff, degree) = analyze_monomial(factors, var);
            *coeffs.entry(degree).or_insert(0) += coeff * multiplier;
        }
        Expression::Pow(base, exp) => {
            if let Expression::Symbol(s) = base.as_ref() {
                if s == var {
                    if let Some(n) = expression_to_i64(exp.as_ref()) {
                        if n >= 0 {
                            *coeffs.entry(n as usize).or_insert(0) += multiplier;
                        }
                    }
                }
            }
        }
        Expression::Constant(_) => {
            *coeffs.entry(0).or_insert(0) += multiplier;
        }
        _ => {
            *coeffs.entry(0).or_insert(0) += multiplier;
        }
    }
}
/// Convert a Number to i64 if possible
#[inline]
fn number_to_i64(num: &Number) -> Option<i64> {
    match num {
        Number::Integer(i) => Some(*i),
        Number::BigInteger(bi) => bi.to_i64(),
        Number::Float(f) => {
            if f.fract().abs() < EPSILON {
                Some(*f as i64)
            } else {
                None
            }
        }
        Number::Rational(r) => {
            if r.is_integer() {
                r.numer().to_i64()
            } else {
                None
            }
        }
    }
}
/// Convert an Expression to i64 if it's an integer
#[inline]
fn expression_to_i64(expr: &Expression) -> Option<i64> {
    match expr {
        Expression::Number(num) => number_to_i64(num),
        _ => None,
    }
}
/// Analyze a monomial (product of factors) to extract coefficient and degree
fn analyze_monomial(factors: &[Expression], var: &Symbol) -> (i64, usize) {
    let mut coeff: i64 = 1;
    let mut degree: usize = 0;
    for factor in factors {
        match factor {
            Expression::Number(num) => {
                if let Some(n) = number_to_i64(num) {
                    coeff *= n;
                }
            }
            Expression::Symbol(s) => {
                if s == var {
                    degree += 1;
                }
            }
            Expression::Pow(base, exp) => {
                if let Expression::Symbol(s) = base.as_ref() {
                    if s == var {
                        if let Some(n) = expression_to_i64(exp.as_ref()) {
                            if n >= 0 {
                                degree += n as usize;
                            }
                        }
                    }
                }
            }
            _ => {}
        }
    }
    (coeff, degree)
}
/// Educational explanations for finite field operations
pub mod educational {
    /// Explain the concept of a finite field
    pub fn explain_finite_field() -> &'static str {
        "A finite field Z_p consists of integers {0, 1, ..., p-1} where p is prime. \
         All arithmetic is done modulo p. The key property is that every non-zero \
         element has a multiplicative inverse, making division always possible \
         (except by zero). This is essential for GCD algorithms to terminate correctly."
    }
    /// Explain modular arithmetic
    pub fn explain_modular_arithmetic() -> &'static str {
        "In modular arithmetic mod p, we 'wrap around' when reaching p. \
         For example, in Z_7: 5 + 3 = 8 = 1 (mod 7). This creates a closed \
         algebraic structure where all operations stay within {0, ..., p-1}."
    }
    /// Explain multiplicative inverse
    pub fn explain_inverse() -> &'static str {
        "The multiplicative inverse of a mod p is the number b such that a*b = 1 (mod p). \
         By Fermat's little theorem, for prime p: a^(p-1) = 1 (mod p), so a^(-1) = a^(p-2). \
         The extended Euclidean algorithm provides a faster computation method."
    }
    /// Explain polynomial GCD over finite fields
    pub fn explain_polynomial_gcd() -> &'static str {
        "The Euclidean algorithm for polynomials over Z_p repeatedly divides \
         and takes remainders until reaching zero. The last non-zero remainder \
         is the GCD. Over finite fields, this always terminates because division \
         is exact (every non-zero element has an inverse)."
    }
    /// Explain why we use finite fields for GCD
    pub fn explain_why_finite_fields_for_gcd() -> &'static str {
        "Integer polynomial GCD can have coefficient explosion (intermediate results \
         with huge coefficients). By working mod p for several primes and using the \
         Chinese Remainder Theorem to reconstruct, we avoid this explosion while \
         maintaining correctness."
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::{expr, symbol};
    #[test]
    fn test_poly_from_expression_simple() {
        let x = symbol!(x);
        let poly = expr!((x ^ 2) + (2 * x) + 3);
        let poly_zp = PolyZp::from_expression(&poly, &x, 7);
        assert_eq!(poly_zp.coefficients(), &[3, 2, 1]);
    }
    #[test]
    fn test_poly_from_expression_negative() {
        let x = symbol!(x);
        let poly = expr!((x ^ 2) - x + 5);
        let poly_zp = PolyZp::from_expression(&poly, &x, 7);
        assert_eq!(poly_zp.coefficients(), &[5, 6, 1]);
    }
    #[test]
    fn test_poly_to_expression_basic() {
        let x = symbol!(x);
        let poly = PolyZp::from_coeffs(vec![3, 2, 1], 7);
        let expr = poly.to_expression(&x);
        let back = PolyZp::from_expression(&expr, &x, 7);
        assert_eq!(back.coefficients(), poly.coefficients());
    }
    #[test]
    fn test_poly_to_expression_symmetric() {
        let x = symbol!(x);
        let poly = PolyZp::from_coeffs(vec![6, 5, 1], 7);
        let expr = poly.to_expression(&x);
        let back = PolyZp::from_expression(&expr, &x, 7);
        assert_eq!(back.coefficients(), poly.coefficients());
    }
    #[test]
    fn test_poly_roundtrip() {
        let x = symbol!(x);
        let original = expr!((3 * (x ^ 3)) - (2 * (x ^ 2)) + x - 5);
        let poly_zp = PolyZp::from_expression(&original, &x, 11);
        let back = poly_zp.to_expression(&x);
        let poly_zp2 = PolyZp::from_expression(&back, &x, 11);
        assert_eq!(poly_zp.coefficients(), poly_zp2.coefficients());
    }
    #[test]
    fn test_poly_from_expression_constant() {
        let x = symbol!(x);
        let poly = expr!(5);
        let poly_zp = PolyZp::from_expression(&poly, &x, 7);
        assert_eq!(poly_zp.coefficients(), &[5]);
    }
    #[test]
    fn test_poly_from_expression_just_x() {
        let x = symbol!(x);
        let expr = Expression::symbol(x.clone());
        let poly_zp = PolyZp::from_expression(&expr, &x, 7);
        assert_eq!(poly_zp.coefficients(), &[0, 1]);
    }
    #[test]
    fn test_poly_to_expression_zero() {
        let x = symbol!(x);
        let poly = PolyZp::zero(7);
        let expr = poly.to_expression(&x);
        assert!(expr.is_zero());
    }
    #[test]
    fn test_poly_to_expression_constant() {
        let x = symbol!(x);
        let poly = PolyZp::constant(5, 7);
        let expr = poly.to_expression(&x);
        let back = PolyZp::from_expression(&expr, &x, 7);
        assert_eq!(back.coefficients(), &[5]);
    }
}
