//! Expression ↔ SparsePolynomial conversion
//!
//! Provides bidirectional conversion between MathHook Expression AST
//! and sparse polynomial representation for efficient multivariate computation.

use num_bigint::BigInt;
use num_rational::BigRational;
use num_traits::One;
use std::collections::HashMap;
use std::sync::Arc;

use super::monomial::Monomial;
use super::polynomial::SparsePolynomial;
use crate::algebra::groebner::MonomialOrder;
use crate::core::{Expression, Number, Symbol};

/// Convert Expression to SparsePolynomial
pub fn expression_to_sparse_polynomial(
    expr: &Expression,
    variables: &[Symbol],
) -> Option<SparsePolynomial> {
    let num_vars = variables.len();

    let mut terms = HashMap::new();

    extract_terms(expr, variables, &mut terms)?;

    Some(SparsePolynomial { terms, num_vars })
}

/// Recursive helper to extract polynomial terms
fn extract_terms(
    expr: &Expression,
    variables: &[Symbol],
    terms: &mut HashMap<Monomial, BigRational>,
) -> Option<()> {
    match expr {
        Expression::Number(Number::Integer(n)) => {
            let monomial = Monomial::constant(variables.len());
            let coeff = BigRational::from(BigInt::from(*n));
            terms
                .entry(monomial)
                .and_modify(|c| *c += &coeff)
                .or_insert(coeff);
            Some(())
        }
        Expression::Number(Number::Rational(r)) => {
            let monomial = Monomial::constant(variables.len());
            let coeff = (**r).clone();
            terms
                .entry(monomial)
                .and_modify(|c| *c += &coeff)
                .or_insert(coeff);
            Some(())
        }
        Expression::Symbol(s) => {
            let exponents = monomial_from_symbol(s, variables)?;
            let monomial = Monomial::new(exponents);
            let coeff = BigRational::one();
            terms
                .entry(monomial)
                .and_modify(|c| *c += &coeff)
                .or_insert(coeff);
            Some(())
        }
        Expression::Add(addends) => {
            for term in addends.iter() {
                extract_terms(term, variables, terms)?;
            }
            Some(())
        }
        Expression::Mul(factors) => {
            let (mono, coeff) = extract_monomial_and_coeff(factors, variables)?;
            terms
                .entry(mono)
                .and_modify(|c| *c += &coeff)
                .or_insert(coeff);
            Some(())
        }
        Expression::Pow(base, exp) => {
            if let (Expression::Symbol(s), Expression::Number(Number::Integer(n))) =
                (base.as_ref(), exp.as_ref())
            {
                if *n > 0 {
                    let mut exponents = vec![0; variables.len()];
                    if let Some(idx) = variables.iter().position(|v| v == s) {
                        exponents[idx] = *n as usize;
                    } else {
                        return None;
                    }
                    let monomial = Monomial::new(exponents);
                    let coeff = BigRational::one();
                    terms
                        .entry(monomial)
                        .and_modify(|c| *c += &coeff)
                        .or_insert(coeff);
                    return Some(());
                }
            }
            None
        }
        _ => None,
    }
}

/// Extract monomial and coefficient from product
fn extract_monomial_and_coeff(
    factors: &[Expression],
    variables: &[Symbol],
) -> Option<(Monomial, BigRational)> {
    let mut exponents = vec![0; variables.len()];
    let mut coeff = BigRational::one();

    for factor in factors {
        match factor {
            Expression::Number(Number::Integer(n)) => {
                coeff *= BigRational::from(BigInt::from(*n));
            }
            Expression::Number(Number::Rational(r)) => {
                coeff *= &**r;
            }
            Expression::Symbol(s) => {
                if let Some(idx) = variables.iter().position(|v| v == s) {
                    exponents[idx] += 1;
                } else {
                    return None;
                }
            }
            Expression::Pow(base, exp) => {
                if let (Expression::Symbol(s), Expression::Number(Number::Integer(n))) =
                    (base.as_ref(), exp.as_ref())
                {
                    if *n > 0 {
                        if let Some(idx) = variables.iter().position(|v| v == s) {
                            exponents[idx] += *n as usize;
                        } else {
                            return None;
                        }
                    } else {
                        return None;
                    }
                } else {
                    return None;
                }
            }
            _ => return None,
        }
    }

    Some((Monomial::new(exponents), coeff))
}

/// Get monomial exponents from symbol
fn monomial_from_symbol(sym: &Symbol, variables: &[Symbol]) -> Option<Vec<usize>> {
    let mut exponents = vec![0; variables.len()];
    if let Some(idx) = variables.iter().position(|v| v == sym) {
        exponents[idx] = 1;
        Some(exponents)
    } else {
        None
    }
}

/// Convert SparsePolynomial back to Expression
pub fn sparse_polynomial_to_expression(
    poly: &SparsePolynomial,
    variables: &[Symbol],
) -> Expression {
    if poly.is_zero() {
        return Expression::integer(0);
    }

    let mut terms_vec: Vec<_> = poly.terms.iter().collect();

    terms_vec.sort_by(|(m1, _), (m2, _)| m2.cmp(m1, &MonomialOrder::Lex));

    let expr_terms: Vec<Expression> = terms_vec
        .iter()
        .map(|(mono, coeff)| monomial_to_expression(mono, coeff, variables))
        .collect();

    if expr_terms.len() == 1 {
        expr_terms[0].clone()
    } else {
        Expression::Add(Arc::new(expr_terms))
    }
}

/// Convert monomial + coefficient to Expression
fn monomial_to_expression(
    monomial: &Monomial,
    coeff: &BigRational,
    variables: &[Symbol],
) -> Expression {
    let coeff_expr = if coeff == &BigRational::one() {
        None
    } else if coeff == &(-BigRational::one()) {
        Some(Expression::integer(-1))
    } else {
        Some(Expression::Number(Number::rational(coeff.clone())))
    };

    let mut mono_factors = Vec::new();
    for (idx, &exp) in monomial.exponents.iter().enumerate() {
        if exp > 0 {
            let var_expr = Expression::symbol(variables[idx].clone());
            if exp == 1 {
                mono_factors.push(var_expr);
            } else {
                mono_factors.push(Expression::pow(var_expr, Expression::integer(exp as i64)));
            }
        }
    }

    let mono_expr = if mono_factors.is_empty() {
        Expression::integer(1)
    } else if mono_factors.len() == 1 {
        mono_factors[0].clone()
    } else {
        Expression::Mul(Arc::new(mono_factors))
    };

    match (coeff_expr, mono_expr) {
        (None, mono) => mono,
        (Some(c), Expression::Number(Number::Integer(1))) => c,
        (Some(c), Expression::Mul(mono_factors)) => {
            let mut all_factors = vec![c];
            all_factors.extend(mono_factors.iter().cloned());
            Expression::Mul(Arc::new(all_factors))
        }
        (Some(c), mono) => Expression::Mul(Arc::new(vec![c, mono])),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::symbol;

    #[test]
    fn test_expression_conversion_roundtrip() {
        let x = symbol!(x);
        let y = symbol!(y);
        let vars = vec![x.clone(), y.clone()];

        let two_xy = Expression::Mul(Arc::new(vec![
            Expression::integer(2),
            Expression::symbol(x.clone()),
            Expression::symbol(y.clone()),
        ]));
        let expr = Expression::Add(Arc::new(vec![
            Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
            two_xy,
            Expression::pow(Expression::symbol(y.clone()), Expression::integer(2)),
            Expression::integer(-1),
        ]));

        let poly = expression_to_sparse_polynomial(&expr, &vars);
        if poly.is_none() {
            panic!("Failed to convert expression to polynomial: {:?}", expr);
        }
        let poly = poly.unwrap();
        let expr2 = sparse_polynomial_to_expression(&poly, &vars);

        let poly2 = expression_to_sparse_polynomial(&expr2, &vars);
        if poly2.is_none() {
            panic!("Failed to convert back: {:?}", expr2);
        }
        let poly2 = poly2.unwrap();
        assert_eq!(poly.terms, poly2.terms);
    }
}
