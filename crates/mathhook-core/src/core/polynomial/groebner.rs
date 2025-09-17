//! Groebner Basis Computation
//!
//! Provides Groebner basis algorithms for polynomial ideals.
//! This module re-exports from `algebra::groebner` while providing
//! a unified interface through the polynomial module.
//!
//! # Overview
//!
//! Groebner bases are a fundamental tool in computational algebraic geometry
//! and polynomial system solving. They provide canonical generators for
//! polynomial ideals, enabling:
//!
//! - Ideal membership testing
//! - Polynomial system solving
//! - Elimination of variables
//! - Geometric theorem proving
//!
//! # Algorithms
//!
//! - **Buchberger's Algorithm**: Classic algorithm for Groebner basis computation
//! - **Efficient Buchberger**: Optimized variant with pair selection strategies
//!
//! # Example
//!
//! ```rust
//! use mathhook_core::core::polynomial::groebner::{GroebnerBasis, MonomialOrder};
//! use mathhook_core::core::Expression;
//! use mathhook_core::symbol;
//!
//! let x = symbol!(x);
//! let y = symbol!(y);
//!
//! // f1 = x - y
//! let f1 = Expression::add(vec![
//!     Expression::symbol(x.clone()),
//!     Expression::mul(vec![Expression::integer(-1), Expression::symbol(y.clone())]),
//! ]);
//! // f2 = y^2 - 1
//! let f2 = Expression::add(vec![
//!     Expression::pow(Expression::symbol(y.clone()), Expression::integer(2)),
//!     Expression::integer(-1),
//! ]);
//!
//! let mut gb = GroebnerBasis::new(
//!     vec![f1, f2],
//!     vec![x.clone(), y.clone()],
//!     MonomialOrder::Lex
//! );
//! gb.compute();
//!
//! // The basis should have at least 2 polynomials
//! assert!(gb.basis.len() >= 2);
//! ```

pub use crate::algebra::groebner::{
    buchberger_algorithm, efficient_buchberger_algorithm, expression_to_sparse_polynomial,
    poly_reduce, poly_reduce_completely, s_polynomial, sparse_polynomial_to_expression,
    GroebnerBasis, Monomial, MonomialOrder, MonomialOrdering, SparsePolynomial,
};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::Expression;
    use crate::symbol;

    #[test]
    fn test_monomial_order_export() {
        let _lex = MonomialOrder::Lex;
        let _grlex = MonomialOrder::Grlex;
        let _grevlex = MonomialOrder::Grevlex;
    }

    #[test]
    fn test_sparse_polynomial_creation() {
        let mono = Monomial::new(vec![1, 2]);
        assert_eq!(mono.degree(), 3);
    }

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
    fn test_expression_conversion_exists() {
        let x = symbol!(x);
        let y = symbol!(y);

        let poly = Expression::add(vec![
            Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
            Expression::symbol(y.clone()),
        ]);

        let vars = vec![x, y];
        let sparse = expression_to_sparse_polynomial(&poly, &vars);

        assert!(sparse.is_some());
    }
}
