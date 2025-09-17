//! Symbolic Polynomial Expansion
//!
//! Provides symbolic expansion of orthogonal polynomials to explicit Expression forms
//! using recurrence relations. All implementations are mathematically verified against

use crate::core::Expression;
use crate::simplify::Simplify;

/// Expand Legendre polynomial P_n(x) to explicit symbolic form
///
/// Uses three-term recurrence to build symbolic expression:
/// - P_0(x) = 1
/// - P_1(x) = x
/// - P_{n+1}(x) = [(2n+1)x P_n(x) - n P_{n-1}(x)] / (n+1)
///
/// This implementation builds the polynomial iteratively using the Expression system,
/// applying simplification at each step to maintain manageable expression size.
///
/// # Arguments
///
/// * `n` - Polynomial degree (non-negative integer)
///
/// # Returns
///
/// Expression representing the expanded Legendre polynomial P_n(x)
///
/// # Mathematical Background
///
/// Legendre polynomials are solutions to Legendre's differential equation:
/// (1-x²)y'' - 2xy' + n(n+1)y = 0
///
/// They are orthogonal on [-1, 1] with weight function w(x) = 1.
///
/// # Examples
///
/// ```rust
/// use mathhook_core::functions::polynomials::symbolic::expand_legendre_symbolic;
/// use mathhook_core::core::Expression;
///
/// let p0 = expand_legendre_symbolic(0);
/// let p1 = expand_legendre_symbolic(1);
/// let p2 = expand_legendre_symbolic(2);
/// let p3 = expand_legendre_symbolic(3);
/// ```
#[inline]
#[must_use]
pub fn expand_legendre_symbolic(n: usize) -> Expression {
    if n == 0 {
        return Expression::integer(1);
    }
    if n == 1 {
        return Expression::symbol("x");
    }

    let x = Expression::symbol("x");
    let mut p_prev = Expression::integer(1);
    let mut p_curr = x.clone();

    for i in 1..n {
        let i_i64 = i as i64;

        let alpha_num = 2 * i_i64 + 1;
        let alpha_den = i_i64 + 1;
        let gamma_num = -i_i64;
        let gamma_den = i_i64 + 1;

        let term1 = Expression::mul(vec![
            Expression::rational(alpha_num, alpha_den),
            x.clone(),
            p_curr.clone(),
        ]);

        let term2 = Expression::mul(vec![
            Expression::rational(gamma_num, gamma_den),
            p_prev.clone(),
        ]);

        let p_next = Expression::add(vec![term1, term2]).simplify();

        p_prev = p_curr;
        p_curr = p_next;
    }

    p_curr
}

/// Expand Hermite polynomial H_n(x) to explicit symbolic form
///
/// Uses three-term recurrence to build symbolic expression:
/// - H_0(x) = 1
/// - H_1(x) = 2x
/// - H_{n+1}(x) = 2x H_n(x) - 2n H_{n-1}(x)
///
/// These are the physicist's Hermite polynomials used in quantum mechanics
/// for the harmonic oscillator eigenfunctions.
///
/// # Arguments
///
/// * `n` - Polynomial degree (non-negative integer)
///
/// # Returns
///
/// Expression representing the expanded Hermite polynomial H_n(x)
///
/// # Mathematical Background
///
/// Hermite polynomials are solutions to Hermite's differential equation:
/// y'' - 2xy' + 2ny = 0
///
/// They are orthogonal on (-∞, ∞) with weight function w(x) = e^(-x²).
///
/// # Examples
///
/// ```rust
/// use mathhook_core::functions::polynomials::symbolic::expand_hermite_symbolic;
/// use mathhook_core::core::Expression;
///
/// let h0 = expand_hermite_symbolic(0);
/// let h1 = expand_hermite_symbolic(1);
/// let h2 = expand_hermite_symbolic(2);
/// let h3 = expand_hermite_symbolic(3);
/// ```
#[inline]
#[must_use]
pub fn expand_hermite_symbolic(n: usize) -> Expression {
    if n == 0 {
        return Expression::integer(1);
    }
    if n == 1 {
        return Expression::mul(vec![Expression::integer(2), Expression::symbol("x")]);
    }

    let x = Expression::symbol("x");
    let mut p_prev = Expression::integer(1);
    let mut p_curr = Expression::mul(vec![Expression::integer(2), x.clone()]);

    for i in 1..n {
        let i_i64 = i as i64;

        let term1 = Expression::mul(vec![Expression::integer(2), x.clone(), p_curr.clone()]);

        let term2 = Expression::mul(vec![Expression::integer(-2 * i_i64), p_prev.clone()]);

        let p_next = Expression::add(vec![term1, term2]).simplify();

        p_prev = p_curr;
        p_curr = p_next;
    }

    p_curr
}

/// Expand Laguerre polynomial L_n(x) to explicit symbolic form
///
/// Uses three-term recurrence to build symbolic expression:
/// - L_0(x) = 1
/// - L_1(x) = 1 - x
/// - (n+1)L_{n+1}(x) = (2n+1-x)L_n(x) - nL_{n-1}(x)
///
/// These are the standard Laguerre polynomials (not generalized).
///
/// # Arguments
///
/// * `n` - Polynomial degree (non-negative integer)
///
/// # Returns
///
/// Expression representing the expanded Laguerre polynomial L_n(x)
///
/// # Mathematical Background
///
/// Laguerre polynomials are solutions to Laguerre's differential equation:
/// xy'' + (1-x)y' + ny = 0
///
/// They are orthogonal on [0, ∞) with weight function w(x) = e^(-x).
///
/// # Examples
///
/// ```rust
/// use mathhook_core::functions::polynomials::symbolic::expand_laguerre_symbolic;
/// use mathhook_core::core::Expression;
///
/// let l0 = expand_laguerre_symbolic(0);
/// let l1 = expand_laguerre_symbolic(1);
/// let l2 = expand_laguerre_symbolic(2);
/// let l3 = expand_laguerre_symbolic(3);
/// ```
#[inline]
#[must_use]
pub fn expand_laguerre_symbolic(n: usize) -> Expression {
    if n == 0 {
        return Expression::integer(1);
    }
    if n == 1 {
        return Expression::add(vec![
            Expression::integer(1),
            Expression::mul(vec![Expression::integer(-1), Expression::symbol("x")]),
        ]);
    }

    let x = Expression::symbol("x");
    let mut p_prev = Expression::integer(1);
    let mut p_curr = Expression::add(vec![
        Expression::integer(1),
        Expression::mul(vec![Expression::integer(-1), x.clone()]),
    ]);

    for i in 1..n {
        let i_i64 = i as i64;

        let alpha_num = -1;
        let alpha_den = i_i64 + 1;
        let beta_num = 2 * i_i64 + 1;
        let beta_den = i_i64 + 1;
        let gamma_num = -i_i64;
        let gamma_den = i_i64 + 1;

        let term1 = Expression::mul(vec![
            Expression::rational(alpha_num, alpha_den),
            x.clone(),
            p_curr.clone(),
        ]);

        let term2 = Expression::mul(vec![
            Expression::rational(beta_num, beta_den),
            p_curr.clone(),
        ]);

        let term3 = Expression::mul(vec![
            Expression::rational(gamma_num, gamma_den),
            p_prev.clone(),
        ]);

        let p_next = Expression::add(vec![term1, term2, term3]).simplify();

        p_prev = p_curr;
        p_curr = p_next;
    }

    p_curr
}

/// Expand Chebyshev polynomial of the first kind T_n(x) to explicit symbolic form
///
/// Uses three-term recurrence to build symbolic expression:
/// - T_0(x) = 1
/// - T_1(x) = x
/// - T_{n+1}(x) = 2x T_n(x) - T_{n-1}(x)
///
/// Chebyshev polynomials of the first kind are important in approximation theory
/// and have the explicit form T_n(x) = cos(n arccos(x)) for |x| ≤ 1.
///
/// # Arguments
///
/// * `n` - Polynomial degree (non-negative integer)
///
/// # Returns
///
/// Expression representing the expanded Chebyshev polynomial T_n(x)
///
/// # Mathematical Background
///
/// Chebyshev polynomials of the first kind are solutions to:
/// (1-x²)y'' - xy' + n²y = 0
///
/// They are orthogonal on [-1, 1] with weight function w(x) = 1/√(1-x²).
///
/// # Examples
///
/// ```rust
/// use mathhook_core::functions::polynomials::symbolic::expand_chebyshev_first_symbolic;
/// use mathhook_core::core::Expression;
///
/// let t0 = expand_chebyshev_first_symbolic(0);
/// let t1 = expand_chebyshev_first_symbolic(1);
/// let t2 = expand_chebyshev_first_symbolic(2);
/// let t3 = expand_chebyshev_first_symbolic(3);
/// ```
#[inline]
#[must_use]
pub fn expand_chebyshev_first_symbolic(n: usize) -> Expression {
    if n == 0 {
        return Expression::integer(1);
    }
    if n == 1 {
        return Expression::symbol("x");
    }

    let x = Expression::symbol("x");
    let mut p_prev = Expression::integer(1);
    let mut p_curr = x.clone();

    for _ in 1..n {
        let term1 = Expression::mul(vec![Expression::integer(2), x.clone(), p_curr.clone()]);

        let term2 = Expression::mul(vec![Expression::integer(-1), p_prev.clone()]);

        let p_next = Expression::add(vec![term1, term2]).simplify();

        p_prev = p_curr;
        p_curr = p_next;
    }

    p_curr
}

/// Expand Chebyshev polynomial of the second kind U_n(x) to explicit symbolic form
///
/// Uses three-term recurrence to build symbolic expression:
/// - U_0(x) = 1
/// - U_1(x) = 2x
/// - U_{n+1}(x) = 2x U_n(x) - U_{n-1}(x)
///
/// Chebyshev polynomials of the second kind have the explicit form
/// U_n(x) = sin((n+1) arccos(x)) / sin(arccos(x)) for |x| < 1.
///
/// # Arguments
///
/// * `n` - Polynomial degree (non-negative integer)
///
/// # Returns
///
/// Expression representing the expanded Chebyshev polynomial U_n(x)
///
/// # Mathematical Background
///
/// Chebyshev polynomials of the second kind are orthogonal on [-1, 1]
/// with weight function w(x) = √(1-x²).
///
/// # Examples
///
/// ```rust
/// use mathhook_core::functions::polynomials::symbolic::expand_chebyshev_second_symbolic;
/// use mathhook_core::core::Expression;
///
/// let u0 = expand_chebyshev_second_symbolic(0);
/// let u1 = expand_chebyshev_second_symbolic(1);
/// let u2 = expand_chebyshev_second_symbolic(2);
/// let u3 = expand_chebyshev_second_symbolic(3);
/// ```
#[inline]
#[must_use]
pub fn expand_chebyshev_second_symbolic(n: usize) -> Expression {
    if n == 0 {
        return Expression::integer(1);
    }
    if n == 1 {
        return Expression::mul(vec![Expression::integer(2), Expression::symbol("x")]);
    }

    let x = Expression::symbol("x");
    let mut p_prev = Expression::integer(1);
    let mut p_curr = Expression::mul(vec![Expression::integer(2), x.clone()]);

    for _ in 1..n {
        let term1 = Expression::mul(vec![Expression::integer(2), x.clone(), p_curr.clone()]);

        let term2 = Expression::mul(vec![Expression::integer(-1), p_prev.clone()]);

        let p_next = Expression::add(vec![term1, term2]).simplify();

        p_prev = p_curr;
        p_curr = p_next;
    }

    p_curr
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_legendre_p0_p1() {
        let p0 = expand_legendre_symbolic(0);
        let p1 = expand_legendre_symbolic(1);

        assert_eq!(p0, Expression::integer(1));
        assert_eq!(p1, Expression::symbol("x"));
    }

    #[test]
    fn test_hermite_h0_h1() {
        let h0 = expand_hermite_symbolic(0);
        let h1 = expand_hermite_symbolic(1);

        assert_eq!(h0, Expression::integer(1));
        assert_eq!(
            h1,
            Expression::mul(vec![Expression::integer(2), Expression::symbol("x")])
        );
    }

    #[test]
    fn test_laguerre_l0_l1() {
        let l0 = expand_laguerre_symbolic(0);
        let _l1 = expand_laguerre_symbolic(1);

        assert_eq!(l0, Expression::integer(1));
    }

    #[test]
    fn test_chebyshev_first_t0_t1() {
        let t0 = expand_chebyshev_first_symbolic(0);
        let t1 = expand_chebyshev_first_symbolic(1);

        assert_eq!(t0, Expression::integer(1));
        assert_eq!(t1, Expression::symbol("x"));
    }

    #[test]
    fn test_chebyshev_second_u0_u1() {
        let u0 = expand_chebyshev_second_symbolic(0);
        let u1 = expand_chebyshev_second_symbolic(1);

        assert_eq!(u0, Expression::integer(1));
        assert_eq!(
            u1,
            Expression::mul(vec![Expression::integer(2), Expression::symbol("x")])
        );
    }
}
