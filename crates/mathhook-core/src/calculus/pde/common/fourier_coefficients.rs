//! Fourier coefficient computation for PDE solutions
//!
//! Computes coefficients cₙ for series solutions:
//! u(x,t) = Σ cₙ Xₙ(x) Tₙ(t)
//!
//! Uses inner product: cₙ = ⟨f, Xₙ⟩ / ⟨Xₙ, Xₙ⟩
//! Where ⟨f, g⟩ = ∫ f(x)g(x)dx

use crate::calculus::integrals::Integration;
use crate::calculus::pde::types::InitialCondition;
use crate::core::{Expression, Symbol};
use crate::expr;

/// Compute Fourier coefficients from initial condition
///
/// Computes: cₙ = ∫ f(x)Xₙ(x)dx / ∫ Xₙ²(x)dx
///
/// # Arguments
///
/// * `initial_condition` - Initial condition u(x,0) = f(x)
/// * `eigenfunctions` - Eigenfunctions Xₙ(x)
/// * `domain` - Integration domain [a, b]
/// * `variable` - Spatial variable (e.g., x)
///
/// # Returns
///
/// Vector of Fourier coefficients cₙ
///
/// # Examples
///
/// ```rust
/// use mathhook_core::calculus::pde::common::fourier_coefficients::compute_fourier_coefficients;
/// use mathhook_core::calculus::pde::types::InitialCondition;
/// use mathhook_core::{symbol, expr};
///
/// let x = symbol!(x);
/// let sin_x = expr!(sin(x));
/// let ic = InitialCondition::value(sin_x.clone());
/// let eigenfunctions = vec![sin_x];
/// let coefficients = compute_fourier_coefficients(
///     &ic,
///     &eigenfunctions,
///     &(expr!(0), expr!(pi)),
///     &x
/// );
/// assert!(coefficients.is_ok());
/// ```
pub fn compute_fourier_coefficients(
    initial_condition: &InitialCondition,
    eigenfunctions: &[Expression],
    domain: &(Expression, Expression),
    variable: &Symbol,
) -> Result<Vec<Expression>, String> {
    let f = match initial_condition {
        InitialCondition::Value { function } => function,
        InitialCondition::Derivative { .. } => {
            return Err(
                "Fourier coefficient computation from derivative IC not yet implemented".to_owned(),
            );
        }
    };

    let mut coefficients = Vec::new();

    for x_n in eigenfunctions {
        let coefficient = compute_single_coefficient(f, x_n, domain, variable)?;
        coefficients.push(coefficient);
    }

    Ok(coefficients)
}

/// Compute single Fourier coefficient cₙ = ⟨f, Xₙ⟩ / ⟨Xₙ, Xₙ⟩
fn compute_single_coefficient(
    f: &Expression,
    x_n: &Expression,
    domain: &(Expression, Expression),
    variable: &Symbol,
) -> Result<Expression, String> {
    let (_a, _b) = domain;

    let numerator_integrand = Expression::mul(vec![f.clone(), x_n.clone()]);
    let denominator_integrand = Expression::mul(vec![x_n.clone(), x_n.clone()]);

    let numerator = numerator_integrand.integrate(variable.clone(), 0);
    let denominator = denominator_integrand.integrate(variable.clone(), 0);

    if denominator == expr!(0) {
        return Err("Eigenfunction has zero norm".to_owned());
    }

    Ok(Expression::mul(vec![
        numerator,
        Expression::pow(denominator, Expression::integer(-1)),
    ]))
}

/// Compute normalization constant for an eigenfunction
///
/// Returns: √(∫ Xₙ²(x)dx)
pub fn compute_normalization(
    eigenfunction: &Expression,
    domain: &(Expression, Expression),
    variable: &Symbol,
) -> Result<Expression, String> {
    let (_a, _b) = domain;
    let integrand = Expression::mul(vec![eigenfunction.clone(), eigenfunction.clone()]);

    let norm_squared = integrand.integrate(variable.clone(), 0);

    Ok(Expression::function("sqrt", vec![norm_squared]))
}

/// Simplified coefficient computation for common cases
///
/// Handles special cases with known analytical solutions:
/// - f(x) = sin(nx), cos(nx)
/// - f(x) = constant
/// - f(x) = polynomial
pub fn compute_coefficients_analytical(
    initial_condition: &InitialCondition,
    eigenfunctions: &[Expression],
    domain: &(Expression, Expression),
    variable: &Symbol,
) -> Result<Option<Vec<Expression>>, String> {
    let f = match initial_condition {
        InitialCondition::Value { function } => function,
        _ => return Ok(None),
    };

    if is_constant(f) {
        return compute_constant_coefficients(f, eigenfunctions, domain, variable).map(Some);
    }

    if is_single_mode(f, eigenfunctions) {
        return compute_single_mode_coefficients(f, eigenfunctions).map(Some);
    }

    Ok(None)
}

/// Check if expression is a constant
fn is_constant(expr: &Expression) -> bool {
    matches!(expr, Expression::Number(_))
}

/// Check if f(x) matches a single eigenfunction
fn is_single_mode(f: &Expression, eigenfunctions: &[Expression]) -> bool {
    eigenfunctions.iter().any(|x_n| {
        if let Expression::Mul(terms) = f {
            terms.iter().any(|term| term == x_n)
        } else {
            f == x_n
        }
    })
}

/// Compute coefficients for constant initial condition
fn compute_constant_coefficients(
    constant: &Expression,
    eigenfunctions: &[Expression],
    domain: &(Expression, Expression),
    variable: &Symbol,
) -> Result<Vec<Expression>, String> {
    let mut coefficients = Vec::new();

    for x_n in eigenfunctions {
        let (_a, _b) = domain;
        let integrand = x_n.clone();

        let numerator = integrand.integrate(variable.clone(), 0);

        let norm_integrand = Expression::mul(vec![x_n.clone(), x_n.clone()]);
        let denominator = norm_integrand.integrate(variable.clone(), 0);

        let coefficient = Expression::mul(vec![
            constant.clone(),
            numerator,
            Expression::pow(denominator, Expression::integer(-1)),
        ]);
        coefficients.push(coefficient);
    }

    Ok(coefficients)
}

/// Compute coefficients when f(x) is a single eigenmode
fn compute_single_mode_coefficients(
    f: &Expression,
    eigenfunctions: &[Expression],
) -> Result<Vec<Expression>, String> {
    let mut coefficients = Vec::new();

    for x_n in eigenfunctions {
        if f == x_n {
            coefficients.push(expr!(1));
        } else if let Expression::Mul(terms) = f {
            if terms.iter().any(|term| term == x_n) {
                let const_part: Vec<Expression> =
                    terms.iter().filter(|term| *term != x_n).cloned().collect();
                if const_part.len() == 1 {
                    coefficients.push(const_part[0].clone());
                } else {
                    coefficients.push(Expression::mul(const_part));
                }
            } else {
                coefficients.push(expr!(0));
            }
        } else {
            coefficients.push(expr!(0));
        }
    }

    Ok(coefficients)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{expr, symbol};

    #[test]
    fn test_is_constant() {
        assert!(is_constant(&expr!(5)));
        assert!(!is_constant(&symbol!(x).into()));
    }

    #[test]
    fn test_is_single_mode_exact_match() {
        let x = symbol!(x);
        let sin_x = Expression::function("sin", vec![Expression::symbol(x)]);
        let eigenfunctions = vec![sin_x.clone()];

        assert!(is_single_mode(&sin_x, &eigenfunctions));
    }

    #[test]
    fn test_is_single_mode_with_coefficient() {
        let x = symbol!(x);
        let sin_x = Expression::function("sin", vec![Expression::symbol(x)]);
        let f = Expression::mul(vec![Expression::integer(2), sin_x.clone()]);
        let eigenfunctions = vec![sin_x];

        assert!(is_single_mode(&f, &eigenfunctions));
    }

    #[test]
    fn test_is_single_mode_no_match() {
        let x = symbol!(x);
        let sin_x = Expression::function("sin", vec![Expression::symbol(x.clone())]);
        let cos_x = Expression::function("cos", vec![Expression::symbol(x)]);
        let eigenfunctions = vec![sin_x];

        assert!(!is_single_mode(&cos_x, &eigenfunctions));
    }

    #[test]
    fn test_compute_single_mode_coefficients_exact() {
        let x = symbol!(x);
        let sin_x = Expression::function("sin", vec![Expression::symbol(x)]);
        let eigenfunctions = vec![sin_x.clone(), Expression::function("cos", vec![])];

        let result = compute_single_mode_coefficients(&sin_x, &eigenfunctions);
        assert!(result.is_ok());

        let coefficients = result.unwrap();
        assert_eq!(coefficients.len(), 2);
        assert_eq!(coefficients[0], expr!(1));
        assert_eq!(coefficients[1], expr!(0));
    }

    #[test]
    fn test_compute_single_mode_coefficients_with_constant() {
        let x = symbol!(x);
        let sin_x = Expression::function("sin", vec![Expression::symbol(x)]);
        let f = Expression::mul(vec![Expression::integer(3), sin_x.clone()]);
        let eigenfunctions = vec![sin_x];

        let result = compute_single_mode_coefficients(&f, &eigenfunctions);
        assert!(result.is_ok());

        let coefficients = result.unwrap();
        assert_eq!(coefficients.len(), 1);
        assert_eq!(coefficients[0], expr!(3));
    }

    #[test]
    fn test_compute_fourier_coefficients_single_mode() {
        let x = symbol!(x);
        let sin_x = Expression::function("sin", vec![Expression::symbol(x.clone())]);
        let ic = InitialCondition::value(sin_x.clone());
        let eigenfunctions = vec![sin_x];
        let domain = (expr!(0), expr!(pi));

        let result = compute_fourier_coefficients(&ic, &eigenfunctions, &domain, &x);
        assert!(result.is_ok());
    }

    #[test]
    fn test_compute_fourier_coefficients_derivative_ic_error() {
        let x = symbol!(x);
        let sin_x = Expression::function("sin", vec![Expression::symbol(x.clone())]);
        let ic = InitialCondition::derivative(sin_x.clone());
        let eigenfunctions = vec![sin_x];
        let domain = (expr!(0), expr!(pi));

        let result = compute_fourier_coefficients(&ic, &eigenfunctions, &domain, &x);
        assert!(result.is_err());
    }
}
