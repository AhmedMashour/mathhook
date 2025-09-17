//! S-Polynomial Computation
//!
//! Implements S-polynomial calculation, which is central to Buchberger's algorithm.
//! The S-polynomial of two polynomials is designed to eliminate their leading terms.

use super::monomial_order::{MonomialOrder, MonomialOrdering};
use crate::core::{Expression, Number, Symbol};

/// Compute the S-polynomial of two polynomials
///
/// The S-polynomial is defined as:
/// S(f, g) = (lcm(LT(f), LT(g)) / LT(f)) * f - (lcm(LT(f), LT(g)) / LT(g)) * g
///
/// where LT is the leading term. The S-polynomial is designed so that the
/// leading terms of f and g cancel out, making it useful for finding
/// new basis elements.
///
/// # Arguments
///
/// * `f` - First polynomial
/// * `g` - Second polynomial
/// * `variables` - Ordered list of variables
/// * `order` - Monomial ordering to use
///
/// # Returns
///
/// The S-polynomial of f and g
///
/// # Examples
///
/// ```rust
/// use mathhook_core::{symbol, expr, Expression};
/// use mathhook_core::algebra::groebner::{s_polynomial, MonomialOrder};
///
/// let x = symbol!(x);
/// let y = symbol!(y);
/// let f = expr!((x^2) + y);
/// let g = expr!((x*y) + 1);
/// let s = s_polynomial(&f, &g, &vec![x, y], &MonomialOrder::Lex);
/// ```
pub fn s_polynomial(
    f: &Expression,
    g: &Expression,
    variables: &[Symbol],
    order: &MonomialOrder,
) -> Expression {
    if f.is_zero() || g.is_zero() {
        return Expression::integer(0);
    }

    let lt_f = order.leading_monomial(f, variables);
    let lt_g = order.leading_monomial(g, variables);

    let lc_f = order.leading_coefficient(f, variables);
    let lc_g = order.leading_coefficient(g, variables);

    let lcm_lt = monomial_lcm(&lt_f, &lt_g, variables);

    let coeff_f = Expression::div(lcm_lt.clone(), Expression::mul(vec![lc_f, lt_f]));
    let coeff_g = Expression::div(lcm_lt, Expression::mul(vec![lc_g, lt_g]));

    let term1 = Expression::mul(vec![coeff_f, f.clone()]);
    let term2 = Expression::mul(vec![coeff_g, g.clone()]);

    // Use subtraction operator instead of Expression::sub
    term1 - term2
}

/// Compute the least common multiple of two monomials
///
/// For monomials x^a1 y^a2 and x^b1 y^b2, the LCM is x^max(a1,b1) y^max(a2,b2)
///
/// # Arguments
///
/// * `mono1` - First monomial
/// * `mono2` - Second monomial
/// * `variables` - Ordered list of variables
///
/// # Returns
///
/// The LCM monomial
fn monomial_lcm(mono1: &Expression, mono2: &Expression, variables: &[Symbol]) -> Expression {
    let exp1 = extract_exponents(mono1, variables);
    let exp2 = extract_exponents(mono2, variables);

    let mut lcm_factors = Vec::new();

    for (i, (e1, e2)) in exp1.iter().zip(exp2.iter()).enumerate() {
        let max_exp = (*e1).max(*e2);
        if max_exp > 0 {
            let var_expr = Expression::symbol(variables[i].clone());
            if max_exp == 1 {
                lcm_factors.push(var_expr);
            } else {
                lcm_factors.push(Expression::pow(var_expr, Expression::integer(max_exp)));
            }
        }
    }

    if lcm_factors.is_empty() {
        Expression::integer(1)
    } else if lcm_factors.len() == 1 {
        lcm_factors[0].clone()
    } else {
        Expression::mul(lcm_factors)
    }
}

/// Extract exponents of a monomial as a vector
fn extract_exponents(mono: &Expression, variables: &[Symbol]) -> Vec<i64> {
    let mut exponents = vec![0i64; variables.len()];

    match mono {
        Expression::Symbol(s) => {
            if let Some(idx) = variables.iter().position(|v| v == s) {
                exponents[idx] = 1;
            }
        }
        Expression::Pow(base, exp) => {
            if let Expression::Symbol(s) = base.as_ref() {
                if let Some(idx) = variables.iter().position(|v| v == s) {
                    // Extract integer from Number variant
                    if let Expression::Number(Number::Integer(i)) = exp.as_ref() {
                        exponents[idx] = *i;
                    }
                }
            }
        }
        Expression::Mul(factors) => {
            // Dereference Box<Vec<Expression>> to iterate
            for factor in factors.iter() {
                if !matches!(factor, Expression::Number(_)) {
                    let factor_exp = extract_exponents(factor, variables);
                    for (i, e) in factor_exp.iter().enumerate() {
                        exponents[i] += e;
                    }
                }
            }
        }
        _ => {}
    }

    exponents
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::symbol;

    #[test]
    fn test_monomial_lcm_simple() {
        let x = symbol!(x);
        let y = symbol!(y);
        let vars = vec![x.clone(), y.clone()];

        let mono1 = Expression::pow(Expression::symbol(x.clone()), Expression::integer(2));
        let mono2 = Expression::symbol(y.clone());

        let lcm = monomial_lcm(&mono1, &mono2, &vars);

        let expected = Expression::mul(vec![
            Expression::pow(Expression::symbol(x), Expression::integer(2)),
            Expression::symbol(y),
        ]);

        assert_eq!(lcm, expected);
    }

    #[test]
    fn test_monomial_lcm_overlap() {
        let x = symbol!(x);
        let y = symbol!(y);
        let vars = vec![x.clone(), y.clone()];

        let mono1 = Expression::mul(vec![
            Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
            Expression::symbol(y.clone()),
        ]);

        let mono2 = Expression::mul(vec![
            Expression::symbol(x.clone()),
            Expression::pow(Expression::symbol(y.clone()), Expression::integer(3)),
        ]);

        let lcm = monomial_lcm(&mono1, &mono2, &vars);

        let expected = Expression::mul(vec![
            Expression::pow(Expression::symbol(x), Expression::integer(2)),
            Expression::pow(Expression::symbol(y), Expression::integer(3)),
        ]);

        assert_eq!(lcm, expected);
    }

    #[test]
    fn test_s_polynomial_basic() {
        let x = symbol!(x);
        let y = symbol!(y);
        let vars = vec![x.clone(), y.clone()];

        let f = Expression::add(vec![
            Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
            Expression::symbol(y.clone()),
        ]);

        let g = Expression::add(vec![
            Expression::mul(vec![
                Expression::symbol(x.clone()),
                Expression::symbol(y.clone()),
            ]),
            Expression::integer(1),
        ]);

        let s = s_polynomial(&f, &g, &vars, &MonomialOrder::Lex);

        assert!(!s.is_zero());
    }

    #[test]
    fn test_s_polynomial_with_zero() {
        let x = symbol!(x);
        let vars = vec![x.clone()];

        let f = Expression::symbol(x.clone());
        let g = Expression::integer(0);

        let s = s_polynomial(&f, &g, &vars, &MonomialOrder::Lex);

        assert!(s.is_zero());
    }

    #[test]
    fn test_s_polynomial_identical() {
        let x = symbol!(x);
        let vars = vec![x.clone()];

        let f = Expression::pow(Expression::symbol(x.clone()), Expression::integer(2));
        let g = f.clone();

        let s = s_polynomial(&f, &g, &vars, &MonomialOrder::Lex);

        assert!(s.is_zero());
    }
}
