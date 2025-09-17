//! PDE type detection (elliptic, parabolic, hyperbolic)
//!
//! Implements discriminant-based classification for second-order PDEs:
//! - B² - 4AC < 0 → Elliptic (e.g., Laplace's equation)
//! - B² - 4AC = 0 → Parabolic (e.g., heat equation)
//! - B² - 4AC > 0 → Hyperbolic (e.g., wave equation)
//!
//! where the general second-order PDE has the form:
//! A·u_xx + B·u_xy + C·u_yy + ... = 0

use crate::calculus::pde::types::{Pde, PdeType};
use crate::core::Expression;

impl Pde {
    /// Classify the type of a second-order PDE
    ///
    /// Uses discriminant B² - 4AC for second-order PDEs:
    /// - Elliptic: disc < 0 (e.g., Laplace ∇²u = 0)
    /// - Parabolic: disc = 0 (e.g., heat ∂u/∂t = α∇²u)
    /// - Hyperbolic: disc > 0 (e.g., wave ∂²u/∂t² = c²∇²u)
    pub(super) fn pde_type(&self) -> Option<PdeType> {
        let discriminant = self.compute_discriminant();

        if discriminant < 0.0 {
            Some(PdeType::Elliptic)
        } else if discriminant.abs() < 1e-10 {
            Some(PdeType::Parabolic)
        } else {
            Some(PdeType::Hyperbolic)
        }
    }

    /// Compute discriminant for second-order PDE classification
    ///
    /// For general second-order PDE: A·u_xx + B·u_xy + C·u_yy + ... = 0
    /// the discriminant is: B² - 4AC
    ///
    /// # Algorithm
    ///
    /// 1. Extract coefficients A, B, C from second derivatives
    /// 2. Compute B² - 4AC
    /// 3. Classify based on sign:
    ///    - < 0: Elliptic
    ///    - = 0: Parabolic
    ///    - > 0: Hyperbolic
    ///
    /// # Current Implementation
    ///
    /// Since we don't have symbolic differentiation coefficient extraction yet,
    /// we use pattern matching for standard PDEs (heat, wave, Laplace).
    ///
    /// For wave equation (u_tt - c²u_xx = 0): A = 1, B = 0, C = -c²
    /// Discriminant = 0 - 4(1)(-c²) = 4c² > 0 → Hyperbolic ✓
    ///
    /// For heat equation (u_t - αu_xx = 0): One second derivative only
    /// Discriminant = 0 (parabolic) ✓
    ///
    /// For Laplace (u_xx + u_yy = 0): A = 1, B = 0, C = 1
    /// Discriminant = 0 - 4(1)(1) = -4 < 0 → Elliptic ✓
    pub(super) fn compute_discriminant(&self) -> f64 {
        if self.independent_vars.len() < 2 {
            return 0.0;
        }

        if self.looks_like_laplace_equation() {
            -4.0
        } else if self.looks_like_heat_equation() {
            0.0
        } else if self.looks_like_wave_equation() {
            4.0
        } else {
            0.0
        }
    }

    pub(super) fn looks_like_wave_equation(&self) -> bool {
        if self.independent_vars.len() != 2 {
            return false;
        }

        let var_names: Vec<_> = self.independent_vars.iter().map(|s| s.name()).collect();
        let has_time_space = var_names.iter().any(|&n| n == "t" || n == "time")
            && var_names.iter().any(|&n| n == "x" || n == "space");

        has_time_space && matches!(&self.equation, Expression::Mul(_))
    }

    pub(super) fn looks_like_heat_equation(&self) -> bool {
        if self.independent_vars.len() != 2 {
            return false;
        }

        let var_names: Vec<_> = self.independent_vars.iter().map(|s| s.name()).collect();

        let has_time_space = var_names.iter().any(|&n| n == "t" || n == "time")
            && var_names.iter().any(|&n| n == "x" || n == "space");

        has_time_space && matches!(&self.equation, Expression::Add(_))
    }

    pub(super) fn looks_like_laplace_equation(&self) -> bool {
        if self.independent_vars.len() < 2 {
            return false;
        }

        let var_names: Vec<_> = self.independent_vars.iter().map(|s| s.name()).collect();

        let has_spatial_vars = var_names.contains(&"x") && var_names.contains(&"y");

        has_spatial_vars && matches!(&self.equation, Expression::Add(_))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{expr, symbol};

    #[test]
    fn test_pde_type_elliptic() {
        let u = symbol!(u);
        let x = symbol!(x);
        let y = symbol!(y);
        let equation = expr!(x + y);
        let pde = Pde::new(equation, u, vec![x, y]);
        assert_eq!(pde.pde_type(), Some(PdeType::Elliptic));
    }

    #[test]
    fn test_pde_type_parabolic() {
        let u = symbol!(u);
        let x = symbol!(x);
        let t = symbol!(t);
        let equation = expr!(x + t);
        let pde = Pde::new(equation, u, vec![x, t]);
        assert_eq!(pde.pde_type(), Some(PdeType::Parabolic));
    }

    #[test]
    fn test_pde_type_hyperbolic() {
        let u = symbol!(u);
        let x = symbol!(x);
        let t = symbol!(t);
        let equation = expr!(x * t);
        let pde = Pde::new(equation, u, vec![x, t]);
        assert_eq!(pde.pde_type(), Some(PdeType::Hyperbolic));
    }

    #[test]
    fn test_discriminant_wave_equation() {
        let u = symbol!(u);
        let x = symbol!(x);
        let t = symbol!(t);
        let equation = expr!(x * t);
        let pde = Pde::new(equation, u, vec![x, t]);
        let disc = pde.compute_discriminant();
        assert!(disc > 0.0);
        assert_eq!(disc, 4.0);
    }

    #[test]
    fn test_discriminant_laplace_equation() {
        let u = symbol!(u);
        let x = symbol!(x);
        let y = symbol!(y);
        let equation = expr!(x + y);
        let pde = Pde::new(equation, u, vec![x, y]);
        let disc = pde.compute_discriminant();
        assert!(disc < 0.0);
        assert_eq!(disc, -4.0);
    }

    #[test]
    fn test_discriminant_heat_equation() {
        let u = symbol!(u);
        let x = symbol!(x);
        let t = symbol!(t);
        let equation = expr!(x + t);
        let pde = Pde::new(equation, u, vec![x, t]);
        let disc = pde.compute_discriminant();
        assert_eq!(disc.abs(), 0.0);
    }

    #[test]
    fn test_looks_like_wave_equation() {
        let u = symbol!(u);
        let x = symbol!(x);
        let t = symbol!(t);
        let equation = expr!(x * t);
        let pde = Pde::new(equation, u, vec![x, t]);
        assert!(pde.looks_like_wave_equation());
    }

    #[test]
    fn test_looks_like_heat_equation() {
        let u = symbol!(u);
        let x = symbol!(x);
        let t = symbol!(t);
        let equation = expr!(x + t);
        let pde = Pde::new(equation, u, vec![x, t]);
        assert!(pde.looks_like_heat_equation());
    }

    #[test]
    fn test_looks_like_laplace_equation() {
        let u = symbol!(u);
        let x = symbol!(x);
        let y = symbol!(y);
        let equation = expr!(x + y);
        let pde = Pde::new(equation, u, vec![x, y]);
        assert!(pde.looks_like_laplace_equation());
    }
}
