//! Tests for PolyZp
use super::PolyZp;
#[test]
fn test_poly_basic() {
    let p = PolyZp::from_coeffs(vec![3, 2, 1], 7);
    assert_eq!(p.degree(), Some(2));
    assert_eq!(p.leading_coeff().unwrap().value(), 1);
    assert_eq!(p.coeff(0).value(), 3);
    assert_eq!(p.coeff(1).value(), 2);
}
#[test]
fn test_poly_zero() {
    let zero = PolyZp::zero(7);
    assert!(zero.is_zero());
    assert_eq!(zero.degree(), None);
    let also_zero = PolyZp::from_coeffs(vec![0, 0, 0], 7);
    assert!(also_zero.is_zero());
}
#[test]
fn test_poly_evaluate() {
    let p = PolyZp::from_coeffs(vec![3, 2, 1], 7);
    assert_eq!(p.evaluate(0).value(), 3);
    assert_eq!(p.evaluate(1).value(), 6);
    assert_eq!(p.evaluate(2).value(), 4);
}
#[test]
fn test_poly_add() {
    let p1 = PolyZp::from_coeffs(vec![1, 2, 3], 7);
    let p2 = PolyZp::from_coeffs(vec![4, 5], 7);
    let sum = p1.add(&p2);
    assert_eq!(sum.coefficients(), &[5, 0, 3]);
}
#[test]
fn test_poly_sub() {
    let p1 = PolyZp::from_coeffs(vec![5, 3, 2], 7);
    let p2 = PolyZp::from_coeffs(vec![2, 1, 1], 7);
    let diff = p1.sub(&p2);
    assert_eq!(diff.coefficients(), &[3, 2, 1]);
}
#[test]
fn test_poly_mul() {
    let p1 = PolyZp::from_coeffs(vec![1, 1], 7);
    let p2 = PolyZp::from_coeffs(vec![1, 1], 7);
    let product = p1.mul(&p2);
    assert_eq!(product.coefficients(), &[1, 2, 1]);
}
#[test]
fn test_poly_mul_fast() {
    let p1 = PolyZp::from_coeffs(vec![1, 1], 7);
    let p2 = PolyZp::from_coeffs(vec![1, 1], 7);
    let product = p1.mul_fast(&p2);
    assert_eq!(product.coefficients(), &[1, 2, 1]);
    let p = crate::core::polynomial::finite_field::ntt::NTT_PRIME_1;
    let large1 = PolyZp::from_coeffs(vec![1; 100], p);
    let large2 = PolyZp::from_coeffs(vec![2; 100], p);
    let fast_product = large1.mul_fast(&large2);
    let naive_product = large1.mul(&large2);
    assert_eq!(fast_product.coefficients(), naive_product.coefficients());
}
#[test]
fn test_poly_div_rem() {
    let dividend = PolyZp::from_coeffs(vec![1, 2, 1], 7);
    let divisor = PolyZp::from_coeffs(vec![1, 1], 7);
    let (q, r) = dividend.div_rem(&divisor).unwrap();
    assert_eq!(q.coefficients(), &[1, 1]);
    assert!(r.is_zero());
}
#[test]
fn test_poly_make_monic() {
    let p = PolyZp::from_coeffs(vec![2, 4, 2], 7);
    let monic = p.make_monic().unwrap();
    assert_eq!(monic.leading_coeff().unwrap().value(), 1);
    assert_eq!(monic.coefficients(), &[1, 2, 1]);
}
#[test]
fn test_poly_x_minus_a() {
    let p = PolyZp::x_minus_a(3, 7);
    assert_eq!(p.evaluate(3).value(), 0);
    assert_eq!(p.evaluate(0).value(), 4);
}
#[test]
fn test_poly_shift() {
    let p = PolyZp::from_coeffs(vec![1, 2], 7);
    let shifted = p.shift(2);
    assert_eq!(shifted.coefficients(), &[0, 0, 1, 2]);
}
