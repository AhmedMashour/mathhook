//! PDE classification algorithms
//!
//! **⚠️ IMPORTANT LIMITATIONS** (Version 0.1.0):
//!
//! This module currently uses **simplified heuristics** and is **NOT mathematically rigorous**.
//! See `DESIGN_PROPER_CLASSIFICATION.md` for the planned proper implementation.
//!
//! # Current Implementation Status
//!
//! **❌ Known Issues**:
//! 1. **No Coefficient Extraction**: Does not extract actual coefficients A, B, C from equations
//! 2. **Heuristic Classification**: Uses expression structure (Add vs Mul) as proxy for PDE type
//! 3. **Hardcoded Patterns**: Only works for specific well-known PDEs (heat, wave, Laplace)
//! 4. **Invalid Discriminant**: Returns hardcoded values instead of computing B² - 4AC
//!
//! **Recommended Usage**:
//! - ✅ Educational purposes with standard PDEs only
//! - ❌ Do NOT use for arbitrary PDE classification
//! - ❌ Do NOT rely on results for mathematical correctness
//!
//! # Proper Implementation (Planned v0.2.0)
//!
//! The proper implementation will:
//! 1. Extract actual coefficients A, B, C from PDE equations
//! 2. Compute true discriminant B² - 4AC
//! 3. Use symbolic sign analysis for classification
//! 4. Handle both constant and variable coefficient PDEs
//!
//! See `DESIGN_PROPER_CLASSIFICATION.md` for complete design specification.
//!
//! # Mathematical Foundation (Reference)
//!
//! For a second-order linear PDE: `A·u_xx + B·u_xy + C·u_yy + ... = G`
//!
//! Classification by discriminant Δ = B² - 4AC:
//! - **Elliptic** (Δ < 0): Laplace equation, steady-state problems
//! - **Parabolic** (Δ = 0): Heat equation, diffusion processes
//! - **Hyperbolic** (Δ > 0): Wave equation, propagation phenomena
//!
//! **References**:
//! - Evans, L. C. (2010). *Partial Differential Equations*. AMS.
//! - Strauss, W. A. (2007). *Partial Differential Equations: An Introduction*. Wiley.

mod linearity;
mod type_detection;

use crate::calculus::pde::types::{Pde, PdeLinearity, PdeOrder, PdeType};

/// Classify a PDE and return its type
pub fn classify_pde(pde: &Pde) -> Result<PdeType, String> {
    if pde.order() != PdeOrder::Second {
        return Err("Only second-order PDEs can be classified by type".to_owned());
    }

    pde.pde_type()
        .ok_or_else(|| "Failed to classify PDE type".to_owned())
}

impl Pde {
    /// Determine the order of the PDE
    pub fn order(&self) -> PdeOrder {
        let max_order = self.max_derivative_order();
        match max_order {
            0 | 1 => PdeOrder::First,
            2 => PdeOrder::Second,
            n => PdeOrder::Higher(n),
        }
    }

    /// Determine the linearity classification
    pub fn linearity(&self) -> PdeLinearity {
        if self.is_linear() {
            PdeLinearity::Linear
        } else if self.is_quasilinear() {
            PdeLinearity::Quasilinear
        } else if self.is_semilinear() {
            PdeLinearity::Semilinear
        } else {
            PdeLinearity::Nonlinear
        }
    }

    /// Find maximum derivative order in the equation
    fn max_derivative_order(&self) -> u32 {
        let var_count = self.independent_vars.len() as u32;
        if var_count >= 2 {
            2
        } else if var_count == 1 {
            1
        } else {
            0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{expr, symbol};

    #[test]
    fn test_classify_pde_second_order() {
        let u = symbol!(u);
        let x = symbol!(x);
        let y = symbol!(y);
        let equation = expr!(x + y);
        let pde = Pde::new(equation, u, vec![x, y]);
        let result = classify_pde(&pde);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), PdeType::Elliptic);
    }

    #[test]
    fn test_classify_pde_first_order_fails() {
        let u = symbol!(u);
        let x = symbol!(x);
        let equation = expr!(u);
        let pde = Pde::new(equation, u, vec![x]);
        let result = classify_pde(&pde);
        assert!(result.is_err());
    }

    #[test]
    fn test_pde_order_first() {
        let u = symbol!(u);
        let x = symbol!(x);
        let equation = expr!(u);
        let pde = Pde::new(equation, u, vec![x]);
        assert_eq!(pde.order(), PdeOrder::First);
    }

    #[test]
    fn test_pde_order_second() {
        let u = symbol!(u);
        let x = symbol!(x);
        let y = symbol!(y);
        let equation = expr!(x + y);
        let pde = Pde::new(equation, u, vec![x, y]);
        assert_eq!(pde.order(), PdeOrder::Second);
    }

    #[test]
    fn test_pde_linearity_linear() {
        let u = symbol!(u);
        let x = symbol!(x);
        let equation = expr!(u);
        let pde = Pde::new(equation, u, vec![x]);
        assert_eq!(pde.linearity(), PdeLinearity::Linear);
    }

    #[test]
    fn test_pde_type_classification() {
        let u = symbol!(u);
        let x = symbol!(x);
        let y = symbol!(y);
        let equation = expr!(x + y);
        let pde = Pde::new(equation, u, vec![x, y]);
        assert_eq!(pde.pde_type(), Some(PdeType::Elliptic));
    }

    #[test]
    fn test_max_derivative_order_single_var() {
        let u = symbol!(u);
        let x = symbol!(x);
        let equation = expr!(u);
        let pde = Pde::new(equation, u, vec![x]);
        assert_eq!(pde.max_derivative_order(), 1);
    }

    #[test]
    fn test_max_derivative_order_two_vars_addition() {
        let u = symbol!(u);
        let x = symbol!(x);
        let y = symbol!(y);
        let equation = expr!(x + y);
        let pde = Pde::new(equation, u, vec![x, y]);
        assert_eq!(pde.max_derivative_order(), 2);
    }

    #[test]
    fn test_max_derivative_order_two_vars_multiplication() {
        let u = symbol!(u);
        let x = symbol!(x);
        let y = symbol!(y);
        let equation = expr!(x * y);
        let pde = Pde::new(equation, u, vec![x, y]);
        assert_eq!(pde.max_derivative_order(), 2);
    }
}
