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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PdeOrder {
    /// First-order PDE
    First,
    /// Second-order PDE
    Second,
    /// Higher-order PDE
    Higher(u32),
}

/// Classification of PDE linearity
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PdeType {
    /// Elliptic (e.g., Laplace equation)
    Elliptic,
    /// Parabolic (e.g., heat equation)
    Parabolic,
    /// Hyperbolic (e.g., wave equation)
    Hyperbolic,
}

/// Unified PDE solution type that can represent any PDE solution.
///
/// This type wraps the general solution expression along with solver-specific
/// metadata (eigenvalues, coefficients, parameters).
#[derive(Debug, Clone, PartialEq)]
pub struct PDESolution {
    /// The general solution expression
    pub solution: Expression,

    /// Solver-specific metadata
    pub metadata: SolutionMetadata,
}

/// Metadata specific to the PDE solver that produced the solution
#[derive(Debug, Clone, PartialEq)]
pub enum SolutionMetadata {
    /// Heat equation solution metadata
    Heat {
        alpha: Expression,
        eigenvalues: Vec<Expression>,
        coefficients: Vec<Expression>,
    },
    /// Wave equation solution metadata
    Wave {
        c: Expression,
        eigenvalues: Vec<Expression>,
        coefficients: Vec<Expression>,
    },
    /// Laplace equation solution metadata
    Laplace {
        eigenvalues: Vec<Expression>,
        coefficients: Vec<Expression>,
    },
    /// General PDE solution (no specific metadata)
    General,
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
    /// use mathhook_core::calculus::pde::types::Pde;
    /// use mathhook_core::{symbol, expr};
    ///
    /// let u = symbol!(u);
    /// let x = symbol!(x);
    /// let t = symbol!(t);
    /// let equation = expr!(u + x + t);
    /// let pde = Pde::new(equation, u, vec![x, t]);
    /// ```
    pub fn new(equation: Expression, dependent_var: Symbol, independent_vars: Vec<Symbol>) -> Self {
        Self {
            equation,
            dependent_var,
            independent_vars,
        }
    }
}

impl PDESolution {
    /// Creates a heat equation solution
    ///
    /// # Arguments
    ///
    /// * `solution` - The general solution expression
    /// * `alpha` - Thermal diffusivity coefficient
    /// * `eigenvalues` - Eigenvalues from boundary conditions
    /// * `coefficients` - Fourier coefficients
    pub fn heat(
        solution: Expression,
        alpha: Expression,
        eigenvalues: Vec<Expression>,
        coefficients: Vec<Expression>,
    ) -> Self {
        Self {
            solution,
            metadata: SolutionMetadata::Heat {
                alpha,
                eigenvalues,
                coefficients,
            },
        }
    }

    /// Creates a wave equation solution
    ///
    /// # Arguments
    ///
    /// * `solution` - The general solution expression
    /// * `c` - Wave speed
    /// * `eigenvalues` - Eigenvalues from boundary conditions
    /// * `coefficients` - Fourier coefficients
    pub fn wave(
        solution: Expression,
        c: Expression,
        eigenvalues: Vec<Expression>,
        coefficients: Vec<Expression>,
    ) -> Self {
        Self {
            solution,
            metadata: SolutionMetadata::Wave {
                c,
                eigenvalues,
                coefficients,
            },
        }
    }

    /// Creates a Laplace equation solution
    ///
    /// # Arguments
    ///
    /// * `solution` - The general solution expression
    /// * `eigenvalues` - Eigenvalues from boundary conditions
    /// * `coefficients` - Fourier coefficients
    pub fn laplace(
        solution: Expression,
        eigenvalues: Vec<Expression>,
        coefficients: Vec<Expression>,
    ) -> Self {
        Self {
            solution,
            metadata: SolutionMetadata::Laplace {
                eigenvalues,
                coefficients,
            },
        }
    }

    /// Creates a general PDE solution with no specific metadata
    ///
    /// # Arguments
    ///
    /// * `solution` - The general solution expression
    pub fn general(solution: Expression) -> Self {
        Self {
            solution,
            metadata: SolutionMetadata::General,
        }
    }
}

impl BoundaryCondition {
    /// Create a Dirichlet boundary condition
    pub fn dirichlet(value: Expression, location: BoundaryLocation) -> Self {
        Self::Dirichlet { value, location }
    }

    /// Creates a Dirichlet boundary condition at a specific point.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::calculus::pde::types::BoundaryCondition;
    /// use mathhook_core::{symbol, expr};
    ///
    /// let x = symbol!(x);
    /// let bc = BoundaryCondition::dirichlet_at(x, expr!(0), expr!(100));
    /// ```
    pub fn dirichlet_at(variable: Symbol, location: Expression, value: Expression) -> Self {
        BoundaryCondition::Dirichlet {
            value,
            location: BoundaryLocation::Simple {
                variable,
                value: location,
            },
        }
    }

    /// Create a Neumann boundary condition
    pub fn neumann(derivative: Expression, location: BoundaryLocation) -> Self {
        Self::Neumann {
            derivative,
            location,
        }
    }

    /// Creates a Neumann boundary condition at a specific point.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::calculus::pde::types::BoundaryCondition;
    /// use mathhook_core::{symbol, expr};
    ///
    /// let x = symbol!(x);
    /// let bc = BoundaryCondition::neumann_at(x, expr!(0), expr!(50));
    /// ```
    pub fn neumann_at(variable: Symbol, location: Expression, value: Expression) -> Self {
        BoundaryCondition::Neumann {
            derivative: value,
            location: BoundaryLocation::Simple {
                variable,
                value: location,
            },
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

    /// Creates a Robin boundary condition at a specific point.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::calculus::pde::types::BoundaryCondition;
    /// use mathhook_core::{symbol, expr};
    ///
    /// let x = symbol!(x);
    /// let bc = BoundaryCondition::robin_at(
    ///     x,
    ///     expr!(0),
    ///     expr!(1),
    ///     expr!(2),
    ///     expr!(10)
    /// );
    /// ```
    pub fn robin_at(
        variable: Symbol,
        location: Expression,
        alpha: Expression,
        beta: Expression,
        value: Expression,
    ) -> Self {
        BoundaryCondition::Robin {
            coeff_u: alpha,
            coeff_du: beta,
            value,
            location: BoundaryLocation::Simple {
                variable,
                value: location,
            },
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
        let equation = expr!(u + x + t);
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
            BoundaryCondition::Dirichlet {
                value: v,
                location: l,
            } => {
                assert_eq!(v, value);
                assert_eq!(l, location);
            }
            _ => panic!("Expected Dirichlet boundary condition"),
        }
    }

    #[test]
    fn test_boundary_condition_dirichlet_at() {
        let x = symbol!(x);
        let bc = BoundaryCondition::dirichlet_at(x.clone(), expr!(0), expr!(100));

        match bc {
            BoundaryCondition::Dirichlet {
                value,
                location:
                    BoundaryLocation::Simple {
                        variable,
                        value: loc_val,
                    },
            } => {
                assert_eq!(value, expr!(100));
                assert_eq!(variable, x);
                assert_eq!(loc_val, expr!(0));
            }
            _ => panic!("Expected Dirichlet boundary condition with Simple location"),
        }
    }

    #[test]
    fn test_boundary_condition_neumann_at() {
        let x = symbol!(x);
        let bc = BoundaryCondition::neumann_at(x.clone(), expr!(0), expr!(50));

        match bc {
            BoundaryCondition::Neumann {
                derivative,
                location:
                    BoundaryLocation::Simple {
                        variable,
                        value: loc_val,
                    },
            } => {
                assert_eq!(derivative, expr!(50));
                assert_eq!(variable, x);
                assert_eq!(loc_val, expr!(0));
            }
            _ => panic!("Expected Neumann boundary condition with Simple location"),
        }
    }

    #[test]
    fn test_boundary_condition_robin_at() {
        let x = symbol!(x);
        let bc = BoundaryCondition::robin_at(x.clone(), expr!(0), expr!(1), expr!(2), expr!(10));

        match bc {
            BoundaryCondition::Robin {
                coeff_u,
                coeff_du,
                value,
                location:
                    BoundaryLocation::Simple {
                        variable,
                        value: loc_val,
                    },
            } => {
                assert_eq!(coeff_u, expr!(1));
                assert_eq!(coeff_du, expr!(2));
                assert_eq!(value, expr!(10));
                assert_eq!(variable, x);
                assert_eq!(loc_val, expr!(0));
            }
            _ => panic!("Expected Robin boundary condition with Simple location"),
        }
    }

    #[test]
    fn test_initial_condition_value() {
        let function = expr!(x);
        let ic = InitialCondition::value(function.clone());

        match ic {
            InitialCondition::Value { function: f } => {
                assert_eq!(f, function);
            }
            _ => panic!("Expected value initial condition"),
        }
    }

    #[test]
    fn test_pde_solution_heat() {
        let sol = PDESolution::heat(
            expr!(x),
            expr!(1),
            vec![expr!(1), expr!(4)],
            vec![expr!(a), expr!(b)],
        );

        assert_eq!(sol.solution, expr!(x));
        match sol.metadata {
            SolutionMetadata::Heat {
                alpha,
                eigenvalues,
                coefficients,
            } => {
                assert_eq!(alpha, expr!(1));
                assert_eq!(eigenvalues.len(), 2);
                assert_eq!(coefficients.len(), 2);
            }
            _ => panic!("Expected Heat metadata"),
        }
    }

    #[test]
    fn test_pde_solution_wave() {
        let sol = PDESolution::wave(expr!(x), expr!(2), vec![expr!(1)], vec![expr!(a)]);

        assert_eq!(sol.solution, expr!(x));
        match sol.metadata {
            SolutionMetadata::Wave {
                c,
                eigenvalues,
                coefficients,
            } => {
                assert_eq!(c, expr!(2));
                assert_eq!(eigenvalues.len(), 1);
                assert_eq!(coefficients.len(), 1);
            }
            _ => panic!("Expected Wave metadata"),
        }
    }

    #[test]
    fn test_pde_solution_laplace() {
        let sol = PDESolution::laplace(expr!(x), vec![expr!(1)], vec![expr!(a)]);

        assert_eq!(sol.solution, expr!(x));
        match sol.metadata {
            SolutionMetadata::Laplace {
                eigenvalues,
                coefficients,
            } => {
                assert_eq!(eigenvalues.len(), 1);
                assert_eq!(coefficients.len(), 1);
            }
            _ => panic!("Expected Laplace metadata"),
        }
    }

    #[test]
    fn test_pde_solution_general() {
        let sol = PDESolution::general(expr!(x));
        assert_eq!(sol.solution, expr!(x));
        assert!(matches!(sol.metadata, SolutionMetadata::General));
    }
}
