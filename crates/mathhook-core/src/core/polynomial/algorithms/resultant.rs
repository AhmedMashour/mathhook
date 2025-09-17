//! Polynomial Resultant Algorithms
//!
//! MOVED TO ALGEBRA LAYER
//!
//! All resultant and discriminant operations are Expression-based
//! and belong in the algebra layer, not the pure polynomial algorithms layer.
//!
//! Use `crate::algebra::polynomial_advanced::AdvancedPolynomial` instead:
//! - `polynomial_resultant()`
//! - `polynomial_discriminant()`
//!
//! This file remains only for backward compatibility exports.

pub use crate::algebra::polynomial_advanced::AdvancedPolynomial;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::Expression;
    use crate::symbol;

    #[test]
    fn test_polynomial_resultant_moved_to_algebra() {
        let x = symbol!(x);
        let p1 = Expression::add(vec![Expression::symbol(x.clone()), Expression::integer(-1)]);
        let p2 = Expression::add(vec![Expression::symbol(x.clone()), Expression::integer(-2)]);

        let result = p1.polynomial_resultant(&p2, &x);
        assert!(!result.is_zero());
    }

    #[test]
    fn test_polynomial_discriminant_moved_to_algebra() {
        let x = symbol!(x);
        let poly = Expression::add(vec![
            Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
            Expression::mul(vec![Expression::integer(-2), Expression::symbol(x.clone())]),
            Expression::integer(1),
        ]);

        let result = poly.polynomial_discriminant(&x);
        assert!(!result.is_zero());
    }
}
