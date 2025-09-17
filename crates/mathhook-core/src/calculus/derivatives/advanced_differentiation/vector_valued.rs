//! Vector-valued function differentiation
//!
//! Handles differentiation of vector-valued functions r(t) = [x(t), y(t), z(t)]
//! and computation of geometric properties like curvature and torsion.
pub mod components;
pub mod geometry;
use crate::core::{Expression, Symbol};
pub use components::VectorComponents;
pub use geometry::VectorGeometry;
/// Vector-valued function differentiation (legacy compatibility wrapper)
pub struct VectorValuedDifferentiation;
impl VectorValuedDifferentiation {
    /// Compute derivative
    pub fn derivative(components: &[Expression], parameter: Symbol) -> Vec<Expression> {
        VectorComponents::derivative(components, &parameter)
    }
    /// Compute second derivative
    pub fn second_derivative(components: &[Expression], parameter: Symbol) -> Vec<Expression> {
        VectorComponents::second_derivative(components, parameter)
    }
    /// Compute nth derivative
    pub fn nth_derivative(
        components: &[Expression],
        parameter: Symbol,
        order: u32,
    ) -> Vec<Expression> {
        VectorComponents::nth_derivative(components, parameter, order)
    }
    /// Compute magnitude of vector
    pub fn magnitude(components: &[Expression]) -> Expression {
        VectorComponents::magnitude(components)
    }
    /// Compute unit tangent vector T = r'(t) / |r'(t)|
    pub fn unit_tangent(components: &[Expression], parameter: Symbol) -> Vec<Expression> {
        VectorGeometry::unit_tangent(components, parameter)
    }
    /// Compute curvature κ = |r' × r''| / |r'|³
    pub fn curvature(components: &[Expression], parameter: Symbol) -> Expression {
        VectorGeometry::curvature(components, parameter)
    }
    /// Compute torsion τ for 3D curves
    pub fn torsion(components: &[Expression], parameter: Symbol) -> Expression {
        VectorGeometry::torsion(components, parameter)
    }
}
