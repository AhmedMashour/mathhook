//! Content extraction and primitive part computation

use super::helpers::integer_gcd;

/// Extract content and primitive part from integer coefficient polynomial
///
/// Returns (content, primitive_part) where:
/// - content = GCD of all coefficients
/// - primitive_part = polynomial / content
///
/// # Arguments
///
/// * `coeffs` - Polynomial coefficients (ascending order: ``coeffs[i]`` is coeff of x^i)
///
/// # Returns
///
/// Tuple (content, primitive_part_coeffs)
///
/// # Examples
///
/// ```rust
/// use mathhook_core::core::polynomial::algorithms::zippel_gcd::primitive_part;
///
/// // 6x^2 + 12x + 18 = 6(x^2 + 2x + 3)
/// let coeffs = vec![18, 12, 6];
/// let (content, prim) = primitive_part(&coeffs);
/// assert_eq!(content, 6);
/// assert_eq!(prim, vec![3, 2, 1]);
/// ```
pub fn primitive_part(coeffs: &[i64]) -> (i64, Vec<i64>) {
    if coeffs.is_empty() {
        return (1, vec![]);
    }

    // Compute GCD of all coefficients
    let mut gcd = coeffs[0].abs();
    for &c in coeffs.iter().skip(1) {
        gcd = integer_gcd(gcd, c.abs());
        if gcd == 1 {
            return (1, coeffs.to_vec());
        }
    }

    if gcd == 0 || gcd == 1 {
        return (gcd, coeffs.to_vec());
    }

    // Divide all coefficients by GCD
    let prim: Vec<i64> = coeffs.iter().map(|&c| c / gcd).collect();
    (gcd, prim)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primitive_part_basic() {
        // 6x^2 + 12x + 18
        let coeffs = vec![18, 12, 6];
        let (content, prim) = primitive_part(&coeffs);
        assert_eq!(content, 6);
        assert_eq!(prim, vec![3, 2, 1]);
    }

    #[test]
    fn test_primitive_part_coprime() {
        // x^2 + 2x + 3 (already primitive)
        let coeffs = vec![3, 2, 1];
        let (content, prim) = primitive_part(&coeffs);
        assert_eq!(content, 1);
        assert_eq!(prim, vec![3, 2, 1]);
    }

    #[test]
    fn test_primitive_part_negative() {
        // -6x^2 - 12x - 18
        let coeffs = vec![-18, -12, -6];
        let (content, prim) = primitive_part(&coeffs);
        assert_eq!(content, 6);
        assert_eq!(prim, vec![-3, -2, -1]);
    }

    #[test]
    fn test_primitive_part_empty() {
        let coeffs = vec![];
        let (content, prim) = primitive_part(&coeffs);
        assert_eq!(content, 1);
        assert_eq!(prim, Vec::<i64>::new());
    }

    #[test]
    fn test_primitive_part_zero() {
        let coeffs = vec![0, 0, 0];
        let (_content, prim) = primitive_part(&coeffs);
        assert_eq!(prim, vec![0, 0, 0]);
    }
}
