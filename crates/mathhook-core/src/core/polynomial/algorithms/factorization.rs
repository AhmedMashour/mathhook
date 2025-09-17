//! Polynomial Factorization Algorithms
//!
//! Pure polynomial factorization using IntPoly and `Poly<T>`.
//!
//! Expression-based wrappers MOVED TO ALGEBRA LAYER.
//! Use `crate::algebra::polynomial_advanced::AdvancedPolynomial` for:
//! - `extract_common_factor()`
//! - `factor_numeric()`
//! - `polynomial_content()`

use crate::core::polynomial::poly::Poly;
use crate::core::polynomial::traits::EuclideanDomain;

/// Square-free factorization using Yun's algorithm
///
/// Pure `Poly<T>` implementation - no Expression conversions during algorithm.
///
/// # Algorithm (Yun's Algorithm)
///
/// Given f(x) ∈ ``R[x]`` where R is a Euclidean domain, computes f = ∏ᵢ `fᵢ`ⁱ where each `fᵢ` is square-free:
///
/// 1. Compute g = gcd(f, f')
/// 2. Set h = f / g (contains all square-free parts)
/// 3. For each multiplicity i:
///    - Compute s = gcd(g, h)
///    - Extract `fᵢ` = h / s
///    - Update: g = g / s, h = s
///    - Increment i
///
/// # Arguments
///
/// * `poly` - Polynomial to decompose
///
/// # Returns
///
/// List of (factor, multiplicity) pairs where f = ∏ (factor^multiplicity)
pub fn square_free_factorization_poly<T: EuclideanDomain>(
    poly: &Poly<T>,
) -> Result<Vec<(Poly<T>, usize)>, crate::error::MathError> {
    if poly.is_zero() {
        return Ok(vec![]);
    }

    if poly.is_constant() {
        return Ok(vec![(poly.clone(), 1)]);
    }

    let derivative = poly.derivative();

    if derivative.is_zero() {
        return Ok(vec![(poly.clone(), 1)]);
    }

    let g = poly.gcd(&derivative)?;

    if g.is_constant() && !g.is_zero() {
        let lc = g.leading_coeff().abs();
        if lc.is_one() {
            return Ok(vec![(poly.clone(), 1)]);
        }
    }

    let (h, _) = poly.div_rem(&g)?;

    let mut result = Vec::new();
    let mut current_g = g;
    let mut current_h = h;
    let mut multiplicity = 1;

    while !current_h.is_constant()
        || (current_h.degree().is_some()
            && current_h.degree() == Some(0)
            && !current_h.leading_coeff().abs().is_one())
    {
        let s = current_g.gcd(&current_h)?;
        if s.is_zero() {
            break;
        }
        let (factor, _) = current_h.div_rem(&s)?;

        if !(factor.is_zero() || (factor.is_constant() && factor.leading_coeff().abs().is_one())) {
            result.push((factor, multiplicity));
        }

        let (new_g, _) = current_g.div_rem(&s)?;
        current_g = new_g;
        current_h = s;
        multiplicity += 1;

        if multiplicity > 1000 {
            break;
        }
    }

    if !current_g.is_constant()
        || (current_g.degree().is_some()
            && !current_g.is_zero()
            && !current_g.leading_coeff().abs().is_one())
    {
        result.push((current_g, multiplicity));
    }

    if result.is_empty() {
        Ok(vec![(Poly::constant(T::one()), 1)])
    } else {
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::polynomial::poly::IntPoly;

    #[test]
    fn test_square_free_poly_direct() {
        let poly = IntPoly::from_coeffs(vec![1, -2, 1]);
        let factors = square_free_factorization_poly(&poly).unwrap();
        assert!(!factors.is_empty());
    }

    #[test]
    fn test_square_free_poly_high_multiplicity() {
        let poly = IntPoly::from_coeffs(vec![1, -1]);
        let p2 = poly.mul(&poly);
        let p4 = p2.mul(&p2);
        let p8 = p4.mul(&p4);
        let factors = square_free_factorization_poly(&p8).unwrap();
        assert!(!factors.is_empty());
        assert!(factors.iter().any(|(_, m)| *m > 1));
    }

    #[test]
    fn test_square_free_constant() {
        let poly = IntPoly::from_coeffs(vec![5]);
        let factors = square_free_factorization_poly(&poly).unwrap();
        assert_eq!(factors.len(), 1);
        assert_eq!(factors[0].1, 1);
    }

    #[test]
    fn test_square_free_linear() {
        let poly = IntPoly::from_coeffs(vec![1, -1]);
        let factors = square_free_factorization_poly(&poly).unwrap();
        assert_eq!(factors.len(), 1);
        assert_eq!(factors[0].1, 1);
    }

    #[test]
    fn test_square_free_quadratic() {
        let poly = IntPoly::from_coeffs(vec![1, 0, -1]);
        let factors = square_free_factorization_poly(&poly).unwrap();
        assert!(!factors.is_empty());
    }
}
