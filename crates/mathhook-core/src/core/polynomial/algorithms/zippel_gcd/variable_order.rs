//! Variable Ordering Optimization
//!
//! Functions for optimizing variable ordering in multivariate polynomial GCD computation.
//! Processing low-degree variables first reduces intermediate coefficient sizes.

use std::collections::HashMap;

/// Order variables by estimated GCD degree (ascending)
///
/// Processing low-degree variables first reduces intermediate coefficient
/// sizes and improves performance of multivariate GCD computation.
///
/// # Arguments
///
/// * `f` - First polynomial as HashMap<degree_vector, coefficient>
/// * `g` - Second polynomial as HashMap<degree_vector, coefficient>
/// * `num_vars` - Number of variables
///
/// # Returns
///
/// Vector of variable indices ordered by ascending estimated GCD degree
///
/// # Algorithm
///
/// For each variable i:
/// 1. `deg_i(f)` = max degree of x_i in f
/// 2. `deg_i(g)` = max degree of x_i in g
/// 3. `estimated_gcd_degree[i]` = min(`deg_i(f)`, `deg_i(g)`)
///
/// Sort variables by ascending estimated_gcd_degree
///
/// # Examples
///
/// ```rust
/// use std::collections::HashMap;
/// use mathhook_core::core::polynomial::algorithms::zippel_gcd::order_variables_by_degree;
///
/// // f = x³y²z
/// let mut f = HashMap::new();
/// f.insert(vec![3, 2, 1], 1);
///
/// // g = x²y³z
/// let mut g = HashMap::new();
/// g.insert(vec![2, 3, 1], 1);
///
/// let ordered = order_variables_by_degree(&f, &g, 3);
/// // Result: [2, 0, 1] because:
/// // - var 2 (z): min(1,1) = 1 (lowest)
/// // - var 0 (x): min(3,2) = 2 (middle)
/// // - var 1 (y): min(2,3) = 2 (middle, tie broken by index)
/// ```
pub fn order_variables_by_degree(
    f: &HashMap<Vec<usize>, i64>,
    g: &HashMap<Vec<usize>, i64>,
    num_vars: usize,
) -> Vec<usize> {
    if num_vars == 0 {
        return vec![];
    }

    if num_vars == 1 {
        return vec![0];
    }

    let mut degree_estimates: Vec<(usize, usize)> = Vec::with_capacity(num_vars);

    for var_idx in 0..num_vars {
        let f_deg = get_degree_in_var(f, var_idx);
        let g_deg = get_degree_in_var(g, var_idx);
        let estimated_gcd_deg = std::cmp::min(f_deg, g_deg);

        degree_estimates.push((var_idx, estimated_gcd_deg));
    }

    degree_estimates.sort_by(|(idx_a, deg_a), (idx_b, deg_b)| match deg_a.cmp(deg_b) {
        std::cmp::Ordering::Equal => idx_a.cmp(idx_b),
        other => other,
    });

    degree_estimates.iter().map(|(idx, _)| *idx).collect()
}

/// Get maximum degree of a variable in a multivariate polynomial
///
/// # Arguments
///
/// * `poly` - Polynomial as HashMap<degree_vector, coefficient>
/// * `var_idx` - Index of variable to check
///
/// # Returns
///
/// Maximum degree of variable at index `var_idx`
fn get_degree_in_var(poly: &HashMap<Vec<usize>, i64>, var_idx: usize) -> usize {
    poly.keys()
        .filter_map(|deg_vec| deg_vec.get(var_idx).copied())
        .max()
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_order_variables_empty() {
        let f = HashMap::new();
        let g = HashMap::new();
        let result = order_variables_by_degree(&f, &g, 0);
        assert_eq!(result.len(), 0);
    }

    #[test]
    fn test_order_variables_single() {
        // f = x³
        let mut f = HashMap::new();
        f.insert(vec![3], 1);

        // g = x²
        let mut g = HashMap::new();
        g.insert(vec![2], 1);

        let result = order_variables_by_degree(&f, &g, 1);
        assert_eq!(result, vec![0]);
    }

    #[test]
    fn test_order_variables_two_vars_same_degree() {
        // f = x³y², g = x²y³
        let mut f = HashMap::new();
        f.insert(vec![3, 2], 1);

        let mut g = HashMap::new();
        g.insert(vec![2, 3], 1);

        let result = order_variables_by_degree(&f, &g, 2);
        // Both have min degree 2, so order by index: [0, 1]
        assert_eq!(result, vec![0, 1]);
    }

    #[test]
    fn test_order_variables_three_vars_different_degrees() {
        // f = x³y²z
        let mut f = HashMap::new();
        f.insert(vec![3, 2, 1], 1);

        // g = x²y³z
        let mut g = HashMap::new();
        g.insert(vec![2, 3, 1], 1);

        let result = order_variables_by_degree(&f, &g, 3);
        // z: min(1,1)=1, x: min(3,2)=2, y: min(2,3)=2
        // Result: [2, 0, 1] (z first, then x and y by index)
        assert_eq!(result, vec![2, 0, 1]);
    }

    #[test]
    fn test_order_variables_sparse_polynomial() {
        // f = x⁴ + y
        let mut f = HashMap::new();
        f.insert(vec![4, 0], 1);
        f.insert(vec![0, 1], 1);

        // g = x + y⁵
        let mut g = HashMap::new();
        g.insert(vec![1, 0], 1);
        g.insert(vec![0, 5], 1);

        let result = order_variables_by_degree(&f, &g, 2);
        // x: min(4,1)=1, y: min(1,5)=1
        // Both degree 1, so order by index: [0, 1]
        assert_eq!(result, vec![0, 1]);
    }

    #[test]
    fn test_get_degree_in_var() {
        // f = 2x³y² + 5xy⁴
        let mut f = HashMap::new();
        f.insert(vec![3, 2], 2);
        f.insert(vec![1, 4], 5);

        assert_eq!(get_degree_in_var(&f, 0), 3); // max x degree is 3
        assert_eq!(get_degree_in_var(&f, 1), 4); // max y degree is 4
    }

    #[test]
    fn test_get_degree_in_var_empty() {
        let f: HashMap<Vec<usize>, i64> = HashMap::new();
        assert_eq!(get_degree_in_var(&f, 0), 0);
    }

    #[test]
    fn test_get_degree_in_var_missing_index() {
        // f = x³ (only one variable, but we check index 1)
        let mut f = HashMap::new();
        f.insert(vec![3], 1);

        assert_eq!(get_degree_in_var(&f, 1), 0);
    }
}
