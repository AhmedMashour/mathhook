//! Univariate Modular GCD
//!
//! Modular GCD computation for univariate polynomials using CRT reconstruction.
//! PURE NUMERIC: works on `&[i64]`, returns `Vec<i64>`. NO Expression usage.

use super::content::primitive_part;
use super::helpers::{
    crt_combine_u128, integer_gcd, mod_positive, symmetric_mod, LARGE_PRIMES, MAX_CRT_ITERATIONS,
};
use super::sparse::is_sparse;
use super::trial_division::verify_gcd_candidate;
use crate::core::polynomial::finite_field::PolyZp;
use crate::core::polynomial::PolynomialError;

/// Modular GCD for univariate polynomials in ``Z[x]``
///
/// PURE NUMERIC: takes `&[i64]` coefficient vectors, returns (gcd, cofactor_f, cofactor_g).
/// NO Expression usage - all computation on `Vec<i64>`.
///
/// Uses the modular approach with CRT reconstruction and full verification:
/// 1. Extract content and compute primitive parts
/// 2. Reduce polynomials mod p
/// 3. Compute GCD in `Z_p[x]`
/// 4. Reconstruct integer coefficients via CRT
/// 5. Verify by trial division
///
/// # Arguments
///
/// * `f_coeffs` - First polynomial coefficients (ascending order: `f[i]` is coeff of x^i)
/// * `g_coeffs` - Second polynomial coefficients (ascending order)
///
/// # Returns
///
/// Result containing (gcd, cofactor_f, cofactor_g) where:
/// - gcd: Greatest common divisor coefficients
/// - cofactor_f: f / gcd coefficients
/// - cofactor_g: g / gcd coefficients
///
/// # Examples
///
/// ```rust
/// use mathhook_core::core::polynomial::algorithms::zippel_gcd::modular_gcd_univariate;
///
/// // (x^2 - 1) = (x-1)(x+1) and (x-1)
/// let f = vec![-1, 0, 1];  // -1 + 0*x + 1*x^2
/// let g = vec![-1, 1];     // -1 + 1*x
/// let (gcd, cof_f, cof_g) = modular_gcd_univariate(&f, &g).unwrap();
/// // gcd should be [c, d] where c*x + d divides both
/// ```
#[allow(clippy::type_complexity)]
pub fn modular_gcd_univariate(
    f_coeffs: &[i64],
    g_coeffs: &[i64],
) -> Result<(Vec<i64>, Vec<i64>, Vec<i64>), PolynomialError> {
    // Zero handling
    if f_coeffs.is_empty() && g_coeffs.is_empty() {
        return Ok((vec![], vec![], vec![]));
    }
    if f_coeffs.is_empty() {
        return Ok((g_coeffs.to_vec(), vec![], vec![1]));
    }
    if g_coeffs.is_empty() {
        return Ok((f_coeffs.to_vec(), vec![1], vec![]));
    }

    // Extract content and primitive parts
    let (f_content, f_prim) = primitive_part(f_coeffs);
    let (g_content, g_prim) = primitive_part(g_coeffs);
    let content_gcd = integer_gcd(f_content, g_content);

    if f_prim.is_empty() || g_prim.is_empty() {
        return Ok((vec![content_gcd], f_coeffs.to_vec(), g_coeffs.to_vec()));
    }

    let f_lc = *f_prim.last().unwrap_or(&1);
    let g_lc = *g_prim.last().unwrap_or(&1);
    let gamma = integer_gcd(f_lc.abs(), g_lc.abs());

    let mut bound = std::cmp::min(f_prim.len(), g_prim.len()) - 1;
    if bound == 0 {
        return Ok((vec![content_gcd], f_coeffs.to_vec(), g_coeffs.to_vec()));
    }

    let f_sparse = is_sparse(&f_prim);
    let g_sparse = is_sparse(&g_prim);
    let _use_sparse = f_sparse || g_sparse;

    let mut modulus: u128 = 1;
    let mut h_coeffs: Vec<i64> = Vec::new();
    let mut prime_idx = 0;
    let mut iterations = 0;

    while prime_idx < LARGE_PRIMES.len() && iterations < MAX_CRT_ITERATIONS {
        iterations += 1;
        let p = LARGE_PRIMES[prime_idx];
        prime_idx += 1;

        if gamma != 0 && gamma.unsigned_abs() % p == 0 {
            continue;
        }

        let f_mod_p: Vec<u64> = f_prim
            .iter()
            .map(|&c| mod_positive(c, p as i64) as u64)
            .collect();
        let f_poly = PolyZp::from_coeffs(f_mod_p, p);

        let g_mod_p: Vec<u64> = g_prim
            .iter()
            .map(|&c| mod_positive(c, p as i64) as u64)
            .collect();
        let g_poly = PolyZp::from_coeffs(g_mod_p, p);

        let hp = match f_poly.gcd(&g_poly) {
            Ok(h) => h,
            Err(_) => continue,
        };

        let deg_hp = hp.degree().unwrap_or(0);
        if deg_hp > bound {
            continue;
        }

        if deg_hp < bound {
            bound = deg_hp;
            modulus = 1;
            h_coeffs.clear();
        }

        let hp_coeffs: Vec<i64> = hp
            .coefficients()
            .iter()
            .map(|&c| symmetric_mod(c as i64, p as i64))
            .collect();

        if modulus == 1 {
            modulus = p as u128;
            h_coeffs = hp_coeffs;
            continue;
        }

        let new_coeffs: Vec<i64> = h_coeffs
            .iter()
            .zip(hp_coeffs.iter())
            .map(|(&c1, &c2)| crt_combine_u128(c1, modulus, c2, p as u128))
            .collect();

        let new_modulus = modulus.saturating_mul(p as u128);
        let converged = new_coeffs == h_coeffs;
        h_coeffs = new_coeffs;
        modulus = new_modulus;

        if converged && !h_coeffs.is_empty() && verify_gcd_candidate(&f_prim, &g_prim, &h_coeffs) {
            let scaled_coeffs: Vec<i64> = h_coeffs.iter().map(|&c| c * content_gcd).collect();

            let f_quot = compute_cofactor(f_coeffs, &h_coeffs);
            let g_quot = compute_cofactor(g_coeffs, &h_coeffs);

            let f_cofactor = f_quot.unwrap_or_else(|| f_coeffs.to_vec());
            let g_cofactor = g_quot.unwrap_or_else(|| g_coeffs.to_vec());

            return Ok((scaled_coeffs, f_cofactor, g_cofactor));
        }
    }

    if iterations >= MAX_CRT_ITERATIONS {
        return Err(PolynomialError::MaxIterationsExceeded {
            operation: "modular GCD CRT reconstruction",
            limit: MAX_CRT_ITERATIONS,
        });
    }

    Ok((vec![1], f_coeffs.to_vec(), g_coeffs.to_vec()))
}

/// Compute cofactor f / gcd using polynomial division
///
/// Returns Some(quotient) if division is exact, None otherwise.
fn compute_cofactor(f: &[i64], gcd: &[i64]) -> Option<Vec<i64>> {
    if gcd.is_empty() {
        return None;
    }
    if gcd.len() == 1 && gcd[0] == 1 {
        return Some(f.to_vec());
    }

    // Polynomial long division
    let mut remainder = f.to_vec();
    let mut quotient = vec![0; f.len().saturating_sub(gcd.len()) + 1];

    let gcd_lc = *gcd.last().unwrap();
    if gcd_lc == 0 {
        return None;
    }

    while !remainder.is_empty() && remainder.len() >= gcd.len() {
        let r_lc = *remainder.last().unwrap();
        if r_lc % gcd_lc != 0 {
            return None; // Not exact division
        }

        let coeff = r_lc / gcd_lc;
        let q_idx = remainder.len() - gcd.len();
        quotient[q_idx] = coeff;

        // Subtract coeff * gcd from remainder
        for (i, &g) in gcd.iter().enumerate() {
            remainder[q_idx + i] -= coeff * g;
        }

        // Remove leading zeros
        while remainder.last() == Some(&0) {
            remainder.pop();
        }
    }

    // Check if remainder is zero
    if remainder.iter().all(|&c| c == 0) {
        Some(quotient)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_modular_gcd_univariate_coprime() {
        // f = x, g = x + 1
        let f = vec![0, 1];
        let g = vec![1, 1];
        let result = modular_gcd_univariate(&f, &g);
        assert!(result.is_ok());
        let (gcd, _, _) = result.unwrap();
        assert_eq!(gcd, vec![1]);
    }

    #[test]
    fn test_modular_gcd_univariate_zero() {
        // f = 0, g = x
        let f = vec![];
        let g = vec![0, 1];
        let result = modular_gcd_univariate(&f, &g);
        assert!(result.is_ok());
        let (gcd, _, _) = result.unwrap();
        assert_eq!(gcd, g);
    }

    #[test]
    fn test_modular_gcd_univariate_same() {
        // f = x^2, g = x^2
        let f = vec![0, 0, 1];
        let g = f.clone();
        let result = modular_gcd_univariate(&f, &g);
        assert!(result.is_ok());
    }

    #[test]
    fn test_modular_gcd_with_content() {
        // f = 6x + 12, g = 9x + 18
        let f = vec![12, 6];
        let g = vec![18, 9];
        let result = modular_gcd_univariate(&f, &g);
        assert!(result.is_ok());
        let (gcd, _, _) = result.unwrap();
        // GCD should include content factor 3
        assert!(gcd[0] >= 3);
    }

    #[test]
    fn test_compute_cofactor() {
        // f = x^2 - 1, gcd = x - 1
        // Expected quotient: x + 1
        let f = vec![-1, 0, 1];
        let gcd = vec![-1, 1];
        let result = compute_cofactor(&f, &gcd);
        assert!(result.is_some());
        let quot = result.unwrap();
        assert_eq!(quot, vec![1, 1]); // 1 + x
    }
}
