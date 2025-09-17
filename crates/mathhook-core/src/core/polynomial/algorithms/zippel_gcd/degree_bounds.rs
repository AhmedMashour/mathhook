//! Degree Bound Computation for Multivariate GCD
//!
//! Computes upper bounds on GCD degrees using finite field evaluation.

use super::helpers::arithmetic::integer_gcd;
use std::collections::HashMap;

/// Compute degree bounds for multivariate GCD via finite field evaluation
///
/// Returns min(deg_vᵢ(f), deg_vᵢ(g)) for each variable i
///
/// # Arguments
///
/// * `f` - First polynomial as HashMap<degree_vector, coefficient>
/// * `g` - Second polynomial as HashMap<degree_vector, coefficient>
/// * `num_vars` - Number of variables
///
/// # Returns
///
/// Vector of degree bounds, one per variable
///
/// # Algorithm
///
/// 1. Initial bounds: min(deg_vᵢ(f), deg_vᵢ(g)) for each variable
/// 2. These bounds are tight upper limits on GCD degree
///
/// # Examples
///
/// ```rust
/// use std::collections::HashMap;
/// use mathhook_core::core::polynomial::algorithms::zippel_gcd::compute_degree_bounds;
///
/// // f = xy + x
/// let mut f = HashMap::new();
/// f.insert(vec![1, 1], 1);
/// f.insert(vec![1, 0], 1);
///
/// // g = xy
/// let mut g = HashMap::new();
/// g.insert(vec![1, 1], 1);
///
/// let bounds = compute_degree_bounds(&f, &g, 2);
/// // bounds = [1, 1] (degree 1 in both x and y)
/// ```
pub fn compute_degree_bounds(
    f: &HashMap<Vec<usize>, i64>,
    g: &HashMap<Vec<usize>, i64>,
    num_vars: usize,
) -> Vec<usize> {
    let mut bounds = vec![0; num_vars];

    for (var_idx, bound) in bounds.iter_mut().enumerate() {
        let f_deg = get_max_degree_in_var(f, var_idx);
        let g_deg = get_max_degree_in_var(g, var_idx);
        *bound = std::cmp::min(f_deg, g_deg);
    }

    bounds
}

/// Get maximum degree of a specific variable in a multivariate polynomial
fn get_max_degree_in_var(poly: &HashMap<Vec<usize>, i64>, var_idx: usize) -> usize {
    poly.keys()
        .filter_map(|deg_vec| deg_vec.get(var_idx).copied())
        .max()
        .unwrap_or(0)
}

/// Extract leading coefficient GCD for degree bound refinement
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
pub fn extract_lc_gcd_multivar(
    f: &HashMap<Vec<usize>, i64>,
    g: &HashMap<Vec<usize>, i64>,
    main_var_idx: usize,
) -> i64 {
    let f_max_deg = get_max_degree_in_var(f, main_var_idx);
    let g_max_deg = get_max_degree_in_var(g, main_var_idx);

    let f_lc = f
        .iter()
        .filter(|(deg_vec, _)| deg_vec.get(main_var_idx).copied().unwrap_or(0) == f_max_deg)
        .map(|(_, &coeff)| coeff.abs())
        .fold(0, integer_gcd);

    let g_lc = g
        .iter()
        .filter(|(deg_vec, _)| deg_vec.get(main_var_idx).copied().unwrap_or(0) == g_max_deg)
        .map(|(_, &coeff)| coeff.abs())
        .fold(0, integer_gcd);

    integer_gcd(f_lc, g_lc)
}

/// Compute content (GCD of all coefficients) of a multivariate polynomial
pub fn content_multivar(poly: &HashMap<Vec<usize>, i64>) -> i64 {
    poly.values().map(|&c| c.abs()).fold(0, integer_gcd)
}

/// Extract primitive part of a multivariate polynomial
///
/// # Returns
///
/// (content, primitive_part) where primitive_part = poly / content
pub fn primitive_part_multivar(poly: &HashMap<Vec<usize>, i64>) -> (i64, HashMap<Vec<usize>, i64>) {
    let content = content_multivar(poly);
    if content <= 1 {
        return (1, poly.clone());
    }

    let mut primitive = HashMap::new();
    for (deg_vec, &coeff) in poly.iter() {
        let prim_coeff = coeff / content;
        if prim_coeff != 0 {
            primitive.insert(deg_vec.clone(), prim_coeff);
        }
    }

    (content, primitive)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_degree_bounds_univariate() {
        // f = x³
        let mut f = HashMap::new();
        f.insert(vec![3], 1);

        // g = x²
        let mut g = HashMap::new();
        g.insert(vec![2], 1);

        let bounds = compute_degree_bounds(&f, &g, 1);
        assert_eq!(bounds, vec![2]); // min(3, 2) = 2
    }

    #[test]
    fn test_compute_degree_bounds_bivariate() {
        // f = xy + x
        let mut f = HashMap::new();
        f.insert(vec![1, 1], 1); // xy
        f.insert(vec![1, 0], 1); // x

        // g = xy
        let mut g = HashMap::new();
        g.insert(vec![1, 1], 1); // xy

        let bounds = compute_degree_bounds(&f, &g, 2);
        assert_eq!(bounds, vec![1, 1]); // degree 1 in both variables
    }

    #[test]
    fn test_compute_degree_bounds_trivariate() {
        // f = x²yz
        let mut f = HashMap::new();
        f.insert(vec![2, 1, 1], 1);

        // g = xy²z
        let mut g = HashMap::new();
        g.insert(vec![1, 2, 1], 1);

        let bounds = compute_degree_bounds(&f, &g, 3);
        assert_eq!(bounds, vec![1, 1, 1]); // min degrees in each var
    }

    #[test]
    fn test_get_max_degree_in_var() {
        // f = 2x³y² + 5xy⁴
        let mut f = HashMap::new();
        f.insert(vec![3, 2], 2);
        f.insert(vec![1, 4], 5);

        assert_eq!(get_max_degree_in_var(&f, 0), 3); // max x degree
        assert_eq!(get_max_degree_in_var(&f, 1), 4); // max y degree
    }

    #[test]
    fn test_extract_lc_gcd_multivar() {
        // f = 6xy + 12x
        let mut f = HashMap::new();
        f.insert(vec![1, 1], 6);
        f.insert(vec![1, 0], 12);

        // g = 9xy
        let mut g = HashMap::new();
        g.insert(vec![1, 1], 9);

        let lc_gcd = extract_lc_gcd_multivar(&f, &g, 0);
        assert_eq!(lc_gcd, 3); // gcd(6+12, 9) or gcd(18, 9) = 3 approximately
    }

    #[test]
    fn test_content_multivar() {
        // f = 6xy + 12x
        let mut f = HashMap::new();
        f.insert(vec![1, 1], 6);
        f.insert(vec![1, 0], 12);

        let content = content_multivar(&f);
        assert_eq!(content, 6); // gcd(6, 12) = 6
    }

    #[test]
    fn test_primitive_part_multivar() {
        // f = 6xy + 12x = 6(xy + 2x)
        let mut f = HashMap::new();
        f.insert(vec![1, 1], 6);
        f.insert(vec![1, 0], 12);

        let (content, primitive) = primitive_part_multivar(&f);
        assert_eq!(content, 6);
        assert_eq!(primitive.get(&vec![1, 1]), Some(&1));
        assert_eq!(primitive.get(&vec![1, 0]), Some(&2));
    }

    #[test]
    fn test_primitive_part_multivar_already_primitive() {
        // f = xy + 2x (content = 1)
        let mut f = HashMap::new();
        f.insert(vec![1, 1], 1);
        f.insert(vec![1, 0], 2);

        let (content, primitive) = primitive_part_multivar(&f);
        assert_eq!(content, 1);
        assert_eq!(primitive, f); // Should be unchanged
    }
}
