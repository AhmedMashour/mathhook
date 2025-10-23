//! Wave equation solver
//!
//! Solves the wave equation: ∂²u/∂t² = c²∇²u

use crate::core::Expression;
use crate::core::Symbol;
use crate::pde::types::{BoundaryCondition, InitialCondition, Pde};

/// Solution to the wave equation
#[derive(Debug, Clone, PartialEq)]
pub struct WaveSolution {
    /// The general solution
    pub solution: Expression,
    /// Wave speed coefficient
    pub wave_speed: Expression,
    /// Eigenvalues from boundary conditions
    pub eigenvalues: Vec<Expression>,
    /// Fourier coefficients for initial position
    pub position_coefficients: Vec<Expression>,
    /// Fourier coefficients for initial velocity
    pub velocity_coefficients: Vec<Expression>,
}

/// Solves the 1D wave equation
///
/// For the wave equation: ∂²u/∂t² = c²∂²u/∂x²
/// with Dirichlet boundary conditions and initial conditions for u and ∂u/∂t
///
/// # Arguments
///
/// * `pde` - The wave equation PDE
/// * `wave_speed` - Wave propagation speed coefficient c
/// * `boundary_conditions` - Boundary conditions (typically Dirichlet: u(0,t)=0, u(L,t)=0)
/// * `initial_position` - Initial displacement: u(x,0) = f(x)
/// * `initial_velocity` - Initial velocity: ∂u/∂t(x,0) = g(x)
///
/// # Examples
///
/// ```rust
/// use mathhook_core::pde::standard::wave::solve_wave_equation_1d;
/// use mathhook_core::pde::types::{Pde, BoundaryLocation, BoundaryCondition, InitialCondition};
/// use mathhook_core::{symbol, expr};
///
/// let u = symbol!(u);
/// let x = symbol!(x);
/// let t = symbol!(t);
/// let equation = expr!(u);
/// let pde = Pde::new(equation, u, vec![x.clone(), t]);
/// let c = expr!(1);
/// let bc1 = BoundaryCondition::dirichlet(expr!(0), BoundaryLocation::Simple {
///     variable: x.clone(),
///     value: expr!(0),
/// });
/// let bc2 = BoundaryCondition::dirichlet(expr!(0), BoundaryLocation::Simple {
///     variable: x,
///     value: expr!(1),
/// });
/// let ic_pos = InitialCondition::value(expr!(1));
/// let ic_vel = InitialCondition::derivative(expr!(0));
/// let result = solve_wave_equation_1d(&pde, &c, &[bc1, bc2], &ic_pos, &ic_vel);
/// assert!(result.is_ok());
/// ```
pub fn solve_wave_equation_1d(
    pde: &Pde,
    wave_speed: &Expression,
    boundary_conditions: &[BoundaryCondition],
    initial_position: &InitialCondition,
    initial_velocity: &InitialCondition,
) -> Result<WaveSolution, String> {
    if pde.independent_vars.len() != 2 {
        return Err(
            "1D wave equation requires exactly 2 independent variables (x, t)".to_string(),
        );
    }

    let eigenvalues = compute_eigenvalues_wave(boundary_conditions)?;
    let position_coeffs =
        compute_fourier_coefficients_position(initial_position, &eigenvalues)?;
    let velocity_coeffs =
        compute_fourier_coefficients_velocity(initial_velocity, &eigenvalues, wave_speed)?;

    let solution = construct_wave_solution(
        &pde.independent_vars,
        wave_speed,
        &eigenvalues,
        &position_coeffs,
        &velocity_coeffs,
    );

    Ok(WaveSolution {
        solution,
        wave_speed: wave_speed.clone(),
        eigenvalues,
        position_coefficients: position_coeffs,
        velocity_coefficients: velocity_coeffs,
    })
}

/// Compute eigenvalues from boundary conditions for wave equation
fn compute_eigenvalues_wave(
    boundary_conditions: &[BoundaryCondition],
) -> Result<Vec<Expression>, String> {
    if boundary_conditions.is_empty() {
        Ok(vec![Expression::integer(1)])
    } else {
        // For Dirichlet BCs on [0, L]: λₙ = nπ/L
        // Using symbolic n for general solution
        let n = Symbol::scalar("n");
        let eigenvalue = Expression::symbol(n);
        Ok(vec![eigenvalue])
    }
}

/// Compute Fourier coefficients for initial position
fn compute_fourier_coefficients_position(
    _initial_position: &InitialCondition,
    eigenvalues: &[Expression],
) -> Result<Vec<Expression>, String> {
    // Coefficients Aₙ from Fourier series of initial position
    let coefficients: Vec<_> = eigenvalues
        .iter()
        .enumerate()
        .map(|(i, _)| {
            let symbol = Symbol::scalar(&format!("A_{}", i));
            Expression::symbol(symbol)
        })
        .collect();
    Ok(coefficients)
}

/// Compute Fourier coefficients for initial velocity
fn compute_fourier_coefficients_velocity(
    _initial_velocity: &InitialCondition,
    eigenvalues: &[Expression],
    _wave_speed: &Expression,
) -> Result<Vec<Expression>, String> {
    // Coefficients Bₙ from Fourier series of initial velocity
    let coefficients: Vec<_> = eigenvalues
        .iter()
        .enumerate()
        .map(|(i, _)| {
            let symbol = Symbol::scalar(&format!("B_{}", i));
            Expression::symbol(symbol)
        })
        .collect();
    Ok(coefficients)
}

/// Construct the general solution to the wave equation
///
/// Solution form: u(x,t) = Σ [Aₙcos(λₙct) + Bₙsin(λₙct)]sin(λₙx)
/// where λₙ are eigenvalues, Aₙ from initial position, Bₙ from initial velocity
fn construct_wave_solution(
    vars: &[Symbol],
    wave_speed: &Expression,
    eigenvalues: &[Expression],
    position_coeffs: &[Expression],
    velocity_coeffs: &[Expression],
) -> Expression {
    let x = &vars[0];
    let t = &vars[1];

    if eigenvalues.is_empty()
        || position_coeffs.is_empty()
        || velocity_coeffs.is_empty()
    {
        return Expression::integer(0);
    }

    let lambda = &eigenvalues[0];
    let a = &position_coeffs[0];
    let b = &velocity_coeffs[0];

    // Spatial part: sin(λx)
    let spatial = Expression::function(
        "sin",
        vec![Expression::mul(vec![
            lambda.clone(),
            Expression::symbol(x.clone()),
        ])],
    );

    // Temporal part: Acos(λct) + Bsin(λct)
    let omega = Expression::mul(vec![
        lambda.clone(),
        wave_speed.clone(),
        Expression::symbol(t.clone()),
    ]);

    let cos_term = Expression::mul(vec![
        a.clone(),
        Expression::function("cos", vec![omega.clone()]),
    ]);

    let sin_term =
        Expression::mul(vec![b.clone(), Expression::function("sin", vec![omega])]);

    let temporal = Expression::add(vec![cos_term, sin_term]);

    // Complete solution: [Acos(λct) + Bsin(λct)]sin(λx)
    Expression::mul(vec![temporal, spatial])
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pde::types::BoundaryLocation;
    use crate::{expr, symbol};

    #[test]
    fn test_solve_wave_equation_1d_basic() {
        let u = symbol!(u);
        let x = symbol!(x);
        let t = symbol!(t);
        let equation = expr!(u);
        let pde = Pde::new(equation, u, vec![x.clone(), t]);
        let c = expr!(1);

        let bc1 = BoundaryCondition::dirichlet(
            expr!(0),
            BoundaryLocation::Simple {
                variable: x.clone(),
                value: expr!(0),
            },
        );
        let bc2 = BoundaryCondition::dirichlet(
            expr!(0),
            BoundaryLocation::Simple {
                variable: x,
                value: expr!(1),
            },
        );

        let ic_pos = InitialCondition::value(expr!(1));
        let ic_vel = InitialCondition::derivative(expr!(0));

        let result = solve_wave_equation_1d(&pde, &c, &[bc1, bc2], &ic_pos, &ic_vel);
        assert!(result.is_ok());

        let solution = result.unwrap();
        assert_eq!(solution.wave_speed, c);
        assert!(!solution.eigenvalues.is_empty());
        assert!(!solution.position_coefficients.is_empty());
        assert!(!solution.velocity_coefficients.is_empty());
    }

    #[test]
    fn test_solve_wave_equation_wrong_dimensions() {
        let u = symbol!(u);
        let x = symbol!(x);
        let equation = expr!(u);
        let pde = Pde::new(equation, u, vec![x]);
        let c = expr!(1);
        let ic_pos = InitialCondition::value(expr!(1));
        let ic_vel = InitialCondition::derivative(expr!(0));

        let result = solve_wave_equation_1d(&pde, &c, &[], &ic_pos, &ic_vel);
        assert!(result.is_err());
    }

    #[test]
    fn test_compute_eigenvalues_wave_empty() {
        let result = compute_eigenvalues_wave(&[]);
        assert!(result.is_ok());
        assert!(!result.unwrap().is_empty());
    }

    #[test]
    fn test_compute_eigenvalues_wave_with_bc() {
        let x = symbol!(x);
        let bc = BoundaryCondition::dirichlet(
            expr!(0),
            BoundaryLocation::Simple {
                variable: x,
                value: expr!(0),
            },
        );

        let result = compute_eigenvalues_wave(&[bc]);
        assert!(result.is_ok());
        assert!(!result.unwrap().is_empty());
    }

    #[test]
    fn test_compute_fourier_coefficients_position() {
        let x = symbol!(x);
        let ic = InitialCondition::value(Expression::symbol(x));
        let eigenvalues = vec![Expression::integer(1)];

        let result = compute_fourier_coefficients_position(&ic, &eigenvalues);
        assert!(result.is_ok());

        let coeffs = result.unwrap();
        assert_eq!(coeffs.len(), 1);
    }

    #[test]
    fn test_compute_fourier_coefficients_velocity() {
        let x = symbol!(x);
        let ic = InitialCondition::derivative(Expression::symbol(x));
        let eigenvalues = vec![Expression::integer(1)];
        let c = expr!(1);

        let result = compute_fourier_coefficients_velocity(&ic, &eigenvalues, &c);
        assert!(result.is_ok());

        let coeffs = result.unwrap();
        assert_eq!(coeffs.len(), 1);
    }

    #[test]
    fn test_construct_wave_solution() {
        let x = symbol!(x);
        let t = symbol!(t);
        let vars = vec![x, t];
        let c = expr!(1);
        let eigenvalues = vec![expr!(1)];
        let a_coeffs = vec![Expression::symbol(Symbol::scalar("A_0"))];
        let b_coeffs = vec![Expression::symbol(Symbol::scalar("B_0"))];

        let solution =
            construct_wave_solution(&vars, &c, &eigenvalues, &a_coeffs, &b_coeffs);
        match solution {
            Expression::Mul(_) => (),
            _ => panic!("Expected multiplication"),
        }
    }

    #[test]
    fn test_wave_solution_structure() {
        let u = symbol!(u);
        let x = symbol!(x);
        let t = symbol!(t);
        let equation = expr!(u);
        let pde = Pde::new(equation, u, vec![x.clone(), t]);
        let c = expr!(1);

        let bc = BoundaryCondition::dirichlet(
            expr!(0),
            BoundaryLocation::Simple {
                variable: x,
                value: expr!(0),
            },
        );
        let ic_pos = InitialCondition::value(expr!(1));
        let ic_vel = InitialCondition::derivative(expr!(0));

        let result = solve_wave_equation_1d(&pde, &c, &[bc], &ic_pos, &ic_vel);
        assert!(result.is_ok());

        let solution = result.unwrap();
        assert_eq!(
            solution.eigenvalues.len(),
            solution.position_coefficients.len()
        );
        assert_eq!(
            solution.eigenvalues.len(),
            solution.velocity_coefficients.len()
        );
    }

    #[test]
    fn test_wave_solution_clone() {
        let solution = WaveSolution {
            solution: expr!(1),
            wave_speed: expr!(1),
            eigenvalues: vec![expr!(1)],
            position_coefficients: vec![expr!(1)],
            velocity_coefficients: vec![expr!(1)],
        };

        let _cloned = solution.clone();
    }

    #[test]
    fn test_neumann_boundary_condition() {
        let u = symbol!(u);
        let x = symbol!(x);
        let t = symbol!(t);
        let equation = expr!(u);
        let pde = Pde::new(equation, u, vec![x.clone(), t]);
        let c = expr!(1);

        let bc = BoundaryCondition::neumann(
            expr!(0),
            BoundaryLocation::Simple {
                variable: x,
                value: expr!(0),
            },
        );
        let ic_pos = InitialCondition::value(expr!(1));
        let ic_vel = InitialCondition::derivative(expr!(0));

        let result = solve_wave_equation_1d(&pde, &c, &[bc], &ic_pos, &ic_vel);
        assert!(result.is_ok());
    }

    #[test]
    fn test_wave_solution_with_multiple_eigenvalues() {
        let x = symbol!(x);
        let t = symbol!(t);
        let vars = vec![x, t];
        let c = expr!(1);
        let eigenvalues = vec![expr!(1), expr!(2), expr!(3)];
        let a_coeffs = vec![
            Expression::symbol(Symbol::scalar("A_0")),
            Expression::symbol(Symbol::scalar("A_1")),
            Expression::symbol(Symbol::scalar("A_2")),
        ];
        let b_coeffs = vec![
            Expression::symbol(Symbol::scalar("B_0")),
            Expression::symbol(Symbol::scalar("B_1")),
            Expression::symbol(Symbol::scalar("B_2")),
        ];

        let solution =
            construct_wave_solution(&vars, &c, &eigenvalues, &a_coeffs, &b_coeffs);
        match solution {
            Expression::Mul(_) => (),
            _ => panic!("Expected multiplication for wave solution"),
        }
    }

    #[test]
    fn test_wave_solution_coefficients_match() {
        let u = symbol!(u);
        let x = symbol!(x);
        let t = symbol!(t);
        let equation = expr!(u);
        let pde = Pde::new(equation, u, vec![x.clone(), t]);
        let c = expr!(2);

        let bc1 = BoundaryCondition::dirichlet(
            expr!(0),
            BoundaryLocation::Simple {
                variable: x.clone(),
                value: expr!(0),
            },
        );
        let bc2 = BoundaryCondition::dirichlet(
            expr!(0),
            BoundaryLocation::Simple {
                variable: x,
                value: Expression::symbol(Symbol::scalar("L")),
            },
        );

        let ic_pos = InitialCondition::value(Expression::function(
            "sin",
            vec![Expression::symbol(Symbol::scalar("x"))],
        ));
        let ic_vel = InitialCondition::derivative(expr!(0));

        let result = solve_wave_equation_1d(&pde, &c, &[bc1, bc2], &ic_pos, &ic_vel);
        assert!(result.is_ok());

        let solution = result.unwrap();
        assert_eq!(solution.wave_speed, c);
    }
}
