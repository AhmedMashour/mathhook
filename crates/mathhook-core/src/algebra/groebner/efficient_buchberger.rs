//! Efficient Buchberger Algorithm using SparsePolynomial
//!
//! This implementation uses the sparse polynomial representation for O(n²) arithmetic
//! instead of the exponential growth from Expression AST.
//!
//! Performance target: Match SymPy (0.00s for circle-line system)

use super::monomial_order::MonomialOrder;
use crate::core::polynomial::sparse_polynomial::{
    expression_to_sparse_polynomial, sparse_polynomial_to_expression, Monomial, SparsePolynomial,
};
use crate::core::{Expression, Symbol};
use crate::error::{MathError, MathResult};
use std::collections::VecDeque;

/// Compute Gröbner basis using efficient sparse polynomial representation
///
/// This implementation uses SparsePolynomial for O(n) addition and O(n²) multiplication,
/// achieving SymPy-level performance.
///
/// # Arguments
///
/// * `generators` - Initial polynomial generators as Expressions
/// * `variables` - Variables in the polynomial ring
/// * `order` - Monomial ordering
///
/// # Returns
///
/// Returns the Gröbner basis as Expressions
///
/// # Examples
///
/// ```rust,ignore
/// use mathhook_core::{symbol, Expression};
/// use mathhook_core::algebra::groebner::efficient_buchberger::efficient_buchberger_algorithm;
/// use mathhook_core::algebra::groebner::MonomialOrder;
///
/// let x = symbol!(x);
/// let y = symbol!(y);
/// // Circle-line system: x² + y² = 1, x - y = 0
/// let f1 = Expression::add(vec![
///     Expression::pow(x.clone().into(), Expression::integer(2)),
///     Expression::pow(y.clone().into(), Expression::integer(2)),
///     Expression::integer(-1),
/// ]);
/// let f2 = Expression::add(vec![
///     x.clone().into(),
///     Expression::mul(vec![Expression::integer(-1), y.clone().into()]),
/// ]);
///
/// let result = efficient_buchberger_algorithm(
///     &[f1, f2],
///     &[x, y],
///     &MonomialOrder::Lex
/// );
/// assert!(result.is_ok());
/// ```
pub fn efficient_buchberger_algorithm(
    generators: &[Expression],
    variables: &[Symbol],
    order: &MonomialOrder,
) -> MathResult<Vec<Expression>> {
    // Convert expressions to sparse polynomials
    let mut basis: Vec<SparsePolynomial> = generators
        .iter()
        .filter_map(|expr| expression_to_sparse_polynomial(expr, variables))
        .filter(|poly| !poly.is_zero())
        .collect();

    if basis.is_empty() {
        return Ok(vec![Expression::integer(0)]);
    }

    // Initialize pairs queue
    let mut pairs = VecDeque::new();
    for i in 0..basis.len() {
        for j in (i + 1)..basis.len() {
            pairs.push_back((i, j));
        }
    }

    // Iteration limit for safety
    let max_iterations = 10000;
    let mut iterations = 0;

    while !pairs.is_empty() && iterations < max_iterations {
        iterations += 1;

        let (i, j) = pairs.pop_front().unwrap();

        // Check Buchberger criterion
        if can_skip_pair_sparse(i, j, &basis, order) {
            continue;
        }

        // Compute S-polynomial
        let s_poly = s_polynomial_sparse(&basis[i], &basis[j], order);

        if s_poly.is_zero() {
            continue;
        }

        // Reduce S-polynomial modulo current basis
        let basis_refs: Vec<&SparsePolynomial> = basis.iter().collect();
        let remainder = poly_reduce_completely_sparse(&s_poly, &basis_refs, order);

        if !remainder.is_zero() {
            let new_idx = basis.len();
            basis.push(remainder);

            // Add new pairs
            for k in 0..new_idx {
                pairs.push_back((k, new_idx));
            }
        }
    }

    if iterations >= max_iterations {
        return Err(MathError::MaxIterationsReached { max_iterations });
    }

    // Remove zero polynomials
    basis.retain(|p| !p.is_zero());

    // Auto-reduce: each polynomial reduced by all others
    let n = basis.len();
    for i in 0..n {
        let others: Vec<&SparsePolynomial> = basis
            .iter()
            .enumerate()
            .filter(|(j, _)| *j != i)
            .map(|(_, p)| p)
            .collect();

        if !others.is_empty() {
            let reduced = poly_reduce_completely_sparse(&basis[i], &others, order);
            basis[i] = reduced;
        }
    }

    basis.retain(|p| !p.is_zero());

    // Convert back to expressions
    let result_exprs: Vec<Expression> = basis
        .iter()
        .map(|poly| sparse_polynomial_to_expression(poly, variables))
        .collect();

    Ok(result_exprs)
}

/// Compute S-polynomial of two sparse polynomials
fn s_polynomial_sparse(
    f: &SparsePolynomial,
    g: &SparsePolynomial,
    order: &MonomialOrder,
) -> SparsePolynomial {
    // Get leading monomials (unwrap is safe since polynomials are non-zero in basis)
    let lt_f = f.leading_monomial(order).expect("f is non-zero");
    let lt_g = g.leading_monomial(order).expect("g is non-zero");

    let lcm_mono = lt_f.lcm(lt_g);

    // S(f, g) = (lcm / LT(f)) * f - (lcm / LT(g)) * g
    let f_factor = lcm_mono.divide(lt_f);
    let g_factor = lcm_mono.divide(lt_g);

    let scaled_f = f.mul_monomial(&f_factor);
    let scaled_g = g.mul_monomial(&g_factor);

    scaled_f.sub(&scaled_g)
}

/// Reduce polynomial modulo a set of polynomials
fn poly_reduce_completely_sparse(
    poly: &SparsePolynomial,
    basis: &[&SparsePolynomial],
    order: &MonomialOrder,
) -> SparsePolynomial {
    let mut remainder = poly.clone();

    loop {
        let mut reduced = false;

        for divisor in basis {
            if divisor.is_zero() {
                continue;
            }

            let divisor_lt = divisor.leading_monomial(order);

            // Try to reduce remainder by divisor
            while !remainder.is_zero() {
                let remainder_lt = remainder.leading_monomial(order);

                if let (Some(r_lt), Some(d_lt)) = (remainder_lt, divisor_lt) {
                    if let Some(quotient_mono) = r_lt.try_divide(d_lt) {
                        // remainder -= (quotient_mono * divisor)
                        let to_subtract = divisor.mul_monomial(&quotient_mono);
                        remainder = remainder.sub(&to_subtract);
                        reduced = true;
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            }
        }

        if !reduced {
            break;
        }
    }

    remainder
}

/// Check if pair can be skipped (Buchberger criterion)
fn can_skip_pair_sparse(
    i: usize,
    j: usize,
    basis: &[SparsePolynomial],
    order: &MonomialOrder,
) -> bool {
    let lt_i = basis[i].leading_monomial(order);
    let lt_j = basis[j].leading_monomial(order);

    if let (Some(mono_i), Some(mono_j)) = (lt_i, lt_j) {
        are_relatively_prime_sparse(mono_i, mono_j)
    } else {
        false
    }
}

/// Check if two monomials are relatively prime
fn are_relatively_prime_sparse(mono1: &Monomial, mono2: &Monomial) -> bool {
    for (e1, e2) in mono1.exponents.iter().zip(mono2.exponents.iter()) {
        if *e1 > 0 && *e2 > 0 {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::symbol;
    use std::sync::Arc;

    #[test]
    fn test_efficient_buchberger_simple() {
        let x = symbol!(x);
        let y = symbol!(y);
        let vars = vec![x.clone(), y.clone()];

        let f1 = Expression::add(vec![
            x.clone().into(),
            Expression::Mul(Arc::new(vec![
                Expression::integer(-1),
                Expression::symbol(y.clone()),
            ])),
        ]);
        let f2 = Expression::add(vec![
            Expression::pow(y.clone().into(), Expression::integer(2)),
            Expression::integer(-1),
        ]);

        let gb = efficient_buchberger_algorithm(&[f1, f2], &vars, &MonomialOrder::Lex)
            .expect("Should converge for simple system");

        assert!(!gb.is_empty());
        assert!(gb.len() >= 2);
    }

    #[test]
    fn test_efficient_buchberger_circle_line() {
        let x = symbol!(x);
        let y = symbol!(y);
        let vars = vec![x.clone(), y.clone()];

        // Circle: x² + y² - 1 = 0
        let f1 = Expression::add(vec![
            Expression::pow(x.clone().into(), Expression::integer(2)),
            Expression::pow(y.clone().into(), Expression::integer(2)),
            Expression::integer(-1),
        ]);

        // Line: x - y = 0
        let f2 = Expression::add(vec![
            x.clone().into(),
            Expression::Mul(Arc::new(vec![
                Expression::integer(-1),
                Expression::symbol(y.clone()),
            ])),
        ]);

        let start = std::time::Instant::now();
        let gb = efficient_buchberger_algorithm(&[f1, f2], &vars, &MonomialOrder::Lex)
            .expect("Should converge for circle-line system");
        let elapsed = start.elapsed();

        assert!(!gb.is_empty());
        // Should complete in < 1 second (SymPy does it in 0.00s)
        assert!(elapsed.as_secs() < 1, "Took {:?}, should be < 1s", elapsed);
    }
}
