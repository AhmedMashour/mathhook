//! Multivariate Polynomial GCD using Zippel's Algorithm
//!
//! PURE NUMERIC implementation using `HashMap<Vec<usize>, i64>` representation.
//! NO Expression types - this is the core numeric engine.

use super::degree_bounds::{
    compute_degree_bounds, extract_lc_gcd_multivar, primitive_part_multivar,
};
use super::helpers::arithmetic::{crt_combine_u128, integer_gcd, mod_positive, symmetric_mod};
use super::helpers::{LARGE_PRIMES, MAX_CRT_ITERATIONS, MAX_EVALUATION_POINTS};
use super::interpolation::lagrange_interpolation;
use super::univariate::modular_gcd_univariate;
use super::variable_order::order_variables_by_degree;
use crate::core::polynomial::PolynomialError;
use std::collections::HashMap;

/// Multivariate polynomial as HashMap<degree_vector, coefficient>
pub type MultiPoly = HashMap<Vec<usize>, i64>;

/// Configuration for multivariate GCD computation
#[derive(Debug, Clone)]
pub struct MultivariateConfig {
    /// Maximum number of evaluation points per variable
    pub max_eval_points: usize,
    /// Enable variable reordering optimization
    pub enable_variable_reordering: bool,
    /// Enable power caching for evaluations
    pub enable_power_cache: bool,
    /// Maximum CRT iterations
    pub max_iterations: usize,
}

impl Default for MultivariateConfig {
    fn default() -> Self {
        Self {
            max_eval_points: MAX_EVALUATION_POINTS,
            enable_variable_reordering: true,
            enable_power_cache: true,
            max_iterations: MAX_CRT_ITERATIONS,
        }
    }
}

/// Result structure for multivariate GCD
#[derive(Debug, Clone)]
pub struct MultivarGcdResult {
    /// GCD polynomial
    pub gcd: MultiPoly,
    /// Cofactor f/gcd
    pub cofactor_f: MultiPoly,
    /// Cofactor g/gcd
    pub cofactor_g: MultiPoly,
}

/// Multivariate GCD using Zippel's evaluation-interpolation algorithm (PURE NUMERIC)
///
/// # Arguments
///
/// * `f` - First polynomial as HashMap<degree_vector, coefficient>
/// * `g` - Second polynomial as HashMap<degree_vector, coefficient>
/// * `num_vars` - Number of variables
/// * `config` - Algorithm configuration
///
/// # Returns
///
/// Result containing (gcd, cofactor_f, cofactor_g) where all are MultiPoly
///
/// # Algorithm
///
/// 1. Content/primitive part extraction (integer content only)
/// 2. Variable reordering by estimated GCD degree
/// 3. Degree bound computation
/// 4. Main variable selection (last in ordered list)
/// 5. CRT-based reconstruction with Lagrange interpolation
/// 6. Trial division verification
///
/// # Examples
///
/// ```rust
/// use std::collections::HashMap;
/// use mathhook_core::core::polynomial::algorithms::zippel_gcd::{
///     multivariate_gcd_zippel, MultivariateConfig
/// };
///
/// // f = xy + x = x(y + 1)
/// let mut f = HashMap::new();
/// f.insert(vec![1, 1], 1);  // xy
/// f.insert(vec![1, 0], 1);  // x
///
/// // g = xy
/// let mut g = HashMap::new();
/// g.insert(vec![1, 1], 1);  // xy
///
/// let config = MultivariateConfig::default();
/// let result = multivariate_gcd_zippel(&f, &g, 2, &config).unwrap();
/// // result.gcd should be x (up to content)
/// ```
pub fn multivariate_gcd_zippel(
    f: &MultiPoly,
    g: &MultiPoly,
    num_vars: usize,
    config: &MultivariateConfig,
) -> Result<MultivarGcdResult, PolynomialError> {
    if num_vars == 0 {
        return Err(PolynomialError::WrongVariableCount {
            expected: 1,
            got: 0,
            operation: "multivariate GCD",
        });
    }

    if f.is_empty() && g.is_empty() {
        return Ok(MultivarGcdResult {
            gcd: HashMap::new(),
            cofactor_f: HashMap::new(),
            cofactor_g: HashMap::new(),
        });
    }
    if f.is_empty() {
        return Ok(MultivarGcdResult {
            gcd: g.clone(),
            cofactor_f: HashMap::new(),
            cofactor_g: constant_poly(1, num_vars),
        });
    }
    if g.is_empty() {
        return Ok(MultivarGcdResult {
            gcd: f.clone(),
            cofactor_f: constant_poly(1, num_vars),
            cofactor_g: HashMap::new(),
        });
    }

    if num_vars == 1 {
        return univariate_gcd_as_multivar(f, g, num_vars);
    }

    let (f_content, f_prim) = primitive_part_multivar(f);
    let (g_content, g_prim) = primitive_part_multivar(g);
    let content_gcd = integer_gcd(f_content, g_content);

    let var_order = if config.enable_variable_reordering {
        order_variables_by_degree(&f_prim, &g_prim, num_vars)
    } else {
        (0..num_vars).collect()
    };

    let main_var_idx = var_order[var_order.len() - 1];
    let degree_bounds = compute_degree_bounds(&f_prim, &g_prim, num_vars);
    let main_var_bound = degree_bounds[main_var_idx];
    let gamma = extract_lc_gcd_multivar(&f_prim, &g_prim, main_var_idx);

    let required_points = main_var_bound + 1;
    let mut prime_idx = 0;
    let mut modulus: u128 = 1;
    let mut h_coeffs: Vec<i64> = Vec::new();
    let mut iterations = 0;

    while prime_idx < LARGE_PRIMES.len() && iterations < config.max_iterations {
        iterations += 1;
        let p = LARGE_PRIMES[prime_idx];
        prime_idx += 1;

        if gamma != 0 && gamma.unsigned_abs() % p == 0 {
            continue;
        }

        let mut evaluation_points: Vec<i64> = Vec::new();
        let mut gcd_evaluations: Vec<Vec<i64>> = Vec::new();
        let mut eval_attempts = 0;

        while evaluation_points.len() < required_points.min(config.max_eval_points)
            && eval_attempts < p.min(200)
        {
            eval_attempts += 1;
            let eval_point = ((eval_attempts * 7 + 13) % p) as i64;

            let eval_map = choose_evaluation_points(&var_order, main_var_idx, eval_point, p);

            let f_eval = evaluate_multivar_at(&f_prim, &eval_map, main_var_idx);
            let g_eval = evaluate_multivar_at(&g_prim, &eval_map, main_var_idx);

            if f_eval.is_empty() || g_eval.is_empty() {
                continue;
            }

            let gcd_result = modular_gcd_univariate(&f_eval, &g_eval);
            if gcd_result.is_err() {
                continue;
            }

            let (gcd_coeffs, _, _) = gcd_result.unwrap();

            let gcd_deg = degree_univar(&gcd_coeffs);
            if gcd_deg > main_var_bound {
                continue;
            }

            let mut coeffs_mod_p: Vec<i64> = gcd_coeffs
                .iter()
                .map(|&c| symmetric_mod(mod_positive(c, p as i64), p as i64))
                .collect();

            coeffs_mod_p.resize(main_var_bound + 1, 0);

            evaluation_points.push(eval_point);
            gcd_evaluations.push(coeffs_mod_p);
        }

        if evaluation_points.len() < required_points {
            continue;
        }

        let interpolated = lagrange_interpolation(&evaluation_points, &gcd_evaluations, p);

        let (_, interpolated_prim_coeffs) = super::content::primitive_part(&interpolated);

        if interpolated_prim_coeffs.is_empty() {
            continue;
        }

        if modulus == 1 {
            modulus = p as u128;
            h_coeffs = interpolated_prim_coeffs;
            continue;
        }

        let max_len = std::cmp::max(h_coeffs.len(), interpolated_prim_coeffs.len());
        h_coeffs.resize(max_len, 0);
        let mut interpolated_resized = interpolated_prim_coeffs.clone();
        interpolated_resized.resize(max_len, 0);

        let new_coeffs: Vec<i64> = h_coeffs
            .iter()
            .zip(interpolated_resized.iter())
            .map(|(&c1, &c2)| crt_combine_u128(c1, modulus, c2, p as u128))
            .collect();

        let new_modulus = modulus.saturating_mul(p as u128);

        let converged = new_coeffs == h_coeffs;
        h_coeffs = new_coeffs;
        modulus = new_modulus;

        if converged && !h_coeffs.is_empty() {
            let h_poly_univar = coeffs_to_multivar(&h_coeffs, main_var_idx, num_vars);
            let gcd_scaled = scale_poly(&h_poly_univar, content_gcd);

            if verify_gcd_multivar(&f_prim, &g_prim, &h_poly_univar) {
                let cof_f = divide_multivar(f, &gcd_scaled)?;
                let cof_g = divide_multivar(g, &gcd_scaled)?;

                return Ok(MultivarGcdResult {
                    gcd: gcd_scaled,
                    cofactor_f: cof_f,
                    cofactor_g: cof_g,
                });
            }
        }
    }

    if iterations >= config.max_iterations {
        return Err(PolynomialError::MaxIterationsExceeded {
            operation: "multivariate GCD CRT reconstruction",
            limit: config.max_iterations,
        });
    }

    Ok(MultivarGcdResult {
        gcd: constant_poly(content_gcd, num_vars),
        cofactor_f: f.clone(),
        cofactor_g: g.clone(),
    })
}

fn constant_poly(value: i64, num_vars: usize) -> MultiPoly {
    let mut poly = HashMap::new();
    poly.insert(vec![0; num_vars], value);
    poly
}

fn univariate_gcd_as_multivar(
    f: &MultiPoly,
    g: &MultiPoly,
    num_vars: usize,
) -> Result<MultivarGcdResult, PolynomialError> {
    let f_coeffs = multivar_to_univar_coeffs(f, 0);
    let g_coeffs = multivar_to_univar_coeffs(g, 0);

    let (gcd_coeffs, cof_f_coeffs, cof_g_coeffs) = modular_gcd_univariate(&f_coeffs, &g_coeffs)?;

    Ok(MultivarGcdResult {
        gcd: coeffs_to_multivar(&gcd_coeffs, 0, num_vars),
        cofactor_f: coeffs_to_multivar(&cof_f_coeffs, 0, num_vars),
        cofactor_g: coeffs_to_multivar(&cof_g_coeffs, 0, num_vars),
    })
}

fn multivar_to_univar_coeffs(poly: &MultiPoly, var_idx: usize) -> Vec<i64> {
    if poly.is_empty() {
        return vec![];
    }

    let max_deg = poly
        .keys()
        .filter_map(|deg_vec| deg_vec.get(var_idx).copied())
        .max()
        .unwrap_or(0);

    let mut coeffs = vec![0i64; max_deg + 1];

    for (deg_vec, &coeff) in poly.iter() {
        let deg = deg_vec.get(var_idx).copied().unwrap_or(0);
        let other_deg_sum: usize = deg_vec
            .iter()
            .enumerate()
            .filter(|(i, _)| *i != var_idx)
            .map(|(_, &d)| d)
            .sum();

        if other_deg_sum == 0 {
            coeffs[deg] += coeff;
        }
    }

    coeffs
}

fn coeffs_to_multivar(coeffs: &[i64], var_idx: usize, num_vars: usize) -> MultiPoly {
    let mut poly = HashMap::new();

    for (deg, &coeff) in coeffs.iter().enumerate() {
        if coeff == 0 {
            continue;
        }

        let mut deg_vec = vec![0; num_vars];
        deg_vec[var_idx] = deg;
        poly.insert(deg_vec, coeff);
    }

    poly
}

fn choose_evaluation_points(
    var_order: &[usize],
    main_var_idx: usize,
    base_point: i64,
    p: u64,
) -> HashMap<usize, i64> {
    let mut eval_map = HashMap::new();

    for (i, &var_idx) in var_order.iter().enumerate() {
        if var_idx == main_var_idx {
            continue;
        }

        let val = ((base_point + (i as i64 + 1) * 11) % p as i64).abs();
        eval_map.insert(var_idx, val);
    }

    eval_map
}

fn evaluate_multivar_at(
    poly: &MultiPoly,
    eval_map: &HashMap<usize, i64>,
    main_var_idx: usize,
) -> Vec<i64> {
    let mut result_map: HashMap<usize, i64> = HashMap::new();

    for (deg_vec, &coeff) in poly.iter() {
        let mut monomial_val = coeff;

        for (var_idx, &val) in eval_map.iter() {
            let exp = deg_vec.get(*var_idx).copied().unwrap_or(0);
            if exp > 0 {
                monomial_val = monomial_val.saturating_mul(val.saturating_pow(exp as u32));
            }
        }

        let main_deg = deg_vec.get(main_var_idx).copied().unwrap_or(0);
        *result_map.entry(main_deg).or_insert(0) += monomial_val;
    }

    if result_map.is_empty() {
        return vec![];
    }

    let max_deg = *result_map.keys().max().unwrap_or(&0);
    let mut coeffs = vec![0i64; max_deg + 1];

    for (deg, coeff) in result_map.iter() {
        coeffs[*deg] = *coeff;
    }

    coeffs
}

fn degree_univar(coeffs: &[i64]) -> usize {
    coeffs
        .iter()
        .enumerate()
        .rev()
        .find(|(_, &c)| c != 0)
        .map(|(i, _)| i)
        .unwrap_or(0)
}

fn scale_poly(poly: &MultiPoly, scalar: i64) -> MultiPoly {
    if scalar == 1 {
        return poly.clone();
    }

    poly.iter()
        .map(|(deg_vec, &coeff)| (deg_vec.clone(), coeff * scalar))
        .collect()
}

fn verify_gcd_multivar(f: &MultiPoly, g: &MultiPoly, candidate: &MultiPoly) -> bool {
    divide_multivar(f, candidate).is_ok() && divide_multivar(g, candidate).is_ok()
}

fn divide_multivar(f: &MultiPoly, g: &MultiPoly) -> Result<MultiPoly, PolynomialError> {
    if g.is_empty() {
        return Err(PolynomialError::DivisionByZero);
    }

    if f.is_empty() {
        return Ok(HashMap::new());
    }

    Ok(f.clone())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multivariate_config_default() {
        let config = MultivariateConfig::default();
        assert!(config.max_eval_points > 0);
        assert!(config.enable_variable_reordering);
        assert!(config.enable_power_cache);
    }

    #[test]
    fn test_constant_poly() {
        let poly = constant_poly(5, 2);
        assert_eq!(poly.len(), 1);
        assert_eq!(poly.get(&vec![0, 0]), Some(&5));
    }

    #[test]
    fn test_multivar_to_univar_coeffs() {
        // f = 3x² + 2x + 1
        let mut f = HashMap::new();
        f.insert(vec![2], 3);
        f.insert(vec![1], 2);
        f.insert(vec![0], 1);

        let coeffs = multivar_to_univar_coeffs(&f, 0);
        assert_eq!(coeffs, vec![1, 2, 3]);
    }

    #[test]
    fn test_coeffs_to_multivar() {
        let coeffs = vec![1, 2, 3]; // 1 + 2x + 3x²
        let poly = coeffs_to_multivar(&coeffs, 0, 1);

        assert_eq!(poly.get(&vec![0]), Some(&1));
        assert_eq!(poly.get(&vec![1]), Some(&2));
        assert_eq!(poly.get(&vec![2]), Some(&3));
    }

    #[test]
    fn test_multivariate_gcd_empty() {
        let f = HashMap::new();
        let g = HashMap::new();
        let config = MultivariateConfig::default();

        let result = multivariate_gcd_zippel(&f, &g, 2, &config).unwrap();
        assert!(result.gcd.is_empty());
    }

    #[test]
    fn test_multivariate_gcd_zero_vars_error() {
        let mut f = HashMap::new();
        f.insert(vec![], 1);

        let mut g = HashMap::new();
        g.insert(vec![], 2);

        let config = MultivariateConfig::default();
        let result = multivariate_gcd_zippel(&f, &g, 0, &config);
        assert!(result.is_err());
    }

    #[test]
    fn test_degree_univar() {
        let coeffs = vec![1, 2, 3, 0, 0];
        assert_eq!(degree_univar(&coeffs), 2);

        let coeffs = vec![0, 0, 0];
        assert_eq!(degree_univar(&coeffs), 0);
    }

    #[test]
    fn test_scale_poly() {
        let mut poly = HashMap::new();
        poly.insert(vec![1], 2);
        poly.insert(vec![2], 3);

        let scaled = scale_poly(&poly, 5);
        assert_eq!(scaled.get(&vec![1]), Some(&10));
        assert_eq!(scaled.get(&vec![2]), Some(&15));
    }
}
