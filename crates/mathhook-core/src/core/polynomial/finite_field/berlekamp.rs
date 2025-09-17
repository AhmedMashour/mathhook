//! Polynomial Factorization over Finite Fields
//!
//! Implements Berlekamp's algorithm for complete factorization over `Z_p[x]`.
//!
//! # Mathematical Background
//!
//! ## Berlekamp's Algorithm
//!
//! Given a square-free monic polynomial f(x) ∈ `Z_p[x]`, Berlekamp's algorithm
//! factors f into irreducible polynomials over Z_p.
//!
//! **Key Idea**: The Frobenius endomorphism φ: a(x) → a(x)^p mod f(x) partitions
//! the algebra `Z_p[x]`/(f) into subspaces, one for each irreducible factor.
//!
//! **Algorithm**:
//! 1. Compute the Berlekamp matrix Q where `Q[i,j]` = coefficient of x^i in (x^p)^j mod f(x)
//! 2. Find the null space of (Q - I)
//! 3. For each non-trivial null space vector v(x), compute gcd(f, v - c) for c ∈ Z_p
//! 4. Recursively factor the results
//!
//! # Complexity
//!
//! - **Berlekamp matrix construction**: O(n² log p) using fast exponentiation
//! - **Null space computation**: O(n³) via Gaussian elimination over Z_p
//! - **Factor splitting**: O(p·n²) for each null space vector
//! - **Total**: O(n³ + p·n²) where n = deg(f)
//!
//! ## References
//!
//! - `[GCL92]` Geddes, Czapor, Labahn. "Algorithms for Computer Algebra", Chapter 6
//! - `[Knuth97]` Knuth. "The Art of Computer Programming", Vol 2, Section 4.6.2
use super::element::Zp;
use super::poly::PolyZp;
use super::{FiniteFieldError, FiniteFieldResult};
/// Compute x^p mod f(x) using repeated squaring
///
/// This is the Frobenius endomorphism evaluation needed for Berlekamp's algorithm.
///
/// # Arguments
///
/// * `f` - Modulus polynomial
/// * `p` - Prime modulus
///
/// # Returns
///
/// The polynomial representing x^p mod f(x)
///
/// # Complexity
///
/// O(log(p) * deg(f)²) using binary exponentiation
fn frobenius_mod(f: &PolyZp, p: u64) -> FiniteFieldResult<PolyZp> {
    let modulus = f.modulus();
    debug_assert_eq!(modulus, p, "prime mismatch");
    if f.is_zero() {
        return Err(FiniteFieldError::DivisionByZero);
    }
    let mut result = PolyZp::constant(1, modulus);
    let x = PolyZp::x(modulus);
    let mut base = x;
    let mut exp = p;
    while exp > 0 {
        if exp & 1 == 1 {
            result = result.mul_fast(&base);
            let (_, rem) = result.div_rem(f)?;
            result = rem;
        }
        if exp > 1 {
            base = base.mul_fast(&base);
            let (_, rem) = base.div_rem(f)?;
            base = rem;
        }
        exp >>= 1;
    }
    Ok(result)
}
/// Compute the Berlekamp matrix Q for a square-free monic polynomial
///
/// Q[i,j] = coefficient of x^i in (x^p)^j mod f(x)
///
/// # Arguments
///
/// * `f` - Square-free monic polynomial
///
/// # Returns
///
/// Matrix Q as Vec<Vec<u64>> where `Q[i][j]` is the `(i,j)` entry
#[allow(clippy::needless_range_loop)]
fn berlekamp_matrix(f: &PolyZp) -> FiniteFieldResult<Vec<Vec<u64>>> {
    let n = match f.degree() {
        Some(d) => d,
        None => return Err(FiniteFieldError::EmptyPolynomial),
    };
    if n == 0 {
        return Ok(vec![vec![1]]);
    }
    let p = f.modulus();
    let mut q = vec![vec![0u64; n]; n];
    let x_p = frobenius_mod(f, p)?;
    let mut current = PolyZp::constant(1, p);
    for j in 0..n {
        for i in 0..n {
            q[i][j] = current.coeff(i).value();
        }
        if j < n - 1 {
            current = current.mul_fast(&x_p);
            let (_, rem) = current.div_rem(f)?;
            current = rem;
        }
    }
    Ok(q)
}
/// Compute null space of (Q - I) over Z_p using Gaussian elimination
///
/// Returns basis vectors for the null space.
///
/// # Arguments
///
/// * `q` - Berlekamp matrix
/// * `p` - Prime modulus
///
/// # Returns
///
/// Vec of basis vectors, each represented as Vec<u64>
#[allow(clippy::needless_range_loop)]
fn null_space(q: &[Vec<u64>], p: u64) -> FiniteFieldResult<Vec<Vec<u64>>> {
    let n = q.len();
    if n == 0 {
        return Ok(vec![]);
    }
    let mut matrix = q.to_vec();
    for i in 0..n {
        let old_val = matrix[i][i];
        matrix[i][i] = if old_val == 0 {
            p - 1
        } else {
            (old_val + p - 1) % p
        };
    }
    let mut pivot_col = vec![None; n];
    let mut row = 0;
    for col in 0..n {
        let mut pivot_row = None;
        for r in row..n {
            if matrix[r][col] % p != 0 {
                pivot_row = Some(r);
                break;
            }
        }
        let Some(pr) = pivot_row else {
            continue;
        };
        if pr != row {
            matrix.swap(row, pr);
        }
        pivot_col[row] = Some(col);
        let pivot = Zp::new(matrix[row][col], p);
        let pivot_inv = pivot.inverse()?;
        for j in 0..n {
            let val = Zp::new(matrix[row][j], p);
            matrix[row][j] = (val * pivot_inv).value();
        }
        for r in 0..n {
            if r == row {
                continue;
            }
            let factor = Zp::new(matrix[r][col], p);
            if factor.is_zero() {
                continue;
            }
            for j in 0..n {
                let row_val = Zp::new(matrix[row][j], p);
                let current = Zp::new(matrix[r][j], p);
                matrix[r][j] = (current - factor * row_val).value();
            }
        }
        row += 1;
    }
    let mut free_vars = Vec::new();
    for col in 0..n {
        if !pivot_col.contains(&Some(col)) {
            free_vars.push(col);
        }
    }
    let mut basis = Vec::new();
    for &free_var in &free_vars {
        let mut vec = vec![0u64; n];
        vec[free_var] = 1;
        for r in 0..n {
            if let Some(pivot_c) = pivot_col[r] {
                vec[pivot_c] = if matrix[r][free_var] == 0 {
                    0
                } else {
                    p - matrix[r][free_var]
                };
            }
        }
        basis.push(vec);
    }
    Ok(basis)
}
/// Factor a square-free monic polynomial over Z_p using Berlekamp's algorithm
///
/// # Arguments
///
/// * `f` - Square-free monic polynomial over Z_p
///
/// # Returns
///
/// Vector of irreducible factors (all monic)
///
/// # Examples
///
/// ```rust
/// use mathhook_core::algebra::PolyZp;
/// use mathhook_core::core::polynomial::finite_field::berlekamp::berlekamp_factor;
///
/// // Factor x^2 - 1 = (x-1)(x+1) over Z_7
/// let f = PolyZp::from_signed_coeffs(&[-1, 0, 1], 7);
/// let factors = berlekamp_factor(&f).unwrap();
/// assert_eq!(factors.len(), 2);
/// ```
pub fn berlekamp_factor(f: &PolyZp) -> FiniteFieldResult<Vec<PolyZp>> {
    let n = match f.degree() {
        Some(d) => d,
        None => return Err(FiniteFieldError::EmptyPolynomial),
    };
    if n == 0 {
        return Ok(vec![f.clone()]);
    }
    if n == 1 {
        let monic = f.make_monic()?;
        return Ok(vec![monic]);
    }
    let p = f.modulus();
    let f_monic = f.make_monic()?;
    let q_matrix = berlekamp_matrix(&f_monic)?;
    let null_basis = null_space(&q_matrix, p)?;
    if null_basis.is_empty() {
        return Ok(vec![f_monic]);
    }
    let mut factors = vec![f_monic];
    for basis_vec in null_basis {
        let v = PolyZp::from_coeffs(basis_vec, p);
        let mut new_factors = Vec::new();
        for factor in factors {
            if factor.degree() == Some(1) {
                new_factors.push(factor);
                continue;
            }
            let mut found_split = false;
            for c in 0..p {
                let v_minus_c = v.sub(&PolyZp::constant(c, p));
                let g = factor.gcd(&v_minus_c)?;
                if !g.is_constant() && g.degree() != factor.degree() {
                    let (q, r) = factor.div_rem(&g)?;
                    if !r.is_zero() {
                        continue;
                    }
                    new_factors.push(g);
                    new_factors.push(q);
                    found_split = true;
                    break;
                }
            }
            if !found_split {
                new_factors.push(factor);
            }
        }
        factors = new_factors;
        if factors.iter().all(|f| f.degree() == Some(1)) {
            break;
        }
    }
    Ok(factors)
}
/// Factor a polynomial over `Z_p[x]` using Berlekamp's algorithm
///
/// # Arguments
///
/// * `poly` - Polynomial over Z_p
///
/// # Returns
///
/// Vector of irreducible factors over `Z_p[x]`
///
/// # Examples
///
/// ```rust
/// use mathhook_core::algebra::PolyZp;
/// use mathhook_core::core::polynomial::finite_field::berlekamp::factor_over_zp;
///
/// // Factor x^2 - 1 = (x-1)(x+1)
/// let f = PolyZp::from_signed_coeffs(&[-1, 0, 1], 7);
/// let factors = factor_over_zp(&f).unwrap();
/// assert_eq!(factors.len(), 2);
/// ```
pub fn factor_over_zp(poly: &PolyZp) -> FiniteFieldResult<Vec<PolyZp>> {
    let n = match poly.degree() {
        Some(d) => d,
        None => return Err(FiniteFieldError::EmptyPolynomial),
    };
    if n == 0 {
        return Ok(vec![poly.clone()]);
    }
    if n == 1 {
        let monic = poly.make_monic()?;
        return Ok(vec![monic]);
    }
    let poly_monic = poly.make_monic()?;
    berlekamp_factor(&poly_monic)
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_frobenius_mod() {
        let f = PolyZp::from_signed_coeffs(&[-1, 0, 1], 7);
        let x_p = frobenius_mod(&f, 7).unwrap();
        assert!(!x_p.is_zero());
    }
    #[test]
    fn test_berlekamp_matrix_linear() {
        let f = PolyZp::from_signed_coeffs(&[-1, 1], 7);
        let q = berlekamp_matrix(&f).unwrap();
        assert_eq!(q.len(), 1);
        assert_eq!(q[0].len(), 1);
    }
    #[test]
    fn test_null_space_trivial() {
        let p = 7;
        let identity = vec![vec![1]];
        let basis = null_space(&identity, p).unwrap();
        assert_eq!(basis.len(), 1);
    }
    #[test]
    fn test_berlekamp_factor_x_minus_one() {
        let f = PolyZp::from_signed_coeffs(&[-1, 1], 7);
        let factors = berlekamp_factor(&f).unwrap();
        assert_eq!(factors.len(), 1);
    }
    #[test]
    fn test_berlekamp_factor_x_squared_minus_one() {
        let f = PolyZp::from_signed_coeffs(&[-1, 0, 1], 7);
        let factors = berlekamp_factor(&f).unwrap();
        assert_eq!(factors.len(), 2);
        for factor in &factors {
            assert_eq!(factor.degree(), Some(1));
            assert_eq!(factor.leading_coeff().unwrap().value(), 1);
        }
    }
    #[test]
    fn test_berlekamp_factor_x_cubed_minus_x() {
        let f = PolyZp::from_signed_coeffs(&[0, -1, 0, 1], 7);
        let monic = f.make_monic().unwrap();
        let factors = berlekamp_factor(&monic).unwrap();
        assert!(factors.len() >= 2);
    }
    #[test]
    fn test_berlekamp_factor_x_to_fourth_minus_one() {
        let f = PolyZp::from_signed_coeffs(&[-1, 0, 0, 0, 1], 7);
        let factors = berlekamp_factor(&f).unwrap();
        assert!(factors.len() >= 2);
    }
    #[test]
    fn test_berlekamp_factor_x_to_sixth_minus_one() {
        let f = PolyZp::from_signed_coeffs(&[-1, 0, 0, 0, 0, 0, 1], 7);
        let factors = berlekamp_factor(&f).unwrap();
        assert!(factors.len() >= 2);
    }
    #[test]
    fn test_factor_over_zp_linear() {
        let f = PolyZp::from_signed_coeffs(&[-1, 1], 7);
        let factors = factor_over_zp(&f).unwrap();
        assert_eq!(factors.len(), 1);
    }
    #[test]
    fn test_factor_over_zp_quadratic() {
        let f = PolyZp::from_signed_coeffs(&[-1, 0, 1], 7);
        let factors = factor_over_zp(&f).unwrap();
        assert_eq!(factors.len(), 2);
    }
    #[test]
    fn test_factor_over_zp_cubic() {
        let f = PolyZp::from_signed_coeffs(&[0, -1, 0, 1], 7);
        let monic = f.make_monic().unwrap();
        let factors = factor_over_zp(&monic).unwrap();
        assert!(factors.len() >= 2);
    }
    #[test]
    fn test_factor_random_degree_10() {
        let coeffs: Vec<i64> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 1];
        let f = PolyZp::from_signed_coeffs(&coeffs, 11);
        let result = factor_over_zp(&f);
        assert!(result.is_ok());
    }
}
