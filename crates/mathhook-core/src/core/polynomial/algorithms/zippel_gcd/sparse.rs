//! Sparse Polynomial Operations
//!
//! Functions for analyzing sparsity and optimized operations for sparse polynomials.

/// Sparsity threshold: if density < this, use sparse algorithms
pub const SPARSITY_THRESHOLD: f64 = 0.3;

/// Sparsity information for a polynomial
#[derive(Debug, Clone)]
pub struct SparsityInfo {
    /// Number of non-zero coefficients
    pub nonzero_count: usize,
    /// Total number of coefficients (degree + 1)
    pub total_count: usize,
    /// Density ratio (nonzero / total)
    pub density: f64,
    /// Indices of non-zero coefficients
    pub nonzero_indices: Vec<usize>,
}

/// Analyze the sparsity of a polynomial
///
/// # Arguments
///
/// * `coeffs` - Polynomial coefficients
///
/// # Returns
///
/// `SparsityInfo` containing sparsity metrics
///
/// # Examples
///
/// ```rust
/// use mathhook_core::core::polynomial::algorithms::zippel_gcd::analyze_sparsity;
///
/// let sparse = vec![1, 0, 0, 0, 5];  // 1 + 5x⁴ (sparse)
/// let info = analyze_sparsity(&sparse);
/// assert_eq!(info.nonzero_count, 2);
/// assert!(info.density < 0.5);
///
/// let dense = vec![1, 2, 3, 4, 5];   // fully dense
/// let info = analyze_sparsity(&dense);
/// assert_eq!(info.density, 1.0);
/// ```
pub fn analyze_sparsity(coeffs: &[i64]) -> SparsityInfo {
    if coeffs.is_empty() {
        return SparsityInfo {
            nonzero_count: 0,
            total_count: 0,
            density: 0.0,
            nonzero_indices: vec![],
        };
    }

    let nonzero_indices: Vec<usize> = coeffs
        .iter()
        .enumerate()
        .filter(|(_, &c)| c != 0)
        .map(|(i, _)| i)
        .collect();

    let nonzero_count = nonzero_indices.len();
    let total_count = coeffs.len();
    let density = if total_count > 0 {
        nonzero_count as f64 / total_count as f64
    } else {
        0.0
    };

    SparsityInfo {
        nonzero_count,
        total_count,
        density,
        nonzero_indices,
    }
}

/// Check if polynomial should use sparse algorithms
///
/// Returns true if the polynomial is sparse enough to benefit from
/// sparse-specific optimizations.
pub fn is_sparse(coeffs: &[i64]) -> bool {
    let info = analyze_sparsity(coeffs);
    info.density < SPARSITY_THRESHOLD && info.total_count > 5
}

/// Sparse polynomial multiplication (only compute non-zero terms)
///
/// More efficient for sparse polynomials where many coefficients are zero.
///
/// # Arguments
///
/// * `f_coeffs` - First polynomial coefficients
/// * `g_coeffs` - Second polynomial coefficients
///
/// # Returns
///
/// Product polynomial coefficients
pub fn sparse_multiply(f_coeffs: &[i64], g_coeffs: &[i64]) -> Vec<i64> {
    let f_info = analyze_sparsity(f_coeffs);
    let g_info = analyze_sparsity(g_coeffs);

    if f_coeffs.is_empty() || g_coeffs.is_empty() {
        return vec![0];
    }

    let result_len = f_coeffs.len() + g_coeffs.len() - 1;
    let mut result = vec![0i64; result_len];

    for &i in &f_info.nonzero_indices {
        for &j in &g_info.nonzero_indices {
            result[i + j] = result[i + j].saturating_add(f_coeffs[i].saturating_mul(g_coeffs[j]));
        }
    }

    result
}

/// Generic polynomial coefficient multiplication
///
/// Multiply two univariate polynomials represented as coefficient vectors.
/// Uses standard convolution algorithm.
///
/// # Arguments
///
/// * `f_coeffs` - First polynomial coefficients (ascending order)
/// * `g_coeffs` - Second polynomial coefficients (ascending order)
///
/// # Returns
///
/// Product polynomial coefficients
///
/// # Algorithm
///
/// Standard convolution: `(f * g)[k]` = Σᵢ `f[i]` * `g[k-i]`
///
/// # Examples
///
/// ```rust
/// use mathhook_core::core::polynomial::algorithms::zippel_gcd::polynomial_multiply_coeffs;
///
/// // (x + 1) * (x + 2) = x² + 3x + 2
/// let f = vec![1, 1];  // 1 + x
/// let g = vec![2, 1];  // 2 + x
/// let product = polynomial_multiply_coeffs(&f, &g);
/// assert_eq!(product, vec![2, 3, 1]);  // 2 + 3x + x²
/// ```
pub fn polynomial_multiply_coeffs(f_coeffs: &[i64], g_coeffs: &[i64]) -> Vec<i64> {
    if f_coeffs.is_empty() || g_coeffs.is_empty() {
        return vec![0];
    }

    let result_len = f_coeffs.len() + g_coeffs.len() - 1;
    let mut result = vec![0i64; result_len];

    for (i, &f_c) in f_coeffs.iter().enumerate() {
        if f_c == 0 {
            continue;
        }
        for (j, &g_c) in g_coeffs.iter().enumerate() {
            if g_c == 0 {
                continue;
            }
            result[i + j] = result[i + j].saturating_add(f_c.saturating_mul(g_c));
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_analyze_sparsity_dense() {
        let dense = vec![1, 2, 3, 4, 5];
        let info = analyze_sparsity(&dense);
        assert_eq!(info.nonzero_count, 5);
        assert_eq!(info.density, 1.0);
    }

    #[test]
    fn test_analyze_sparsity_sparse() {
        let sparse = vec![1, 0, 0, 0, 5];
        let info = analyze_sparsity(&sparse);
        assert_eq!(info.nonzero_count, 2);
        assert!(info.density < 0.5);
        assert_eq!(info.nonzero_indices, vec![0, 4]);
    }

    #[test]
    fn test_is_sparse() {
        let dense = vec![1, 2, 3, 4, 5, 6, 7, 8];
        assert!(!is_sparse(&dense));

        let sparse = vec![1, 0, 0, 0, 0, 0, 0, 5];
        assert!(is_sparse(&sparse));
    }

    #[test]
    fn test_sparse_multiply() {
        let f = vec![1, 0, 0, 2];
        let g = vec![3, 0, 1];
        let product = sparse_multiply(&f, &g);

        let expected = vec![3, 0, 1, 6, 0, 2];
        assert_eq!(product, expected);
    }

    #[test]
    fn test_polynomial_multiply_coeffs() {
        // (x + 1) * (x + 2) = x² + 3x + 2
        let f = vec![1, 1]; // 1 + x
        let g = vec![2, 1]; // 2 + x
        let product = polynomial_multiply_coeffs(&f, &g);
        assert_eq!(product, vec![2, 3, 1]); // 2 + 3x + x²

        // (2x + 3) * (x² + x + 1) = 2x³ + 5x² + 5x + 3
        let f = vec![3, 2]; // 3 + 2x
        let g = vec![1, 1, 1]; // 1 + x + x²
        let product = polynomial_multiply_coeffs(&f, &g);
        assert_eq!(product, vec![3, 5, 5, 2]); // 3 + 5x + 5x² + 2x³
    }

    #[test]
    fn test_polynomial_multiply_coeffs_empty() {
        let f: Vec<i64> = vec![];
        let g = vec![1, 2, 3];
        let product = polynomial_multiply_coeffs(&f, &g);
        assert_eq!(product, vec![0]);
    }

    #[test]
    fn test_polynomial_multiply_coeffs_with_zeros() {
        let f = vec![1, 0, 2]; // 1 + 2x²
        let g = vec![3, 0, 1]; // 3 + x²
        let product = polynomial_multiply_coeffs(&f, &g);
        // (1 + 2x²)(3 + x²) = 3 + x² + 6x² + 2x⁴ = 3 + 7x² + 2x⁴
        assert_eq!(product, vec![3, 0, 7, 0, 2]);
    }
}
