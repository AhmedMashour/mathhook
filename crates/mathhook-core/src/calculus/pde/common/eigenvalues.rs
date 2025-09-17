//! Eigenvalue computation for standard boundary value problems

use crate::calculus::pde::common::extract_domain_length;
use crate::calculus::pde::registry::PDEError;
use crate::calculus::pde::types::BoundaryCondition;
use crate::core::{Expression, Symbol};

/// Computes eigenvalues for 1D Dirichlet boundary conditions (heat equation form).
///
/// For domain [0, L] with homogeneous Dirichlet boundary conditions:
/// u(0,t) = 0, u(L,t) = 0
///
/// The eigenvalues are: λₙ = (nπ/L)² for n = 1, 2, 3, ...
///
/// # Arguments
/// * `boundary_conditions` - Boundary conditions (used to extract domain length)
/// * `spatial_var` - Spatial variable symbol
/// * `max_terms` - Maximum number of eigenvalues to compute
///
/// # Returns
/// Vector of eigenvalue expressions λ₁, λ₂, ..., λₙ
///
/// # Examples
/// ```rust
/// use mathhook_core::calculus::pde::common::compute_dirichlet_1d_eigenvalues;
/// use mathhook_core::symbol;
///
/// let x = symbol!(x);
/// let bcs = vec![];
/// let eigenvalues = compute_dirichlet_1d_eigenvalues(&bcs, &x, 5).unwrap();
/// assert_eq!(eigenvalues.len(), 5);
/// ```
pub fn compute_dirichlet_1d_eigenvalues(
    boundary_conditions: &[BoundaryCondition],
    spatial_var: &Symbol,
    max_terms: usize,
) -> Result<Vec<Expression>, PDEError> {
    if boundary_conditions.is_empty() {
        let eigenvalues: Vec<_> = (1..=max_terms)
            .map(|n| Expression::integer(n as i64))
            .collect();
        return Ok(eigenvalues);
    }

    let domain_length = extract_domain_length(boundary_conditions, spatial_var)?;

    let eigenvalues: Vec<_> = (1..=max_terms)
        .map(|n| {
            let n_expr = Expression::integer(n as i64);
            let pi = Expression::pi();

            Expression::pow(
                Expression::mul(vec![
                    n_expr,
                    pi,
                    Expression::pow(domain_length.clone(), Expression::integer(-1)),
                ]),
                Expression::integer(2),
            )
        })
        .collect();

    Ok(eigenvalues)
}

/// Computes eigenvalues for 1D Dirichlet boundary conditions (wave equation form).
///
/// For domain [0, L] with homogeneous Dirichlet boundary conditions:
/// u(0,t) = 0, u(L,t) = 0
///
/// The eigenvalues are: λₙ = nπ/L for n = 1, 2, 3, ...
///
/// # Arguments
/// * `boundary_conditions` - Boundary conditions (used to extract domain length)
/// * `spatial_var` - Spatial variable symbol
/// * `max_terms` - Maximum number of eigenvalues to compute
///
/// # Returns
/// Vector of eigenvalue expressions λ₁, λ₂, ..., λₙ
///
/// # Examples
/// ```rust
/// use mathhook_core::calculus::pde::common::compute_wave_eigenvalues;
/// use mathhook_core::symbol;
///
/// let x = symbol!(x);
/// let bcs = vec![];
/// let eigenvalues = compute_wave_eigenvalues(&bcs, &x, 5).unwrap();
/// assert_eq!(eigenvalues.len(), 5);
/// ```
pub fn compute_wave_eigenvalues(
    boundary_conditions: &[BoundaryCondition],
    spatial_var: &Symbol,
    max_terms: usize,
) -> Result<Vec<Expression>, PDEError> {
    if boundary_conditions.is_empty() {
        let eigenvalues: Vec<_> = (1..=max_terms)
            .map(|n| Expression::integer(n as i64))
            .collect();
        return Ok(eigenvalues);
    }

    let domain_length = extract_domain_length(boundary_conditions, spatial_var)?;

    let eigenvalues: Vec<_> = (1..=max_terms)
        .map(|n| {
            let n_expr = Expression::integer(n as i64);
            let pi = Expression::pi();

            Expression::mul(vec![
                n_expr,
                pi,
                Expression::pow(domain_length.clone(), Expression::integer(-1)),
            ])
        })
        .collect();

    Ok(eigenvalues)
}
