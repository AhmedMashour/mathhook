//! Mathematical constant simplification
//!
//! Handles simplification of mathematical constants.
//! Implements constant arithmetic and special identities.

use crate::core::{Expression, MathConstant};

/// Simplify mathematical constants
pub fn simplify_constant(constant: &MathConstant) -> Expression {
    // Most constants remain as-is for now, but we can handle special cases
    match constant {
        MathConstant::Pi => Expression::constant(MathConstant::Pi),
        MathConstant::E => Expression::constant(MathConstant::E),
        MathConstant::I => Expression::constant(MathConstant::I),
        MathConstant::Infinity => Expression::constant(MathConstant::Infinity),
        MathConstant::NegativeInfinity => Expression::constant(MathConstant::NegativeInfinity),
        MathConstant::Undefined => Expression::constant(MathConstant::Undefined),
        _ => Expression::constant(*constant),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constant_simplification() {
        let pi = simplify_constant(&MathConstant::Pi);
        assert_eq!(pi, Expression::constant(MathConstant::Pi));

        let e = simplify_constant(&MathConstant::E);
        assert_eq!(e, Expression::constant(MathConstant::E));

        let i = simplify_constant(&MathConstant::I);
        assert_eq!(i, Expression::constant(MathConstant::I));
    }
}
