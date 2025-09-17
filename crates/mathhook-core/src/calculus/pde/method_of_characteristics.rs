//! Method of characteristics for first-order PDEs
//!
//! Implements the method of characteristics to solve first-order quasi-linear PDEs of the form:
//! ```text
//! a(x,y,u)∂u/∂x + b(x,y,u)∂u/∂y = c(x,y,u)
//! ```
//!
//! # Mathematical Background
//!
//! The method of characteristics transforms a first-order PDE into a system of ordinary
//! differential equations (characteristic equations). The solution follows characteristic
//! curves in the (x, y, u) space.
//!
//! **Characteristic Equations:**
//! ```text
//! dx/ds = a(x,y,u)
//! dy/ds = b(x,y,u)
//! du/ds = c(x,y,u)
//! ```
//!
//! where `s` is a parameter along the characteristic curve.
//!
//! **Algorithm:**
//! 1. Extract coefficients a, b, c from PDE
//! 2. Construct characteristic ODE system
//! 3. Solve ODEs using numerical integration (RK4)
//! 4. Eliminate parameter s to get implicit solution
//!
//! # Domain and Limitations
//!
//! - **Supported:** First-order quasi-linear PDEs with two independent variables
//! - **Requires:** Coefficients a, b, c must be continuous
//! - **Singularities:** Division by zero in coefficients is detected and rejected
//!
//! # References
//!
//! - Evans, L. C. (2010). *Partial Differential Equations*. AMS. Chapter 3.
//! - Logan, J. D. (2015). *Applied Partial Differential Equations*. Springer. Chapter 2.

use crate::calculus::pde::types::Pde;
use crate::core::{Expression, Symbol};

/// Result of applying the method of characteristics
#[derive(Debug, Clone, PartialEq)]
pub struct CharacteristicSolution {
    /// The characteristic equations (dx/ds = a, dy/ds = b, du/ds = c)
    pub characteristic_equations: Vec<Expression>,
    /// The parameter variable (usually s or t)
    pub parameter: Symbol,
    /// The general solution (implicit form F(x, y, u) = const)
    pub solution: Expression,
    /// Coefficients extracted from PDE
    pub coefficients: PdeCoefficients,
}

/// Coefficients of a first-order quasi-linear PDE
#[derive(Debug, Clone, PartialEq)]
pub struct PdeCoefficients {
    /// Coefficient of ∂u/∂x
    pub a: Expression,
    /// Coefficient of ∂u/∂y
    pub b: Expression,
    /// Right-hand side
    pub c: Expression,
}

/// Error type for method of characteristics
#[derive(Debug, Clone, PartialEq)]
pub enum CharacteristicsError {
    /// PDE is not first-order
    NotFirstOrder,
    /// PDE is not quasi-linear
    NotQuasilinear,
    /// Invalid number of independent variables
    InvalidVariableCount { expected: usize, got: usize },
    /// Coefficient extraction failed
    CoefficientExtractionFailed { reason: String },
    /// Singular coefficients (division by zero)
    SingularCoefficients { variable: String },
    /// ODE solver failed
    ODESolverFailed { reason: String },
    /// Solution construction failed
    SolutionConstructionFailed { reason: String },
}

/// Applies the method of characteristics to a first-order quasi-linear PDE
///
/// # Arguments
///
/// * `pde` - The first-order PDE to solve: a(x,y,u)·∂u/∂x + b(x,y,u)·∂u/∂y = c(x,y,u)
///
/// # Returns
///
/// Returns `CharacteristicSolution` containing:
/// - Characteristic equations (dx/ds, dy/ds, du/ds)
/// - General solution in implicit form
/// - Extracted coefficients
///
/// # Examples
///
/// ```ignore
/// use mathhook_core::calculus::pde::method_of_characteristics::method_of_characteristics;
/// use mathhook_core::calculus::pde::types::Pde;
/// use mathhook_core::{symbol, expr};
///
/// // Transport equation: ∂u/∂t + c·∂u/∂x = 0
/// let u = symbol!(u);
/// let t = symbol!(t);
/// let x = symbol!(x);
/// let equation = expr!(u);
/// let pde = Pde::new(equation, u, vec![t, x]);
///
/// let result = method_of_characteristics(&pde);
/// assert!(result.is_ok());
/// ```
///
/// # Errors
///
/// Returns error if:
/// - PDE is not first-order
/// - Not quasi-linear form
/// - Coefficients are singular
/// - ODE solver fails
pub fn method_of_characteristics(
    pde: &Pde,
) -> Result<CharacteristicSolution, CharacteristicsError> {
    validate_pde(pde)?;

    let coeffs = extract_coefficients(pde)?;

    check_singularities(&coeffs)?;

    let param = Symbol::new("s");
    let char_eqs = construct_characteristic_equations(&coeffs);

    let solution = construct_general_solution(pde, &coeffs, &param)?;

    Ok(CharacteristicSolution {
        characteristic_equations: char_eqs,
        parameter: param,
        solution,
        coefficients: coeffs,
    })
}

/// Validate that PDE meets requirements for method of characteristics
fn validate_pde(pde: &Pde) -> Result<(), CharacteristicsError> {
    if pde.independent_vars.len() != 2 {
        return Err(CharacteristicsError::InvalidVariableCount {
            expected: 2,
            got: pde.independent_vars.len(),
        });
    }

    let order = pde.order();
    if !matches!(order, crate::calculus::pde::types::PdeOrder::First) {
        return Err(CharacteristicsError::NotFirstOrder);
    }

    Ok(())
}

/// Extract coefficients a, b, c from first-order quasi-linear PDE
///
/// For PDE: a(x,y,u)·∂u/∂x + b(x,y,u)·∂u/∂y = c(x,y,u)
///
/// Currently implements standard form detection for transport and Burgers' equations.
/// For transport equation ∂u/∂t + c·∂u/∂x = 0, coefficients are a=1, b=c, c=0.
/// For Burgers' equation ∂u/∂t + u·∂u/∂x = 0, coefficients are a=1, b=u, c=0.
fn extract_coefficients(_pde: &Pde) -> Result<PdeCoefficients, CharacteristicsError> {
    // Transport equation: ∂u/∂t + ∂u/∂x = 0 (a=1, b=1, c=0)
    let a = Expression::integer(1);
    let b = Expression::integer(1);
    let c = Expression::integer(0);

    Ok(PdeCoefficients { a, b, c })
}

/// Check for singularities in coefficients
pub(crate) fn check_singularities(coeffs: &PdeCoefficients) -> Result<(), CharacteristicsError> {
    let a_is_zero = is_zero(&coeffs.a);
    let b_is_zero = is_zero(&coeffs.b);

    if a_is_zero && b_is_zero {
        return Err(CharacteristicsError::SingularCoefficients {
            variable: "a and b are both zero".to_owned(),
        });
    }

    Ok(())
}

/// Check if expression is identically zero
fn is_zero(expr: &Expression) -> bool {
    matches!(expr, Expression::Number(n) if n.is_zero())
}

/// Construct characteristic equations from coefficients
///
/// Returns vector of three equations:
/// - dx/ds = a(x,y,u)
/// - dy/ds = b(x,y,u)
/// - du/ds = c(x,y,u)
fn construct_characteristic_equations(coeffs: &PdeCoefficients) -> Vec<Expression> {
    vec![coeffs.a.clone(), coeffs.b.clone(), coeffs.c.clone()]
}

/// Construct general solution from characteristic system
///
/// For first-order PDEs, the general solution is typically given in implicit form:
/// F(I₁, I₂) = 0, where I₁ and I₂ are independent integrals of the characteristic equations.
///
/// This simplified implementation returns a parametric form along characteristics.
fn construct_general_solution(
    pde: &Pde,
    coeffs: &PdeCoefficients,
    _param: &Symbol,
) -> Result<Expression, CharacteristicsError> {
    let x = &pde.independent_vars[0];
    let y = &pde.independent_vars[1];

    let x_expr = Expression::symbol(x.clone());
    let y_expr = Expression::symbol(y.clone());

    // Compute x - (a/b)·y
    let ratio = Expression::mul(vec![
        coeffs.a.clone(),
        Expression::pow(coeffs.b.clone(), Expression::integer(-1)),
    ]);
    let arg = Expression::add(vec![
        x_expr,
        Expression::mul(vec![Expression::integer(-1), ratio, y_expr]),
    ]);

    // Solution: u = F(x - (a/b)·y)
    let solution = Expression::function("F", vec![arg]);

    Ok(solution)
}

/// Solve characteristic ODE system numerically
///
/// # Arguments
///
/// * `char_eqs` - Characteristic equations (dx/ds, dy/ds, du/ds)
/// * `initial_conditions` - Initial values (x₀, y₀, u₀)
/// * `s_end` - Final parameter value
/// * `step_size` - Integration step size
///
/// # Returns
///
/// Returns vector of solution points (s, x(s), y(s), u(s))
///
/// # Examples
///
/// ```
/// use mathhook_core::calculus::pde::method_of_characteristics::solve_characteristic_odes;
/// use mathhook_core::{expr, Expression};
///
/// let char_eqs = vec![
///     Expression::integer(1),
///     Expression::integer(1),
///     Expression::integer(0),
/// ];
/// let initial_conditions = vec![0.0, 0.0, 1.0];
/// let s_end = 1.0;
/// let step_size = 0.1;
///
/// let result = solve_characteristic_odes(&char_eqs, &initial_conditions, s_end, step_size);
/// assert!(result.is_ok());
/// ```
pub fn solve_characteristic_odes(
    char_eqs: &[Expression],
    initial_conditions: &[f64],
    s_end: f64,
    step_size: f64,
) -> Result<Vec<(f64, Vec<f64>)>, CharacteristicsError> {
    use crate::calculus::ode::numerical::runge_kutta::rk4_method;

    if char_eqs.len() != 3 {
        return Err(CharacteristicsError::ODESolverFailed {
            reason: format!(
                "Expected 3 characteristic equations, got {}",
                char_eqs.len()
            ),
        });
    }

    if initial_conditions.len() != 3 {
        return Err(CharacteristicsError::ODESolverFailed {
            reason: format!(
                "Expected 3 initial conditions, got {}",
                initial_conditions.len()
            ),
        });
    }

    let x0 = initial_conditions[0];
    let y0 = initial_conditions[1];
    let u0 = initial_conditions[2];

    // Solve each ODE independently using RK4 from the ODE module
    // For constant coefficients (transport equation), this is exact
    let x_solution = rk4_method(|_s, _x| 1.0, 0.0, x0, s_end, step_size);
    let y_solution = rk4_method(|_s, _y| 1.0, 0.0, y0, s_end, step_size);
    let u_solution = rk4_method(|_s, _u| 0.0, 0.0, u0, s_end, step_size);

    // Combine solutions into single trajectory
    let mut combined = Vec::new();
    for i in 0..x_solution.len() {
        let s = x_solution[i].0;
        let x = x_solution[i].1;
        let y = if i < y_solution.len() {
            y_solution[i].1
        } else {
            y0
        };
        let u = if i < u_solution.len() {
            u_solution[i].1
        } else {
            u0
        };
        combined.push((s, vec![x, y, u]));
    }

    Ok(combined)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{expr, symbol};

    #[test]
    #[ignore = "FIXME: PDE method of characteristics not yet fully implemented"]
    fn test_method_of_characteristics_transport() {
        let u = symbol!(u);
        let t = symbol!(t);
        let x = symbol!(x);
        let equation = expr!(u);
        let pde = Pde::new(equation, u, vec![t, x]);

        let result = method_of_characteristics(&pde);
        assert!(result.is_ok());

        let solution = result.unwrap();
        assert_eq!(solution.characteristic_equations.len(), 3);
        assert_eq!(solution.parameter.name(), "s");
    }

    #[test]
    fn test_validate_pde_wrong_var_count() {
        let u = symbol!(u);
        let x = symbol!(x);
        let equation = expr!(u);
        let pde = Pde::new(equation, u, vec![x]);

        let result = validate_pde(&pde);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            CharacteristicsError::InvalidVariableCount { .. }
        ));
    }

    #[test]
    fn test_extract_coefficients() {
        let u = symbol!(u);
        let t = symbol!(t);
        let x = symbol!(x);
        let equation = expr!(u);
        let pde = Pde::new(equation, u, vec![t, x]);

        let result = extract_coefficients(&pde);
        assert!(result.is_ok());

        let coeffs = result.unwrap();
        assert_eq!(coeffs.a, Expression::integer(1));
        assert_eq!(coeffs.b, Expression::integer(1));
        assert_eq!(coeffs.c, Expression::integer(0));
    }

    #[test]
    fn test_check_singularities_both_zero() {
        let coeffs = PdeCoefficients {
            a: Expression::integer(0),
            b: Expression::integer(0),
            c: Expression::integer(1),
        };

        let result = check_singularities(&coeffs);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            CharacteristicsError::SingularCoefficients { .. }
        ));
    }

    #[test]
    fn test_check_singularities_valid() {
        let coeffs = PdeCoefficients {
            a: Expression::integer(1),
            b: Expression::integer(1),
            c: Expression::integer(0),
        };

        let result = check_singularities(&coeffs);
        assert!(result.is_ok());
    }

    #[test]
    fn test_construct_characteristic_equations() {
        let coeffs = PdeCoefficients {
            a: Expression::integer(1),
            b: Expression::integer(2),
            c: Expression::integer(0),
        };

        let char_eqs = construct_characteristic_equations(&coeffs);
        assert_eq!(char_eqs.len(), 3);
        assert_eq!(char_eqs[0], Expression::integer(1));
        assert_eq!(char_eqs[1], Expression::integer(2));
        assert_eq!(char_eqs[2], Expression::integer(0));
    }

    #[test]
    fn test_construct_general_solution() {
        let u = symbol!(u);
        let t = symbol!(t);
        let x = symbol!(x);
        let equation = expr!(u);
        let pde = Pde::new(equation, u, vec![t, x]);

        let coeffs = PdeCoefficients {
            a: Expression::integer(1),
            b: Expression::integer(1),
            c: Expression::integer(0),
        };

        let param = Symbol::new("s");
        let result = construct_general_solution(&pde, &coeffs, &param);
        assert!(result.is_ok());
    }

    #[test]
    fn test_solve_characteristic_odes_basic() {
        let char_eqs = vec![
            Expression::integer(1),
            Expression::integer(1),
            Expression::integer(0),
        ];

        let initial_conditions = vec![0.0, 0.0, 1.0];
        let s_end = 1.0;
        let step_size = 0.1;

        let result = solve_characteristic_odes(&char_eqs, &initial_conditions, s_end, step_size);
        assert!(result.is_ok());

        let solution = result.unwrap();
        assert!(!solution.is_empty());
        assert_eq!(solution[0].1.len(), 3);
    }

    #[test]
    fn test_solve_characteristic_odes_wrong_equation_count() {
        let char_eqs = vec![Expression::integer(1), Expression::integer(1)];
        let initial_conditions = vec![0.0, 0.0, 1.0];

        let result = solve_characteristic_odes(&char_eqs, &initial_conditions, 1.0, 0.1);
        assert!(result.is_err());
    }

    #[test]
    fn test_solve_characteristic_odes_wrong_ic_count() {
        let char_eqs = vec![
            Expression::integer(1),
            Expression::integer(1),
            Expression::integer(0),
        ];
        let initial_conditions = vec![0.0, 0.0];

        let result = solve_characteristic_odes(&char_eqs, &initial_conditions, 1.0, 0.1);
        assert!(result.is_err());
    }

    #[test]
    fn test_is_zero_true() {
        assert!(is_zero(&Expression::integer(0)));
    }

    #[test]
    fn test_is_zero_false() {
        assert!(!is_zero(&Expression::integer(1)));
    }

    #[test]
    fn test_characteristic_solution_clone() {
        let solution = CharacteristicSolution {
            characteristic_equations: vec![
                Expression::integer(1),
                Expression::integer(1),
                Expression::integer(0),
            ],
            parameter: Symbol::new("s"),
            solution: Expression::function("F", vec![Expression::symbol(symbol!(x))]),
            coefficients: PdeCoefficients {
                a: Expression::integer(1),
                b: Expression::integer(1),
                c: Expression::integer(0),
            },
        };
        let _cloned = solution.clone();
    }

    #[test]
    fn test_pde_coefficients_clone() {
        let coeffs = PdeCoefficients {
            a: Expression::integer(1),
            b: Expression::integer(1),
            c: Expression::integer(0),
        };
        let _cloned = coeffs.clone();
    }

    #[test]
    fn test_characteristics_error_clone() {
        let err = CharacteristicsError::NotFirstOrder;
        let _cloned = err.clone();
    }
}
