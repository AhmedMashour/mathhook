//! Efficient Sparse Polynomial Representation for Gröbner Basis
//!
//! This module provides an optimized polynomial representation using:
//! - HashMap<Monomial, Coefficient> for O(1) term lookup
//! - Efficient monomial ordering for Gröbner basis computation
//! - Optimized arithmetic operations (O(n) addition, O(n²) multiplication)
//!
//! This achieves SymPy-level performance by avoiding Expression tree explosion.

mod conversion;
mod monomial;
mod polynomial;

pub use conversion::{expression_to_sparse_polynomial, sparse_polynomial_to_expression};
pub use monomial::Monomial;
pub use polynomial::SparsePolynomial;
