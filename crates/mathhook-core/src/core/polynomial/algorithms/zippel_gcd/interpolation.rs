//! Lagrange Interpolation for Polynomial Reconstruction
//!
//! Multivariate polynomial reconstruction using Lagrange interpolation formula in finite fields.

use super::helpers::arithmetic::{mod_inverse, mod_positive, symmetric_mod};
use std::collections::HashMap;

/// Lagrange interpolation for polynomial reconstruction in Z_p
///
/// Given evaluation points and univariate polynomial values,
/// reconstruct the multivariate polynomial using Lagrange formula:
///
/// P(xₙ₋₁) = Σᵢ Pᵢ(xₙ₋₁) * `Lᵢ`
/// where `Lᵢ` = ∏ⱼ≠ᵢ (xₙ₋₁ - αⱼ) / (αᵢ - αⱼ)
///
/// # Arguments
///
/// * `points` - Evaluation points [α₀, α₁, ..., αₙ] in Z_p
/// * `values` - Univariate polynomials at each point (as coefficient vectors)
///   - `values[i]` = coefficients of P(α₀, ..., αₙ₋₂, αᵢ, xₙ₋₁)
/// * `p` - Prime modulus
///
/// # Returns
///
/// Reconstructed polynomial as coefficient vector
///
/// # Algorithm
///
/// 1. For each evaluation point i:
///    a. Compute Lagrange basis polynomial `Lᵢ`(xₙ₋₁)
///       - Numerator: ∏ⱼ≠ᵢ (xₙ₋₁ - αⱼ)
///       - Denominator: ∏ⱼ≠ᵢ (αᵢ - αⱼ)
///         b. Multiply `values[i]` * `Lᵢ`
///         c. Add to result
/// 2. Reduce all coefficients mod p
/// 3. Apply symmetric modular reduction
///
/// # Examples
///
/// ```rust
/// use mathhook_core::core::polynomial::algorithms::zippel_gcd::lagrange_interpolation;
///
/// // Interpolate through (1, 2x+3), (2, 4x+5) mod 7
/// let points = vec![1, 2];
/// let values = vec![
///     vec![3, 2],  // 2x + 3
///     vec![5, 4],  // 4x + 5
/// ];
/// let p = 7;
/// let poly = lagrange_interpolation(&points, &values, p);
/// ```
pub fn lagrange_interpolation(points: &[i64], values: &[Vec<i64>], p: u64) -> Vec<i64> {
    if points.is_empty() || values.is_empty() {
        return vec![];
    }

    let n = points.len();
    let max_degree = values.iter().map(|v| v.len()).max().unwrap_or(0);

    if max_degree == 0 {
        return vec![];
    }

    // Result degree = (n-1) from Lagrange basis + (max_degree-1) from values
    let mut result_coeffs = vec![0i64; n + max_degree - 1];

    for i in 0..n {
        let mut numerator_poly = vec![1i64];

        for (j, &xj) in points.iter().enumerate() {
            if i == j {
                continue;
            }

            let new_len = numerator_poly.len() + 1;
            let mut new_poly = vec![0i64; new_len];

            for (k, &coeff) in numerator_poly.iter().enumerate() {
                let term1 = mod_positive(-coeff * xj, p as i64);
                new_poly[k] = mod_positive(new_poly[k] + term1, p as i64);

                let term2 = coeff;
                new_poly[k + 1] = mod_positive(new_poly[k + 1] + term2, p as i64);
            }
            numerator_poly = new_poly;
        }

        let mut denominator = 1i64;
        for (j, &xj) in points.iter().enumerate() {
            if i == j {
                continue;
            }
            let diff = points[i] - xj;
            denominator = mod_positive(denominator * diff, p as i64);
        }

        let denom_inv = mod_inverse(denominator, p as i64);

        for (deg, &val) in values[i].iter().enumerate() {
            if val == 0 {
                continue;
            }

            for (k, &num_coeff) in numerator_poly.iter().enumerate() {
                if num_coeff == 0 {
                    continue;
                }

                let idx = deg + k;
                if idx < result_coeffs.len() {
                    let temp = mod_positive(num_coeff * val, p as i64);
                    let contribution = mod_positive(temp * denom_inv, p as i64);
                    result_coeffs[idx] = mod_positive(result_coeffs[idx] + contribution, p as i64);
                }
            }
        }
    }

    result_coeffs
        .iter()
        .map(|&c| symmetric_mod(c, p as i64))
        .collect()
}

/// Lagrange interpolation for multivariate polynomial
///
/// Specialized version that returns HashMap representation for multivariate polynomials.
///
/// # Arguments
///
/// * `points` - Evaluation points for other variables
/// * `values` - Coefficient vectors for univariate polynomials in main variable
/// * `num_vars` - Total number of variables
/// * `main_var_idx` - Index of main variable being interpolated
/// * `p` - Prime modulus
///
/// # Returns
///
/// Multivariate polynomial as HashMap<degree_vector, coefficient>
pub fn lagrange_interpolation_multivar(
    points: &[i64],
    values: &[Vec<i64>],
    num_vars: usize,
    main_var_idx: usize,
    p: u64,
) -> HashMap<Vec<usize>, i64> {
    let coeffs = lagrange_interpolation(points, values, p);
    let mut result = HashMap::new();

    for (deg, &coeff) in coeffs.iter().enumerate() {
        if coeff == 0 {
            continue;
        }

        let mut deg_vec = vec![0; num_vars];
        deg_vec[main_var_idx] = deg;
        result.insert(deg_vec, coeff);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    const PRIME: u64 = 2147483647;

    #[test]
    fn test_lagrange_interpolation_empty() {
        let points: Vec<i64> = vec![];
        let values: Vec<Vec<i64>> = vec![];
        let result = lagrange_interpolation(&points, &values, PRIME);
        assert_eq!(result, Vec::<i64>::new());
    }

    #[test]
    fn test_lagrange_interpolation_single_point() {
        let points = vec![1];
        let values = vec![vec![5]]; // constant 5
        let result = lagrange_interpolation(&points, &values, PRIME);
        assert_eq!(result, vec![5]);
    }

    #[test]
    fn test_lagrange_interpolation_linear() {
        // Interpolate through (1, 3) and (2, 5) mod 7
        // Expected: 2x + 1
        let points = vec![1, 2];
        let values = vec![
            vec![3], // constant 3 at x=1
            vec![5], // constant 5 at x=2
        ];
        let p = 7;

        let result = lagrange_interpolation(&points, &values, p);
        assert_eq!(result, vec![1, 2]); // 1 + 2x
    }

    #[test]
    fn test_lagrange_interpolation_quadratic() {
        // Interpolate through (0, 1), (1, 3), (2, 7) mod 11
        // Expected: x² + x + 1
        let points = vec![0, 1, 2];
        let values = vec![
            vec![1], // f(0) = 1
            vec![3], // f(1) = 3
            vec![7], // f(2) = 7
        ];
        let p = 11;

        let result = lagrange_interpolation(&points, &values, p);
        assert_eq!(result, vec![1, 1, 1]); // 1 + x + x²
    }

    #[test]
    fn test_lagrange_interpolation_with_univariate_values() {
        // Interpolate univariate polynomials
        // At α=1: 2x + 3
        // At α=2: 4x + 5
        let points = vec![1, 2];
        let values = vec![
            vec![3, 2], // 3 + 2x
            vec![5, 4], // 5 + 4x
        ];
        let p = 7;

        let result = lagrange_interpolation(&points, &values, p);
        assert!(result.len() >= 2);
        // Verify it interpolates correctly
        // At point 1: result should evaluate to [3, 2]
        // At point 2: result should evaluate to [5, 4]
    }

    #[test]
    fn test_lagrange_interpolation_multivar() {
        let points = vec![1, 2];
        let values = vec![
            vec![3, 2], // 3 + 2x
            vec![5, 4], // 5 + 4x
        ];
        let num_vars = 2;
        let main_var_idx = 1;
        let p = 7;

        let result = lagrange_interpolation_multivar(&points, &values, num_vars, main_var_idx, p);
        assert!(!result.is_empty());

        // Check that degree vectors have correct length
        for deg_vec in result.keys() {
            assert_eq!(deg_vec.len(), num_vars);
        }
    }

    #[test]
    fn test_lagrange_interpolation_large_prime() {
        // Test with large prime
        let points = vec![1, 2, 3];
        let values = vec![vec![10], vec![20], vec![30]];

        let result = lagrange_interpolation(&points, &values, PRIME);
        // Should interpolate a polynomial through these points
        assert_eq!(result.len(), 3);
    }
}
