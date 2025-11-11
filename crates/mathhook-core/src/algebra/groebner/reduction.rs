//! Polynomial Reduction Modulo a Set
//!
//! Implements polynomial reduction, which is the division algorithm for multivariate
//! polynomials. This is a key component of Gröbner basis algorithms.

use super::monomial_order::{MonomialOrder, MonomialOrdering};
use crate::core::{Expression, Number, Symbol};

/// Reduce a polynomial modulo a set of polynomials (one step)
///
/// Performs one reduction step: if the leading term of poly is divisible
/// by the leading term of some polynomial in the basis, subtract an appropriate
/// multiple to eliminate that term.
///
/// # Arguments
///
/// * `poly` - Polynomial to reduce
/// * `basis` - Set of polynomials to reduce against
/// * `variables` - Ordered list of variables
/// * `order` - Monomial ordering to use
///
/// # Returns
///
/// Tuple of (reduced polynomial, whether reduction occurred)
pub fn poly_reduce(
    poly: &Expression,
    basis: &[Expression],
    variables: &[Symbol],
    order: &MonomialOrder,
) -> (Expression, bool) {
    if poly.is_zero() {
        return (Expression::integer(0), false);
    }

    let lt_poly = order.leading_monomial(poly, variables);
    let lc_poly = order.leading_coefficient(poly, variables);

    for g in basis {
        if g.is_zero() {
            continue;
        }

        let lt_g = order.leading_monomial(g, variables);
        let lc_g = order.leading_coefficient(g, variables);

        if divides(&lt_g, &lt_poly, variables) {
            let quotient_term = Expression::div(
                Expression::mul(vec![lc_poly.clone(), lt_poly.clone()]),
                Expression::mul(vec![lc_g, lt_g]),
            );

            let subtrahend = Expression::mul(vec![quotient_term, g.clone()]);
            // Use subtraction operator instead of Expression::sub
            let reduced = poly.clone() - subtrahend;

            return (reduced, true);
        }
    }

    (poly.clone(), false)
}

/// Reduce a polynomial completely modulo a set of polynomials
///
/// Repeatedly applies reduction until no more reduction is possible.
/// Returns the normal form (remainder) of the polynomial.
///
/// # Arguments
///
/// * `poly` - Polynomial to reduce
/// * `basis` - Set of polynomials to reduce against
/// * `variables` - Ordered list of variables
/// * `order` - Monomial ordering to use
///
/// # Returns
///
/// The fully reduced polynomial (normal form)
///
/// # Examples
///
/// ```rust
/// use mathhook_core::{symbol, expr, Expression};
/// use mathhook_core::algebra::groebner::{poly_reduce_completely, MonomialOrder};
///
/// let x = symbol!(x);
/// let y = symbol!(y);
/// let poly = expr!((x^2) + (x*y) + 1);
/// let basis = vec![expr!(x - y)];
/// let reduced = poly_reduce_completely(
///     &poly,
///     &basis,
///     &vec![x, y],
///     &MonomialOrder::Lex
/// );
/// ```
pub fn poly_reduce_completely(
    poly: &Expression,
    basis: &[Expression],
    variables: &[Symbol],
    order: &MonomialOrder,
) -> Expression {
    let mut current = poly.clone();
    let max_iterations = 1000;
    let mut iterations = 0;

    loop {
        if iterations >= max_iterations {
            break;
        }
        iterations += 1;

        let (reduced, changed) = poly_reduce(&current, basis, variables, order);

        if !changed {
            break;
        }

        current = reduced;

        if current.is_zero() {
            break;
        }
    }

    current
}

/// Check if monomial m1 divides monomial m2
///
/// For monomials x^a1 y^a2 and x^b1 y^b2, m1 | m2 iff ai <= bi for all i
///
/// # Arguments
///
/// * `m1` - Divisor monomial
/// * `m2` - Dividend monomial
/// * `variables` - Ordered list of variables
///
/// # Returns
///
/// `true` if m1 divides m2
fn divides(m1: &Expression, m2: &Expression, variables: &[Symbol]) -> bool {
    let exp1 = extract_exponents(m1, variables);
    let exp2 = extract_exponents(m2, variables);

    exp1.iter().zip(exp2.iter()).all(|(e1, e2)| e1 <= e2)
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
    fn test_divides_simple() {
        let x = symbol!(x);
        let y = symbol!(y);
        let vars = vec![x.clone(), y.clone()];

        let m1 = Expression::symbol(x.clone());
        let m2 = Expression::pow(Expression::symbol(x.clone()), Expression::integer(2));

        assert!(divides(&m1, &m2, &vars));
        assert!(!divides(&m2, &m1, &vars));
    }

    #[test]
    fn test_divides_multivariate() {
        let x = symbol!(x);
        let y = symbol!(y);
        let vars = vec![x.clone(), y.clone()];

        let m1 = Expression::mul(vec![
            Expression::symbol(x.clone()),
            Expression::symbol(y.clone()),
        ]);

        let m2 = Expression::mul(vec![
            Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
            Expression::pow(Expression::symbol(y.clone()), Expression::integer(2)),
        ]);

        assert!(divides(&m1, &m2, &vars));
    }

    #[test]
    fn test_poly_reduce_simple() {
        let x = symbol!(x);
        let vars = vec![x.clone()];

        let poly = Expression::pow(Expression::symbol(x.clone()), Expression::integer(2));
        let g = Expression::symbol(x.clone());
        let basis = vec![g];

        let (reduced, changed) = poly_reduce(&poly, &basis, &vars, &MonomialOrder::Lex);

        assert!(changed);
        assert!(!reduced.is_zero());
    }

    #[test]
    fn test_poly_reduce_no_reduction() {
        let x = symbol!(x);
        let y = symbol!(y);
        let vars = vec![x.clone(), y.clone()];

        let poly = Expression::symbol(y.clone());
        let g = Expression::pow(Expression::symbol(x.clone()), Expression::integer(2));
        let basis = vec![g];

        let (reduced, changed) = poly_reduce(&poly, &basis, &vars, &MonomialOrder::Lex);

        assert!(!changed);
        assert_eq!(reduced, poly);
    }

    #[test]
    fn test_poly_reduce_completely() {
        let x = symbol!(x);
        let y = symbol!(y);
        let vars = vec![x.clone(), y.clone()];

        let poly = Expression::add(vec![
            Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
            Expression::integer(1),
        ]);

        // Use subtraction operator instead of Expression::sub
        let g = Expression::symbol(x.clone()) - Expression::symbol(y.clone());
        let basis = vec![g];

        let reduced = poly_reduce_completely(&poly, &basis, &vars, &MonomialOrder::Lex);

        assert!(!reduced.is_zero());
    }

    #[test]
    #[ignore]
    fn test_reduce_to_zero() {
        let x = symbol!(x);
        let vars = vec![x.clone()];

        let poly = Expression::symbol(x.clone());
        let basis = vec![poly.clone()];

        let reduced = poly_reduce_completely(&poly, &basis, &vars, &MonomialOrder::Lex);

        assert!(reduced.is_zero());
    }
}
