//! Eigenvalue problem solver for Sturm-Liouville problems
//!
//! Solves standard eigenvalue problems of the form:
//! X''(x) + λX(x) = 0
//!
//! With various boundary conditions:
//! - Dirichlet: X(a) = 0, X(b) = 0
//! - Neumann: X'(a) = 0, X'(b) = 0
//! - Mixed: Dirichlet on one end, Neumann on the other
//! - Robin: αX(a) + βX'(a) = 0
//!
//! Returns both eigenvalues and corresponding eigenfunctions.

use crate::calculus::pde::types::{BoundaryCondition, BoundaryLocation};
use crate::core::{Expression, Symbol};

/// Result of solving an eigenvalue problem
#[derive(Debug, Clone, PartialEq)]
pub struct EigenvalueSolution {
    /// The eigenvalues λₙ
    pub eigenvalues: Vec<Expression>,
    /// The eigenfunctions Xₙ(x)
    pub eigenfunctions: Vec<Expression>,
    /// The variable (e.g., x)
    pub variable: Symbol,
    /// The domain [a, b]
    pub domain: (Expression, Expression),
}

/// Type of boundary condition pair
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum BoundaryType {
    /// Dirichlet on both ends
    DirichletDirichlet,
    /// Neumann on both ends
    NeumannNeumann,
    /// Dirichlet at left, Neumann at right
    DirichletNeumann,
    /// Neumann at left, Dirichlet at right
    NeumannDirichlet,
}

/// Solve Sturm-Liouville eigenvalue problem with boundary conditions
///
/// Solves: X''(x) + λX(x) = 0 on [a, b] with given BCs
///
/// # Arguments
///
/// * `bc_left` - Boundary condition at left endpoint
/// * `bc_right` - Boundary condition at right endpoint
/// * `num_modes` - Number of eigenvalue/eigenfunction pairs to compute
///
/// # Returns
///
/// Eigenvalues and eigenfunctions, or error if BCs are incompatible
///
/// # Examples
///
/// ```rust
/// use mathhook_core::calculus::pde::common::eigenvalue_problem::solve_sturm_liouville;
/// use mathhook_core::calculus::pde::types::BoundaryCondition;
/// use mathhook_core::{symbol, expr};
///
/// let x = symbol!(x);
/// let bc_left = BoundaryCondition::dirichlet_at(x.clone(), expr!(0), expr!(0));
/// let bc_right = BoundaryCondition::dirichlet_at(x.clone(), expr!(pi), expr!(0));
///
/// let result = solve_sturm_liouville(&bc_left, &bc_right, 5);
/// assert!(result.is_ok());
/// let solution = result.unwrap();
/// assert_eq!(solution.eigenvalues.len(), 5);
/// ```
pub fn solve_sturm_liouville(
    bc_left: &BoundaryCondition,
    bc_right: &BoundaryCondition,
    num_modes: usize,
) -> Result<EigenvalueSolution, String> {
    let (var, domain) = extract_domain(bc_left, bc_right)?;
    let bc_type = classify_boundary_conditions(bc_left, bc_right)?;

    let (a, b) = domain.clone();
    let length = compute_length(&a, &b);

    let (eigenvalues, eigenfunctions) = match bc_type {
        BoundaryType::DirichletDirichlet => solve_dirichlet_dirichlet(&var, &length, num_modes),
        BoundaryType::NeumannNeumann => solve_neumann_neumann(&var, &length, num_modes),
        BoundaryType::DirichletNeumann => solve_dirichlet_neumann(&var, &length, num_modes),
        BoundaryType::NeumannDirichlet => solve_neumann_dirichlet(&var, &length, num_modes),
    };

    Ok(EigenvalueSolution {
        eigenvalues,
        eigenfunctions,
        variable: var,
        domain,
    })
}

/// Extract spatial variable and domain from boundary conditions
fn extract_domain(
    bc_left: &BoundaryCondition,
    bc_right: &BoundaryCondition,
) -> Result<(Symbol, (Expression, Expression)), String> {
    let (var_left, a) = extract_location(bc_left)?;
    let (var_right, b) = extract_location(bc_right)?;

    if var_left != var_right {
        return Err(format!(
            "Boundary conditions have different variables: {} and {}",
            var_left.name(),
            var_right.name()
        ));
    }

    Ok((var_left, (a, b)))
}

/// Extract variable and location from a boundary condition
fn extract_location(bc: &BoundaryCondition) -> Result<(Symbol, Expression), String> {
    let location = match bc {
        BoundaryCondition::Dirichlet { location, .. } => location,
        BoundaryCondition::Neumann { location, .. } => location,
        BoundaryCondition::Robin { location, .. } => location,
    };

    match location {
        BoundaryLocation::Simple { variable, value } => Ok((variable.clone(), value.clone())),
        _ => Err("Only simple boundary locations (var = value) are supported".to_owned()),
    }
}

/// Classify boundary condition types
fn classify_boundary_conditions(
    bc_left: &BoundaryCondition,
    bc_right: &BoundaryCondition,
) -> Result<BoundaryType, String> {
    let left_is_dirichlet = matches!(bc_left, BoundaryCondition::Dirichlet { .. });
    let right_is_dirichlet = matches!(bc_right, BoundaryCondition::Dirichlet { .. });

    let left_is_neumann = matches!(bc_left, BoundaryCondition::Neumann { .. });
    let right_is_neumann = matches!(bc_right, BoundaryCondition::Neumann { .. });

    if matches!(bc_left, BoundaryCondition::Robin { .. })
        || matches!(bc_right, BoundaryCondition::Robin { .. })
    {
        return Err("Robin boundary conditions not yet implemented".to_owned());
    }

    match (left_is_dirichlet, right_is_dirichlet) {
        (true, true) => Ok(BoundaryType::DirichletDirichlet),
        (false, false) if left_is_neumann && right_is_neumann => Ok(BoundaryType::NeumannNeumann),
        (true, false) if right_is_neumann => Ok(BoundaryType::DirichletNeumann),
        (false, true) if left_is_neumann => Ok(BoundaryType::NeumannDirichlet),
        _ => Err("Unsupported boundary condition combination".to_owned()),
    }
}

/// Compute domain length L = b - a
fn compute_length(a: &Expression, b: &Expression) -> Expression {
    Expression::add(vec![
        b.clone(),
        Expression::mul(vec![Expression::integer(-1), a.clone()]),
    ])
}

/// Solve Dirichlet-Dirichlet problem: X(0) = 0, X(L) = 0
///
/// Solution: λₙ = (nπ/L)², Xₙ(x) = sin(nπx/L)
fn solve_dirichlet_dirichlet(
    var: &Symbol,
    length: &Expression,
    num_modes: usize,
) -> (Vec<Expression>, Vec<Expression>) {
    let mut eigenvalues = Vec::new();
    let mut eigenfunctions = Vec::new();

    for n in 1..=num_modes {
        let n_expr = Expression::integer(n as i64);

        let n_pi = Expression::mul(vec![n_expr.clone(), Expression::pi()]);
        let n_pi_squared = Expression::pow(n_pi.clone(), Expression::integer(2));
        let length_squared = Expression::pow(length.clone(), Expression::integer(2));
        let lambda_n = Expression::mul(vec![
            n_pi_squared,
            Expression::pow(length_squared, Expression::integer(-1)),
        ]);
        eigenvalues.push(lambda_n);

        let arg = Expression::mul(vec![
            n_pi,
            Expression::symbol(var.clone()),
            Expression::pow(length.clone(), Expression::integer(-1)),
        ]);
        let x_n = Expression::function("sin", vec![arg]);
        eigenfunctions.push(x_n);
    }

    (eigenvalues, eigenfunctions)
}

/// Solve Neumann-Neumann problem: X'(0) = 0, X'(L) = 0
///
/// Solution: λ₀ = 0, X₀ = 1; λₙ = (nπ/L)², Xₙ(x) = cos(nπx/L) for n ≥ 1
fn solve_neumann_neumann(
    var: &Symbol,
    length: &Expression,
    num_modes: usize,
) -> (Vec<Expression>, Vec<Expression>) {
    let mut eigenvalues = Vec::new();
    let mut eigenfunctions = Vec::new();

    eigenvalues.push(Expression::integer(0));
    eigenfunctions.push(Expression::integer(1));

    for n in 1..num_modes {
        let n_expr = Expression::integer(n as i64);

        let n_pi = Expression::mul(vec![n_expr.clone(), Expression::pi()]);
        let n_pi_squared = Expression::pow(n_pi.clone(), Expression::integer(2));
        let length_squared = Expression::pow(length.clone(), Expression::integer(2));
        let lambda_n = Expression::mul(vec![
            n_pi_squared,
            Expression::pow(length_squared, Expression::integer(-1)),
        ]);
        eigenvalues.push(lambda_n);

        let arg = Expression::mul(vec![
            n_pi,
            Expression::symbol(var.clone()),
            Expression::pow(length.clone(), Expression::integer(-1)),
        ]);
        let x_n = Expression::function("cos", vec![arg]);
        eigenfunctions.push(x_n);
    }

    (eigenvalues, eigenfunctions)
}

/// Solve Dirichlet-Neumann problem: X(0) = 0, X'(L) = 0
///
/// Solution: λₙ = ((2n-1)π/2L)², Xₙ(x) = sin((2n-1)πx/2L)
fn solve_dirichlet_neumann(
    var: &Symbol,
    length: &Expression,
    num_modes: usize,
) -> (Vec<Expression>, Vec<Expression>) {
    let mut eigenvalues = Vec::new();
    let mut eigenfunctions = Vec::new();

    for n in 1..=num_modes {
        let two_n_minus_1 = Expression::add(vec![
            Expression::mul(vec![Expression::integer(2), Expression::integer(n as i64)]),
            Expression::integer(-1),
        ]);

        let numerator = Expression::mul(vec![two_n_minus_1.clone(), Expression::pi()]);
        let numerator_squared = Expression::pow(numerator.clone(), Expression::integer(2));

        let denominator = Expression::mul(vec![
            Expression::integer(4),
            Expression::pow(length.clone(), Expression::integer(2)),
        ]);

        let lambda_n = Expression::mul(vec![
            numerator_squared,
            Expression::pow(denominator, Expression::integer(-1)),
        ]);
        eigenvalues.push(lambda_n);

        let arg_numerator = Expression::mul(vec![
            two_n_minus_1,
            Expression::pi(),
            Expression::symbol(var.clone()),
        ]);
        let arg_denominator = Expression::mul(vec![Expression::integer(2), length.clone()]);
        let arg = Expression::mul(vec![
            arg_numerator,
            Expression::pow(arg_denominator, Expression::integer(-1)),
        ]);
        let x_n = Expression::function("sin", vec![arg]);
        eigenfunctions.push(x_n);
    }

    (eigenvalues, eigenfunctions)
}

/// Solve Neumann-Dirichlet problem: X'(0) = 0, X(L) = 0
///
/// Solution: λₙ = ((2n-1)π/2L)², Xₙ(x) = cos((2n-1)πx/2L)
fn solve_neumann_dirichlet(
    var: &Symbol,
    length: &Expression,
    num_modes: usize,
) -> (Vec<Expression>, Vec<Expression>) {
    let mut eigenvalues = Vec::new();
    let mut eigenfunctions = Vec::new();

    for n in 1..=num_modes {
        let two_n_minus_1 = Expression::add(vec![
            Expression::mul(vec![Expression::integer(2), Expression::integer(n as i64)]),
            Expression::integer(-1),
        ]);

        let numerator = Expression::mul(vec![two_n_minus_1.clone(), Expression::pi()]);
        let numerator_squared = Expression::pow(numerator.clone(), Expression::integer(2));

        let denominator = Expression::mul(vec![
            Expression::integer(4),
            Expression::pow(length.clone(), Expression::integer(2)),
        ]);

        let lambda_n = Expression::mul(vec![
            numerator_squared,
            Expression::pow(denominator, Expression::integer(-1)),
        ]);
        eigenvalues.push(lambda_n);

        let arg_numerator = Expression::mul(vec![
            two_n_minus_1,
            Expression::pi(),
            Expression::symbol(var.clone()),
        ]);
        let arg_denominator = Expression::mul(vec![Expression::integer(2), length.clone()]);
        let arg = Expression::mul(vec![
            arg_numerator,
            Expression::pow(arg_denominator, Expression::integer(-1)),
        ]);
        let x_n = Expression::function("cos", vec![arg]);
        eigenfunctions.push(x_n);
    }

    (eigenvalues, eigenfunctions)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{expr, symbol};

    #[test]
    fn test_dirichlet_dirichlet_eigenvalues() {
        let x = symbol!(x);
        let bc_left = BoundaryCondition::dirichlet_at(x.clone(), expr!(0), expr!(0));
        let bc_right = BoundaryCondition::dirichlet_at(x.clone(), expr!(pi), expr!(0));

        let result = solve_sturm_liouville(&bc_left, &bc_right, 3);
        assert!(result.is_ok());

        let solution = result.unwrap();
        assert_eq!(solution.eigenvalues.len(), 3);
        assert_eq!(solution.eigenfunctions.len(), 3);
    }

    #[test]
    fn test_neumann_neumann_eigenvalues() {
        let x = symbol!(x);
        let bc_left = BoundaryCondition::neumann_at(x.clone(), expr!(0), expr!(0));
        let bc_right = BoundaryCondition::neumann_at(x.clone(), expr!(pi), expr!(0));

        let result = solve_sturm_liouville(&bc_left, &bc_right, 3);
        assert!(result.is_ok());

        let solution = result.unwrap();
        assert_eq!(solution.eigenvalues.len(), 3);
        assert_eq!(solution.eigenfunctions.len(), 3);
    }

    #[test]
    fn test_mixed_boundary_conditions() {
        let x = symbol!(x);
        let bc_left = BoundaryCondition::dirichlet_at(x.clone(), expr!(0), expr!(0));
        let bc_right = BoundaryCondition::neumann_at(x.clone(), expr!(pi), expr!(0));

        let result = solve_sturm_liouville(&bc_left, &bc_right, 3);
        assert!(result.is_ok());

        let solution = result.unwrap();
        assert_eq!(solution.eigenvalues.len(), 3);
        assert_eq!(solution.eigenfunctions.len(), 3);
    }

    #[test]
    fn test_incompatible_variables() {
        let x = symbol!(x);
        let y = symbol!(y);
        let bc_left = BoundaryCondition::dirichlet_at(x, expr!(0), expr!(0));
        let bc_right = BoundaryCondition::dirichlet_at(y, expr!(pi), expr!(0));

        let result = solve_sturm_liouville(&bc_left, &bc_right, 3);
        assert!(result.is_err());
    }

    #[test]
    fn test_extract_domain() {
        let x = symbol!(x);
        let bc_left = BoundaryCondition::dirichlet_at(x.clone(), expr!(0), expr!(0));
        let bc_right = BoundaryCondition::dirichlet_at(x.clone(), expr!(1), expr!(0));

        let result = extract_domain(&bc_left, &bc_right);
        assert!(result.is_ok());

        let (var, (a, b)) = result.unwrap();
        assert_eq!(var, x);
        assert_eq!(a, expr!(0));
        assert_eq!(b, expr!(1));
    }

    #[test]
    fn test_classify_boundary_conditions_dirichlet_dirichlet() {
        let x = symbol!(x);
        let bc_left = BoundaryCondition::dirichlet_at(x.clone(), expr!(0), expr!(0));
        let bc_right = BoundaryCondition::dirichlet_at(x, expr!(1), expr!(0));

        let result = classify_boundary_conditions(&bc_left, &bc_right);
        assert_eq!(result.unwrap(), BoundaryType::DirichletDirichlet);
    }

    #[test]
    fn test_classify_boundary_conditions_neumann_neumann() {
        let x = symbol!(x);
        let bc_left = BoundaryCondition::neumann_at(x.clone(), expr!(0), expr!(0));
        let bc_right = BoundaryCondition::neumann_at(x, expr!(1), expr!(0));

        let result = classify_boundary_conditions(&bc_left, &bc_right);
        assert_eq!(result.unwrap(), BoundaryType::NeumannNeumann);
    }

    #[test]
    fn test_dirichlet_neumann_mode_count() {
        let x = symbol!(x);
        let length = Expression::integer(1);

        let (eigenvalues, eigenfunctions) = solve_dirichlet_neumann(&x, &length, 5);
        assert_eq!(eigenvalues.len(), 5);
        assert_eq!(eigenfunctions.len(), 5);
    }

    #[test]
    fn test_neumann_dirichlet_mode_count() {
        let x = symbol!(x);
        let length = Expression::integer(1);

        let (eigenvalues, eigenfunctions) = solve_neumann_dirichlet(&x, &length, 4);
        assert_eq!(eigenvalues.len(), 4);
        assert_eq!(eigenfunctions.len(), 4);
    }
}
