//! Monomial Orderings for Gröbner Basis Computation
//!
//! Implements standard monomial orderings: lexicographic, graded lexicographic,
//! and graded reverse lexicographic. These orderings are fundamental to
//! Gröbner basis algorithms.

use crate::core::{Expression, Number, Symbol};
use std::cmp::Ordering;

/// Monomial ordering types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MonomialOrder {
    /// Lexicographic ordering (Lex)
    ///
    /// Compares monomials by exponents left to right
    /// x^a1 y^a2 > x^b1 y^b2 if first non-equal exponent ai > bi
    Lex,

    /// Graded lexicographic ordering (Grlex)
    ///
    /// First compare total degree, then use lex for ties
    Grlex,

    /// Graded reverse lexicographic ordering (Grevlex)
    ///
    /// First compare total degree, then reverse lex for ties
    Grevlex,
}

/// Trait for comparing monomials according to a specific ordering
pub trait MonomialOrdering {
    /// Compare two monomials using this ordering
    ///
    /// # Arguments
    ///
    /// * `mono1` - First monomial
    /// * `mono2` - Second monomial
    /// * `variables` - Ordered list of variables
    ///
    /// # Returns
    ///
    /// Ordering result (Less, Equal, Greater)
    fn compare_monomials(
        &self,
        mono1: &Expression,
        mono2: &Expression,
        variables: &[Symbol],
    ) -> Ordering;

    /// Get the leading monomial of a polynomial
    ///
    /// # Arguments
    ///
    /// * `poly` - Polynomial expression
    /// * `variables` - Ordered list of variables
    ///
    /// # Returns
    ///
    /// The leading monomial according to this ordering
    fn leading_monomial(&self, poly: &Expression, variables: &[Symbol]) -> Expression;

    /// Get the leading coefficient of a polynomial
    ///
    /// # Arguments
    ///
    /// * `poly` - Polynomial expression
    /// * `variables` - Ordered list of variables
    ///
    /// # Returns
    ///
    /// The coefficient of the leading monomial
    fn leading_coefficient(&self, poly: &Expression, variables: &[Symbol]) -> Expression;
}

impl MonomialOrdering for MonomialOrder {
    fn compare_monomials(
        &self,
        mono1: &Expression,
        mono2: &Expression,
        variables: &[Symbol],
    ) -> Ordering {
        let exp1 = extract_exponents(mono1, variables);
        let exp2 = extract_exponents(mono2, variables);

        match self {
            MonomialOrder::Lex => compare_lex(&exp1, &exp2),
            MonomialOrder::Grlex => compare_grlex(&exp1, &exp2),
            MonomialOrder::Grevlex => compare_grevlex(&exp1, &exp2),
        }
    }

    fn leading_monomial(&self, poly: &Expression, variables: &[Symbol]) -> Expression {
        match poly {
            Expression::Add(terms) => {
                let mut leading = terms[0].clone();
                for term in &terms[1..] {
                    if self.compare_monomials(term, &leading, variables) == Ordering::Greater {
                        leading = term.clone();
                    }
                }
                extract_monomial_part(&leading)
            }
            _ => extract_monomial_part(poly),
        }
    }

    fn leading_coefficient(&self, poly: &Expression, variables: &[Symbol]) -> Expression {
        match poly {
            Expression::Add(terms) => {
                let mut leading = terms[0].clone();
                for term in &terms[1..] {
                    if self.compare_monomials(term, &leading, variables) == Ordering::Greater {
                        leading = term.clone();
                    }
                }
                extract_coefficient(&leading)
            }
            _ => extract_coefficient(poly),
        }
    }
}

/// Extract exponents of a monomial as a vector
///
/// # Arguments
///
/// * `mono` - Monomial expression (possibly with coefficient)
/// * `variables` - Ordered list of variables
///
/// # Returns
///
/// Vector of exponents in the same order as variables
fn extract_exponents(mono: &Expression, variables: &[Symbol]) -> Vec<i64> {
    let mut exponents = vec![0i64; variables.len()];
    let mono_part = extract_monomial_part(mono);

    match mono_part {
        Expression::Symbol(s) => {
            if let Some(idx) = variables.iter().position(|v| v == &s) {
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
                let factor_exp = extract_exponents(factor, variables);
                for (i, e) in factor_exp.iter().enumerate() {
                    exponents[i] += e;
                }
            }
        }
        _ => {}
    }

    exponents
}

/// Extract the monomial part (without coefficient) of a term
fn extract_monomial_part(term: &Expression) -> Expression {
    match term {
        Expression::Mul(factors) => {
            let mut mono_factors = Vec::new();
            // Dereference Box<Vec<Expression>> to iterate
            for factor in factors.iter() {
                if !is_numeric(factor) {
                    mono_factors.push(factor.clone());
                }
            }
            if mono_factors.is_empty() {
                Expression::integer(1)
            } else if mono_factors.len() == 1 {
                mono_factors[0].clone()
            } else {
                Expression::mul(mono_factors)
            }
        }
        _ => {
            if is_numeric(term) {
                Expression::integer(1)
            } else {
                term.clone()
            }
        }
    }
}

/// Extract the coefficient of a term
fn extract_coefficient(term: &Expression) -> Expression {
    match term {
        Expression::Mul(factors) => {
            let mut coeffs = Vec::new();
            // Dereference Box<Vec<Expression>> to iterate
            for factor in factors.iter() {
                if is_numeric(factor) {
                    coeffs.push(factor.clone());
                }
            }
            if coeffs.is_empty() {
                Expression::integer(1)
            } else if coeffs.len() == 1 {
                coeffs[0].clone()
            } else {
                Expression::mul(coeffs)
            }
        }
        Expression::Number(_) => term.clone(),
        _ => Expression::integer(1),
    }
}

/// Check if expression is a numeric constant
fn is_numeric(expr: &Expression) -> bool {
    matches!(expr, Expression::Number(_))
}

/// Lexicographic comparison of exponent vectors
fn compare_lex(exp1: &[i64], exp2: &[i64]) -> Ordering {
    for (e1, e2) in exp1.iter().zip(exp2.iter()) {
        match e1.cmp(e2) {
            Ordering::Equal => continue,
            other => return other,
        }
    }
    Ordering::Equal
}

/// Graded lexicographic comparison
fn compare_grlex(exp1: &[i64], exp2: &[i64]) -> Ordering {
    let deg1: i64 = exp1.iter().sum();
    let deg2: i64 = exp2.iter().sum();

    match deg1.cmp(&deg2) {
        Ordering::Equal => compare_lex(exp1, exp2),
        other => other,
    }
}

/// Graded reverse lexicographic comparison
fn compare_grevlex(exp1: &[i64], exp2: &[i64]) -> Ordering {
    let deg1: i64 = exp1.iter().sum();
    let deg2: i64 = exp2.iter().sum();

    match deg1.cmp(&deg2) {
        Ordering::Equal => {
            for (e1, e2) in exp1.iter().zip(exp2.iter()).rev() {
                match e2.cmp(e1) {
                    Ordering::Equal => continue,
                    other => return other,
                }
            }
            Ordering::Equal
        }
        other => other,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::symbol;

    #[test]
    fn test_extract_exponents_simple() {
        let x = symbol!(x);
        let y = symbol!(y);
        let vars = vec![x.clone(), y.clone()];

        let mono = Expression::mul(vec![
            Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
            Expression::symbol(y.clone()),
        ]);

        let exp = extract_exponents(&mono, &vars);
        assert_eq!(exp, vec![2, 1]);
    }

    #[test]
    fn test_lex_ordering() {
        let x = symbol!(x);
        let y = symbol!(y);
        let vars = vec![x.clone(), y.clone()];

        let mono1 = Expression::pow(Expression::symbol(x.clone()), Expression::integer(2));
        let mono2 = Expression::pow(Expression::symbol(y.clone()), Expression::integer(3));

        let order = MonomialOrder::Lex;
        let cmp = order.compare_monomials(&mono1, &mono2, &vars);

        assert_eq!(cmp, Ordering::Greater);
    }

    #[test]
    fn test_grlex_ordering() {
        let x = symbol!(x);
        let y = symbol!(y);
        let vars = vec![x.clone(), y.clone()];

        let mono1 = Expression::pow(Expression::symbol(x.clone()), Expression::integer(1));
        let mono2 = Expression::pow(Expression::symbol(y.clone()), Expression::integer(2));

        let order = MonomialOrder::Grlex;
        let cmp = order.compare_monomials(&mono1, &mono2, &vars);

        assert_eq!(cmp, Ordering::Less);
    }

    #[test]
    fn test_grevlex_ordering() {
        let x = symbol!(x);
        let y = symbol!(y);
        let vars = vec![x.clone(), y.clone()];

        let mono1 = Expression::mul(vec![
            Expression::symbol(x.clone()),
            Expression::symbol(y.clone()),
        ]);
        let mono2 = Expression::pow(Expression::symbol(x.clone()), Expression::integer(2));

        let order = MonomialOrder::Grevlex;
        let cmp = order.compare_monomials(&mono1, &mono2, &vars);

        assert_eq!(cmp, Ordering::Greater);
    }

    #[test]
    fn test_leading_monomial() {
        let x = symbol!(x);
        let y = symbol!(y);
        let vars = vec![x.clone(), y.clone()];

        let poly = Expression::add(vec![
            Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
            Expression::mul(vec![
                Expression::integer(3),
                Expression::symbol(y.clone()),
            ]),
        ]);

        let order = MonomialOrder::Lex;
        let lm = order.leading_monomial(&poly, &vars);

        assert_eq!(
            lm,
            Expression::pow(Expression::symbol(x), Expression::integer(2))
        );
    }
}
