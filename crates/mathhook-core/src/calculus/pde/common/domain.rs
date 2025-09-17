//! Domain extraction utilities for PDE boundary value problems

use crate::calculus::pde::registry::PDEError;
use crate::calculus::pde::types::{BoundaryCondition, BoundaryLocation};
use crate::core::{Expression, Symbol};

/// Extracts domain length from boundary conditions.
///
/// Assumes domain [0, L] with boundary conditions specified at x=0 and x=L.
/// Searches for the right boundary value (non-zero location) and returns it as domain length.
///
/// # Arguments
/// * `boundary_conditions` - Boundary conditions
/// * `spatial_var` - Spatial variable symbol
///
/// # Returns
/// Domain length L. Defaults to π if no explicit length found.
///
/// # Examples
/// ```rust
/// use mathhook_core::calculus::pde::common::extract_domain_length;
/// use mathhook_core::calculus::pde::types::{BoundaryCondition, BoundaryLocation};
/// use mathhook_core::{symbol, expr};
///
/// let x = symbol!(x);
/// let bc = BoundaryCondition::dirichlet(
///     expr!(0),
///     BoundaryLocation::Simple { variable: x.clone(), value: expr!(10) }
/// );
/// let length = extract_domain_length(&[bc], &x).unwrap();
/// ```
pub fn extract_domain_length(
    boundary_conditions: &[BoundaryCondition],
    spatial_var: &Symbol,
) -> Result<Expression, PDEError> {
    for bc in boundary_conditions {
        let location = match bc {
            BoundaryCondition::Dirichlet { location, .. } => location,
            BoundaryCondition::Neumann { location, .. } => location,
            BoundaryCondition::Robin { location, .. } => location,
        };

        if let BoundaryLocation::Simple {
            variable,
            value: location_value,
        } = location
        {
            if variable == spatial_var
                && !matches!(location_value, Expression::Number(n) if n.is_zero())
            {
                return Ok(location_value.clone());
            }
        }
    }

    Ok(Expression::pi())
}
