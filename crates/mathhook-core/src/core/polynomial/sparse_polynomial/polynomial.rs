//! Sparse polynomial representation using HashMap
//!
//! Efficient representation for polynomial computation:
//! - Addition: O(n) - merge hashmaps
//! - Multiplication: O(n²) - compute all monomial products
//! - No expression tree growth - coefficients are BigRational

use num_rational::BigRational;
use num_traits::Zero;
use std::collections::HashMap;

use super::monomial::Monomial;
use crate::algebra::groebner::MonomialOrder;

/// Sparse polynomial: HashMap<Monomial, Coefficient>
///
/// Efficient representation for polynomial computation:
/// - Addition: O(n) - merge hashmaps
/// - Multiplication: O(n²) - compute all monomial products
/// - No expression tree growth - coefficients are BigRational
#[derive(Debug, Clone)]
pub struct SparsePolynomial {
    /// Terms: monomial → coefficient
    /// Only non-zero coefficients are stored
    pub terms: HashMap<Monomial, BigRational>,

    /// Number of variables
    pub num_vars: usize,
}

impl SparsePolynomial {
    /// Create zero polynomial
    pub fn zero(num_vars: usize) -> Self {
        Self {
            terms: HashMap::new(),
            num_vars,
        }
    }

    /// Create polynomial from single term
    pub fn from_term(monomial: Monomial, coeff: BigRational, num_vars: usize) -> Self {
        let mut terms = HashMap::new();
        if !coeff.is_zero() {
            terms.insert(monomial, coeff);
        }
        Self { terms, num_vars }
    }

    /// Create constant polynomial
    pub fn constant(value: BigRational, num_vars: usize) -> Self {
        Self::from_term(Monomial::constant(num_vars), value, num_vars)
    }

    /// Check if polynomial is zero
    pub fn is_zero(&self) -> bool {
        self.terms.is_empty()
    }

    /// Get leading monomial (w.r.t. given ordering)
    pub fn leading_monomial(&self, order: &MonomialOrder) -> Option<&Monomial> {
        self.terms.keys().max_by(|a, b| a.cmp(b, order))
    }

    /// Get leading coefficient
    pub fn leading_coefficient(&self, order: &MonomialOrder) -> Option<&BigRational> {
        self.leading_monomial(order).and_then(|m| self.terms.get(m))
    }

    /// Get leading term (monomial + coefficient)
    pub fn leading_term(&self, order: &MonomialOrder) -> Option<(Monomial, BigRational)> {
        self.leading_monomial(order)
            .and_then(|m| self.terms.get(m).map(|c| (m.clone(), c.clone())))
    }

    /// Add two polynomials (O(n) operation)
    pub fn add(&self, other: &Self) -> Self {
        debug_assert_eq!(self.num_vars, other.num_vars);

        let mut result = self.terms.clone();

        for (monomial, coeff) in &other.terms {
            result
                .entry(monomial.clone())
                .and_modify(|c| *c += coeff)
                .or_insert_with(|| coeff.clone());
        }

        result.retain(|_, coeff| !coeff.is_zero());

        Self {
            terms: result,
            num_vars: self.num_vars,
        }
    }

    /// Negate polynomial
    pub fn neg(&self) -> Self {
        Self {
            terms: self.terms.iter().map(|(m, c)| (m.clone(), -c)).collect(),
            num_vars: self.num_vars,
        }
    }

    /// Subtract two polynomials
    pub fn sub(&self, other: &Self) -> Self {
        self.add(&other.neg())
    }

    /// Multiply two polynomials (O(n²) operation)
    pub fn mul(&self, other: &Self) -> Self {
        debug_assert_eq!(self.num_vars, other.num_vars);

        let mut result = HashMap::new();

        for (m1, c1) in &self.terms {
            for (m2, c2) in &other.terms {
                let monomial = m1.mul(m2);
                let coeff = c1 * c2;

                result
                    .entry(monomial)
                    .and_modify(|c: &mut BigRational| *c += &coeff)
                    .or_insert(coeff);
            }
        }

        result.retain(|_, coeff| !coeff.is_zero());

        Self {
            terms: result,
            num_vars: self.num_vars,
        }
    }

    /// Multiply by scalar
    pub fn scalar_mul(&self, scalar: &BigRational) -> Self {
        if scalar.is_zero() {
            return Self::zero(self.num_vars);
        }

        Self {
            terms: self
                .terms
                .iter()
                .map(|(m, c)| (m.clone(), c * scalar))
                .collect(),
            num_vars: self.num_vars,
        }
    }

    /// Multiply polynomial by a monomial
    pub fn mul_monomial(&self, monomial: &Monomial) -> Self {
        Self {
            terms: self
                .terms
                .iter()
                .map(|(m, c)| (m.mul(monomial), c.clone()))
                .collect(),
            num_vars: self.num_vars,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::conversion::expression_to_sparse_polynomial;
    use crate::core::Expression;
    use crate::symbol;

    #[test]
    fn test_polynomial_addition() {
        let x = symbol!(x);
        let y = symbol!(y);
        let vars = vec![x.clone(), y.clone()];

        let p1 = expression_to_sparse_polynomial(
            &Expression::add(vec![
                Expression::mul(vec![Expression::integer(2), Expression::symbol(x.clone())]),
                Expression::mul(vec![Expression::integer(3), Expression::symbol(y.clone())]),
            ]),
            &vars,
        )
        .unwrap();

        let p2 = expression_to_sparse_polynomial(
            &Expression::add(vec![
                Expression::symbol(x.clone()),
                Expression::symbol(y.clone()),
            ]),
            &vars,
        )
        .unwrap();

        let sum = p1.add(&p2);
        assert_eq!(sum.terms.len(), 2);
    }

    #[test]
    fn test_polynomial_multiplication() {
        let x = symbol!(x);
        let vars = vec![x.clone()];

        let p1 = expression_to_sparse_polynomial(
            &Expression::add(vec![Expression::symbol(x.clone()), Expression::integer(1)]),
            &vars,
        )
        .unwrap();

        let p2 = expression_to_sparse_polynomial(
            &Expression::add(vec![Expression::symbol(x.clone()), Expression::integer(2)]),
            &vars,
        )
        .unwrap();

        let product = p1.mul(&p2);
        assert_eq!(product.terms.len(), 3);
    }
}
