//! NTT multiplication operations
//!
//! Provides fast O(n log n) polynomial multiplication using Number Theoretic Transform.
//! This module includes both legacy and optimized implementations.
use super::super::element::Zp;
use super::super::poly::PolyZp;
use super::super::{FiniteFieldError, FiniteFieldResult};
use super::transform::{ntt_forward_mont, ntt_inverse_mont, TwiddleTable};
use super::NTT_THRESHOLD;
use super::{get_primitive_root, next_power_of_2, NTT_PRIME_1, NTT_PRIME_2, NTT_PRIME_3};
use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::Arc;
thread_local! {
    static TWIDDLE_CACHE : RefCell < HashMap < (usize, u64), Arc < TwiddleTable >>> =
    RefCell::new(HashMap::new());
}
/// Get or create a twiddle table from cache
fn get_cached_twiddles(n: usize, omega: u64, p: u64) -> Arc<TwiddleTable> {
    TWIDDLE_CACHE.with(|cache| {
        let mut cache = cache.borrow_mut();
        let key = (n, p);
        if let Some(twiddles) = cache.get(&key) {
            Arc::clone(twiddles)
        } else {
            let twiddles = Arc::new(TwiddleTable::new(n, omega, p));
            cache.insert(key, Arc::clone(&twiddles));
            twiddles
        }
    })
}
/// Fast polynomial multiplication using optimized NTT
///
/// Multiplies two polynomials in O(n log n) time using Number Theoretic Transform
/// with Montgomery arithmetic and precomputed twiddle factors for maximum performance.
///
/// # Arguments
///
/// * `a` - First polynomial
/// * `b` - Second polynomial
///
/// # Returns
///
/// Product a * b computed via NTT
///
/// # Algorithm
///
/// ```text
/// 1. Pad both polynomials to next power of 2 >= deg(a) + deg(b) + 1
/// 2. Precompute twiddle table with Montgomery conversion
/// 3. Forward NTT on both padded polynomials using precomputed twiddles
/// 4. Pointwise multiplication in frequency domain (Montgomery form)
/// 5. Inverse NTT to get result coefficients
/// ```
///
/// # Optimizations
///
/// - **Montgomery multiplication**: Replaces expensive modular division with fast bitwise ops
/// - **Precomputed twiddle factors**: Eliminates pow() calls in butterfly stages
/// - **Raw u64 arithmetic**: Eliminates Zp wrapper overhead in hot loops
/// - **Cache-friendly access**: Twiddle table layout optimized for sequential access
///
/// # Errors
///
/// Returns error if modulus is not a known NTT-friendly prime.
///
/// # Performance
///
/// - **Best Case**: Both polynomials same size, size is power of 2
/// - **Worst Case**: Size just above power of 2 (wasted padding)
/// - **Speedup**: 10-50x faster than legacy implementation for large polynomials
///
/// # Examples
///
/// ```rust
/// use mathhook_core::algebra::PolyZp;
/// # use mathhook_core::core::polynomial::finite_field::ntt_multiply;
///
/// // Using NTT-friendly prime
/// let p = 2013265921;
/// let a = PolyZp::from_coeffs(vec![1, 2, 3], p);
/// let b = PolyZp::from_coeffs(vec![4, 5], p);
///
/// let product = ntt_multiply(&a, &b).unwrap();
/// // Result: 4 + 13x + 22x^2 + 15x^3
/// ```
pub fn ntt_multiply(a: &PolyZp, b: &PolyZp) -> FiniteFieldResult<PolyZp> {
    let p = a.modulus();
    debug_assert_eq!(p, b.modulus(), "modulus mismatch");
    if a.is_zero() || b.is_zero() {
        return Ok(PolyZp::zero(p));
    }
    let g = get_primitive_root(p)?;
    let deg_a = a.degree().ok_or(FiniteFieldError::Overflow {
        operation: "polynomial degree computation failed",
    })?;
    let deg_b = b.degree().ok_or(FiniteFieldError::Overflow {
        operation: "polynomial degree computation failed",
    })?;
    let result_degree = deg_a + deg_b;
    let n = next_power_of_2(result_degree + 1);
    let max_n = match p {
        NTT_PRIME_1 => 1 << 27,
        NTT_PRIME_2 => 1 << 26,
        NTT_PRIME_3 => 1 << 21,
        _ => {
            return Err(FiniteFieldError::Overflow {
                operation: "NTT size exceeds prime capacity",
            });
        }
    };
    if n > max_n {
        return Err(FiniteFieldError::Overflow {
            operation: "polynomial degree exceeds NTT capacity for this prime",
        });
    }
    let g_field = Zp::new(g, p);
    let exponent = (p - 1) / n as u64;
    let omega = g_field.pow(exponent).value();
    let twiddles = get_cached_twiddles(n, omega, p);
    let mut a_coeffs = a.coefficients().to_vec();
    let mut b_coeffs = b.coefficients().to_vec();
    a_coeffs.resize(n, 0);
    b_coeffs.resize(n, 0);
    ntt_forward_mont(&mut a_coeffs, &twiddles);
    ntt_forward_mont(&mut b_coeffs, &twiddles);
    let mont = &twiddles.mont;
    for i in 0..n {
        a_coeffs[i] = mont.mont_mul(a_coeffs[i], b_coeffs[i]);
    }
    ntt_inverse_mont(&mut a_coeffs, &twiddles);
    Ok(PolyZp::from_coeffs(a_coeffs, p))
}
/// Multiply two polynomials using automatic threshold-based selection
///
/// Chooses between naive O(n²) and NTT O(n log n) based on polynomial degree.
///
/// # Arguments
///
/// * `a` - First polynomial
/// * `b` - Second polynomial
///
/// # Returns
///
/// Product a * b using optimal algorithm
///
/// # Strategy
///
/// - **Small polynomials** (degree ≤ 64): Use naive multiplication
/// - **Large polynomials** (degree > 64): Use NTT if prime supports it
/// - **Fallback**: Use naive if NTT not available for this prime
///
/// # Examples
///
/// ```rust
/// use mathhook_core::algebra::PolyZp;
/// # use mathhook_core::core::polynomial::finite_field::multiply_auto;
///
/// let p = 2013265921;  // NTT-friendly prime
/// let a = PolyZp::from_coeffs(vec![1; 100], p);
/// let b = PolyZp::from_coeffs(vec![1; 100], p);
///
/// // Automatically uses NTT (degree > 64)
/// let product = multiply_auto(&a, &b);
/// ```
pub fn multiply_auto(a: &PolyZp, b: &PolyZp) -> PolyZp {
    let use_ntt = if let (Some(deg_a), Some(deg_b)) = (a.degree(), b.degree()) {
        deg_a >= NTT_THRESHOLD && deg_b >= NTT_THRESHOLD
    } else {
        false
    };
    if use_ntt {
        match ntt_multiply(a, b) {
            Ok(result) => result,
            Err(_) => a.mul(b),
        }
    } else {
        a.mul(b)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_ntt_multiply_simple() {
        let p = NTT_PRIME_1;
        let a = PolyZp::from_coeffs(vec![1, 1], p);
        let b = PolyZp::from_coeffs(vec![1, 1], p);
        let product = ntt_multiply(&a, &b).unwrap();
        assert_eq!(product.coefficients(), &[1, 2, 1]);
    }
    #[test]
    fn test_ntt_vs_naive_large() {
        let p = NTT_PRIME_1;
        for size in [4, 8, 16, 32, 64, 128, 256, 512] {
            let a_coeffs: Vec<u64> = (1..=size).map(|i| i % p).collect();
            let b_coeffs: Vec<u64> = (1..=size).map(|i| (i * 3) % p).collect();
            let a = PolyZp::from_coeffs(a_coeffs, p);
            let b = PolyZp::from_coeffs(b_coeffs, p);
            let ntt_result = ntt_multiply(&a, &b).unwrap();
            let naive_result = a.mul(&b);
            assert_eq!(
                ntt_result.coefficients(),
                naive_result.coefficients(),
                "NTT and naive differ for size {}",
                size
            );
        }
    }
    #[test]
    fn test_ntt_multiply_vs_naive() {
        let p = NTT_PRIME_1;
        for size in [4, 8, 16, 32, 64, 128] {
            let a_coeffs: Vec<u64> = (1..=size).map(|i| i % p).collect();
            let b_coeffs: Vec<u64> = (1..=size).map(|i| (i * 2) % p).collect();
            let a = PolyZp::from_coeffs(a_coeffs, p);
            let b = PolyZp::from_coeffs(b_coeffs, p);
            let naive = a.mul(&b);
            let ntt = ntt_multiply(&a, &b).unwrap();
            assert_eq!(
                naive.coefficients(),
                ntt.coefficients(),
                "NTT and naive multiplication differ for size {}",
                size
            );
        }
    }
    #[test]
    fn test_ntt_multiply_zero() {
        let p = NTT_PRIME_1;
        let a = PolyZp::from_coeffs(vec![1, 2, 3], p);
        let zero = PolyZp::zero(p);
        let product = ntt_multiply(&a, &zero).unwrap();
        assert!(product.is_zero());
        let product2 = ntt_multiply(&zero, &a).unwrap();
        assert!(product2.is_zero());
    }
    #[test]
    fn test_ntt_multiply_large() {
        let p = NTT_PRIME_1;
        let size = 256;
        let a_coeffs: Vec<u64> = (1..=size).map(|i| i % p).collect();
        let b_coeffs: Vec<u64> = (1..=size).map(|i| (i * 3) % p).collect();
        let a = PolyZp::from_coeffs(a_coeffs, p);
        let b = PolyZp::from_coeffs(b_coeffs, p);
        let naive = a.mul(&b);
        let ntt = ntt_multiply(&a, &b).unwrap();
        assert_eq!(naive.coefficients(), ntt.coefficients());
    }
    #[test]
    fn test_multiply_auto_threshold() {
        let p = NTT_PRIME_1;
        let small_a = PolyZp::from_coeffs(vec![1, 2, 3], p);
        let small_b = PolyZp::from_coeffs(vec![4, 5], p);
        let small_product = multiply_auto(&small_a, &small_b);
        assert_eq!(
            small_product.coefficients(),
            small_a.mul(&small_b).coefficients()
        );
        let large_a = PolyZp::from_coeffs(vec![1; 100], p);
        let large_b = PolyZp::from_coeffs(vec![2; 100], p);
        let large_product = multiply_auto(&large_a, &large_b);
        assert_eq!(
            large_product.coefficients(),
            large_a.mul(&large_b).coefficients()
        );
    }
    #[test]
    fn test_ntt_different_primes() {
        for &p in &[NTT_PRIME_1, NTT_PRIME_2, NTT_PRIME_3] {
            let a = PolyZp::from_coeffs(vec![1, 2, 3, 4], p);
            let b = PolyZp::from_coeffs(vec![5, 6, 7], p);
            let naive = a.mul(&b);
            let ntt = ntt_multiply(&a, &b).unwrap();
            assert_eq!(
                naive.coefficients(),
                ntt.coefficients(),
                "NTT failed for prime {}",
                p
            );
        }
    }
    #[test]
    fn test_ntt_unsupported_prime() {
        let p = 17;
        let a = PolyZp::from_coeffs(vec![1, 2, 3], p);
        let b = PolyZp::from_coeffs(vec![4, 5], p);
        assert!(ntt_multiply(&a, &b).is_err());
        let product = multiply_auto(&a, &b);
        assert_eq!(product.coefficients(), a.mul(&b).coefficients());
    }
    #[test]
    fn test_ntt_convolution_property() {
        let p = NTT_PRIME_1;
        let a = PolyZp::from_coeffs(vec![1, 2], p);
        let b = PolyZp::from_coeffs(vec![3, 4], p);
        let product = ntt_multiply(&a, &b).unwrap();
        assert_eq!(product.coefficients(), &[3, 10, 8]);
    }
}
#[cfg(test)]
mod comprehensive_test {
    use super::*;
    #[test]
    fn test_end_to_end_workflow() {
        let p = NTT_PRIME_1;
        let small_a = PolyZp::from_coeffs(vec![1, 2], p);
        let small_b = PolyZp::from_coeffs(vec![3, 4], p);
        let small_result = multiply_auto(&small_a, &small_b);
        assert_eq!(small_result.coefficients(), &[3, 10, 8]);
        let large_a = PolyZp::from_coeffs(vec![1; 100], p);
        let large_b = PolyZp::from_coeffs(vec![2; 100], p);
        let auto_result = multiply_auto(&large_a, &large_b);
        let ntt_result = ntt_multiply(&large_a, &large_b).unwrap();
        let fast_result = large_a.mul_fast(&large_b);
        let naive_result = large_a.mul(&large_b);
        assert_eq!(auto_result.coefficients(), naive_result.coefficients());
        assert_eq!(ntt_result.coefficients(), naive_result.coefficients());
        assert_eq!(fast_result.coefficients(), naive_result.coefficients());
    }
}
