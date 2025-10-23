//! Type definitions for partial differential equations
//!
//! This module provides the core type system for representing PDEs, boundary conditions,
//! and initial conditions.

use crate::core::Expression;
use crate::core::Symbol;

/// Represents a partial differential equation
#[derive(Debug, Clone, PartialEq)]
pub struct Pde {
    /// The PDE expression (should equal zero)
    pub equation: Expression,
    /// The dependent variable (e.g., u in u(x,t))
    pub dependent_var: Symbol,
    /// The independent variables (e.g., x, t)
    pub independent_vars: Vec<Symbol>,
}

/// Types of boundary conditions
#[derive(Debug, Clone, PartialEq)]
pub enum BoundaryCondition {
    /// Dirichlet: u = f on boundary
    Dirichlet {
        /// The function value on the boundary
        value: Expression,
        /// The location/surface where this applies
        location: BoundaryLocation,
    },
    /// Neumann: ∂u/∂n = f on boundary
    Neumann {
        /// The normal derivative value on the boundary
        derivative: Expression,
        /// The location/surface where this applies
        location: BoundaryLocation,
    },
    /// Robin: au + b∂u/∂n = f on boundary
    Robin {
        /// Coefficient of u
        coeff_u: Expression,
        /// Coefficient of ∂u/∂n
        coeff_du: Expression,
        /// Right-hand side value
        value: Expression,
        /// The location/surface where this applies
        location: BoundaryLocation,
    },
}

/// Specifies where a boundary condition applies
#[derive(Debug, Clone, PartialEq)]
pub enum BoundaryLocation {
    /// At a specific point
    Point(Vec<Expression>),
    /// On a curve (1D boundary in 2D space)
    Curve {
        /// Parameter variable
        parameter: Symbol,
        /// Parametric equations
        equations: Vec<Expression>,
    },
    /// On a surface (2D boundary in 3D space)
    Surface {
        /// Parameter variables
        parameters: Vec<Symbol>,
        /// Parametric equations
        equations: Vec<Expression>,
    },
    /// Simple boundary: var = value
    Simple {
        /// The variable
        variable: Symbol,
        /// The value
        value: Expression,
    },
}

/// Initial conditions for time-dependent PDEs
#[derive(Debug, Clone, PartialEq)]
pub enum InitialCondition {
    /// Value at initial time: u(x,0) = f(x)
    Value {
        /// The initial value function
        function: Expression,
    },
    /// Derivative at initial time: ∂u/∂t(x,0) = f(x)
    Derivative {
        /// The time derivative at t=0
        function: Expression,
    },
}

/// Classification of PDE order
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PdeOrder {
    /// First-order PDE
    First,
    /// Second-order PDE
    Second,
    /// Higher-order PDE
    Higher(u32),
}

/// Classification of PDE linearity
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PdeLinearity {
    /// Linear PDE
    Linear,
    /// Quasilinear PDE
    Quasilinear,
    /// Semilinear PDE
    Semilinear,
    /// Fully nonlinear PDE
    Nonlinear,
}

/// Classification of second-order PDE type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PdeType {
    /// Elliptic (e.g., Laplace equation)
    Elliptic,
    /// Parabolic (e.g., heat equation)
    Parabolic,
    /// Hyperbolic (e.g., wave equation)
    Hyperbolic,
}

impl Pde {
    /// Create a new PDE
    ///
    /// # Arguments
    ///
    /// * `equation` - The PDE expression (should equal zero)
    /// * `dependent_var` - The dependent variable
    /// * `independent_vars` - The independent variables
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::pde::types::Pde;
    /// use mathhook_core::{symbol, expr};
    ///
    /// let u = symbol!(u);
    /// let x = symbol!(x);
    /// let t = symbol!(t);
    /// let equation = expr!(add: u, x, t);
    /// let pde = Pde::new(equation, u, vec![x, t]);
    /// ```
    pub fn new(
        equation: Expression,
        dependent_var: Symbol,
        independent_vars: Vec<Symbol>,
    ) -> Self {
        Self {
            equation,
            dependent_var,
            independent_vars,
        }
    }
}

impl BoundaryCondition {
    /// Create a Dirichlet boundary condition
    pub fn dirichlet(value: Expression, location: BoundaryLocation) -> Self {
        Self::Dirichlet { value, location }
    }

    /// Create a Neumann boundary condition
    pub fn neumann(derivative: Expression, location: BoundaryLocation) -> Self {
        Self::Neumann {
            derivative,
            location,
        }
    }

    /// Create a Robin boundary condition
    pub fn robin(
        coeff_u: Expression,
        coeff_du: Expression,
        value: Expression,
        location: BoundaryLocation,
    ) -> Self {
        Self::Robin {
            coeff_u,
            coeff_du,
            value,
            location,
        }
    }
}

impl InitialCondition {
    /// Create an initial value condition
    pub fn value(function: Expression) -> Self {
        Self::Value { function }
    }

    /// Create an initial derivative condition
    pub fn derivative(function: Expression) -> Self {
        Self::Derivative { function }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{expr, symbol};

    #[test]
    fn test_pde_creation() {
        let u = symbol!(u);
        let x = symbol!(x);
        let t = symbol!(t);
        let equation = expr!(add: u, x, t);
        let pde = Pde::new(equation.clone(), u.clone(), vec![x.clone(), t.clone()]);

        assert_eq!(pde.equation, equation);
        assert_eq!(pde.dependent_var, u);
        assert_eq!(pde.independent_vars, vec![x, t]);
    }

    #[test]
    fn test_boundary_condition_dirichlet() {
        let x = symbol!(x);
        let value = expr!(0);
        let location = BoundaryLocation::Simple {
            variable: x.clone(),
            value: expr!(0),
        };
        let bc = BoundaryCondition::dirichlet(value.clone(), location.clone());

        match bc {
            BoundaryCondition::Dirichlet { value: v, location: l } => {
                assert_eq!(v, value);
                assert_eq!(l, location);
            }
            _ => panic!("Expected Dirichlet boundary condition"),
        }
    }

    #[test]
    fn test_initial_condition_value() {
        let x = symbol!(x);
        let function = expr!(x);
        let ic = InitialCondition::value(function.clone());

        match ic {
            InitialCondition::Value { function: f } => {
                assert_eq!(f, function);
            }
            _ => panic!("Expected value initial condition"),
        }
    }
}
