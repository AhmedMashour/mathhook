//! Fluid dynamics operations for velocity fields

use super::conservative::ConservativeFields;
use super::operations::VectorFieldOperations;
use crate::core::{Expression, Symbol};

/// Fluid dynamics operations
pub struct FluidDynamicsOperations;

impl FluidDynamicsOperations {
    /// Compute vorticity (curl of velocity field)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::calculus::derivatives::FluidDynamicsOperations;
    /// use mathhook_core::{Expression, symbol};
    ///
    /// let x = symbol!(x);
    /// let y = symbol!(y);
    /// let velocity_field = vec![
    ///     Expression::mul(vec![Expression::integer(-1), Expression::symbol(y.clone())]),
    ///     Expression::symbol(x.clone())
    /// ];
    /// let vorticity = FluidDynamicsOperations::vorticity(&velocity_field, vec![x, y]);
    /// ```
    pub fn vorticity(velocity_field: &[Expression], variables: Vec<Symbol>) -> Vec<Expression> {
        VectorFieldOperations::curl(velocity_field, variables)
    }

    /// Compute circulation (line integral of velocity around closed curve)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::calculus::derivatives::FluidDynamicsOperations;
    /// use mathhook_core::{Expression};
    /// use mathhook_core::symbol;
    ///
    /// let x = symbol!(x);
    /// let y = symbol!(y);
    /// let velocity_field = vec![
    ///     Expression::symbol(y.clone()),
    ///     Expression::mul(vec![Expression::integer(-1), Expression::symbol(x.clone())])
    /// ];
    /// let circulation = FluidDynamicsOperations::circulation(&velocity_field, vec![x, y]);
    /// ```
    pub fn circulation(velocity_field: &[Expression], variables: Vec<Symbol>) -> Expression {
        Expression::function(
            "line_integral",
            vec![
                Expression::function("vector_field", velocity_field.iter().cloned().collect()),
                Expression::function(
                    "variables",
                    variables
                        .iter()
                        .map(|v| Expression::symbol(v.clone()))
                        .collect(),
                ),
            ],
        )
    }

    /// Check if velocity field is incompressible (divergence = 0)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::calculus::derivatives::FluidDynamicsOperations;
    /// use mathhook_core::{Expression};
    /// use mathhook_core::symbol;
    ///
    /// let x = symbol!(x);
    /// let y = symbol!(y);
    /// let velocity_field = vec![
    ///     Expression::symbol(y.clone()),
    ///     Expression::mul(vec![Expression::integer(-1), Expression::symbol(x.clone())])
    /// ];
    /// let is_incompressible = FluidDynamicsOperations::is_incompressible(&velocity_field, vec![x, y]);
    /// ```
    pub fn is_incompressible(velocity_field: &[Expression], variables: Vec<Symbol>) -> bool {
        ConservativeFields::is_solenoidal(velocity_field, variables)
    }
}
