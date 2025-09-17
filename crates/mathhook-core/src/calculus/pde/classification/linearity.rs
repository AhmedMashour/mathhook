//! Linearity detection for PDEs

use crate::calculus::pde::types::Pde;
use crate::core::Expression;

impl Pde {
    /// Check if equation is linear
    pub(super) fn is_linear(&self) -> bool {
        !self.contains_nonlinear_terms()
    }

    /// Check if equation is quasilinear
    pub(super) fn is_quasilinear(&self) -> bool {
        !self.is_linear() && !self.contains_products_of_highest_derivatives()
    }

    /// Check if equation is semilinear
    pub(super) fn is_semilinear(&self) -> bool {
        !self.is_linear() && !self.is_quasilinear() && !self.contains_nonlinear_derivative_terms()
    }

    /// Helper: check for nonlinear terms
    pub(super) fn contains_nonlinear_terms(&self) -> bool {
        matches!(&self.equation, Expression::Pow(..) | Expression::Mul(_))
    }

    /// Helper: check for products of highest derivatives
    pub(super) fn contains_products_of_highest_derivatives(&self) -> bool {
        matches!(&self.equation, Expression::Mul(_))
    }

    /// Helper: check for nonlinear derivative terms
    pub(super) fn contains_nonlinear_derivative_terms(&self) -> bool {
        matches!(&self.equation, Expression::Pow(..) | Expression::Mul(_))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{expr, symbol};

    #[test]
    fn test_is_linear_simple() {
        let u = symbol!(u);
        let x = symbol!(x);
        let equation = expr!(u);
        let pde = Pde::new(equation, u, vec![x]);
        assert!(pde.is_linear());
    }

    #[test]
    fn test_is_linear_addition() {
        let u = symbol!(u);
        let x = symbol!(x);
        let y = symbol!(y);
        let equation = expr!(x + y);
        let pde = Pde::new(equation, u, vec![x, y]);
        assert!(pde.is_linear());
    }

    #[test]
    fn test_is_nonlinear_multiplication() {
        let u = symbol!(u);
        let x = symbol!(x);
        let y = symbol!(y);
        let equation = expr!(x * y);
        let pde = Pde::new(equation, u, vec![x, y]);
        assert!(!pde.is_linear());
    }
}
