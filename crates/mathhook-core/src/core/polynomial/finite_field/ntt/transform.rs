//! Optimized NTT transform operations
//!
//! This module provides high-performance Number Theoretic Transform using:
//! - Precomputed twiddle factors (eliminates repeated pow() calls)
//! - Raw u64 arithmetic in hot loops (eliminates Zp wrapper overhead)
//! - Montgomery multiplication for fast modular reduction
//! - Cache-friendly memory access patterns

use super::super::element::Zp;

/// Montgomery reduction constants for a prime p
/// Used to replace expensive modular division with fast bitwise operations
#[derive(Clone)]
#[allow(dead_code)]
pub struct MontgomeryContext {
    /// The prime modulus
    pub p: u64,
    /// R = 2^64 mod p (Montgomery radix)
    pub r: u64,
    /// R^2 mod p (for converting to Montgomery form)
    pub r2: u64,
    /// -p^(-1) mod 2^64 (for Montgomery reduction)
    pub p_inv_neg: u64,
}

impl MontgomeryContext {
    /// Create Montgomery context for a prime
    pub fn new(p: u64) -> Self {
        // R = 2^64 mod p
        let r = ((1u128 << 64) % p as u128) as u64;

        // R^2 mod p
        let r2 = ((r as u128 * r as u128) % p as u128) as u64;

        // Compute -p^(-1) mod 2^64 using Newton's method
        // p * p_inv ≡ 1 (mod 2^64)
        // We want -p_inv mod 2^64
        let mut p_inv: u64 = 1;
        for _ in 0..6 {
            p_inv = p_inv.wrapping_mul(2u64.wrapping_sub(p.wrapping_mul(p_inv)));
        }
        let p_inv_neg = p_inv.wrapping_neg();

        Self {
            p,
            r,
            r2,
            p_inv_neg,
        }
    }

    /// Convert to Montgomery form: a -> aR mod p
    #[inline(always)]
    pub fn to_montgomery(&self, a: u64) -> u64 {
        self.mont_mul(a, self.r2)
    }

    /// Convert from Montgomery form: aR -> a mod p
    #[inline(always)]
    pub fn reduce_from_montgomery(&self, a: u64) -> u64 {
        self.mont_reduce(a as u128)
    }

    /// Montgomery multiplication: (aR * bR) / R mod p = abR mod p
    #[inline(always)]
    pub fn mont_mul(&self, a: u64, b: u64) -> u64 {
        self.mont_reduce(a as u128 * b as u128)
    }

    /// Montgomery reduction: t / R mod p
    #[inline(always)]
    fn mont_reduce(&self, t: u128) -> u64 {
        // m = (t mod R) * (-p^(-1)) mod R
        let m = (t as u64).wrapping_mul(self.p_inv_neg);
        // t = (t + m * p) / R
        let t = ((t + m as u128 * self.p as u128) >> 64) as u64;
        // Conditional subtraction
        if t >= self.p {
            t - self.p
        } else {
            t
        }
    }

    /// Modular addition in Montgomery form
    #[inline(always)]
    pub fn mont_add(&self, a: u64, b: u64) -> u64 {
        let sum = a + b;
        if sum >= self.p {
            sum - self.p
        } else {
            sum
        }
    }

    /// Modular subtraction in Montgomery form
    #[inline(always)]
    pub fn mont_sub(&self, a: u64, b: u64) -> u64 {
        if a >= b {
            a - b
        } else {
            a + self.p - b
        }
    }
}

/// Precomputed twiddle factors for NTT
/// Stores all powers of omega needed for a specific size NTT
pub struct TwiddleTable {
    /// Forward twiddle factors: omega^0, omega^1, ..., omega^(n/2-1)
    /// Stored in Montgomery form for fast multiplication
    pub forward: Vec<u64>,
    /// Inverse twiddle factors: omega_inv^0, omega_inv^1, ..., omega_inv^(n/2-1)
    pub inverse: Vec<u64>,
    /// n^(-1) mod p in Montgomery form
    pub n_inv_mont: u64,
    /// Montgomery context
    pub mont: MontgomeryContext,
    /// Size of NTT
    pub n: usize,
}

impl TwiddleTable {
    /// Precompute twiddle factors for NTT of size n
    pub fn new(n: usize, omega: u64, p: u64) -> Self {
        debug_assert!(n.is_power_of_two(), "n must be power of 2");

        let mont = MontgomeryContext::new(p);

        // Compute omega^(-1) using Fermat's little theorem: omega^(p-2) = omega^(-1) mod p
        let omega_inv = Zp::new(omega, p).inverse().unwrap().value();

        // Compute n^(-1) mod p
        let n_inv = Zp::new(n as u64, p).inverse().unwrap().value();
        let n_inv_mont = mont.to_montgomery(n_inv);

        // Precompute all twiddle factors in Montgomery form
        let half_n = n / 2;
        let mut forward = Vec::with_capacity(half_n);
        let mut inverse = Vec::with_capacity(half_n);

        let omega_mont = mont.to_montgomery(omega);
        let omega_inv_mont = mont.to_montgomery(omega_inv);

        let mut w = mont.to_montgomery(1); // 1 in Montgomery form
        let mut w_inv = mont.to_montgomery(1);

        for _ in 0..half_n {
            forward.push(w);
            inverse.push(w_inv);
            w = mont.mont_mul(w, omega_mont);
            w_inv = mont.mont_mul(w_inv, omega_inv_mont);
        }

        Self {
            forward,
            inverse,
            n_inv_mont,
            mont,
            n,
        }
    }
}

/// Bit-reverse permutation for NTT (optimized version)
#[inline]
pub(super) fn bit_reverse_permute(a: &mut [u64], n: usize) {
    debug_assert!(n.is_power_of_two(), "n must be power of 2");
    debug_assert_eq!(a.len(), n, "array length must match n");

    let log_n = n.trailing_zeros() as usize;

    for i in 0..n {
        let j = i.reverse_bits() >> (usize::BITS as usize - log_n);
        if i < j {
            a.swap(i, j);
        }
    }
}

/// Forward NTT that outputs in Montgomery form (no final conversion)
/// Use with `ntt_inverse_mont` for optimal NTT multiplication pipeline.
#[inline]
pub fn ntt_forward_mont(a: &mut [u64], twiddles: &TwiddleTable) {
    let n = a.len();
    debug_assert!(n.is_power_of_two(), "length must be power of 2");
    debug_assert_eq!(n, twiddles.n, "twiddle table size mismatch");

    if n <= 1 {
        // Single element still needs Montgomery conversion for consistency
        if n == 1 {
            a[0] = twiddles.mont.to_montgomery(a[0]);
        }
        return;
    }

    let mont = &twiddles.mont;

    // Convert to Montgomery form
    for x in a.iter_mut() {
        *x = mont.to_montgomery(*x);
    }

    ntt_forward_core(a, twiddles);
    // Output stays in Montgomery form
}

/// Core NTT butterfly operations (assumes input already in Montgomery form)
#[inline]
fn ntt_forward_core(a: &mut [u64], twiddles: &TwiddleTable) {
    let n = a.len();
    let mont = &twiddles.mont;

    bit_reverse_permute(a, n);

    // Cooley-Tukey butterfly with precomputed twiddles
    let mut m = 2;
    while m <= n {
        let half_m = m / 2;
        let step = n / m;

        for k in (0..n).step_by(m) {
            for j in 0..half_m {
                let twiddle = twiddles.forward[j * step];

                let u = a[k + j];
                let v = mont.mont_mul(a[k + j + half_m], twiddle);

                a[k + j] = mont.mont_add(u, v);
                a[k + j + half_m] = mont.mont_sub(u, v);
            }
        }

        m *= 2;
    }
}

/// Inverse NTT that takes Montgomery form input and outputs standard form
/// Use with `ntt_forward_mont` for optimal NTT multiplication pipeline.
#[inline]
pub fn ntt_inverse_mont(a: &mut [u64], twiddles: &TwiddleTable) {
    let n = a.len();
    debug_assert!(n.is_power_of_two(), "length must be power of 2");
    debug_assert_eq!(n, twiddles.n, "twiddle table size mismatch");

    if n <= 1 {
        // Single element needs conversion from Montgomery + scaling
        if n == 1 {
            let mont = &twiddles.mont;
            a[0] = mont.reduce_from_montgomery(mont.mont_mul(a[0], twiddles.n_inv_mont));
        }
        return;
    }

    ntt_inverse_core(a, twiddles);

    // Convert back from Montgomery form
    let mont = &twiddles.mont;
    for x in a.iter_mut() {
        *x = mont.reduce_from_montgomery(*x);
    }
}

/// Core inverse NTT butterfly operations (assumes input in Montgomery form, outputs Montgomery form with n^-1 scaling)
#[inline]
fn ntt_inverse_core(a: &mut [u64], twiddles: &TwiddleTable) {
    let n = a.len();
    let mont = &twiddles.mont;

    bit_reverse_permute(a, n);

    // Cooley-Tukey butterfly with inverse twiddles
    let mut m = 2;
    while m <= n {
        let half_m = m / 2;
        let step = n / m;

        for k in (0..n).step_by(m) {
            for j in 0..half_m {
                let twiddle = twiddles.inverse[j * step];

                let u = a[k + j];
                let v = mont.mont_mul(a[k + j + half_m], twiddle);

                a[k + j] = mont.mont_add(u, v);
                a[k + j + half_m] = mont.mont_sub(u, v);
            }
        }

        m *= 2;
    }

    // Scale by n^(-1), stay in Montgomery form
    for x in a.iter_mut() {
        *x = mont.mont_mul(*x, twiddles.n_inv_mont);
    }
}

#[cfg(test)]
mod tests {
    use super::super::NTT_PRIME_1;
    use super::*;

    #[test]
    fn test_bit_reverse_permute() {
        let mut a = vec![0, 1, 2, 3, 4, 5, 6, 7];
        bit_reverse_permute(&mut a, 8);
        assert_eq!(a, vec![0, 4, 2, 6, 1, 5, 3, 7]);

        bit_reverse_permute(&mut a, 8);
        assert_eq!(a, vec![0, 1, 2, 3, 4, 5, 6, 7]);
    }

    #[test]
    fn test_montgomery_context() {
        let p = NTT_PRIME_1;
        let mont = MontgomeryContext::new(p);

        // Test round-trip conversion
        for &val in &[0u64, 1, 2, 100, 1000000, p - 1] {
            let mont_val = mont.to_montgomery(val);
            let back = mont.reduce_from_montgomery(mont_val);
            assert_eq!(back, val, "Round-trip failed for {}", val);
        }

        // Test Montgomery multiplication
        let a = 12345u64;
        let b = 67890u64;
        let expected = ((a as u128 * b as u128) % p as u128) as u64;

        let a_mont = mont.to_montgomery(a);
        let b_mont = mont.to_montgomery(b);
        let c_mont = mont.mont_mul(a_mont, b_mont);
        let c = mont.reduce_from_montgomery(c_mont);

        assert_eq!(c, expected, "Montgomery multiplication failed");
    }
}
