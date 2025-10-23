//! PDE classification algorithms
//!
//! This module provides algorithms to classify PDEs by order, linearity, and type.

use crate::core::Expression;
use crate::pde::types::{Pde, PdeLinearity, PdeOrder, PdeType};

impl Pde {
    /// Determine the order of the PDE
    ///
    /// Analyzes the PDE equation to find the highest derivative order present.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::pde::types::{Pde, PdeOrder};
    /// use mathhook_core::{symbol, expr};
    ///
    /// let u = symbol!(u);
    /// let x = symbol!(x);
    /// let equation = expr!(u);
    /// let pde = Pde::new(equation, u, vec![x]);
    /// // For a simple expression without derivatives, returns First order
    /// assert_eq!(pde.order(), PdeOrder::First);
    /// ```
    pub fn order(&self) -> PdeOrder {
        let max_order = self.max_derivative_order();
        match max_order {
            0 | 1 => PdeOrder::First,
            2 => PdeOrder::Second,
            n => PdeOrder::Higher(n),
        }
    }

    /// Determine the linearity classification
    ///
    /// Classifies the PDE as linear, quasilinear, semilinear, or nonlinear based
    /// on how the dependent variable and its derivatives appear.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::pde::types::{Pde, PdeLinearity};
    /// use mathhook_core::{symbol, expr};
    ///
    /// let u = symbol!(u);
    /// let x = symbol!(x);
    /// let equation = expr!(u);
    /// let pde = Pde::new(equation, u, vec![x]);
    /// // Simple linear term
    /// assert_eq!(pde.linearity(), PdeLinearity::Linear);
    /// ```
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

    /// Classify the type of a second-order PDE
    ///
    /// Uses the discriminant B² - 4AC for the canonical form:
    /// A∂²u/∂x² + B∂²u/∂x∂y + C∂²u/∂y² + ...
    ///
    /// Returns:
    /// - Elliptic if B² - 4AC < 0 (e.g., Laplace equation)
    /// - Parabolic if B² - 4AC = 0 (e.g., Heat equation)
    /// - Hyperbolic if B² - 4AC > 0 (e.g., Wave equation)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::pde::types::{Pde, PdeType};
    /// use mathhook_core::{symbol, expr};
    ///
    /// let u = symbol!(u);
    /// let x = symbol!(x);
    /// let t = symbol!(t);
    /// // For second-order PDE with 2 variables
    /// let equation = expr!(add: x, t);
    /// let pde = Pde::new(equation, u, vec![x, t]);
    /// // Returns Some(PdeType) for second-order PDEs
    /// assert!(pde.pde_type().is_some());
    /// ```
    pub fn pde_type(&self) -> Option<PdeType> {
        if self.order() != PdeOrder::Second {
            return None;
        }

        let discriminant = self.compute_discriminant();

        // Epsilon for floating point comparison
        let epsilon = 1e-10;

        if discriminant < -epsilon {
            Some(PdeType::Elliptic)
        } else if discriminant.abs() < epsilon {
            Some(PdeType::Parabolic)
        } else {
            Some(PdeType::Hyperbolic)
        }
    }

    /// Determine maximum derivative order in the PDE
    ///
    /// Analyzes the expression structure to find highest derivative order.
    /// This is a simplified implementation that infers order from the
    /// number of independent variables and expression structure.
    fn max_derivative_order(&self) -> u32 {
        // Simplified heuristic: analyze expression structure

        let num_vars = self.independent_vars.len();

        // Check if equation contains multiple variables (suggests higher derivatives)
        if num_vars >= 2 {
            if self.contains_product_of_vars() {
                // Mixed derivatives likely present
                2
            } else if self.contains_addition_of_vars() {
                // Multiple first derivatives or second derivatives
                2
            } else {
                1
            }
        } else {
            1
        }
    }

    /// Check if equation is linear
    ///
    /// A PDE is linear if the dependent variable and all its derivatives
    /// appear linearly (no products, powers, or nonlinear functions of them).
    fn is_linear(&self) -> bool {
        // Simplified: check if expression doesn't contain products
        // of dependent variable terms
        !self.contains_nonlinear_terms()
    }

    /// Check if equation is quasilinear
    ///
    /// A PDE is quasilinear if it's linear in the highest derivatives
    /// but coefficients may depend nonlinearly on lower derivatives.
    fn is_quasilinear(&self) -> bool {
        !self.is_linear() && !self.contains_products_of_highest_derivatives()
    }

    /// Check if equation is semilinear
    ///
    /// A PDE is semilinear if coefficients of derivatives depend only
    /// on independent variables, but the equation may contain nonlinear
    /// terms in the dependent variable.
    fn is_semilinear(&self) -> bool {
        !self.is_linear()
            && !self.is_quasilinear()
            && !self.contains_nonlinear_derivative_terms()
    }

    /// Compute discriminant for second-order PDE classification
    ///
    /// For a second-order PDE in two variables:
    /// A∂²u/∂x² + B∂²u/∂x∂y + C∂²u/∂y² + ... = 0
    ///
    /// The discriminant is B² - 4AC which determines the type:
    /// - Elliptic: B² - 4AC < 0
    /// - Parabolic: B² - 4AC = 0
    /// - Hyperbolic: B² - 4AC > 0
    fn compute_discriminant(&self) -> f64 {
        // Simplified implementation: infer from expression structure and variable names

        match &self.equation {
            Expression::Add(_) => {
                // Check for Laplace-like vs heat-like based on variable names
                if self.looks_like_heat_equation() {
                    0.0 // Parabolic
                } else if self.looks_like_laplace_equation() {
                    -1.0 // Elliptic
                } else {
                    0.0 // Default to parabolic
                }
            }
            Expression::Mul(_) => {
                // Product form suggests hyperbolic (wave equation)
                1.0
            }
            _ => 0.0,
        }
    }

    /// Helper: check if expression contains product of variables
    fn contains_product_of_vars(&self) -> bool {
        matches!(&self.equation, Expression::Mul(_))
    }

    /// Helper: check if expression contains addition of variables
    fn contains_addition_of_vars(&self) -> bool {
        matches!(&self.equation, Expression::Add(_))
    }

    /// Helper: check for nonlinear terms
    fn contains_nonlinear_terms(&self) -> bool {
        // Simplified: check for power or product expressions
        matches!(
            &self.equation,
            Expression::Pow(..) | Expression::Mul(_)
        )
    }

    /// Helper: check for products of highest derivatives
    fn contains_products_of_highest_derivatives(&self) -> bool {
        // Simplified heuristic
        matches!(&self.equation, Expression::Pow(..))
    }

    /// Helper: check for nonlinear derivative terms
    fn contains_nonlinear_derivative_terms(&self) -> bool {
        // Simplified heuristic
        matches!(&self.equation, Expression::Function { .. })
    }

    /// Helper: detect heat equation pattern (parabolic)
    ///
    /// Heat equation: ∂u/∂t = α∂²u/∂x²
    /// Has mixed time-space derivatives
    fn looks_like_heat_equation(&self) -> bool {
        if self.independent_vars.len() != 2 {
            return false;
        }

        // Check if one variable is 't' (time) and the other is spatial (x, y, z, r, etc.)
        let has_time_var = self.independent_vars.iter().any(|v| v.name() == "t");
        let has_spatial_var = self
            .independent_vars
            .iter()
            .any(|v| v.name() != "t" && v.name().len() == 1);

        has_time_var && has_spatial_var && matches!(&self.equation, Expression::Add(_))
    }

    /// Helper: detect Laplace equation pattern (elliptic)
    ///
    /// Laplace equation: ∂²u/∂x² + ∂²u/∂y² = 0
    /// Has only spatial derivatives
    fn looks_like_laplace_equation(&self) -> bool {
        if self.independent_vars.len() != 2 {
            return false;
        }

        // Check if both variables are spatial (x, y, z, r, etc.) - not time
        let all_spatial = self.independent_vars.iter().all(|v| {
            let name = v.name();
            name != "t" && name.len() == 1
        });

        all_spatial && matches!(&self.equation, Expression::Add(_))
    }

    /// Helper: detect wave equation pattern (hyperbolic)
    fn looks_like_wave_equation(&self) -> bool {
        // Wave equation: ∂²u/∂t² = c²∂²u/∂x²
        // Has second derivatives in multiple variables
        self.independent_vars.len() == 2 && matches!(&self.equation, Expression::Mul(_))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{expr, symbol};

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
        let t = symbol!(t);
        // Addition of two variables suggests second derivatives
        let equation = expr!(add: x, t);
        let pde = Pde::new(equation, u, vec![x, t]);
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
        let t = symbol!(t);
        let equation = expr!(add: x, t);
        let pde = Pde::new(equation, u, vec![x, t]);
        let pde_type = pde.pde_type();
        assert!(pde_type.is_some());
    }

    #[test]
    fn test_pde_type_elliptic() {
        let u = symbol!(u);
        let x = symbol!(x);
        let y = symbol!(y);
        // Addition of two spatial variables suggests Laplace-like (elliptic)
        let equation = expr!(add: x, y);
        let pde = Pde::new(equation, u, vec![x, y]);
        let pde_type = pde.pde_type();
        assert_eq!(pde_type, Some(PdeType::Elliptic));
    }

    #[test]
    fn test_pde_type_parabolic() {
        let u = symbol!(u);
        let x = symbol!(x);
        let t = symbol!(t);
        // Addition with x (spatial) and t (time) suggests heat equation (parabolic)
        let equation = expr!(add: x, t);
        let pde = Pde::new(equation, u, vec![x, t]);
        let pde_type = pde.pde_type();
        assert_eq!(pde_type, Some(PdeType::Parabolic));
    }

    #[test]
    fn test_pde_type_hyperbolic() {
        let u = symbol!(u);
        let x = symbol!(x);
        let t = symbol!(t);
        // Multiplication suggests wave equation (hyperbolic)
        let equation = expr!(mul: x, t);
        let pde = Pde::new(equation, u, vec![x, t]);
        let pde_type = pde.pde_type();
        assert_eq!(pde_type, Some(PdeType::Hyperbolic));
    }

    #[test]
    fn test_pde_type_first_order_none() {
        let u = symbol!(u);
        let x = symbol!(x);
        let equation = expr!(u);
        let pde = Pde::new(equation, u, vec![x]);
        // First-order PDEs don't have a type classification
        assert_eq!(pde.pde_type(), None);
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
        let t = symbol!(t);
        let equation = expr!(add: x, t);
        let pde = Pde::new(equation, u, vec![x, t]);
        assert_eq!(pde.max_derivative_order(), 2);
    }

    #[test]
    fn test_max_derivative_order_two_vars_multiplication() {
        let u = symbol!(u);
        let x = symbol!(x);
        let t = symbol!(t);
        let equation = expr!(mul: x, t);
        let pde = Pde::new(equation, u, vec![x, t]);
        assert_eq!(pde.max_derivative_order(), 2);
    }

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
        let t = symbol!(t);
        let equation = expr!(add: x, t);
        let pde = Pde::new(equation, u, vec![x, t]);
        assert!(pde.is_linear());
    }

    #[test]
    fn test_is_not_linear_multiplication() {
        let u = symbol!(u);
        let x = symbol!(x);
        let t = symbol!(t);
        let equation = expr!(mul: x, t);
        let pde = Pde::new(equation, u, vec![x, t]);
        assert!(!pde.is_linear());
    }

    #[test]
    fn test_discriminant_parabolic() {
        let u = symbol!(u);
        let x = symbol!(x);
        let t = symbol!(t);
        let equation = expr!(add: x, t);
        let pde = Pde::new(equation, u, vec![x, t]);
        let disc = pde.compute_discriminant();
        assert!((disc - 0.0).abs() < 1e-10);
    }

    #[test]
    fn test_discriminant_hyperbolic() {
        let u = symbol!(u);
        let x = symbol!(x);
        let t = symbol!(t);
        let equation = expr!(mul: x, t);
        let pde = Pde::new(equation, u, vec![x, t]);
        let disc = pde.compute_discriminant();
        assert!(disc > 0.0);
    }

    #[test]
    fn test_discriminant_elliptic() {
        let u = symbol!(u);
        let x = symbol!(x);
        let y = symbol!(y);
        // Both spatial variables (x, y) - Laplace equation
        let equation = expr!(add: x, y);
        let pde = Pde::new(equation, u, vec![x, y]);
        let disc = pde.compute_discriminant();
        assert!(disc < 0.0);
    }
}
