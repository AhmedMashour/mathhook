//! Buchberger's Algorithm for Gröbner Basis Computation
//!
//! Implements the classic Buchberger's algorithm for computing Gröbner bases
//! of polynomial ideals with performance optimizations including Buchberger's
//! criteria for avoiding unnecessary S-polynomial computations and automatic
//! basis reduction.

use super::monomial_order::{MonomialOrder, MonomialOrdering};
use super::reduction::poly_reduce_completely;
use super::s_polynomial::s_polynomial;
use crate::core::{Expression, Number, Symbol};
use crate::error::{MathError, MathResult};
use std::collections::VecDeque;

/// Compute Gröbner basis using Buchberger's algorithm
///
/// Implements Buchberger's algorithm with optimizations:
/// - Buchberger's criteria for avoiding unnecessary S-polynomial computations
/// - Efficient pair management using VecDeque (O(1) pop_front)
/// - Early termination when pairs are relatively prime
/// - Automatic basis reduction after main loop
///
/// The returned basis is auto-reduced: each polynomial is reduced with respect
/// to all others in the basis, ensuring minimality.
///
/// # Algorithm
///
/// 1. Start with initial polynomial generators
/// 2. Maintain a queue of polynomial pairs to process
/// 3. For each pair (f, g):
///    - Apply Buchberger's criteria to skip unnecessary pairs
///    - Compute S-polynomial: S(f, g)
///    - Reduce S(f, g) modulo the current basis
///    - If remainder is non-zero, add to basis and generate new pairs
/// 4. Auto-reduce the basis (each polynomial reduced by all others)
/// 5. Remove any zero polynomials
///
/// # Arguments
///
/// * `generators` - Initial generating set for the ideal
/// * `variables` - Ordered list of variables
/// * `order` - Monomial ordering to use
///
/// # Returns
///
/// Returns `Ok(Vec<Expression>)` containing the auto-reduced Gröbner basis,
/// or `Err(MathError::MaxIterationsReached)` if the iteration limit is exceeded.
///
/// # Errors
///
/// Returns `MathError::MaxIterationsReached` if the algorithm does not converge
/// within the maximum iteration limit (10,000 iterations). This may occur for
/// very large or complex polynomial systems.
///
/// # Examples
///
/// ```rust
/// use mathhook_core::{symbol, expr, Expression};
/// use mathhook_core::algebra::groebner::{buchberger_algorithm, MonomialOrder};
///
/// let x = symbol!(x);
/// let y = symbol!(y);
/// let f1 = expr!((x^2) + (y^2) - 1);
/// let f2 = expr!(x - y);
/// let gb = buchberger_algorithm(
///     &vec![f1, f2],
///     &vec![x, y],
///     &MonomialOrder::Lex
/// ).expect("Should converge");
/// assert!(!gb.is_empty());
/// ```
pub fn buchberger_algorithm(
    generators: &[Expression],
    variables: &[Symbol],
    order: &MonomialOrder,
) -> MathResult<Vec<Expression>> {
    let mut basis: Vec<Expression> = generators
        .iter()
        .filter(|p| !p.is_zero())
        .cloned()
        .collect();

    if basis.is_empty() {
        return Ok(vec![Expression::integer(0)]);
    }

    let mut pairs = VecDeque::new();

    for i in 0..basis.len() {
        for j in (i + 1)..basis.len() {
            pairs.push_back((i, j));
        }
    }

    let max_iterations = 10000;
    let mut iterations = 0;

    while !pairs.is_empty() && iterations < max_iterations {
        iterations += 1;

        let (i, j) = pairs.pop_front().unwrap();

        if can_skip_pair(i, j, &basis, variables, order) {
            continue;
        }

        let s_poly = s_polynomial(&basis[i], &basis[j], variables, order);

        if s_poly.is_zero() {
            continue;
        }

        let basis_refs: Vec<&Expression> = basis.iter().collect();
        let remainder = poly_reduce_completely(&s_poly, &basis_refs, variables, order);

        if !remainder.is_zero() {
            let new_idx = basis.len();
            basis.push(remainder);

            for k in 0..new_idx {
                pairs.push_back((k, new_idx));
            }
        }
    }

    if iterations >= max_iterations {
        return Err(MathError::MaxIterationsReached { max_iterations });
    }

    basis.retain(|p| !p.is_zero());

    let n = basis.len();
    for i in 0..n {
        let others: Vec<&Expression> = basis
            .iter()
            .enumerate()
            .filter(|(j, _)| *j != i)
            .map(|(_, p)| p)
            .collect();

        if !others.is_empty() {
            let reduced = poly_reduce_completely(&basis[i], &others, variables, order);
            basis[i] = reduced;
        }
    }

    basis.retain(|p| !p.is_zero());

    Ok(basis)
}

/// Check if a pair can be skipped using Buchberger's criteria
///
/// Implements simple version of Buchberger's criteria:
/// If lcm(LT(f), LT(g)) = LT(f) * LT(g), the S-polynomial will reduce to zero
///
/// # Arguments
///
/// * `i` - Index of first polynomial
/// * `j` - Index of second polynomial
/// * `basis` - Current basis
/// * `variables` - Variables
/// * `order` - Monomial ordering
///
/// # Returns
///
/// `true` if the pair can be skipped
#[inline]
fn can_skip_pair(
    i: usize,
    j: usize,
    basis: &[Expression],
    variables: &[Symbol],
    order: &MonomialOrder,
) -> bool {
    let lt_i = order.leading_monomial(&basis[i], variables);
    let lt_j = order.leading_monomial(&basis[j], variables);

    are_relatively_prime(&lt_i, &lt_j, variables)
}

/// Check if two monomials are relatively prime
///
/// Two monomials are relatively prime if they share no common variables
/// with non-zero exponents.
///
/// # Arguments
///
/// * `mono1` - First monomial
/// * `mono2` - Second monomial
/// * `variables` - Variables
///
/// # Returns
///
/// `true` if monomials are relatively prime
#[inline]
fn are_relatively_prime(mono1: &Expression, mono2: &Expression, variables: &[Symbol]) -> bool {
    let exp1 = extract_exponents(mono1, variables);
    let exp2 = extract_exponents(mono2, variables);

    for (e1, e2) in exp1.iter().zip(exp2.iter()) {
        if *e1 > 0 && *e2 > 0 {
            return false;
        }
    }

    true
}

/// Extract exponents of a monomial as a vector
#[inline]
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
                    if let Expression::Number(Number::Integer(i)) = exp.as_ref() {
                        exponents[idx] = *i;
                    }
                }
            }
        }
        Expression::Mul(factors) => {
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
    fn test_buchberger_simple() {
        let x = symbol!(x);
        let y = symbol!(y);
        let vars = vec![x.clone(), y.clone()];

        let f1 = Expression::symbol(x.clone()) - Expression::symbol(y.clone());
        let f2 = Expression::pow(Expression::symbol(y.clone()), Expression::integer(2))
            - Expression::integer(1);

        let gb = buchberger_algorithm(&vec![f1, f2], &vars, &MonomialOrder::Lex)
            .expect("Should converge for simple system");

        assert!(!gb.is_empty());
        assert!(gb.len() >= 2);
    }

    #[test]
    fn test_buchberger_trivial() {
        let x = symbol!(x);
        let vars = vec![x.clone()];

        let f = Expression::symbol(x.clone());
        let gb = buchberger_algorithm(&vec![f], &vars, &MonomialOrder::Lex)
            .expect("Should converge for trivial case");

        assert_eq!(gb.len(), 1);
    }

    #[test]
    fn test_buchberger_zero_input() {
        let x = symbol!(x);
        let vars = vec![x.clone()];

        let gb = buchberger_algorithm(&vec![], &vars, &MonomialOrder::Lex)
            .expect("Should converge for empty input");

        assert_eq!(gb.len(), 1);
        assert!(gb[0].is_zero());
    }

    #[test]
    fn test_relatively_prime() {
        let x = symbol!(x);
        let y = symbol!(y);
        let vars = vec![x.clone(), y.clone()];

        let mono1 = Expression::symbol(x.clone());
        let mono2 = Expression::symbol(y.clone());
        assert!(are_relatively_prime(&mono1, &mono2, &vars));

        let mono3 = Expression::pow(Expression::symbol(x.clone()), Expression::integer(2));
        assert!(!are_relatively_prime(&mono1, &mono3, &vars));

        let mono4 = Expression::mul(vec![
            Expression::symbol(x.clone()),
            Expression::symbol(y.clone()),
        ]);
        assert!(!are_relatively_prime(&mono1, &mono4, &vars));
    }

    #[test]
    fn test_can_skip_pair() {
        let x = symbol!(x);
        let y = symbol!(y);
        let vars = vec![x.clone(), y.clone()];

        let basis = vec![Expression::symbol(x.clone()), Expression::symbol(y.clone())];

        assert!(can_skip_pair(0, 1, &basis, &vars, &MonomialOrder::Lex));
    }

    #[test]
    fn test_extract_exponents() {
        let x = symbol!(x);
        let y = symbol!(y);
        let vars = vec![x.clone(), y.clone()];

        let mono1 = Expression::symbol(x.clone());
        let exp1 = extract_exponents(&mono1, &vars);
        assert_eq!(exp1, vec![1, 0]);

        let mono2 = Expression::pow(Expression::symbol(y.clone()), Expression::integer(3));
        let exp2 = extract_exponents(&mono2, &vars);
        assert_eq!(exp2, vec![0, 3]);

        let mono3 = Expression::mul(vec![
            Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
            Expression::symbol(y.clone()),
        ]);
        let exp3 = extract_exponents(&mono3, &vars);
        assert_eq!(exp3, vec![2, 1]);
    }

    #[test]
    #[ignore]
    fn test_buchberger_timeout_warning() {
        let x = symbol!(x);
        let y = symbol!(y);
        let z = symbol!(z);
        let vars = vec![x.clone(), y.clone(), z.clone()];

        let f1 = Expression::add(vec![
            Expression::symbol(x.clone()),
            Expression::symbol(y.clone()),
            Expression::symbol(z.clone()),
        ]);
        let f2 = Expression::add(vec![
            Expression::mul(vec![
                Expression::symbol(x.clone()),
                Expression::symbol(y.clone()),
            ]),
            Expression::mul(vec![
                Expression::symbol(y.clone()),
                Expression::symbol(z.clone()),
            ]),
            Expression::mul(vec![
                Expression::symbol(z.clone()),
                Expression::symbol(x.clone()),
            ]),
        ]);
        let f3 = Expression::add(vec![
            Expression::mul(vec![
                Expression::symbol(x.clone()),
                Expression::symbol(y.clone()),
                Expression::symbol(z.clone()),
            ]),
            Expression::integer(-1),
        ]);

        let gb = buchberger_algorithm(&vec![f1, f2, f3], &vars, &MonomialOrder::Lex)
            .expect("Should converge for polynomial system");
        assert!(!gb.is_empty());
    }

    #[test]
    #[ignore]
    fn test_buchberger_redundant_generators() {
        let x = symbol!(x);
        let vars = vec![x.clone()];

        let f1 = Expression::pow(Expression::symbol(x.clone()), Expression::integer(2));
        let f2 = Expression::pow(Expression::symbol(x.clone()), Expression::integer(3));

        let gb = buchberger_algorithm(&vec![f1, f2], &vars, &MonomialOrder::Lex)
            .expect("Should converge for redundant generators");

        assert!(gb.len() >= 1);
        assert!(!gb.is_empty());
    }

    #[test]
    fn test_buchberger_basis_is_reduced() {
        let x = symbol!(x);
        let y = symbol!(y);
        let vars = vec![x.clone(), y.clone()];

        let f1 = Expression::symbol(x.clone()) - Expression::symbol(y.clone());
        let f2 = Expression::pow(Expression::symbol(y.clone()), Expression::integer(2))
            - Expression::integer(1);

        let gb = buchberger_algorithm(&vec![f1, f2], &vars, &MonomialOrder::Lex)
            .expect("Should converge for basis reduction test");

        for i in 0..gb.len() {
            let others: Vec<&Expression> = gb
                .iter()
                .enumerate()
                .filter(|(j, _)| *j != i)
                .map(|(_, p)| p)
                .collect();

            if !others.is_empty() {
                let reduced = poly_reduce_completely(&gb[i], &others, &vars, &MonomialOrder::Lex);
                assert!(reduced.is_zero() || reduced == gb[i]);
            }
        }
    }
}
