//! Arithmetic Helper Functions
//!
//! Integer arithmetic utilities for modular computation and CRT reconstruction.

/// Integer GCD using binary Euclidean algorithm
#[inline]
pub fn integer_gcd(mut a: i64, mut b: i64) -> i64 {
    a = a.abs();
    b = b.abs();
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a.max(1)
}

/// Symmetric modular representation [-m/2, m/2]
#[inline]
pub fn symmetric_mod(a: i64, m: i64) -> i64 {
    let r = ((a % m) + m) % m;
    if r > m / 2 {
        r - m
    } else {
        r
    }
}

/// Positive modular representation [0, m)
#[inline]
pub fn mod_positive(a: i64, m: i64) -> i64 {
    ((a % m) + m) % m
}

/// Extended Euclidean algorithm for i64
pub fn extended_gcd_i64(a: i64, b: i64) -> (i64, i64, i64) {
    if b == 0 {
        (a.abs(), if a >= 0 { 1 } else { -1 }, 0)
    } else {
        let (g, x, y) = extended_gcd_i64(b, a % b);
        (g, y, x - (a / b) * y)
    }
}

/// CRT combination with u128 modulus for larger products
pub fn crt_combine_u128(r1: i64, m1: u128, r2: i64, m2: u128) -> i64 {
    let m1_i64 = (m1 % (i64::MAX as u128)) as i64;
    let m2_i64 = (m2 % (i64::MAX as u128)) as i64;

    let (_, s, _) = extended_gcd_i64(m1_i64, m2_i64);
    let m = m1.saturating_mul(m2);

    let diff = mod_positive(r2 - r1, m2_i64);
    let adjustment = (m1_i64 as i128) * (s as i128) * (diff as i128);
    let result = (r1 as i128) + adjustment;

    let m_i128 = m as i128;
    let result_mod = ((result % m_i128) + m_i128) % m_i128;

    if result_mod > m_i128 / 2 {
        (result_mod - m_i128) as i64
    } else {
        result_mod as i64
    }
}

/// Modular multiplicative inverse using extended Euclidean algorithm
///
/// # Arguments
///
/// * `a` - Integer to invert
/// * `m` - Modulus (should be prime)
///
/// # Returns
///
/// a⁻¹ mod m such that (a * a⁻¹) ≡ 1 (mod m)
/// Returns 1 if gcd(a, m) > 1 (not invertible)
///
/// # Examples
///
/// ```rust,ignore
/// use mathhook_core::core::polynomial::algorithms::zippel_gcd::helpers::arithmetic::mod_inverse;
///
/// assert_eq!(mod_inverse(3, 7), 5);  // 3 * 5 ≡ 1 (mod 7)
/// assert_eq!(mod_inverse(2, 5), 3);  // 2 * 3 ≡ 1 (mod 5)
/// ```
#[inline]
pub fn mod_inverse(a: i64, m: i64) -> i64 {
    let a = mod_positive(a, m);
    let (g, x, _) = extended_gcd_i64(a, m);
    if g > 1 {
        return 1;
    }
    mod_positive(x, m)
}

/// Evaluate univariate polynomial at a point using Horner's method
///
/// Evaluates f(r) = `f[0]` + `f[1]`*r + ... + `f[n]`*r^n
///
/// # Arguments
///
/// * `coeffs` - Polynomial coefficients (ascending order: c₀, c₁, ..., cₙ)
/// * `r` - Evaluation point
///
/// # Returns
///
/// f(r) computed using Horner's method for numerical stability
///
/// # Examples
///
/// ```rust,ignore
/// use mathhook_core::core::polynomial::algorithms::zippel_gcd::helpers::arithmetic::evaluate_polynomial_at;
///
/// // f = 2x² + 3x + 1
/// let coeffs = vec![1, 3, 2];
/// assert_eq!(evaluate_polynomial_at(&coeffs, 2), 15);  // 2*4 + 3*2 + 1 = 15
/// ```
#[inline]
pub fn evaluate_polynomial_at(coeffs: &[i64], r: i64) -> i64 {
    if coeffs.is_empty() {
        return 0;
    }

    let mut result = coeffs[coeffs.len() - 1];
    for &coeff in coeffs.iter().rev().skip(1) {
        result = result.saturating_mul(r).saturating_add(coeff);
    }
    result
}

/// Extract GCD of leading coefficients
///
/// For multivariate polynomials f, g represented as `HashMap<Vec<usize>, i64>`,
/// extract gcd(lc(f), lc(g)) where lc is the leading coefficient in the main variable.
///
/// # Arguments
///
/// * `f` - First polynomial
/// * `g` - Second polynomial
/// * `main_var_idx` - Index of main variable
///
/// # Returns
///
/// GCD of leading coefficients (integer)
///
/// # Examples
///
/// ```rust,ignore
/// use std::collections::HashMap;
/// use mathhook_core::core::polynomial::algorithms::zippel_gcd::helpers::arithmetic::extract_lc_gcd;
///
/// // f = 6x² + 12x (lc = 6)
/// let mut f = HashMap::new();
/// f.insert(vec![2], 6);
/// f.insert(vec![1], 12);
///
/// // g = 9x² + 18x (lc = 9)
/// let mut g = HashMap::new();
/// g.insert(vec![2], 9);
/// g.insert(vec![1], 18);
///
/// assert_eq!(extract_lc_gcd(&f, &g, 0), 3);  // gcd(6, 9) = 3
/// ```
pub fn extract_lc_gcd(
    f: &std::collections::HashMap<Vec<usize>, i64>,
    g: &std::collections::HashMap<Vec<usize>, i64>,
    main_var_idx: usize,
) -> i64 {
    let f_lc = f
        .iter()
        .filter(|(deg_vec, _)| deg_vec.get(main_var_idx).copied().unwrap_or(0) > 0)
        .map(|(deg_vec, &coeff)| (deg_vec.get(main_var_idx).copied().unwrap_or(0), coeff))
        .max_by_key(|&(deg, _)| deg)
        .map(|(_, coeff)| coeff.abs())
        .unwrap_or(1);

    let g_lc = g
        .iter()
        .filter(|(deg_vec, _)| deg_vec.get(main_var_idx).copied().unwrap_or(0) > 0)
        .map(|(deg_vec, &coeff)| (deg_vec.get(main_var_idx).copied().unwrap_or(0), coeff))
        .max_by_key(|&(deg, _)| deg)
        .map(|(_, coeff)| coeff.abs())
        .unwrap_or(1);

    integer_gcd(f_lc, g_lc)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_symmetric_mod() {
        assert_eq!(symmetric_mod(8, 7), 1);
        assert_eq!(symmetric_mod(6, 7), -1);
        assert_eq!(symmetric_mod(3, 7), 3);
        assert_eq!(symmetric_mod(4, 7), -3);
        assert_eq!(symmetric_mod(0, 7), 0);
    }

    #[test]
    fn test_extended_gcd_i64() {
        let (g, x, y) = extended_gcd_i64(35, 15);
        assert_eq!(g, 5);
        assert_eq!(35 * x + 15 * y, 5);

        let (g, x, y) = extended_gcd_i64(17, 13);
        assert_eq!(g, 1);
        assert_eq!(17 * x + 13 * y, 1);
    }

    #[test]
    fn test_integer_gcd() {
        assert_eq!(integer_gcd(12, 18), 6);
        assert_eq!(integer_gcd(0, 5), 5);
        assert_eq!(integer_gcd(5, 0), 5);
        assert_eq!(integer_gcd(-12, 18), 6);
        assert_eq!(integer_gcd(17, 13), 1);
    }

    #[test]
    fn test_mod_inverse() {
        assert_eq!(mod_inverse(3, 7), 5); // 3 * 5 ≡ 1 (mod 7)
        assert_eq!(mod_inverse(2, 5), 3); // 2 * 3 ≡ 1 (mod 5)
        assert_eq!(mod_inverse(7, 11), 8); // 7 * 8 ≡ 1 (mod 11)
    }

    #[test]
    fn test_evaluate_polynomial_at() {
        // f = 2x² + 3x + 1
        let coeffs = vec![1, 3, 2];
        assert_eq!(evaluate_polynomial_at(&coeffs, 0), 1); // f(0) = 1
        assert_eq!(evaluate_polynomial_at(&coeffs, 1), 6); // f(1) = 6
        assert_eq!(evaluate_polynomial_at(&coeffs, 2), 15); // f(2) = 15

        // f = 1 + x³
        let coeffs = vec![1, 0, 0, 1];
        assert_eq!(evaluate_polynomial_at(&coeffs, 2), 9); // f(2) = 9
    }

    #[test]
    fn test_extract_lc_gcd() {
        // f = 6x² + 12x
        let mut f = HashMap::new();
        f.insert(vec![2], 6);
        f.insert(vec![1], 12);

        // g = 9x² + 18x
        let mut g = HashMap::new();
        g.insert(vec![2], 9);
        g.insert(vec![1], 18);

        assert_eq!(extract_lc_gcd(&f, &g, 0), 3); // gcd(6, 9) = 3
    }
}
