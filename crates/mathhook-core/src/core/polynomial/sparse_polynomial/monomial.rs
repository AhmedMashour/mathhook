//! Monomial representation for multivariate polynomials
//!
//! A monomial is represented as an exponent vector.
//! For polynomial in variables [x, y, z], monomial x²y³z is [2, 3, 1].

use std::cmp::Ordering;

use crate::algebra::groebner::MonomialOrder;

/// Monomial represented as exponent vector
///
/// For polynomial in variables [x, y, z], monomial x²y³z is [2, 3, 1]
/// This representation enables O(1) monomial comparison and operations.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Monomial {
    /// Exponents for each variable (index corresponds to variable order)
    pub exponents: Vec<usize>,
}

impl Monomial {
    /// Create monomial from exponent vector
    pub fn new(exponents: Vec<usize>) -> Self {
        Self { exponents }
    }

    /// Create constant monomial (all exponents zero)
    pub fn constant(num_vars: usize) -> Self {
        Self {
            exponents: vec![0; num_vars],
        }
    }

    /// Total degree of monomial
    pub fn degree(&self) -> usize {
        self.exponents.iter().sum()
    }

    /// Multiply two monomials (add exponents)
    pub fn mul(&self, other: &Self) -> Self {
        debug_assert_eq!(self.exponents.len(), other.exponents.len());
        Self {
            exponents: self
                .exponents
                .iter()
                .zip(&other.exponents)
                .map(|(a, b)| a + b)
                .collect(),
        }
    }

    /// Divide two monomials if divisible (subtract exponents)
    pub fn div(&self, other: &Self) -> Option<Self> {
        debug_assert_eq!(self.exponents.len(), other.exponents.len());

        let mut result = Vec::with_capacity(self.exponents.len());
        for (a, b) in self.exponents.iter().zip(&other.exponents) {
            if a < b {
                return None;
            }
            result.push(a - b);
        }
        Some(Self { exponents: result })
    }

    /// Least common multiple of two monomials
    pub fn lcm(&self, other: &Self) -> Self {
        debug_assert_eq!(self.exponents.len(), other.exponents.len());
        Self {
            exponents: self
                .exponents
                .iter()
                .zip(&other.exponents)
                .map(|(a, b)| (*a).max(*b))
                .collect(),
        }
    }

    /// Divide monomial by another (assumes divisibility)
    pub fn divide(&self, other: &Self) -> Self {
        debug_assert_eq!(self.exponents.len(), other.exponents.len());
        Self {
            exponents: self
                .exponents
                .iter()
                .zip(&other.exponents)
                .map(|(a, b)| {
                    assert!(*a >= *b, "Monomial not divisible");
                    a - b
                })
                .collect(),
        }
    }

    /// Try to divide monomial by another (returns None if not divisible)
    pub fn try_divide(&self, other: &Self) -> Option<Self> {
        debug_assert_eq!(self.exponents.len(), other.exponents.len());

        for (a, b) in self.exponents.iter().zip(&other.exponents) {
            if a < b {
                return None;
            }
        }

        Some(Self {
            exponents: self
                .exponents
                .iter()
                .zip(&other.exponents)
                .map(|(a, b)| a - b)
                .collect(),
        })
    }

    /// Compare monomials using given ordering
    pub fn cmp(&self, other: &Self, order: &MonomialOrder) -> Ordering {
        match order {
            MonomialOrder::Lex => {
                for (a, b) in self.exponents.iter().zip(&other.exponents) {
                    match a.cmp(b) {
                        Ordering::Equal => continue,
                        other => return other,
                    }
                }
                Ordering::Equal
            }
            MonomialOrder::Grlex => match self.degree().cmp(&other.degree()) {
                Ordering::Equal => self.cmp(other, &MonomialOrder::Lex),
                other => other,
            },
            MonomialOrder::Grevlex => match self.degree().cmp(&other.degree()) {
                Ordering::Equal => {
                    for (a, b) in self.exponents.iter().zip(&other.exponents).rev() {
                        match b.cmp(a) {
                            Ordering::Equal => continue,
                            other => return other,
                        }
                    }
                    Ordering::Equal
                }
                other => other,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_monomial_operations() {
        let m1 = Monomial::new(vec![2, 1]);
        let m2 = Monomial::new(vec![1, 3]);
        let product = m1.mul(&m2);
        assert_eq!(product.exponents, vec![3, 4]);
    }

    #[test]
    fn test_monomial_division() {
        let m1 = Monomial::new(vec![3, 2]);
        let m2 = Monomial::new(vec![1, 1]);
        let quotient = m1.div(&m2).unwrap();
        assert_eq!(quotient.exponents, vec![2, 1]);

        let m3 = Monomial::new(vec![1, 1]);
        let m4 = Monomial::new(vec![2, 1]);
        assert!(m3.div(&m4).is_none());
    }
}
