//! Special Polynomial Families
//!
//! Provides access to classical orthogonal polynomial families
//! including Legendre, Chebyshev, Hermite, and Laguerre polynomials.
//!
//! This module re-exports from `functions::polynomials` while providing
//! a unified interface through the polynomial module.
//!
//! # Polynomial Families
//!
//! ## Legendre Polynomials P_n(x)
//!
//! Solutions to Legendre's differential equation. Orthogonal on [-1, 1]
//! with weight function w(x) = 1.
//!
//! ## Chebyshev Polynomials T_n(x) and U_n(x)
//!
//! First and second kind Chebyshev polynomials. Orthogonal on [-1, 1]
//! with weight function w(x) = 1/sqrt(1-x²).
//!
//! ## Hermite Polynomials H_n(x)
//!
//! Solutions to Hermite's differential equation. Orthogonal on (-∞, ∞)
//! with weight function w(x) = exp(-x²).
//!
//! ## Laguerre Polynomials L_n(x)
//!
//! Solutions to Laguerre's differential equation. Orthogonal on [0, ∞)
//! with weight function w(x) = exp(-x).

// Re-export from functions::polynomials for access via core::polynomial::special_families
pub use crate::functions::polynomials::{
    chebyshev, evaluation, hermite, laguerre, legendre, symbolic, PolynomialIntelligence,
};

// Re-export specific functions for convenience
pub use crate::functions::polynomials::evaluation::{
    evaluate_chebyshev_first_numerical, evaluate_chebyshev_second_numerical,
    evaluate_hermite_numerical, evaluate_laguerre_numerical, evaluate_legendre_numerical,
};
pub use crate::functions::polynomials::symbolic::{
    expand_chebyshev_first_symbolic, expand_chebyshev_second_symbolic, expand_hermite_symbolic,
    expand_laguerre_symbolic, expand_legendre_symbolic,
};

use crate::core::{Expression, Symbol};
use crate::pattern::Substitutable;

/// Trait for generating orthogonal polynomial expressions
///
/// This trait provides a unified interface for all orthogonal polynomial families,
/// allowing generic code to work with any family.
pub trait OrthogonalPolynomial {
    /// Generate the n-th polynomial in the family as an Expression
    ///
    /// # Arguments
    ///
    /// * `n` - The degree/order of the polynomial
    /// * `var` - The variable symbol
    ///
    /// # Returns
    ///
    /// The n-th polynomial expression in the family
    fn polynomial(n: usize, var: &Symbol) -> Expression;

    /// Evaluate the n-th polynomial at a specific value
    ///
    /// # Arguments
    ///
    /// * `n` - The degree/order of the polynomial
    /// * `x` - The value to evaluate at
    ///
    /// # Returns
    ///
    /// The evaluated polynomial value
    fn evaluate(n: usize, x: f64) -> f64;

    /// Get the recurrence relation coefficients
    ///
    /// For the relation: P_{n+1}(x) = (a_n * x + b_n) * P_n(x) - c_n * P_{n-1}(x)
    ///
    /// # Returns
    ///
    /// Tuple (a_n, b_n, c_n)
    fn recurrence_coefficients(n: usize) -> (f64, f64, f64);
}

/// Legendre polynomial family
pub struct Legendre;

impl OrthogonalPolynomial for Legendre {
    fn polynomial(n: usize, var: &Symbol) -> Expression {
        // Use existing implementation and substitute variable
        let expr = expand_legendre_symbolic(n);
        if var.name() == "x" {
            expr
        } else {
            expr.subs(&Expression::symbol("x"), &Expression::symbol(var.clone()))
        }
    }

    fn evaluate(n: usize, x: f64) -> f64 {
        let result = evaluate_legendre_numerical(&[n as f64, x]);
        result.first().copied().unwrap_or(0.0)
    }

    fn recurrence_coefficients(n: usize) -> (f64, f64, f64) {
        let n_f64 = n as f64;
        let a_n = (2.0 * n_f64 + 1.0) / (n_f64 + 1.0);
        let b_n = 0.0;
        let c_n = n_f64 / (n_f64 + 1.0);
        (a_n, b_n, c_n)
    }
}

/// Chebyshev polynomial family (first kind)
pub struct ChebyshevT;

impl OrthogonalPolynomial for ChebyshevT {
    fn polynomial(n: usize, var: &Symbol) -> Expression {
        let expr = expand_chebyshev_first_symbolic(n);
        if var.name() == "x" {
            expr
        } else {
            expr.subs(&Expression::symbol("x"), &Expression::symbol(var.clone()))
        }
    }

    fn evaluate(n: usize, x: f64) -> f64 {
        let result = evaluate_chebyshev_first_numerical(&[n as f64, x]);
        result.first().copied().unwrap_or(0.0)
    }

    fn recurrence_coefficients(n: usize) -> (f64, f64, f64) {
        if n == 0 {
            (1.0, 0.0, 0.0)
        } else {
            (2.0, 0.0, 1.0)
        }
    }
}

/// Chebyshev polynomial family (second kind)
pub struct ChebyshevU;

impl OrthogonalPolynomial for ChebyshevU {
    fn polynomial(n: usize, var: &Symbol) -> Expression {
        let expr = expand_chebyshev_second_symbolic(n);
        if var.name() == "x" {
            expr
        } else {
            expr.subs(&Expression::symbol("x"), &Expression::symbol(var.clone()))
        }
    }

    fn evaluate(n: usize, x: f64) -> f64 {
        let result = evaluate_chebyshev_second_numerical(&[n as f64, x]);
        result.first().copied().unwrap_or(0.0)
    }

    fn recurrence_coefficients(n: usize) -> (f64, f64, f64) {
        if n == 0 {
            (2.0, 0.0, 0.0)
        } else {
            (2.0, 0.0, 1.0)
        }
    }
}

/// Hermite polynomial family (physicist's convention)
pub struct Hermite;

impl OrthogonalPolynomial for Hermite {
    fn polynomial(n: usize, var: &Symbol) -> Expression {
        let expr = expand_hermite_symbolic(n);
        if var.name() == "x" {
            expr
        } else {
            expr.subs(&Expression::symbol("x"), &Expression::symbol(var.clone()))
        }
    }

    fn evaluate(n: usize, x: f64) -> f64 {
        let result = evaluate_hermite_numerical(&[n as f64, x]);
        result.first().copied().unwrap_or(0.0)
    }

    fn recurrence_coefficients(n: usize) -> (f64, f64, f64) {
        let n_f64 = n as f64;
        (2.0, 0.0, 2.0 * n_f64)
    }
}

/// Laguerre polynomial family
pub struct Laguerre;

impl OrthogonalPolynomial for Laguerre {
    fn polynomial(n: usize, var: &Symbol) -> Expression {
        let expr = expand_laguerre_symbolic(n);
        if var.name() == "x" {
            expr
        } else {
            expr.subs(&Expression::symbol("x"), &Expression::symbol(var.clone()))
        }
    }

    fn evaluate(n: usize, x: f64) -> f64 {
        let result = evaluate_laguerre_numerical(&[n as f64, x]);
        result.first().copied().unwrap_or(0.0)
    }

    fn recurrence_coefficients(n: usize) -> (f64, f64, f64) {
        let n_f64 = n as f64;
        let a_n = -1.0 / (n_f64 + 1.0);
        let b_n = (2.0 * n_f64 + 1.0) / (n_f64 + 1.0);
        let c_n = n_f64 / (n_f64 + 1.0);
        (a_n, b_n, c_n)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::symbol;

    #[test]
    fn test_legendre_base_cases() {
        let x = symbol!(x);
        let p0 = Legendre::polynomial(0, &x);
        let p1 = Legendre::polynomial(1, &x);

        assert_eq!(p0, Expression::integer(1));
        assert_eq!(p1, Expression::symbol(x));
    }

    #[test]
    fn test_legendre_eval() {
        // P_0(0.5) = 1
        assert!((Legendre::evaluate(0, 0.5) - 1.0).abs() < 1e-10);
        // P_1(0.5) = 0.5
        assert!((Legendre::evaluate(1, 0.5) - 0.5).abs() < 1e-10);
        // P_2(0.5) = (3*0.25 - 1)/2 = -0.125
        assert!((Legendre::evaluate(2, 0.5) - (-0.125)).abs() < 1e-10);
    }

    #[test]
    fn test_chebyshev_t_base_cases() {
        let x = symbol!(x);
        let t0 = ChebyshevT::polynomial(0, &x);
        let t1 = ChebyshevT::polynomial(1, &x);

        assert_eq!(t0, Expression::integer(1));
        assert_eq!(t1, Expression::symbol(x));
    }

    #[test]
    fn test_chebyshev_t_eval() {
        // T_0(0.5) = 1
        assert!((ChebyshevT::evaluate(0, 0.5) - 1.0).abs() < 1e-10);
        // T_1(0.5) = 0.5
        assert!((ChebyshevT::evaluate(1, 0.5) - 0.5).abs() < 1e-10);
        // T_2(0.5) = 2*0.25 - 1 = -0.5
        assert!((ChebyshevT::evaluate(2, 0.5) - (-0.5)).abs() < 1e-10);
    }

    #[test]
    fn test_chebyshev_u_eval() {
        // U_0(0.5) = 1
        assert!((ChebyshevU::evaluate(0, 0.5) - 1.0).abs() < 1e-10);
        // U_1(0.5) = 2*0.5 = 1
        assert!((ChebyshevU::evaluate(1, 0.5) - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_hermite_base_cases() {
        let x = symbol!(x);
        let h0 = Hermite::polynomial(0, &x);
        let _h1 = Hermite::polynomial(1, &x);

        assert_eq!(h0, Expression::integer(1));
        // H_1(x) = 2x in physicist's convention
    }

    #[test]
    fn test_hermite_eval() {
        // H_0(0.5) = 1
        assert!((Hermite::evaluate(0, 0.5) - 1.0).abs() < 1e-10);
        // H_1(0.5) = 2*0.5 = 1
        assert!((Hermite::evaluate(1, 0.5) - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_laguerre_base_cases() {
        let x = symbol!(x);
        let l0 = Laguerre::polynomial(0, &x);

        assert_eq!(l0, Expression::integer(1));
    }

    #[test]
    fn test_laguerre_eval() {
        // L_0(0.5) = 1
        assert!((Laguerre::evaluate(0, 0.5) - 1.0).abs() < 1e-10);
        // L_1(0.5) = 1 - 0.5 = 0.5
        assert!((Laguerre::evaluate(1, 0.5) - 0.5).abs() < 1e-10);
    }

    #[test]
    fn test_recurrence_coefficients() {
        // Test that recurrence coefficients are sensible
        let (a, b, c) = Legendre::recurrence_coefficients(2);
        assert!(a > 0.0);
        assert_eq!(b, 0.0);
        assert!(c > 0.0);

        let (a, _, c) = ChebyshevT::recurrence_coefficients(2);
        assert_eq!(a, 2.0);
        assert_eq!(c, 1.0);
    }

    #[test]
    fn test_variable_substitution() {
        let t = symbol!(t);
        let p1 = Legendre::polynomial(1, &t);
        // Should be the variable t, not x
        assert_eq!(p1, Expression::symbol(t));
    }
}
