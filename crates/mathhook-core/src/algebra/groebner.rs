//! Gröbner Basis Computation
//!
//! Implements Buchberger's algorithm for computing Gröbner bases of polynomial ideals.
//! Supports multiple monomial orderings and provides tools for ideal membership testing,
//! solving systems of polynomial equations, and computational algebraic geometry.

mod buchberger;
mod efficient_buchberger;
mod monomial_order;
mod reduction;
mod s_polynomial;

pub use buchberger::buchberger_algorithm;
pub use efficient_buchberger::efficient_buchberger_algorithm;
pub use monomial_order::{MonomialOrder, MonomialOrdering};
pub use reduction::{poly_reduce, poly_reduce_completely};
pub use s_polynomial::s_polynomial;

pub use crate::core::polynomial::sparse_polynomial::{
    expression_to_sparse_polynomial, sparse_polynomial_to_expression, Monomial, SparsePolynomial,
};

use crate::core::{Expression, Symbol};
use std::collections::HashSet;

/// Represents a Gröbner basis for a polynomial ideal
///
/// A Gröbner basis is a special generating set for a polynomial ideal that
/// has useful computational properties, analogous to row echelon form for
/// matrices or GCD for integers.
///
/// # Mathematical Background
///
/// For an ideal I = <f1, f2, ..., fn> in k[x1, ..., xm], a Gröbner basis
/// is a finite subset G of I such that:
/// 1. G generates I (every element of I is a polynomial combination of G)
/// 2. The leading terms of G generate the ideal of leading terms of I
///
/// # Applications
///
/// - Ideal membership testing: Check if f ∈ I
/// - Solving systems of polynomial equations
/// - Computing ideal operations (intersection, quotient, elimination)
/// - Implicitization in algebraic geometry
/// - Computational commutative algebra
#[derive(Debug, Clone)]
pub struct GroebnerBasis {
    /// The basis polynomials
    pub basis: Vec<Expression>,

    /// Variables in the polynomial ring
    pub variables: Vec<Symbol>,

    /// Monomial ordering used for computation
    pub ordering: MonomialOrder,

    /// Whether the basis is reduced
    pub is_reduced: bool,
}

impl GroebnerBasis {
    /// Create a new Gröbner basis from polynomials
    ///
    /// # Arguments
    ///
    /// * `polynomials` - Initial generating set for the ideal
    /// * `variables` - Variables in the polynomial ring
    /// * `ordering` - Monomial ordering to use
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// use mathhook_core::{symbol, expr, Expression};
    /// use mathhook_core::algebra::groebner::{GroebnerBasis, MonomialOrder};
    ///
    /// let x = symbol!(x);
    /// let y = symbol!(y);
    /// let f1 = Expression::add(vec![Expression::pow(x.clone().into(), Expression::integer(2)), Expression::pow(y.clone().into(), Expression::integer(2)), Expression::integer(-1)]);
    /// let f2 = Expression::add(vec![x.clone().into(), Expression::mul(vec![Expression::integer(-1), y.clone().into()])]);
    /// let gb = GroebnerBasis::new(
    ///     vec![f1, f2],
    ///     vec![x, y],
    ///     MonomialOrder::Lex
    /// );
    /// ```
    pub fn new(
        polynomials: Vec<Expression>,
        variables: Vec<Symbol>,
        ordering: MonomialOrder,
    ) -> Self {
        Self {
            basis: polynomials,
            variables,
            ordering,
            is_reduced: false,
        }
    }

    /// Compute the Gröbner basis using Buchberger's algorithm
    ///
    /// Transforms the initial generators into a Gröbner basis by computing
    /// S-polynomials and adding non-zero remainders to the basis.
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// use mathhook_core::{symbol, expr};
    /// use mathhook_core::algebra::groebner::{GroebnerBasis, MonomialOrder};
    ///
    /// let x = symbol!(x);
    /// let y = symbol!(y);
    /// let f1 = Expression::add(vec![Expression::pow(x.clone().into(), Expression::integer(2)), Expression::pow(y.clone().into(), Expression::integer(2)), Expression::integer(-1)]);
    /// let f2 = Expression::add(vec![x.clone().into(), Expression::mul(vec![Expression::integer(-1), y.clone().into()])]);
    /// let mut gb = GroebnerBasis::new(
    ///     vec![f1, f2],
    ///     vec![x, y],
    ///     MonomialOrder::Lex
    /// );
    /// gb.compute();
    /// ```
    pub fn compute(&mut self) {
        self.basis = efficient_buchberger_algorithm(&self.basis, &self.variables, &self.ordering)
            .expect("Efficient Buchberger algorithm should converge for valid polynomial ideals");
        self.is_reduced = false;
    }

    /// Compute the Gröbner basis with explicit error handling
    ///
    /// Returns `Ok(())` on success or `Err(MathError)` if computation times out
    /// or exceeds iteration limit.
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// use mathhook_core::{symbol, expr};
    /// use mathhook_core::algebra::groebner::{GroebnerBasis, MonomialOrder};
    ///
    /// let x = symbol!(x);
    /// let y = symbol!(y);
    /// let f1 = Expression::add(vec![Expression::pow(x.clone().into(), Expression::integer(2)), Expression::pow(y.clone().into(), Expression::integer(2)), Expression::integer(-1)]);
    /// let f2 = Expression::add(vec![x.clone().into(), Expression::mul(vec![Expression::integer(-1), y.clone().into()])]);
    /// let mut gb = GroebnerBasis::new(
    ///     vec![f1, f2],
    ///     vec![x, y],
    ///     MonomialOrder::Lex
    /// );
    /// if gb.compute_with_result().is_ok() {
    ///     // Computation succeeded
    /// } else {
    ///     // Computation timed out or exceeded iteration limit
    /// }
    /// ```
    pub fn compute_with_result(&mut self) -> crate::error::MathResult<()> {
        match efficient_buchberger_algorithm(&self.basis, &self.variables, &self.ordering) {
            Ok(basis) => {
                self.basis = basis;
                self.is_reduced = false;
                Ok(())
            }
            Err(e) => Err(e),
        }
    }

    /// Reduce the Gröbner basis to minimal form
    ///
    /// A reduced Gröbner basis has:
    /// 1. Leading coefficients are 1 (monic)
    /// 2. No monomial of any basis element is divisible by the leading
    ///    term of another basis element
    pub fn reduce(&mut self) {
        if self.is_reduced {
            return;
        }

        let mut reduced = Vec::new();

        for poly in &self.basis {
            if !poly.is_zero() {
                let mut p = poly.clone();
                let reduced_refs: Vec<&Expression> = reduced.iter().collect();
                p = poly_reduce_completely(&p, &reduced_refs, &self.variables, &self.ordering);

                if !p.is_zero() {
                    reduced.push(p);
                }
            }
        }

        self.basis = reduced;
        self.is_reduced = true;
    }

    /// Test if a polynomial is in the ideal generated by this basis
    ///
    /// # Arguments
    ///
    /// * `poly` - Polynomial to test for membership
    ///
    /// # Returns
    ///
    /// Returns `true` if the polynomial reduces to zero modulo the basis
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// use mathhook_core::{symbol, expr};
    /// use mathhook_core::algebra::groebner::{GroebnerBasis, MonomialOrder};
    ///
    /// let x = symbol!(x);
    /// let y = symbol!(y);
    /// let f1 = expr!(x - y);
    /// let f2 = Expression::add(vec![Expression::pow(y.clone().into(), Expression::integer(2)), Expression::integer(-1)]);
    /// let mut gb = GroebnerBasis::new(
    ///     vec![f1, f2],
    ///     vec![x.clone(), y.clone()],
    ///     MonomialOrder::Lex
    /// );
    /// gb.compute();
    ///
    /// let test = Expression::add(vec![Expression::pow(x.clone().into(), Expression::integer(2)), Expression::integer(-1)]);
    /// assert!(gb.contains(&test));
    /// ```
    pub fn contains(&self, poly: &Expression) -> bool {
        let basis_refs: Vec<&Expression> = self.basis.iter().collect();
        let reduced = poly_reduce_completely(poly, &basis_refs, &self.variables, &self.ordering);
        reduced.is_zero()
    }

    /// Get all variables that appear in the basis
    pub fn get_variables(&self) -> Vec<Symbol> {
        let mut vars = HashSet::new();
        for poly in &self.basis {
            for var in find_variables(poly) {
                vars.insert(var);
            }
        }
        vars.into_iter().collect()
    }
}

/// Extract all variables from an expression
fn find_variables(expr: &Expression) -> Vec<Symbol> {
    fn collect_symbols(expr: &Expression, symbols: &mut HashSet<Symbol>) {
        match expr {
            Expression::Symbol(s) => {
                symbols.insert(s.clone());
            }
            Expression::Add(terms) | Expression::Mul(terms) => {
                for term in terms.iter() {
                    collect_symbols(term, symbols);
                }
            }
            Expression::Pow(base, exp) => {
                collect_symbols(base, symbols);
                collect_symbols(exp, symbols);
            }
            Expression::Function { args, .. } => {
                for arg in args.iter() {
                    collect_symbols(arg, symbols);
                }
            }
            _ => {}
        }
    }

    let mut symbols = HashSet::new();
    collect_symbols(expr, &mut symbols);
    symbols.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::symbol;

    #[test]
    fn test_groebner_basis_creation() {
        let x = symbol!(x);
        let y = symbol!(y);
        let f1 = Expression::add(vec![
            Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
            Expression::integer(-1),
        ]);
        let gb = GroebnerBasis::new(vec![f1], vec![x, y], MonomialOrder::Lex);
        assert_eq!(gb.basis.len(), 1);
        assert_eq!(gb.variables.len(), 2);
    }

    #[test]
    fn test_groebner_basis_simple() {
        let x = symbol!(x);
        let y = symbol!(y);

        // Use subtraction operator instead of Expression::sub
        let f1 = Expression::symbol(x.clone()) - Expression::symbol(y.clone());
        let f2 = Expression::pow(Expression::symbol(y.clone()), Expression::integer(2))
            - Expression::integer(1);

        let mut gb =
            GroebnerBasis::new(vec![f1, f2], vec![x.clone(), y.clone()], MonomialOrder::Lex);

        gb.compute();

        assert!(!gb.basis.is_empty());
        assert!(gb.basis.len() >= 2);
    }

    #[test]
    #[ignore = "FIXME: Gröbner basis ideal membership test needs convergence tuning"]
    fn test_ideal_membership() {
        let x = symbol!(x);
        let y = symbol!(y);

        // Use subtraction operator instead of Expression::sub
        let f1 = Expression::symbol(x.clone()) - Expression::symbol(y.clone());
        let f2 = Expression::pow(Expression::symbol(y.clone()), Expression::integer(2))
            - Expression::integer(1);

        let mut gb =
            GroebnerBasis::new(vec![f1, f2], vec![x.clone(), y.clone()], MonomialOrder::Lex);

        gb.compute();

        let test = Expression::pow(Expression::symbol(x.clone()), Expression::integer(2))
            - Expression::integer(1);

        assert!(gb.contains(&test));
    }
}
